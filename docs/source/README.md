# Source Documents

Primary source material this project's specs were derived from. These are **inputs**, not
outputs — the specs in `../superpowers/specs/` cite them and may correct them, but must not
silently contradict them.

| File | Contents |
|---|---|
| `memory-spec-v1.4.md` | MEMORY SPECIFICATION v1.4 — the Memory Gravity Well, the force-balance control loop, the inline-skill flow, the ten hand-off instructions |
| `omniroute-forensic-autopsy.md` | OmniRoute Architectural Autopsy — MVK, 12-factor scoring, 17 routing strategies, the resilience stack |
| `omniroute-trace-inventory.md` | OmniRoute Observable Trace Inventory — ~506 enumerated traces across 40 categories |
| `yantrikdb-engine-brief.md` | YantrikDB — the five fronts, unified five-index engine, `think()`, cluster/ghosting, scale ceiling |
| `yantrikdb-native-and-cei.md` | YantrikDB native capabilities (LSM/DeltaIndex, FSRS decay) and the proposed Controlled Entropy Invariant |
| `metaclaw-skills-only.md` | MetaClaw `skills_only` — proxy control plane, SkillManager retrieval, SkillEvolver, threat model |
| `samwise-v3-full.md` | SAMWISE v3 superset (504 KB) — governance event bus, n8n workflow programs, `openclaw.json` policy, per-component deep dives |

## Reading notes

- These are **escaped-markdown exports**: headings appear as `\#\#`, bold as `\*\*`. Content is
  verbatim and intentionally unmodified — grep for content words, not markdown syntax.
- `memory-spec-v1.4.md` was extracted from a combined export whose first 662 lines are the
  GRADIENT document already present at `../GRADIENT.md`. Only the v1.4 portion is here.
- Where a source document and a spec disagree, the spec must say so explicitly and give evidence.
  Several already do — see `../superpowers/specs/2026-08-06-governor-control-loop-design.md`,
  which corrects v1.4 §5 (placement), §2.5 (statelessness), and §6/§9 (the stability claim).

A claim traced to a document absent from this directory is **not verified**. Ask for the document.
