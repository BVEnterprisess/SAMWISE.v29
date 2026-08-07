# Authority Boundary Correction — Design

**Status:** Proposed
**Date:** 2026-08-06
**Scope:** Remove `samwise-core` from the active architecture; re-establish the SAMWISE service boundary under the authority test; redefine Pass 5 as artifact emission rather than runtime interception.
**Out of scope:** Implementing Passes 1–9; building the OmniRoute consumer; any change to OmniRoute itself.

## 1. Problem

`samwise-core` bundles three concerns that do not share an authority:

1. YantrikDB access,
2. MetaClaw skill lookup and injection,
3. an inline `/v1/chat/completions` inference proxy.

The third is disqualifying. The closure contract scopes this project to `skills_only` and lists **runtime interception as explicitly out of scope**. The proxy *is* runtime interception: it receives the client request, mutates the prompt, and forwards to `LLM_API_BASE` (default `http://127.0.0.1:20128/v1`, i.e. OmniRoute). SAMWISE therefore sits synchronously in front of every inference call.

This is not a routing duplication — the proxy selects no provider, identity, model, or fallback, so OmniRoute remains the router. It is a **request-path ownership** violation, which is a different and worse problem: it makes SAMWISE a hard synchronous dependency of all execution while claiming a scope that forbids exactly that.

Two further violations exist at the write boundary:

- `buffer_turn` accepts raw conversational execution material and decides how to persist it. That is trace-to-asset production, which OmniRoute owns solely.
- `claim_unprocessed_traces` has YantrikDB claiming *raw traces* for interpretation, competing with OmniRoute's canonicalization policy.

The service boundary itself is wrong. The lint debt and missing tests are symptoms, not the defect.

## 2. Canonical authority chain

> **OmniRoute observes and canonicalizes. YantrikDB governs evidence and projections. MetaClaw governs procedures. OmniRoute applies approved procedural decisions during execution.**

OmniRoute is an **active authority on every execution**, not a passive upstream logger — all inference continues to route through it. SAMWISE consumes its assets and emits decisions back; it never owns the request path in either direction.

Any SAMWISE-side extraction must be downstream, versioned, and explicitly scoped — never a competing interpretation layer.

## 3. The authority test

For each unit of code, ask: **whose authority does this exercise?**

- Exercises an OmniRoute authority (observation, canonicalization, provider/identity/model selection, retries, failover, request path) → **out**.
- Exercises a SAMWISE authority (evidence, projection, procedure governance) → **may survive**, but must be provenance-preserving and append-only.

Applied to the existing 982 LOC:

| File | LOC | Authority exercised | Verdict |
|---|---:|---|---|
| `proxy.rs` | 166 | Runtime interception; owns request path | **Delete** — violates `skills_only` |
| `main.rs` | 112 | Bootstraps the proxy service | **Delete** — boundary is wrong |
| `circuit_breaker.rs` | 139 | Local sidecar availability | **Survives** as a utility — rename to `YantrikDbAvailabilityBreaker` |
| `errors.rs` | 98 | Error taxonomy | **Survives** |
| `engine_wrapper.rs` | 273 | YantrikDB access, `think()`, governance epoch | **Partial** — adapter and epoch survive; `search_skills` as a proxy feeder does not |
| `server.rs` | 180 | HTTP surface | **Partial** — endpoint *shapes* inform Pass 1/3; this service boundary does not survive |
| `migrations/0001_initial.sql` | 14 | Sidecar state schema | **Review** at Pass 1 |

The naming collision between `circuit_breaker.rs` and OmniRoute's circuit breaking is a documentation defect that blurred the boundary. The mechanism is sidecar resilience and must be named as such.

## 4. Ingest boundary

**YantrikDB begins at the immutable typed-asset boundary and never before it.**

Invalid:

```text
raw turn → SAMWISE buffer → SAMWISE interpretation
```

Valid:

```text
raw execution → OmniRoute canonicalization → immutable typed asset
    → YantrikDB provenance-preserving ingest
```

Decisions:

| Operation | Verdict |
|---|---|
| `buffer_turn` | **Delete.** Duplicates trace-to-asset authority at the write boundary. |
| `claim_unprocessed_traces` | **Delete** under that name and semantic. YantrikDB must never claim raw traces for interpretation. |
| `ingest_asset` | Replacement if needed. Idempotent; accepts only canonicalized, provenance-carrying typed assets. |
| `claim_unprojected_assets` | Replacement if needed. Selects **already canonicalized** OmniRoute assets for deterministic projection maintenance. Must not transform raw execution into evidence, alter source meaning, or compete with canonicalization policy. |

## 5. Pass 5 redefined

