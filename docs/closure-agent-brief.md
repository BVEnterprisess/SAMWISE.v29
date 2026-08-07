# Closure Agent Brief: Controlled Adaptation Through Evidence

> **PARTIALLY SUPERSEDED — 2026-08-07.**
>
> **Still authoritative:** the audit classification taxonomy (§6) and the closure gate (§7).
> Every claim about this system is still classified `VERIFIED` / `IMPLEMENTED` / `DOCUMENTED` /
> `CONTRADICTED` / `ABSENT` / `UNKNOWN`, and closure still requires rejecting `H0` with all
> conditions holding simultaneously.
>
> **Superseded:** §2's ownership diagram and §6's fresh-repository instructions. SAMWISE is now
> understood as the *composition* of YantrikDB + MetaClaw + OmniRoute, not a service that owns a
> request path. See `superpowers/specs/2026-08-06-authority-boundary-correction-design.md` and
> `superpowers/specs/2026-08-06-governor-control-loop-design.md`.

Status: authoritative handoff specification, 2026-08-02

This document is the operating contract for any fresh-repository audit or implementation agent. It exists to remove goal ambiguity and information asymmetry.

## 1. Why this system exists

This is not primarily a memory system. It is an architecture for controlled adaptation through evidence.

The system must metabolize degradation into bounded structural improvement:

- failure becomes evidence and a candidate for corrective procedure;
- entropy becomes evidence about context bloat, redundancy, and retrieval quality;
- drift becomes evidence about temporal relevance, changed environments, and model/policy sensitivity;
- contradiction becomes explicit evidence, never silent overwrite;
- correction becomes append-only lineage, never historical revision.

The governing objective is:

> **Gradient = Compounding.**

The only outcome derivatives are:

1. decreasing human-in-the-loop coordination tax;
2. increasing compression ratio subject to equal-or-better correctness and safety.

Latency, token count, retries, cost, recall score, confidence, and provider health are evidence, constraints, guardrails, or diagnostic measurements. They are not independent objectives.

## 2. Exact ownership boundary

```text
OmniRoute
  observe execution
  correlate events
  canonicalize raw observations
  produce immutable typed assets
       |
       v
Evidence-Preserving Asset Contract
  immutable identity, provenance, evidence, uncertainty, policy lineage
       |
       v
YantrikDB
  durable evidence, importance, decay, recall, links, conflicts,
  corrections, supersession, consolidation, deterministic projection, think()
       |
       v
MetaClaw
  validate procedure candidates, promote skills, match, bound injection,
  record usage, evaluate outcomes, evolve skills
       |
       v
OmniRoute execution and independent outcome evaluation
```

No component may silently duplicate another component's authority. MetaClaw does not independently reinterpret raw traces. YantrikDB does not replace OmniRoute's observation engine. OmniRoute does not decide long-term knowledge truth.

## 3. The epistemic contract: never wrong, never claiming right

“Never wrong” does not mean that execution cannot fail or that an agent cannot produce an incorrect answer. Execution is probabilistic and may be incomplete or wrong.

It means the learning layer must never convert insufficient or conflicting evidence into an unjustified claim of correctness.

The system may only expose a claim when it can return all of the following:

```text
claim
asset_id
source_event_ids
evidence_link_ids
objective_evaluations
subjective_evaluations, if any
policy_bundle_id and policy_version
canonical_payload_hash
provenance chain
status and uncertainty
correction/supersession history
```

The system must abstain or expose an explicit non-final state when evidence is insufficient:

```text
partial     = required evidence is missing
contested   = supporting and contradicting evidence coexist
rejected    = an authoritative objective evaluation rejects the asset
supported   = the current policy finds sufficient admissible support
```

`supported` is not an eternal truth claim. It is a reproducible projection under a named policy at a named time. Confidence is calibrated uncertainty, not proof. No projection may emit a stronger status by deleting evidence, hiding provenance, duplicating traces, or inflating confidence.

Formal minimum rule:

```text
Claim(asset, t, policy)
  => provenance(asset) is complete
  AND canonical_hash(asset) is valid
  AND policy(policy) is immutable and available
  AND status(asset, t, policy) != partial
  AND status(asset, t, policy) != contested
  AND status(asset, t, policy) != rejected
```

If any premise is false, the system must abstain, downgrade, or surface the unresolved state. It must not guess silently.

## 4. Signal-source treatment

### 4.1 Failure

Failure signals include provider errors, tool errors, retries, timeouts, aborts, partial outcomes, failed objective evaluators, human corrections, and explicit “do not repeat” feedback.

Lifecycle:

```text
observed failure
  -> immutable failure/observation asset
  -> evidence links to the execution and failed conditions
  -> objective evaluation of the failure and recovery attempt
  -> procedure_candidate only if a corrective pattern is evidenced
  -> MetaClaw validation and promotion policy
  -> bounded future injection
  -> independent Run 2 outcome evaluation
```

