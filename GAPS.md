# SAMWISE — Gap Analysis

## The Question

What exists, what needs to be built, what needs to be deployed, and in what order — before any more code generation makes sense.

---

## Layer 1: Network Substrate

### Tailscale

| | |
|---|---|
| **Status** | Off-the-shelf |
| **What exists** | Tailscale is a production product. Install on each node, authenticate, done. |
| **What's needed** | Install on all 4 nodes (GTX Desktop, Dell, Chromebook, Android). Configure ACLs for inter-service communication. Set up SSH access policy. Verify mesh connectivity from every node to every other node. |
| **Dependencies** | None. This is layer zero. |
| **Effort** | Low. Configuration, not development. |

**Gap:** Tailscale deployment + ACL policy across the fleet.

---

## Layer 2: Off-the-Shelf Services (Deploy + Configure)

These are existing tools. The gap is deployment, integration, and configuration — not invention.

### n8n

| | |
|---|---|
| **Status** | Open-source, self-hostable |
| **What exists** | Full workflow engine. Docker image, npm package, or binary. |
| **What's needed** | Deploy on primary node. Configure persistence. Define the initial deterministic workflows (failure recovery, service materialization, node health monitoring, deployment pipelines). Connect to YantrikDB for state. Expose webhook endpoints for OpenClaw event dispatch. |
| **Dependencies** | Tailscale (for node reachability). YantrikDB (for state). OpenClaw (for event dispatch — but n8n can start with static triggers). |
| **Effort** | Medium. Deployment is easy. Workflow authoring is the real work. |

**Gap:** Deployment + initial workflow library (failure recovery, service materialization, health checks, deployment).

### Midscene

| | |
|---|---|
| **Status** | Open-source |
| **What exists** | Pure-vision semantic normalization and UI grounding. |
| **What's needed** | Deploy as a service. Integrate with Tandem Browser for visual context. Build the procedure extraction pipeline — when Midscene discovers a UI interaction path, that path should be serialized into YantrikDB as a reusable procedure. Connect to MetaClaw for skill evolution. |
| **Dependencies** | Tandem Browser (for browser context). YantrikDB (for procedure storage). MetaClaw (for skill evolution). |
| **Effort** | Medium. Deployment is straightforward. The integration layer (procedure serialization, skill evolution handoff) is custom. |

**Gap:** Deployment + integration with Tandem, YantrikDB, and MetaClaw.

### Tandem Browser

| | |
|---|---|
| **Status** | **Needs clarification** |
| **What exists** | Unclear. Is this an existing product, a self-hosted browser with persistent profiles, or a custom build? If it's Chromium/Playwright/Puppeteer with persistent user-data-dir and identity-pool session management, it's a configuration problem. If it's something more, it needs scoping. |
| **What's needed** | Persistent authenticated browser sessions across 6× identity pools. Session state that survives process restart. Profile isolation per identity. Cookie/session/token persistence. Integration with Midscene for visual grounding. Integration with OpenClaw for task dispatch. |
| **Dependencies** | Identity pool credentials. Midscene (for visual grounding). OpenClaw (for task dispatch). |
| **Effort** | **Unknown until clarified.** |

**Gap:** Clarify what Tandem is. Then deploy or build accordingly.

### Autonomous Dev Team Tools

| Tool | Status | Gap |
|---|---|---|
| **OpenCode** | Open-source (github.com/anomalyco/opencode) | Deploy + configure model routing through OmniRoute |
| **Oh My OpenAgent** | Open-source (github.com/code-yeongyu/oh-my-openagent) | Deploy + configure model routing through OmniRoute |
| **Claude Code** | Anthropic product | Install + configure API key routing through OmniRoute |
| **Codex** | OpenAI product | Install + configure API key routing through OmniRoute |

**Gap:** Deploy all four. Configure each to route inference through OmniRoute rather than directly to providers. This is the "replaceable labor" layer — the tools exist, the integration is routing configuration.

