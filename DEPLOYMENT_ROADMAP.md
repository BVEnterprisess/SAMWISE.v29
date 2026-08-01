# DEPLOYMENT ROADMAP — YantrikDB + MetaClaw [skills_only]

## From This Second to Running in Production

This is the executable plan. Every phase has entry criteria, exit criteria, and a definition of done. Nothing moves forward until the previous phase is validated.

**The process at every phase:**

```
1. PROPOSE     → Design doc / ADR for the phase
2. APPROVE     → Review, refactor, lock scope
3. EXECUTE     → Build with tests
4. VALIDATE    → Verify against exit criteria
```

---

## Where We Are Right Now

```
✅ Architecture vision        (GRADIENT.md — merged)
✅ Gap analysis               (GAPS.md — local)
✅ YantrikDB + MetaClaw spec  (YANTRIKDB.md — local)
❌ Repository bootstrap
❌ Design decisions (language, storage, API)
❌ Code
❌ Tests
❌ CI/CD
❌ Deployment
❌ Running system
```

---

## Phase 0: Repository Bootstrap

**Entry:** Spec documents exist. No code.

**What gets built:**

```
YantrikDB/
├── README.md                  # What this is, how to run it, how to contribute
├── ARCHITECTURE.md            # Link to YANTRIKDB.md spec + local notes
├── LICENSE
├── .gitignore
├── .editorconfig
├── CHANGELOG.md
├── docs/
│   └── adr/                   # Architecture Decision Records
│       ├── 001-language-choice.md
│       ├── 002-storage-backend.md
│       ├── 003-api-framework.md
│       └── 004-deployment-target.md
├── src/                       # (empty, created in Phase 1)
├── tests/                     # (empty, created in Phase 1)
├── scripts/                   # Build, test, deploy scripts
├── docker/                    # Dockerfiles and compose files
├── .github/
│   └── workflows/
│       ├── ci.yml             # Lint + test on every PR
│       └── release.yml        # Build artifacts on tag
└── Makefile                   # or Taskfile — one command for every operation
```

**CI pipeline (minimum viable):**

```yaml
# .github/workflows/ci.yml
on: [push, pull_request]
jobs:
  ci:
    - lint        # code style, no TODO without issue #
    - typecheck   # static type checking
    - test        # unit + integration
    - build       # produces artifact
    - coverage    # minimum threshold enforced
```

**Exit criteria:**

- [ ] Repo exists on GitHub under BVEnterprisess
- [ ] CI pipeline runs green on empty project
- [ ] `make test` works locally
- [ ] `make lint` works locally
- [ ] Branch protection: main requires CI pass + 1 review
- [ ] README explains what this is and how to get started

**Definition of done:** A stranger can clone, build, and run tests in one command.

---

## Phase 0.5: Design Decisions (ADRs)

These four decisions MUST be locked before any code is written. Each one gets an ADR (Architecture Decision Record) documenting the options considered, the choice made, and the rationale.

### ADR-001: Language & Runtime

| Option | Pros | Cons |
|---|---|---|
| **TypeScript (Node.js/Bun)** | Spec schemas already in TS. Fast iteration. Same language as n8n, Midscene, dev tools. Strong typing. Huge ecosystem. | Runtime overhead. Not ideal for WAL-heavy I/O. |
| **Rust** | Performance. Memory safety. Existing yantrikos/yantrikdb is Rust. Ideal for WAL + storage engine. | Slower development velocity. Steeper learning curve. |
| **Go** | Fast compilation. Good concurrency. Strong stdlib. Deployable as single binary. | Less expressive type system than TS/Rust. |
| **Python** | Fastest prototyping. PyO3 bindings available. | Performance ceiling. Type safety is optional. |

**Recommendation:** TypeScript. The spec is already in TypeScript. The surrounding ecosystem (n8n, Midscene, dev tools) is TypeScript. Speed of iteration matters more than raw throughput for this phase. Storage-hot-paths can be optimized later or delegated to an embedded engine (SQLite/LevelDB).

### ADR-002: Storage Backend

