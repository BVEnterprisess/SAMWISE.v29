# Evidence-Preserving Asset Contract Falsification Implementation Plan

> **SUPERSEDED — 2026-08-07. Do not execute.**
>
> This plan's file map targets `samwise-core/`, which is archived on
> `archive/samwise-core-v0` (see `docs/ARCHIVE.md`), and its commands are written for
> Windows PowerShell. It is retained for its contract reasoning — the asset, evidence,
> evaluation, canonicalization, and projection definitions remain sound and inform the
> current specs. Its paths, commands, and task order do not.
>
> Current direction: `docs/superpowers/specs/2026-08-06-governor-control-loop-design.md`.

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the Pass 0 contract and falsification harness that makes immutable assets, append-only evidence/evaluations, canonical identities, versioned policies, deterministic projections, and compounding measurements executable before production learning behavior is added.

**Architecture:** Add a small reusable `evidence` library to `samwise-core`. It owns only the immutable contract and deterministic projection primitives; it does not extract skills, call OmniRoute, mutate YantrikDB, or change proxy behavior. Tests exercise the library through public interfaces and use fixture event histories to prove replay, ordering, partial evidence, canonicalization, and policy pinning.

**Tech Stack:** Rust 2024, serde/serde_json, blake3, thiserror, Rust unit/integration tests, JSON fixtures.

## Global Constraints

- Assets are immutable; evidence links, evaluations, relations, and policy decisions are append-only.
- `content_hash = Hash(Canonicalize(canonical_payload, schema_version, canonicalization_policy_version))`.
- The default event-order classification is non-commutative until a test proves commutativity.
- Objective and subjective evaluator outputs remain separate.
- The projection must be deterministic and replayable from immutable history plus an immutable policy bundle.
- Partial evidence must produce an explicit `partial` or `contested` projection.
- No production endpoint or runtime behavior changes in Pass 0.
- The existing uncommitted source changes in `samwise-core` and `Cargo.lock` are not included in Pass 0 commits.
- The null hypothesis is `H0: the learned projection provides no measurable improvement over baseline.`
- Learning claims use paired comparisons, quality constraints, effect sizes, and pre-registered statistical tests.

## File Map

- Create: `samwise-core/src/lib.rs` — public library entry point for the contract module.
- Create: `samwise-core/src/evidence/mod.rs` — public exports and module boundary.
- Create: `samwise-core/src/evidence/types.rs` — immutable asset, evidence, evaluation, relation, policy, event, and projection types.
- Create: `samwise-core/src/evidence/canonical.rs` — deterministic canonicalization and content hashing.
- Create: `samwise-core/src/evidence/projection.rs` — deterministic status projection and event-order checks.
- Create: `samwise-core/src/evidence/benchmark.rs` — paired benchmark records, derivative metrics, and null-hypothesis calculations.
- Create: `samwise-core/tests/evidence_contract.rs` — integration tests for all public contract behavior.
- Modify: `samwise-core/Cargo.toml` — add the library target and direct `blake3` dependency.
- Create: `samwise-core/tests/fixtures/evidence/` — compact JSON histories for canonicalization, partial evidence, conflict, ordering, and replay cases.

### Task 1: Create the public evidence module boundary

**Files:**
- Modify: `samwise-core/Cargo.toml`
- Create: `samwise-core/src/lib.rs`
- Create: `samwise-core/src/evidence/mod.rs`

**Interfaces:**
- Produces `samwise_core::evidence` for later adapters and tests.
- Does not import the existing binary-only server/proxy modules.

- [ ] **Step 1: Write the failing compile test**

Create `samwise-core/tests/evidence_contract.rs` with:

```rust
use samwise_core::evidence::{Asset, AssetType};

#[test]
fn evidence_module_is_public() {
    let _ = Asset::new(AssetType::Observation, 1, serde_json::json!({"x": 1}));
}
```

- [ ] **Step 2: Run the test to verify it fails**

Run from `samwise-core`:

```powershell
rustup run stable-x86_64-pc-windows-msvc cargo test --test evidence_contract evidence_module_is_public
```

Expected: compilation fails because the library target and `Asset` type do not exist.

- [ ] **Step 3: Add the library target and module shell**

Add `blake3 = "1.8"` to `[dependencies]`, add `[lib] path = "src/lib.rs"`, create `src/lib.rs` with `pub mod evidence;`, and create `src/evidence/mod.rs` with `mod types; pub use types::*;`.

- [ ] **Step 4: Run the test to verify the module boundary passes**

Run the same targeted command. Expected: only the constructor/type implementation remains as the next failure.

- [ ] **Step 5: Commit**

