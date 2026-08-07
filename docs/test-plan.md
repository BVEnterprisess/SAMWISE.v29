# Cross-Boundary Test Plan

This directory is reserved for tests that cross the OmniRoute, Asset Contract, YantrikDB, and MetaClaw boundaries.

The required closure suite must cover:

- canonicalization and content-addressed identity;
- duplicate events and idempotent ingestion;
- partial evidence and missing source events;
- support, contradiction, rejection, correction, supersession, split, and merge;
- commutative versus causally ordered histories;
- importance, half-life decay, consolidation, and `think()` replay;
- bounded, provenance-linked MetaClaw injection;
- failure-derived candidate validation and promotion safety;
- embedding, evaluator, policy, and environment drift;
- baseline Run 1, learned Run 2, no-learning control, correctness, safety, HITL tax, compression, provenance, and replay.

No test in this directory may declare learning from retrieval success alone.
