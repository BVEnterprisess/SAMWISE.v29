# The Governor — Control Loop Design

**Status:** Proposed
**Date:** 2026-08-06
**Supersedes:** the Pass 5 section of `2026-08-06-authority-boundary-correction-design.md` (its archive decision stands)
**Relationship to MEMORY SPECIFICATION v1.4:** corrects v1.4 §5 (placement) and §6/§9 (control law and stability claim). The invariants in v1.4 §2 are preserved unchanged.

## 1. The defect

v1.4 §5.1/§10 specifies the control loop as an inline Skill registered via `POST /api/skills` with `type:'middleware', phase:'both'`, exporting `preRequest` and `postResponse`.

**That API does not exist, and the abstraction it assumes is the wrong shape.**

Verified against `/home/johnh/services/OmniRoute`:

| v1.4 assumes | Shipped reality | Evidence |
|---|---|---|
| `preRequest` / `postResponse` hooks | Do not exist | zero occurrences repo-wide |
| Skill = request middleware | Skill = tool handler | `SkillHandler = (input, ctx{apiKeyId,sessionId,provider?,model?}) => Promise<Record<string,unknown>>` in `src/lib/skills/types.ts:58` |
| Skill runs on every request | Skill runs **only if the model calls it** | `src/lib/skills/injection.ts` injects skills as *tools*; `src/lib/skills/interception.ts:83 interceptToolCalls` executes them on tool-call detection |
| `omniroute_set_api_key_scopes` | Absent | not in `src/shared/constants/mcpScopes.ts` |
| `omniroute_set_combo_parameter` | Absent | not in registry |
| Prompt Injection Guard | **Exists** | `src/middleware/promptInjectionGuard.ts` |

The Skills Framework is a **tool-provider** framework. Skill execution is **model-elective**.

A governor cannot be elective. An agent that has drifted simply does not call the skill that would correct it, and every gate still reports green. Implemented as a Skill, the control loop is advisory — which is indistinguishable from absent under exactly the conditions it exists to handle.

**The governor is request-path middleware, not a Skill.** `promptInjectionGuard.ts` is the proof-of-pattern already in the tree.

## 2. Corrected placement

```text
request
  → OmniRoute middleware chain
      → promptInjectionGuard        (exists)
      → GOVERNOR.preRequest         (new: approval + physics)
  → provider relay
  → GOVERNOR.postResponse           (new: validation + commit/dampen)
  → response
```

The governor is unconditional, ordered, and not addressable by the model. It is not registered through `/api/skills` and is not injectable as a tool.

## 3. Control law corrections

### 3.1 Drift must be measured server-side (highest severity)

v1.4 reads drift from the agent-supplied `X-Drift-Score` header. The restoring force therefore depends on a measurement produced by the plant being controlled. An agent that is drifted, buggy, or adversarial reports `0.0`, escapes gravity entirely, and passes every gate. This also inverts invariant 4 — the agent owns the measurement, so memory does not own the agent.

**Rule:** the governor computes drift itself, from the request body against the session's baseline anchor retrieved from YantrikDB.

