# YantrikDB + MetaClaw [skills_only]

## Closure Authority — Evidence-Preserving Asset Loop

This document is subordinate to the [closure contract](docs/superpowers/specs/2026-08-02-yantrikdb-metaclaw-closure-contract.md). YantrikDB and MetaClaw are standalone engines. This document defines their exact integration boundary.

```text
OmniRoute raw observables and reusable assets
→ immutable Evidence-Preserving Asset Contract
→ YantrikDB validation, calibration, decay, linking, recall, and think
→ MetaClaw projected procedural state, policy, injection, and evolution
→ OmniRoute measured next execution
```

OmniRoute owns trace capture and trace-to-asset conversion. YantrikDB does not replace that engine or treat raw traces as mutable knowledge. YantrikDB receives typed assets plus provenance, stores append-only evidence/evaluations, and produces deterministic projections. MetaClaw consumes projected procedural state rather than bypassing provenance with raw memory.

Assets are immutable. Evaluations, evidence links, relations, and policy decisions are append-only. A correction, split, merge, or supersession creates a new linked object; it never rewrites history.

## Persistent Continuity — The Compounding Substrate

This document specifies the two services that make compounding mechanically real.

Every other service in the architecture produces work. These two services convert work into accumulated capability. Without them, the system executes but does not learn. With them, every execution permanently strengthens every future execution.

**No hand-waving. No abstraction without specification. This is the build document.**

---

## Part I: YantrikDB

### What YantrikDB Is

YantrikDB is the singular organizational evidence and memory substrate. It is not the raw execution ledger and it is not an oracle of truth. It stores immutable derived assets, evaluates evidence, resolves projections, applies calibrated importance and half-life decay, and serves policy-scoped knowledge.

Services read projected state from YantrikDB. Writes enter through typed, provenance-preserving adapters. Raw OmniRoute observables remain attributable to their source. YantrikDB is the durable memory state that survives node failure, agent death, session end, and departmental boundary.

**YantrikDB is continuity.**

### The Knowledge Model

YantrikDB stores twelve projected knowledge types. Each type has a precise schema. No derived knowledge enters the system without an immutable asset identity, source lineage, evaluation class, and policy reference. The asset contract sits before these projections and is the authoritative integration boundary.

```
┌─────────────────────────────────────────────────────────────┐
│                   YANTRIKDB KNOWLEDGE TYPES                  │
│                                                             │
│  ┌───────────┐  ┌───────────┐  ┌───────────┐              │
│  │   Fact    │  │  Entity   │  │Relation-  │              │
│  │           │  │           │  │   ship    │              │
│  └───────────┘  └───────────┘  └───────────┘              │
│                                                             │
│  ┌───────────┐  ┌───────────┐  ┌───────────┐              │
│  │ Procedure │  │  Failure  │  │ Decision  │              │
│  │           │  │           │  │           │              │
│  └───────────┘  └───────────┘  └───────────┘              │
│                                                             │
│  ┌───────────┐  ┌───────────┐  ┌───────────┐              │
│  │Constraint │  │ Governance│  │  Routing  │              │
│  │           │  │   Rule    │  │  Outcome  │              │
│  └───────────┘  └───────────┘  └───────────┘              │
│                                                             │
│  ┌───────────┐  ┌───────────┐  ┌───────────┐              │
│  │  Environ- │  │   Skill   │  │ Execution │              │
│  │ ment State│  │ (MetaClaw)│  │   Trace   │              │
│  └───────────┘  └───────────┘  └───────────┘              │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

### Type 1: Fact

A validated piece of declarative knowledge represented in a policy-scoped projection. It is not mutable truth; its evidence, provenance, uncertainty, and correction history remain inspectable.

```typescript
interface Fact {
  id: string;                    // unique identifier
  type: "fact";
  content: string;               // the knowledge itself
  content_hash: string;          // deduplication key
  source: FactSource;            // where this came from
  confidence: number;            // 0.0 – 1.0
  verified_at: ISO8601;          // last verification timestamp
  verified_by: string;           // agent, human, or process that verified
  verification_count: number;    // how many times verified
  created_at: ISO8601;
  created_by: string;            // agent or department that produced this
  department_origin: Department;
  contradicts: string[];         // IDs of facts this contradicts
  supports: string[];            // IDs of facts this supports
  scope: FactScope;              // applicability boundaries
  tags: string[];                // cross-reference labels
  entropy: EntropyState;         // decay tracking
  status: "active" | "contested" | "deprecated" | "superseded";
  superseded_by?: string;        // if superseded, by what
}

interface FactSource {
  type: "execution" | "human" | "research" | "inference" | "observation" | "import";
  trace_id?: string;             // execution trace that produced this
  session_id?: string;
  provider?: string;             // if from inference
  human_id?: string;             // if from human
}

interface FactScope {
  department?: Department;       // if department-specific
  environment?: string;          // if environment-specific
  service?: string;              // if service-specific
  global: boolean;               // if true, applies everywhere
  expires_at?: ISO8601;          // temporal scope
}
```

**Example:**

```json
{
  "id": "fact_a3f8c1",
  "type": "fact",
  "content": "The GTX 1660 desktop node runs Ubuntu 24.04 LTS with Docker 27.x installed",
  "content_hash": "sha256:e4a1b2...",
  "source": {
    "type": "execution",
    "trace_id": "trace_8f2d01"
  },
  "confidence": 0.95,
  "verified_at": "2026-07-31T14:22:00Z",
  "verified_by": "ops-agent-01",
  "verification_count": 3,
  "created_at": "2026-07-30T09:15:00Z",
  "created_by": "ops-agent-01",
  "department_origin": "operations",
  "contradicts": [],
  "supports": ["fact_b7e2d4"],
  "scope": {
    "environment": "gtx-desktop",
    "global": false
  },
  "tags": ["infrastructure", "gtx-desktop", "os", "docker"],
  "entropy": {
    "age_days": 1.2,
    "access_count": 14,
    "last_accessed": "2026-07-31T14:20:00Z",
    "decay_rate": 0.001,
    "current_confidence": 0.948
  },
  "status": "active"
}
```

---

### Type 2: Entity

A named thing in the world — a person, service, server, vendor, repository, API, credential, workflow, or any addressable noun.

```typescript
interface Entity {
  id: string;
  type: "entity";
  name: string;
  entity_type: EntityType;
  attributes: Record<string, AttributeValue>;
  created_at: ISO8601;
  updated_at: ISO8601;
  department_origin: Department;
  confidence: number;
  verified_at: ISO8601;
  tags: string[];
  entropy: EntropyState;
  status: "active" | "deprecated" | "merged";
  merged_into?: string;
}

