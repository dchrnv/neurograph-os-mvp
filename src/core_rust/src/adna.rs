//! ADNA v3.1 - Active DNA (Policy Engine + Appraiser Configuration)
//!
//! **Version:** 3.1.0
//! **Size:** 256 bytes core + variable policy storage + appraiser config
//!
//! ADNA is the Policy Engine - a dynamic decision-making system that maps
//! environmental states to actions. This represents the "learned knowledge" layer
//! of NeuroGraph OS, continuously evolving within boundaries set by CDNA.
//!
//! # Core Principles
//!
//! - **Policy as first-class entity**: ADNA is fundamentally a State → Action function
//! - **Versioned evolution**: Each ADNA state has lineage tracking and fitness metrics
//! - **Gradient-based updates**: Changes driven by Intuition Engine analyzing experience
//! - **CDNA constraint satisfaction**: All mutations validated against constitutional rules
//! - **Asynchronous learning**: Policy updates happen in dedicated learning phases
//! - **Appraiser configuration**: Parameters for all 4 reward appraisers (v3.1+)

use std::time::{SystemTime, UNIX_EPOCH};

/// Magic number for ADNA structure validation: 'ADNA' in ASCII
pub const ADNA_MAGIC: u32 = 0x41444E41;

/// Current ADNA version
pub const ADNA_VERSION_MAJOR: u16 = 3;
pub const ADNA_VERSION_MINOR: u16 = 1;

// ============================================================================
// Core ADNA Structure (256 bytes total)
// ============================================================================

/// Complete ADNA structure (256 bytes, cache-aligned)
///
/// This is the Policy Engine core that maps states to actions.
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct ADNA {
    pub header: ADNAHeader,             // 64 bytes (offset 0-63)
    pub evolution: EvolutionMetrics,    // 64 bytes (offset 64-127)
    pub policy_ptr: PolicyPointer,      // 64 bytes (offset 128-191)
    pub state_mapping: StateMapping,    // 64 bytes (offset 192-255)
}

// Compile-time assertion: ADNA must be exactly 256 bytes
const _: () = assert!(std::mem::size_of::<ADNA>() == 256);

// ============================================================================
// Header Block (64 bytes)
// ============================================================================

/// ADNA header containing version and lineage information
/// Exactly 64 bytes
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ADNAHeader {
    /// Magic number 'ADNA' (0x41444E41) for validation
    pub magic: u32,                     // 4 bytes

    /// Version (major.minor)
    pub version_major: u16,             // 2 bytes
    pub version_minor: u16,             // 2 bytes

    /// Policy type: Linear, Neural, TreeBased, Hybrid
    pub policy_type: u16,               // 2 bytes

    /// Reserved for alignment
    pub _reserved1: [u8; 22],           // 22 bytes (total so far: 32)

    /// SHA256 hash of parent ADNA version (for lineage tracking)
    pub parent_hash: [u8; 32],          // 32 bytes (total: 64)
}

/// Policy type enumeration
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyType {
    /// Linear policy: weight matrix for state→action mapping
    Linear = 0,
    /// Neural network policy
    Neural = 1,
    /// Decision tree policy
    TreeBased = 2,
    /// Hybrid combination of multiple approaches
    Hybrid = 3,
    /// Compiled rule-based policy
    Programmatic = 4,
}

impl From<u16> for PolicyType {
    fn from(value: u16) -> Self {
        match value {
            0 => PolicyType::Linear,
            1 => PolicyType::Neural,
            2 => PolicyType::TreeBased,
            3 => PolicyType::Hybrid,
            4 => PolicyType::Programmatic,
            _ => PolicyType::Linear, // Default fallback
        }
    }
}

// ============================================================================
// Evolution Metrics Block (64 bytes)
// ============================================================================

