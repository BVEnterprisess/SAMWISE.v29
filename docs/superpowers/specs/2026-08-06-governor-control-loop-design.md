# The Governor — Two-Speed Control Loop Design (v2)

**Status:** Proposed
**Date:** 2026-08-06
**Supersedes:** v1 of this document (which assumed a synchronous hot path) and the Pass 5 section of `2026-08-06-authority-boundary-correction-design.md` (its archive decision stands).
**Relationship to MEMORY SPECIFICATION v1.4:** corrects §5 (placement), §2.5 (statelessness), §6/§9 (control law and stability claim). v1.4's invariants 1–4 are preserved unchanged.

## 1. Execution model — CQRS, two speeds

The control loop is **not** synchronous. Command and query responsibilities are segregated across two speeds, mirroring System 1 / System 2:

| Component | Speed | Role | Work performed |
|---|---|---|---|
| **OmniRoute** | Hot, O(1) | **Enforcer** | Reads materialized state variables. Enforces scopes, applies friction γ, trips breakers, drops violating packets. **Zero heavy math.** |
| **MetaClaw** | Hot, O(1) | **Injector** | Keyword / cached-embedding scan, retrieves bounded `SKILL.md` artifacts, injects into system prompt. **Does not think.** |
| **YantrikDB** | Slow, async | **Lawmaker** | On session boundary, `think()` sweeps append-only logs: computes embedding divergence, integrates friction, applies temporal decay, detects contradictions, synthesizes skills, resets baseline. |

**Handoff:** `think()` publishes updated baseline, friction scores, and evolved skills back to OmniRoute and MetaClaw. The next hot-path request meets a newly tightened reality.

Execution speed is decoupled from cognitive depth. The v1 concern that YantrikDB would choke the inference loop is **withdrawn** — it was correct for a synchronous placement and does not apply here.

### 1.1 Materialized state contract

Hot-path physics are **reads, not computations**:

```text
γ_session      friction coefficient        materialized scalar   O(1)
λ, d_session   gravity gain, deviation     materialized scalar   O(1)
baseline_ref   anchor prompt / vector      materialized ref      O(1)
epoch          governance epoch            materialized scalar   O(1)
```

All are authored **solely** by YantrikDB's previous `think()` cycle.

### 1.2 Reconciling v1.4 invariant 5 (statelessness)

v1.4 §2.5 requires the inline component be stateless with "no local caching, no split-brain." Materialized state is, mechanically, local state — so the invariant needs restating rather than violating.

**These are projections, not caches.** A cache is written by its reader and may diverge. A projection is:

- **single-writer** — authored only by YantrikDB `think()`;
- **read-only on the hot path** — OmniRoute and MetaClaw never write it;
- **versioned** — carries `policy_bundle_id` and `epoch`, so staleness is detectable rather than silent.

Single-writer semantics preserve the singularity. Split-brain is impossible because the hot path has no write authority over control state.

## 2. Placement — the governor is middleware, not a Skill

v1.4 §5.1/§10 specifies registering the loop via `POST /api/skills` with `type:'middleware', phase:'both'`, exporting `preRequest`/`postResponse`.

**That API does not exist.** Verified against `/home/johnh/services/OmniRoute`:

| v1.4 assumes | Shipped reality | Evidence |
|---|---|---|
| `preRequest` / `postResponse` hooks | Do not exist | zero occurrences repo-wide |
| Skill = request middleware | Skill = **tool handler** | `SkillHandler = (input, ctx{apiKeyId,sessionId,…}) => Promise<…>` — `src/lib/skills/types.ts:58` |
| Skill runs every request | Runs **only if the model calls it** | `src/lib/skills/injection.ts` injects skills as *tools*; `interception.ts:83 interceptToolCalls` fires on tool-call detection |
| `omniroute_set_api_key_scopes` | Absent | not in `src/shared/constants/mcpScopes.ts` |
| `omniroute_set_combo_parameter` | Absent | not in registry |
| Prompt Injection Guard | **Exists** | `src/middleware/promptInjectionGuard.ts` |

OmniRoute's Skills Framework is a **tool-provider** framework; execution is **model-elective**.

