# IntuitionEngine v2.2 - Hybrid Learning (ADNA + Connections)

**Version:** 2.2 (was 2.1 Statistical, now 2.2 Hybrid)
**Language:** Rust
**Status:** 🚧 In Development
**Previous Version:** v2.1 (ADNA-only learning)
**Based on:** Architectural pivot 2025-11-17

## Philosophical Pivot: Why Hybrid?

### The Original Plan (v2.1 → v2.0 Neural)

The original roadmap proposed:
- v2.1: Statistical learning (t-tests, state binning)
- v2.0: Neural learning (gradient-based, deep patterns)

**Problem identified**: This focused exclusively on **ADNA** (behavioral policies), ignoring **Connection** learning (causal models).

### The Hybrid Realization (v2.2)

After reflection on Connection vs IntuitionEngine overlap:
- Connections encode **causal knowledge** ("Action X causes State Y")
- IntuitionEngine discovers **what works** ("Action X succeeds 75% of time")
- **These should inform each other!**

**Hybrid Model**:
1. **ADNA Learning** - behavioral strategies (which appraisers to weight, exploration rate)
2. **Connection Learning** - causal models (which actions cause which states, with what confidence)

Both use the **same statistical foundation** but update different structures.

### Why Not Neural (Yet)?

The hybrid model postpones neural approach for pragmatic reasons:
- **Data efficiency**: Statistical works with less data
- **Interpretability**: Can explain why proposals were made
- **Incremental**: Can add Connection learning without rewriting everything
- **Future path**: Can still do Neural v2.3 later, now with both ADNA and Connection targets

## Overview

`IntuitionEngine` is the analytical brain of the system. It runs in the background, analyzing accumulated experience from `ExperienceStream`, finding correlations between actions and outcomes, and generating concrete `Proposal`s to improve **both**:
1. **ADNA** - behavioral policies (weights, thresholds, exploration)
2. **Connections** - causal models (confidence, strength, existence)

## Key Functions

1. **Credit Assignment Analysis**: Answers "Why did we succeed or fail?"
   - Searches for behavior patterns (sequences of states and actions) that consistently lead to high or low rewards
   - Uses batch analysis of ExperienceEvents with appraiser rewards

2. **Policy Improvement Synthesis**: Answers "How can we act better in the future?"
   - Formulates hypotheses as concrete Proposals to change ADNA
   - Example: "In states similar to X, action Y is preferable to Z. Propose increasing weight of rule for action Y"

## Architecture Principles

- **Offline Learning**: Main analysis on batches of accumulated experience (not real-time)
- **Hypothesis-Driven**: Output is structured `Proposal`, not direct ADNA changes
- **Reward-Focused**: Focuses on trajectories with anomalously high/low total rewards
- **Async Tokio**: Background service with periodic analysis cycles

## Data Structures

### Input: ExperienceEvent (from ExperienceStream v2.1)

```rust
#[repr(C, packed)]
pub struct ExperienceEvent {
    // Header (32 bytes)
    event_type: u16,           // Event type ID
    flags: u8,                 // EventFlags
    _reserved1: u8,
    timestamp_ms: u64,         // Milliseconds since epoch
    sequence_number: u64,      // Monotonic sequence
    source_token_id: u64,      // Source token
    target_token_id: u64,      // Target token (0 if N/A)

    // L1-L8 Coordinates (64 bytes)
    coordinates: [i16; 8],     // L1-L8 values [-32768, 32767]
    _reserved_coords: [u8; 48],

    // Appraiser Rewards (16 bytes)
    reward_homeostasis: f32,
    reward_curiosity: f32,
    reward_efficiency: f32,
    reward_goal: f32,

    // Extension (16 bytes)
    _reserved_ext: [u8; 16],
}
```

### Output: Proposal (Expanded for Hybrid Learning)