| Option | Pros | Cons |
|---|---|---|
| **SQLite + FTS5** | Zero-config. Embedded. Full-text search built in. WAL mode for concurrency. Proven. Single-file backup. | Single-writer. No native graph queries. |
| **PostgreSQL** | Full SQL. JSONB. Full-text search. Concurrent writes. Mature. | External dependency. Needs a running server. Adds ops complexity. |
| **LevelDB/RocksDB** | Fast KV store. Good for WAL. Embedded. | No SQL. No full-text search. Must build query layer on top. |
| **Hybrid: SQLite + graph index** | SQLite for storage + FTS. Adjacency list or in-memory graph for relationships. | Custom graph layer to build and maintain. |

**Recommendation:** SQLite in WAL mode + FTS5 for full-text search. It's embedded (no external server), it's a single file (trivial backup/replication), it supports concurrent reads, and FTS5 handles text search. Relationships stored as adjacency table. Graph queries via recursive CTEs or in-memory index. This keeps deployment dead simple — YantrikDB is a single process with a single file.

### ADR-003: API Framework

| Option | Pros | Cons |
|---|---|---|
| **Hono** | Lightweight. Fast. TypeScript-first. OpenAPI generation. Edge-ready. | Smaller ecosystem than Express. |
| **Fastify** | Mature. Schema validation. Fast. Plugin system. | Heavier than Hono. |
| **tRPC** | End-to-end type safety. No schema duplication. | Tightly coupled client/server. Less suitable for cross-service. |
| **REST + OpenAPI** | Universal. Language-agnostic. Documentable. Tooling everywhere. | More boilerplate. Schema drift risk. |

**Recommendation:** Hono with OpenAPI schema generation. REST endpoints that any service in any language can call. OpenAPI spec auto-generated from route definitions. Type-safe internally, language-agnostic externally.

### ADR-004: Deployment Target

| Option | Pros | Cons |
|---|---|---|
| **Docker + docker-compose** | Portable. Reproducible. Well-understood. Compose for multi-service. | Overhead on low-resource nodes (Chromebook). |
| **Systemd service** | Native. Minimal overhead. Auto-restart. Journal logging. | Linux-only. Less portable. |
| **Single binary + systemd** | Zero dependencies. Minimal footprint. Easy to replicate across nodes. | No isolation. Manual dependency management. |
| **Hybrid: binary in Docker** | Binary for Chromebook/Android. Docker for Desktop/Dell. | Two deployment paths to maintain. |

**Recommendation:** Single compiled binary (Bun compile or pkg) wrapped in a Dockerfile. Deploy as Docker on GTX Desktop and Dell (plenty of resources). Deploy as bare binary + systemd on Chromebook and Android (resource-constrained). One build artifact, two deployment modes.

**Exit criteria:**

- [ ] Four ADRs written and committed
- [ ] Language chosen
- [ ] Storage backend chosen
- [ ] API framework chosen
- [ ] Deployment target chosen
- [ ] All decisions documented with alternatives considered

**Definition of done:** No ambiguity remains about what to build or how to build it.

---

## Phase 1: YantrikDB Core Storage

**Entry:** Phase 0 + 0.5 complete. Repo bootstrapped. Decisions locked.

**What gets built:**

```
src/
├── storage/
│   ├── wal.ts                 # Write-ahead log (append-only, fsync)
│   ├── store.ts               # Primary knowledge store (SQLite)
│   ├── migrations/            # Schema migrations (numbered, ordered)
│   │   ├── 001_knowledge_types.sql
│   │   ├── 002_entities.sql
│   │   ├── 003_relationships.sql
│   │   ├── 004_fts_indices.sql
│   │   └── 005_entropy.sql
│   └── backup.ts              # Snapshot mechanism
├── schema/
│   ├── types.ts               # The 12 knowledge type interfaces (from spec)
│   ├── validation.ts          # Schema validation (zod or arktype)
│   └── enums.ts               # Department, Status, EntityType, etc.
└── index.ts                   # Storage public interface
```

**Tests:**

```
tests/
├── storage/
│   ├── wal.test.ts            # Append, read, replay, crash recovery
│   ├── store.test.ts          # CRUD operations, migrations
│   ├── validation.test.ts     # Valid + invalid payloads for all 12 types
│   └── backup.test.ts         # Snapshot + restore
```