/// Metrics tracking policy evolution and performance
/// Exactly 64 bytes
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct EvolutionMetrics {
    /// Generation number (increments on each update)
    pub generation: u32,                // 4 bytes

    /// Fitness score (0.0 - 1.0, higher is better)
    pub fitness_score: f32,             // 4 bytes

    /// Confidence in current configuration (0.0 - 1.0)
    pub confidence: f32,                // 4 bytes

    /// Exploration rate (0.0 - 1.0, balance explore/exploit)
    pub exploration_rate: f32,          // 4 bytes

    /// Learning rate for gradient updates
    pub learning_rate: f32,             // 4 bytes

    /// Number of trajectories experienced
    pub trajectory_count: u32,          // 4 bytes

    /// Success rate (0.0 - 1.0)
    pub success_rate: f32,              // 4 bytes

    /// Last update timestamp (Unix epoch seconds)
    pub last_update: u64,               // 8 bytes (total: 36)

    /// Update frequency (updates per hour)
    pub update_frequency: u32,          // 4 bytes (total: 40)

    /// Reserved for future use
    pub _reserved: [u8; 24],            // 24 bytes (total: 64)
}

// ============================================================================
// Policy Pointer Block (64 bytes)
// ============================================================================

/// Pointer to policy storage (separate from core structure)
/// Exactly 64 bytes
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct PolicyPointer {
    /// Size of policy data in bytes
    pub policy_size: u32,               // 4 bytes

    /// Policy offset in memory/disk
    pub policy_offset: u64,             // 8 bytes

    /// Compression type (0 = none, 1 = LZ4, etc.)
    pub compression_type: u8,           // 1 byte

    /// Encryption flag (0 = none, 1 = encrypted)
    pub encryption_flag: u8,            // 1 byte

    /// Cache strategy (0 = always, 1 = lazy, 2 = periodic)
    pub cache_strategy: u8,             // 1 byte (total: 15)

    /// Reserved for future use
    pub _reserved: [u8; 49],            // 49 bytes (total: 64)
}

// ============================================================================
// State Mapping Block (64 bytes)
// ============================================================================

/// Configuration for state-action space mapping
/// Exactly 64 bytes
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct StateMapping {
    /// Input dimensions (8D semantic space compressed)
    pub input_dimensions: u16,          // 2 bytes

    /// Output dimensions (action space)
    pub output_dimensions: u16,         // 2 bytes

    /// State normalization parameters [mean, std, min, max]
    pub state_normalization: [f32; 4],  // 16 bytes (total: 20)

    /// Action bounds [min_x, max_x, min_y, max_y]
    pub action_bounds: [f32; 4],        // 16 bytes (total: 36)

    /// Reserved for future use
    pub _reserved: [u8; 28],            // 28 bytes (total: 64)
}

// ============================================================================
// ADNA Implementation
// ============================================================================

impl ADNA {
    /// Create new ADNA with default parameters
    pub fn new(policy_type: PolicyType) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            header: ADNAHeader {
                magic: ADNA_MAGIC,
                version_major: ADNA_VERSION_MAJOR,
                version_minor: ADNA_VERSION_MINOR,
                policy_type: policy_type as u16,
                _reserved1: [0; 22],
                parent_hash: [0; 32],
            },
            evolution: EvolutionMetrics {
                generation: 0,
                fitness_score: 0.0,
                confidence: 0.5,
                exploration_rate: 0.9, // High exploration initially
                learning_rate: 0.01,
                trajectory_count: 0,
                success_rate: 0.0,
                last_update: now,
                update_frequency: 0,
                _reserved: [0; 24],
            },
            policy_ptr: PolicyPointer {
                policy_size: 0,
                policy_offset: 0,
                compression_type: 0,
                encryption_flag: 0,
                cache_strategy: 0,
                _reserved: [0; 49],
            },
            state_mapping: StateMapping {
                input_dimensions: 8, // 8D semantic space
                output_dimensions: 8, // 8D action space
                state_normalization: [0.0, 1.0, -1.0, 1.0], // [mean, std, min, max]
                action_bounds: [-1.0, 1.0, -1.0, 1.0],
                _reserved: [0; 28],
            },
        }
    }

    /// Validate ADNA structure
    pub fn is_valid(&self) -> bool {
        self.header.magic == ADNA_MAGIC
            && self.header.version_major == ADNA_VERSION_MAJOR
    }

    /// Get policy type
    pub fn policy_type(&self) -> PolicyType {
        PolicyType::from(self.header.policy_type)
    }

    /// Update fitness score
    pub fn update_fitness(&mut self, new_fitness: f32) {
        self.evolution.fitness_score = new_fitness.clamp(0.0, 1.0);
        self.evolution.last_update = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Increment generation
    pub fn increment_generation(&mut self) {
        self.evolution.generation += 1;
    }

    /// Record trajectory
    pub fn record_trajectory(&mut self, success: bool) {
        self.evolution.trajectory_count += 1;

        // Update success rate (exponential moving average)
        let alpha = 0.1;
        let new_value = if success { 1.0 } else { 0.0 };
        self.evolution.success_rate =
            alpha * new_value + (1.0 - alpha) * self.evolution.success_rate;
    }
}