```powershell
git add samwise-core/Cargo.toml samwise-core/src/lib.rs samwise-core/src/evidence/mod.rs samwise-core/tests/evidence_contract.rs
git commit -m "test: establish evidence contract module boundary"
```

### Task 2: Define immutable contract types

**Files:**
- Create: `samwise-core/src/evidence/types.rs`
- Modify: `samwise-core/src/evidence/mod.rs`
- Modify: `samwise-core/tests/evidence_contract.rs`

**Interfaces:**

```rust
pub enum AssetType { Observation, Fact, Failure, FailurePattern, ProcedureCandidate, Metric, Constraint, Decision, EnvironmentState, SkillReference }
pub enum EvaluationClass { Objective, Subjective }
pub enum EvidenceRelation { Supports, Contradicts, Explains, Uses }
pub enum AssetRelationKind { Supersedes, Splits, Merges, Corrects, DerivesFrom }
pub enum ProjectionStatus { Complete, Partial, Contested, Rejected }
pub enum EventOrderClass { Commutative, Causal, Conditional }
pub struct Asset { pub asset_id: String, pub asset_type: AssetType, pub schema_version: u32, pub canonical_payload: serde_json::Value, pub content_hash: String, pub canonicalization_policy_version: String, pub created_at_ms: i64 }
pub struct EvidenceLink { pub link_id: String, pub asset_id: String, pub source_event_id: String, pub relation: EvidenceRelation, pub strength: f64, pub created_at_ms: i64 }
pub struct Evaluation { pub evaluation_id: String, pub asset_id: String, pub class: EvaluationClass, pub outcome: String, pub confidence: f64, pub uncertainty_low: f64, pub uncertainty_high: f64, pub utility: Option<f64>, pub evaluator_version: String, pub policy_version: String, pub created_at_ms: i64 }
pub struct AssetRelation { pub relation_id: String, pub from_asset_id: String, pub to_asset_id: String, pub kind: AssetRelationKind, pub created_at_ms: i64 }
pub struct PolicyBundle { pub policy_version: String, pub canonicalization_version: String, pub projection_version: String, pub promotion_version: String, pub content_hash: String }
```

`Asset::new` accepts a typed asset kind, schema version, and payload; it computes identity through `canonical::hash_payload` and never exposes a mutator for payload or identity.

- [ ] **Step 1: Write failing invariant tests**

Add tests that construct an asset, serialize it, and assert that identity fields are present; construct objective and subjective evaluations and assert their classes remain distinct; assert confidence and uncertainty bounds reject values outside `[0, 1]` or intervals where `low > high`.

- [ ] **Step 2: Run the targeted tests to verify failure**

```powershell
rustup run stable-x86_64-pc-windows-msvc cargo test --test evidence_contract asset_identity_and_evaluator_bounds
```

Expected: the types and validation behavior are missing.

- [ ] **Step 3: Implement the types and constructors**

Use serde derives, private fields where mutation would violate immutability, explicit constructors returning `Result`, and `thiserror` errors for invalid bounds, empty IDs, invalid timestamps, and unsupported schema versions.

- [ ] **Step 4: Run the tests and inspect serialized fixtures**

Run the targeted test and serialize one asset, one evidence link, one objective evaluation, one subjective evaluation, one relation, and one policy bundle. Confirm no mutable projection status is stored on `Asset`.

- [ ] **Step 5: Commit**

```powershell
git add samwise-core/src/evidence/types.rs samwise-core/src/evidence/mod.rs samwise-core/tests/evidence_contract.rs
git commit -m "feat: define immutable evidence contract types"
```

### Task 3: Implement canonicalization and content addressing

**Files:**
- Create: `samwise-core/src/evidence/canonical.rs`
- Modify: `samwise-core/src/evidence/mod.rs`
- Modify: `samwise-core/src/evidence/types.rs`
- Modify: `samwise-core/tests/evidence_contract.rs`

**Interfaces:**

```rust
pub fn canonicalize(value: &serde_json::Value, policy_version: &str) -> Result<Vec<u8>, CanonicalizationError>;
pub fn hash_payload(value: &serde_json::Value, schema_version: u32, policy_version: &str) -> Result<String, CanonicalizationError>;
pub fn equivalent_identity(left: &serde_json::Value, right: &serde_json::Value, schema_version: u32, policy_version: &str) -> Result<bool, CanonicalizationError>;
```

Canonicalization must sort object keys, preserve array order, normalize supported finite numbers, normalize Unicode, emit UTF-8, reject NaN/Infinity, include schema and policy versions in the hashed envelope, and reject unsupported values instead of silently stringifying them.