`X-Drift-Score`, if present, is recorded as an **advisory observation** (evidence about the agent's self-model) and is never the control input. Divergence between reported and measured drift is itself a signal worth persisting.

### 3.2 State the equation as a dynamical system

v1.4 §6 states `Acceleration + Friction + Gravity = Drift`, with gravity defined as a function of drift. That is an accounting identity, not an equation of motion, and cannot be analyzed for stability.

Replace with a damped system under bounded exploration noise:

```text
ẍ = −γẋ − λx + η(t)

x     = measured deviation from baseline anchor
γ     = adaptive friction coefficient (§3.4)
λ     = gravity gain (§3.3)
η(t)  = exploration; deliberately non-zero, bounded
```

### 3.3 Gravity: proportional with hysteresis, not bang-bang

v1.4's math says `λ·d`; its implementation says `if d > threshold, inject baseline`. Discontinuous restoring force across a threshold produces chattering, and below the threshold drift is entirely uncorrected.

**Rule:** graded reinforcement proportional to measured deviation, with an explicit hysteresis band `[d_lo, d_hi]` — engage above `d_hi`, disengage below `d_lo`, `d_lo < d_hi`. Both bounds are versioned policy parameters.

### 3.4 Friction must saturate and must not reward incapacity

v1.4's friction raises on *any* failure, and responds by clamping temperature and stripping scopes. Where failures were caused by insufficient capability, that response causes more failures, which raises friction further. This is positive feedback wearing a damping costume.

Damping requires `∂(failure rate)/∂(friction) < 0`, which is false for capability-limited failures.

**Rules:**

1. **Classify failures by cause before they touch γ.**
   - *Divergence failures* (schema violation, signature mismatch, injection attempt, policy breach) → **increase** friction. Restricting the agent genuinely reduces these.
   - *Capability failures* (missing scope, permission denied, unavailable tool, budget exhausted) → **must not** increase friction. These are the positive-feedback path.
2. **Saturate:** `γ ≤ γ_max`.
3. **Release:** γ decays by `e^(−α·age)` and must be *provably* monotonically decreasing when no divergence failures occur in the window.

### 3.5 The stability claim must be weakened to what is true

v1.4 §6/§9 claim total energy "strictly dissipates." Strict dissipation requires no persistent forcing, but exploration is continuous and deliberate by design (§7: "drift is our exploration budget"). With persistent `η`, the system does not decay monotonically.

**Correct claim:** the system converges to a **bounded stationary distribution** — bounded variance around the baseline, not monotonic dissipation. This remains Lyapunov-stable and fully sufficient for the guarantee in §9.

This correction is mandatory: claiming strict dissipation asserts more than the evidence supports, which is the exact failure the epistemic contract forbids. The architecture survives the weakening intact.

### 3.6 Availability policy must be explicit

Invariant 5 forbids local caching (correctly — split-brain would break the singularity). That places ≥2 YantrikDB reads on the critical path of every inference call. "ALWAYS AVAILABLE" is an aspiration, not a mechanism.

**Rule — fail-closed by default.** If YantrikDB is unreachable, the governor cannot verify approval or compute gravity, therefore it must not silently allow. Denial is the honest default.

An operator-set emergency bypass may exist. Every request served under bypass is marked `unverified`, is never committed as validated continuity, and is reconciled on recovery. The bypass state itself is a recorded event.

## 4. The reconciliation rule (the actual continuity gap)

v1.4 §2 asserts: *"If it wasn't Approved AND Validated, it never happened."*

This is false at the execution layer. Validation is post-response (§5.2 Step 4), and the response is returned to the agent regardless (Step 5). An unvalidated response **is delivered and may already have caused side effects**. It never happened *in memory*; it happened in the world.

A singular source of truth cannot tolerate silent divergence between the timeline and reality.

**Rule — classify by side-effect risk:**

| Response class | Handling |
|---|---|
| Contains tool calls / can cause side effects | **Validate before delivery.** Buffer the response; release only on validation pass. Latency cost is accepted. |
| Pure text completion | Deliver, then validate. On validation failure, append an explicit **divergence record**. |

A divergence record states: response `R`, trace `T`, delivered to key `K` at time `t`, subsequently failed validation `V`. Any asset derived from a diverged response projects as `contested` until reconciled. Divergence records are never suppressed to make the timeline look clean.

## 5. Interfaces

```text
GovernorDecision
  decision_id
  trace_id
  session_id
  api_key_id
  measured_drift            server-computed
  reported_drift            advisory, nullable
  gravity_applied           bool + magnitude
  friction_gamma            value at decision time
  friction_inputs[]         divergence-failure ids only
  policy_bundle_id + version
  governance_epoch
  gate_result               approved | rejected(reason)
  availability_mode         verified | bypass_unverified
  created_at
```

`ValidationOutcome` links to `decision_id` and records `pass | fail(reason)`, `delivered_before_validation: bool`, and `divergence_record_id` when applicable.

Both are immutable; evidence links and evaluations are append-only, per v1.4 §2 and the closure contract.

## 6. Falsification

The governor is not accepted on green tests. It is accepted when these adversarial cases pass:

1. **Lying agent** — agent reports `X-Drift-Score: 0.0` while its request is measurably far from baseline. Gravity must still engage. *Proves drift is server-measured.*
2. **Capability starvation** — repeated permission-denied failures. Friction must **not** rise. *Proves §3.4 classification.*
3. **Chatter** — drift oscillating across `d_hi`. Engagement must not flap. *Proves hysteresis.*
4. **Saturation** — sustained divergence failures. `γ` must cap and must release once failures stop. *Proves §3.4.2/3.*
5. **YantrikDB down** — the governor must deny, or serve marked `unverified`. It must never serve as verified. *Proves §3.6.*
6. **Post-delivery invalidation** — a pure-text response fails validation after delivery. A divergence record must exist and derived assets must project `contested`. *Proves §4.*
7. **Side-effecting response** — a tool-call response failing validation must never have been delivered. *Proves §4.*

## 7. What this does not close

- It does not implement Passes 1–9 of the closure contract.
- It does not prove compounding. A stable governor bounds drift; it does not demonstrate that execution improves execution. `H0` remains untested.
- It does not deliver the missing MCP tools (`set_api_key_scopes`, `set_combo_parameter`). Friction actuation currently has no mechanism; §3.4 defines the law, not its actuator. Building those is separate scoped work.
- Stability is claimed as bounded variance (§3.5), never as correctness.