impl Default for ADNA {
    fn default() -> Self {
        Self::new(PolicyType::Linear)
    }
}

// ============================================================================
// Appraiser Configuration (v3.1+)
// ============================================================================

/// Parameters for HomeostasisAppraiser
///
/// Controls penalties for deviations from target ranges in L1-L8 coordinates.
#[derive(Debug, Clone, Copy)]
pub struct HomeostasisParams {
    /// Overall weight/importance of homeostasis rewards
    pub weight: f32,

    /// Target range for L5 Cognitive Load [min, max]
    pub cognitive_load_range: (f32, f32),

    /// Target range for L6 Certainty [min, max]
    pub certainty_range: (f32, f32),

    /// Target range for L8 Coherence [min, max]
    pub coherence_range: (f32, f32),

    /// Multiplier for penalty calculation (default: 1.0)
    pub penalty_multiplier: f32,
}

impl Default for HomeostasisParams {
    fn default() -> Self {
        Self {
            weight: 0.3,
            cognitive_load_range: (0.2, 0.7),
            certainty_range: (0.4, 0.9),
            coherence_range: (0.5, 1.0),
            penalty_multiplier: 1.0,
        }
    }
}

/// Parameters for CuriosityAppraiser
///
/// Controls rewards for novelty and exploration.
#[derive(Debug, Clone, Copy)]
pub struct CuriosityParams {
    /// Overall weight/importance of curiosity rewards
    pub weight: f32,

    /// Minimum L2 Novelty to trigger reward (threshold)
    pub novelty_threshold: f32,

    /// Reward multiplier for novelty above threshold
    pub reward_multiplier: f32,

    /// Decay factor for repeated exposure (0.0 - 1.0)
    pub habituation_rate: f32,
}

impl Default for CuriosityParams {
    fn default() -> Self {
        Self {
            weight: 0.2,
            novelty_threshold: 0.3,
            reward_multiplier: 1.0,
            habituation_rate: 0.95,
        }
    }
}

/// Parameters for EfficiencyAppraiser
///
/// Controls penalties for resource usage.
#[derive(Debug, Clone, Copy)]
pub struct EfficiencyParams {
    /// Overall weight/importance of efficiency penalties
    pub weight: f32,

    /// Cost factor for L3 Motor activity (velocity/acceleration)
    pub motor_cost_factor: f32,

    /// Cost factor for L5 Cognitive Load
    pub cognitive_cost_factor: f32,

    /// Cost factor for creating new tokens/connections
    pub creation_cost_factor: f32,
}

impl Default for EfficiencyParams {
    fn default() -> Self {
        Self {
            weight: 0.1,
            motor_cost_factor: 0.01,
            cognitive_cost_factor: 0.02,
            creation_cost_factor: 0.05,
        }
    }
}

/// Parameters for GoalDirectedAppraiser
///
/// Controls retroactive reward distribution for goal achievement.
#[derive(Debug, Clone, Copy)]
pub struct GoalDirectedParams {
    /// Overall weight/importance of goal-directed rewards
    pub weight: f32,

    /// Temporal discount factor (0.0 - 1.0, higher = more patient)
    pub gamma: f32,