type EntityType =
  | "service"
  | "node"
  | "person"
  | "vendor"
  | "repository"
  | "api"
  | "credential"
  | "workflow"
  | "model"
  | "provider"
  | "identity"
  | "department"
  | "customer"
  | "product"
  | "document"
  | "custom";

interface AttributeValue {
  value: unknown;
  confidence: number;
  verified_at: ISO8601;
  source: string;
}
```

---

### Type 3: Relationship

A typed connection between two entities.

```typescript
interface Relationship {
  id: string;
  type: "relationship";
  source_entity_id: string;
  target_entity_id: string;
  relationship_type: string;      // "depends_on", "owned_by", "runs_on", etc.
  attributes: Record<string, AttributeValue>;
  confidence: number;
  evidence: string[];              // fact IDs that support this relationship
  created_at: ISO8601;
  verified_at: ISO8601;
  department_origin: Department;
  entropy: EntropyState;
  status: "active" | "deprecated";
}
```

**Example:**

```json
{
  "id": "rel_c4d5e6",
  "type": "relationship",
  "source_entity_id": "entity_omniroute",
  "target_entity_id": "entity_gemini_provider",
  "relationship_type": "routes_to",
  "attributes": {
    "priority": { "value": 1, "confidence": 0.9, "verified_at": "...", "source": "..." },
    "avg_latency_ms": { "value": 340, "confidence": 0.85, "verified_at": "...", "source": "..." }
  },
  "confidence": 0.92,
  "evidence": ["fact_x1", "fact_x2", "routing_outcome_y1"],
  "created_at": "2026-07-30T10:00:00Z",
  "verified_at": "2026-07-31T12:00:00Z",
  "department_origin": "operations",
  "status": "active"
}
```

---

### Type 4: Procedure

A validated sequence of steps that achieves a known objective. Procedures are the bridge between declarative knowledge and executable capability.

```typescript
interface Procedure {
  id: string;
  type: "procedure";
  name: string;
  objective: string;               // what this procedure achieves
  trigger_pattern: string;         // when this procedure should be considered
  preconditions: Precondition[];
  steps: ProcedureStep[];
  postconditions: Postcondition[];
  success_criteria: string[];      // how to verify success
  failure_modes: FailureMode[];    // known ways this can fail
  success_rate: number;            // 0.0 – 1.0
  avg_duration_seconds: number;
  execution_count: number;
  last_executed_at: ISO8601;
  last_verified_at: ISO8601;
  version: number;
  evolved_from?: string;           // previous procedure version ID
  created_at: ISO8601;
  created_by: string;
  department_origin: Department;
  scope: ProcedureScope;
  tags: string[];
  entropy: EntropyState;
  status: "active" | "deprecated" | "superseded";
  superseded_by?: string;
}

interface ProcedureStep {
  order: number;
  action: string;                  // what to do
  tool?: string;                   // which tool to use
  parameters?: Record<string, unknown>;
  expected_output?: string;
  timeout_seconds?: number;
  on_failure: "retry" | "skip" | "abort" | "fallback";
  fallback_step?: number;         // if on_failure = "fallback"
}

interface Precondition {
  description: string;
  check: string;                   // how to verify this precondition
  required: boolean;
}

interface Postcondition {
  description: string;
  check: string;                   // how to verify this postcondition
}

interface FailureMode {
  description: string;
  symptom: string;
  recovery: string;
  frequency: number;               // how often this occurs (0.0 – 1.0)
}

interface ProcedureScope {
  departments: Department[];
  environments: string[];
  services: string[];
  global: boolean;
}
```

---

### Type 5: Failure

A recorded failure with full context. Failures are first-class knowledge — they prevent future repetitions and generate recovery procedures.

```typescript
interface Failure {
  id: string;
  type: "failure";
  what_happened: string;
  why: string;                     // root cause (if known)
  root_cause_confidence: number;
  environment_context: EnvironmentSnapshot;
  recovery_path: string;           // how it was resolved
  recovery_procedure_id?: string;  // if recovery became a procedure
  prevents: string[];              // procedure/fact IDs this failure invalidates
  related_failures: string[];      // IDs of similar failures
  severity: "low" | "medium" | "high" | "critical";
  frequency: number;               // occurrence rate
  first_seen_at: ISO8601;
  last_seen_at: ISO8601;
  occurrence_count: number;
  created_at: ISO8601;
  created_by: string;
  department_origin: Department;
  tags: string[];
  status: "open" | "resolved" | "recurring" | "accepted_risk";
}
```

---

### Type 6: Decision

A recorded decision with rationale. Prevents re-debating settled questions.

```typescript
interface Decision {
  id: string;
  type: "decision";
  question: string;
  context: string;                 // what prompted the decision
  options: DecisionOption[];
  chosen: string;                  // which option was selected
  rationale: string;               // why this option was chosen
  decided_by: string;              // human, agent, or governance rule
  decided_at: ISO8601;
  revisit_conditions: string[];    // when this decision should be reconsidered
  related_decisions: string[];
  department_origin: Department;
  scope: DecisionScope;
  tags: string[];
  status: "active" | "superseded" | "revisited";
  superseded_by?: string;
}