An enforcer cannot be elective. A drifted agent simply declines to call the thing that would correct it, and every gate still reports green. The governor must sit in the request path unconditionally, as `promptInjectionGuard.ts` already does. It is not registered through `/api/skills` and is not addressable by the model.

## 2.1 The trace substrate already exists — SAMWISE builds none of it

Per the OmniRoute Observable Trace Inventory (2026-08-05), OmniRoute emits **~506 enumerated traces** across 40 categories. Trace capture, correlation, and canonicalization are **solved and shipped**. SAMWISE must not rebuild any of it (invariant 15).

Load-bearing for this design:

| Need | Already provided | Source |
|---|---|---|
| Content-addressed immutable artifacts | **Four-Stage Request Artifacts** — raw client req, translated provider req, raw provider resp, translated client resp, each with relative path, size, **SHA-256** | `${DATA_DIR}/call_logs/` + `request_detail_logs` |
| Append-only evidence log | `audit_log`, `mcp_tool_audit` (input hash, no payload), `call_logs`, `proxy_logs` | SQLite tables |
| Session boundary signal | `X-Session-Id`, effective session id, session age, last-active, per-key breakdown | §11 Session Tracking |
| Provenance correlation | `X-Request-Id` correlation id threaded through all logs | §3 |
| Compression derivative | Per-request tokens saved, mode, techniques, latency; `compression_savings` histogram | §12, §14 |
| Failure classification inputs (§3.4) | error code, error type, error source, retry count, circuit state, lockout reason, terminal-vs-stale error detection | §2, §3 |

**Stage 2→3 artifacts with SHA-256 are the asset boundary.** YantrikDB ingests from there and never earlier (per the authority-boundary spec §4).

### 2.1.1 Friction and gravity actuators exist under different names

v1 of this spec claimed the actuators were missing because `omniroute_set_api_key_scopes` and `omniroute_set_combo_parameter` are absent from the MCP registry. That was too narrow — the *capabilities* exist:

| Physics | Shipped actuator |
|---|---|
| Gravity (baseline re-injection) | **Global system prompt** — enabled flag, content, position (§21) |
| Friction (parameter clamp) | **Thinking budget** — mode `passthrough/auto/custom/adaptive`, custom value, effort level (§20) |
| Friction (capability restriction) | Scope enforcement before handler dispatch, 9 scopes + wildcard (§39); combo target disable / reorder / auto-removal (§10) |
| Hard stop | Circuit breaker state + manual reset (§34); rate-limit profiles (§22); IP filter (§19) |

What is missing is not the mechanism but a **programmatic, per-session, TTL-bounded write path** to these controls. That is a materially smaller build than "the actuators don't exist."

### 2.1.1b The write path largely exists too

Per the OmniRoute forensic audit, per-request and per-session control mutation is already shipped:

| Need | Shipped mechanism |
|---|---|
| Per-request scoring override | `X-OmniRoute-Mode`, `X-OmniRoute-Steer` headers |
| Runtime strategy mutation | `omniroute_set_routing_strategy` (MCP) |
| Breaker threshold tuning (friction) | `omniroute_set_resilience_profile` (MCP) |
| Budget clamp | `omniroute_set_budget_guard` (MCP) |
| Routing manifest swap | `omniroute_switch_combo` (MCP) |
| Dry-run before applying | `omniroute_simulate_route` (MCP) |
| Full trajectory read | `omniroute_get_session_snapshot` (MCP) |
| Conditional policy as data | Payload rules, tag-based routing, scheduled budgets |

The three named MCP tools are confirmed present in `src/shared/constants/mcpScopes.ts`. **The governor therefore actuates through existing surfaces; it does not require new OmniRoute primitives.** This retires the "missing actuators" blocker from v1.

### 2.1.2 Benchmark confounds — must be controlled or `H0` is meaningless

OmniRoute ships several mechanisms that **independently make a second run cheaper or better**, with no learning involved. If any are active during a paired Run 1 / Run 2, rejecting `H0` proves nothing:

| Mechanism | Confound |
|---|---|
| **Semantic cache** — keyed on SHA-256 of the full request | Run 2 returns Run 1's cached response. Measures caching, not compounding. |
| **Request dedup / idempotency** — identical requests within 5s | Same class of false positive. |
| **Context Relay** | Injects a compact summary across account rotation — improves Run 2 through a path that is not the learned projection. |
| **Zero-Latency Mode** — predictive TTFT skipping + hedging | Latency deltas become routing artefacts, not learning artefacts. |
| **Session affinity / prompt-cache pinning** | Provider-side prompt caching improves the warm run independently. |