    /// Minimum trajectory length to apply discounting
    pub min_trajectory_length: u32,
}

impl Default for GoalDirectedParams {
    fn default() -> Self {
        Self {
            weight: 0.4,
            gamma: 0.99,
            min_trajectory_length: 2,
        }
    }
}

/// Complete appraiser configuration
///
/// This structure holds all parameters for the 4 reward appraisers.
#[derive(Debug, Clone, Copy)]
pub struct AppraiserConfig {
    pub homeostasis: HomeostasisParams,
    pub curiosity: CuriosityParams,
    pub efficiency: EfficiencyParams,
    pub goal_directed: GoalDirectedParams,
}

impl Default for AppraiserConfig {
    fn default() -> Self {
        Self {
            homeostasis: HomeostasisParams::default(),
            curiosity: CuriosityParams::default(),
            efficiency: EfficiencyParams::default(),
            goal_directed: GoalDirectedParams::default(),
        }
    }
}

// ============================================================================
// ADNAReader Trait
// ============================================================================

/// Trait for reading ADNA configuration
///
/// This provides async access to appraiser parameters. Implementations can
/// store config in memory, on disk, or load from remote sources.
#[async_trait::async_trait]
pub trait ADNAReader: Send + Sync {
    /// Get homeostasis appraiser parameters
    async fn get_homeostasis_params(&self) -> Result<HomeostasisParams, ADNAError>;

    /// Get curiosity appraiser parameters
    async fn get_curiosity_params(&self) -> Result<CuriosityParams, ADNAError>;

    /// Get efficiency appraiser parameters
    async fn get_efficiency_params(&self) -> Result<EfficiencyParams, ADNAError>;

    /// Get goal-directed appraiser parameters
    async fn get_goal_directed_params(&self) -> Result<GoalDirectedParams, ADNAError>;

    /// Get complete appraiser configuration
    async fn get_appraiser_config(&self) -> Result<AppraiserConfig, ADNAError>;

    /// Get action policy for a given state
    ///
    /// Returns the ActionPolicy that should be used for action selection
    /// in the given L1-L8 coordinate state.
    async fn get_action_policy(&self, state: &[i16; 8]) -> Result<ActionPolicy, ADNAError>;
}

/// Error type for ADNA operations
#[derive(Debug, thiserror::Error)]
pub enum ADNAError {
    #[error("ADNA not found")]
    NotFound,

    #[error("Invalid ADNA structure")]
    InvalidStructure,

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

// ============================================================================
// In-Memory ADNAReader Implementation
// ============================================================================

/// Simple in-memory implementation of ADNAReader
///
/// Stores appraiser config in memory with atomic updates support.
pub struct InMemoryADNAReader {
    config: std::sync::Arc<tokio::sync::RwLock<AppraiserConfig>>,
    /// Action policies indexed by state_bin_id
    /// This is filled by EvolutionManager when proposals are applied
    policies: std::sync::Arc<tokio::sync::RwLock<std::collections::HashMap<String, ActionPolicy>>>,
}

impl InMemoryADNAReader {
    pub fn new(config: AppraiserConfig) -> Self {
        Self {
            config: std::sync::Arc::new(tokio::sync::RwLock::new(config)),
            policies: std::sync::Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(AppraiserConfig::default())
    }

    /// Set action policy for a state (used by EvolutionManager)
    pub async fn set_action_policy(&self, state_bin_id: String, policy: ActionPolicy) {
        let mut policies = self.policies.write().await;
        policies.insert(state_bin_id, policy);
    }

    /// Get action policy by state_bin_id (internal helper)
    pub async fn get_policy_by_bin(&self, state_bin_id: &str) -> Option<ActionPolicy> {
        let policies = self.policies.read().await;
        policies.get(state_bin_id).cloned()
    }

    /// Update appraiser configuration
    pub async fn update_config(&self, config: AppraiserConfig) {
        let mut lock = self.config.write().await;
        *lock = config;
    }

    /// Update specific appraiser parameters
    pub async fn update_homeostasis(&self, params: HomeostasisParams) {
        let mut lock = self.config.write().await;
        lock.homeostasis = params;
    }

    pub async fn update_curiosity(&self, params: CuriosityParams) {
        let mut lock = self.config.write().await;
        lock.curiosity = params;
    }

    pub async fn update_efficiency(&self, params: EfficiencyParams) {
        let mut lock = self.config.write().await;
        lock.efficiency = params;
    }

    pub async fn update_goal_directed(&self, params: GoalDirectedParams) {
        let mut lock = self.config.write().await;
        lock.goal_directed = params;
    }
}

#[async_trait::async_trait]
impl ADNAReader for InMemoryADNAReader {
    async fn get_homeostasis_params(&self) -> Result<HomeostasisParams, ADNAError> {
        let config = self.config.read().await;
        Ok(config.homeostasis)
    }

