# Archived Components

## samwise-core (Rust sidecar, v3.0.0)

**Archived:** 2026-08-07
**Branch:** `archive/samwise-core-v0` at `c51c249`
**Restore:** `git checkout archive/samwise-core-v0 -- samwise-core`

### Why

`samwise-core` bundled three concerns that share no authority: YantrikDB access,
MetaClaw skill lookup/injection, and an inline `/v1/chat/completions` inference proxy.

The proxy is disqualifying. This project is scoped to `skills_only`, which lists runtime
interception as explicitly out of scope, and the proxy *is* runtime interception — it
received the client request, mutated the prompt, and forwarded to `LLM_API_BASE`.

This was not a routing duplication. The proxy selected no provider, identity, or model, so
OmniRoute remained the router. It was a *request-path ownership* violation: SAMWISE became a
synchronous dependency of all execution while claiming a scope that forbids exactly that.

The deeper reason: SAMWISE is the name of the composition of YantrikDB, MetaClaw, and
OmniRoute — not a component. A fourth stateful service is precisely what the governing axiom
("YantrikDB = persistent continuity, everything else stateless") forbids.

### What may return

- `circuit_breaker.rs` — as a utility named `YantrikDbAvailabilityBreaker`. It wrapped engine
  calls only and never selected providers, so it did not duplicate OmniRoute's breaker. The
  shared name blurred the boundary; that was a documentation defect.
- `errors.rs` — the error taxonomy.
- `engine_wrapper.rs` — the YantrikDB adapter and governance-epoch handling only.

### What does not return

- `proxy.rs`, `main.rs` — runtime interception and its bootstrap.
- `buffer_turn` — accepted raw conversational material and decided how to persist it,
  duplicating OmniRoute's sole trace-to-asset authority at the write boundary.
- `claim_unprocessed_traces` — YantrikDB must never claim raw traces for interpretation.
  A narrower `claim_unprojected_assets`, operating only on already-canonicalized assets,
  may replace it.

### Also preserved on that branch

Uncommitted Pass 0 work-in-progress produced by the ralph loop (`[lib]` target, `blake3`,
`src/lib.rs`, `src/evidence/`, `tests/evidence_contract.rs`). It never passed the gate — it was
blocked by pre-existing clippy dead-code failures unrelated to the work itself.