```rust
/// Proposal for changing system state (ADNA or Connections)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Proposal {
    /// Modify existing ADNA policy
    ModifyADNA {
        proposal_id: uuid::Uuid,
        target_policy_id: String,
        proposed_change: serde_json::Value,  // JSON Patch format
        justification: String,
        expected_impact: f64,
        confidence: f64,
        created_at: std::time::SystemTime,
    },

    /// Modify existing Connection (confidence, strength, etc.)
    ModifyConnection {
        proposal_id: uuid::Uuid,
        connection_id: u64,
        field: ConnectionField,
        old_value: f32,
        new_value: f32,
        justification: String,
        evidence: Vec<u64>,  // ExperienceEvent IDs
        expected_impact: f64,
        confidence: f64,
        created_at: std::time::SystemTime,
    },

    /// Create new hypothesis Connection
    CreateConnection {
        proposal_id: uuid::Uuid,
        token_a_id: u32,
        token_b_id: u32,
        connection_type: ConnectionType,
        initial_strength: f32,
        initial_confidence: f32,
        justification: String,
        evidence: Vec<u64>,
        expected_impact: f64,
        confidence: f64,
        created_at: std::time::SystemTime,
    },

    /// Delete hypothesis Connection
    DeleteConnection {
        proposal_id: uuid::Uuid,
        connection_id: u64,
        reason: String,
        evidence: Vec<u64>,
        created_at: std::time::SystemTime,
    },

    /// Promote Hypothesis → Learnable
    PromoteConnection {
        proposal_id: uuid::Uuid,
        connection_id: u64,
        evidence_count: u16,
        justification: String,
        created_at: std::time::SystemTime,
    },
}

/// Fields modifiable in Connection
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum ConnectionField {
    Confidence,
    PullStrength,
    PreferredDistance,
    LearningRate,
    DecayRate,
}
```

### Configuration

```rust
#[derive(Debug, Clone, serde::Deserialize)]
pub struct IntuitionConfig {
    /// Analysis cycle interval (seconds)
    pub analysis_interval_secs: u64,

    /// Batch size for analysis
    pub batch_size: usize,

    /// Sampling strategy for selecting "interesting" experience
    pub sampling_strategy: SamplingStrategy,

    /// Minimum confidence threshold for sending proposals
    pub min_confidence: f64,

    /// Maximum proposals per cycle
    pub max_proposals_per_cycle: usize,
}

#[derive(Debug, Clone, serde::Deserialize)]
pub enum SamplingStrategy {
    /// Random uniform sampling
    Uniform,

    /// Prioritized by absolute total reward
    PrioritizedByReward {
        /// Probability exponent (higher = more biased toward high |reward|)
        alpha: f64,
    },

    /// Recency-weighted (prefer recent experience)
    RecencyWeighted {
        /// Decay factor
        decay: f64,
    },

    /// Mixed strategy
    Mixed {
        reward_weight: f64,
        recency_weight: f64,
    },
}
```

## Core API

```rust
pub struct IntuitionEngine {
    config: IntuitionConfig,
    dna_reader: Arc<dyn DNAReader>,
    experience_reader: Arc<dyn ExperienceReader>,
    proposal_sender: tokio::sync::mpsc::Sender<Proposal>,
}

impl IntuitionEngine {
    pub fn new(
        config: IntuitionConfig,
        dna_reader: Arc<dyn DNAReader>,
        experience_reader: Arc<dyn ExperienceReader>,
        proposal_sender: tokio::sync::mpsc::Sender<Proposal>,
    ) -> Self {
        Self { config, dna_reader, experience_reader, proposal_sender }
    }

    /// Run main analysis loop (async background task)
    pub async fn run(self) {
        let mut interval = tokio::time::interval(
            tokio::time::Duration::from_secs(self.config.analysis_interval_secs)
        );

        loop {
            interval.tick().await;

            if let Err(e) = self.run_analysis_cycle().await {
                eprintln!("IntuitionEngine analysis error: {}", e);
            }
        }
    }

    /// Single analysis cycle: sample → analyze → propose
    async fn run_analysis_cycle(&self) -> Result<(), IntuitionError> {
        // 1. Sample "interesting" batch using prioritized sampling
        let batch = self.experience_reader
            .sample_batch(self.config.batch_size, self.config.sampling_strategy.clone())
            .await?;

        // 2. Analyze batch to find patterns
        let patterns = self.find_patterns_in_batch(&batch).await?;

        // 3. Generate proposals from patterns
        let proposals = self.generate_proposals_from_patterns(patterns).await?;

        // 4. Send proposals to EvolutionManager
        for proposal in proposals {
            if proposal.confidence >= self.config.min_confidence {
                self.proposal_sender.send(proposal).await?;
            }
        }

        Ok(())
    }

    /// Core ML logic: find patterns in batch
    async fn find_patterns_in_batch(
        &self,
        batch: &ExperienceBatch
    ) -> Result<Vec<IdentifiedPattern>, IntuitionError>;

    /// Convert patterns to concrete proposals
    async fn generate_proposals_from_patterns(
        &self,
        patterns: Vec<IdentifiedPattern>
    ) -> Result<Vec<Proposal>, IntuitionError>;
}
```

## Pattern Analysis (v1.0 - Statistical)

### Phase 1: State Space Quantization

