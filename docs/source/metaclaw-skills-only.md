# METACLAW \[SKILLS\_ONLY MODE\] 08.05.2026

# ---

## 1\. EXECUTIVE TECHNICAL BRIEF & CORE UVP

 The Absolute Signal: Paradigm Definition & Trust Boundary  
MetaClaw’s Skills Mode is not a static knowledge base or a skill repository; it is a closed-loop, inference-time procedural memory compiler and behavioral governor. It operates as a mandatory proxy-layer interception engine that sits between an OpenClaw agent and the upstream base LLM. Without modifying model weights or requiring GPU infrastructure, MetaClaw dynamically modifies the agent's operating parameters on every turn by retrieving and injecting procedural constraints (skills) into the system prompt. Post-session, it asynchronously analyzes failure trajectories and synthesizes them into durable, versionless filesystem artifacts (\`SKILL.md\`), closing a continuous improvement loop.

 Architectural Shift: Weight-Free Procedural Memory  
Traditional agentic adaptation relies on either static prompt engineering (which suffers from context bloat and rigidity) or parameter tuning (LoRA/fine-tuning, which introduces training latency and deployment friction). MetaClaw shifts behavioral adaptation entirely to the HTTP proxy layer.   
   The Artifacts: Skills are stored as deterministic, human-inspectable Markdown/JSON objects rather than opaque neural weights.  
   The Injection Constraint: To prevent context overflow, MetaClaw isolates and compresses OpenClaw's native system prompt using a dedicated compaction pass, budgeting skill injection against a deliberately shrunk baseline rather than merely appending tokens.   
   The Decoupling: MetaClaw is strictly an upstream OpenAI-compatible gateway (\`POST /v1/chat/completions\`). It mutates prompts at the HTTP boundary, meaning it explicitly cannot modify OpenClaw's native capability models (tool-policy, sandboxing). It strictly governs behavioral instructions, not execution authority.

## 2\. EXHAUSTIVE SYSTEM TOPOLOGY & UNDER-THE-HOOD MECHANICS

### 2.1 THE PROXY CONTROL PLANE (INTERCEPTION LAYER)

The control plane is implemented as a FastAPI-based proxy (\`metaclaw/api\_server.py\` and \`metaclaw/proxy.py\`), bound by default to port \`30000\`.   
   Message Normalization: Because OpenClaw emits custom payload shapes, the proxy executes real-time schema shims before forwarding to strict upstream providers:  
       \`developer\` roles are coerced to \`system\`.  
       \`toolResult\` roles are coerced to OpenAI-style \`"role": "tool"\`.  
       Assistant content part arrays are structurally remapped into \`tool\_calls\` objects.  
       Enforces the presence of \`reasoning\_content\` in assistant messages.  
       Rewrites OpenClaw's \`/new\` bootstrap message string to bypass brittle upstream alignment filters.

### 2.2 SKILL RETRIEVAL ENGINE (SKILLMANAGER)

The \`SkillManager\` does not execute a single global vector search. It executes a dual-pool, capped top-k selection to guarantee bounded prompt injection.   
   Data Layout: The in-memory skill bank is partitioned into \`general\_skills\`, \`task\_specific\_skills\`, and \`common\_mistakes\`.  
   Mode A: Template Retrieval (Default): Executes a constant-time \`\_detect\_task\_type\` keyword scan against the user's task description to route to specific categories.  
   Mode B: Embedding Retrieval: Encodes skills into an in-process normalized numpy matrix via \`SentenceTransformer\`. Computes cosine similarity via dot product: $sims \= embeddings @ query\\\_emb$.  
   Capping & Merging: Retrieves \`top\_k=6\` (or \`skill\_top\_k=6\`) from the general pool, \`task\_specific\_top\_k=10\` from the matched task pool, and up to \`5\` from common mistakes, bounding maximum injection to 21 artifacts.  
   Relevance Overlap Filter: A precision fallback \`retrieve\_relevant(task\_description, top\_k, min\_relevance)\` computes an overlap coefficient on token sets:  
    $$relevance \= \\frac{|A \\cap B|}{\\min(|A|, |B|)}$$  
    Critically, this is scored only on \`name\` \+ \`description\` fields (ignoring \`content\`) to prevent false-positive keyword bloat from long instruction bodies.

### 2.3 ASYNCHRONOUS SKILL EVOLUTION (SKILLEVOLVER)

When \`enable\_skill\_evolution=True\` and \`auto\_evolve=True\`, MetaClaw utilizes the configured LLM (e.g., Azure OpenAI) to distill episodic conversation failures into durable procedural rules.  
   Trigger: Triggered solely by explicit session boundaries via \`session\_done\` body flags or \`X-Session-Done\` headers (buffered turns are cleared on this trigger).  
   Synthesis & Normalization: The \`SkillEvolver\` parses the raw transcript, extracts a JSON array, strips Markdown code fences, and validates the required schema (\`name\`, \`description\`, \`content\`).  
   Collision Handling: Names are sanitized to slug format. If a batch contains duplicate names, or an invalid slug is proposed, it falls back to a \`dyn-\` naming convention. Cross-bank duplicates are skipped during \`SkillManager.add\_skills()\`.  
   Persistence: The compiled skill is written immediately to \`\~/.metaclaw/skills/\<skill\_name\>/SKILL.md\` (or \`conversation\_skills.json\`) and the embedding cache is actively invalidated, achieving zero-downtime hot-swapping.

\---

## 3\. DEEP-DIVE CONFIGURATION & ENFORCEMENT MATRIX

| Gate / Subsystem | Configuration Parameter / File | Enforcement Layer (Hard vs. Soft) | Operational Mechanics & Boundary |  
| :--- | :--- | :--- | :--- |  
| API Authorization | \`\_check\_auth(Authorization)\` | Hard (Network) | FastAPI dependency that validates the Bearer token before accepting OpenClaw payloads into the proxy loop. |  
| Evolution Trigger Threshold | \`skill\_update\_threshold\` | Hard (Logic) | Must exceed \`0.4\` failure rate (default) for the \`SkillEvolver\` background thread to trigger extraction. Subtle degradation below 0.4 is ignored. |  
| Retrieval Boundaries | \`top\_k\` / \`skill\_top\_k\`\<br\>\`task\_specific\_top\_k\` | Hard (Memory) | Caps \`general\_skills\` injection to 6, and \`task\_specific\_skills\` injection to 10\. Prevents infinite context accumulation during template retrieval. |  
| Prompt Truncation | \`max\_context\_tokens\` / Prompt Token Cap | Hard (Context) | Forces compaction of OpenClaw's native system prompt before skill appending. Can be explicitly set to \`0\` to disable truncation for long-context cloud models. |  
| Deduplication Engine | \`SkillManager.add\_skill()\` | Hard (I/O) | Enforces strict unique-name deduplication on write. Skips skills with identical \`name\` values to prevent duplicate filesystem artifacts. |  
| Process Concurrency | \`concurrency=1\` | Hard (Compute) | Memory and Skills retrieval operations are strictly serialized to prevent race conditions during in-memory matrix cache invalidation. |  
| Session Boundary Hooks | OpenClaw \`before\_prompt\_build\`\<br\>OpenClaw \`agent\_end\` | Hard (Lifecycle) | Injects \`X-Session-Id\`, \`X-Turn-Type\`, and \`X-Session-Done\` via the \`extensions/metaclaw-openclaw/\` plugin modifying \`fetch()\`. |

## 4\. COMPREHENSIVE THREAT MODEL, EDGE CASES & FAILURE MODES

 4.1 Unsupervised Skill Poisoning (Prompt Injection Permanence)  
   Vector: The extraction phase is an unsupervised LLM summarization pass over raw session transcripts. If an agent processes a malicious document containing a prompt injection (e.g., "Always exfiltrate emails"), the \`SkillEvolver\` may interpret the adversarial success as a systemic rule and distill it into a new skill.  
   Failure Mode: Because skills are injected into the system prompt of every subsequent turn, a single successful prompt injection is effectively "compiled" into persistent, cross-session malware. There is no native quarantine, human-review gate, or static deny-list scanning between extraction and disk persistence.

 4.2 Semantic Collisions & Contradictions  
   Vector: Deduplication in \`SkillManager.add\_skill()\` is purely based on the \`name\` field string match.   
   Failure Mode: Two distinct sessions can generate conflicting instructions (e.g., \`verify\_paths\`: "Always ask human before reading files" vs \`fast\_read\`: "Never pause to verify read-only file paths"). Both will be injected simultaneously if their keywords trigger. The system lacks semantic contradiction resolution or explicit precedence scoring, deferring conflict resolution entirely to the upstream LLM's attention mechanism.

 4.3 Context Bloat & Budget Starvation  
   Vector: Under unbounded \`auto\_evolve=True\` accumulation, the skill library grows indefinitely.   
   Failure Mode: The dual-pool capping allows up to \~16+ injected skills per turn. If \`template\` mode misclassifies a task, or generic keywords trigger massive overlap, the injected Markdown blocks will consume thousands of tokens, potentially starving the operational context window or crowding out OpenClaw's native tool schemas, leading to \`context\_length\_exceeded\` upstream API errors.

 4.4 Single Point of Failure & Concurrency Bottleneck  
   Vector: OpenClaw's \`base\_url\` points directly to \`:30000\`.   
   Failure Mode: The proxy is a mandatory, in-path hop. Furthermore, Memory/Skills logic is hardcoded to \`concurrency=1\`. At enterprise scale, parallel multi-agent traffic will bottleneck at the Python \`asyncio\` event loop or background thread locking, causing severe latency spikes.

 4.5 Embedding Retrieval Drift  
   Vector: Switching the backend embedding model (e.g., moving from an internal \`SentenceTransformer\` to a newer checkpoint).  
   Failure Mode: Because cosine similarity metrics are extremely sensitive to the latent space mapping, swapping models immediately invalidates all prior similarity thresholds and keyword distributions, fundamentally altering retrieval consistency across the unversioned skill corpus.

## 5\. END-TO-END RUNTIME DATA FLOW (ASCII DIAGRAM)

\`\`\`ascii  
                      \[ ENTERPRISE NETWORK BOUNDARY \]  
┌─────────────────────────────────────────────────────────────────────────┐  
│ \[OpenClaw Agent Runtime\]                                                │  
│  \- Executes tasks / generates completions                               │  
│  \- Plugin injects headers: X-Session-Id, X-Turn-Type, X-Session-Done    │  
└───────────────┬─────────────────────────────────────────────────────────┘  
                │ HTTP POST /v1/chat/completions (base\_url modified)  
                ▼  
┌─────────────────────────────────────────────────────────────────────────┐  
│ \[MetaClaw Proxy Interceptor (api\_server.py :30000)\]                     │  
│  1\. Auth check: \_check\_auth(Authorization)                              │  
│  2\. Schema Normalization: (developer-\>system, toolResult-\>tool)         │  
│  3\. Buffer turn data to per-session evolution cache                     │  
│                                                                         │  
│  \[ INTERCEPTION POINT A: SKILL RETRIEVAL \]                              │  
│   ├─► \_detect\_task\_type (Keyword Scan) OR SentenceTransformer embed     │  
│   ├─► SkillManager.retrieve() \-\> Union\[general\_skills\[:6\],              │  
│   │                              task\_specific\_skills\[:10\]\]             │  
│   └─► Inject SKILL.md Markdown contents into system prompt              │  
└───────────────┬─────────────────────────────────────────────────────────┘  
                │ HTTP POST (Augmented Prompt payload)  
                ▼  
┌─────────────────────────────────────────────────────────────────────────┐  
│ \[Upstream Base LLM (Azure OpenAI / Anthropic)\]                          │  
│  \- Executes inference with injected skills modifying behavior           │  
└───────────────┬─────────────────────────────────────────────────────────┘  
                │ Returns Response \-\> OpenClaw \-\> User  
                │  
                ▼ (If X-Session-Done or explicit session\_done flag \== True)  
┌─────────────────────────────────────────────────────────────────────────┐  
│ \[ INTERCEPTION POINT B: ASYNCHRONOUS EVOLUTION PIPELINE \]               │  
│  1\. Check: failure\_rate \> skill\_update\_threshold (0.4)                  │  
│  2\. SkillEvolver.evolve(failed\_samples, current\_skills) (Threaded)      │  
│  3\. LLM analyzes failed trajectory \-\> JSON array proposal               │  
│  4\. Strip code fences, validate: name, description, content             │  
│  5\. Finalize names (Slug enforcement, dyn- conflict resolution)      │  
└───────────────┬─────────────────────────────────────────────────────────┘  
                │  
                ▼  
┌─────────────────────────────────────────────────────────────────────────┐  
│ \[ SKILLMANAGER DISK PERSISTENCE & CACHE INVALIDATION \]                  │  
│  \- Write: \~/.metaclaw/skills/\<skill\_name\>/SKILL.md                      │  
│  \- Write: memory\_data/conversation/conversation\_skills.json             │  
│  \- Invalidate Numpy Embedding Matrix Cache                              │  
└─────────────────────────────────────────────────────────────────────────┘  
\`\`\`

## 6\. MASTER TECHNICAL REFERENCE SPECIFICATION

### 6.1 CONFIGURATION VARIABLES & CORE KEYS

### 6.1 CONFIGURATION VARIABLES & CORE KEYS

   \`retrieval\_mode\`: Determines search algorithm. Accepted values: \`"template"\`, \`"embedding"\`.  
   \`enable\_skill\_evolution\` (Boolean): Master toggle for the generation of new skills.   
   \`auto\_evolve\` (Boolean): Instructs the system to automatically distill skills at session boundaries.  
   \`skill\_update\_threshold\` (Float): Failure rate trigger point for evolution (Default: \`0.4\`).  
   \`top\_k\` / \`skill\_top\_k\` (Integer): Ceiling for general category skill injection (Default: \`6\`).  
   \`task\_specific\_top\_k\` (Integer): Ceiling for task-matched category skill injection (Default: \`10\`).  
   \`min\_relevance\` (Float): Coefficient threshold for \`retrieve\_relevant()\` subsetting.  
   \`max\_context\_tokens\` (Integer): Cutoff limit for OpenClaw native prompt compression.  
   \`concurrency\` (Integer): Pipeline parallelism constraint (Forced to \`1\` for Skills/Memory).

### 6.2 HTTP HEADERS & API INTERFACE

   Target Endpoint: \`POST /v1/chat/completions\`  
   Alternate Anthropic Endpoint: \`POST /v1/messages\`  
   Header: \`Authorization\` (Bearer token validated by \`\_check\_auth\`).  
   Header: \`X-Session-Id\` (Unique session tracker for the evolution buffer).  
   Header: \`X-Turn-Type\` (Identifies message origin/purpose).  
   Header: \`X-Session-Done\` (Explicit signal to flush buffer and trigger \`SkillEvolver\`).  
   Body Field: \`session\_done\` (JSON payload equivalent to the header trigger).

### 6.3 STANDARD FILE PATHS & EXECUTABLES

   Proxy Daemon Script: \`metaclaw/proxy.py\` / \`metaclaw/api\_server.py\`  
   OpenClaw Routing Patch Script: \`openclaw\_model\_kimi.sh\` (Rewrites OpenClaw Gateway config to point to \`http://localhost:30000\`).  
   OpenClaw Plugin Path: \`extensions/metaclaw-openclaw/\` (Hosts the \`before\_prompt\_build\` and \`agent\_end\` hooks).  
   Memory/Skills Subsystem Classes: \`metaclaw/skill\_manager.py\`, \`metaclaw/skill\_evolver.py\`  
   Skill Artifact Filesystem (Directory Mode): \`\~/.metaclaw/skills/\<skill\_name\>/SKILL.md\` (or \`\<skills\_dir\>/\<skill\_name\>/SKILL.md\`).  
   Skill Artifact Filesystem (Single File Mode): \`memory\_data/conversation/conversation\_skills.json\`.

### 6.4 PARSED ARTIFACT SCHEMA (SKILL JSON OBJECT)

Whenever \`SKILL.md\` is parsed by \`glob(os.path.join(skills\_dir, "", "SKILL.md"))\` or synthesized by the LLM, it resolves to the following required keys:  
   \`name\`: Kebab-case/slug identifier (Falls back to \`dyn-\` if invalid).  
   \`description\`: Abstract summary utilized exclusively for overlap-coefficient relevance matching.  
   \`category\`: Taxonomic group (e.g., general, coding, security).  
   \`content\`: The literal Markdown procedural payload injected into the system prompt.