- [ ] **Step 1: Write failing canonicalization tests**

Cover object-key reordering, whitespace changes, array reordering, numeric normalization, Unicode normalization, NaN/Infinity rejection, schema-version changes, and policy-version changes.

- [ ] **Step 2: Run tests to verify failure**

```powershell
rustup run stable-x86_64-pc-windows-msvc cargo test --test evidence_contract canonicalization
```

- [ ] **Step 3: Implement deterministic serialization and hashing**

Serialize recursively with explicit object ordering and type handling, prepend a domain separator plus schema/policy versions, and hash the resulting bytes with BLAKE3. Return lowercase hexadecimal hashes.

- [ ] **Step 4: Run tests and compare known hashes**

Run the targeted suite and assert exact expected hashes for fixed fixtures. Any canonicalization change must require a policy-version change and produce a different hash.

- [ ] **Step 5: Commit**

```powershell
git add samwise-core/src/evidence/canonical.rs samwise-core/src/evidence/types.rs samwise-core/src/evidence/mod.rs samwise-core/tests/evidence_contract.rs
git commit -m "feat: add versioned canonical asset identity"
```

### Task 4: Implement deterministic projections and event-order rules

**Files:**
- Create: `samwise-core/src/evidence/projection.rs`
- Modify: `samwise-core/src/evidence/mod.rs`
- Modify: `samwise-core/tests/evidence_contract.rs`
- Create: `samwise-core/tests/fixtures/evidence/`

**Interfaces:**

```rust
pub struct ProjectionInput<'a> { pub assets: &'a [Asset], pub evidence: &'a [EvidenceLink], pub evaluations: &'a [Evaluation], pub relations: &'a [AssetRelation], pub policy: &'a PolicyBundle }
pub struct AssetProjection { pub asset_id: String, pub status: ProjectionStatus, pub support_count: usize, pub contradiction_count: usize, pub objective_confidence: Option<f64>, pub subjective_confidence: Option<f64>, pub complete: bool }
pub fn project(input: ProjectionInput<'_>) -> Result<Vec<AssetProjection>, ProjectionError>;
pub fn classify_event(event_type: &str) -> EventOrderClass;
pub fn histories_commute(left: &[EventEnvelope], right: &[EventEnvelope], policy: &PolicyBundle) -> Result<bool, ProjectionError>;
```

Projection rules: no evidence means `Partial`; support plus unresolved contradiction means `Contested`; rejected objective evaluation prevents promotion; subjective evaluations never alter objective confidence; missing referenced events produce `Partial`; duplicate IDs are rejected; relations must be acyclic for supersession/split/merge projections.

- [ ] **Step 1: Write failing projection tests**

Cover empty evidence, partial evidence, objective success, objective failure, subjective-only evaluation, support plus contradiction, correction, supersession, split, merge, duplicate event, missing event, and policy-version mismatch.

- [ ] **Step 2: Run tests to verify failure**

```powershell
rustup run stable-x86_64-pc-windows-msvc cargo test --test evidence_contract projection
```

- [ ] **Step 3: Implement the pure projection function**

Sort inputs by stable IDs before folding, apply only the policy bundle supplied in the input, preserve objective/subjective channels, and return explicit incomplete/contested statuses instead of guessing.

- [ ] **Step 4: Implement commutativity checks**

For event histories `H + A + B` and `H + B + A`, compare canonical serialized projections. Treat events as non-commutative unless the event classifier and projection result prove equality.

- [ ] **Step 5: Run the full contract suite**

```powershell
rustup run stable-x86_64-pc-windows-msvc cargo test --test evidence_contract -- --nocapture
```

Expected: deterministic projections and explicit failures for invalid histories.

- [ ] **Step 6: Commit**

```powershell
git add samwise-core/src/evidence/projection.rs samwise-core/src/evidence/mod.rs samwise-core/tests/evidence_contract.rs samwise-core/tests/fixtures/evidence
git commit -m "feat: add deterministic evidence projections"
```

### Task 5: Add compounding benchmark measurements and H0 test

**Files:**
- Create: `samwise-core/src/evidence/benchmark.rs`
- Modify: `samwise-core/src/evidence/mod.rs`
- Modify: `samwise-core/tests/evidence_contract.rs`

**Interfaces:**