**Exit criteria:**

- [ ] WAL: append, read, replay from offset, crash recovery (kill mid-write, replay succeeds)
- [ ] Store: all 12 knowledge types can be written and read back
- [ ] Validation: every type rejects invalid payloads with clear error messages
- [ ] Migrations: run in order, idempotent, rollback-safe
- [ ] Backup: snapshot produces a restorable file, restore recovers all data
- [ ] Coverage: ≥ 90% on storage layer
- [ ] CI: green

**Definition of done:** Data goes in, data comes out, data survives a crash.

---

## Phase 2: YantrikDB Write Pipeline + Query Engine

**Entry:** Phase 1 complete.

**What gets built:**

```
src/
├── pipeline/
│   ├── writer.ts              # Write pipeline (validate → dedup → contradict → persist → index → notify)
│   ├── deduplication.ts       # content_hash matching, idempotency keys
│   └── contradiction.ts       # Contradiction detection (basic — full resolution in Phase 4)
├── query/
│   ├── engine.ts              # Query builder + executor
│   ├── filters.ts             # Tag, department, environment, status, confidence, age filters
│   ├── search.ts              # FTS5 full-text search wrapper
│   └── graph.ts               # Entity relationship traversal (recursive CTEs)
└── api/
    ├── routes/
│   ├── knowledge.ts       # POST/GET/PATCH/DELETE /knowledge
│   ├── query.ts           # POST /query
│   ├── search.ts          # GET /search
│   ├── entities.ts        # Entity + relationship endpoints
│   └── health.ts          # GET /healthz
├── server.ts              # Hono server setup
└── openapi.ts             # OpenAPI spec generation
```

**Tests:**

```
tests/
├── pipeline/
│   ├── writer.test.ts         # Full pipeline: valid write, invalid write, dedup, contradiction
│   ├── deduplication.test.ts  # Same content_hash → upsert vs reject
│   └── contradiction.test.ts  # Contradicting facts both stored, linked
├── query/
│   ├── engine.test.ts         # All filter combinations
│   ├── search.test.ts         # Text search with filters
│   └── graph.test.ts          # Relationship traversal (1-hop, 2-hop, N-hop)
└── api/
    ├── knowledge.test.ts      # HTTP endpoint tests
    ├── query.test.ts          # Query endpoint tests
    └── openapi.test.ts        # Spec validates against implementation
```

**Exit criteria:**

- [ ] Write pipeline: 7-step validation runs on every write (schema → dedup → contradiction → scope → entropy → persist → notify)
- [ ] Deduplication: identical content_hash is handled per write_mode
- [ ] Contradiction detection: contradicting facts are linked (resolution deferred to Phase 4)
- [ ] Query engine: all KnowledgeQuery filters work, combinable
- [ ] Full-text search: works with tag + department + confidence filters
- [ ] Graph traversal: "get all knowledge related to entity X" works (1-hop and 2-hop)
- [ ] REST API: all endpoints functional, OpenAPI spec generated and valid
- [ ] Health check: GET /healthz returns status
- [ ] Coverage: ≥ 85% overall
- [ ] CI: green

**Definition of done:** Any service can write knowledge and query it back through the API.

---

## Phase 3: Entropy + Continuity

**Entry:** Phase 2 complete.

**What gets built:**

```
src/
├── entropy/
│   ├── decay.ts               # Confidence decay computation
│   ├── stale.ts               # Stale knowledge detection
│   ├── reverification.ts      # Reverification task generation
│   └── report.ts              # Entropy report generation
├── continuity/
│   ├── replication.ts         # Async replication to secondary node
│   ├── snapshot.ts            # Periodic snapshot scheduling
│   ├── recovery.ts            # Restore from snapshot + WAL replay
│   └── health.ts              # Replication lag monitoring
└── subscriptions/
    ├── manager.ts             # Subscription registration and matching
    └── dispatcher.ts          # Notification dispatch to subscribers
```

**Tests:**

