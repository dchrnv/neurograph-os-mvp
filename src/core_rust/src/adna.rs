//! ADNA v3.0 - Active DNA (Policy Engine)
//!
//! **Version:** 3.0.0
//! **Size:** 256 bytes core + variable policy storage
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

use std::time::{SystemTime, UNIX_EPOCH};

/// Magic number for ADNA structure validation: 'ADNA' in ASCII
pub const ADNA_MAGIC: u32 = 0x41444E41;

/// Current ADNA version
pub const ADNA_VERSION_MAJOR: u16 = 3;
pub const ADNA_VERSION_MINOR: u16 = 0;

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
        assert_eq!(adna.evolution.generation, 0);
        assert_eq!(adna.evolution.exploration_rate, 0.9);
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
        assert_eq!(adna.evolution.trajectory_count, 1);
        assert!(adna.evolution.success_rate > 0.0);

        adna.record_trajectory(false);
        assert_eq!(adna.evolution.trajectory_count, 2);
    }

    #[test]
    fn test_generation_increment() {
        let mut adna = ADNA::new(PolicyType::Linear);
        assert_eq!(adna.evolution.generation, 0);

        adna.increment_generation();
        assert_eq!(adna.evolution.generation, 1);
    }
}
