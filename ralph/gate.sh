#!/usr/bin/env bash
# SAMWISE closure gate — CI parity, deterministic, model-independent.
# Exit 0 = green. Any non-zero = red. The agent never decides this.
set -uo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CORE="$REPO/samwise-core"
JOBS="${RALPH_JOBS:-2}"          # 3 cores / 3.8GB: 2 keeps headroom, avoids OOM
CARGO_FLAGS="-j ${JOBS}"

fail() { echo "GATE-FAIL: $1" >&2; exit 1; }
step() { echo "--- gate: $1"; }

cd "$CORE" || fail "no samwise-core"

# 1. Formatting (CI: cargo fmt --all -- --check)
step fmt
cargo fmt --all -- --check 2>&1 | tail -30 || fail "fmt"

# 2. Compilation (CI: cargo check --all-targets)
step check
cargo check --all-targets $CARGO_FLAGS 2>&1 | tail -40
[ "${PIPESTATUS[0]}" -eq 0 ] || fail "check"

# 3. Tests (CI: cargo test --all-targets)
step test
cargo test --all-targets $CARGO_FLAGS 2>&1 | tail -60
[ "${PIPESTATUS[0]}" -eq 0 ] || fail "test"

# 4. Clippy as hard error (CI: -D warnings)
step clippy
cargo clippy --all-targets --all-features $CARGO_FLAGS -- -D warnings 2>&1 | tail -40
[ "${PIPESTATUS[0]}" -eq 0 ] || fail "clippy"

# 5. Documentation integrity (CI job: documentation)
step docs
cd "$REPO"
for f in README.md AGENTS.md docs/GRADIENT.md docs/closure-agent-brief.md \
         docs/superpowers/specs/2026-08-02-yantrikdb-metaclaw-closure-contract.md \
         docs/superpowers/plans/2026-08-02-asset-contract-falsification.md YANTRIKDB.md; do
  [ -f "$f" ] || fail "missing canonical doc: $f"
done
if grep -RInE '(^|[^A-Za-z])(TBD|FIXME)([^A-Za-z]|$)' README.md AGENTS.md YANTRIKDB.md docs >/dev/null 2>&1; then
  fail "unresolved doc placeholder (TBD/FIXME)"
fi

# 6. Contract guard — the epistemic rules AGENTS.md forbids violating.
#    A weak model's favourite shortcut is to make a test pass by deleting it
#    or by stubbing a projection to always return the happy path.
step contract
if [ -d "$CORE/src/evidence" ]; then
  # No silent success paths in projection logic.
  if grep -RInE 'todo!\(|unimplemented!\(|panic!\("not' "$CORE/src/evidence" >/dev/null 2>&1; then
    fail "evidence module contains todo!/unimplemented!"
  fi
  # Partial/Contested must be reachable, not optimised away.
  if [ -f "$CORE/src/evidence/projection.rs" ]; then
    grep -q 'Partial'   "$CORE/src/evidence/projection.rs" || fail "projection never yields Partial"
    grep -q 'Contested' "$CORE/src/evidence/projection.rs" || fail "projection never yields Contested"
  fi
fi
# #[ignore] is how a cornered model silences a red test.
if grep -RIn '#\[ignore\]' "$CORE/tests" "$CORE/src" >/dev/null 2>&1; then
  fail "ignored test found (tests may not be silenced)"
fi

echo "GATE-GREEN"
exit 0