**Rule:** the benchmark harness must disable semantic cache, dedup, context relay, and hedging, and must pin routing, for all three arms (Run 1, Run 2, no-learning control). Any that cannot be disabled must be reported as a stated confound alongside the result. This is also why Run 2 must use a *held-out sibling variant* rather than a byte-identical repeat — an identical repeat is indistinguishable from a cache hit by construction.

### 2.1.3 Evidence-substrate integrity — unverified, potentially blocking

The forensic audit states of the audit tables: *"Audit writes never throw — failure is silently swallowed so audit cannot break the request flow."*

If accurate, the evidence substrate can **silently lose records under pressure**. That is incompatible with Gate 3's requirement of *100% provenance for every injected item*: provenance cannot be guaranteed on a log whose writes may drop without signal.

This is recorded as **claimed-by-documentation and not verified in code** — the relevant handler was not located during this pass. It must be confirmed before any provenance claim is made. If confirmed, SAMWISE requires a durable, non-swallowing evidence path for asset-bearing writes, distinct from OmniRoute's best-effort operational audit.

### 2.1.4 Unresolved: three memory systems

OmniRoute ships its own memory layer — FTS5 + optional Qdrant, with **memory extraction facts** and **memory injection tokens** (§17). MetaClaw injects skills. YantrikDB is defined as the singular substrate.

That is three retrieval-and-injection systems on one hot path. Invariant 15 forbids a parallel extractor reinterpreting the same source without an explicit versioned contract, and OmniRoute's own memory extraction is, as written, exactly that relative to YantrikDB.

**This must be resolved before implementation.** The options are to scope OmniRoute's memory to non-durable request-local recall, to disable it in favour of YantrikDB, or to declare an explicit versioned contract between them. This design does not choose; it records the collision as blocking.

## 3. Control law

### 3.1 Drift is materialized by YantrikDB, never supplied by the agent

v1.4 reads drift from the agent-supplied `X-Drift-Score` header. The restoring force would then depend on a measurement produced by the plant being controlled: a drifted, buggy, or adversarial agent reports `0.0`, escapes gravity, and passes every gate. This also inverts invariant 4 — the agent would own the measurement, so memory would not own the agent.

**Rule:** `d_session` is computed by `think()` (true embedding divergence against the stored baseline) and read O(1) on the hot path.

`X-Drift-Score`, if present, is recorded as an **advisory observation** — evidence about the agent's self-model — and is never the control input. Divergence between reported and materialized drift is itself high-value signal and is persisted.

### 3.2 This is a sampled-data control system — and that has a cost

Because γ and λ·d are held constant between `think()` cycles (zero-order hold), the loop is **discrete-continuous hybrid**, sampled at session boundaries:

```text
ẍ = −γẋ − λx + η(t)      γ, λ held constant across a session
sampling period T = session length
```

**Consequence: the governor cannot correct drift that develops and resolves within a single session.** Intra-session drift is invisible until the next boundary. Zero-order-hold sampling preserves stability only when `T` is short relative to the system's dynamics; as `T → ∞` the loop is effectively open.

This is not a defect — it is the correct division of labor, and it must be stated rather than discovered:

- **Hard safety is synchronous.** Scope enforcement, policy rules, injection guard, and circuit breaking run on every request in O(1). These are reflexes; they do not need the slow loop.
- **Soft correction is sampled.** Gravity and friction are between-session forces. They shape the *next* session, not the current one.

**Rule:** session length is a bounded, versioned policy parameter (`T_max`). A session exceeding `T_max` forces a `think()` checkpoint. Without a bound on `T`, no stability claim can be made.

### 3.3 Gravity: graded with hysteresis, not bang-bang

v1.4's math says `λ·d`; its implementation says `if d > threshold, inject baseline`. Discontinuous restoring force across a threshold chatters, and below the threshold drift is uncorrected.

**Rule:** `think()` materializes a graded gravity gain; OmniRoute/MetaClaw apply it O(1). Engagement uses an explicit hysteresis band `[d_lo, d_hi]`, `d_lo < d_hi` — engage above `d_hi`, disengage below `d_lo`. Both are versioned policy parameters.

