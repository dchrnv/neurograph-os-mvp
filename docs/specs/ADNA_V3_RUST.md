# ADNA v3.0 - Active DNA (Policy Engine) - Rust Specification

**Version:** 3.0.0
**Status:** ✅ IMPLEMENTED (v0.22.0)
**Language:** Rust 2021
**File:** `src/core_rust/src/adna.rs`
**Size:** 256 bytes (fixed, cache-aligned)
**Last Updated:** 2025-01-11

---

## Table of Contents

1. [Overview](#overview)
2. [Philosophy & Design Principles](#philosophy--design-principles)
3. [Memory Layout](#memory-layout)
4. [Structure Definitions](#structure-definitions)
5. [API Reference](#api-reference)
6. [Integration Points](#integration-points)
7. [Performance Characteristics](#performance-characteristics)
8. [Usage Examples](#usage-examples)
9. [Testing](#testing)
10. [Migration from v1.0](#migration-from-v10)
11. [Future Roadmap](#future-roadmap)

---

## Overview

ADNA v3.0 represents a **fundamental redefinition** of ADNA from an adaptive parameter cache to a **Policy Engine** - a dynamic decision-making system that maps environmental states to actions. This is the "learned knowledge" layer of NeuroGraph OS, continuously evolving within boundaries set by CDNA.

### Key Concepts

- **Policy-as-First-Class-Entity**: ADNA is fundamentally a `State → Action` function
- **Versioned Evolution**: Each ADNA state has lineage tracking and fitness metrics
- **Gradient-Based Updates**: Changes driven by Intuition Engine analyzing experience trajectories
- **CDNA Constraint Satisfaction**: All mutations validated against constitutional rules
- **Asynchronous Learning**: Policy updates happen in dedicated learning phases

### Role in Cognitive Hierarchy

```
CDNA (Constitutional) → validates → ADNA (Policy) → generates → Actions
                                         ↑
                                    Intuition
                                   (analyzes)
                                         ↑
                                    Experience
```

---

## Philosophy & Design Principles

### 1. Machine-Friendly Architecture

All structures use power-of-2 sizes (32, 64, 128, 256 bytes) for optimal CPU cache performance:
- **64-byte blocks**: Fit exactly one cache line (typical L1 cache line size)
- **256-byte total**: Fits in 4 cache lines, minimizing cache misses
- **Alignment**: `#[repr(C, align(64))]` ensures proper alignment
- **Packing**: `#[repr(C, packed)]` for sub-structures to eliminate padding

### 2. Separation of Concerns

ADNA core (256 bytes) contains only **metadata and pointers**. Actual policy data (weights, neural networks, decision trees) stored separately. This allows:
- Fast ADNA swapping during evolution
- Efficient version control (small core structure)
- Variable-sized policies without core structure changes
- Cache-friendly metadata access

### 3. Lineage Tracking

Every ADNA state tracks its ancestry via **SHA256 parent hash**, enabling:
- Evolutionary tree reconstruction
- Rollback to previous stable versions
- A/B testing of policy variants
- Credit assignment across generations

### 4. Multi-Modal Policy Support

ADNA supports multiple policy types through `PolicyType` enum:
- **Linear**: Simple weight matrix (fast, interpretable)
- **Neural**: Neural network policies (expressive, differentiable)
- **TreeBased**: Decision trees (interpretable, discrete)
- **Hybrid**: Combination of multiple approaches
- **Programmatic**: Compiled rule-based policies

---

## Memory Layout

### Overall Structure (256 bytes)

```
┌─────────────────────────────────────────────────────────┐
│ ADNA Structure (256 bytes, align(64))                   │
├─────────────────────────────────────────────────────────┤
│ Offset 0-63:   ADNAHeader        (64 bytes)             │
│ Offset 64-127: EvolutionMetrics  (64 bytes, packed)     │
│ Offset 128-191: PolicyPointer    (64 bytes, packed)     │
│ Offset 192-255: StateMapping     (64 bytes)             │
└─────────────────────────────────────────────────────────┘
```

### Block 1: ADNAHeader (64 bytes)

```
┌─────────────────────────────────────────────────────────┐
│ ADNAHeader (64 bytes, repr(C))                          │
├──────────────────────┬──────────────────────────────────┤
│ Offset 0-3    (4B)   │ magic: u32                       │
│ Offset 4-5    (2B)   │ version_major: u16               │
│ Offset 6-7    (2B)   │ version_minor: u16               │
│ Offset 8-9    (2B)   │ policy_type: u16                 │
│ Offset 10-31  (22B)  │ _reserved1: [u8; 22]             │
│ Offset 32-63  (32B)  │ parent_hash: [u8; 32]            │
└──────────────────────┴──────────────────────────────────┘
```

**Fields:**
- `magic`: Magic number `0x41444E41` ('ADNA' in ASCII) for validation
- `version_major/minor`: ADNA version (currently 3.0)
- `policy_type`: Enum discriminator for policy type (0=Linear, 1=Neural, etc.)
- `_reserved1`: Reserved for future use (e.g., flags, creation timestamp)
- `parent_hash`: SHA256 hash of parent ADNA for lineage tracking

### Block 2: EvolutionMetrics (64 bytes, packed)

```
┌─────────────────────────────────────────────────────────┐
│ EvolutionMetrics (64 bytes, repr(C, packed))            │
├──────────────────────┬──────────────────────────────────┤
│ Offset 0-3    (4B)   │ generation: u32                  │
│ Offset 4-7    (4B)   │ fitness_score: f32               │
│ Offset 8-11   (4B)   │ confidence: f32                  │
│ Offset 12-15  (4B)   │ exploration_rate: f32            │
│ Offset 16-19  (4B)   │ learning_rate: f32               │
│ Offset 20-23  (4B)   │ trajectory_count: u32            │
│ Offset 24-27  (4B)   │ success_rate: f32                │
│ Offset 28-35  (8B)   │ last_update: u64                 │
│ Offset 36-39  (4B)   │ update_frequency: u32            │
│ Offset 40-63  (24B)  │ _reserved: [u8; 24]              │
└──────────────────────┴──────────────────────────────────┘
```

**Fields:**
- `generation`: Increments on each policy update (lineage depth)
- `fitness_score`: Overall performance metric (0.0-1.0, higher is better)
- `confidence`: System's confidence in current policy (0.0-1.0)
- `exploration_rate`: ε-greedy exploration parameter (0.0-1.0)
- `learning_rate`: Step size for gradient updates
- `trajectory_count`: Total number of experience trajectories collected
- `success_rate`: Moving average of successful trajectories (0.0-1.0)
- `last_update`: Unix timestamp of last policy update
- `update_frequency`: Updates per hour (for monitoring update velocity)

### Block 3: PolicyPointer (64 bytes, packed)

```
┌─────────────────────────────────────────────────────────┐
│ PolicyPointer (64 bytes, repr(C, packed))               │
├──────────────────────┬──────────────────────────────────┤
│ Offset 0-3    (4B)   │ policy_size: u32                 │
│ Offset 4-11   (8B)   │ policy_offset: u64               │
│ Offset 12     (1B)   │ compression_type: u8             │
│ Offset 13     (1B)   │ encryption_flag: u8              │
│ Offset 14     (1B)   │ cache_strategy: u8               │
│ Offset 15-63  (49B)  │ _reserved: [u8; 49]              │
└──────────────────────┴──────────────────────────────────┘
```

**Fields:**
- `policy_size`: Size of policy data in bytes
- `policy_offset`: Memory/disk offset to policy storage
- `compression_type`: 0=none, 1=LZ4, 2=zstd (for cold storage)
- `encryption_flag`: 0=none, 1=encrypted (for secure policies)
- `cache_strategy`: 0=always, 1=lazy, 2=periodic (cache control)

### Block 4: StateMapping (64 bytes)

```
┌─────────────────────────────────────────────────────────┐
│ StateMapping (64 bytes, repr(C))                        │
├──────────────────────┬──────────────────────────────────┤
│ Offset 0-1    (2B)   │ input_dimensions: u16            │
│ Offset 2-3    (2B)   │ output_dimensions: u16           │
│ Offset 4-19   (16B)  │ state_normalization: [f32; 4]    │
│ Offset 20-35  (16B)  │ action_bounds: [f32; 4]          │
│ Offset 36-63  (28B)  │ _reserved: [u8; 28]              │
└──────────────────────┴──────────────────────────────────┘
```

**Fields:**
- `input_dimensions`: State space dimensionality (default: 8 for L1-L8)
- `output_dimensions`: Action space dimensionality (default: 8)
- `state_normalization`: `[mean, std, min, max]` for state preprocessing
- `action_bounds`: `[min_x, max_x, min_y, max_y]` for action clipping

---

## Structure Definitions

### Complete ADNA Structure

```rust
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct ADNA {
    pub header: ADNAHeader,             // 64 bytes (offset 0-63)
    pub evolution: EvolutionMetrics,    // 64 bytes (offset 64-127)
    pub policy_ptr: PolicyPointer,      // 64 bytes (offset 128-191)
    pub state_mapping: StateMapping,    // 64 bytes (offset 192-255)
}

// Compile-time assertion
const _: () = assert!(std::mem::size_of::<ADNA>() == 256);
```

### PolicyType Enumeration

```rust
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyType {
    Linear = 0,        // Weight matrix: action = W * state + b
    Neural = 1,        // Neural network (MLP, Transformer, etc.)
    TreeBased = 2,     // Decision tree or random forest
    Hybrid = 3,        // Ensemble of multiple policies
    Programmatic = 4,  // Compiled rule-based policy
}
```

### Magic Numbers

```rust
pub const ADNA_MAGIC: u32 = 0x41444E41; // 'ADNA' in ASCII
pub const ADNA_VERSION_MAJOR: u16 = 3;
pub const ADNA_VERSION_MINOR: u16 = 0;
```

---

## API Reference

### Core Methods

#### `ADNA::new(policy_type: PolicyType) -> Self`

Creates new ADNA instance with default parameters.

**Example:**
```rust
let adna = ADNA::new(PolicyType::Linear);
assert!(adna.is_valid());
assert_eq!(adna.evolution.generation, 0);
assert_eq!(adna.evolution.exploration_rate, 0.9); // High initial exploration
```

**Initialization:**
- Generation: 0
- Fitness: 0.0
- Confidence: 0.5
- Exploration rate: 0.9 (90% exploration initially)
- Learning rate: 0.01
- Timestamp: current Unix time

#### `adna.is_valid() -> bool`

Validates ADNA structure integrity.

**Checks:**
- Magic number matches `ADNA_MAGIC`
- Version matches current implementation

**Example:**
```rust
if !adna.is_valid() {
    return Err(ADNAError::InvalidStructure);
}
```

#### `adna.policy_type() -> PolicyType`

Returns the policy type.

**Example:**
```rust
match adna.policy_type() {
    PolicyType::Linear => run_linear_policy(&adna),
    PolicyType::Neural => run_neural_policy(&adna),
    _ => unimplemented!(),
}
```

### Evolution Tracking

#### `adna.update_fitness(new_fitness: f32)`

Updates fitness score with clamping to [0.0, 1.0].

**Example:**
```rust
// After evaluating policy performance
let avg_reward = evaluate_policy(&adna, &test_episodes);
adna.update_fitness(avg_reward.clamp(0.0, 1.0));
```

**Side effects:**
- Updates `last_update` timestamp
- Clamps fitness to valid range

#### `adna.increment_generation()`

Increments generation counter (use when creating descendant ADNA).

**Example:**
```rust
let mut child_adna = parent_adna.clone();
child_adna.increment_generation();
child_adna.header.parent_hash = compute_hash(&parent_adna);
```

#### `adna.record_trajectory(success: bool)`

Records trajectory outcome and updates success rate via exponential moving average.

**Example:**
```rust
for episode in episodes {
    let success = episode.total_reward > threshold;
    adna.record_trajectory(success);
}

println!("Success rate: {:.2}%", adna.evolution.success_rate * 100.0);
```

**Formula:**
```
new_rate = α * (1 if success else 0) + (1 - α) * old_rate
where α = 0.1
```

---

## Integration Points

### With CDNA (Constitutional DNA)

ADNA operates **within constraints** defined by CDNA:

```rust
// Before applying policy update
if !cdna.validate_policy_update(&gradient, &adna) {
    return Err(PolicyError::ConstraintViolation);
}

// CDNA defines boundaries
if action_magnitude > cdna.max_action_magnitude {
    action = clip_action(action, cdna.action_bounds);
}
```

**Key interactions:**
- CDNA provides `action_bounds` → ADNA uses in `StateMapping`
- CDNA validates all policy mutations
- CDNA can trigger emergency policy rollback

### With ExperienceToken

Experience tokens provide training data for policy learning:

```rust
// ExperienceToken → ADNA gradient calculation
pub struct ExperienceToken {
    pub state: [f32; 8],        // Input to policy
    pub action: [f32; 8],       // Action taken
    pub reward: f32,            // Reward received
    pub next_state: [f32; 6],   // Resulting state
    pub adna_version_hash: [u8; 4], // Which ADNA generated action
}

// Policy uses this to compute gradients
let gradient = policy.get_gradient(&experience_token);
```

### With Intuition Engine

Intuition Engine analyzes batches of experience to generate policy updates:

```rust
// Intuition analyzes experience batch
let experiences = experience_stream.sample_batch(1000, PrioritizedSampling);

// Generates gradient
let gradient = intuition_engine.analyze_trajectories(&experiences, &adna);

// Proposes update
let proposal = Proposal {
    gradient,
    confidence: 0.85,
    expected_improvement: 0.05,
    risk_score: 0.1,
};

// Evolution Manager decides whether to apply
if evolution_manager.should_apply(&proposal, &adna, &cdna) {
    policy.apply_gradient(&gradient, adna.evolution.learning_rate)?;
    adna.increment_generation();
}
```

### With Policy Trait

ADNA metadata guides policy execution:

```rust
pub trait Policy {
    fn map_state(&self, state: &[f32; 8]) -> [f32; 8];
    fn get_gradient(&self, experience: &ExperienceToken) -> Gradient;
    fn apply_gradient(&mut self, gradient: &Gradient, lr: f32) -> Result<()>;
}

// Usage
let state = get_current_state(); // [f32; 8]
let action = policy.map_state(&state);

// Apply action bounds from ADNA
let bounded_action = clip_action(action, &adna.state_mapping.action_bounds);
```

---

## Performance Characteristics

### Memory

| Metric | Value | Notes |
|--------|-------|-------|
| ADNA core size | 256 bytes | Fixed, cache-aligned |
| Policy storage | 1KB - 10MB | Variable, depends on type |
| Cache lines used | 4 | Optimal for L1 cache |
| Alignment | 64 bytes | Matches typical cache line |

### Time Complexity

| Operation | Complexity | Typical Time |
|-----------|-----------|--------------|
| `new()` | O(1) | ~50ns |
| `is_valid()` | O(1) | ~5ns |
| `update_fitness()` | O(1) | ~10ns |
| `increment_generation()` | O(1) | ~5ns |
| `record_trajectory()` | O(1) | ~15ns |

### Benchmark Results (Rust 1.91.1, x86_64)

```
ADNA::new()              ... 48.2ns
ADNA::is_valid()         ... 4.8ns
ADNA::update_fitness()   ... 12.1ns
ADNA::increment_gen()    ... 5.3ns
ADNA::record_trajectory()... 16.7ns
```

---

## Usage Examples

### Example 1: Basic ADNA Creation and Validation

```rust
use neurograph_core::{ADNA, PolicyType};

fn main() {
    // Create new ADNA with linear policy
    let adna = ADNA::new(PolicyType::Linear);

    // Validate structure
    assert!(adna.is_valid());
    assert_eq!(adna.policy_type(), PolicyType::Linear);

    // Check initial state
    assert_eq!(adna.evolution.generation, 0);
    assert_eq!(adna.evolution.fitness_score, 0.0);
    assert_eq!(adna.evolution.confidence, 0.5);
    assert_eq!(adna.evolution.exploration_rate, 0.9);

    println!("ADNA initialized successfully");
}
```

### Example 2: Policy Evaluation Loop

```rust
use neurograph_core::{ADNA, PolicyType, LinearPolicy, Policy};

fn evaluate_policy(adna: &mut ADNA, policy: &LinearPolicy, episodes: usize) {
    let mut total_reward = 0.0;

    for episode in 0..episodes {
        let mut state = initialize_environment();
        let mut episode_reward = 0.0;

        for step in 0..100 {
            // Use policy to select action
            let action = policy.map_state(&state);

            // Apply action bounds from ADNA
            let bounded_action = clip_action(
                action,
                &adna.state_mapping.action_bounds
            );

            // Execute action
            let (next_state, reward, done) = environment_step(bounded_action);

            episode_reward += reward;
            state = next_state;

            if done { break; }
        }

        // Record trajectory outcome
        let success = episode_reward > 0.0;
        adna.record_trajectory(success);
        total_reward += episode_reward;
    }

    // Update fitness
    let avg_reward = total_reward / episodes as f32;
    adna.update_fitness(avg_reward.clamp(0.0, 1.0));

    println!("Fitness: {:.3}, Success rate: {:.2}%",
        adna.evolution.fitness_score,
        adna.evolution.success_rate * 100.0
    );
}
```

### Example 3: Policy Evolution

```rust
use neurograph_core::{ADNA, PolicyType, LinearPolicy, Gradient};

fn evolve_policy(
    parent_adna: &ADNA,
    policy: &mut LinearPolicy,
    gradient: &Gradient
) -> Result<ADNA, PolicyError> {
    // Create child ADNA
    let mut child_adna = parent_adna.clone();

    // Apply gradient update
    policy.apply_gradient(gradient, parent_adna.evolution.learning_rate)?;

    // Update evolution metrics
    child_adna.increment_generation();
    child_adna.header.parent_hash = compute_hash(parent_adna);

    // Decrease exploration over time
    child_adna.evolution.exploration_rate *= 0.99;

    Ok(child_adna)
}
```

### Example 4: A/B Testing Policies

```rust
fn ab_test_policies(
    policy_a: &LinearPolicy,
    policy_b: &LinearPolicy,
    adna_a: &mut ADNA,
    adna_b: &mut ADNA,
    episodes: usize
) -> PolicyType {
    // Evaluate both policies
    let score_a = run_evaluation(policy_a, adna_a, episodes);
    let score_b = run_evaluation(policy_b, adna_b, episodes);

    println!("Policy A: {:.3} (gen {})", score_a, adna_a.evolution.generation);
    println!("Policy B: {:.3} (gen {})", score_b, adna_b.evolution.generation);

    // Return winner
    if score_a > score_b {
        println!("Policy A wins!");
        PolicyType::Linear
    } else {
        println!("Policy B wins!");
        PolicyType::Neural
    }
}
```

---

## Testing

### Unit Tests

All tests located in `src/core_rust/src/adna.rs`:

```rust
#[test]
fn test_adna_size() {
    assert_eq!(std::mem::size_of::<ADNA>(), 256);
    assert_eq!(std::mem::size_of::<ADNAHeader>(), 64);
    assert_eq!(std::mem::size_of::<EvolutionMetrics>(), 64);
    assert_eq!(std::mem::size_of::<PolicyPointer>(), 64);
    assert_eq!(std::mem::size_of::<StateMapping>(), 64);
}

#[test]
fn test_adna_creation() {
    let adna = ADNA::new(PolicyType::Linear);
    assert!(adna.is_valid());
    assert_eq!(adna.policy_type(), PolicyType::Linear);
}

#[test]
fn test_adna_fitness_update() {
    let mut adna = ADNA::new(PolicyType::Linear);
    adna.update_fitness(0.75);
    assert_eq!(adna.evolution.fitness_score, 0.75);

    // Test clamping
    adna.update_fitness(1.5);
    assert_eq!(adna.evolution.fitness_score, 1.0);
}
```

### Running Tests

```bash
cd src/core_rust
cargo test adna::tests
```

---

## Migration from v1.0

ADNA v1.0 → v3.0 is a **breaking change**. Key differences:

### ADNA v1.0 (Deprecated)
- **Size**: 256 bytes
- **Purpose**: Static policy parameters
- **Structure**: ADNAHeader + ADNAParameters
- **Updates**: Manual configuration files
- **Policy**: Stored in parameters directly

### ADNA v3.0 (Current)
- **Size**: 256 bytes (same)
- **Purpose**: Dynamic policy engine
- **Structure**: Header + Metrics + Pointer + Mapping (4x64)
- **Updates**: Gradient-based learning
- **Policy**: Separate storage, pointer-based

### Migration Steps

1. **Backup old ADNA configs**
2. **Convert parameters to LinearPolicy**:
   ```rust
   // Old: parameters in ADNA
   let old_weights = adna_v1.parameters.weights;

   // New: separate policy object
   let mut policy = LinearPolicy::new();
   for i in 0..8 {
       for j in 0..8 {
           policy.set_weight(i, j, old_weights[i][j]);
       }
   }
   ```

3. **Initialize ADNA v3.0**:
   ```rust
   let mut adna = ADNA::new(PolicyType::Linear);
   adna.evolution.fitness_score = old_fitness;
   ```

4. **Set up policy storage**:
   ```rust
   let policy_bytes = policy.serialize();
   adna.policy_ptr.policy_size = policy_bytes.len() as u32;
   adna.policy_ptr.policy_offset = write_to_storage(&policy_bytes);
   ```

---

## Future Roadmap

### v3.1 (Planned)
- [ ] Add `current_hash` calculation (SHA256 of entire ADNA)
- [ ] Implement policy versioning storage
- [ ] Add compression support for large policies

### v3.2 (Planned)
- [ ] Neural policy support (PyTorch/ONNX integration)
- [ ] Multi-objective fitness (Pareto frontier)
- [ ] Automatic learning rate scheduling

### v4.0 (Vision)
- [ ] Self-modifying architecture (ADNA evolves its own structure)
- [ ] Meta-learning across environments
- [ ] Quantum-inspired policy representations

---

## References

- [ADNA v3.0 Full Specification](/home/chrnv/neurograph-os-mvp/docs/arch/ADNA v3.0.md)
- [ExperienceToken Specification](ExperienceStream_v2.0.md)
- [Policy Trait Implementation](../src/core_rust/src/policy.rs)
- [Integration Examples](../examples/)

---

**Document Version:** 1.0
**Author:** NeuroGraph OS Team
**Last Review:** 2025-01-11
**Status:** Active