```
tests/
├── entropy/
│   ├── decay.test.ts          # Decay formula correctness for all 12 types
│   ├── stale.test.ts          # Stale detection at configurable thresholds
│   └── reverification.test.ts # Task generation for stale high-value items
├── continuity/
│   ├── replication.test.ts    # Write on primary → appears on secondary
│   ├── snapshot.test.ts       # Snapshot + restore preserves all data
│   ├── recovery.test.ts       # Kill primary → restore on secondary → all data present
│   └── chaos.test.ts          # Random kills during writes → no data loss
└── subscriptions/
    └── subscriptions.test.ts  # Subscribe → write matching knowledge → notification received
```

**Exit criteria:**

- [ ] Entropy: confidence decays correctly per type-specific rate
- [ ] Entropy: stale items detected when confidence drops below threshold
- [ ] Entropy: reverification tasks generated for stale high-value items
- [ ] Replication: writes on primary appear on secondary within RPO (< 60s)
- [ ] Snapshots: periodic snapshots created and stored
- [ ] Recovery: kill primary → restore on secondary from snapshot + WAL → zero data loss
- [ ] Chaos: random process kills during active writes → WAL replay recovers all committed writes
- [ ] Subscriptions: services can subscribe to knowledge changes and receive notifications
- [ ] Coverage: ≥ 85%
- [ ] CI: green

**Definition of done:** YantrikDB survives node failure. Knowledge decays correctly. Subscribers are notified.

---

## Phase 4: Contradiction Resolution + Governance

**Entry:** Phase 3 complete.

**What gets built:**

```
src/
├── contradiction/
│   ├── resolver.ts            # Full contradiction resolution pipeline
│   ├── scoring.ts             # Confidence weighting (verification + recency)
│   └── report.ts              # Contradiction report generation
├── governance/
│   ├── engine.ts              # Governance rule evaluation
│   ├── rules.ts               # Rule CRUD + matching
│   └── enforcement.ts         # Constraint enforcement on writes and queries
└── api/
    └── routes/
        ├── contradictions.ts  # GET/POST /contradictions
        └── governance.ts      # CRUD /governance/rules, /constraints
```

**Exit criteria:**

- [ ] Contradiction: effective confidence computed with verification_weight × recency_weight
- [ ] Contradiction: higher-confidence fact → active, lower → contested
- [ ] Contradiction: within-threshold contradictions → both contested, verification task generated
- [ ] Contradiction: resolution recorded as Decision knowledge type
- [ ] Governance: rules evaluate on write and query
- [ ] Governance: constraints enforced (hard = block, soft = warn)
- [ ] Coverage: ≥ 85%
- [ ] CI: green

**Definition of done:** Contradictions resolve automatically. Governance rules fire correctly.

---

## Phase 5: MetaClaw Observe + Inject

**Entry:** Phase 4 complete. YantrikDB fully operational.

**What gets built:**

```
src/
├── metaclaw/
│   ├── observer/
│   │   ├── ingest.ts          # Trace ingestion from YantrikDB
│   │   └── filter.ts          # shouldAnalyze() filter
│   ├── matcher/
│   │   ├── matching.ts        # Skill matching: trigger_pattern → objective
│   │   ├── ranking.ts         # Rank by match_score, success_rate, usage, recency
│   │   └── budget.ts          # Token budget allocation
│   ├── injector/
│   │   └── formatter.ts       # Format skills for agent context injection
│   ├── store/
│   │   └── skills.ts          # Skill CRUD on top of YantrikDB
│   └── api/
│       └── routes/
│           ├── ingest.ts      # POST /metaclaw/ingest
│           ├── match.ts       # POST /metaclaw/match
│           └── skills.ts      # CRUD /metaclaw/skills
```

**Tests:**

```
tests/
├── metaclaw/
│   ├── ingest.test.ts         # Trace ingestion + filtering
│   ├── matching.test.ts       # Skill matching against various objectives
│   ├── ranking.test.ts        # Ranking correctness
│   ├── budget.test.ts         # Token budget allocation
│   ├── formatter.test.ts      # Injection format correctness
│   └── integration.test.ts    # Full loop: ingest trace → match → inject
```

**Exit criteria:**

