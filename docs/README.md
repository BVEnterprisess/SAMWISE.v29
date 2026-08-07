# Documentation Authority Map

This directory contains the authoritative design, contract, and execution documents for SAMWISE.

## Authority hierarchy

1. [`closure-agent-brief.md`](closure-agent-brief.md) — operational handoff for fresh-repository agents.
2. [`superpowers/specs/2026-08-02-yantrikdb-metaclaw-closure-contract.md`](superpowers/specs/2026-08-02-yantrikdb-metaclaw-closure-contract.md) — normative Asset Contract, invariants, gates, and definition of done.
3. [`GRADIENT.md`](GRADIENT.md) — purpose, philosophy, and compounding objective.
4. [`roles/yantrikdb.md`](roles/yantrikdb.md) — YantrikDB responsibilities and implementation boundary.
5. [`superpowers/plans/2026-08-02-asset-contract-falsification.md`](superpowers/plans/2026-08-02-asset-contract-falsification.md) — ordered Pass 0 implementation plan.

If two documents disagree, the higher-ranked document wins and the contradiction must be reported. Historical or superseded material must not be treated as an active requirement.

## Documentation rules

- Every claim about behavior must identify whether it is verified, implemented, documented, contradicted, absent, or unknown.
- Every threshold, trigger, model version, policy version, and decay rule must have a source and a test or be marked unverified.
- Documentation must distinguish current implementation from target architecture.
- No document may call the system complete based only on startup, retrieval, or skill creation.
