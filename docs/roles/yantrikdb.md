# YantrikDB: Durable Evidence and Deterministic Projection

Status: authoritative role document, 2026-08-02

This document defines YantrikDB's role in SAMWISE. The [closure contract](../superpowers/specs/2026-08-02-yantrikdb-metaclaw-closure-contract.md) is normative when this document and an implementation detail disagree. The [closure agent brief](../closure-agent-brief.md) is the handoff contract for independent reviewers.

## 1. Why YantrikDB exists

YantrikDB is not a generic memory store and it is not an oracle of truth. It is the durable evidence and projection substrate that lets the system metabolize execution degradation into controlled adaptation without rewriting history.

Its purpose is to preserve and govern:

- what was observed;
- where it came from;
- how important it was at the time;
- how its relevance decays;
- what supports or contradicts it;
- what corrected or superseded it;
- which policy produced the current projection;
- what may be safely recalled or exposed to a downstream procedure engine.

The system may execute incorrectly. YantrikDB must not represent insufficient evidence as established correctness.

## 2. Boundary with OmniRoute and MetaClaw

```text
OmniRoute
  owns execution observation, correlation, canonicalization,
  and trace-to-asset production
       |
       v
Evidence-Preserving Asset Contract
  immutable identity, provenance, evidence, uncertainty, policy lineage
       |
       v
YantrikDB
  owns durable evidence, weighting, decay, recall, links, conflicts,
  correction, supersession, consolidation, and deterministic projection
       |
       v
MetaClaw
  consumes projected procedure candidates and governs skill policy,
  bounded injection, usage evaluation, and evolution
```

YantrikDB must not recreate OmniRoute's trace interpreter. MetaClaw must not bypass YantrikDB with raw trace extraction or provenance-free prompt injection.

## 3. Canonical records

### 3.1 Immutable asset

An asset is a typed, content-addressed interpretation of one or more observed events. It is immutable and is not itself “truth.” A new interpretation creates a new asset linked to its predecessors.

Minimum fields:

```typescript
interface Asset {
  asset_id: string;                 // content-addressed identity
  asset_type:
    | "observation"
    | "fact"
    | "failure"
    | "failure_pattern"
    | "procedure_candidate"
    | "metric"
    | "constraint"
    | "decision"
    | "environment_state"
    | "skill_reference";
  canonical_payload_hash: string;
  schema_version: string;
  source_event_ids: string[];
  source_system: "omniroute";
  observed_at: string;
  created_at: string;
  uncertainty: Uncertainty;
  policy_bundle_id: string;
}
```

One event may produce zero or many assets. Many events may support one asset through append-only evidence links. Assets are never updated in place.

### 3.2 Evidence and evaluations

```typescript
interface EvidenceLink {
  link_id: string;
  asset_id: string;
  event_id?: string;
  related_asset_id?: string;
  relation: "supports" | "contradicts" | "explains" | "uses";
  recorded_at: string;
  policy_bundle_id: string;
}

interface Evaluation {
  evaluation_id: string;
  asset_id: string;
  evaluator_class: "objective" | "subjective";
  evaluator_id: string;
  result: "pass" | "fail" | "abstain";
  score?: number;
  rationale: string;
  evidence_ids: string[];
  policy_bundle_id: string;
  created_at: string;
}
```

Objective evaluations govern correctness and safety decisions. Subjective evaluations may inform usefulness or preference but must not silently raise objective confidence.

### 3.3 Projection status

Projection status is derived from immutable history, not stored as mutable truth:

```text
status(asset, time, policy)
  = Projection(
      asset,
      evidence_links up to time,
      evaluations up to time,
      relations up to time,
      immutable policy bundle
    )
```

The minimum status set is:

| Status | Meaning | Permitted downstream behavior |
|---|---|---|
| `partial` | required evidence is missing | no correctness claim; no promotion |
| `contested` | admissible support and contradiction coexist | surface conflict; no automatic promotion |
| `rejected` | authoritative objective evaluation rejects the asset | do not inject or promote |
| `supported` | current policy finds sufficient admissible support | eligible for policy-scoped use |

`supported` is a time- and policy-scoped projection, not an eternal truth claim.

## 4. How degradation becomes signal

### 4.1 Failure

Failure signals include provider errors, tool errors, retries, timeouts, aborted work, partial outcomes, failed objective evaluations, human corrections, and explicit “do not repeat” feedback.

YantrikDB stores the failure evidence and its links. A failure may produce a `failure_pattern` or `procedure_candidate` asset, but a candidate is only a hypothesis. MetaClaw may promote a procedure only after provenance-linked corrective evidence passes objective success and safety policies.

