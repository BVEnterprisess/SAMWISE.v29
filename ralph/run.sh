#!/usr/bin/env bash
# SAMWISE Ralph Loop — harness-agnostic overnight driver.
#
# The model proposes. This script disposes.
#   pick task -> run agent -> PROOF must pass -> GATE must pass -> commit
#   anything less -> git reset --hard (the model cannot corrupt the repo)
#
# Usage:  ./ralph/run.sh
# Config (env):
#   RALPH_AGENT=codex|opencode   (default: codex)
#   RALPH_MODEL=<model id>       (default: agent's own default)
#   RALPH_MAX_ITER=<n>           (default: 200)
#   RALPH_HOURS=<n>              (default: 10)  wall-clock budget
#   RALPH_MAX_FAILS=<n>          (default: 4)   consecutive fails before task is BLOCKED
set -uo pipefail

REPO="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CORE="$REPO/samwise-core"
RALPH="$REPO/ralph"
STATE="$RALPH/state"
mkdir -p "$STATE"

AGENT="${RALPH_AGENT:-codex}"
MODEL="${RALPH_MODEL:-}"
MAX_ITER="${RALPH_MAX_ITER:-200}"
HOURS="${RALPH_HOURS:-10}"
MAX_FAILS="${RALPH_MAX_FAILS:-4}"
DEADLINE=$(( $(date +%s) + HOURS*3600 ))

LEDGER="$STATE/ledger.md"
[ -f "$LEDGER" ] || echo "# Ralph ledger — started $(date -Is)" > "$LEDGER"

log()  { echo "[$(date +%H:%M:%S)] $*" | tee -a "$STATE/run.log"; }
note() { echo "$*" >> "$LEDGER"; }

# --- task table -------------------------------------------------------------
# Returns "ID|TITLE|PROOF" of the first task whose PROOF does not yet pass.
next_task() {
  while IFS='|' read -r id title proof; do
    case "$id" in ''|\#*) continue;; esac
    id="$(echo "$id" | xargs)"
    grep -q "^BLOCKED $id\$" "$STATE/blocked" 2>/dev/null && continue
    if ! ( cd "$CORE" && eval "$(echo "$proof" | xargs)" ) >/dev/null 2>&1; then
      echo "$id|$(echo "$title" | xargs)|$(echo "$proof" | xargs)"
      return 0
    fi
  done < "$RALPH/tasks.txt"
  return 1
}

# --- agent adapter ----------------------------------------------------------
run_agent() {
  local prompt_file="$1" out="$2"
  case "$AGENT" in
    codex)
      local args=(exec --skip-git-repo-check -s workspace-write -C "$REPO" -o "$out")
      [ -n "$MODEL" ] && args+=(-m "$MODEL")
      codex "${args[@]}" - < "$prompt_file"
      ;;
    opencode)
      ( cd "$REPO" && opencode run "$(cat "$prompt_file")" ) | tee "$out"
      ;;
    *) log "FATAL unknown RALPH_AGENT=$AGENT"; exit 2;;
  esac
}

# --- main loop --------------------------------------------------------------
log "ralph start agent=$AGENT model=${MODEL:-default} max_iter=$MAX_ITER budget=${HOURS}h"
cd "$REPO"
git rev-parse --is-inside-work-tree >/dev/null 2>&1 || { log "FATAL not a git repo"; exit 2; }

iter=0; fails=0; last_id=""
while :; do
  iter=$((iter+1))
  [ "$iter" -gt "$MAX_ITER" ]        && { log "STOP max iterations"; break; }
  [ "$(date +%s)" -ge "$DEADLINE" ]  && { log "STOP wall-clock budget"; break; }

  task="$(next_task)" || {
    log "ALL TASKS PROVEN — running final gate"
    if "$RALPH/gate.sh" > "$STATE/final-gate.log" 2>&1; then
      log "PASS 0 CLOSED. evidence: $STATE/final-gate.log"
      note "## PASS0 CLOSED $(date -Is)"
      exit 0
    fi
    log "final gate RED despite all proofs — see final-gate.log"; exit 1
  }

  id="${task%%|*}"; rest="${task#*|}"; title="${rest%%|*}"; proof="${rest##*|}"
  [ "$id" = "$last_id" ] || fails=0
  last_id="$id"

  log "iter=$iter task=$id fails=$fails :: $title"

  # Clean slate. Untracked build output is gitignored, so this is safe.
  git reset --hard -q && git clean -fdq -e ralph/state

  # Feed the previous failure back in — this is how a cheap model self-corrects.
  FEEDBACK=""
  if [ "$fails" -gt 0 ] && [ -f "$STATE/last-fail.log" ]; then
    FEEDBACK=$'\n\n## Your previous attempt FAILED. Fix exactly this:\n```\n'
    FEEDBACK+="$(tail -60 "$STATE/last-fail.log")"
    FEEDBACK+=$'\n```\n'
  fi

  P="$STATE/prompt-$iter.md"
  { sed -e "s|{{TASK_ID}}|$id|g" -e "s|{{TASK_TITLE}}|$title|g" -e "s|{{PROOF}}|$proof|g" \
        "$RALPH/PROMPT.md"; printf '%s' "$FEEDBACK"; } > "$P"

  run_agent "$P" "$STATE/agent-$iter.txt" >> "$STATE/agent-$iter.log" 2>&1

  # ---- verification: proof first (cheap), then full gate (expensive) -------
  if ! ( cd "$CORE" && eval "$proof" ) > "$STATE/last-fail.log" 2>&1; then
    fails=$((fails+1)); log "  PROOF RED ($id)"
    git reset --hard -q; git clean -fdq -e ralph/state
  elif ! "$RALPH/gate.sh" > "$STATE/last-fail.log" 2>&1; then
    fails=$((fails+1)); log "  GATE RED ($id)"
    git reset --hard -q; git clean -fdq -e ralph/state
  else
    git add -A ":!ralph/state"
    if git diff --cached --quiet; then
      fails=$((fails+1)); log "  NO-OP: proof passed but agent changed nothing"
    else
      git commit -qm "feat($id): $title" -m "Proof: $proof" -m "Gate: green (fmt/check/test/clippy/docs/contract)"
      log "  GREEN -> committed $(git rev-parse --short HEAD)"
      note "- [x] $id $title — $(git rev-parse --short HEAD) $(date -Is)"
      fails=0; continue
    fi
  fi

  if [ "$fails" -ge "$MAX_FAILS" ]; then
    log "  BLOCKED $id after $fails attempts"
    echo "BLOCKED $id" >> "$STATE/blocked"
    note "- [!] BLOCKED $id $title — $(date -Is)"
    fails=0
  fi
done

log "ralph stopped. iterations=$iter"
[ -s "$STATE/blocked" ] && { log "BLOCKED tasks:"; cat "$STATE/blocked" | tee -a "$STATE/run.log"; }
exit 0
