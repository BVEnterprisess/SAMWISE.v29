# SAMWISE — Pass 0 worker

You are implementing ONE task in the SAMWISE repository. Do that task only. Stop when it is done.

## Your task

**{{TASK_ID}}: {{TASK_TITLE}}**

You are done when this command passes from `samwise-core/`:

```
{{PROOF}}
```

## Required reading (read these files before editing)

1. `docs/superpowers/plans/2026-08-02-asset-contract-falsification.md` — your task's exact interfaces, file map, and steps. This is your primary reference. Find your task ID's section.
2. `AGENTS.md` — the rules you may not break.
3. `docs/superpowers/specs/2026-08-02-yantrikdb-metaclaw-closure-contract.md` §2.1 and §3 — the invariants.

The plan file writes its commands for Windows PowerShell. **Ignore that.** You are on Linux. Use plain `cargo test ...` from `samwise-core/`. Do not use `rustup run stable-x86_64-pc-windows-msvc`.

## Method — test first

1. Write or extend the failing test in `samwise-core/tests/evidence_contract.rs`.
2. Run it. Confirm it fails for the right reason.
3. Write the minimum implementation that makes it pass.
4. Run the proof command above until green.

## Hard rules

- **Never** make a test pass by deleting it, weakening its assertion, or adding `#[ignore]`. The gate rejects this and your work is thrown away.
- **Never** use `todo!()`, `unimplemented!()`, or a stub that returns a happy-path value. Incomplete evidence must produce `Partial` or `Contested` — never a best guess.
- Assets are immutable; evidence, evaluations, and relations are append-only. No setter may mutate a payload or an identity.
- Objective and subjective evaluations stay mathematically separate. A subjective score may never raise objective confidence.
- Projection must be deterministic: same history + same policy version ⇒ same result.
- Do not touch `src/main.rs`, `src/server.rs`, `src/proxy.rs`, `src/engine_wrapper.rs`, or `src/circuit_breaker.rs`. Pass 0 changes no runtime behaviour.
- Do not edit anything under `ralph/`.
- Keep the change scoped to your task. Leave unrelated work alone.

## Before you finish

Run these from `samwise-core/` and fix anything they report:

```
cargo fmt --all
cargo test --all-targets -j 2
cargo clippy --all-targets --all-features -j 2 -- -D warnings
```

`clippy -D warnings` means **any** warning fails the build, including dead code. If you add a public type that nothing uses yet, that is fine — it is reachable from the library target. If you add a private helper nothing calls, remove it.

Do not commit. The harness commits for you once it has verified your work independently.
