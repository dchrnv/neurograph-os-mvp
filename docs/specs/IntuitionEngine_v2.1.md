# IntuitionEngine v2.1 - Learning Loop Integration

**Version:** 2.1
**Language:** Rust
**Status:** Specification
**Based on:** docs/arch/3/IntuitionEngine.md v2.0

## Overview

`IntuitionEngine` is the analytical brain of the system. It runs in the background, analyzing accumulated experience from `ExperienceStream`, finding correlations between actions (defined by `ADNA`) and their outcomes (`reward` from 4 Appraisers), and generating concrete, testable `Proposal`s to improve `ADNA`.

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

### Output: Proposal

```rust
/// Proposal for changing ADNA policy
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Proposal {
    /// Unique proposal ID
    pub proposal_id: uuid::Uuid,

    /// Target entity ID (ADNA rule ID or policy component)
    pub target_entity_id: String,

    /// Proposed change (JSON Patch format)
    /// Example: {"op": "replace", "path": "/weight", "value": 0.85}
    pub proposed_change: serde_json::Value,

    /// Justification: data-driven reasoning
    pub justification: String,

    /// Expected impact on total reward
    pub expected_impact: f64,

    /// Confidence level [0.0, 1.0]
    pub confidence: f64,

    /// Timestamp when proposal was created
    pub created_at: std::time::SystemTime,
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

**Version History:**
- v2.1 (2025-01-13): Integration with ExperienceStream v2.1, ADNA v3.1, 4 Appraisers
- v2.0: Original specification (docs/arch/3/IntuitionEngine.md)