    async fn get_curiosity_params(&self) -> Result<CuriosityParams, ADNAError> {
        let config = self.config.read().await;
        Ok(config.curiosity)
    }

    async fn get_efficiency_params(&self) -> Result<EfficiencyParams, ADNAError> {
        let config = self.config.read().await;
        Ok(config.efficiency)
    }

    async fn get_goal_directed_params(&self) -> Result<GoalDirectedParams, ADNAError> {
        let config = self.config.read().await;
        Ok(config.goal_directed)
    }

    async fn get_appraiser_config(&self) -> Result<AppraiserConfig, ADNAError> {
        let config = self.config.read().await;
        Ok(*config)
    }

    async fn get_action_policy(&self, state: &[i16; 8]) -> Result<ActionPolicy, ADNAError> {
        // Quantize state to bin_id (same logic as IntuitionEngine)
        let state_bin_id = quantize_state_to_bin(state, 4); // 4 bins per dimension

        // Look up policy for this state bin
        let policies = self.policies.read().await;
        if let Some(policy) = policies.get(&state_bin_id) {
            Ok(policy.clone())
        } else {
            // Return default policy: uniform weights across common action types
            let mut default_policy = ActionPolicy::new(format!("default_{}", state_bin_id));
            // Common action types with equal weights
            default_policy.set_weight(1, 1.0); // action type 1
            default_policy.set_weight(2, 1.0); // action type 2
            default_policy.set_weight(3, 1.0); // action type 3
            Ok(default_policy)
        }
    }
}

/// Quantize 8D state into a string bin ID
///
/// This uses the same quantization logic as IntuitionEngine for consistency.
fn quantize_state_to_bin(state: &[i16; 8], bins_per_dim: u32) -> String {
    let mut bin_id: u64 = 0;
    let bins_per_dim_u64 = bins_per_dim as u64;

    for &value in state.iter() {
        // Convert i16 to normalized f32 [-1.0, 1.0] → [0.0, 1.0]
        let normalized = ((value as f32 / 32767.0) + 1.0) / 2.0;
        let clamped = normalized.clamp(0.0, 0.999);
        let bin = (clamped * bins_per_dim as f32) as u64;
        bin_id = bin_id * bins_per_dim_u64 + bin;
    }

    format!("adna_state_bin_{}", bin_id)
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

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
        // Copy packed fields to avoid alignment issues
        let generation = adna.evolution.generation;
        let exploration_rate = adna.evolution.exploration_rate;
        assert_eq!(generation, 0);
        assert_eq!(exploration_rate, 0.9);
    }

    #[test]
    fn test_adna_fitness_update() {
        let mut adna = ADNA::new(PolicyType::Linear);
        adna.update_fitness(0.75);
        assert_eq!(adna.evolution.fitness_score, 0.75);

        // Test clamping
        adna.update_fitness(1.5);
        assert_eq!(adna.evolution.fitness_score, 1.0);

        adna.update_fitness(-0.5);
        assert_eq!(adna.evolution.fitness_score, 0.0);
    }