Pass 5 is **not** bounded injection performed by SAMWISE. It is the production of a versioned decision artifact that OmniRoute validates and applies:

```text
YantrikDB projection
  → MetaClaw match/evaluate
  → InjectionDecision artifact
  → OmniRoute validates and applies
  → provider execution
```

Authority split:

- **SAMWISE specifies and emits** the versioned `InjectionDecision` contract.
- **OmniRoute implements the consumer/application point**, because applying context to a live model request belongs to the component that owns that request.
- SAMWISE **must not** ship an adapter that intercepts the request "temporarily." Temporary architectural violations become permanent.

### `InjectionDecision` — minimum fields

```text
decision_id
objective_id
skill_version_ids[]
evidence_link_ids[]
policy_bundle_id + policy_version
governance_epoch
budget_requested
budget_granted
canonical_payload_hash
status
uncertainty
expires_at / validity conditions
```

### Mock-consumer honesty constraint

No accessible OmniRoute repository was available to verify that an artifact-consumption surface exists. We therefore do **not** claim it exists.

Pass 5 may benchmark artifact *production* against a deterministic mock consumer. **A passing mock does not close end-to-end runtime integration**, and no completion claim may state or imply otherwise. End-to-end integration closes only against the live OmniRoute consumer.

## 6. Target repository structure

```text
samwise/
  docs/                 normative documents (corrected per §7)
  samwise-evidence/     Pass 0 — pure contract logic
  ralph/                autonomous implementation harness
```

`samwise-evidence` dependencies: `serde`, `serde_json`, `blake3`, `thiserror`. No `tokio`, `axum`, `sqlx`, `reqwest`, or `yantrikdb`.

Rationale: Pass 0 is the artifact every later pass is verified against, so it must be independently testable and free of runtime coupling. It also eliminates the inherited lint debt rather than suppressing it, and reduces a cold build from 4m24s to seconds.

`samwise-core` is preserved on branch `archive/samwise-core-v0`, not deleted and not kept in-tree (an in-tree `archive/` directory would be compiled by CI and the gate). A pointer note in `docs/` records the branch and this rationale.

## 7. Required document corrections

These become false claims the moment `samwise-core` is archived. `AGENTS.md` forbids claiming more than the evidence supports, so they are part of this change, not follow-up work.

| Document | Correction |
|---|---|
| Closure contract §1 | OmniRoute scoped **per gate**, not globally (see §8). Record that `skills_only` forbids inline runtime interception, and that Pass 5 is artifact emission. |
| Closure contract Pass 5 | Rewrite to the artifact-emission shape in §5. |
| `YANTRIKDB.md` §7 | Currently describes a running sidecar with proxy routing and memory buffering. Rewrite to state what actually exists. |
| `README.md` | Structure tree and the `samwise-core` paragraph. |
| Pass 0 plan | ~30 `samwise-core/` path references; `samwise_core::` → `samwise_evidence::`; PowerShell commands → Linux. |
| `.github/workflows/ci.yml` | `working-directory: samwise-core` → `samwise-evidence`. |

## 8. Resolving the OmniRoute scope contradiction

Closure contract §1 lists OmniRoute as out of scope. The closure agent brief §2/§6 makes OmniRoute the owner of trace-to-asset production and requires the audit to cover it. Both are marked authoritative.

A global scope cannot be correct for a per-gate property. The contract's own four gates split cleanly:

- **Gates 1, 3, 4** (functional closure, integrity, safety) are *invariant* claims. They must be proven deterministically, on fixtures. OmniRoute is out of scope for these.
- **Gate 2** (learning improvement; reject `H0`) is a *statistical* claim about real execution. Fixtures cannot prove it — authoring the fixtures would author the improvement. OmniRoute is in scope for this gate.

Fixtures prove the mechanism; live execution proves the compounding. §1 is corrected to state scope per gate.

## 9. What this design explicitly does not close

- It does not implement Passes 1–9.
- It does not prove compounding. Pass 0 makes the contract executable and falsifiable; it demonstrates no learning.
- It does not establish end-to-end OmniRoute integration (see §5).
- Archiving unproven code removes an unproven implementation. It does not reduce the remaining proof obligations.

## 10. Acceptance

This design is satisfied when:

1. `archive/samwise-core-v0` exists and contains the full crate at its pre-archive commit.
2. The main tree contains no inline inference proxy and no `buffer_turn` / `claim_unprocessed_traces`.
3. Every document in §7 is corrected; no document describes a component that no longer exists.
4. `ralph/gate.sh` returns `GATE-GREEN` on the main tree.
5. The `InjectionDecision` field set in §5 is recorded in the contract as the Pass 5 deliverable.
