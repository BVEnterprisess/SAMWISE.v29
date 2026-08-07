# YANTRIKDB  08.05.2026

# ---

## SECTION 0: THE EXECUTIVE THESIS — "THE FIVE FRONTS OF DISRUPTION"

YantrikDB is not a database. It is a cognitive operating system for AI agents — the first piece of infrastructure that treats memory as a living process rather than a passive store.

The disruption unfolds across five simultaneous fronts:

| Front | The Shift | The Killer Implication |  
|-------|-----------|----------------------|  
| Architectural | Five indexes, one engine — not bolted together, but sharing WAL, memory pages, and query planner | No more stitching vector DBs \+ graph DBs \+ caches. One 10MB file replaces five infrastructure pieces |  
| Cognitive | Memory that forgets, consolidates, and detects contradictions autonomously | Agents stop getting noisier at scale. Precision improves with more data — the opposite of every other system |  
| Operational | Raft clustering with a documented asymmetry that must be understood to avoid "ghosting" | The system is battle-hardened (1178 tests \+ chaos harness \+ cargo-fuzz), but cluster routing requires explicit leadership awareness |  
| Scale | O(1) selective recall, \~70 tokens per query regardless of memory count | Enterprise-grade longitudinal ingestion becomes feasible without context explosion |  
| Agentic | 19 MCP tools \+ autonomous \`think()\` cycles enable proactive self-evolution | Agents become proactive — they surface what needs attention without being asked |

The net-net: If an agent lacks this memory architecture, it is fundamentally crippled compared to agents that have it — especially when facing unbounded, lifelong data streams. YantrikDB is the first system that makes sustained, evolving AI relationships possible at scale.

## SECTION 1: THE UNIFIED INDEX — WHY VECTOR DBS LOSE

### 1.1 THE FIVE-INDEX ARCHITECTURE

Vector databases are search engines with extra steps. They store memories but don't manage them. YantrikDB's five indexes share the same memory pages, WAL, and query planner:

\`\`\`  
┌──────────────────────────────────────────────────────┐  
│                   YantrikDB Engine                    │  
│  ┌──────────┬──────────┬──────────┬──────────┐       │  
│  │  Vector  │  Graph   │ Temporal │  Decay   │       │  
│  │ (HNSW)   │(Entities)│ (Events) │  (Heap)  │       │  
│  └──────────┴──────────┴──────────┴──────────┘       │  
│  ┌──────────┐                                         │  
│  │ Key-Value│                                         │  
│  │  Store   │                                         │  
│  └──────────┘                                         │  
│  ┌──────────────────────────────────────────────┐      │  
│  │         Write-Ahead Log (WAL)               │      │  
│  └──────────────────────────────────────────────┘      │  
└──────────────────────────────────────────────────────┘  
\`\`\`

| Index | Purpose | Why It Matters |  
|-------|---------|----------------|  
| Vector (HNSW) | Semantic similarity search across memories | Finds conceptually related content |  
| Graph | Entity relationships — "Max is user's dog," "user works at Meta" | Enables relational reasoning beyond keyword matching |  
| Temporal | Time-series style — "what happened around Tuesday" | Understands sequence and causality |  
| Decay Heap | Priority queue with degrading importance scores | Models human forgetting — old, irrelevant memories fade |  
| Key-Value | Fast facts — "user's name is Pranab" | Instant lookup for known truths |

The paradigm shift: This isn't five databases glued together. It's a unified engine where the query planner can blend signals from all five indexes simultaneously. The result is retrieval that understands what is relevant, when it happened, who it's connected to, and how important it is — all in one pass.

### 1.2 MULTIPLICATIVE GATING: THE SCORING ENGINE

Recall isn't just cosine similarity. YantrikDB uses multi-signal scoring:

\`\`\`  
score \= recency × importance × similarity × graph\_proximity  
\`\`\`

This is multiplicative, not additive. Each factor can dominate:  
\- An old but highly important memory still surfaces  
\- A recent but unimportant one fades  
\- A memory with strong graph connections gets boosted

The query planner doesn't just retrieve top-K vectors and filter afterward. It filters across all five indexes simultaneously — temporal decay pruning, graph proximity weighting, and importance scaling all happen during retrieval, not as post-processing.

### 1.3 THE COGNITIVE STATE GRAPH

The graph isn't just entities and relationships. It stores typed nodes representing cognitive entities — beliefs, goals, routines, needs, episodes, opportunities — and typed edges representing relationships: supports, contradicts, causes, predicts.

This enables:  
\- Belief tracking: Each belief has a log-odds score that updates as evidence accumulates  
\- Contradiction detection: When two nodes contradict, the system flags it  
\- Personality derivation: Stable patterns extracted from memory over time  
\- Bridge detection: Finding connections between previously unrelated domains

## SECTION 2: THE FOUR-STAGE COGNITIVE PIPELINE — "THE SYNTHETIC HIPPOCAMPUS"

### STAGE 1: TEMPORAL DECAY

 Stage 1: Temporal Decay

Memories age with configurable half-life:

\`\`\`python  
db.record("read the SLA doc by Friday", importance=0.4, half\_life=86400)   1 day  
 24 hours later: relevance score has decayed  
 7 days later: recall stops surfacing it unless explicitly queried  
\`\`\`

Why this is a paradigm shift: Most AI systems treat all memories as equally relevant forever. YantrikDB models how actual cognition works — recent, frequently accessed memories matter more; stale ones fade. The decay heap is a priority queue where importance scores degrade over time.

### STAGE 2: SEMANTIC CONSOLIDATION

Feed it 20 similar meeting notes. YantrikDB collapses them into canonical memories:

\`\`\`python  
db.think()  
 → {"consolidation\_count": 5}  
 collapsed 20 fragments into 5 canonical memories  
\`\`\`

The mechanism: Consolidation runs on a bounded window of recent active memories (configurable limit, default 30\) with pairwise cosine similarity at threshold 0.85. Shared entities are required for merging — no shared entities means no merge, even if similarity is high.

Why this is a paradigm shift: Vector databases store every duplicate. You get 20 near-identical embeddings cluttering your recall. YantrikDB extracts the pattern and keeps one canonical version.

### STAGE 3: CONTRADICTION DETECTION

\`\`\`python  
db.record("CEO is Alice")  
db.record("CEO is Bob")   added later in another conversation  
db.think()  
 → {"conflicts\_found": 1, "conflicts": \[{"memory\_a": "CEO is Alice",   
    "memory\_b": "CEO is Bob", "type": "factual\_contradiction"}\]}  
\`\`\`

The mechanism: The conflict scanner looks for \`(src, rel\_type) → ≥2 dsts\` — same source and relationship type, different destinations.

The honest admission: Contradictions on meaningful data are incredibly contextual. YantrikDB's design stance is that contradictions are surfaced, not resolved. The engine returns a review queue; the agent resolves them conversationally. Temporal supersession (knowing that "CEO is Alice" from 2023 and "CEO is Bob" from 2025 are different valid states) is coming in v0.6.

### STAGE 4: PROACTIVE TRIGGERS

After consolidation and conflict scanning, YantrikDB surfaces what needs attention:  
\- Pending conflicts — contradictions awaiting resolution  
\- Decaying important memories — high-importance items about to fade  
\- Approaching deadlines — time-sensitive memories  
\- Patterns across domains — cross-domain discovery like "work stress correlates with health entries"

Why this is a paradigm shift: Agents become proactive, not reactive. The memory tells them what they should care about. This is what moves agents from reactive Q\&A to proactive self-evolution.

 Performance of the Cognitive Pipeline

Live numbers from a 2-core LXC cluster with 1,689 memories:

| Operation | Latency |  
|-----------|---------|  
| Recall p50 | 112ms (most is query embedding \~100ms) |  
| Recall p99 | 190ms |  
| Batch write | 76 writes/sec |  
| Engine lock acquire | \<0.1ms |  
| Deep health probe | \<1ms |

For pre-computed embeddings, recall p50 drops to \~5ms.

## SECTION 3: THE MCP/TOOL SURFACE — "AUTONOMOUS COGNITIVE AUTONOMY"

### 3.1 THE 19 TOOLS

YantrikDB ships as an MCP server with 19 tools that constitute an agent's "mental hygiene" toolkit:

| Category | Tools | Purpose |  
|----------|-------|---------|  
| Core Memory | \`remember\`, \`recall\`, \`forget\`, \`correct\` | Read/write/update/delete memories |  
| Cognition | \`think\` | Run the full cognitive pipeline autonomously |  
| Graph | \`graph\`, \`conflict\` | Explore relationships and contradictions |  
| Temporal | \`temporal\`, \`session\`, \`trigger\` | Time-aware queries, session tracking |  
| Procedural | \`procedure\`, \`skill\`, \`reinforce\_procedure\` | Learn what works, not just what's known |  
| Personality | \`personality\`, \`category\` | Extract stable tendencies, manage categories |  
| Meta | \`stats\`, \`gaps\`, \`conversation\`, \`task\` | System introspection and task management |

### 3.2 THE CRITICAL TOOL: THINK()

The \`think()\` function enables unsupervised self-reflection, consolidation, and self-correction without human prompting:

\`\`\`python  
 The agent calls this autonomously during idle periods  
db.think(config=ThinkConfig(  
    consolidation\_limit=100,  
    max\_decay\_nodes\_per\_tick=500,  
    run\_pattern\_mining=False   explicitly slow, enable when needed  
))  
 → {"consolidation\_count": 2, "conflicts\_found": 0, "patterns\_new": 1}  
\`\`\`

Why this changes everything: The agent auto-recalls context, auto-remembers decisions, and auto-detects contradictions — no prompting needed. This is what moves agents from reactive assistants to autonomous, self-evolving entities.

### 3.3 EMBEDDER OPTIONS

The MCP server supports multiple embedders:

| Embedder | Dimensions | Install Size | Use Case |  
|----------|-----------|--------------|----------|  
| Bundled Rust | 64-dim | \~10 MB | Default, no native ML deps, \~80ms cold start |  
| ONNX MiniLM-L6-v2 | 384-dim | \~150 MB | Higher quality, auto-detected for pre-v0.6 databases |  
| Multilingual | 256-dim | Variable | 101 languages |

Privacy: All data stays on your machine. No telemetry. No external services.

## SECTION 4: OPERATIONAL REALITY — THE RAFT CLUSTER & THE GHOSTING TRAP

### 4.1 THE ASYMMETRY THAT BITES

In cluster mode with \`cluster.raft\_mode=openraft\`, YantrikDB has an asymmetry that must be understood:

| Operation | Follower Behavior | Leader Behavior |  
|-----------|------------------|-----------------|  
| Write | Returns 503 (not the leader) | Succeeds |  
| Read | Returns 200 OK from local SQLite/HNSW state | Succeeds |

The "Ghosting" Bug: If replication is broken (the cosmetic-openraft bug that RFC 010 PR-6 fixes), a follower returns stale data with a 200\. Net effect: writes consistently land on the leader; reads return whatever the node-you-hit happens to have.

The symptom: Clients that list a follower first see "I wrote it, I can't read it".

### 4.2 THE SURVIVAL MANUAL

Until PR-6 ships (target v0.8.13), the mitigation is explicit:

1\. List the leader first in your client's URL config  
2\. Monitor \`/v1/cluster/raft\` on any node — when \`leader\_id\` changes, update client config \+ restart  
3\. Pin via DNS: point a DNS name (e.g., \`yantrikdb-leader.internal\`) to the current leader

Diagnostic probe:  
\`\`\`bash  
 Find the current leader  
LEADER=$(curl \-s http://NODE\_A:7438/v1/cluster/raft | jq \-r '.current\_leader')  
 Write a probe to the leader directly  
RID=$(curl \-s \-X POST "$LEADER\_ADDR/v1/remember" ...)  
 Read from leader (should hit immediately)  
curl \-s \-X POST "$LEADER\_ADDR/v1/recall" ...  
 Read from follower — if data is missing, you have the ghosting bug  
curl \-s \-X POST "$FOLLOWER\_ADDR/v1/recall" ...  
\`\`\`

### 4.3 THE FIX

RFC 010 PR-6 restructures routing: handlers route writes through openraft consensus → followers replicate → reads on any node return the same data. The Submitter/Applier trait split is the architectural change enabling this.

### 4.4 BATTLE HARDENING

The system has undergone a 42-task hardening sprint across 8 epics:  
\- \`parking\_lot\` mutexes everywhere with runtime deadlock detection (caught a self-deadlock that would have taken hours to find with \`std::sync\`)  
\- Per-handler Prometheus metrics, structured JSON logging, deep health checks  
\- Chaos-tested failover (leader kill, network partition, kill-9 mid-write)  
\- Per-tenant quotas, load shedding, control plane replication  
\- 1178 core tests \+ chaos harness \+ cargo-fuzz \+ CRDT property tests  
\- 5 operational runbooks, watchdog with auto-restart

## SECTION 5: THE SCALE CEILING — ABSTRACT STRESS-TEST ANALYSIS

### 5.1 TOKEN EFFICIENCY: THE 99.9% SAVINGS

| Memories | File-Based | YantrikDB | Token Savings | Recall Precision |  
|----------|-----------|-----------|---------------|------------------|  
| 100 | 1,770 tokens | 69 tokens | 96% | 66% |  
| 500 | 9,807 tokens | 72 tokens | 99.3% | 77% |  
| 1,000 | 19,988 tokens | 72 tokens | 99.6% | 84% |  
| 5,000 | 101,739 tokens | 53 tokens | 99.9% | 88% |

At 500 memories, file-based memory exceeds 32K context. At 5,000, it doesn't fit in any model — not even 200K. YantrikDB stays at \~70 tokens per query.

Critical insight: Precision improves with more data — the opposite of context stuffing. More memories \= better embeddings \= more graph context \= higher precision.

### 5.2 SELECTIVE RECALL: O(1) VS. O(N)

The cognitive engine keeps a bounded working set — it loads only the K most relevant persistent nodes into a fresh \`WorkingSet\` on each query. Query cost is O(K), not O(N). The HNSW index provides O(log N) lookup.

The implication: Enterprise-grade longitudinal ingestion (decades of dense logs/code) doesn't degrade query performance. The working set remains bounded.

### 5.3 WRITE SATURATION

Sustained write throughput at c=4 (HTTP, pre-computed embedding): 381 writes/second sustained. Engine ceiling with writers=32, no compactor: 1,115 writes/second.

### 5.4 INDEX REBUILD CATASTROPHE

 5.4 Index Rebuild Catastrophe

The HNSW index is single-threaded, derived from SQLite as source of truth, and rebuilt on startup.

The scaling reality: At enterprise-scale densities (millions of vectors), HNSW rebuild shifts from milliseconds to seconds or minutes. This is a deliberate tradeoff: consistency and durability over instant startup.

Mitigations:  
\- The engine uses a \`DeltaIndex\` and \`ArcSwap\<SearchState\>\` to atomically publish new index states during rebuild  
\- Writes continue during rebuild  
\- The host application needs to handle startup latency

### 5.5 MEMORY FOOTPRINT

\- Bundled embedder: \~10 MB install, \~80ms cold start, no native ML deps  
\- ONNX MiniLM: \~150 MB install  
\- RAM scaling: The engine loads only the working set into memory. At scale, RAM usage is dominated by the HNSW index size (which grows logarithmically) and the working set (bounded).

No hard OOM wall — the engine degrades gracefully as SQLite page cache pressure increases.

### 5.6 CONSOLIDATION WALL

The consolidation pipeline runs on a bounded window (default 30 memories). Under a firehose of duplicate/contradictory data:  
\- Consolidation keeps up because it's bounded  
\- The backlog is managed via the \`consolidation\_limit\` param (default 100), which caps the O(n²) clustering work  
\- \`run\_pattern\_mining\` defaults to \`false\` — explicitly slow on large DBs, enable only when needed

Honest admission: The consolidation benchmark (59 memories: 8 canonical facts × 3-4 paraphrases \+ 6 seeded contradictions \+ 20 distractors) flagged 60 conflicts — \~54 were noise-or-ambiguous. The system is conservative — it surfaces potential issues for the agent to resolve rather than silently merging.

### 5.7 THE SCALE CEILING SUMMARY

| Component | Bottleneck | Mitigation |  
|-----------|-----------|------------|  
| Writes | SQLite WAL / Raft consensus | Batch writes, embedded mode for max throughput |  
| Reads | HNSW index size (logarithmic) | Working set bounds query cost |  
| Startup | HNSW rebuild from SQLite | DeltaIndex \+ ArcSwap for atomic cutover |  
| RAM | HNSW index \+ working set | Bounded working set, configurable HNSW parameters |  
| Consolidation | O(n²) clustering | Bounded window, configurable limits |  
| Storage | SQLite row count (billions) | Tombstone-based deletion, CRDT-friendly replication |

## SECTION 6: THE YANTRIKOS ENDGAME

### 6.1 THE ROAD TO YANTRIKOS

YantrikDB is the memory layer being built on the road to YantrikOS — an AI-native operating system where agents are first-class primitives, not apps on top.

The vision: Memory was the bottleneck, so they shipped it first. The cognitive memory engine is the foundation upon which an entire AI-native OS will be built — where agents have persistent identity, evolving relationships, and genuine memory that works like a human's.

 6.2 "Skill as Memory, Not Document" (May 2026\)

### 6.2 "SKILL AS MEMORY, NOT DOCUMENT" (MAY 2026\)

1\. Context stuffing failure — dumping all skills into context is O(n) and breaks at scale  
2\. Retrieval failure — vector-only retrieval misses skills that are semantically distant but contextually relevant  
3\. Forgetting failure — without decay, old skills crowd out new ones

The YantrikDB approach: Keep memory \~70 tokens vs. 100k context. This is academically validated and operationally superior for massive skill catalogs.

### 6.3 PROCEDURAL MEMORY: "AGENTS LEARN WHAT TO DO"

Beyond facts and events, YantrikDB stores procedural memory — strategies that worked before get recorded and reinforced. Agents learn what to do, not just what they know.

This enables:  
\- Skill acquisition: Agents record what worked and reinforce it  
\- Strategy optimization: Successful patterns get weighted higher  
\- Autonomous improvement: Agents evolve their own behavior without human retraining

### 6.4 DERIVED PERSONALITY

Stable tendencies are extracted from memory patterns over time:  
\- "This user prefers X, reacts to Y, values Z."  
\- Informs default agent behavior across sessions  
\- No prompting needed — the agent knows the user

 The Complete Signal — Final Assessment

 What YantrikDB Actually Is

YantrikDB is a cognitive memory engine — not a vector database, not a knowledge graph, not a key-value store. It's the first system built for memory as a living process: hierarchical, compressed, contextual, self-updating, emotionally weighted, time-aware, and predictive.

 The Five Fronts of Disruption — Revisited

| Front | The Shift | The Proof |  
|-------|-----------|-----------|  
| Architectural | Five indexes, one engine, sharing WAL and query planner | One 10MB file replaces five infrastructure pieces |  
| Cognitive | Forgets, consolidates, detects contradictions | 99.9% token savings, precision improves with scale |  
| Operational | Raft with documented asymmetry | 1178 tests \+ chaos harness \+ 5 runbooks |  
| Scale | O(1) recall, \~70 tokens/query | Works at 5,000+ memories where 200K context fails |  
| Agentic | 19 MCP tools \+ autonomous \`think()\` | Agents become proactive, not reactive |

 The Hard Truths

1\. HNSW rebuild on startup is the primary scaling bottleneck — seconds to minutes at enterprise scale  
2\. Cluster routing requires explicit leadership awareness until PR-6 ships  
3\. \`run\_pattern\_mining\` is explicitly slow — enable only when needed

 Why This Changes Everything

YantrikDB isn't just storage with operations. It's a living system that does work between conversations. It gives agents:  
\- Genuine reasons to initiate conversation — proactive triggers  
\- The ability to learn from experience — procedural memory  
\- A coherent identity across sessions — derived personality  
\- The capacity to scale indefinitely — O(1) recall, bounded working set

The net-net: If an agent lacks this memory architecture, it is fundamentally crippled compared to agents that have it. YantrikDB is the first system that makes sustained, evolving AI relationships possible at scale.