- **Problem**: Continuous L1-L8 coordinate space
- **Solution**: Cluster states using k-means or grid quantization
- **Output**: Discrete "context clusters" or "state bins"

### Phase 2: Action-Reward Aggregation

For each state cluster:
1. Group events by action type (`event_type`)
2. Calculate statistics:
   - Mean total reward per action: `mean(r_homeostasis + r_curiosity + r_efficiency + r_goal)`
   - Standard deviation
   - Sample count

### Phase 3: Pattern Identification

Find clusters where:
- **Significant difference** between mean rewards for different actions
- **Statistical significance** (e.g., t-test, p < 0.05)
- **Sufficient samples** (e.g., n ≥ 10 per action type)

**IdentifiedPattern**:
```rust
struct IdentifiedPattern {
    state_cluster_id: usize,
    better_action: u16,        // event_type with higher reward
    worse_action: u16,         // event_type with lower reward
    reward_delta: f64,         // Difference in mean rewards
    confidence: f64,           // Statistical confidence
    sample_count: usize,
}
```

### Phase 4: Proposal Generation

For each pattern:
```rust
Proposal {
    target_entity_id: format!("adna_rule_state_{}", pattern.state_cluster_id),
    proposed_change: json!({
        "op": "replace",
        "path": "/action_weights",
        "value": {
            pattern.better_action.to_string(): 0.8,
            pattern.worse_action.to_string(): 0.2,
        }
    }),
    justification: format!(
        "Analysis of {} samples: action {} outperforms {} by {:.2} reward in state cluster {}",
        pattern.sample_count,
        pattern.better_action,
        pattern.worse_action,
        pattern.reward_delta,
        pattern.state_cluster_id
    ),
    expected_impact: pattern.reward_delta,
    confidence: pattern.confidence,
}
```

## Integration with ExperienceStream v2.1

### Required ExperienceReader Extensions

```rust
#[async_trait]
pub trait ExperienceReader: Send + Sync {
    // ... existing methods ...

    /// Sample batch using specified strategy
    async fn sample_batch(
        &self,
        size: usize,
        strategy: SamplingStrategy
    ) -> Result<ExperienceBatch, StreamError>;

    /// Get total event count (for sampling probabilities)
    async fn total_events(&self) -> usize;
}

pub struct ExperienceBatch {
    pub events: Vec<ExperienceEvent>,
    pub sampled_at: std::time::SystemTime,
}
```

## Integration with ADNA v3.1

### Required DNAReader Extensions

```rust
#[async_trait]
pub trait DNAReader: Send + Sync {
    // ... existing appraiser param methods ...

    /// Get action selection policy for given state
    async fn get_action_policy_for_state(
        &self,
        state: &[i16; 8]  // L1-L8 coordinates
    ) -> Result<ActionPolicy, ADNAError>;
}

pub struct ActionPolicy {
    /// Map of action_type → weight/probability
    pub action_weights: HashMap<u16, f64>,

    /// Metadata
    pub rule_id: String,
    pub last_updated: std::time::SystemTime,
}
```

## Error Handling

```rust
#[derive(Debug, thiserror::Error)]
pub enum IntuitionError {
    #[error("Experience sampling failed: {0}")]
    SamplingError(String),

    #[error("Pattern analysis failed: {0}")]
    AnalysisError(String),

    #[error("Proposal generation failed: {0}")]
    ProposalError(String),

    #[error("Channel send error: {0}")]
    ChannelError(#[from] tokio::sync::mpsc::error::SendError<Proposal>),

    #[error("ADNA read error: {0}")]
    ADNAError(#[from] crate::adna::ADNAError),
}
```

## Learning Loop Closure

```
┌─────────────────────────────────────────────────────────────┐
│                    FULL LEARNING LOOP                       │
└─────────────────────────────────────────────────────────────┘

1. ACTION (ActionController)
   ├─ Reads ActionPolicy from ADNA
   ├─ Selects action based on current state
   └─ Writes ExperienceEvent to ExperienceStream
          ↓
2. APPRAISAL (4 Appraisers)
   ├─ Homeostasis: penalizes L5/L6/L8 deviations
   ├─ Curiosity: rewards L2 Novelty
   ├─ Efficiency: penalizes L3/L5 resource use
   └─ Goal: rewards L7 Valence (proxy for goal achievement)
          ↓ (writes rewards to event)

3. ANALYSIS (IntuitionEngine) ← WE ARE HERE
   ├─ Samples batch with high |total_reward|
   ├─ Clusters states (L1-L8)
   ├─ Finds action-reward correlations
   └─ Generates Proposals
          ↓
4. VALIDATION (EvolutionManager) ← NEXT PHASE
   ├─ Validates Proposal against CDNA rules
   ├─ Applies if accepted (atomic ADNA update)
   ├─ Logs outcome to ExperienceStream
   └─ Feeds back to IntuitionEngine (meta-learning)
          ↓
5. NEW ACTION (loop back to step 1)
   └─ Uses updated ADNA policy
```

