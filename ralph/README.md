# Ralph Loop — SAMWISE Pass 0 closure harness

Harness-agnostic overnight driver. **The model proposes; `gate.sh` disposes.**

Each iteration: pick the first unproven task → run the agent on it → require the
task's own proof command to pass → require the full CI-parity gate to pass →
commit. Anything less and the working tree is `git reset --hard`. A weak model
cannot fake progress and cannot corrupt the repo.

## Launch (overnight)

```bash
cd /home/johnh/services/samwise
RALPH_AGENT=codex RALPH_HOURS=10 nohup ./ralph/run.sh > ralph/state/nohup.log 2>&1 &
```

With an explicit model:

```bash
RALPH_AGENT=codex RALPH_MODEL=gpt-5.6-luna RALPH_HOURS=10 ./ralph/run.sh
```

OpenCode instead:

```bash
RALPH_AGENT=opencode RALPH_MODEL=deepseek-v4-flash ./ralph/run.sh
```

## Config

| Env | Default | Meaning |
|---|---|---|
| `RALPH_AGENT` | `codex` | `codex` or `opencode` |
| `RALPH_MODEL` | agent default | model id |
| `RALPH_MAX_ITER` | `200` | hard iteration cap |
| `RALPH_HOURS` | `10` | wall-clock budget |
| `RALPH_MAX_FAILS` | `4` | consecutive fails before a task is marked BLOCKED and skipped |
| `RALPH_JOBS` | `2` | cargo `-j`. Box has 3 cores / 3.8 GB; **do not raise to 3** — cargo+rustc will OOM |

## Monitor

```bash
tail -f ralph/state/run.log          # one line per iteration
cat    ralph/state/ledger.md         # what closed, with commit SHAs
cat    ralph/state/blocked           # tasks that beat the model
tail -40 ralph/state/last-fail.log   # why the current task is red
git log --oneline                    # committed work
```

## Stopping

```bash
pkill -f ralph/run.sh
```

Work already committed is safe. An in-flight iteration is discarded, not half-applied.

## Exit conditions

- **0** — every task proven *and* the final gate is green → Pass 0 closed.
- **1** — all proofs pass but the full gate is red (see `state/final-gate.log`).
- otherwise — iteration cap or wall-clock budget reached; re-launch to continue
  where it stopped (proofs are re-derived from the repo, so state is resumable).

## What the gate enforces

1. `samwise-core` is absent from the main tree (archived on `archive/samwise-core-v0`)
2. canonical documents present
3. `scripts/check-placeholders.sh` — no unresolved placeholders outside fenced code blocks
4. contract guard — no `todo!()`/`unimplemented!()` in evidence code, no silenced tests

Rust checks (`fmt`, `check`, `test`, `clippy -D warnings`) return when the first runtime
crate lands. Restore them in `gate.sh` and point `CORE` at the new crate.


## Adding work beyond Pass 0

Append to `tasks.txt` as `ID | TITLE | PROOF_COMMAND`. The proof command must be
specific enough that it cannot pass by accident. Passes 1–9 of the closure
contract have gate definitions but no task-level plan yet — write the plan first
(`docs/superpowers/plans/`), then encode its tasks here.
