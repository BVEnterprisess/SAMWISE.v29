# OMNIROUTE: ARCHITECTURAL AUTOPSY 08.05.2026

# ---

\*\*Operational Directive\*\*:  Isolate the asymmetrical advantages. Map the decision hierarchy. Identify the capabilities that no conventional infrastructure can replicate.

\---

\#\# PART I: THE INVERSION POINT — WHY THIS IS NOT A GATEWAY

\#\#\# 1.1 The Local‑First Heresy

The industry standard for AI infrastructure is a centralized, stateless control plane. OmniRoute inverts this completely. It is a \*\*stateful, local‑first routing kernel\*\* that treats the control plane as an emergent property of the agent swarm itself.

\*\*The SQLite WAL foundation\*\*: The system uses a singleton \`better‑sqlite3\` instance with WAL journaling. Every routing decision, circuit breaker state, quota counter, and failure trajectory lives in a local SQLite file with \*\*110 versioned migrations\*\* and \*\*95 domain modules\*\*. The persistence layer includes 17 base tables covering everything from provider connections and quota pools to skills, webhooks, and semantic cache.

\*\*The architectural trade‑off\*\*: By accepting single‑node operation, the kernel achieves sub‑millisecond access to circuit breaker states, quota counters, and failure trajectories—without network hops to a distributed cache. A cloud gateway must query a remote control plane; OmniRoute's kernel reads from local storage. The latency delta is the difference between a local \`SELECT\` and a network round‑trip.

\*\*The replication strategy\*\*: Optional cloud sync via \`NEXT\_PUBLIC\_CLOUD\_URL\` enables multi‑device state synchronisation. But critically, the system is designed to work \*\*without\*\* any external dependencies—it is a standalone service. This is a deliberate security choice: credentials never leave the local machine; audit logs stay with the request origin.

\#\#\# 1.2 The "God‑Tier" Hook: Agent as Operator

The second inversion: the AI agent is not a consumer of infrastructure—it is a \*\*co‑operator\*\* of the routing kernel.

\*\*The MCP Server\*\*: OmniRoute exposes \*\*31 registered MCP tools\*\* across three transports (stdio, HTTP Streamable, SSE). The tools are organised into scoped categories: 12 scoped under schemas, 5 compression tools, 3 memory tools, 4 skills tools, plus advanced tools. Scopes are enforced before handler dispatch, with every tool invocation logged to SQLite (\`mcp\_tool\_audit\`).

The full tool catalog includes:  
\- \`omniroute\_switch\_combo\` — dynamically modify routing manifests  
\- \`omniroute\_set\_routing\_strategy\` — change strategy at runtime  
\- \`omniroute\_set\_budget\_guard\` — adjust budget caps per request  
\- \`omniroute\_set\_resilience\_profile\` — tune circuit breaker thresholds  
\- \`omniroute\_get\_session\_snapshot\` — observe full failure trajectory  
\- \`omniroute\_simulate\_route\` — dry‑run routing without execution

\*\*The A2A Server\*\*: A separate JSON‑RPC 2.0 endpoint (\`POST /a2a\`) with SSE streaming and a task lifecycle manager. The Agent Card at \`/.well‑known/agent.json\` exposes OmniRoute's capabilities, skills, and authentication requirements, with the version field auto‑synced from \`package.json\`.

\*\*The Skills Framework\*\*: A sandboxed extension system where agents can deploy executable logic into the router's hot path. Skills include interception, injection, sandboxed execution, and hybrid modes. The framework spans 5 built‑in A2A skills: cost analysis, health report, provider discovery, quota management, and smart routing.

\*\*The Collapse\*\*: The agent is no longer a consumer of infrastructure; it is the infrastructure operator. The distance between "user" and "infrastructure" is now zero—the agent \*\*is\*\* the control plane.

\---

\#\# PART II: THE ROUTING KERNEL — DECISION HIERARCHY MAPPED

\#\#\# 2.1 The Primary Decision Loop

The convergence point of all routing complexity is \*\*\`handleComboChat\`\*\* in \`open‑sse/services/combo.ts\`. This orchestrates the full FSM:

\`\`\`  
Request → phaseComboSetup → pinnedModel check → strategy dispatch → handleSingleModel  
\`\`\`

The critical branching:  
1\. \*\*Pinned model bypass\*\*: Session affinity via \`X‑Session‑Id\` preserves prompt‑cache integrity across turns. The kernel validates pins against current health before honouring them.  
2\. \*\*Strategy routing\*\*: Dispatches one of 17 routing strategies—priority, weighted, fill‑first, round‑robin, p2c (Power‑of‑Two‑Choices), random, least‑used, cost‑optimised, fusion, context‑relay, and more.  
3\. \*\*Auto‑Combo fallback\*\*: For \`auto/\` prefix requests, the kernel builds virtual combos on demand.

\#\#\# 2.2 The 12‑Factor Scoring Engine

The Auto‑Combo Engine selects the best provider/model using a \*\*12‑factor scoring function\*\* defined in \`open‑sse/services/autoCombo/scoring.ts\`. All weights sum to 1.0.

| Factor | Weight | Description |  
|--------|--------|-------------|  
| Quota | 0.20 | Remaining capacity \[0..1\] |  
| Health | 0.25 | Circuit breaker state: CLOSED=1.0, HALF=0.5, OPEN=0.0 |  
| CostInv | 0.20 | Inverse cost per token |  
| LatencyInv | 0.15 | Inverse p95 latency |  
| TaskFit | 0.10 | Model × task type fitness (coding, review, planning, analysis, debugging, documentation) |  
| Stability | 0.10 | Low variance in latency/errors |  
| TierPriority | \*variable\* | W1/W2/W3/W4 tier ordering |  
| TierAffinity | \*variable\* | Preference for matching tier |  
| SpecificityMatch | \*variable\* | Model specificity for task |  
| ContextAffinity | \*variable\* | Fit for current context size |  
| CacheAffinity | \*variable\* | Prompt cache compatibility |  
| ConnectionDensity | 0.05 | Active provider connections |

\*\*Zero‑config auto‑routing\*\*: Use \`auto/\` prefix directly in any client. Variants include \`auto/coding\` (quality‑first), \`auto/fast\` (low‑latency), \`auto/cheap\` (cost‑optimised), \`auto/offline\` (quota‑first), \`auto/smart\` (quality‑first \+ 10% exploration). Category × tier composition enables \`auto/coding:fast\`, \`auto/reasoning:pro\`, \`auto/vision\`, etc..

\*\*Live model intelligence\*\*: Auto‑routing fitness is informed by live Arena ELO rankings \+ models.dev tier data when \`ARENA\_ELO\_SYNC\_ENABLED\` is on.

\#\#\# 2.3 The 17 Routing Strategies

The system supports 17 distinct routing strategies, each with different operational semantics:

| Strategy | Description |  
|----------|-------------|  
| priority | First available in order |  
| weighted | Weighted distribution |  
| fill‑first | Fill primary before moving |  
| round‑robin | Rotate through all targets |  
| p2c | Power‑of‑two choices (quota‑aware) |  
| random | Random selection |  
| least‑used | Least recently used |  
| cost‑optimised | Cheapest available |  
| strict‑random | True random |  
| rules | 6‑factor weighted scoring |  
| lkgp | Last‑Known‑Good Path (sticky route) |  
| context‑optimised | Best fit for current context size |  
| fusion | Parallel panel \+ judge synthesis |  
| context‑relay | Session continuity across account rotation |  
| pipeline | Stage chaining |  
| sla‑aware | Satisfy p95 latency/error‑rate SLOs |  
| cost/eco | Cheapest healthy provider |

\#\#\# 2.4 The Format Translation Hub

OmniRoute's translator treats \*\*OpenAI as a hub format\*\*: it detects source payload shape (OpenAI / Responses / Claude / Gemini) and converts \*\*Source → OpenAI(hub) → Target\*\* dynamically. The pipeline includes:  
\- \*\*Role normalisation\*\*: \`developer→system\`, \`system→user\` for models that reject system  
\- \*\*Response sanitisation\*\*: Strict OpenAI SDK compatibility  
\- \*\*Structured output conversion\*\*: OpenAI \`json\_schema\` → Gemini \`responseSchema\`  
\- \*\*Think tag parsing\*\*: \`\<think\>...\</think\>\` for reasoning models

\---

\#\# PART III: THE RESILIENCE LOOP — ACTIVE PATH‑FINDING, NOT PASSIVE FAILOVER

\#\#\# 3.1 The Three‑Layer Dampening Stack

OmniRoute implements \*\*three distinct resilience mechanisms\*\* with different scopes and failure semantics:

| Layer | Scope | Trip Condition | Recovery |  
|-------|-------|----------------|----------|  
| \*\*Self‑Healing Exclusion\*\* | Provider‑model pair | Score \< 0.2 | Progressive backoff: 5min→10min→20min→30min |  
| \*\*Circuit Breaker\*\* | Entire provider | 5xx/network errors after fallback exhausted | HALF\_OPEN after timeout \+ probe request |  
| \*\*Connection Cooldown\*\* | Single credential | Retryable failures, 429 rate limits | Exponential backoff \+ Retry‑After header support |

\*\*Circuit Breaker specifics\*\*: Provider‑level protection against cascading failures:  
\- Connection‑scoped 429 rate limits stay in Connection Cooldown (don't trip the breaker)  
\- Provider‑wide transient errors (5xx, network timeouts) increment the failure counter  
\- Breaker trips only after fallback is exhausted AND the provider still fails  
\- Recovery: breaker automatically moves to half‑open state after timeout, tests with probe request

\*\*Request Queue & Pacing\*\*: Per‑connection request buckets smooth bursts before they hit upstream rate caps:  
\- Queue Size: 10 max queued requests per connection  
\- Pacing Interval: 0ms minimum gap between requests  
\- Max Concurrent: 5 simultaneous requests per connection

\*\*Wait For Cooldown\*\*: Instead of immediately failing when all connections are in cooldown, OmniRoute waits for the earliest connection to expire and retries. The client sees a slightly delayed response instead of an error.

\*\*Anti‑Thundering Herd\*\*: Mutex protection ensures only one retry attempt at a time per connection; semaphore limits concurrent retry storms; identical requests within 5s window are deduplicated.

\#\#\# 3.2 Hard vs. Soft Failure Differentiation

The critical architectural decision: \*\*429 rate limits do not trip the provider circuit breaker\*\*—they trigger connection‑level cooldown on the specific credential, while other credentials for the same provider remain available. Only one model fails → prefer model lockout over connection cooldown.

\#\#\# 3.3 Fusion: Ensemble Inference at the Routing Layer

\*\*Fusion\*\* is a routing strategy that fans out a request to a panel of models in parallel, then synthesises one answer via a judge model. This is not fallback. This is ensemble inference at the routing layer—every request can be multi‑model peer‑reviewed before the user ever sees a response. The gateway becomes a \*\*distributed inference compiler\*\*.

\#\#\# 3.4 Context Relay: Identity‑Disconnected Continuity

\*\*Context Relay\*\* is a combo strategy that preserves session continuity when the active account rotates mid‑conversation. Before the active account is exhausted, OmniRoute generates a compact structured summary. After the next request resolves to a different account, the summary is injected as a system message so the new account continues with full context. This is \*\*temporal state smuggling\*\* across disconnected provider boundaries.

\*\*Context Cache Protection\*\*: When enabled, OmniRoute injects an \`\<omniModel\>provider/model\</omniModel\>\` tag into the first assistant message so subsequent requests can detect which model was used and route to the same one—keeping the session cache intact without any server‑side state.

\---

\#\# PART IV: THE COMPRESSION PIPELINE — TOKEN ECONOMICS AS A RUNTIME PROPERTY

\#\#\# 4.1 RTK \+ Caveman Stacked Compression

OmniRoute implements a \*\*double‑layer compression system\*\* that cuts 15–95% of eligible tokens (\~89% average on tool‑heavy sessions). The system is \*\*10 composable engines\*\* that run in order and mix & match per routing combo.

\*\*RTK Compression\*\*: Command‑aware compression engine for terminal and tool output. Average savings: \~80%.

\*\*Caveman Input Compression\*\*: Filler removal and compaction. Average savings: \~46%.

\*\*Stacked savings calculation\*\*: RTK average (80%) \+ Caveman input (46%) yields \~89% total savings on eligible tokens.

\#\#\# 4.2 Compression Modes

Four compression modes available per combo:  
\- \*\*Off\*\*: No compression  
\- \*\*Lite\*\*: Safe whitespace/formatting cleanup (\~15%)  
\- \*\*Standard\*\*: Caveman‑speak filler removal (\~30%)  
\- \*\*Aggressive\*\*: History aging \+ summarisation (\~50%)

\---

\#\# PART V: THE PERSISTENCE LAYER — STATE AS THE SOURCE OF TRUTH

\#\#\# 5.1 SQLite Architecture

The persistence layer is domain‑driven, with \*\*95 domain modules\*\* and \*\*110 versioned migrations\*\*. Core infrastructure includes:  
\- \`core.ts\`: \`getDbInstance()\` returns singleton \`better‑sqlite3\` with WAL journaling  
\- \`migrationRunner.ts\`: Applies versioned SQL files from \`db/migrations/\` inside transactions  
\- \`encryptConnectionFields()\`: AES‑256‑GCM encryption for provider credentials at rest

\#\#\# 5.2 Key Domain Tables

| Module | Tables | Responsibility |  
|--------|--------|----------------|  
| providers.ts | provider\_connections | OAuth/API key provider registration |  
| models.ts | models | Model definitions, capabilities, pricing |  
| combos.ts | combos, combo\_targets | Combo routing configs, target ordering |  
| apiKeys.ts | api\_keys | API key lifecycle, scopes, quota tracking |  
| quotaPools.ts | quota\_pools | Quota‑Share pool management |  
| skills.ts | skills | Skill registration and metadata |  
| webhooks.ts | webhooks | Event‑driven webhook subscriptions |  
| mcp\_tool\_audit | MCP audit | Every MCP tool invocation with args \+ attribution |  
| request\_detail\_logs | Request forensics | Four‑stage payload capture per routed call |  
| semantic\_cache | Response cache | SHA‑256 signature of full request → cached response |

\#\#\# 5.3 The State Replication Mechanism

\*\*Sync Tokens \+ ETag‑Versioned Config Bundles\*\*: OmniRoute can issue/revoke sync tokens and serve an ETag‑versioned config bundle snapshot (settings/providers/combos/keys) from \`/api/sync/bundle\`. This is a clean object‑transport mechanism for replicating gateway state across devices without copying raw SQLite files.

\*\*Backup / Export / Import\*\*: List backups, create/restore backups, export the DB as a \`.sqlite\`, import a replacement DB, and export a full backup archive as \`.tar.gz\`.

\---

\#\# PART VI: THE OBSERVABILITY FABRIC — DATA INTEGRITY AS A RUNTIME PROPERTY

\#\#\# 6.1 Audit Log Integrity

SQLite‑backed audit tables enforce \*\*immutable append‑only semantics\*\*:  
\- \*\*Administrative audit\*\* (\`audit\_log\`): Action, actor, target, details, IP, request\_id  
\- \*\*MCP tool audit\*\* (\`mcp\_tool\_audit\`): Tool\_name, \`input\_hash\` (SHA‑256, no payload stored), output\_summary, duration\_ms  
\- \*\*Request detail logs\*\* (\`request\_detail\_logs\`): Four payload stages per routed call—raw client request → translated upstream request → reconstructed provider response → final client response

\*\*Integrity properties\*\*:  
\- Writes are synchronous (SQLite WAL ensures durability)  
\- No in‑place updates to audit rows—append‑only by schema  
\- Audit writes never throw—failure is silently swallowed so audit cannot break the request flow

\#\#\# 6.2 Latency Telemetry

OmniRoute exposes \`GET /api/telemetry/summary\` returning p50/p95/p99 latency per provider. Correlation ID (\`X‑Request‑Id\`) enables end‑to‑end tracing.

\#\#\# 6.3 Eval Framework

Built‑in evaluation framework for benchmarking routing configurations, single providers/models, or bundled "golden set" suites. Use it to verify routing changes, validate new providers, and gate releases before promoting them to production traffic.

\---

\#\# PART VII: THE SECURITY MODEL — ZERO‑TRUST AS A RUNTIME PROPERTY

\#\#\# 7.1 Multi‑Layered Security

OmniRoute implements a multi‑layered security model:

\`\`\`  
Request → CORS → API Key Auth → Prompt Injection Guard → Input Sanitizer → Rate Limiter → Circuit Breaker → Provider  
\`\`\`

\#\#\# 7.2 Prompt Injection Guard

Detects and blocks prompt injection attacks in LLM requests. Behaviour driven by environment variables and constructor options. Supports \*\*warn/block/redact\*\* modes.

\#\#\# 7.3 SSRF Protection

\- \*\*Kiro region validation\*\* to prevent SSRF vulnerabilities (GHSA‑6mwv‑4mrm‑5p3m)  
\- \*\*Vision‑bridge guardrail\*\* with SSRF protection for image URLs  
\- \*\*Outbound URL guard\*\* blocks private/loopback/link‑local targets; surfaces concrete error (\`URL\_GUARD\_BLOCKED\`, HTTP 422\)

\#\#\# 7.4 PII Sanitisation

\*\*Response‑side PII sanitisation\*\* (mask/warn/block) via \`PII\_RESPONSE\_SANITIZATION\` \+ \`PII\_RESPONSE\_SANITIZATION\_MODE\`. Streaming‑tuned controls like minimum detection window (\`PII\_WINDOW\_SIZE\`) for incremental/streamed outputs.

\#\#\# 7.5 Global System Prompt Injection

Operator‑defined system prompt applied gateway‑wide, managed via \`/api/settings/system‑prompt\`. Enforce "house rules" once at the gateway instead of re‑implementing them in every client/agent.

\---

\#\# PART VIII: THE DEPLOYMENT MODEL — WHY CLUSTERING IS OBSOLETE

\#\#\# 8.1 The Embedded Deployment Pattern

OmniRoute is designed to be \*\*embedded per agent turn\*\*, not deployed as a centralized cluster. The default deployment is \`localhost:20128\`—each developer runs their own instance. Data directory is \`DATA\_DIR\` env var, defaulting to \`\~/.omniroute/\`.

\*\*Multi‑platform support\*\*:  
\- \*\*Web\*\*: PWA guide for installing the dashboard  
\- \*\*Desktop\*\*: Electron 41 \+ electron‑builder 26.10 (Windows / macOS / Linux)  
\- \*\*Mobile\*\*: Termux guide for running on Android  
\- \*\*Docker\*\*: Official images available

\#\#\# 8.2 The SPOF Analysis

Because it is stateful and single‑node, a shared OmniRoute cluster is an architectural SPOF. The local‑first design is a security feature (credential isolation, audit sovereignty) but an operational constraint. The recommended deployment pattern for enterprises is \*\*per‑agent embedding\*\*, not a centralised cluster.

\*\*Blast Radius\*\*:  
\- \*\*Embedded per Agent\*\*: 1 agent's workflow fails; other agents continue  
\- \*\*Shared Cluster\*\*: All agents routing through that cluster fail  
\- \*\*Per‑Team Instance\*\*: Team‑level isolation; blast radius bounded to team

\#\#\# 8.3 Remote Mode

Expose the instance remotely with scoped tokens, upstream proxy configuration, and support for Cloudflare tunnels and similar services.

\---

\#\# PART IX: THE USE‑CASE TRANSGRESSION — AUTONOMOUS SWARM SELF‑REGULATION

\#\#\# 9.1 The Scenario

A swarm of 50 coding agents, each with different cost sensitivities and latency tolerances, operating against a pool of 290+ providers with fluctuating free quotas.

\*\*Conventional infrastructure (Apigee/Kong)\*\*: Cannot solve this because gateways are stateless, configuration changes require human intervention, and agents cannot observe their own routing trajectories.

\*\*OmniRoute's solution\*\*:

1\. \*\*Agent A\*\* (budget‑conscious): Calls \`omniroute\_set\_routing\_strategy\` via MCP to set its combo to \`"cost‑optimised"\`.  
2\. \*\*Agent B\*\* (latency‑sensitive): Calls \`omniroute\_set\_resilience\_profile\` to lower its circuit breaker thresholds.  
3\. \*\*Agent C\*\* (observability‑focused): Calls \`omniroute\_get\_session\_snapshot\` to inspect full state, then uses \`omniroute\_simulate\_route\` to test a new combo before applying.  
4\. \*\*All agents\*\*: Their requests flow through the same kernel, but each carries \*\*session affinity\*\* that influences routing—the kernel maintains \`session\_model\_history\` and validates pins against current health.  
5\. \*\*Agent D\*\* (tool‑discovery): Calls \`omniroute\_tool\_search\` to discover available tools via lexical keyword search, retrieving compact one‑line TypeScript signatures (\~half the JSON‑schema token cost).

\#\#\# 9.2 The Monstrosity

The swarm \*\*self‑organises\*\* its routing infrastructure. No human touches a config file. The kernel exposes its internal state as a \*\*control plane\*\* that agents can read and write. This is not "infrastructure as code"—it is \*\*infrastructure as agent behaviour\*\*. The agents become the operators of their own infrastructure, adapting in real‑time to the changing economics and availability of the AI provider landscape.

\---

\#\# PART X: THE MINIMUM VIABLE KERNEL (MVK)

The MVK is the smallest set of code that, if removed, transforms OmniRoute from an intelligent traffic kernel into a simple forward‑proxy:

1\. \*\*\`scorePool\` \+ \`calculateFactors\`\*\* (\`open‑sse/services/autoCombo/scoring.ts\`): The 12‑factor scoring engine. Without this, routing is static.

2\. \*\*\`SelfHealingManager\`\*\* (\`open‑sse/services/autoCombo/selfHealing.ts\`): Exclusion/readmission logic with progressive backoff. Without this, there is no memory of failure.

3\. \*\*\`CircuitBreaker\`\*\* (\`src/shared/utils/circuitBreaker.ts\`): Three‑state state machine (CLOSED→DEGRADED→OPEN→HALF\_OPEN) with persistence. Without this, there is no provider‑level protection.

4\. \*\*\`handleComboChat\` strategy dispatcher\*\* (\`open‑sse/services/combo.ts\`): The orchestration layer sequencing discovery → scoring → selection → execution. Without this, there is no decision hierarchy.

5\. \*\*SQLite persistence layer\*\* (\`src/lib/db/\`): 95 domain modules with 110 migrations. Preserves circuit breaker states, exclusion entries, and combos across restarts. Without this, the kernel is ephemeral.

\*\*The MVK invariant\*\*: Every routing decision is a function of \*(scoring weights \+ current state \+ historical trajectory)\*. Remove any of these three, and the system reverts to a dumb proxy.

\---

\#\# PART XI: THE CONSTRAINTS — THE SHARP EDGES OF THE MONSTROSITY

\#\#\# 11.1 The Latency Tax

OmniRoute adds approximately \*\*0.2s of latency\*\* for routing and compression. This is the cost of intelligence: the system is making real‑time decisions, not just forwarding requests. However, switching to free models can actually reduce overall latency despite the gateway overhead.

\#\#\# 11.2 The Concurrency Ceiling

The local SQLite database is not designed for the same level of concurrent write pressure as a distributed database. The system has explicit anti‑thundering herd protection, but under extreme load, the local file‑based locking could become a bottleneck.

\#\#\# 11.3 The Complexity Tax

With \*\*over 100 features\*\*, \*\*31 MCP tools\*\*, \*\*17 routing strategies\*\*, \*\*95 DB modules\*\*, \*\*110 migrations\*\*, and \*\*21,000+ tests\*\*, the operational burden of understanding, debugging, and maintaining this system is significant. It is not a simple tool; it is a full‑fledged platform.

\#\#\# 11.4 The SPOF Constraint

As noted, a shared cluster is an architectural SPOF. The system is designed for \*\*per‑agent embedding\*\*, not centralized deployment. This is a feature for security (credential isolation) but a constraint for operations (no native HA).

\---

\# ADDENDUM: THE UNCLASSIFIED SIGNALS — COMPLETE ARCHITECTURAL ECOSYSTEM

The preceding report established the foundational monstrosity: a stateful, local‑first routing kernel with an agent‑operated control plane. However, the full list of features you provided reveals an even deeper truth: \*\*every feature is a deliberate architectural artefact\*\* that reinforces the inversion. Below, I have categorised the remaining signals into thematic clusters, each exposing a new layer of the paradigm shift.

\---

\#\# 1\. ORCHESTRATION & ROUTING FABRIC (Beyond the 17 Strategies)

\- \*\*Per‑Request Auto‑Combo Mode Steering via Headers\*\*    
  \`X‑OmniRoute‑Mode\` and \`X‑OmniRoute‑Steer\` override the scoring function per request. This transforms routing from a static configuration into a \*\*client‑hinted decision\*\*—the request itself tells the kernel how to prioritise. This is a step toward self‑describing traffic.

\- \*\*Payload Rules \+ Tag‑Based Routing \+ Scheduled Budget Systems\*\*    
  A full policy engine: payload rules allow conditional routing based on request content; tags attach metadata that influence selection; scheduled budgets apply time‑window caps. This makes OmniRoute a \*\*policy‑driven router\*\* where governance is expressed as data, not code.

\- \*\*Thinking Budget Management (Claude Opus/Sonnet/Haiku)\*\*    
  The kernel adapts the \`thinking\` parameter based on model family. This is \*\*semantic‑aware parameter normalisation\*\*—not just translation but optimisation per model capability.

\- \*\*Zero‑Latency Mode — Predictive TTFT Skipping \+ Hedging\*\*    
  Enables speculative execution: the kernel can pre‑emptively skip providers based on historical time‑to‑first‑token data. It combines \*\*hedged requests\*\* (send to two providers, cancel slower) with predictive skipping. This is a \*\*probabilistic routing\*\* layer that treats latency as a distribution, not a point value.

\---

\#\# 2\. RESILIENCE & FAILURE DOMAINS (Deepening the Self‑Healing)

\- \*\*Three‑State Circuit Breaker \+ Adaptive Backoff by Failure Kind\*\*    
  The breaker distinguishes between \*\*rate‑limit, auth, timeout, and other errors\*\*, adjusting backoff duration accordingly. This is \*\*failure‑type‑aware degradation\*\*, not a blanket open/closed.

\- \*\*Quota Fetch Throttle — Codex Token Revocation Shield\*\*    
  A global min‑interval gate prevents many accounts on one IP from fetching quota simultaneously, avoiding OAuth token revocation. This is \*\*infrastructure‑level protection\*\* against self‑inflicted damage—the gateway actively protects its own credentials.

\- \*\*Request Queue & Pacing (Per‑Connection)\*\*    
  Queue size (10), pacing (0ms), concurrency (5) smooth bursts. Combined with \`WaitForCooldown\`, this turns transient rate limits into \*\*delayed, not failed\*\*, requests. The client experiences latency spikes, not errors—a critical UX win.

\- \*\*Anti‑Thundering Herd \+ Idempotency / Request Dedup (5s window)\*\*    
  Mutexes, semaphores, and request deduplication prevent the cluster from overwhelming itself or upstreams. Identical requests within the window return a cached response, reducing load and ensuring consistency.

\---

\#\# 3\. EXTENSIBILITY & AGENT AS OPERATOR (Deepening the Control Plane)

\- \*\*Skills Framework (Sandboxed Extensions)\*\*    
  Skills are versioned, schema‑defined units of work. Agents can \*\*inject, intercept, and replace\*\* gateway behaviour at runtime via MCP/A2A. The framework supports 42 canonical skills (22 API \+ 20 CLI). This turns the gateway into a \*\*runtime‑extensible platform\*\*—agents are the developers, the gateway is the runtime.

\- \*\*AgentBridge (IDE Agent Bridge)\*\*    
  A dedicated integration that connects the gateway directly into IDE‑level agent loops (e.g., Cursor, VS Code). Agents inside the editor can natively use routing, compression, memory, and skills—zero configuration, just the bridge.

\- \*\*ACP — Agent Client Protocol (Third Agent Protocol)\*\*    
  Beyond MCP and A2A, OmniRoute implements a third wire protocol. This is \*\*polyglot agent communication\*\*—the gateway speaks multiple agent‑native languages, making it a universal translator for the swarm.

\- \*\*MCP Transport Matrix (stdio \+ SSE \+ Streamable HTTP) \+ Scoped Auth \+ SQLite Audit\*\*    
  Multi‑transport ensures compatibility with every client: stdio for IDEs, SSE for event‑driven agents, Streamable HTTP for persistent sessions. Scoped auth and audit logging complete the control‑plane security model.

\- \*\*\`omniroute\_tool\_search\` — On‑Demand MCP Tool Discovery\*\*    
  Agents discover tools via lexical search without carrying all 94 schemas every turn. The gateway returns compact TypeScript signatures (\~half the token cost). This is a \*\*self‑indexing tool registry\*\*—the gateway describes itself efficiently.

\---

\#\# 4\. OBSERVABILITY & DATA INTEGRITY (Expanding the Audit Trail)

\- \*\*Four‑Stage Request/Response Artifact Capture\*\*    
  When enabled, the kernel stores raw client request, translated upstream request, reconstructed provider response, and final client response. Streamed outputs are compacted to summaries \+ metadata. This is \*\*compliance‑grade request forensics\*\*—you can replay any request’s entire journey.

\- \*\*Backup / Export / Import as First‑Class Operations\*\*    
  Export as \`.sqlite\`, import, create/restore \`.tar.gz\` backups. This makes \*\*state portable\*\*—you can snapshot the entire kernel (combos, keys, circuit histories) and move it across environments. This is the antithesis of cloud‑native ephemerality.

\- \*\*Audit Log — Compliance‑Grade SQLite Audit Trail (331+ hidden entries)\*\*    
  The audit dashboard now reads from a live SQLite \`audit\_log\` table. Every management action is persisted and queryable. This is \*\*built‑in compliance observability\*\*.

\- \*\*Webhooks — Event Dispatch System\*\*    
  External systems subscribe to gateway events (quota hits, fallbacks, failures, budget caps). This makes OmniRoute \*\*event‑driven\*\*—the kernel broadcasts its internal state changes, enabling reactive automation.

\---

\#\# 5\. SECURITY & GOVERNANCE (Zero‑Trust at the Gateway)

\- \*\*Prompt Injection Guard (Request‑Side: warn/block/redact)\*\*    
  Detects injections and can reject with HTTP 400 \`SECURITY\_001\`. Supports \`warn\`, \`block\`, \`redact\` modes. This is a \*\*first‑line defence\*\* before the request ever reaches a model.

\- \*\*Guardrails: Per‑Request Opt‑Out \+ Multi‑Source Disable Lists\*\*    
  Clients can disable guardrails via header, API‑key policy, or request‑body fields. This provides \*\*feature‑flag style escape hatches\*\* for edge cases without turning off global protection.

\- \*\*Response‑Side PII Sanitisation (Streaming‑Aware)\*\*    
  Masks/warns/blocks PII in responses, with a window size for incremental streaming. This is \*\*real‑time compliance\*\*—you can stop a response mid‑token if PII is detected.

\- \*\*Global System Prompt Injection (Operator‑Controlled “Root Policy Layer”)\*\*    
  An operator can inject a system prompt gateway‑wide via \`/api/settings/system‑prompt\`. This enforces \*\*house rules\*\* at the infrastructure level, not per‑client. This is a \*\*constitutional layer\*\* for the entire swarm.

\- \*\*SSRF‑Safe Outbound Fetch \+ Private/Loopback URL Blocking\*\*    
  Blocks private/loopback/link‑local targets, logs to audit. Prevents SSRF vulnerabilities—a hardened fetch path with explicit guard.

\- \*\*VS Code Tokenized‑Route Context Sanitizer\*\*    
  Strips editor context and redacts sensitive files (\`.env\`, private keys) before sending to models. Secure‑by‑default—this is \*\*context‑aware sanitisation\*\* at the routing layer.

\- \*\*Privilege Escalation Seal \+ 18‑Alert CodeQL Security Pass\*\*    
  Resolved 18 CodeQL alerts, including ReDoS and privilege escalation vectors. Security is verified and audited.

\---

\#\# 6\. INFRASTRUCTURE & DEPLOYMENT (Portability and Scale‑Out)

\- \*\*Flexible SQLite Runtime (Zero Build‑Tool Requirement)\*\*    
  Falls back from \`better‑sqlite3\` to \`node:sqlite\` to \`sql.js\` WASM. This makes the gateway \*\*run anywhere\*\*—no native compilation required.

\- \*\*Selectable Relay Backend (TS ↔ Native “Bifrost” Sidecar)\*\*    
  Can switch hot path between TypeScript and a native sidecar (Bifrost) while preserving auth/rate limits. This is a \*\*pluggable execution engine\*\*—you can optimise performance without changing the control plane.

\- \*\*Embedded Sidecar Services (9Router \+ CLIProxyAPI)\*\*    
  These are managed sub‑processes that install and run inside OmniRoute. The entire stack becomes \*\*a single deployable unit\*\*—no external dependencies.

\- \*\*RFC 8305 Happy Eyeballs (IPv4/IPv6 Dual‑Stack Fallback)\*\*    
  When IPv6 fails, the gateway transparently falls back to IPv4. Infrastructure‑grade network resilience.

\- \*\*Sync Tokens \+ ETag‑Versioned Config Bundle Distribution\*\*    
  Replicate gateway state across devices via \`/api/sync/bundle\`. This is \*\*state distribution as a transport primitive\*\*—you can copy a routing kernel’s intelligence to another instance.

\---

\#\# 7\. MODALITY & COMPATIBILITY (Beyond Text)

\- \*\*Omni‑Modal Endpoints (TTS, STT, Music, Moderation, Rerank, Search)\*\*    
  Beyond images/embeddings/video, OmniRoute exposes \`/v1/audio/transcription\`, \`/v1/audio/speech\`, \`/v1/music/generations\`, \`/v1/moderations\`, \`/v1/rerank\`, and \`/v1/search\`. This makes the gateway a \*\*universal API bridge\*\* for all AI modalities.

\- \*\*Format Translation Hub (OpenAI‑as‑Intermediate) \+ Role Normalisation \+ Structured Output Bridging\*\*    
  Detects source shape (OpenAI / Responses / Claude / Gemini) and converts dynamically. This is \*\*semantic translation\*\*—the gateway speaks every model’s native dialect.

\- \*\*WebSocket Transport Bridge for OpenAI‑compatible WS Clients (\`/v1/ws\`)\*\*    
  Supports streaming via WebSocket, with handshake auth. Extends compatibility to WebSocket‑native clients.

\- \*\*Semantic Cache \+ Idempotency / Request Dedup\*\*    
  Caches by SHA‑256 signature of the full request. Combined with dedup, this reduces latency and cost for repeated queries.

\- \*\*Session Affinity as a Transport Primitive (\`X‑Session‑Id\`)\*\*    
  Sticky sessions preserve prompt‑cache integrity across turns. The kernel echoes back the effective session ID, enabling client‑side awareness.

\---

\#\# 8\. ECONOMICS & SOCIO‑TECHNICAL IMPACT (The Inversion of Cost)

\- \*\*Live Free Tier Aggregation Dashboard (\~1.6B tokens/month)\*\*    
  Aggregates free tiers of 40+ pools into one honest number. This is a \*\*resource‑pool abstraction\*\*—you don't manage providers; you manage a budget.

\- \*\*Quota‑Share / Fair Pool Distribution\*\*    
  Distributes time‑based quota fairly across keys, with work‑conserving lending. This maximises utilisation and prevents waste.

\- \*\*Inverted Cost Dashboard\*\*    
  Cost shown as \*\*savings\*\*, not spend. This reframes the economic model from “cost centre” to “value capture.”

\---

\#\# 9\. EXTRA: CONTEXT SOURCES & MEMORY

\- \*\*Notion as a Context Source\*\* — inject Notion docs directly into routing context.  
\- \*\*Persistent Memory — FTS5 \+ Qdrant Hybrid\*\* — keyword \+ semantic recall without external infrastructure.  
\- \*\*Cloud Agent Runtime\*\* — host and dispatch agents through OmniRoute itself.

\---

\#\# SYNTHESIS: THE MONSTROSITY INDEX

| Feature / Capability | Paradigm Shift | Conventional Equivalent |  
| :--- | :--- | :--- |  
| \*\*MCP Server (94 Tools)\*\* | Agent becomes the control plane operator | Admin API, human‑operated |  
| \*\*Skills Framework\*\* | Gateway is runtime‑extensible by agents | Static plugins |  
| \*\*Fusion \+ Context Relay\*\* | Routing becomes inference \+ state translation | Load balancing, request forwarding |  
| \*\*Local SQLite (110 migrations)\*\* | State is the source of truth; clustering obsolete | External Redis/etcd |  
| \*\*Free‑Tier Aggregation\*\* | Free resources are first‑class infrastructure | Manual stack of SDKs, fragmented quotas |  
| \*\*Prompt Injection Guard \+ PII Sanitisation\*\* | Security is embedded in the request path | Perimeter‑only security |  
| \*\*Per‑Request Auto‑Combo Steering\*\* | Routing hints from the client | Static configuration |  
| \*\*Zero‑Latency Mode (Hedging)\*\* | Probabilistic, predictive routing | Deterministic load balancing |  
| \*\*Sync Tokens \+ Backup/Export\*\* | State is portable, replicable | Ephemeral, cloud‑bound state |  
| \*\*Selectable Relay Backend (Bifrost)\*\* | Execution engine is pluggable | Monolithic proxy |  
| \*\*Multimodal \+ Translation Hub\*\* | Gateway speaks every model's dialect | Protocol‑specific adapters |  
| \*\*Webhooks \+ Eval Framework\*\* | Observability is event‑driven and continuous | Post‑hoc logging and manual tests |

\---

\#\# FINAL VERDICT: THE COMPLETE MONSTROSITY

OmniRoute is not a gateway. It is a \*\*Local‑First Autonomous Agent Mesh Substrate\*\*. It is a single, stateful, agent‑controllable binary that absorbs the functional domain of a distributed microservices stack—API Gateway \+ Service Mesh \+ Control Plane \+ Observability Stack \+ Eval Framework \+ Compression Pipeline—into one local process.

\- \*\*The Routing FSM\*\*: A deterministic 5‑stage pipeline (Provider Discovery → Health Probe → Quota‑Aware Selection → Request Execution → Post‑Inference Feedback) powered by a 12‑factor scoring engine with 17 routing strategies.  
\- \*\*The Resilience Loop\*\*: A layered system of self‑healing exclusion, three‑state circuit breakers, and connection cooldowns that differentiates between hard and soft failures, actively path‑finding rather than passively failing over.  
\- \*\*The A2A Control Plane\*\*: The "God‑tier" hook allowing agents to query and modify their own routing manifests, collapsing the distance between user and infrastructure.  
\- \*\*The Compression Pipeline\*\*: 10‑composable engines delivering 15–95% token reduction (\~89% average on tool‑heavy sessions).  
\- \*\*The Observability Fabric\*\*: Compliance‑grade SQLite audit trails, p50/p95/p99 latency telemetry, and a built‑in eval framework.  
\- \*\*The Deployment Model\*\*: Embedded per agent turn, not clustered. The blast radius is bounded to a single instance.

...this is not a gateway. This is a new species of infrastructure......