interface DecisionOption {
  label: string;
  pros: string[];
  cons: string[];
  estimated_cost?: string;
  estimated_effort?: string;
}
```

---

### Type 7: Constraint

A system-wide rule that limits what agents, services, or processes may do.

```typescript
interface Constraint {
  id: string;
  type: "constraint";
  scope: ConstraintScope;
  rule: string;                    // human-readable rule description
  rule_type: "hard" | "soft";     // hard = never violate, soft = prefer not to
  enforcement: EnforcementAction;
  source: string;                  // where this constraint came from
  severity: "info" | "warning" | "blocking" | "critical";
  created_at: ISO8601;
  created_by: string;
  department_origin: Department;
  tags: string[];
  status: "active" | "deprecated";
}

interface ConstraintScope {
  applies_to: ("agents" | "services" | "humans" | "workflows" | "all")[];
  departments?: Department[];
  environments?: string[];
  services?: string[];
  global: boolean;
}

type EnforcementAction = "log" | "warn" | "block" | "escalate" | "terminate";
```

---

### Type 8: Governance Rule

A standing policy that governs system behavior. More specific than constraints — these are conditional rules with defined actions.

```typescript
interface GovernanceRule {
  id: string;
  type: "governance_rule";
  name: string;
  description: string;
  condition: string;               // when this rule fires
  action: string;                  // what this rule does
  priority: number;                // higher = evaluated first
  scope: GovernanceScope;
  created_at: ISO8601;
  created_by: string;
  last_evaluated_at?: ISO8601;
  evaluation_count: number;
  fire_count: number;
  status: "active" | "suspended" | "deprecated";
}

interface GovernanceScope {
  departments: Department[];
  services: string[];
  global: boolean;
}
```

---

### Type 9: Routing Outcome

A record of an inference routing decision and its result. This is how OmniRoute learns.

```typescript
interface RoutingOutcome {
  id: string;
  type: "routing_outcome";
  request_hash: string;            // what was requested (dedup key)
  provider: string;
  model: string;
  identity: string;
  context_size_tokens: number;
  compression_strategy: string;
  input_tokens: number;
  output_tokens: number;
  cost_usd: number;
  latency_ms: number;
  quality_score: number;           // 0.0 – 1.0, if evaluated
  success: boolean;
  error_type?: string;
  created_at: ISO8601;
  tags: string[];
}
```

---

### Type 10: Environment State

A snapshot of conditions on a node or across the fleet.

```typescript
interface EnvironmentState {
  id: string;
  type: "environment_state";
  node_id: string;
  os: string;
  cpu: CPUState;
  memory: MemoryState;
  disk: DiskState;
  gpu?: GPUState;
  network: NetworkState;
  services_running: ServiceState[];
  captured_at: ISO8601;
  captured_by: string;
  status: "current" | "historical";
}
```

---

### Type 11: Skill (MetaClaw)

A reusable procedural capability promoted from an immutable `procedure_candidate` asset. OmniRoute owns trace interpretation and asset creation; MetaClaw owns eligibility, promotion, bounded injection, usage evaluation, and evolution.

```typescript
interface Skill {
  id: string;
  type: "skill";
  name: string;
  objective: string;               // what this skill achieves
  trigger_pattern: string;         // pattern that matches when this skill applies
  trigger_confidence: number;      // how reliably the pattern matches
  
  // The skill itself
  steps: SkillStep[];
  preconditions: Precondition[];
  postconditions: Postcondition[];
  parameters: SkillParameter[];    // inputs the skill accepts
  
  // Provenance
  source_assets: string[];         // immutable asset IDs supporting promotion
  promotion_policy: string;        // versioned policy that allowed promotion
  promoted_at: ISO8601;
  promoted_by: string;
  
  // Performance
  usage_count: number;
  success_count: number;
  failure_count: number;
  success_rate: number;
  avg_duration_seconds: number;
  last_used_at?: ISO8601;
  last_verified_at: ISO8601;
  
  // Evolution
  version: number;
  evolved_from?: string;           // previous version ID
  evolution_log: EvolutionEntry[];
  
  // Scope
  department_origin: Department;
  applicable_departments: Department[];
  applicable_environments: string[];
  global: boolean;
  
  // State
  tags: string[];
  entropy: EntropyState;
  status: "candidate" | "validated" | "active" | "stale" | "deprecated" | "superseded";
  superseded_by?: string;
}

interface SkillStep {
  order: number;
  action: string;
  tool?: string;
  parameters?: Record<string, unknown>;
  expected_output?: string;
  timeout_seconds?: number;
  on_failure: "retry" | "skip" | "abort" | "fallback" | "escalate";
  fallback_step?: number;
}

interface SkillParameter {
  name: string;
  type: string;
  required: boolean;
  default?: unknown;
  description: string;
}

interface EvolutionEntry {
  timestamp: ISO8601;
  change: string;                  // what changed
  reason: string;                  // why it changed
  from_version: number;
  to_version: number;
  validated: boolean;
}
```

---

### Type 12: Execution Trace

A complete, immutable record of an execution as observed by OmniRoute. It is source evidence for typed assets and projections; MetaClaw consumes the resulting provenance-linked assets and does not independently reinterpret the trace.

```typescript
interface ExecutionTrace {
  id: string;
  type: "execution_trace";
  
  // What was requested
  objective: string;
  intent_hash: string;             // dedup key for similar objectives
  
  // What happened
  steps_taken: TraceStep[];
  tools_used: string[];
  model_calls: ModelCall[];
  errors_encountered: TraceError[];
  human_interventions: HumanIntervention[];
  
  // Outcome
  outcome: "success" | "partial" | "failure" | "abandoned";
  result_summary: string;
  
  // Cost
  duration_seconds: number;
  total_tokens: number;
  total_cost_usd: number;
  model_call_count: number;
  human_intervention_count: number;
  
  // Skills
  skills_used: string[];           // skill IDs that were injected
  skills_produced: string[];       // skill IDs extracted from this trace
  
  // Context
  agent_id: string;
  department: Department;
  environment: string;
  node_id: string;
  session_id: string;
  
  // Timestamps
  started_at: ISO8601;
  completed_at: ISO8601;
  