## Future Enhancements (v2.0+)

### World Model Learning
- Train neural network to predict `(next_state, reward)` from `(state, action)`
- Use model gradients to generate proposals
- Enables "counterfactual reasoning" ("what if we did X instead?")

### Multi-Step Credit Assignment
- Track full trajectories (sequences of events)
- Compute discounted cumulative reward
- Find patterns in action sequences, not just single actions

### Meta-Learning
- Analyze `intuition_proposal_accepted/rejected` events
- Learn which types of proposals are more likely to be accepted
- Improve proposal confidence calibration

## Testing Strategy

### Unit Tests
- [ ] State clustering (k-means convergence)
- [ ] Statistical significance calculations
- [ ] Proposal generation from patterns

### Integration Tests
- [ ] End-to-end: ExperienceStream → sample_batch → analyze → Proposals
- [ ] Different sampling strategies produce valid batches
- [ ] Proposals conform to expected JSON Patch format

### Performance Tests
- [ ] Analysis cycle completes within interval (even with large batches)
- [ ] Memory usage bounded (no accumulation over cycles)

## Dependencies

```toml
[dependencies]
tokio = { version = "1.42", features = ["sync", "time", "macros"] }
async-trait = "0.1"
thiserror = "1.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
```

## File Structure

```
src/core_rust/src/
├── intuition_engine.rs     # Main IntuitionEngine impl
├── pattern_analysis.rs     # Statistical analysis logic
└── bin/
    └── learning-loop-demo.rs  # Full cycle demo
```

---

## Implementation Plan (v0.29.0)

### Phase 1: Connection v2.0 Foundation
1. ✅ Spec completed - 64-byte structure with learning fields
2. ⏳ Implement `Connection` struct in Rust
3. ⏳ Add `ConnectionMutability` enum
4. ⏳ Update Grid/Graph to handle 64-byte connections
5. ⏳ Write unit tests for new fields

### Phase 2: Proposal System Extension
1. ⏳ Implement `Proposal` enum (ModifyADNA/ModifyConnection/CreateConnection/etc.)
2. ⏳ Implement `ConnectionField` enum
3. ⏳ Update `IntuitionEngine` to generate Connection proposals
4. ⏳ Add causal pattern discovery logic

### Phase 3: Guardian Integration
1. ⏳ Extend Guardian to validate Connection proposals
2. ⏳ Add mutability checks (reject Immutable modifications)
3. ⏳ Add confidence delta validation
4. ⏳ Add evidence requirement checks

### Phase 4: Learning Algorithms
1. ⏳ Implement confidence update formula (Bayesian or frequency-based)
2. ⏳ Implement hypothesis decay mechanism
3. ⏳ Implement promotion logic (Hypothesis → Learnable)
4. ⏳ Implement deletion logic (low confidence threshold)

### Phase 5: Integration & Testing
1. ⏳ E2E test: Create hypothesis, accumulate evidence, promote
2. ⏳ E2E test: Contradictory evidence → confidence drop → deletion
3. ⏳ E2E test: Immutable connection protection
4. ⏳ Benchmark: performance impact of 64-byte connections
5. ⏳ Update documentation and examples

### Estimated Scope
- **Connection v2.0**: ~200 LOC (struct, impl, tests)
- **Proposal extension**: ~150 LOC (enum variants, serialization)
- **IntuitionEngine updates**: ~300 LOC (causal discovery, proposal generation)
- **Guardian updates**: ~200 LOC (validation logic)
- **Tests**: ~400 LOC (unit + integration)
- **Total**: ~1250 LOC

### Success Criteria
- [ ] Connection can be created with all three mutability levels
- [ ] IntuitionEngine generates Connection proposals from experience data
- [ ] Guardian correctly validates/rejects proposals based on mutability
- [ ] Hypothesis connections decay and promote as expected
- [ ] All existing tests pass with 64-byte connections
- [ ] Performance overhead < 10% compared to V1.0

---

**Version History:**
- v2.2 (2025-11-17): Hybrid learning - ADNA + Connection learning, architectural pivot
- v2.1 (2025-01-13): Integration with ExperienceStream v2.1, ADNA v3.1, 4 Appraisers
- v2.0: Original specification (docs/arch/3/IntuitionEngine.md)