- [ ] Ingest: execution traces ingested, filter correctly identifies analyzable traces
- [ ] Match: given an objective, applicable skills are returned ranked correctly
- [ ] Inject: matched skills formatted within token budget
- [ ] Skill store: full CRUD on Skill type through YantrikDB
- [ ] Integration: write trace → match returns skill → injection payload is valid
- [ ] Coverage: ≥ 85% on MetaClaw modules
- [ ] CI: green

**Definition of done:** Known skills are injected into future executions. The inject side of the loop works.

---

## Phase 6: MetaClaw Extract

**Entry:** Phase 5 complete.

**What gets built:**

```
src/
├── metaclaw/
│   └── extractor/
│       ├── repeated.ts        # Strategy A: Repeated sequence detection
│       ├── explicit.ts        # Strategy B: Explicit procedure detection
│       ├── mining.ts          # Strategy C: Cross-trace pattern mining
│       ├── dedup.ts           # Deduplication against existing skills
│       └── pipeline.ts        # Extraction orchestrator
```

**Tests:**

```
tests/
├── metaclaw/
│   └── extractor/
│       ├── repeated.test.ts   # Feed 5 similar traces → extract common skill
│       ├── explicit.test.ts   # Feed annotated trace → extract procedure
│       ├── mining.test.ts     # Feed 20 traces → find predictive patterns
│       ├── dedup.test.ts      # Existing similar skill → merge, don't duplicate
│       └── pipeline.test.ts   # Full extraction pipeline end-to-end
```

**Exit criteria:**

- [ ] Strategy A: given ≥3 similar successful traces, common step sequence extracted as candidate skill
- [ ] Strategy B: given trace with explicit procedure annotations, skill extracted
- [ ] Strategy C: given ≥20 traces, predictive patterns identified
- [ ] Deduplication: similar existing skill → evidence merged, not duplicated
- [ ] Candidate skills written to YantrikDB with status = "candidate"
- [ ] Coverage: ≥ 80% on extractor
- [ ] CI: green

**Definition of done:** New skills are extracted from execution traces and stored as candidates.

---

## Phase 7: MetaClaw Evolve

**Entry:** Phase 6 complete.

**What gets built:**

```
src/
├── metaclaw/
│   └── evolution/
│       ├── usage.ts           # Skill usage outcome recording
│       ├── deviation.ts       # Deviation detection from expected steps
│       ├── evaluator.ts       # Evolution trigger evaluation
│       ├── versioning.ts      # Skill version management
│       └── recommender.ts     # Evolution recommendation generation
```

**Tests:**

```
tests/
├── metaclaw/
│   └── evolution/
│       ├── usage.test.ts      # Usage recording updates skill stats
│       ├── deviation.test.ts  # Deviations detected and classified
│       ├── evaluator.test.ts  # All 6 evolution triggers fire correctly
│       ├── versioning.test.ts # v1 → v2 → v1 superseded
│       └── recommender.test.ts # Recommendations are valid and actionable
```

**Exit criteria:**

- [ ] Usage recording: skill usage updates success_rate, usage_count, last_used_at
- [ ] Deviation detection: agent deviations from skill steps are recorded and classified
- [ ] All 6 evolution triggers fire under correct conditions
- [ ] Versioning: new version created, old version superseded, chain preserved
- [ ] Skill lifecycle: candidate → validated → active → stale → deprecated all transition correctly
- [ ] Coverage: ≥ 80% on evolution
- [ ] CI: green

**Definition of done:** Skills evolve based on usage feedback. The full compounding loop is closed.

---

## Phase 8: Integration Testing + Deployment

**Entry:** Phase 7 complete. All components built and unit-tested.

**What gets built:**

```
tests/
├── integration/
│   ├── compounding-loop.test.ts   # Full loop: intent → execute → trace → extract → inject → cheaper execution
│   ├── cross-department.test.ts   # Knowledge from dept A is queryable by dept B
│   ├── node-failure.test.ts       # Kill node → service materializes elsewhere
│   ├── contradiction-flow.test.ts # Contradicting facts → resolution → consumers notified
│   └── load.test.ts               # 10K writes, 1K queries/sec — measure latency
├── e2e/
│   └── full-system.test.ts        # Real objective, real execution, real compounding
docker/
├── Dockerfile
├── docker-compose.yml             # YantrikDB + dependencies for local dev
└── docker-compose.prod.yml        # Production deployment config
scripts/
├── deploy.sh                      # Deploy to target node
├── failover.sh                    # Simulate node failure, verify recovery
└── benchmark.sh                   # Run load tests
```

