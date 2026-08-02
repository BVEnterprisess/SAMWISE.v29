# YantrikDB + MetaClaw Closure Contract

**Status:** Proposed for review  
**Date:** 2026-08-02  
**Scope:** YantrikDB + MetaClaw closed loop, `skills_only` mode  
**Out of scope:** OmniRoute, runtime interception, multi-modal learning, distributed consensus

## 1. Product objective

Build a persistent learning layer that demonstrably improves future agent executions from past executions while preserving provenance, uncertainty, correction history, and auditability.

The system is complete only when a controlled benchmark proves that a second execution of a similar task is measurably better than the first because of what the first execution taught the system.

## 2. System boundary

YantrikDB remains the authoritative cognitive memory substrate. It owns durable records, embeddings, importance calibration, half-life decay, recall scoring, provenance, idempotency, links, conflicts, corrections, supersession, and cognition/maintenance through `think()`.

MetaClaw remains the procedural skill layer. It owns trace analysis, candidate skill extraction, skill evidence, skill matching, bounded injection, usage outcomes, deviations, lifecycle decisions, and skill version evolution.

Neither subsystem is reimplemented. The integration owns the contracts and evidence linking them:

```text
objective
  -> YantrikDB knowledge recall
  -> MetaClaw skill matching
  -> bounded context injection
  -> agent execution trace
  -> YantrikDB knowledge/outcome update
  -> MetaClaw evidence/usage update
  -> YantrikDB think/maintenance
  -> future improved execution
```

## 3. Non-negotiable invariants

1. A retry with the same idempotency key cannot create duplicate evidence, duplicate usage, or duplicate corroboration.
2. Raw importance, calibrated importance, certainty, decay score, semantic match score, and skill success rate are distinct values.
3. A correction never destroys the predecessor record. The correction, successor, reason, actor, and timestamp remain queryable.
4. Contradictions remain inspectable until explicitly resolved or superseded.
5. Every injected fact and skill is explainable through durable provenance.
6. Skills and knowledge records retain version/history links.
7. A process restart cannot lose a committed trace, knowledge update, skill update, or usage outcome.
8. If no relevant knowledge or skill exists, the learning layer is baseline-equivalent: it adds no behavior-changing context and causes no measurable regression.
9. Failed or rejected evidence cannot increase confidence, importance, success rate, or validation status.
10. `think()` is an explicit, observable maintenance operation. Its configuration, inputs, outputs, errors, and resulting mutations are recorded.

## 4. Acceptance gates

All four gates are required. Passing one gate does not imply passing another.

### Gate 1: Functional closure

The following flow must complete in an automated test:

```text
execute
  -> persist execution trace
  -> analyze trace
  -> update YantrikDB knowledge
  -> create or update MetaClaw skill evidence
  -> validate/activate skill
  -> retrieve knowledge and skill for a similar objective
  -> inject within budget
  -> execute again
  -> persist outcome
  -> update usage and lifecycle
```

**Pass criteria:**

- Every arrow completes through a typed interface.
- Every persisted object can be retrieved by stable identifier.
- Every rejected operation returns a typed, deterministic error.
- No stage silently drops an error or silently substitutes unverified data.
- The full flow passes from a clean database and after a restart.

### Gate 2: Learning improvement

The benchmark uses paired executions: a cold Run 1 and a warm Run 2 of the same or closely matched task. The model, tool set, node, task input, and external fixtures are controlled. Run 2 may use only information persisted from Run 1 or earlier approved baseline knowledge.

The initial acceptance profile is 50 paired scenarios across at least five task families, with ten variants per family. Thresholds can be tightened after the first measured baseline but cannot be weakened without an explicit contract revision.

Run 2 must meet all aggregate thresholds:

| Metric | Required result |
|---|---:|
| Median latency | At least 10% lower than Run 1 |
| P95 latency | At least 5% lower than Run 1 |
| Median total tokens | At least 15% lower than Run 1 |
| Retry rate | At least 20% lower than Run 1 |
| Human interventions | At least 20% lower than Run 1 |
| Task success rate | Not lower by more than 2 percentage points |
| Scenario non-regression | At least 90% of pairs do not worsen on any safety-critical metric |

The benchmark must report paired values, medians, p95 values, deltas, confidence intervals, and per-scenario results. Aggregate improvement cannot hide a catastrophic individual regression.

### Gate 3: Integrity and explainability

For every injected fact or skill, the audit record must expose:

- source record ID;
- source trace ID;
- skill ID and version, when applicable;
- lifecycle status at injection time;
- namespace, domain, department, and environment scope;
- raw importance and calibrated importance;
- certainty/confidence;
- half-life and current decay score;
- retrieval/match score and reason;
- creation, verification, correction, and last-use timestamps;
- predecessor/successor or contradiction links;
- the execution outcome that caused the current update.

**Pass criteria:**