    #[test]
    fn test_trajectory_recording() {
        let mut adna = ADNA::new(PolicyType::Linear);

        adna.record_trajectory(true);
        // Copy packed fields to avoid alignment issues
        let count = adna.evolution.trajectory_count;
        let rate = adna.evolution.success_rate;
        assert_eq!(count, 1);
        assert!(rate > 0.0);

        adna.record_trajectory(false);
        let count2 = adna.evolution.trajectory_count;
        assert_eq!(count2, 2);
    }

    #[test]
    fn test_generation_increment() {
        let mut adna = ADNA::new(PolicyType::Linear);
        // Copy packed field to avoid alignment issues
        let gen = adna.evolution.generation;
        assert_eq!(gen, 0);

        adna.increment_generation();
        let gen2 = adna.evolution.generation;
        assert_eq!(gen2, 1);
    }

    #[test]
    fn test_appraiser_config_defaults() {
        let config = AppraiserConfig::default();

        assert_eq!(config.homeostasis.weight, 0.3);
        assert_eq!(config.curiosity.weight, 0.2);
        assert_eq!(config.efficiency.weight, 0.1);
        assert_eq!(config.goal_directed.weight, 0.4);

        // Sum of weights should be 1.0
        let total_weight = config.homeostasis.weight
            + config.curiosity.weight
            + config.efficiency.weight
            + config.goal_directed.weight;
        assert!((total_weight - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_homeostasis_params() {
        let params = HomeostasisParams::default();
        assert_eq!(params.cognitive_load_range, (0.2, 0.7));
        assert_eq!(params.certainty_range, (0.4, 0.9));
        assert_eq!(params.coherence_range, (0.5, 1.0));
        assert_eq!(params.penalty_multiplier, 1.0);
    }

    #[test]
    fn test_curiosity_params() {
        let params = CuriosityParams::default();
        assert_eq!(params.novelty_threshold, 0.3);
        assert_eq!(params.reward_multiplier, 1.0);
        assert_eq!(params.habituation_rate, 0.95);
    }

    #[test]
    fn test_efficiency_params() {
        let params = EfficiencyParams::default();
        assert_eq!(params.motor_cost_factor, 0.01);
        assert_eq!(params.cognitive_cost_factor, 0.02);
        assert_eq!(params.creation_cost_factor, 0.05);
    }

    #[test]
    fn test_goal_directed_params() {
        let params = GoalDirectedParams::default();
        assert_eq!(params.gamma, 0.99);
        assert_eq!(params.min_trajectory_length, 2);
    }

    #[tokio::test]
    async fn test_in_memory_adna_reader() {
        let reader = InMemoryADNAReader::with_defaults();

        // Test reading default config
        let config = reader.get_appraiser_config().await.unwrap();
        assert_eq!(config.homeostasis.weight, 0.3);

        // Test reading individual params
        let homeostasis = reader.get_homeostasis_params().await.unwrap();
        assert_eq!(homeostasis.weight, 0.3);

        let curiosity = reader.get_curiosity_params().await.unwrap();
        assert_eq!(curiosity.weight, 0.2);

        let efficiency = reader.get_efficiency_params().await.unwrap();
        assert_eq!(efficiency.weight, 0.1);

        let goal_directed = reader.get_goal_directed_params().await.unwrap();
        assert_eq!(goal_directed.weight, 0.4);
    }

    #[tokio::test]
    async fn test_in_memory_adna_reader_update() {
        let reader = InMemoryADNAReader::with_defaults();

        // Update homeostasis params
        let new_homeostasis = HomeostasisParams {
            weight: 0.5,
            ..Default::default()
        };
        reader.update_homeostasis(new_homeostasis).await;

        // Verify update
        let params = reader.get_homeostasis_params().await.unwrap();
        assert_eq!(params.weight, 0.5);

        // Other params should remain unchanged
        let curiosity = reader.get_curiosity_params().await.unwrap();
        assert_eq!(curiosity.weight, 0.2);
    }
}

// ============================================================================
// Learning Loop Structures (IntuitionEngine + EvolutionManager)
// ============================================================================

use std::collections::HashMap;

/// Proposal for changing ADNA policy
///
/// Generated by IntuitionEngine based on experience analysis.
/// Validated and applied by EvolutionManager.
#[derive(Debug, Clone)]
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
    pub created_at: SystemTime,
}

impl Proposal {
    pub fn new(
        target_entity_id: String,
        proposed_change: serde_json::Value,
        justification: String,
        expected_impact: f64,
        confidence: f64,
    ) -> Self {
        Self {
            proposal_id: uuid::Uuid::new_v4(),
            target_entity_id,
            proposed_change,
            justification,
            expected_impact,
            confidence,
            created_at: SystemTime::now(),
        }
    }
}

/// Intent - abstract high-level description of what the system wants to do
///
/// Used by ActionController to select appropriate executor and parameters.
#[derive(Debug, Clone)]
pub struct Intent {
    /// Intent type (e.g., "generate_code", "answer_question")
    pub intent_type: String,

    /// Context information for decision making (JSON)
    pub context: serde_json::Value,

    /// Current state (L1-L8 coordinates)
    pub state: [i16; 8],
}

impl Intent {
    pub fn new(intent_type: impl Into<String>, context: serde_json::Value, state: [i16; 8]) -> Self {
        Self {
            intent_type: intent_type.into(),
            context,
            state,
        }
    }
}

/// Action selection policy from ADNA
///
/// Maps actions to weights/probabilities for a given state.
#[derive(Debug, Clone)]
pub struct ActionPolicy {
    /// Map of action_type (event_type) → weight/probability
    pub action_weights: HashMap<u16, f64>,

    /// Rule ID in ADNA
    pub rule_id: String,

    /// When this policy was last updated
    pub last_updated: SystemTime,

    /// Policy metadata (optional parameters for executor)
    pub metadata: serde_json::Value,
}

impl ActionPolicy {
    pub fn new(rule_id: impl Into<String>) -> Self {
        Self {
            action_weights: HashMap::new(),
            rule_id: rule_id.into(),
            last_updated: SystemTime::now(),
            metadata: serde_json::Value::Null,
        }
    }

    /// Add or update action weight
    pub fn set_weight(&mut self, action_type: u16, weight: f64) {
        self.action_weights.insert(action_type, weight);
    }

    /// Get weight for action type (returns 0.0 if not found)
    pub fn get_weight(&self, action_type: u16) -> f64 {
        self.action_weights.get(&action_type).copied().unwrap_or(0.0)
    }

    /// Sample action based on weights (returns highest weight action)
    pub fn select_action(&self) -> Option<u16> {
        self.action_weights
            .iter()
            .max_by(|(_, w1), (_, w2)| w1.partial_cmp(w2).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(action, _)| *action)
    }
}

#[cfg(test)]
mod learning_tests {
    use super::*;

    #[test]
    fn test_proposal_creation() {
        let proposal = Proposal::new(
            "adna_rule_state_5".to_string(),
            serde_json::json!({"op": "replace", "path": "/weight", "value": 0.85}),
            "Statistical analysis shows improvement".to_string(),
            0.15,
            0.82,
        );

        assert_eq!(proposal.target_entity_id, "adna_rule_state_5");
        assert_eq!(proposal.expected_impact, 0.15);
        assert_eq!(proposal.confidence, 0.82);
        assert!(proposal.proposal_id.to_string().len() > 0);
    }

    #[test]
    fn test_intent_creation() {
        let intent = Intent::new(
            "generate_code",
            serde_json::json!({"language": "rust"}),
            [100, 200, 50, 300, 150, 400, 250, 350]
        );

        assert_eq!(intent.intent_type, "generate_code");
        assert_eq!(intent.state[0], 100);
    }

    #[test]
    fn test_action_policy() {
        let mut policy = ActionPolicy::new("test_rule");
        policy.set_weight(1, 0.8);
        policy.set_weight(2, 0.2);

        assert_eq!(policy.get_weight(1), 0.8);
        assert_eq!(policy.get_weight(2), 0.2);
        assert_eq!(policy.get_weight(999), 0.0);

        // Should select action 1 (highest weight)
        assert_eq!(policy.select_action(), Some(1));
    }
}