**Exit criteria:**

- [ ] Compounding loop: end-to-end test proves second execution is cheaper than first
- [ ] Cross-department: knowledge written by "engineering" is queryable by "operations"
- [ ] Node failure: kill primary → YantrikDB recovers on secondary within RTO (< 300s)
- [ ] Load: system handles expected write/query volume without degradation
- [ ] Docker: `docker-compose up` starts YantrikDB with all features operational
- [ ] Deploy script: one command deploys to target node
- [ ] Failover script: simulates failure, verifies recovery
- [ ] CI: all integration + e2e tests green
- [ ] README: deployment instructions complete

**Definition of done:** The system runs, survives failure, and demonstrably compounds.

---

## Phase 9: Deploy to Fleet

**Entry:** Phase 8 complete. Docker image built. Deploy scripts tested.

**Deployment sequence:**

```
1. GTX Desktop (primary infrastructure node)
   ├── Deploy YantrikDB as primary
   ├── Verify health check
   ├── Run smoke tests
   └── Confirm replication ready

2. Dell Laptop (control cockpit / secondary)
   ├── Deploy YantrikDB as secondary (replication target)
   ├── Verify replication lag < 60s
   ├── Test failover: kill primary → secondary promotes
   └── Restore primary, verify re-sync

3. Tailscale integration
   ├── Verify YantrikDB reachable from all nodes via Tailscale
   ├── Verify SSH access for admin
   └── Verify health check accessible from n8n (future)

4. Monitoring
   ├── Health check endpoint operational
   ├── Replication status endpoint operational
   ├── Entropy report endpoint operational
   └── Skill report endpoint operational
```

**Exit criteria:**

- [ ] YantrikDB running on GTX Desktop as primary
- [ ] YantrikDB replicating to Dell as secondary
- [ ] Failover tested and verified
- [ ] Reachable from all Tailscale nodes
- [ ] Health, replication, entropy, and skill reports accessible
- [ ] No data loss during failover test
- [ ] Monitoring: n8n can poll health check (or will be able to when n8n is deployed)

**Definition of done:** YantrikDB is running, replicated, and reachable across the fleet.

---

## The Dependency Graph

```
Phase 0    Repo Bootstrap
  │
Phase 0.5  Design Decisions (ADRs)
  │
Phase 1    Core Storage          ← foundation
  │
Phase 2    Write Pipeline + Query ← makes it usable
  │
Phase 3    Entropy + Continuity  ← makes it durable
  │
Phase 4    Contradiction + Gov.  ← makes it trustworthy
  │                                ← YANTRIKDB COMPLETE
Phase 5    MetaClaw Observe+Inject ← makes it useful
  │
Phase 6    MetaClaw Extract      ← makes it learning
  │
Phase 7    MetaClaw Evolve       ← makes it compounding
  │                                ← METACLAW COMPLETE
Phase 8    Integration + Deploy  ← makes it real
  │
Phase 9    Fleet Deployment      ← makes it operational
```

**Parallelization opportunities:**

| Can run in parallel | Phases |
|---|---|
| API routes + storage | Phase 2 (routes can be stubbed against mock store) |
| Entropy + Subscriptions | Phase 3 (independent subsystems) |
| Contradiction + Governance | Phase 4 (independent subsystems) |
| Extract + Evolve scaffolding | Phase 6-7 (evolution scaffolding while extraction is built) |

---

## The Compounding Validation Test

The single test that proves the system works:

