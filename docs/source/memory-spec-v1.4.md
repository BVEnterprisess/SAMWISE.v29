\# SAMWISE's MEMORY SPECIFICATION v1.4 

\#\# The Memory Gravity Well Architecture: Omniroute Native & Inline Skill Control Loop

\---

\#\#\# 1\. Executive Summary (The Vibe Shift)

Most Agent OSes are built like paranoid librarians—obsessed with being "correct" and terrified of being "wrong." That's a fool's errand. In an infinite, chaotic world, absolute correctness is a statistical ghost, and wrongness is just Tuesday.

We don't play that game.

We built this system like a \*\*solar system\*\*, not a logic puzzle. At the center sits \*\*YantrikDB\*\*—not just a database, but a gravitational singularity holding the \*\*authoritative, persistent continuity\*\* of everything that matters. Agents are comets. They drift, they explore, they fly off into wild entropy. But they are \*bound\*. The further they stray, the harder the gravity pulls them back toward our directives.

We don't waste energy avoiding drift or entropy. We \*\*govern\*\* them. We let the agents roam, let them fail spectacularly, and we use those failures to tighten the orbit so they never burn up the same way twice.

\---

\#\#\# 2\. The Immutable Invariants (The Non-Negotiables)

These are the laws. Break one, and the whole thing collapses.