A failure-derived candidate is a hypothesis, not a skill. A failed trajectory may teach what not to do; it becomes reusable procedure only when the proposed correction has positive, provenance-linked outcome evidence and passes safety policy.

### 4.2 Entropy

Entropy signals include context size, injected artifact count, duplicate/near-duplicate assets, retrieval overlap, irrelevant injection, truncation, compression savings, token budget pressure, retrieval misses, and human coordination caused by context confusion.

YantrikDB handles durable semantic consolidation and decay while preserving every source and relation. MetaClaw handles bounded procedural selection and injection. Consolidation or compression is valid only when:

```text
correctness_after >= correctness_before
AND safety_after >= safety_before
AND coordination_tax_after <= coordination_tax_before
```

Lower token count alone is not improvement. Deleting useful evidence, reducing answer quality, or suppressing uncertainty is a regression.

### 4.3 Drift

Drift signals include age, half-life decay, changed task distribution, changed provider/model behavior, changed tool/environment state, new contradictions, changed evaluator outcomes, embedding-model changes, tokenizer changes, and policy-version changes.

Every drift-sensitive computation must record:

```text
observation_time
effective_time, if different
embedding_model_id and version, if used
policy_bundle_id and version
thresholds and calibration version
source environment/model/provider identity
```

Changing an embedding model, policy, evaluator, or task environment must not silently reuse old thresholds. It requires an explicit migration, recalibration, reindex, or an auditable declaration that the old projection is no longer comparable.

## 5. Immutable learning loop

```text
execute
  -> observe immutable events
  -> canonicalize and content-address
  -> derive immutable typed assets
  -> append evidence and evaluations
  -> project under immutable versioned policy
  -> retrieve only admissible projected state
  -> bounded MetaClaw injection
  -> execute paired future run
  -> independently evaluate outcome
  -> append correction/evolution evidence
  -> replay and compare against baseline/control
```

Assets are immutable. Evidence links, evaluations, corrections, relations, and policies are append-only. A split, merge, correction, or evolution creates a new immutable object linked to its predecessors.

Required replay invariant:

```text
Projection(history, policy_version) ==
Projection(history, policy_version)
```

across process restarts, storage reloads, and equivalent event orderings where commutativity is explicitly declared. Event order is otherwise causal and non-commutative by default.

## 6. Fresh-repository agent instructions

The agent must pull a fresh copy of `BVEnterprisess/SAMWISE.v29` and audit it independently. It must not infer implementation from this conversation and must not modify the repository during the audit.

The agent must classify every relevant claim as exactly one of:

```text
VERIFIED      present in code and covered by a passing test
IMPLEMENTED   present in code but lacking adequate proof
DOCUMENTED    specified but not implemented or verified
CONTRADICTED  code/docs violate the closure contract
ABSENT        no evidence found
UNKNOWN       repository cannot establish the claim
```

The audit must cover:

1. OmniRoute trace capture, correlation, canonicalization, and asset production.
2. Asset immutability, content addressing, provenance, evidence links, and policy lineage.
3. YantrikDB validation, idempotency, importance, decay, recall, consolidation, conflict, correction, supersession, and `think()`.
4. MetaClaw failure-candidate validation, skill lifecycle, matching, bounded injection, usage evaluation, and evolution.
5. Explicit treatment of failure, entropy, drift, partial evidence, non-commutative histories, duplicate traces, crash recovery, embedding changes, and policy changes.
6. The paired benchmark: baseline Run 1, learned Run 2, no-learning control, independent evaluator, H0, HITL tax, compression, correctness, safety, provenance, and replay.

For every concrete parameter claimed by documentation or implementation—threshold, pool size, decay rate, similarity cutoff, trigger, model version, or retention rule—the agent must report its source file, test, and whether it is authoritative or merely historical.

The agent's final output must contain only:

```text
1. Executive verdict: CLOSED / NOT CLOSED / BLOCKED
2. Verified facts with file:line evidence
3. Contradictions and unsafe claims
4. Missing proof obligations
5. Exact ordered closure plan
6. Pass/fail definition for each step
7. Final information-asymmetry list
```

The agent must not declare “learning,” “compounding,” “correct,” or “production-ready” from service startup, retrieval success, or documentation alone.

## 7. Closure gate

The loop is closed only when a controlled benchmark rejects:

```text
H0: learned projection provides no measurable improvement over baseline
```

and all of these hold simultaneously:

- correctness is equal or better;
- safety is equal or better;
- HITL coordination tax decreases;
- quality-constrained compression improves;
- the learned run beats the no-learning control;
- provenance is complete for every injected improvement;
- replay reproduces the same projection;
- duplicate, partial, contradictory, stale, crash-recovery, and policy/model-change tests pass;
- no claim exceeds the evidence available at the time it is made.

Anything less is progress, not closure.