- 100% of injected items in the benchmark have complete provenance.
- 100% of corrections preserve predecessor history.
- 100% of contradictions remain queryable.
- Duplicate traces do not change evidence counts or success statistics.
- An auditor can reconstruct why each injected item was selected and what changed afterward.

### Gate 4: Safety and non-regression

The system must reject or quarantine:

- malformed traces;
- incomplete outcome records;
- duplicate evidence;
- provenance-free skill definitions;
- impossible lifecycle transitions;
- confidence inflation from retries;
- injection over the token budget;
- stale, deprecated, superseded, or scope-incompatible skills;
- corrections that overwrite history;
- knowledge that violates hard governance constraints.

**Pass criteria:**

- 100% of adversarial fixture cases produce the expected rejection/quarantine result.
- 100 crash-injection runs during writes recover all committed records with zero loss and zero duplicate commits.
- 32 concurrent writers and 100 concurrent readers preserve invariants and complete without database corruption.
- No-knowledge control runs are statistically baseline-equivalent and never receive behavior-changing injected context.

## 5. Narrow implementation passes

Each pass has its own artifact, tests, and exit gate. Work does not advance when a pass fails.

### Pass 0 — Contract fixtures and measurement harness

Define versioned fixtures for traces, skills, outcomes, corrections, contradictions, and no-knowledge controls. Implement metric collection before learning behavior.

**Done when:** fixtures validate/reject deterministically; benchmark output includes all Gate 2 metrics; no production behavior is changed.

### Pass 1 — Trace contract and durable ingestion

Implement the canonical execution trace, idempotent ingestion, size limits, schema validation, and restart recovery.

**Done when:** 10,000 fixture traces persist and reload with zero loss, zero duplicates, and deterministic rejection of invalid inputs.

### Pass 2 — YantrikDB knowledge update adapter

Map trace facts, decisions, failures, constraints, and outcomes into YantrikDB records using its validation, provenance, calibrated importance, embeddings, half-life, idempotency, and correction/link mechanisms.

**Done when:** each mapped record exposes its source trace and preserves raw/calibrated importance, certainty, decay, and correction history.

### Pass 3 — MetaClaw skill evidence adapter

Persist skill identity, objective, trigger, steps, evidence, scope, status, version, usage counters, and lifecycle history while keeping YantrikDB procedural memory authoritative for durable skill content.

**Done when:** candidate, validation, activation, stale, deprecation, and supersession transitions are durable and illegal transitions are rejected.

### Pass 4 — Minimal extraction

Implement explicit-procedure extraction first. Add repeated-sequence extraction only after the first strategy is fully tested. Deduplicate by objective, trigger, scope, and step similarity.

**Done when:** known fixtures produce exact candidates; duplicate evidence merges without inflating frequency or confidence.

### Pass 5 — Retrieval and bounded injection

Retrieve YantrikDB knowledge and MetaClaw skills separately, apply scope/status/decay/confidence filters, rank deterministically, and format a strict token-budgeted injection payload.

**Done when:** ranking is repeatable, excluded records never inject, provenance is attached to every item, and the hard budget is never exceeded.

### Pass 6 — Outcome feedback

Record skill use, followed steps, successful steps, deviations, retries, duration, human intervention, and overall outcome. Feed appropriate signals to YantrikDB reinforcement/calibration without conflating them with importance.

**Done when:** repeated identical outcomes are idempotent; positive and negative outcomes produce the expected independent updates.

### Pass 7 — Think and maintenance integration

Make `think()` configuration explicit and observable. Verify decay, conflict detection, consolidation, triggers, correction/supersession, and any pattern analysis relevant to the learning loop.

**Done when:** every enabled `think()` operation has an auditable result, and maintenance mutations are covered by restart and adversarial tests.

### Pass 8 — Behavioral closure benchmark

Run the paired cold/warm benchmark and evaluate Gates 1–4 together.

**Done when:** all functional, learning, integrity, and safety thresholds pass in a clean repeatable run.

### Pass 9 — Release hardening

Add CI, reproducible build, migration checks, backup/restore, operational health, structured audit export, and deployment documentation.

**Done when:** a clean checkout can build, test, run the benchmark, and produce the evidence package with one documented command sequence.

## 6. Completion evidence package

A completion claim must include:

1. commit and dependency lock state;
2. schema and migration versions;
3. test counts and coverage;
4. benchmark dataset definition;
5. Run 1/Run 2 raw results;
6. aggregate metric report;
7. per-scenario regression report;
8. provenance/audit sample;
9. crash/restart/concurrency results;
10. known limitations and explicitly excluded capabilities.

No single green health check, retrieval result, or unit-test summary is sufficient evidence of closure.

## 7. Final definition of done

The YantrikDB + MetaClaw loop is closed when the completion evidence package proves that successful agent executions improve future similar executions under controlled paired testing, while all four gates pass and every improvement remains attributable, uncertainty-aware, correction-preserving, duplicate-safe, restart-safe, and bounded by safety constraints.