Required chain:

```text
failure observation
  -> immutable failure asset
  -> evidence and condition links
  -> recovery evaluation
  -> procedure candidate, if warranted
  -> MetaClaw validation and promotion
  -> bounded future use
  -> independent outcome evaluation
```

### 4.2 Entropy and redundancy

Entropy signals include repeated or near-duplicate assets, retrieval overlap, irrelevant recall, context size, injected artifact count, truncation, compression savings, token budget pressure, retrieval misses, and coordination caused by context confusion.

YantrikDB may consolidate redundant knowledge into a canonical projection, but consolidation must preserve every source asset, evidence link, relation, correction, and policy version. The canonical representation is new derived state; it does not erase the source history.

Compression is valid only when the outcome is equal-or-better:

```text
correctness_after >= correctness_before
AND safety_after >= safety_before
AND coordination_tax_after <= coordination_tax_before
```

### 4.3 Drift

Drift signals include age, half-life decay, changed task distribution, changed provider/model behavior, changed environment state, new contradictions, changed evaluator outcomes, embedding-model changes, tokenizer changes, and policy-version changes.

Every drift-sensitive projection records:

```text
observation_time
effective_time, when different
embedding_model_id and version, when used
policy_bundle_id and version
threshold and calibration version
provider/model/tool/environment identity
```

Changing an embedding model, evaluator, policy, or task environment requires an explicit migration, recalibration, reindex, or comparability declaration. Old thresholds must not be silently reused.

## 5. YantrikDB lifecycle

### Ingest

1. Accept only typed, provenance-preserving asset/evidence writes.
2. Canonicalize the payload using the declared schema version.
3. Verify the content hash and source-event references.
4. Apply idempotency before creating a new record.
5. Append evidence, evaluations, and relations; never overwrite history.
6. Record the policy bundle and timestamps used for the write.

### Think and maintenance

`think()` is a deterministic maintenance operation under an explicit `ThinkConfig` and policy bundle. It may:

- apply importance calibration;
- apply half-life decay;
- update recall indexes;
- identify redundant candidates for consolidation;
- identify contradictions and unresolved conflicts;
- create correction, supersession, split, or merge proposals;
- emit re-verification triggers;
- advance a governance epoch.

It may not rewrite an old asset, delete a contradiction to make a projection look clean, or promote a procedure without admissible evidence.

### Recall

Recall returns policy-scoped projected state, not an unqualified dump of records. Each returned item must be traceable to its asset identity, current status, evidence, uncertainty, importance, decay state, and policy version.

### Correction and supersession

Corrections and supersessions are append-only. The original asset remains queryable. A correction creates a new asset or evaluation with links to the predecessor and a reason for the change.

## 6. Required invariants

1. Duplicate source events cannot inflate support, confidence, or importance.
2. Missing evidence produces `partial`, never a silent best guess.
3. Support plus unresolved contradiction produces `contested`.
4. Objective and subjective evaluations remain mathematically separate.
5. Every projection is reproducible from the same history and policy version.
6. Append order is causal and non-commutative by default; commutativity must be explicitly declared and tested.
7. No provenance-free item may be injected into MetaClaw.
8. No correction or consolidation may destroy source history.
9. Policy, evaluator, embedding, and threshold versions are part of comparability.
10. Confidence is uncertainty calibration, not proof of correctness.

## 7. Current repository implementation status

This repository contains **no runtime code**. It holds the normative documents, the
implementation plans, and the autonomous implementation harness.

The former `samwise-core` Rust sidecar has been archived to branch
`archive/samwise-core-v0` (see `docs/ARCHIVE.md`). Its inline inference proxy was runtime
interception, which `skills_only` places out of scope.

SAMWISE is the composition of three independently shipped systems — YantrikDB, MetaClaw,
and OmniRoute. It is delivered as contracts, wiring, and invariants, not as a service. What
remains to be built is the connective tissue between them, not a new daemon.

Nothing in this repository constitutes proof of closure. The asset contract, the ingestion
wires, the skill promotion gate, and the paired compounding benchmark are all outstanding.

## 8. Completion standard

YantrikDB is complete for this objective only when the full system can demonstrate:

- a controlled Run 1, learned Run 2, and no-learning control;
- equal-or-better correctness and safety;
- lower HITL coordination tax;
- higher quality-constrained compression;
- complete provenance for every injected improvement;
- deterministic replay of the same projection;
- explicit handling of duplicate, partial, contradictory, stale, crash-recovered, and model/policy-changed histories.

Service startup, retrieval success, a populated database, or a successful skill write is not evidence of compounding.