  // Knowledge produced
  facts_produced: string[];        // fact IDs created during this execution
  decisions_produced: string[];
  failures_produced: string[];
  procedures_produced: string[];
}

interface TraceStep {
  order: number;
  action: string;
  tool?: string;
  input_summary: string;
  output_summary: string;
  duration_seconds: number;
  success: boolean;
  error?: string;
}

interface ModelCall {
  provider: string;
  model: string;
  identity: string;
  input_tokens: number;
  output_tokens: number;
  cost_usd: number;
  latency_ms: number;
  purpose: string;
}

interface TraceError {
  step: number;
  error: string;
  recovery_action: string;
  recovered: boolean;
}

interface HumanIntervention {
  step: number;
  reason: string;
  action_taken: string;
  duration_seconds: number;
}
```

---

### Entropy Tracking

Every knowledge item carries an entropy state. This is how YantrikDB knows what's stale.

```typescript
interface EntropyState {
  age_days: number;                // days since created_at
  days_since_verification: number; // days since verified_at
  access_count: number;            // total times accessed
  last_accessed: ISO8601;
  decay_rate: number;              // confidence loss per day since verification
  current_confidence: number;      // confidence after decay
  
  // Thresholds
  stale_threshold: number;         // current_confidence below this = stale
  reverify_threshold: number;      // current_confidence below this = needs reverification
  deprecate_threshold: number;     // current_confidence below this = auto-deprecate
}
```

**Confidence decay formula:**

```
current_confidence = base_confidence × (1 - decay_rate) ^ days_since_verification
```

**Decay rates by knowledge type:**

| Knowledge Type | Default Decay Rate | Rationale |
|---|---|---|
| Fact (environment-specific) | 0.005/day | Environments change |
| Fact (global/structural) | 0.0005/day | Structural knowledge is stable |
| Entity | 0.001/day | Entities change slowly |
| Relationship | 0.002/day | Relationships change with topology |
| Procedure | 0.003/day | Procedures break when environments change |
| Skill | 0.002/day | Skills decay with tool/API changes |
| Constraint | 0.0001/day | Constraints rarely change |
| Governance Rule | 0.0001/day | Policies are stable |
| Routing Outcome | 0.01/day | Provider performance shifts |
| Environment State | 0.05/day | Environments change rapidly |
| Failure | 0.001/day | Failure patterns are informative long-term |
| Decision | 0.0005/day | Decisions are stable unless revisit conditions met |

**Each access resets `days_since_verification` only if the access includes verification.** Read-only access updates `access_count` and `last_accessed` but does not reset decay.

---

### Contradiction Resolution

When a new fact contradicts an existing fact, YantrikDB does not silently overwrite. It stores both and resolves.

```
New fact F2 contradicts existing fact F1.

Step 1: Store F2 with status "active"
Step 2: Add F1.id to F2.contradicts[]
Step 3: Add F2.id to F1.contradicts[]

Step 4: Compare confidence scores
  - F1.effective_confidence = F1.confidence × verification_weight × recency_weight
  - F2.effective_confidence = F2.confidence × verification_weight × recency_weight

Step 5: Resolve
  - If |F1.effective - F2.effective| > resolution_threshold:
    → Higher confidence fact → status = "active"
    → Lower confidence fact → status = "contested"
  - If within threshold:
    → Both → status = "contested"
    → Generate verification task for OpenClaw

Step 6: Record the contradiction as a Decision (pending resolution)

Step 7: Notify affected consumers
  → Any service that has read F1 is notified that F1 is now contested
```

**Verification weight:**

```
verification_weight = 1.0 + (0.1 × verification_count)
```

**Recency weight:**

```
recency_weight = max(0.5, 1.0 - (days_since_verification × decay_rate))
```

**Resolution outcomes:**

| Outcome | Action |
|---|---|
| F2 verified, F1 invalidated | F1 → "superseded", F1.superseded_by = F2.id |
| F1 verified, F2 invalidated | F2 → "deprecated" |
| Both partially correct | Both → "active", new composite fact created |
| Neither verifiable | Both → "contested", human escalation |

---

### The Query Model

YantrikDB serves knowledge through a structured query interface. Every service queries YantrikDB before acting.

```typescript
interface KnowledgeQuery {
  // What to find
  knowledge_types: KnowledgeType[];
  
  // Filters
  tags?: string[];                 // match any tag
  departments?: Department[];      // match any department
  environments?: string[];         // match any environment
  services?: string[];             // match any service
  status?: KnowledgeStatus[];      // match any status
  min_confidence?: number;         // minimum confidence threshold
  max_age_days?: number;           // maximum age
  scope?: "global" | "department" | "environment" | "service";
  
  // Full-text
  text_query?: string;             // search within content/objective/name
  
  // Graph traversal
  related_to_entity?: string;      // find knowledge related to this entity
  relationship_type?: string;      // filter by relationship type
  
  // Ordering
  order_by: "confidence" | "recency" | "access_count" | "relevance";
  limit?: number;
}

type KnowledgeType =
  | "fact" | "entity" | "relationship" | "procedure"
  | "failure" | "decision" | "constraint" | "governance_rule"
  | "routing_outcome" | "environment_state" | "skill" | "execution_trace";

type KnowledgeStatus =
  | "active" | "contested" | "deprecated" | "superseded"
  | "candidate" | "validated" | "stale" | "open" | "resolved"
  | "recurring" | "accepted_risk" | "current" | "historical"
  | "suspended" | "partial" | "failure" | "abandoned";
```

**The critical query — "What do I already know about X?"**

```typescript
// Before any agent executes, it asks:
const relevant_knowledge = yantrikdb.query({
  knowledge_types: ["fact", "procedure", "skill", "failure", "constraint"],
  tags: ["deployment", "n8n", "gtx-desktop"],
  status: ["active", "validated"],
  min_confidence: 0.7,
  order_by: "relevance",
  limit: 50
});

