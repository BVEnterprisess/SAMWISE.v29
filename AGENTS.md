# Agent Operating Contract

This repository is being reviewed and implemented by multiple independent agent runtimes. These rules eliminate information asymmetry.

## Required reading order

Before analyzing or changing anything, read:

1. `README.md`
2. `docs/closure-agent-brief.md`
3. `docs/superpowers/specs/2026-08-02-yantrikdb-metaclaw-closure-contract.md`
4. `docs/GRADIENT.md`
5. `YANTRIKDB.md`
6. `docs/superpowers/plans/2026-08-02-asset-contract-falsification.md`

The closure contract is normative. The closure agent brief defines the audit output. The plan defines implementation order; it does not prove completion.

## Goal

Build controlled adaptation through evidence:

```text
Gradient = Compounding
```

The only outcome derivatives are decreasing HITL coordination tax and increasing quality-constrained compression ratio. Correctness and safety are hard constraints, not optional metrics.

## Ownership rules

- OmniRoute owns execution observation and trace-to-asset production.
- The Asset Contract owns immutable identity, provenance, evidence, uncertainty, and policy lineage.
- YantrikDB owns durable evidence and deterministic projections.
- MetaClaw owns procedural validation, bounded injection, usage evaluation, and evolution.
- No agent may create a second trace interpreter or bypass provenance with direct prompt memory.

## Epistemic rules

- Execution can be wrong; learning cannot claim more than evidence supports.
- `partial`, `contested`, and `rejected` state must remain explicit.
- `supported` is policy- and time-scoped, not eternal truth.
- Assets are immutable. Evidence, evaluations, corrections, relations, and policies are append-only.
- Duplicate traces may not inflate support or confidence.
- Objective and subjective evaluations remain separate.
- Projection must be deterministic and replayable under a named policy version.
- Failure-derived procedures are hypotheses until corrective evidence passes objective and safety evaluation.

## Audit classification

Every claim must be classified as exactly one of:

```text
VERIFIED      code exists and a relevant test passes
IMPLEMENTED   code exists but adequate proof is missing
DOCUMENTED    specified but not implemented or verified
CONTRADICTED  code or documentation violates the contract
ABSENT        no evidence found
UNKNOWN       repository evidence is insufficient
```

Never infer `VERIFIED` from documentation, service startup, retrieval success, or a populated database.

## Change rules

- Preserve the ownership boundary.
- Prefer the smallest change that strengthens an executable invariant.
- Do not add architecture without a named proof obligation.
- Do not delete evidence, provenance, corrections, or contradictions to make a result look cleaner.
- Keep changes scoped and leave unrelated work untouched.
- Run the narrowest relevant checks, then report failures honestly.

## Required completion language

Do not use “learning,” “compounding,” “correct,” or “production-ready” unless the paired benchmark, provenance checks, deterministic replay, and non-regression gates pass.

When reporting, include exact file paths, line references where possible, commands run, observed results, and unresolved information asymmetry.