### 3.4 Friction must saturate and must not punish incapacity

v1.4 raises friction on *any* failure and responds by clamping temperature and stripping scopes. Where failures were caused by insufficient capability, that response causes more failures, raising friction further — positive feedback wearing a damping costume. Damping requires `∂(failure rate)/∂(friction) < 0`, which is false for capability-limited failures.

CQRS makes the fix natural: **classification is slow-pass work**, performed inside `think()`, which has the time and the full trajectory to reason about cause.

**Rules:**

1. **Classify before integrating.**
   - *Divergence failures* (schema violation, signature mismatch, injection attempt, policy breach) → **increase** γ. Restriction genuinely reduces these.
   - *Capability failures* (missing scope, permission denied, tool unavailable, budget exhausted) → **must not** increase γ. This is the positive-feedback path.
2. **Saturate:** `γ ≤ γ_max`.
3. **Release:** γ decays as `e^(−α·age)` and must be provably monotonically decreasing across cycles with no divergence failures.

### 3.5 The stability claim must be weakened to what is true

v1.4 §6/§9 claim total energy "strictly dissipates." Strict dissipation requires no persistent forcing, but exploration is continuous and deliberate by design (§7: "drift is our exploration budget").

**Correct claim:** the system converges to a **bounded stationary distribution** — bounded variance about the baseline — under a bounded sampling period `T ≤ T_max` (§3.2). This is Lyapunov-stable and fully sufficient for the guarantee in §9.

The weakening is mandatory: claiming strict dissipation asserts more than the evidence supports, the exact failure the epistemic contract forbids. The architecture survives intact.

### 3.6 Availability of materialized state

The hot path reads local projections, so YantrikDB being down does **not** stall inference. It does mean control state goes stale.

**Rule:** projections carry `epoch` and `computed_at`. Beyond `staleness_max`, the governor degrades to **conservative defaults** (maximum friction, gravity engaged) rather than to permissive defaults. Requests served on stale projections are marked `degraded` and reconciled at the next `think()`. Failing safe means failing *tight*, not failing open.

## 4. The reconciliation rule

v1.4 §2 asserts: *"If it wasn't Approved AND Validated, it never happened."*

This is false at the execution layer, and CQRS widens the gap: semantic validation now happens in `think()`, long after the response was delivered. An unvalidated response **is delivered and may already have caused side effects**. It never happened *in memory*; it happened in the world.

A singular source of truth cannot tolerate silent divergence between the timeline and reality.

**Rule — classify by side-effect risk:**

| Response class | Handling |
|---|---|
| Contains tool calls / can cause side effects | **Synchronous validation before delivery.** Schema + signature only — both O(1), no semantic reasoning. Release on pass. |
| Pure text completion | Deliver; validate in the slow pass. On failure, append an explicit **divergence record**. |

A divergence record states: response `R`, trace `T`, delivered to key `K` at time `t`, later failed validation `V`. Assets derived from a diverged response project as `contested` until reconciled. Divergence records are never suppressed to make the timeline look clean.

## 5. The skill promotion gate — where SAMWISE earns its place

MetaClaw's own threat model (§4.1) states plainly:

> "a single successful prompt injection is effectively 'compiled' into persistent, cross-session malware. There is no native quarantine, human-review gate, or static deny-list scanning between extraction and disk persistence."

And §4.2: deduplication is **name-string match only**, with "no semantic contradiction resolution or explicit precedence scoring."

This directly contradicts the SAMWISE epistemic contract, which requires that a failure-derived candidate is a *hypothesis* until corrective evidence passes objective and safety evaluation. MetaClaw as shipped writes synthesized skills straight to disk, unsupervised, and injects them into every subsequent turn.

**This is the integration gap, and it is precisely the value SAMWISE adds.** The two systems are complementary:

| MetaClaw lacks | YantrikDB provides |
|---|---|
| Quarantine between extraction and persistence | Projection status: `partial` / `contested` / `rejected` / `supported` |
| Semantic contradiction resolution | Conflict scanner, contradiction detection |
| Precedence / evidence scoring | Importance calibration, decay, evidence links |
| Provenance on injected artifacts | Immutable assets, append-only evidence |