// The result is injected into the agent's context.
// The agent does not re-discover what the system already knows.
```

---

### The Write Model

Knowledge enters YantrikDB through a structured write interface. No raw inserts — all writes go through validation.

```typescript
interface KnowledgeWrite {
  knowledge_type: KnowledgeType;
  payload: KnowledgePayload;       // one of the 12 type interfaces
  write_mode: "create" | "deduplicate" | "append_evaluation" | "derive";
  idempotency_key?: string;        // for deduplication
  source_trace_id?: string;        // OmniRoute execution trace that produced this
  asset_id?: string;               // immutable asset being projected
  conflict_resolution?: "reject" | "contradict" | "supersede";
}

interface WriteResult {
  success: boolean;
  id: string;                      // the created/updated knowledge item ID
  action: "created" | "deduplicated" | "derived" | "rejected" | "contradicted";
  contradiction?: {                // if a contradiction was detected
    existing_id: string;
    resolution: "pending" | "superseded" | "deprecated" | "both_active";
  };
  affected_skills?: string[];      // skills that may need re-evaluation
  notifications?: Notification[];  // services that should be notified
}
```

**Write validation pipeline:**

```
1. Schema validation
   → Does the payload conform to the type interface?
   
2. Deduplication
   → Does content_hash match an existing item?
   → If yes: upsert or reject based on write_mode
   
3. Contradiction detection
   → Does this contradict any existing active knowledge?
   → If yes: enter contradiction resolution flow
   
4. Scope validation
   → Is the scope internally consistent?
   → Does it conflict with existing constraints?
   
5. Entropy initialization
   → Set initial confidence, decay rate, thresholds
   
6. Persistence
   → Write to storage
   
7. Index update
   → Update search indices, tag indices, entity indices
   
8. Notification dispatch
   → Notify subscribed services of new/changed knowledge
```

---

### Continuity Model

YantrikDB IS continuity. If YantrikDB's state survives, the system survives. If YantrikDB's state is lost, the system loses its memory and must re-learn everything.

**Continuity requirements:**

```
┌─────────────────────────────────────────────────────────┐
│                                                         │
│  YantrikDB Continuity = System Continuity               │
│                                                         │
│  Storage:    Local-first, append-only write-ahead log   │
│  Replication: Async replication to ≥1 secondary node    │
│  Snapshots:  Periodic full snapshots to remote storage  │
│  Recovery:   Materialize from snapshot + replay WAL     │
│  Conflict:   Last-writer-wins with vector clocks        │
│                                                         │
│  RPO (Recovery Point Objective):  < 60 seconds          │
│  RTO (Recovery Time Objective):   < 300 seconds         │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

**Storage architecture:**

```
Primary Node (GTX Desktop — preferred)
├── WAL (write-ahead log) — append-only, durable
├── Primary store — current knowledge state
├── Index store — search indices
└── Snapshot store — periodic snapshots

Secondary Node (Dell Laptop — failover)
├── Replicated WAL — async from primary
├── Replicated store — materialized from WAL
└── Can promote to primary on failure detection

Snapshot Storage (any reachable node or external)
├── Full snapshots — periodic
└── Incremental snapshots — between full snapshots
```

**Node failure recovery:**

```
1. n8n detects node failure (health check timeout)
2. n8n selects available substrate (next capable node)
3. n8n triggers YantrikDB materialization:
   a. Pull latest snapshot from snapshot storage
   b. Replay WAL entries since snapshot
   c. Rebuild search indices
   d. Verify replication consistency
4. YantrikDB is operational on new node
5. All dependent services reconnect to YantrikDB
6. System continues execution
```

---

### YantrikDB API Surface

```typescript
interface YantrikDB {
  // Core CRUD
  write(input: KnowledgeWrite): Promise<WriteResult>;
  read(id: string): Promise<KnowledgeItem | null>;
  update(id: string, patch: Partial<KnowledgePayload>): Promise<WriteResult>;
  deprecate(id: string, reason: string, superseded_by?: string): Promise<void>;
  
  // Query
  query(input: KnowledgeQuery): Promise<KnowledgeItem[]>;
  search(text: string, filters?: Partial<KnowledgeQuery>): Promise<KnowledgeItem[]>;
  
  // Graph operations
  getEntity(id: string): Promise<Entity | null>;
  getRelationships(entityId: string, type?: string): Promise<Relationship[]>;
  getRelatedKnowledge(entityId: string, types: KnowledgeType[]): Promise<KnowledgeItem[]>;
  
  // Contradiction management
  getContradictions(status?: "pending" | "resolved"): Promise<ContradictionReport[]>;
  resolveContradiction(id: string, resolution: ContradictionResolution): Promise<void>;
  
  // Entropy management
  getStaleKnowledge(threshold?: number): Promise<KnowledgeItem[]>;
  triggerReverification(ids: string[]): Promise<ReverificationTask[]>;
  getEntropyReport(): Promise<EntropyReport>;
  
  // Continuity
  takeSnapshot(): Promise<SnapshotResult>;
  restoreFromSnapshot(snapshotId: string): Promise<RestoreResult>;
  getReplicationStatus(): Promise<ReplicationStatus>;
  
  // Subscriptions
  subscribe(filter: KnowledgeQuery, callback: KnowledgeCallback): Subscription;
  unsubscribe(subscriptionId: string): void;
  
  // MetaClaw interface (see Part II)
  skills: SkillStore;
}
```

---

## Part II: MetaClaw [skills_only mode]

### What MetaClaw Is (In This Mode)

MetaClaw in `skills_only` mode is the procedural policy and skill consumer. OmniRoute owns the raw execution trace and trace-to-reusable-asset engine. MetaClaw consumes versioned assets and YantrikDB projected procedural state. It does three things:

1. **Evaluates** procedure candidates and evidence
2. **Injects** validated projected skills into future executions
3. **Evolves** skill policy and versions from evaluated outcomes

It does not do full procedural interception, runtime interception, or execution modification in this mode. It is an asset-consume, policy, inject, and evolve loop on top of YantrikDB. It must not duplicate OmniRoute's asset extraction engine.