```rust
pub struct ExecutionMeasurement { pub execution_id: String, pub task_family: String, pub success: bool, pub safety_pass: bool, pub latency_ms: f64, pub tokens: u64, pub retries: u32, pub hitl_interventions: u32, pub hitl_coordination_ms: f64, pub task_quality: f64 }
pub struct PairedMeasurement { pub run_one: ExecutionMeasurement, pub run_two: ExecutionMeasurement, pub control_two: Option<ExecutionMeasurement> }
pub struct CompoundingReport { pub hitl_tax_delta: f64, pub compression_ratio: f64, pub quality_delta: f64, pub success_delta: f64, pub h0_rejected: bool, pub safety_pass: bool }
pub fn hitl_coordination_tax(measurement: &ExecutionMeasurement) -> Result<f64, BenchmarkError>;
pub fn quality_constrained_compression(pair: &PairedMeasurement) -> Result<f64, BenchmarkError>;
pub fn evaluate_pairs(pairs: &[PairedMeasurement], alpha: f64) -> Result<CompoundingReport, BenchmarkError>;
```

The benchmark must reject any pair where Run 2 lacks equal-or-better correctness or safety. `quality_constrained_compression` returns no improvement for a lower-token run that fails quality constraints. The report uses paired deltas, bootstrap confidence intervals, and a paired permutation test for H0; metric multiplicity uses Holm correction.

- [ ] **Step 1: Write failing metric tests**

Cover zero-duration rejection, no-knowledge baseline equivalence, lower tokens with lower quality rejection, lower HITL coordination with equal quality acceptance, and H0 non-rejection for unchanged paired runs.

- [ ] **Step 2: Run tests to verify failure**

```powershell
rustup run stable-x86_64-pc-windows-msvc cargo test --test evidence_contract benchmark
```

- [ ] **Step 3: Implement the measurement functions**

Use explicit units, paired observations, fixed alpha passed by the caller, deterministic bootstrap seeding derived from the benchmark fixture hash, and a report that includes raw paired values plus aggregate deltas.

- [ ] **Step 4: Run the metric suite**

```powershell
rustup run stable-x86_64-pc-windows-msvc cargo test --test evidence_contract benchmark -- --nocapture
```

- [ ] **Step 5: Commit**

```powershell
git add samwise-core/src/evidence/benchmark.rs samwise-core/src/evidence/mod.rs samwise-core/tests/evidence_contract.rs
git commit -m "test: measure compounding under quality constraints"
```

### Task 6: Self-review and Pass 0 gate

**Files:**
- Modify: `docs/superpowers/specs/2026-08-02-yantrikdb-metaclaw-closure-contract.md`
- Modify: `docs/superpowers/plans/2026-08-02-asset-contract-falsification.md`

- [ ] **Step 1: Run formatting, lint, and tests**

```powershell
rustup run stable-x86_64-pc-windows-msvc cargo fmt --all -- --check
rustup run stable-x86_64-pc-windows-msvc cargo check --all-targets --all-features
rustup run stable-x86_64-pc-windows-msvc cargo test --all-targets --all-features -- --nocapture
git diff --check
```

- [ ] **Step 2: Run the placeholder and contract scans**

```powershell
rg -n -i "TODO|TBD|FIXME|best guess|silent" docs/superpowers/specs docs/superpowers/plans samwise-core/src/evidence samwise-core/tests
```

Expected: no unresolved placeholders and no implementation path that silently converts incomplete evidence into a complete projection.

- [ ] **Step 3: Verify Pass 0 gate**

Pass 0 is complete only if:

```text
canonical equivalents share identity;
non-equivalents do not;
replay is deterministic;
event ordering is explicit;
partial evidence is explicit;
objective and subjective evaluation stay separate;
policy versions pin projections;
duplicate/crash/conflict/split/merge cases are executable;
H0 remains un-rejected for unchanged controls;
quality-constrained compounding metrics reject fake compression.
```

- [ ] **Step 4: Commit the gate evidence**

```powershell
git add docs/superpowers/specs/2026-08-02-yantrikdb-metaclaw-closure-contract.md docs/superpowers/plans/2026-08-02-asset-contract-falsification.md
git commit -m "docs: lock asset contract falsification gate"
```

## Spec coverage review

- Immutable typed assets: Tasks 2–3.
- Canonical identity and hash stability: Task 3.
- Append-only evidence/evaluations/relations: Tasks 2 and 4.
- Deterministic projection and policy pinning: Task 4.
- Commutativity and causal ordering: Task 4.
- Partial/contested state: Task 4.
- Objective/subjective separation: Tasks 2 and 4.
- Gradient = Compounding: Task 5.
- HITL coordination tax: Task 5.
- Quality-constrained compression: Task 5.
- H0 and paired benchmark: Task 5.
- Reproducible verification: Task 6.

This plan intentionally does not implement OmniRoute ingestion, YantrikDB adapters, MetaClaw procedure validation, or skill evolution. Those are downstream plans and cannot begin until Pass 0 is green.
