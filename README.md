# SAMWISE

## Controlled adaptation through evidence

SAMWISE is an architecture for turning execution into reusable structure without rewriting history or pretending uncertain evidence is truth.

The governing objective is:

> **Gradient = Compounding.**

The only outcome derivatives are:

1. decreasing human-in-the-loop coordination tax;
2. increasing compression ratio while preserving equal-or-better correctness and safety.

Everything else—latency, tokens, retries, cost, recall, confidence, provider health, and error rates—is evidence, a guardrail, or a measurement supporting those outcomes.

## Why this exists

The system is designed to metabolize degradation rather than discard it:

- execution failures become evidence and possible corrective-procedure candidates;
- contradictions remain inspectable instead of being silently overwritten;
- entropy becomes evidence about duplication, context bloat, and irrelevant retrieval;
- drift becomes explicit temporal, environmental, model, evaluator, and policy state;
- corrections and evolution append to history instead of changing the past.

Execution may be probabilistic, incomplete, or wrong. Learning must be deterministic, replayable, explainable, and evidence-backed.

“Never wrong” therefore means: the system may fail, but it must never claim more certainty or correctness than its admissible evidence supports.

## System boundary

| Component | Sole authority |
|---|---|
| OmniRoute | execution observation, correlation, canonicalization, trace-to-asset production |
| Evidence-Preserving Asset Contract | immutable identity, provenance, evidence, uncertainty, and policy lineage |
| YantrikDB | durable evidence, importance, decay, recall, conflicts, correction, consolidation, and deterministic projection |
| MetaClaw | procedure validation, skill lifecycle, matching, bounded injection, usage evaluation, and evolution |

The systems cooperate through typed immutable assets. They do not share ownership of trace interpretation or long-term projection.

## What full realization achieves

For a controlled family of similar tasks, a learned Run 2 will measurably outperform Run 1 and a no-learning control while preserving correctness, safety, provenance, uncertainty, and replayability.

The system is not complete when the service starts, memory can be queried, or a skill can be written. It is complete only when the paired benchmark rejects:

```text
H0: learned projection provides no measurable improvement over baseline
```

and demonstrates lower HITL coordination tax plus higher quality-constrained compression.

## Read these files in order

1. [`AGENTS.md`](AGENTS.md) — mandatory operating instructions for agent runtimes.
2. [`docs/closure-agent-brief.md`](docs/closure-agent-brief.md) — exact audit handoff and epistemic contract.
3. [`docs/superpowers/specs/2026-08-02-yantrikdb-metaclaw-closure-contract.md`](docs/superpowers/specs/2026-08-02-yantrikdb-metaclaw-closure-contract.md) — normative closure specification.
4. [`docs/GRADIENT.md`](docs/GRADIENT.md) — the why and compounding philosophy.
5. [`docs/roles/yantrikdb.md`](docs/roles/yantrikdb.md) — YantrikDB's current role and implementation boundary.
6. [`docs/superpowers/plans/2026-08-02-asset-contract-falsification.md`](docs/superpowers/plans/2026-08-02-asset-contract-falsification.md) — Pass 0 implementation plan.

## Current repository status

This repository contains no runtime code. SAMWISE is the composition of three independently
shipped systems — YantrikDB (evidence and projections), MetaClaw (procedural skills), and
OmniRoute (execution and enforcement). It is delivered as contracts, wiring, and invariants.

The former `samwise-core` sidecar is archived on `archive/samwise-core-v0`; see
`docs/ARCHIVE.md`.

It is runtime-hardened but not closure-complete. The remaining proof obligations are the Asset Contract implementation, OmniRoute adapter, deterministic projection fixtures, MetaClaw evidence adapter, independent outcome evaluators, and the paired compounding benchmark.

## Repository map

```text
.
├── AGENTS.md          # Operating contract — read automatically by Codex / OpenCode
├── README.md
├── docs/
│   ├── source/        # Primary source material (verbatim, unmodified)
│   ├── roles/         # Who owns what
│   ├── superpowers/   # Specs and implementation plans
│   ├── GRADIENT.md    # The objective
│   ├── closure-agent-brief.md
│   ├── test-plan.md
│   └── ARCHIVE.md
├── ralph/             # Autonomous implementation harness
└── scripts/
```

## Non-negotiable rule

Every implementation decision must answer:

> Does this improve the system's ability to convert execution into reusable structure while preserving evidence and replayability?

If not, it does not belong in the core.