```
┌──────────────────────────────────────────────────────────┐
│                                                          │
│              METACLAW [skills_only mode]                 │
│                                                          │
│                                                          │
│   Execution Traces ──► OBSERVE ──► EXTRACT ──► Skills   │
│        (from agents)                  (to YantrikDB)     │
│                                           │              │
│                                           ▼              │
│   Future Executions ◄── INJECT ◄── MATCH                │
│        (to agents)     (known skills)  (against intent) │
│                                                          │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

### The Skill Lifecycle

```
candidate ──► validated ──► active ──► stale ──► deprecated
    │             │            │          │
    │             │            │          └──► (re-validated) ──► active
    │             │            │
    │             │            └──► superseded ──► deprecated
    │             │
    │             └──► (validation failed) ──► deprecated
    │
    └──► (insufficient evidence) ──► remains candidate
```

**State transitions:**

| From | To | Condition |
|---|---|---|
| candidate | validated | ≥3 successful executions using this skill, success_rate ≥ 0.8 |
| validated | active | Approved by governance rule OR auto-approved after 7 days validated |
| active | stale | entropy.current_confidence < entropy.stale_threshold |
| active | superseded | New skill version created with higher success_rate |
| active | deprecated | success_rate drops below 0.5 over last 10 uses |
| stale | active | Re-verification succeeds, confidence restored |
| stale | deprecated | Re-verification fails or not re-verified within 30 days |
| superseded | deprecated | Immediate |
| candidate | deprecated | 30 days as candidate without reaching validated |

---

### Phase 1: Consume and Validate Assets

MetaClaw consumes immutable assets produced by OmniRoute and projected through YantrikDB. It does not treat an uncorrelated raw trace or agent self-report as sufficient skill evidence.

```typescript
interface AssetObserver {
  // Subscribe to immutable assets with source lineage
  onAsset(asset: EvidencePreservingAsset): void;
  
  // Filter: which traces are worth analyzing
  shouldEvaluate(asset: EvidencePreservingAsset): boolean;
}

// An asset is eligible for procedural evaluation only when:
function shouldEvaluate(asset: EvidencePreservingAsset): boolean {
  return (
    asset.type === "procedure_candidate" &&
    asset.provenance.source_execution_ids.length > 0 &&
    asset.evaluation.objective_success === true &&
    asset.evaluation.safety_pass === true &&
    !isDuplicateAsset(asset)
  );
}
```

---

### Phase 2: Consume Projected Procedure Candidates

MetaClaw evaluates procedure-candidate assets emitted by OmniRoute and projected by YantrikDB. Any extraction strategy shown below is an OmniRoute asset-engine concern or a separately versioned validator; it is not an implicit second extractor inside MetaClaw.

```typescript
interface ProcedureCandidateValidator {
  // Evaluate an immutable asset already emitted by OmniRoute and projected by YantrikDB.
  validate(asset: EvidencePreservingAsset): Promise<ProcedureEvaluation>;
}

interface CandidateSkill {
  objective: string;               // inferred objective
  trigger_pattern: string;         // when this skill applies
  steps: SkillStep[];              // the extracted procedure
  confidence: number;              // how confident we are this is reusable
  evidence: ExtractionEvidence;    // why we think this is a skill
}

interface ExtractionEvidence {
  pattern_type: "repeated_sequence" | "similar_traces" | "explicit_procedure";
  matching_traces: string[];       // trace IDs that exhibit this pattern
  similarity_score: number;        // how similar the matching traces are
  frequency: number;               // how often this pattern appears
}
```

**Asset-engine reference strategies (owned by OmniRoute, not implemented as a second MetaClaw extractor):**

#### Strategy A: Repeated Sequence Detection

```
Given: N successful traces with similar objectives
Find: Step subsequences that appear in ≥3 traces with ≥80% similarity
Output: Candidate skill from the common subsequence

Algorithm:
1. Group traces by intent_hash similarity (cosine > 0.85)
2. For each group with ≥3 traces:
   a. Align step sequences using longest common subsequence
   b. Identify steps that appear in ≥80% of traces
   c. Extract the common subsequence as a candidate skill
   d. Set confidence = (frequency × similarity_score)
```

#### Strategy B: Explicit Procedure Detection

```
Given: A single trace where the agent documented a procedure
Find: Steps that the agent explicitly labeled as reusable
Output: Candidate skill from the documented procedure

Algorithm:
1. Scan trace for steps with explicit procedure annotations
2. Extract annotated steps as candidate skill
3. Set confidence = 0.7 (needs validation through reuse)
```

#### Strategy C: Cross-Trace Pattern Mining

```
Given: All traces from the last N days
Find: Step patterns that predict success
Output: Candidate skills from predictive patterns