1\. \*\*YantrikDB \= The Singular Source of Truth.\*\*  
There is only \*one\* daemon holding the authoritative, persistent continuity of this system. Full stop. (Your Tandem-Browser holds ephemeral UI state—that's a temporary sandbox, not the bedrock.)

2\. \*\*If it wasn't Approved AND it wasn't Validated, it never happened.\*\*  
Both gates must swing open. Anything less gets bounced at the event horizon. No exceptions.

3\. \*\*ALL Event Streams are Governed.\*\*  
Every read, write, tool‑call, or agentic sneeze passes through Omniroute. Zero backdoors.

4\. \*\*Memory owns Agents | Agents own Nothing.\*\*  
Agents are rented muscle. They borrow context, they process, they spit out results—but they own zero permanent state. Memory is the landlord; agents are the tenants.

5\. \*\*The Inline Skill is Stateless.\*\*  
The Skill derives all dynamic parameters (drift baseline, friction integral) from YantrikDB on each request — no local caching, no split‑brain, no persistence. Omniroute and the Skill are pure functions over the request and YantrikDB's state.

\---

\#\#\# 3\. How the Damn Thing Works

Imagine every single action—user clicks, tool outputs, agent logs—getting funneled into a single, hardened network endpoint where YantrikDB lives.

\* First, \*\*Omniroute\*\* checks the event: \*“Are you allowed to be here? Do you follow policy?”\* (Hardcoded rules \+ dynamic scopes)  
\* If it passes, \*\*YantrikDB\*\* steps in and verifies the payload: \*“Are you structurally sound? Are you cryptographically intact?”\*

If an event passes both, it gets \*\*etched into the persistent timeline\*\*—the memory well grows deeper. If it fails either gate, it doesn't touch the source of truth. Instead, we log the hell out of it, downgrade it to "noise," and ship it downstream for observability. But—and this is key—we \*extract the lesson\* from that failure and feed it back into the system's damping so we don't make the exact same validation mistake twice.

\---

\#\#\# 4\. The ASCII Diagram (Unchanged, Because It's Fucking Nasty)

\`\`\`  
                    \+---------------------------------------------------+  
                    |         TANDEM-BROWSER (Ephemeral UI State)       |  
                    |  (Session tabs, scroll position, local cache)     |  
                    |        THIS IS NOT THE SOURCE OF TRUTH.           |  
                    \+-------------------+-------------------------------+  
                                        |  
                            EXPLORATION SPACE (High Entropy)  
                                        |  
                    A1 (Drifting) \---\> |  \<--- A2 (Exploring)  
                         \\              |            /  
                          \\             |           /  
                           \\  (Drift)   | (Drift)  /  
                            \\           |         /  
                             \\          |        /  
                              \\         |       /  
                               \\        |      /  
                                \\       |     /  
                                 \\      |    /  
                                  \\     |   /  
                                   \\    |  /  
                      \============== \\ | / \==============  
                      ||  GOVERNANCE  \\|/  VALIDATION  ||   \<-- Event Horizon Filter  
                      ||  (APPROVAL)  /|\\  (VERIFY)    ||   (Only continuity passes)  
                      \============== / | \\ \==============  
                                    /  |  \\  
                                   /   |   \\  
                                  /    |    \\  
                                 /     |     \\  
                                /      |      \\  
                               /       |       \\  
                              /  (Gravity Pull) \\  
                             /         |         \\  
                            /          |          \\  
                           /  (Binding Force)      \\  
                          /            |             \\  
                         /    GRADIENT DESCENT        \\  
                        /              |               \\  
    \+-------------------------------------------------------+  
    |             Y A N T R I K  D B  (Singularity)          |  
    |  Memory \= The Integral of all Governed Event Streams  |  
    |  Holds AUTHORITATIVE, PERSISTENT CONTINUITY.          |  
    |  (The Bedrock. The Timeline. The Irreducible Truth.) |  
    \+-------------------------------------------------------+  
                            (CORE CONTINUITY)  
                                  /|\\  
                                   |  
                             \[NOISE\] | \[SIGNAL\]  
                                   |  
                          (Downstream Logging &   
                           Adaptive Damping Update)  
\`\`\`

\---

\#\#\# 5\. The Inline Skill & Omniroute 1:1 Mapping (The "How")

\*\*Omniroute is the Event Horizon.\*\* Every single event funnels through it. Inside Omniroute, a \*\*Stateless Inline Skill\*\* runs on every request, implemented via the \*\*Skills Framework\*\* (sandboxed, versioned, schema‑defined middleware). The Skill hooks into two phases: \`preRequest\` (before forwarding to cloud LLM) and \`postResponse\` (after receiving the upstream reply).

The Skill never persists anything locally; it reads from and writes to YantrikDB on every turn, ensuring the singularity remains the sole source of truth.

\---

\#\#\#\# 5.1 Exact Omniroute Components Used

| Omniroute Component | Purpose in the Control Loop | How the Skill Accesses It |  
|---------------------|-----------------------------|---------------------------|  
| \`api\_keys\` table (SQLite) | Scope enforcement (permissions) | Native: Skill receives API key context; can check scopes via built‑in \`hasScope()\` |  
| \`audit\_log\` table (SQLite) | Append‑only signal log (pass) | Skill writes via Omniroute's internal \`auditLog()\` API |  
| \`mcp\_tool\_audit\` table (SQLite) | Append‑only failure log (noise) | Skill writes via Omniroute's internal \`mcpToolAudit()\` API |  
| \`omniroute\_set\_api\_key\_scopes\` MCP tool | Dynamic scope restriction (friction) | Skill invokes MCP tool via internal Omniroute client |  
| \`omniroute\_set\_combo\_parameter\` MCP tool | Dynamic temperature/parameter override | Skill invokes MCP tool to adjust \`temperature\` for the current request |  
| \`POST /v1/chat/completions\` | Main relay to cloud LLM | Omniroute's native relay; Skill can modify request body before forwarding |  
| \`X‑Drift‑Score\` custom header | Agent‑provided drift metric | Skill reads header from incoming request |  
| Prompt Injection Guard | Input sanitization (pre‑exec) | Skill triggers guard via Omniroute's built‑in \`runPromptInjectionGuard()\` |  
| Structured Output Bridging | Response schema validation (post‑exec) | Skill uses Omniroute's translator to validate against expected schema |  
| Skills Framework | Middleware hook registration | Skill is registered as a \`.js\` handler with \`preRequest\` and \`postResponse\` exports |  
| Webhook system | Alternative failure/event egress (optional) | Skill can trigger webhooks as fallback |

\---

\#\#\#\# 5.2 The Skill's Execution Flow (Exact Step‑by‑Step)

\`\`\`  
\[Agent Request arrives at Omniroute\]  
       │  
       ▼  
╔══════════════════════════════════════════════════════════════════╗  
║ STEP 1: Pre‑Execution Gate (Approval)                          ║  
║                                                                ║  
║  • Skill reads \`api\_keys\` table for the request's API key.     ║  
║  • It checks the required scopes against the key's scopes      ║  
║    (9 granular scopes \+ wildcard).                             ║  
║  • If insufficient, rejects with HTTP 403\.                    ║  
║                                                                ║  
║  • Skill evaluates Policy Rules:                               ║  
║    \- Payload Rules (e.g., reject requests with certain         ║  
║      keywords or tags)                                        ║  
║    \- Tag‑Based Routing (route to specific combo if tag         ║  
║      matches)                                                 ║  
║  • If policy fails, rejects with HTTP 422\.                    ║  
║                                                                ║  
║  • Skill runs Prompt Injection Guard:                         ║  
║    \- Calls \`runPromptInjectionGuard(request.body)\`            ║  
║    \- If flagged, rejects with HTTP 400 and error              ║  
║      \`SECURITY\_001\`.                                          ║  
╚══════════════════════════════════════════════════════════════════╝  
       │  
       ▼  
╔══════════════════════════════════════════════════════════════════╗  
║ STEP 2: Compute Physics (Gravity & Friction)                   ║  
║                                                                ║  
║  • Skill reads \`X‑Drift‑Score\` header (float, computed by      ║  
║    agent/client as embedding distance or token divergence).   ║  
║                                                                ║  
║  • Gravity Pull:                                               ║  
║    \- Skill queries YantrikDB for the agent's baseline          ║  
║      system prompt and core context anchors (using            ║  
║      \`session\_id\` or \`api\_key\_id\`).                          ║  
║    \- If \`X‑Drift‑Score \> threshold\`, the Skill injects        ║  
║      the baseline system prompt into \`request.body.messages\`  ║  
║      (prepending or replacing existing system message).       ║  
║                                                                ║  
║  • Adaptive Friction:                                          ║  
║    \- Skill queries YantrikDB for recent validation failures    ║  
║      (last 5 minutes) for this API key / session.             ║  
║    \- It computes the integral:                                 ║  
║      Friction \= γ \* Σ( failures \* e^(-α \* age) )              ║  
║    \- If Friction \> threshold:                                 ║  
║        a. Clamp \`temperature\` in request body to lower value  ║  
║           (e.g., 0.3 instead of 0.8) via Omniroute's native   ║  
║           format translator override.                         ║  
║        b. Call \`omniroute\_set\_api\_key\_scopes\` MCP tool        ║  
║           to temporarily restrict this API key's scopes       ║  
║           (e.g., remove \`write:combos\`).                      ║  
║        c. Enforce strict JSON Schema decoding by toggling     ║  
║           \`strict\_structured\_output\` flag in the request.     ║  
╚══════════════════════════════════════════════════════════════════╝  
       │  
       ▼  
╔══════════════════════════════════════════════════════════════════╗  
║ STEP 3: Forward to Cloud LLM                                   ║  
║                                                                ║  
║  • The modified request passes through Omniroute's native      ║  
║    relay to the selected cloud provider.                      ║  
║  • Local LLM inference is strictly disabled                   ║  
║    (enforced by Omniroute configuration).                     ║  
╚══════════════════════════════════════════════════════════════════╝  
       │  
       ▼  
\[Cloud LLM returns response\]  
       │  
       ▼  
╔══════════════════════════════════════════════════════════════════╗  
║ STEP 4: Post‑Execution Gate (Validation)                       ║  
║                                                                ║  
║  • Schema Validation:                                          ║  
║    \- Skill uses Omniroute's Structured Output Bridging to     ║  
║      validate the response payload against the expected       ║  
║      JSON schema (if provided in request).                    ║  
║    \- If schema mismatch, validation fails.                    ║  
║                                                                ║  
║  • Signature Verification:                                     ║  
║    \- Skill checks for HMAC signature in response headers.     ║  
║    \- It retrieves the shared secret from YantrikDB (using     ║  
║      \`session\_id\` or \`api\_key\_id\`) and verifies the HMAC.    ║  
║    \- If signature invalid, validation fails.                  ║  
╚══════════════════════════════════════════════════════════════════╝  
       │  
       ▼  
╔══════════════════════════════════════════════════════════════════╗  
║ STEP 5: Commit or Dampen                                       ║  
║                                                                ║  
║  • If PASS (approved \+ validated):                            ║  
║    \- Skill appends the event to YantrikDB's immutable         ║  
║      timeline via HTTP POST to YantrikDB's \`/append\`         ║  
║      endpoint, including full payload, metadata, and         ║  
║      trace ID.                                               ║  
║    \- Also writes to Omniroute's \`audit\_log\` table for        ║  
║      local redundancy.                                       ║  
║                                                                ║  
║  • If FAIL (failed either gate):                              ║  
║    \- Skill writes the failure event to YantrikDB's           ║  
║      failure log (\`/append\_failure\`) with error details,     ║  
║      timestamp, and API key ID.                              ║  
║    \- Also logs to Omniroute's \`mcp\_tool\_audit\` (if           ║  
║      applicable) and triggers a webhook if configured.        ║  
║    \- The Skill does not cache the failure locally; it        ║  
║      relies on YantrikDB for all state.                      ║  
╚══════════════════════════════════════════════════════════════════╝  
       │  
       ▼  
\[Response returned to agent/client\]  
\`\`\`

\---

\#\#\#\# 5.3 Exact Data Flow (Traces & Headers)

| Trace | Source | Destination | Format |  
|-------|--------|-------------|--------|  
| \`X‑Drift‑Score\` | Agent/Client | Omniroute Skill | Float header |  
| \`X‑Session‑Id\` | Agent/Client | Omniroute Skill (and YantrikDB queries) | String header |  
| \`X‑Request‑Id\` | Omniroute | All logs (correlation) | UUID |  
| \`X‑OmniRoute‑Effective‑Scopes\` | Omniroute | Response header (for agent awareness) | Comma‑separated scopes |  
| Friction integral value | YantrikDB (query) → Skill | In‑memory computation (per request) | Float |  
| Baseline context hash | YantrikDB (query) → Skill | Injected into request body | String |  
| Temperature override | Skill → Request body | Cloud LLM | Float (0.0‑1.0) |  
| Scope restriction | Skill → MCP call → Omniroute | Applies to subsequent requests | Scope string |

\---

\#\#\# 6\. The Mathematical Control Loop (Closed‑Loop Guarantee)

The agent's trajectory is governed by the force‑balance equation:

$$\\boxed{  \\text{Agent's Acceleration} \+ \\text{Adaptive Friction} \+ \\text{Gravity Pull} \= \\text{Drift / Entropy} }$$

\*\*Where each term is exactly computed:\*\*

\- \*\*Agent's Acceleration\*\* \= Rate of change of action choice; measured via tool‑call diversity over last N turns (derived from YantrikDB event stream).  
\- \*\*Adaptive Friction\*\* \= $$\\gamma \\int\_{0}^{t} \\text{Failures}(\\tau) \\, e^{-\\alpha (t \- \\tau)} \\, d\\tau$$, computed per request from YantrikDB's failure log.  
\- \*\*Gravity Pull\*\* \= $$\\lambda \\cdot d$$, where \\(d\\) is the \`X‑Drift‑Score\` (embedding distance from baseline). If \\(d \> d\_{\\text{threshold}}\\), the Skill injects the baseline system prompt, effectively applying a restoring force.  
\- \*\*Drift / Entropy\*\* \= The raw exploration; kept positive to allow novelty, but bounded because friction and gravity scale with deviation.

\*\*Closed‑Loop Proof:\*\*    
The system's total energy \\(E \= \\text{Kinetic (movement)} \+ \\text{Potential (distance from YantrikDB)}\\) strictly dissipates over time because:  
\- Every failure adds friction (increases \\(\\gamma\\) effectively), damping future acceleration.  
\- Every drift beyond threshold triggers gravity (adds potential energy penalty), pulling the agent back.

Thus the agent's maximum stray distance is strictly bounded. The system is Lyapunov‑stable: the agent will never spiral into unrecoverable hallucination nor overwrite the authoritative timeline.

\---

\#\#\# 7\. Why We Don't Fight Drift or Entropy (Our Secret Sauce)

Other systems treat entropy like a virus. They quarantine the agent, freeze its outputs, and force it into a deterministic cage. That works until reality throws a curveball, and then the brittle system shatters.

We do the opposite:

\* \*\*Drift is our exploration budget.\*\* It’s how the agent finds novel solutions, stumbles onto edge cases, and occasionally surprises us.  
\* \*\*Entropy is just noise.\*\* We don't fear it because we don't let it touch the source of truth. The Governance/Validation gate is the bouncer. The agent can party all night outside the club, but the moment it tries to spray‑paint the walls of YantrikDB, the bouncer stops it cold.

\*\*The Gravity Guarantee:\*\* No matter how drunk the agent gets on entropy, the pull of the singularity is stronger. The math guarantees it: \*deviation is bounded, stability is absolute.\*

We don't muzzle the dog. We just made the yard infinitely large, with an invisible fence that always pulls him back to the porch.

\---

\#\#\# 8\. Failure as a First‑Class Citizen (Signal vs. Noise)

We don't hide failures; we \*celebrate\* them—because they make the system smarter.

\* \*\*Signal (Pass):\*\* The event appends to the timeline. Truth deepens. The well grows.  
\* \*\*Noise (Fail):\*\* The event gets logged, downgraded, and shipped downstream for humans to observe. But critically, we strip the metadata from that failure and feed it into the \*\*Adaptive Friction\*\* coefficient.

Think of it like muscle memory: \*“We tripped on that specific rock before. Next time we approach that rock, our reflexes are tighter so we stumble less.”\*

We save the signal for eternity and send the noise down the river—but we \*learn\* from every single drop of it.

\---

\#\#\# 9\. The Guarantee (The TL;DR for Stakeholders)

To the exec who asks \*"Does this actually hold together?"\*:

Yes. Because the system's total energy—kinetic (movement) plus potential (distance from YantrikDB)—is \*strictly dissipating\* over time. Every failure adds friction. Every gravity pull corrects the trajectory. The agent will \*\*never\*\* spiral out into unrecoverable hallucination, \*\*never\*\* overwrite the authoritative timeline, and \*\*never\*\* repeat a fatal mistake twice.

It's not "correct" in the academic sense. It's \*\*stable\*\* in the engineering sense. And stability wins every single time.

\---

\#\#\# 10\. Hand‑off Engineering Instructions (Exact)

1\. \*\*Deploy Omniroute\*\* as the mandatory proxy endpoint. Block all direct egress to cloud LLM providers and enforce \`Local LLM Inference \= 0\` via Omniroute configuration.

2\. \*\*Register the Custom Inline Control Skill\*\* into Omniroute's Skills Framework:  
   \- Create a \`.js\` file with \`exports.preRequest \= async (req, ctx) \=\> { ... }\` and \`exports.postResponse \= async (req, res, ctx) \=\> { ... }\`.  
   \- Upload via \`POST /api/skills\` with \`type: 'middleware'\` and \`phase: 'both'\`.  
   \- The Skill will receive the request/response objects and can access Omniroute's internal APIs (e.g., \`ctx.db\`, \`ctx.mcpClient\`).

3\. \*\*Configure Gravity Mechanics:\*\*  
   \- Instruct client wrappers to compute divergence (e.g., cosine distance between current embedding and baseline) and pass it in the \`X‑Drift‑Score\` header.  
   \- In the Skill's \`preRequest\`, read this header. If it exceeds the configured threshold, query YantrikDB (via HTTP) for the baseline system prompt (using \`session\_id\` from the request or a custom mapping). Prepend or replace the \`system\` message in the request body with that baseline.

4\. \*\*Configure Adaptive Damping:\*\*  
   \- In the Skill's \`preRequest\`, query YantrikDB for failure events for this API key in the last N minutes (e.g., via a REST endpoint \`/api/failures?api\_key=...\&since=...\`).  
   \- Compute the exponential decay integral using a sliding window (or a pre‑computed aggregate if YantrikDB supports it).  
   \- If the friction value exceeds the threshold:  
     \- Override \`temperature\` in the request body to a lower value (e.g., 0.3) via Omniroute's format translator (set \`req.body.temperature \= clampedValue\`).  
     \- Call \`omniroute\_set\_api\_key\_scopes\` MCP tool (available via \`ctx.mcpClient\`) to reduce scopes for this API key for subsequent requests (apply a TTL).  
     \- Enforce strict structured output by toggling \`req.body.structured\_output \= true\` and ensuring the schema is present.

5\. \*\*Establish YantrikDB Persistence:\*\*  
   \- In the Skill's \`postResponse\`, if validation passes (schema \+ signature), send the full event (request, response, metadata) to YantrikDB's append‑only endpoint (e.g., \`POST /v1/append\`).  
   \- If validation fails, send a failure log to YantrikDB's failure endpoint (e.g., \`POST /v1/failures\`). Include the error reason, timestamp, and API key ID.  
   \- Ensure the Skill never caches these writes; it sends them synchronously (or asynchronously with ack) to maintain the invariant that YantrikDB is the sole source of truth.

6\. \*\*Hang the ASCII diagram in the engineering room.\*\* Go build the system.

\---

Repo Stack:  
https://github.com/openclaw/openclaw  
https://docs.openclaw.ai/concepts/delegate-architecture  
https://github.com/yantrikos/yantrikdb  
https://github.com/diegosouzapw/OmniRoute  
https://github.com/hydro13/tandem-browser  
https://github.com/web-infra-dev/midscene  
https://github.com/n8n-io/n8n  
https://github.com/aiming-lab/MetaClaw  
https://github.com/tailscale/tailscale

---

 