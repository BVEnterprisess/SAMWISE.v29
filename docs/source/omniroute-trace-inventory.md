# OMNIROUTE: OBSERVABLE TRACE INVENTORY

# 08.05.2026

# ---

\#\# 1\. SYSTEM HEALTH & INFRASTRUCTURE

\*\*Source:\*\* \`GET /api/monitoring/health\` \+ \`src/lib/localHealthCheck.ts\` \+ \`src/lib/db/healthCheck.ts\`

| Trace |  
|-------|  
| System status (healthy/unhealthy) |  
| OmniRoute version |  
| Uptime (seconds) |  
| Active connections |  
| Database connection status (pass/fail) |  
| Database latency (ms) |  
| Database writeable status |  
| Database integrity status |  
| Foreign key violation count |  
| Orphaned artifact detection |  
| WAL file size |  
| Recent migration count |  
| Heap pressure (MB used) |  
| Heap pressure threshold (MB) |  
| Memory usage (MB) |  
| CPU usage |  
| Heap pressure indicator |  
| Last 10 errors with stack traces |  
| Port availability |  
| Native dependency status |

\---

\#\# 2\. PROVIDER HEALTH & RESILIENCE

\*\*Source:\*\* \`src/lib/monitoring/providerHealthAutopilot.ts\` \+ \`src/lib/monitoring/providerHealthMatrix.ts\` \+ \`domain\_circuit\_breakers\` \+ \`domain\_lockout\_state\` \+ \`provider\_connections\`

| Trace |  
|-------|  
| Health score per provider/model |  
| Health status (green/yellow/red) per provider |  
| Circuit breaker state (CLOSED/DEGRADED/OPEN/HALF\_OPEN) per provider |  
| Circuit breaker failure count |  
| Circuit breaker last failure time |  
| Circuit breaker retry-after (ms) |  
| Circuit state change counter |  
| Connection count per provider |  
| Connection health scores per connection |  
| Connection last refresh time |  
| Connection test status |  
| Connection error code |  
| Connection last error |  
| Connection last error type |  
| Connection last error source |  
| Connection backoff level |  
| Connection rate\_limited\_until |  
| Connection consecutive\_failure\_count |  
| Connection consecutive\_use\_count |  
| Connection last\_used\_at |  
| Connection is\_active (enabled/disabled) |  
| Connection proxy\_enabled |  
| Connection per\_key\_proxy\_enabled |  
| Per-model lockout status |  
| Locked until timestamp |  
| Lockout reason |  
| Available models per provider |  
| Health per model |  
| Autopilot recommendations |  
| Autopilot issue severity (critical/warning/info) |  
| Autopilot recommended action type (clear\_breaker/cooldown/model\_lockout/reactivate/deactivate) |  
| Autopilot action risk level (low/medium/high) |  
| Token status (valid/expiring\_soon/expired/refresh\_failed) |  
| Token expiration timestamp |  
| Token last refresh timestamp |  
| Token next refresh timestamp |  
| Token consecutive refresh failures |  
| Token refresh sweep tick interval (60s) |  
| Per-connection health check interval (default 60min) |  
| Circuit open detection |  
| Circuit half-open detection |  
| Connection cooldown detection |  
| Stale connection error detection |  
| Terminal connection error detection |  
| Inactive connection detection |  
| Model lockout detection |  
| Quota monitor warning (80%+ usage) |

\---

\#\# 3\. REQUEST LOGS & TELEMETRY

\*\*Source:\*\* \`call\_logs\` table \+ \`usage\_history\` table \+ \`GET /api/telemetry/summary\`

| Trace |  
|-------|  
| Request ID |  
| Correlation ID (\`X-Request-Id\`) |  
| API key ID |  
| API key name |  
| Provider |  
| Model |  
| Connection ID |  
| Combo name |  
| Combo step ID |  
| Combo execution key |  
| HTTP status code |  
| Duration (ms) |  
| Input tokens |  
| Output tokens |  
| Compressed tokens |  
| Cost (USD) |  
| Retry count |  
| Error summary |  
| Error code |  
| Cache source |  
| Source format (OpenAI/Claude/Gemini) |  
| Target format |  
| Artifact relative path |  
| Artifact size (bytes) |  
| Artifact SHA-256 |  
| Has request body flag |  
| Has response body flag |  
| Has pipeline details flag |  
| Request summary |  
| Detail state (\`none\`/\`pipeline\`/\`full\`) |  
| Timestamp |  
| p50 latency per provider |  
| p95 latency per provider |  
| p99 latency per provider |  
| Request count |  
| Request latency histogram |  
| Tokens consumed counter |  
| Cost (USD) counter |  
| Provider errors counter |

\---

\#\# 4\. FOUR-STAGE REQUEST ARTIFACTS

\*\*Source:\*\* \`${DATA\_DIR}/call\_logs/\` \+ \`request\_detail\_logs\` table

| Trace |  
|-------|  
| Stage 1: Raw client request |  
| Stage 2: Translated provider request |  
| Stage 3: Raw provider response |  
| Stage 4: Translated client response |  
| SSE stream truncation data |  
| Stream metadata |

\---

\#\# 5\. ADMINISTRATIVE AUDIT LOG

\*\*Source:\*\* \`audit\_log\` table

| Trace |  
|-------|  
| Action (domain.verb pattern) |  
| Actor (system/user/API key) |  
| Target |  
| Details (JSON metadata) |  
| IP address |  
| Resource type |  
| Status |  
| Request ID |  
| Timestamp |

\*\*Specific audit actions logged:\*\*  
\- \`auth.login.success\`  
\- \`auth.login.failed\`  
\- \`auth.login.locked\`  
\- \`auth.login.error\`  
\- \`auth.login.misconfigured\`  
\- \`auth.login.setup\_required\`  
\- \`auth.logout.success\`  
\- \`provider.credentials.created\`  
\- \`provider.credentials.updated\`  
\- \`provider.credentials.revoked\`  
\- \`provider.credentials.batch\_revoked\`  
\- \`sync.token.created\`  
\- \`sync.token.revoked\`  
\- \`compliance.cleanup\`

\---

\#\# 6\. MCP TOOL AUDIT

\*\*Source:\*\* \`mcp\_tool\_audit\` table

| Trace |  
|-------|  
| Tool name |  
| Input hash (SHA-256, no payload stored) |  
| Output summary (truncated) |  
| Duration (ms) |  
| API key ID (caller) |  
| Success flag (1/0) |  
| Error code (on failure) |  
| Timestamp |

\*\*All 104 MCP tool names (source of truth):\*\*  
\- 42 canonical definitions (including six CCR lifecycle tools and agent-skills trio)  
\- 3 memory tools  
\- 4 skills tools  
\- 3 GitHub skills tools  
\- 6 pool tools  
\- 8 gamification tools  
\- 8 plugins tools  
\- 6 Notion tools  
\- 22 Obsidian tools  
\- 2 RTK-only compression tools

\---

\#\# 7\. PROXY LOGS

\*\*Source:\*\* \`proxy\_logs\` table

| Trace |  
|-------|  
| Proxy status |  
| Proxy latency (ms) |  
| Proxy error |  
| TLS fingerprint |  
| Timestamp |

\---

\#\# 8\. WEBHOOK EVENTS

\*\*Source:\*\* \`src/lib/webhookDispatcher.ts\`

| Trace |  
|-------|  
| \`request.completed\` — proxied request succeeds |  
| \`request.failed\` — proxied request fails after retries |  
| \`provider.error\` — provider returns circuit-breaking error |  
| \`provider.recovered\` — failing provider returns to healthy |  
| \`quota.exceeded\` — API key crosses budget/quota threshold |  
| \`combo.switched\` — combo strategy switches primary target |  
| \`test.ping\` — synthetic test event |  
| \`"\*"\` — wildcard for all events |

\*\*Webhook delivery metrics:\*\*  
\- Webhook URL  
\- Events subscribed (JSON array)  
\- HMAC secret (auto-generated)  
\- Enabled flag (0/1)  
\- Description (human label)  
\- Creation timestamp  
\- Last triggered timestamp  
\- Last HTTP status  
\- Failure count  
\- Delivery success/failure per attempt  
\- Retry count per delivery (max 3\)  
\- Retry backoff (1s, 2s, 4s)  
\- Timeout per attempt (10s)  
\- Auto-disable threshold (10 failures)

\---

\#\# 9\. QUOTA & BUDGET

\*\*Source:\*\* \`domain\_budgets\` \+ \`domain\_budget\_reset\_logs\` \+ \`domain\_cost\_history\` \+ \`/dashboard/quota\`

| Trace |  
|-------|  
| Daily limit (USD) |  
| Weekly limit (USD) |  
| Monthly limit (USD) |  
| Warning threshold |  
| Budget reset timestamp |  
| Last budget reset timestamp |  
| Reset timestamp |  
| Reset actor |  
| Reset reason |  
| Cost per timestamp |  
| Cost timestamp |  
| Current usage vs limit (progress bar) |  
| Quota trend (30-day chart) |  
| Next reset time |  
| Alert history |  
| Per-session quota monitors |  
| Quota status (healthy/warning/alert/exhausted/error) |  
| Remaining quota percentage |  
| Polling status (idle/in progress) |

\---

\#\# 10\. COMBO HEALTH & ROUTING

\*\*Source:\*\* \`/dashboard/combos\` \+ \`comboHealthAutopilot.ts\` \+ \`comboMetrics.ts\`

| Trace |  
|-------|  
| Strategy per combo |  
| Targets per combo |  
| Health per target |  
| Recent fallback events |  
| Success rate (24h) |  
| Success rate (7d) |  
| Success rate (30d) |  
| Unhealthy combo detection |  
| Target reordering recommendations |  
| Disable broken target suggestions |  
| Auto-removal of dead targets after N failures |  
| Per-step provider metrics |  
| Success rates per execution key |  
| Latency per execution key |  
| Historical usage per execution key |  
| Real-time provider scores |  
| 6-factor scoring breakdown (quota, health, cost, latency, task fitness, stability) |  
| Mode pack selector |  
| Incident mode status |  
| Excluded providers list |

\---

\#\# 11\. SESSION TRACKING

\*\*Source:\*\* In-memory session state \+ \`X-Session-Id\` header

| Trace |  
|-------|  
| Session ID (\`X-Session-Id\`) |  
| Effective session ID (\`X-OmniRoute-Session-Id\`) |  
| Session creation timestamp |  
| Session last active timestamp |  
| Request count per session |  
| Connection ID per session |  
| Session age (ms) |  
| Sticky session count |  
| Active session count |  
| Sticky-bound session count |  
| Per-API-key session breakdown |

\---

\#\# 12\. COMPRESSION & TOKEN EFFICIENCY

\*\*Source:\*\* Compression pipeline \+ \`/dashboard/compression\` \+ \`semantic\_cache\` table

| Trace |  
|-------|  
| RTK filter match count per request |  
| RTK filter resolution order (project → global → built-in) |  
| Caveman compression rules applied (30+ regex rules) |  
| Compression latency per request |  
| Ultra-mode binary-search truncation point |  
| Aggressive-mode message aging timestamp |  
| Compression savings histogram |  
| Cumulative compression savings chart |  
| Per-provider compression savings table |  
| Per-request compression stats (tokens saved, mode, techniques, latency) |  
| Cache hit count per entry |  
| Tokens saved by cache hits |  
| Cache key (SHA-256 signature) |  
| Cache response |  
| Cache creation timestamp |  
| Cache last hit timestamp |  
| Cache hit counter |  
| Compression mode per combo (\`Off\`/\`Lite\`/\`Standard\`/\`Aggressive\`/\`Ultra\`/\`RTK\`/\`Stacked\`) |  
| Per-combo compression personality (10-engine pipeline) |

\---

\#\# 13\. ALERTING

\*\*Source:\*\* \`src/lib/monitoring/observability.ts\`

| Trace |  
|-------|  
| \`provider\_circuit\_open\` — critical |  
| \`provider\_circuit\_half\_open\` — info |  
| \`quota\_warning\` — warning (80%+) |  
| \`quota\_exhausted\` — critical (100%) |  
| \`token\_refresh\_failed\` — warning (3+ failures) |  
| \`token\_expired\` — critical |  
| \`combo\_target\_unhealthy\` — warning (cooldown 1h+) |  
| \`db\_integrity\_warning\` — warning (FK violations \> 0\) |  
| \`heap\_pressure\` — warning (\>80% threshold) |  
| Dashboard banner |  
| Webhook (Slack, Discord, PagerDuty) |  
| Log (external log aggregation) |

\---

\#\# 14\. PERFORMANCE METRICS

\*\*Source:\*\* \`src/lib/monitoring/observability.ts\`

| Trace |  
|-------|  
| \`request\_count\` — counter |  
| \`request\_latency\_ms\` — histogram |  
| \`tokens\_consumed\` — counter |  
| \`cost\_usd\` — counter |  
| \`provider\_errors\` — counter |  
| \`circuit\_state\_changes\` — counter |  
| \`cache\_hits\` — counter |  
| \`compression\_savings\` — histogram |  
| \`quota\_used\` — gauge |  
| \`memory\_used\_mb\` — gauge |

\---

\#\# 15\. CLOUD SYNC

| Trace |  
|-------|  
| Cloud sync enabled flag |  
| Cloud sync machine ID |  
| Sync token created |  
| Sync token revoked |  
| Cloud sync status |  
| Sync verification status |  
| Last sync timestamp |

\---

\#\# 16\. COST & USAGE

\*\*Source:\*\* \`/dashboard/costs\` \+ \`costCalculator.ts\`

| Trace |  
|-------|  
| Cost by provider |  
| Cost by model |  
| Cost by API key |  
| Cost by account |  
| Cost by service tier |  
| Cost per request |  
| Weekly usage pattern |  
| Activity heatmap |  
| Raw usage history rows |  
| Per-request call logs |

\---

\#\# 17\. MEMORY & RETRIEVAL

\*\*Source:\*\* FTS5 \+ Qdrant hybrid memory system

| Trace |  
|-------|  
| FTS5 full-text search table (\`memory\_fts\`) |  
| Qdrant vector store (optional) |  
| Memory extraction facts |  
| Memory injection tokens |  
| \`x-omniroute-no-memory\` header opt-out |

\---

\#\# 18\. PROXY CONFIGURATION

\*\*Source:\*\* \`src/lib/db/proxies.ts\`

| Trace |  
|-------|  
| Global proxy config |  
| Per-provider proxy config |  
| Account/Connection proxy |  
| Provider proxy |  
| Proxy test endpoint |  
| Proxy pool status |  
| Playwright proxy resolution |  
| 4-level proxy resolution order |

\---

\#\# 19\. IP FILTER

| Trace |  
|-------|  
| IP filter mode |  
| Allowlist |  
| Blocklist |

\---

\#\# 20\. THINKING BUDGET

| Trace |  
|-------|  
| Thinking budget mode (passthrough/auto/custom/adaptive) |  
| Custom budget value |  
| Effort level |

\---

\#\# 21\. SYSTEM PROMPT

| Trace |  
|-------|  
| Global system prompt enabled flag |  
| Global system prompt content |  
| System prompt position |

\---

\#\# 22\. RATE LIMIT

| Trace |  
|-------|  
| Per-account rate limit state |  
| Enhanced rate limiting profiles |  
| Provider-specific rate limit profiles |

\---

\#\# 23\. LOGGING CONFIGURATION

\*\*Source:\*\* Environment variables

| Trace |  
|-------|  
| \`APP\_LOG\_TO\_FILE\` — enable disk logging |  
| \`ENABLE\_REQUEST\_LOGS\` — enable request logging |  
| \`DATA\_DIR\` — data directory path |  
| \`APP\_LOG\_RETENTION\_DAYS\` (default 7\) |  
| \`CALL\_LOG\_RETENTION\_DAYS\` (default 7\) |  
| \`CALL\_LOGS\_TABLE\_MAX\_ROWS\` (default 100,000) |  
| \`PROXY\_LOGS\_TABLE\_MAX\_ROWS\` (default 100,000) |  
| \`APP\_LOG\_MAX\_FILES\` |  
| \`CALL\_LOG\_MAX\_FILES\` |  
| Log rotation file count |  
| Log retention days |

\---

\#\# 24\. COMBO BUILDER & CONFIGURATION

| Trace |  
|-------|  
| Combo builder options |  
| Provider metadata |  
| Model metadata |  
| Connection metadata |  
| Duplicate detection |  
| Automatic next-connection suggestion |  
| Combo step schema (v2) |  
| Step ID |  
| Step kind |  
| Step providerId |  
| Step connectionId |  
| Step weight |  
| Step label |  
| Tiered model routing |  
| Tier to combo step mapping |  
| Fallback chains per tier |  
| Circular fallback detection |  
| Tool calling support |  
| Reasoning support |  
| Vision support |  
| Context window size |  
| Thinking budget support |  
| Modalities supported |  
| Model lifecycle metadata |

\---

\#\# 25\. DASHBOARD LOGS PAGES

| Trace |  
|-------|  
| Request Logs tab |  
| Proxy Logs tab |  
| Audit Logs tab |  
| Console Log Viewer |  
| Color-coded log levels |  
| Log search |  
| Level filter |  
| Auto-scroll |

\---

\#\# 26\. APPLICATION LOGS

\*\*Source:\*\* \`${DATA\_DIR}/logs/\`

| Trace |  
|-------|  
| Application log files |  
| Application log retention |  
| Application log max files |

\---

\#\# 27\. STORAGE ENCRYPTION

| Trace |  
|-------|  
| \`STORAGE\_ENCRYPTION\_KEY\` — encryption key |  
| AES-256-GCM encryption at rest |

\---

\#\# 28\. REQUEST-HISTORY PURGE

| Trace |  
|-------|  
| Manual request-history purge |  
| Delete call\_logs on purge |  
| Delete request\_detail\_logs on purge |  
| Delete local request artifacts on purge |

\---

\#\# 29\. COMPLIANCE REST APIS

| Trace |  
|-------|  
| Paginated admin audit entries |  
| Paginated MCP tool audit entries |  
| Aggregated MCP audit stats |  
| Filter by action (LIKE %value%) |  
| Filter by actor (LIKE %value%) |  
| Filter by target (LIKE %value%) |  
| Filter by resourceType (LIKE %value%) |  
| Filter by status (LIKE %value%) |  
| Filter by requestId (LIKE %value%) |  
| Filter by from/since timestamp |  
| Filter by to/until timestamp |  
| Limit (default 50, max 500\) |  
| \`?level=high|all\` filter |

\---

\#\# 30\. MCP TRANSPORTS

\*\*Source:\*\* \`open-sse/mcp-server/server.ts\`

| Trace |  
|-------|  
| stdio transport — IDE integrations |  
| sse transport — browser/agent clients |  
| streamable-http transport — multi-session HTTP clients |  
| Active HTTP transport selection |  
| Remote access (manage-scope bypass) |  
| LOCAL\_ONLY tier enforcement |  
| Authorization: Bearer \<api-key\> with manage scope |

\---

\#\# 31\. OBSERVABILITY SNAPSHOT (MCP Tool)

\*\*Source:\*\* \`observability\_snapshot\` MCP tool

| Trace |  
|-------|  
| Complete system snapshot |  
| Circuit breakers |  
| Sessions |  
| Quota monitors |  
| Token health |  
| Latency |

\---

\#\# 32\. PROVIDER FREE TIER AGGREGATION

\*\*Source:\*\* \`/dashboard/free-tiers\`

| Trace |  
|-------|  
| Aggregated free tiers of 40+ provider pools |  
| 500+ models |  
| \~1.6B free tokens/month (steady) |  
| \~2.1B first month with signup credits |

\---

\#\# 33\. SESSION & QUOTA MONITOR DASHBOARD

| Trace |  
|-------|  
| Live Session Activity panel |  
| Quota Monitors panel |  
| Active session counts |  
| Sticky-bound sessions |  
| Per-API-key session breakdown |  
| Quota monitor alerting status |  
| Quota monitor exhausted status |  
| Quota monitor error status |

\---

\#\# 34\. RESILIENCE UI DASHBOARD

\*\*Source:\*\* \`src/app/dashboard/resilience/\`

| Trace |  
|-------|  
| Real-time circuit breaker status |  
| Provider profiles |  
| Rate limit state |  
| Circuit breaker manual reset |  
| Model cooldowns |

\---

\#\# 35\. NO-LOG OPT-OUT

| Trace |  
|-------|  
| \`no\_log\` flag per API key |  
| In-memory no-log set |

\---

\#\# 36\. SYSTEM INFO REPORT

\*\*Source:\*\* \`npm run system-info\`

| Trace |  
|-------|  
| Node version |  
| OmniRoute version |  
| Operating system |  
| CLI tools installed |  
| Docker status |  
| PM2 status |

\---

\#\# 37\. RETAINED METRICS (Aggregated)

| Trace |  
|-------|  
| \`request\_count\` — counter |  
| \`request\_latency\_ms\` — histogram |  
| \`tokens\_consumed\` — counter |  
| \`cost\_usd\` — counter |  
| \`provider\_errors\` — counter |  
| \`circuit\_state\_changes\` — counter |  
| \`cache\_hits\` — counter |  
| \`compression\_savings\` — histogram |  
| \`quota\_used\` — gauge |  
| \`memory\_used\_mb\` — gauge |

\---

\#\# 38\. A2A SERVER TRACES

\*\*Source:\*\* A2A Server (JSON-RPC 2.0)

| Trace |  
|-------|  
| \`message/send\` — send message |  
| \`message/stream\` — SSE streaming |  
| \`tasks/get\` — get task |  
| \`tasks/cancel\` — cancel task |  
| Agent card at \`/.well-known/agent.json\` |  
| Task lifecycle states |  
| TTL cleanup |

\---

\#\# 39\. PERMISSION SCOPES

\*\*Source:\*\* Scoped permissions

| Trace |  
|-------|  
| 9 Scoped Permissions with wildcard support |  
| Scope enforcement before handler dispatch |

\---

\#\# 40\. RETENTION & CLEANUP METRICS

| Trace |  
|-------|  
| Per-table delete counts on cleanup |  
| Row-cap trim batch size (5,000) |  
| \`compliance.cleanup\` audit event |

\---

\# SUMMARY TABLE

| Category | Trace Count |  
|----------|-------------|  
| System Health & Infrastructure | 18 |  
| Provider Health & Resilience | 45 |  
| Request Logs & Telemetry | 35 |  
| Four-Stage Request Artifacts | 6 |  
| Administrative Audit Log | 22 |  
| MCP Tool Audit | 112 |  
| Proxy Logs | 5 |  
| Webhook Events | 18 |  
| Quota & Budget | 18 |  
| Combo Health & Routing | 18 |  
| Session Tracking | 10 |  
| Compression & Token Efficiency | 18 |  
| Alerting | 12 |  
| Performance Metrics | 10 |  
| Cloud Sync | 6 |  
| Cost & Usage | 12 |  
| Memory & Retrieval | 6 |  
| Proxy Configuration | 8 |  
| IP Filter | 3 |  
| Thinking Budget | 3 |  
| System Prompt | 3 |  
| Rate Limit | 3 |  
| Logging Configuration | 10 |  
| Combo Builder & Configuration | 18 |  
| Dashboard Logs Pages | 8 |  
| Application Logs | 3 |  
| Storage Encryption | 2 |  
| Request-History Purge | 3 |  
| Compliance REST APIs | 13 |  
| MCP Transports | 6 |  
| Observability Snapshot | 5 |  
| Provider Free Tier Aggregation | 4 |  
| Session & Quota Monitor Dashboard | 8 |  
| Resilience UI Dashboard | 5 |  
| No-Log Opt-Out | 2 |  
| System Info Report | 6 |  
| Retained Metrics | 10 |  
| A2A Server Traces | 7 |  
| Permission Scopes | 2 |  
| Retention & Cleanup Metrics | 3 |  
| \*\*TOTAL\*\* | \*\*\~506\*\* |

\---

The number of observable traces, enumerated from the codebase, documentation, and SQLite schemas, is approximately 506\.

\---