---

## Layer 3: Custom Builds (Must Be Designed + Implemented)

These four services do not exist as off-the-shelf products. They are the architectural core of SAMWISE and must be built.

### YantrikDB — Singular Memory & Truth

| | |
|---|---|
| **Status** | Custom build required |
| **What it must do** | Store validated facts, entity relationships, decisions, constraints, procedures, failures, tool mappings, environment state, provider performance, routing outcomes, workflows, verification criteria, governance rules. Provide contradiction resolution. Track entropy (staleness, confidence decay). Support procedural skill storage and retrieval. Be the singular source of continuity across node failures. |
| **Design questions** | Graph DB vs document DB vs hybrid? What's the schema for organizational knowledge? How does contradiction resolution work? How is entropy tracked (TTL, confidence scoring, last-verified timestamps)? How does it handle concurrent writes from multiple agents/departments? What's the backup/replication strategy for continuity across node failure? |
| **Dependencies** | None — this is the foundational layer everything else reads from and writes to. |
| **Effort** | **High.** This is the hardest service to get right because every other service depends on its schema and query model. |

**Gap:** Full design + implementation. This is the single most important build in the architecture.

### OmniRoute — Stateful Inference Router

| | |
|---|---|
| **Status** | Custom build required |
| **What it must do** | Aggregate identity pools (6× Gemini, 6× Zen, 6× Kilo, 6× GitHub, etc.). Track per-identity quota usage and predict exhaustion. Route inference requests across providers based on cost, latency, capability, and quota availability. Compress context (token optimization). Cache responses. Score provider performance. Circuit-break failing providers. Failover across providers transparently. Provide a unified API that abstracts all providers behind a single interface. |
| **Design questions** | What's the identity pool abstraction? How is quota tracked (per-identity, per-provider, per-model)? What's the compression strategy (summarization, truncation, selective inclusion)? What's the cache key schema? How does circuit-breaking work (error rate thresholds, backoff)? What's the scoring model for provider selection? How does it handle streaming responses? |
| **Dependencies** | Identity pool credentials. Provider API access. YantrikDB (for routing history and performance data). |
| **Effort** | **High.** This is the second hardest build. It must be reliable, stateful, and handle the full diversity of provider APIs. |

**Gap:** Full design + implementation. Every inference call in the system flows through this.

### OpenClaw — Governance Brain

| | |
|---|---|
| **Status** | Custom build required |
| **What it must do** | Receive human intent. Decompose objectives into executable plans. Enforce standing orders and governance rules. Delegate to agents and departments. Maintain lifecycle ownership from intent to verified completion. Coordinate cross-departmental workflows. Dispatch events to n8n for deterministic execution. Query YantrikDB for accumulated capability. Route inference through OmniRoute. |
| **Design questions** | What's the intent decomposition model? How are standing orders defined and enforced? What's the delegation protocol? How does it track lifecycle state (planned → executing → verifying → complete)? How does it decide when to delegate vs. when to escalate to human? What's the governance rule engine? How does it compose capabilities from YantrikDB into execution plans? |
| **Dependencies** | YantrikDB (for knowledge and governance state). OmniRoute (for inference). n8n (for deterministic workflow dispatch). MetaClaw (for procedural skills). All execution agents. |
| **Effort** | **High.** This is the orchestration brain. It must be reliable, governable, and capable of complex decomposition. |

**Gap:** Full design + implementation. This is the service that makes SAMWISE "supervised" rather than "autonomous chaos."

### MetaClaw — Procedural Skill Evolution