```
Test: "The second execution is cheaper than the first."

Setup:
  - YantrikDB empty. No skills. No knowledge.

Execution 1:
  - Submit objective: "Deploy n8n to the GTX Desktop"
  - System researches, discovers, executes
  - Produces: execution trace, facts, procedures, failures
  - Record: total_steps, total_tokens, total_cost, human_interventions, duration

MetaClaw extraction:
  - Ingest trace
  - Extract skill: "deploy-n8n-to-gtx-desktop"
  - Store in YantrikDB

Execution 2:
  - Submit same objective: "Deploy n8n to the GTX Desktop"
  - System queries YantrikDB → finds facts, procedures
  - MetaClaw matches → injects skill
  - System follows known procedure
  - Record: total_steps, total_tokens, total_cost, human_interventions, duration

Assert:
  - execution_2.total_steps     < execution_1.total_steps
  - execution_2.total_tokens    < execution_1.total_tokens
  - execution_2.total_cost      < execution_1.total_cost
  - execution_2.human_interventions ≤ execution_1.human_interventions
  - execution_2.duration        < execution_1.duration

This is the proof that Gradient = Compounding.
```

---

## Effort Estimates

| Phase | Effort | PRs | Key risk |
|---|---|---|---|
| 0 — Bootstrap | 0.5 day | 1 | Low |
| 0.5 — ADRs | 0.5 day | 1 | Low — but blocks everything |
| 1 — Core Storage | 2–3 days | 2–3 | WAL crash recovery |
| 2 — Write + Query | 3–4 days | 3–4 | Graph traversal performance |
| 3 — Entropy + Continuity | 3–4 days | 2–3 | Replication consistency |
| 4 — Contradiction + Gov. | 2–3 days | 2 | Contradiction edge cases |
| 5 — MetaClaw Observe+Inject | 2–3 days | 2 | Matching accuracy |
| 6 — MetaClaw Extract | 3–4 days | 2–3 | Pattern quality / noise |
| 7 — MetaClaw Evolve | 2–3 days | 2 | Evolution correctness |
| 8 — Integration + Deploy | 2–3 days | 2 | End-to-end reliability |
| 9 — Fleet Deploy | 1 day | 1 | Environment-specific issues |
| **Total** | **~21–31 days** | **~20–25 PRs** | |

---

## What This Repo Should Look Like When We're Done

```
YantrikDB/
├── README.md
├── ARCHITECTURE.md
├── CHANGELOG.md
├── LICENSE
├── Makefile
├── docs/
│   ├── adr/                   # 4+ architecture decision records
│   ├── GRADIENT.md            # Architecture vision
│   ├── GAPS.md                # Gap analysis
│   ├── YANTRIKDB.md           # Full specification
│   └── DEPLOYMENT_ROADMAP.md  # This document
├── src/
│   ├── storage/               # WAL, store, migrations, backup
│   ├── schema/                # Types, validation, enums
│   ├── pipeline/              # Write pipeline, dedup, contradiction detection
│   ├── query/                 # Query engine, search, graph traversal
│   ├── entropy/               # Decay, stale detection, reverification
│   ├── continuity/            # Replication, snapshots, recovery
│   ├── contradiction/         # Resolution engine, scoring
│   ├── governance/            # Rule engine, enforcement
│   ├── subscriptions/         # Event subscriptions, notifications
│   ├── metaclaw/
│   │   ├── observer/          # Trace ingestion, filtering
│   │   ├── extractor/         # Pattern extraction (3 strategies)
│   │   ├── matcher/           # Skill matching, ranking, budget
│   │   ├── injector/          # Context formatting
│   │   ├── evolution/         # Usage tracking, deviation, versioning
│   │   └── store/             # Skill CRUD
│   ├── api/                   # Hono server, routes, OpenAPI
│   └── index.ts               # Public interface
├── tests/
│   ├── unit/                  # Mirrors src/ structure
│   ├── integration/           # Cross-module tests
│   └── e2e/                   # Full system tests
├── docker/
│   ├── Dockerfile
│   ├── docker-compose.yml
│   └── docker-compose.prod.yml
├── scripts/
│   ├── deploy.sh
│   ├── failover.sh
│   └── benchmark.sh
└── .github/
    └── workflows/
        ├── ci.yml
        └── release.yml
```

---

*The plan is the plan. Propose. Approve. Execute. Validate.*

*No boilerplate. No premature code. No hand-waving.*

*Deploy the fucker.*
