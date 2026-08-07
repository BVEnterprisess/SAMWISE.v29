#!/usr/bin/env bash
# SAMWISE closure gate — deterministic, model-independent.
# Exit 0 = green. Any non-zero = red. The agent never decides this.
set -uo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

fail() { echo "GATE-FAIL: $1" >&2; exit 1; }
step() { echo "--- gate: $1"; }

cd "$REPO" || fail "no repo"

# 1. No runtime crate exists in the main tree. samwise-core is archived on
#    archive/samwise-core-v0. When the first crate lands, restore fmt/check/test/clippy
#    here and point CORE at its path.
step no-runtime-crate
[ -d "$REPO/samwise-core" ] && fail "samwise-core reappeared in the main tree; it belongs on archive/samwise-core-v0"

# 2. Canonical documents present
step docs
for f in README.md AGENTS.md docs/GRADIENT.md docs/closure-agent-brief.md \
         docs/superpowers/specs/2026-08-02-yantrikdb-metaclaw-closure-contract.md \
         docs/superpowers/plans/2026-08-02-asset-contract-falsification.md YANTRIKDB.md; do
  [ -f "$f" ] || fail "missing canonical doc: $f"
done

# 3. Placeholder scan (code-block aware)
step placeholders
"$REPO/scripts/check-placeholders.sh" >/dev/null || fail "unresolved doc placeholder (TBD/FIXME)"

# 4. Contract guard — the shortcuts a cornered model reaches for.
step contract
if [ -d "$REPO/samwise-evidence/src" ]; then
  grep -RInE 'todo!\(|unimplemented!\(' "$REPO/samwise-evidence/src" >/dev/null 2>&1 \
    && fail "evidence code contains todo!/unimplemented!"
fi
if [ -d "$REPO/samwise-ingest" ]; then
  grep -RIn '@pytest.mark.skip\|# type: ignore' "$REPO/samwise-ingest" >/dev/null 2>&1 \
    && fail "silenced test or type check in samwise-ingest"
fi

echo "GATE-GREEN"
exit 0