| | |
|---|---|
| **Status** | Custom build required |
| **What it must do** | Observe execution traces. Identify repeated successful patterns. Extract reusable procedures from those patterns. Store procedures in YantrikDB. Inject known procedures into future executions (so agents don't re-discover what's already known). Evolve procedures as environments change. Detect when a procedure is stale or failing. |
| **Design questions** | What's an "execution trace" and how is it captured? How does pattern detection work (frequency, success rate, similarity)? What's the procedure schema? How are procedures versioned? How does injection work (pre-prompt, tool call, workflow step)? How does evolution work (mutation, testing, rollback)? |
| **Dependencies** | YantrikDB (for procedure storage). Execution trace data from agents. OmniRoute (for inference during extraction). |
| **Effort** | **Medium-High.** The extraction and evolution logic is non-trivial, but it builds on top of YantrikDB and trace data. |

**Gap:** Full design + implementation. This is the service that converts execution into compounding.

---

## Layer 4: Integration & Orchestration

Even after all services exist, they need to talk to each other.

### Service Communication

| | |
|---|---|
| **Status** | Not designed |
| **What's needed** | How do services discover each other? How do they communicate (HTTP, gRPC, message queue, event bus)? What's the authentication model between services? How is service health monitored? What's the service registry? |
| **Dependencies** | All services deployed. Tailscale mesh operational. |
| **Effort** | Medium. This is architecture, not invention. |

**Gap:** Communication protocol design, service discovery, inter-service auth, health monitoring.

### Identity Pool Configuration

| | |
|---|---|
| **Status** | Not configured |
| **What's needed** | All identity pools (6× Gemini, 6× Zen, 6× Kilo, 6× GitHub, etc.) need credentials provisioned, quota baselines established, and rotation/refresh policies defined. OmniRoute needs to be configured with the full pool topology. |
| **Dependencies** | OmniRoute built. |
| **Effort** | Low-Medium. Credential management and quota tracking setup. |

**Gap:** Identity pool provisioning + OmniRoute pool configuration.

### Tier 3 Operator Surfaces

| | |
|---|---|
| **Status** | Not configured |
| **What's needed** | 6× identity sessions established in Tandem Browser for each operator surface (NotebookLM, AI Studio, ChatGPT, Gemini, DeepSeek, Qwen, Z.ai, Grok, Kimi, Arena.ai). Authentication state persisted. Midscene procedures discovered for each surface. n8n delegate workflows for infrastructure operations. |
| **Dependencies** | Tandem Browser operational. Midscene operational. n8n operational. OpenClaw operational. |
| **Effort** | Medium. Authentication setup + procedure discovery per surface. |

**Gap:** Identity session setup + procedure discovery for all 10+ operator surfaces.

---

## Layer 5: Operating System & Runtime

### Dell Control Cockpit — OS Decision

| | |
|---|---|
| **Status** | **Unresolved decision** |
| **The question** | What OS should the Dell run? The architecture document frames this as: "The decision should determine the best role and operating environment for the Dell Control Cockpit within this distributed topology while minimizing: operating-system overhead, build friction, coordination tax, remote-administration complexity, state fragmentation, recovery complexity." |
| **Options** | Current OS (Windows?), Linux distro (which one?), dual-boot, WSL2, containerized workspace? |
| **Dependencies** | This decision affects deployment strategy for every service. |
| **Effort** | The decision itself is zero effort. The implementation depends on the choice. |

**Gap:** OS decision for the Dell. This blocks deployment planning.

### Container / Runtime Strategy

| | |
|---|---|
| **Status** | Not designed |
| **What's needed** | How are services packaged (Docker, podman, bare metal, systemd services)? How are they orchestrated (docker-compose, k3s, nomad, manual)? How does service materialization work when a node fails (n8n needs to know how to stand up a service on a new node)? |
| **Dependencies** | OS decision. Service builds complete. |
| **Effort** | Medium. Design + configuration. |

**Gap:** Container strategy + orchestration + service materialization playbooks.

---

## The Dependency Graph

