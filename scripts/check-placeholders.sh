#!/usr/bin/env bash
# Reject unresolved placeholders in authoritative docs.
#
# Code is skipped, prose is checked. A doc that *documents* a grep pattern
# mentioning TBD/FIXME contains no unresolved placeholder, so both fenced
# blocks and inline `backtick` spans are stripped before matching.
set -euo pipefail

FILES=(README.md AGENTS.md docs/roles/yantrikdb.md)
mapfile -t DOCS < <(find docs -name '*.md' -type f)
FILES+=("${DOCS[@]}")

status=0
for f in "${FILES[@]}"; do
  [ -f "$f" ] || continue
  awk -v file="$f" '
    /^[ \t]*```/ { infence = !infence; next }
    infence { next }
    {
      line = $0
      gsub(/`[^`]*`/, "", line)
      if (line ~ /(^|[^A-Za-z])(TBD|FIXME)([^A-Za-z]|$)/) {
        printf "%s:%d: %s\n", file, NR, $0
        found = 1
      }
    }
    END { exit found ? 1 : 0 }
  ' "$f" || status=1
done

if [ "$status" -ne 0 ]; then
  echo "Unresolved documentation placeholder found." >&2
  exit 1
fi
echo "placeholder scan: clean"