**Rule:** a `SkillEvolver` proposal is a **procedure candidate asset**, not a skill. It is written to YantrikDB as immutable evidence and projects as `partial` until corrective outcome evidence promotes it. **Only `supported` candidates are materialized into MetaClaw's injectable skill bank.** MetaClaw's direct-to-disk path is disabled; YantrikDB becomes the sole writer of the skill bank, consistent with §1.2 single-writer projection semantics.

This closes the poisoning vector without modifying MetaClaw's hot-path injection behaviour.

## 6. Interfaces

```text
GovernorDecision              (hot path, O(1))
  decision_id, trace_id, session_id, api_key_id
  d_session                   materialized, from think()
  reported_drift              advisory, nullable
  gamma                       materialized
  gravity_engaged             bool + magnitude
  policy_bundle_id + version, epoch
  gate_result                 approved | rejected(reason)
  projection_state            fresh | degraded(staleness)
  created_at

ThinkOutcome                  (slow pass)
  cycle_id, session_id, epoch_in → epoch_out
  d_computed, gamma_out
  divergence_failures[]       ids that raised gamma
  capability_failures[]       ids explicitly excluded
  candidates_proposed[] / candidates_promoted[]
  contradictions[]
  baseline_reset              bool
```

Both immutable; evidence links and evaluations append-only.

## 7. Falsification

Accepted only when these adversarial cases pass:

1. **Lying agent** — reports `X-Drift-Score: 0.0` while measurably far from baseline. Gravity must still engage on the next cycle. *Proves §3.1.*
2. **Unbounded session** — a session exceeding `T_max` must force a `think()` checkpoint. *Proves §3.2.*
3. **Capability starvation** — repeated permission-denied failures must **not** raise γ. *Proves §3.4.1.*
4. **Chatter** — drift oscillating across `d_hi` must not flap engagement. *Proves §3.3.*
5. **Saturation and release** — sustained divergence caps γ; cessation releases it monotonically. *Proves §3.4.2–3.*
6. **Stale projections** — beyond `staleness_max`, the governor tightens, never loosens. *Proves §3.6.*
7. **Post-delivery invalidation** — a pure-text response failing slow-pass validation yields a divergence record; derived assets project `contested`. *Proves §4.*
8. **Side-effecting response** — a tool-call response failing sync validation must never have been delivered. *Proves §4.*
9. **Skill poisoning** — a transcript containing an injected instruction must produce a `partial` candidate that is **never** materialized into the injectable bank absent corrective evidence. *Proves §5.*

## 8. Blocking gap — half the Gradient is uninstrumented

`Gradient = Compounding` has two derivatives. Against ~506 enumerated OmniRoute traces:

| Derivative | Instrumented? | Source |
|---|---|---|
| Quality-constrained **compression ratio** ↑ | **Yes** | per-request tokens saved, mode, techniques; `compression_savings` histogram; `tokens_consumed`, `cost_usd` counters (§12, §14) |
| **HITL coordination tax** ↓ | **No** | *no trace in the inventory measures human intervention* |

Nothing in OmniRoute emits a HITL event — correctly, because human coordination happens *outside* the router, at the agent/human boundary (approval prompts, re-explanation, manual correction, restarts).

**Consequence: `H0` cannot be tested today.** The benchmark requires `hitl_interventions` and `hitl_coordination_ms` per execution; there is no producer for either. Compression alone cannot reject `H0`, and optimising compression alone is explicitly forbidden (a system may not "compound" by answering less).

**This is the highest-priority instrumentation gap in the whole programme**, and it sits outside all three components — it belongs to the agent harness (OpenClaw / Claude Code / Codex), not to OmniRoute, MetaClaw, or YantrikDB. It must be specified and built before any compounding claim is possible.

## 9. What this does not close

- Does not implement Passes 1–9 of the closure contract.
- Does not prove compounding. A stable, sampled governor bounds drift; it does not demonstrate that execution improves execution. `H0` remains untested — and per §8, currently *untestable*.
- Does not resolve the three-memory-system collision (§2.1.2), which is blocking.
- Does not build the per-session TTL-bounded write path to the existing actuators (§2.1.1).
- Stability is claimed as bounded variance under bounded `T`, never as correctness.