```
Tailscale (Layer 0)
    │
    ▼
YantrikDB (Layer 3 — foundational)
    │
    ├──► OmniRoute (Layer 3 — needs YantrikDB for routing state)
    │       │
    │       ▼
    │    Autonomous Dev Team tools (Layer 2 — route through OmniRoute)
    │
    ├──► MetaClaw (Layer 3 — needs YantrikDB for procedure storage)
    │
    ├──► n8n (Layer 2 — needs YantrikDB for workflow state)
    │
    ├──► OpenClaw (Layer 3 — needs YantrikDB, OmniRoute, n8n, MetaClaw)
    │       │
    │       ▼
    │    Tier 3 Operator Surfaces (Layer 4 — needs OpenClaw for dispatch)
    │
    ├──► Tandem Browser (Layer 2 — needs clarification on what it is)
    │       │
    │       ▼
    │    Midscene (Layer 2 — needs Tandem for browser context)
    │
    ▼
Service Communication (Layer 4)
    │
    ▼
Identity Pool Configuration (Layer 4)
    │
    ▼
Container / Runtime Strategy (Layer 5)
    │
    ▼
Dell OS Decision (Layer 5 — blocks deployment planning)
```

---

## The Critical Path

What must happen, in what order, to get from architecture document to running system:

### Phase 0: Decisions (Blocks Everything)

1. **Resolve the Tandem Browser question** — Is it off-the-shelf or custom? This determines whether Layer 2 or Layer 3 has 5 items.
2. **Resolve the Dell OS decision** — This determines the deployment strategy for every service.

### Phase 1: Foundation

3. **Tailscale deployment** — Get the mesh running across all 4 nodes.
4. **YantrikDB design + implementation** — Schema, storage, query model, contradiction resolution, entropy tracking. This is the hardest and most important build.

### Phase 2: Routing & Execution

5. **OmniRoute design + implementation** — Identity pool aggregation, quota tracking, routing, compression, caching, failover.
6. **Deploy Autonomous Dev Team tools** — OpenCode, Oh My OpenAgent, Claude Code, Codex. Route through OmniRoute.
7. **n8n deployment + initial workflows** — Deploy the engine. Author failure recovery, service materialization, and health monitoring workflows.

### Phase 3: Intelligence

8. **MetaClaw design + implementation** — Execution trace capture, pattern extraction, procedure evolution.
9. **OpenClaw design + implementation** — Intent decomposition, governance, delegation, lifecycle management.

### Phase 4: Browser & UI

10. **Tandem Browser deployment** — Persistent authenticated sessions across identity pools.
11. **Midscene deployment + integration** — Visual grounding, procedure discovery, integration with Tandem and YantrikDB.

### Phase 5: Integration & Operators

12. **Service communication layer** — Discovery, auth, health monitoring.
13. **Identity pool configuration** — Provision all pools in OmniRoute.
14. **Tier 3 operator surface setup** — Authenticate all surfaces, discover procedures, configure n8n delegate workflows.

### Phase 6: Hardening

15. **Container / runtime strategy** — Packaging, orchestration, service materialization playbooks.
16. **Failure mode testing** — Kill nodes, kill services, verify continuity model works.
17. **Compounding validation** — Run tasks, verify that knowledge is captured, procedures are extracted, and subsequent executions are cheaper.

---

## Summary: What's Left

| Category | Count | Effort |
|---|---|---|
| Unresolved decisions | 2 | Low effort, high impact |
| Off-the-shelf deployments | 6 | Low-Medium effort each |
| Custom builds | 4 | High effort each |
| Integration layers | 3 | Medium effort each |
| Hardening & validation | 3 | Medium effort each |
| **Total distinct work items** | **18** | |

### The Four Custom Builds (In Priority Order)

| Priority | Service | Why |
|---|---|---|
| 1 | **YantrikDB** | Everything reads from and writes to it. Wrong schema here cascades everywhere. |
| 2 | **OmniRoute** | Every inference call flows through it. Without it, no agent can execute. |
| 3 | **OpenClaw** | Without governance and decomposition, the system is uncoordinated agents. |
| 4 | **MetaClaw** | Without procedural extraction, the system executes but does not compound. |

---