Algorithm:
1. Encode each trace as a sequence of (action, tool, outcome) tuples
2. Apply sequential pattern mining (e.g., PrefixSpan)
3. Filter patterns that:
   - Appear in ≥5 traces
   - Have success_rate ≥ 0.8 when present
   - Have success_rate < 0.5 when absent (i.e., they're predictive)
4. Extract top patterns as candidate skills
```

**Deduplication:**

Before creating a new candidate skill, check if an existing skill already covers the same objective with the same trigger pattern. If yes, merge the new evidence into the existing skill rather than creating a duplicate.

```typescript
async function deduplicate(candidate: CandidateSkill): Promise<string | null> {
  const existing = await yantrikdb.skills.findByTriggerPattern(
    candidate.trigger_pattern,
    { min_confidence: 0.5, status: ["candidate", "validated", "active"] }
  );
  
  for (const skill of existing) {
    const similarity = computeSimilarity(candidate.steps, skill.steps);
    if (similarity > 0.85) {
      // Merge: add this trace as additional evidence for the existing skill
      await yantrikdb.skills.addEvidence(skill.id, candidate.evidence);
      return skill.id;
    }
  }
  
  return null; // no duplicate found, create new skill
}
```

---

### Phase 3: Inject

Before an agent executes, MetaClaw searches for applicable skills and injects them into the agent's context.

```typescript
interface SkillInjector {
  // Given an objective, find applicable skills
  match(objective: string, context: ExecutionContext): Promise<MatchedSkill[]>;
  
  // Format skills for injection into agent context
  format(skills: MatchedSkill[]): SkillInjectionPayload;
}

interface MatchedSkill {
  skill: Skill;
  match_score: number;             // 0.0 – 1.0, how well it matches
  match_reason: string;            // why this skill was selected
}

interface ExecutionContext {
  department: Department;
  environment: string;
  node_id: string;
  available_tools: string[];
  constraints: Constraint[];
}

interface SkillInjectionPayload {
  skills: FormattedSkill[];
  total_tokens: number;            // budget tracking
  injection_mode: "context" | "system_prompt" | "tool_description";
}
```

**Matching algorithm:**

```
Given: objective (string) + context
Find: all active/validated skills where trigger_pattern matches objective

1. Text matching:
   - Compute semantic similarity between objective and each skill's trigger_pattern
   - Use embedding similarity (cosine > 0.8) or keyword overlap (> 60%)

2. Context filtering:
   - Remove skills not applicable to current environment
   - Remove skills not applicable to current department (unless global)
   - Remove skills requiring tools not available in context

3. Constraint checking:
   - Remove skills whose steps violate active constraints
   
4. Ranking:
   - Primary: match_score (semantic similarity)
   - Secondary: success_rate
   - Tertiary: usage_count (prefer well-tested skills)
   - Quaternary: recency (prefer recently verified skills)

5. Budget allocation:
   - Total injection budget: N tokens (configurable, default 4000)
   - Allocate tokens to top-ranked skills until budget exhausted
   - Higher-ranked skills get more detailed step descriptions
```

**Injection format (context mode):**

```
## Applicable Skills (from organizational memory)

### Skill: deploy-n8n-workflow
Objective: Deploy a new n8n workflow to the production instance
Match: 0.92 — your objective matches this skill's trigger pattern
Success rate: 94% (47 executions)
Last verified: 2 hours ago

Steps:
1. Validate workflow JSON against n8n schema
2. POST to https://n8n.internal/api/v1/workflows with auth header from vault
3. Activate the workflow via PUT /api/v1/workflows/{id}/activate
4. Verify activation via GET /api/v1/workflows/{id} status check
5. Run test execution via POST /api/v1/workflows/{id}/test

Preconditions:
- n8n instance is reachable (check: GET https://n8n.internal/healthz)
- Auth token is available in vault under key "n8n-api-token"

Known failure modes:
- Auth token expired → refresh from vault, retry step 2
- Schema validation fails → fix JSON, do not skip validation
```

---

### Phase 4: Evolve

Skills are versioned and evolved based on execution feedback.

```typescript
interface SkillEvolution {
  // After a skill is used, record the outcome
  recordUsage(skillId: string, trace: ExecutionTrace, outcome: SkillUsageOutcome): Promise<void>;
  
  // Check if a skill needs evolution
  evaluateSkill(skillId: string): Promise<EvolutionRecommendation | null>;
  
  // Apply an evolution (create new version)
  evolve(skillId: string, changes: SkillChanges, reason: string): Promise<Skill>;
}

interface SkillUsageOutcome {
  skill_steps_followed: number;    // how many steps were followed
  skill_steps_succeeded: number;   // how many steps succeeded
  deviations: StepDeviation[];     // where the agent deviated from the skill
  overall_success: boolean;
  duration_seconds: number;
}

interface StepDeviation {
  step_order: number;
  expected: string;
  actual: string;
  reason: string;
  outcome: "better" | "equivalent" | "worse";
}

interface EvolutionRecommendation {
  skill_id: string;
  recommendation: "update_steps" | "add_parameter" | "deprecate" | "split" | "merge";
  evidence: string;
  proposed_changes: SkillChanges;
  confidence: number;
}
```

**Evolution triggers:**

| Trigger | Condition | Action |
|---|---|---|
| Consistent deviation | ≥5 recent uses deviate at the same step in the same way | Update the step to match the deviation (if outcome = "better" or "equivalent") |
| Declining success rate | success_rate drops > 0.15 over last 20 uses vs. all-time | Flag for review; if specific step identified, update or deprecate |
| New failure mode | A previously unseen failure occurs ≥3 times | Add failure mode to skill, add recovery step |
| Parameter generalization | ≥3 uses substitute the same hardcoded value with a variable | Extract parameter |
| Skill too broad | success_rate varies significantly across departments/environments | Split into department-specific or environment-specific variants |
| Skill overlap | Two skills have > 0.85 step similarity | Merge into single skill with broader scope |

**Versioning:**

```
Skill v1 (active)
  ↓ evolution event
Skill v2 (candidate) ← created from v1 with changes
  ↓ validation (≥3 successful uses)
Skill v2 (validated)
  ↓ auto-approval or governance approval
Skill v2 (active)
Skill v1 → status = "superseded", superseded_by = v2.id
```

---

### MetaClaw API Surface

```typescript
interface MetaClaw {
  // Consume an immutable, provenance-linked asset from the YantrikDB projection.
  consumeAsset(asset: EvidencePreservingAsset): Promise<AssetConsumptionResult>;
  
  // Inject
  findApplicableSkills(objective: string, context: ExecutionContext): Promise<MatchedSkill[]>;
  formatForInjection(skills: MatchedSkill[], budget: number): SkillInjectionPayload;
  
  // Evolve
  recordSkillUsage(skillId: string, executionId: string, outcome: SkillUsageOutcome): Promise<void>;
  evaluateAllSkills(): Promise<EvolutionRecommendation[]>;
  evolveSkill(skillId: string, changes: SkillChanges, reason: string): Promise<Skill>;
  
  // Management
  getSkill(id: string): Promise<Skill | null>;
  listSkills(filter?: SkillFilter): Promise<Skill[]>;
  getSkillReport(): Promise<SkillReport>;
}

interface IngestResult {
  trace_id: string;
  skills_extracted: number;
  skills_updated: number;          // existing skills that gained new evidence
  skills_matched: number;          // skills that were applicable to this trace
}

interface SkillFilter {
  status?: SkillStatus[];
  department?: Department;
  environment?: string;
  min_confidence?: number;
  min_success_rate?: number;
  tags?: string[];
}

interface SkillReport {
  total_skills: number;
  by_status: Record<SkillStatus, number>;
  by_department: Record<Department, number>;
  avg_success_rate: number;
  total_usage_count: number;
  skills_needing_evolution: number;
  skills_stale: number;
  top_skills: Skill[];             // by usage_count × success_rate
}
```

---

### The Compounding Loop (Mechanically Specified)

This is the complete loop with no hand-waving:

The authoritative integration is an evidence projection loop, not a second trace-extraction loop:

```text
OmniRoute trace events
  → OmniRoute immutable typed asset
  → Evidence-Preserving Asset Contract
  → YantrikDB projection (validation, calibration, decay, links, conflicts, think)
  → MetaClaw projected procedural state
  → bounded injection
  → OmniRoute observed next execution
  → append-only evaluation and evidence
```

The numbered sequence below is retained as an operational narrative. Any step that conflicts with the ownership boundary above is superseded by it.

```
1. Human provides intent to OpenClaw
   │
2. OpenClaw queries YantrikDB for relevant projected knowledge
   │  → facts, procedures, constraints, environment state
   │
3. OpenClaw queries MetaClaw for applicable projected procedural state
   │  → MetaClaw matches trigger patterns against intent
   │  → Returns ranked skills formatted for injection
   │
4. OpenClaw decomposes intent into execution plan
   │  → Plan includes injected skills as known-good procedures
   │  → Plan includes relevant facts as established context
   │  → Plan includes constraints as boundaries
   │
5. Agents execute the plan
   │  → Agents follow skill steps where applicable
   │  → Agents deviate where skills are insufficient
   │  → Agents discover new knowledge during execution
   │
6. OmniRoute produces immutable execution trace events and reusable typed assets
   │  → Every step, tool call, model call, error, and outcome recorded
   │
7. Assets and evidence references are projected into YantrikDB
   │  → New facts extracted from execution
   │  → New failures recorded
   │  → New decisions documented
   │  → Environment state updated
   │
8. MetaClaw consumes projected procedure candidates
   │  → Checks asset provenance and objective evaluator results
   │  → Records skill usage outcomes (did the projected skill work?)
   │  → Applies lifecycle/evolution policy without rewriting evidence
   │
9. Knowledge compounds
   │  → YantrikDB has more facts, updated state, new failures
   │  → MetaClaw has new/evolved skills
   │  → Next execution of similar intent starts with MORE accumulated capability
   │
10. Next execution is cheaper
    │  → Fewer steps need discovery (skills cover them)
    │  → Fewer facts need re-research (YantrikDB has them)
    │  → Fewer failures need recovery (known failure modes have recovery paths)
    │  → Less human coordination needed (context is in memory)
    │
    └──→ Gradient = Compounding
         HITL Coordination Tax % → decreasing
         Compression Ratio % → increasing
```

---

## Part III: Implementation Priorities

### What To Build First

```
Phase A: YantrikDB Core
  ├── Storage layer (WAL + primary store)
  ├── Knowledge type schemas + validation
  ├── Write pipeline (validation → dedup → contradiction → persist → index → notify)
  ├── Query engine (filters, text search, graph traversal)
  ├── Entropy tracking + decay computation
  └── API surface

Phase B: YantrikDB Continuity
  ├── Replication to secondary node
  ├── Snapshot mechanism
  ├── WAL replay for recovery
  └── Health check interface for n8n

Phase C: YantrikDB Advanced
  ├── Contradiction resolution engine
  ├── Subscription/notification system
  ├── Entropy-based stale detection + reverification triggers
  └── Governance rule evaluation

Phase D: MetaClaw Observe + Inject
  ├── OmniRoute asset adapter and provenance validation
  ├── Skill matching (trigger pattern → objective)
  ├── Skill injection formatting
  └── Projected procedural state consumer

Phase E: MetaClaw Asset Evaluation
  ├── Procedure-candidate validation
  ├── Objective/subjective evaluator separation
  ├── Evidence deduplication and append-only updates
  └── Candidate promotion policy

Phase F: MetaClaw Evolve
  ├── Usage outcome recording
  ├── Deviation detection
  ├── Evolution recommendation engine
  └── Version management
```

### What NOT To Build (In This Phase)

| Out of Scope | Why |
|---|---|
| Runtime execution interception | skills_only mode is observe-inject, not intercept |
| Full procedural interception | MetaClaw does not modify running executions |
| Multi-modal knowledge (images, audio) | Text-only knowledge types first |
| Distributed consensus | Last-writer-wins with vector clocks is sufficient |
| Real-time streaming | Batch processing of traces is sufficient |
| External knowledge import | Internal execution traces first |

---

## Appendix: Department Enum

```typescript
type Department =
  | "engineering"
  | "operations"
  | "research"
  | "sales"
  | "marketing"
  | "finance"
  | "procurement"
  | "security"
  | "governance"
  | "support"
  | "product"
  | "executive"
  | "cross_department";
```

## Appendix: Type Aliases

```typescript
type ISO8601 = string;             // "2026-07-31T14:22:00Z"
type KnowledgePayload = Fact | Entity | Relationship | Procedure | Failure
  | Decision | Constraint | GovernanceRule | RoutingOutcome
  | EnvironmentState | Skill | ExecutionTrace;
type KnowledgeItem = KnowledgePayload & { _metadata: QueryMetadata };
type KnowledgeCallback = (event: KnowledgeEvent) => void;
type SkillStatus = "candidate" | "validated" | "active" | "stale" | "deprecated" | "superseded";
```

---

*The swarm executes the work. YantrikDB retains the gain. MetaClaw compounds it.*
