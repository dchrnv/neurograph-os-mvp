//! ADNA v1.0 MVP - Adaptive DNA (Static Policy Edition)
//!
//! **Version:** 1.0.0 (MVP)
//! **Size:** 256 bytes (fixed, cache-aligned)
//! **Purpose:** Static policy parameters for system behavior
//!
//! ADNA is the "genetic code" of the system, controlling:
//! - Appraiser weights (Homeostasis, Curiosity, Efficiency, GoalDirected)
//! - System behavior parameters (exploration rate, timeouts, limits)
//! - Version history and evolution tracking
//!
//! Unlike ADNA v3.0 (full RL policy engine), this MVP version stores
//! policies as static configurations (JSON/TOML) that can be manually
//! updated through Guardian API with full CDNA validation.

use std::time::{SystemTime, UNIX_EPOCH};

/// Magic number for ADNA structure validation: 'ADNA' in ASCII
pub const ADNA_MAGIC: u32 = 0x41444E41;

/// Current ADNA version
pub const ADNA_VERSION_MAJOR: u16 = 1;
pub const ADNA_VERSION_MINOR: u16 = 0;

// ============================================================================
// Core ADNA Structure (256 bytes total)
// ============================================================================

/// Complete ADNA structure (256 bytes, cache-aligned)
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct ADNA {
    pub header: ADNAHeader,         // 64 bytes (offset 0-63)
    pub metrics: EvolutionMetrics,  // 64 bytes (offset 64-127)
    pub pointer: PolicyPointer,     // 64 bytes (offset 128-191)
    pub parameters: ADNAParameters, // 64 bytes (offset 192-255)
}

// ============================================================================
// Header Block (64 bytes)
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ADNAHeader {
    /// Magic number 'ADNA' (0x41444E41) for validation
    pub magic: u32,

    /// Version (major.minor)
    pub version_major: u16,
    pub version_minor: u16,

    /// Policy type: 0 = StaticRules, 1-255 reserved for future
    pub policy_type: u16,

    /// Reserved for alignment
    pub _reserved1: u16,

    /// Creation timestamp (Unix epoch seconds)
    pub created_at: u64,

    /// Last modification timestamp
    pub modified_at: u64,

    /// SHA256 hash of parent ADNA version (for lineage tracking)
    pub parent_hash: [u8; 32],
}

/// Policy type enumeration
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyType {
    /// Static rules from JSON/TOML configuration
    StaticRules = 0,
    // Reserved for future:
    // Neural = 1,
    // TreeBased = 2,
    // Hybrid = 3,
}

// ============================================================================
// Evolution Metrics Block (64 bytes)
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct EvolutionMetrics {
    /// Generation number (increments on each update)
    pub generation: u32,

    /// Manual quality score (0.0 - 1.0)
    pub fitness_score: f32,

    /// Confidence in current configuration (0.0 - 1.0)
    pub confidence: f32,

    /// Reserved for future learning rate
    pub learning_rate: f32,

    /// Number of times this ADNA was activated
    pub activation_count: u32,

    /// Reserved for future success tracking
    pub success_rate: f32,

    /// Reserved for rollback tracking
    pub rollback_count: u32,

    /// Reserved for future use (total: 4+4+4+4+4+4+4 = 28, need 36 more for 64)
    pub _reserved: [u8; 36],
}

// ============================================================================
// Policy Pointer Block (64 bytes)
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct PolicyPointer {
    /// File path hash (FNV-1a for identification)
    pub policy_path_hash: u64,

    /// Checksum of policy file (FNV-1a)
    pub policy_checksum: u64,

    /// Size of external policy file (bytes)
    pub policy_size: u32,

    /// Compression type (0 = none, 1 = LZ4, 2 = Zstd)
    pub compression_type: u8,

    /// Encryption flag (0 = none, 1 = AES-256)
    pub encryption_flag: u8,

    /// Cache strategy (0 = always, 1 = on-demand)
    pub cache_strategy: u8,

    /// Reserved
    pub _reserved1: u8,

    /// Reserved for future (8+8+4+1+1+1+1 = 24, need 40 more for 64)
    pub _reserved2: [u8; 40],
}

// ============================================================================
// Parameters Block (64 bytes)
// ============================================================================

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct ADNAParameters {
    // === Appraiser Weights (16 bytes) ===
    /// Weight for HomeostasisAppraiser (0.0 - 1.0)
    pub homeostasis_weight: f32,

    /// Weight for CuriosityAppraiser (0.0 - 1.0)
    pub curiosity_weight: f32,

    /// Weight for EfficiencyAppraiser (0.0 - 1.0)
    pub efficiency_weight: f32,

    /// Weight for GoalDirectedAppraiser (0.0 - 1.0)
    pub goal_weight: f32,

    // === System Behavior (20 bytes) ===
    /// Exploration rate (0.0 = exploit only, 1.0 = explore only)
    pub exploration_rate: f32,

    /// Learning rate for Hebbian learning (0.001 - 0.1)
    pub learning_rate: f32,

    /// Decision timeout (milliseconds)
    pub decision_timeout_ms: u32,

    /// Max actions per cycle
    pub max_actions_per_cycle: u32,

    // === Reserved for future (32 bytes) ===
    pub _reserved2: [u8; 32],
}

// ============================================================================
// Profile Presets
// ============================================================================

/// Predefined ADNA profiles for common use cases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ADNAProfile {
    /// Balanced weights, moderate exploration
    Balanced,
    /// High homeostasis, low exploration, safety-first
    Cautious,
    /// High curiosity, high exploration, discovery-focused
    Curious,
    /// Moderate all weights, adaptive to environment
    Adaptive,
}

// ============================================================================
// Implementation
// ============================================================================

impl ADNA {
    /// Create new ADNA with default parameters
    pub fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            header: ADNAHeader {
                magic: ADNA_MAGIC,
                version_major: ADNA_VERSION_MAJOR,
                version_minor: ADNA_VERSION_MINOR,
                policy_type: PolicyType::StaticRules as u16,
                _reserved1: 0,
                created_at: now,
                modified_at: now,
                parent_hash: [0; 32],
            },
            metrics: EvolutionMetrics::default(),
            pointer: PolicyPointer::default(),
            parameters: ADNAParameters::default(),
        }
    }

    /// Create ADNA from a predefined profile
    pub fn from_profile(profile: ADNAProfile) -> Self {
        let mut adna = Self::new();
        adna.parameters = ADNAParameters::from_profile(profile);
        adna.update_hash();
        adna
    }

    /// Validate ADNA structure (magic number, version, etc.)
    pub fn validate(&self) -> Result<(), ADNAError> {
        if self.header.magic != ADNA_MAGIC {
            return Err(ADNAError::InvalidMagic(self.header.magic));
        }

        if self.header.version_major != ADNA_VERSION_MAJOR {
            return Err(ADNAError::IncompatibleVersion {
                found: (self.header.version_major, self.header.version_minor),
                expected: (ADNA_VERSION_MAJOR, ADNA_VERSION_MINOR),
            });
        }

        // Validate parameter ranges
        self.parameters.validate()?;

        Ok(())
    }

    /// Update modification timestamp
    pub fn update_hash(&mut self) {
        // Update parent hash with current parameters hash (FNV-1a)
        let hash = self.compute_fnv1a_hash();
        self.header.parent_hash[0..8].copy_from_slice(&hash.to_le_bytes());
        self.header.modified_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Compute FNV-1a hash of parameters
    fn compute_fnv1a_hash(&self) -> u64 {
        const FNV_OFFSET: u64 = 14695981039346656037;
        const FNV_PRIME: u64 = 1099511628211;

        let mut hash = FNV_OFFSET;

        // Hash parameters block
        let params_bytes = unsafe {
            std::slice::from_raw_parts(
                &self.parameters as *const ADNAParameters as *const u8,
                std::mem::size_of::<ADNAParameters>(),
            )
        };

        for &byte in params_bytes {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
        }

        hash
    }

    /// Increment generation counter
    pub fn increment_generation(&mut self) {
        self.metrics.generation += 1;
        self.update_hash();
    }

    /// Record activation
    pub fn record_activation(&mut self) {
        self.metrics.activation_count += 1;
    }

    /// Create evolved version (new generation) based on current ADNA
    ///
    /// Creates a copy with:
    /// - Generation counter incremented
    /// - Parent hash set to current parameters hash
    /// - Modified timestamp updated
    ///
    /// # Returns
    /// New ADNA instance with incremented generation
    ///
    /// # Example
    /// ```rust
    /// use neurograph_core::{ADNA, ADNAProfile};
    ///
    /// let adna1 = ADNA::from_profile(ADNAProfile::Balanced);
    /// let mut adna2 = adna1.evolve();
    ///
    /// // Modify parameters
    /// adna2.parameters.exploration_rate = 0.5;
    /// adna2.update_hash();
    ///
    /// assert_eq!(adna2.metrics.generation, adna1.metrics.generation + 1);
    /// ```
    pub fn evolve(&self) -> Self {
        let mut new_adna = *self;

        // Store current parameters hash in parent_hash for lineage tracking
        let current_hash = self.compute_fnv1a_hash();
        new_adna.header.parent_hash[0..8].copy_from_slice(&current_hash.to_le_bytes());

        // Increment generation
        new_adna.metrics.generation += 1;

        // Update modification timestamp
        new_adna.header.modified_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        new_adna
    }

    /// Get total size in bytes
    pub const fn size_bytes() -> usize {
        std::mem::size_of::<ADNA>()
    }
}

impl Default for ADNA {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// ADNAParameters Implementation
// ============================================================================

impl ADNAParameters {
    /// Create parameters from a profile
    pub fn from_profile(profile: ADNAProfile) -> Self {
        match profile {
            ADNAProfile::Balanced => Self {
                homeostasis_weight: 0.25,
                curiosity_weight: 0.25,
                efficiency_weight: 0.25,
                goal_weight: 0.25,
                exploration_rate: 0.3,
                learning_rate: 0.01, // Moderate learning
                decision_timeout_ms: 1000,
                max_actions_per_cycle: 10,
                _reserved2: [0; 32],
            },
            ADNAProfile::Cautious => Self {
                homeostasis_weight: 0.5,
                curiosity_weight: 0.1,
                efficiency_weight: 0.2,
                goal_weight: 0.2,
                exploration_rate: 0.1,
                learning_rate: 0.005, // Slow learning (cautious)
                decision_timeout_ms: 500,
                max_actions_per_cycle: 5,
                _reserved2: [0; 32],
            },
            ADNAProfile::Curious => Self {
                homeostasis_weight: 0.1,
                curiosity_weight: 0.5,
                efficiency_weight: 0.2,
                goal_weight: 0.2,
                exploration_rate: 0.7,
                learning_rate: 0.02, // Fast learning (curious)
                decision_timeout_ms: 2000,
                max_actions_per_cycle: 20,
                _reserved2: [0; 32],
            },
            ADNAProfile::Adaptive => Self {
                homeostasis_weight: 0.3,
                curiosity_weight: 0.3,
                efficiency_weight: 0.2,
                goal_weight: 0.2,
                exploration_rate: 0.4,
                learning_rate: 0.015, // Adaptive learning
                decision_timeout_ms: 1000,
                max_actions_per_cycle: 15,
                _reserved2: [0; 32],
            },
        }
    }

    /// Validate parameter ranges
    pub fn validate(&self) -> Result<(), ADNAError> {
        // Check weight ranges [0.0, 1.0]
        if !(0.0..=1.0).contains(&self.homeostasis_weight) {
            return Err(ADNAError::InvalidParameter(
                "homeostasis_weight out of range [0, 1]".into(),
            ));
        }
        if !(0.0..=1.0).contains(&self.curiosity_weight) {
            return Err(ADNAError::InvalidParameter(
                "curiosity_weight out of range [0, 1]".into(),
            ));
        }
        if !(0.0..=1.0).contains(&self.efficiency_weight) {
            return Err(ADNAError::InvalidParameter(
                "efficiency_weight out of range [0, 1]".into(),
            ));
        }
        if !(0.0..=1.0).contains(&self.goal_weight) {
            return Err(ADNAError::InvalidParameter(
                "goal_weight out of range [0, 1]".into(),
            ));
        }
        if !(0.0..=1.0).contains(&self.exploration_rate) {
            return Err(ADNAError::InvalidParameter(
                "exploration_rate out of range [0, 1]".into(),
            ));
        }

        // Check learning rate [0.001, 0.1]
        if !(0.001..=0.1).contains(&self.learning_rate) {
            return Err(ADNAError::InvalidParameter(
                "learning_rate out of range [0.001, 0.1]".into(),
            ));
        }

        // Check reasonable timeout
        if self.decision_timeout_ms == 0 || self.decision_timeout_ms > 60000 {
            return Err(ADNAError::InvalidParameter(
                "decision_timeout_ms out of range (0, 60000]".into(),
            ));
        }

        // Check reasonable action limit
        if self.max_actions_per_cycle == 0 || self.max_actions_per_cycle > 1000 {
            return Err(ADNAError::InvalidParameter(
                "max_actions_per_cycle out of range (0, 1000]".into(),
            ));
        }

        Ok(())
    }

    /// Normalize weights to sum to 1.0
    pub fn normalize_weights(&mut self) {
        let total = self.homeostasis_weight
            + self.curiosity_weight
            + self.efficiency_weight
            + self.goal_weight;

        if total > 0.0 {
            self.homeostasis_weight /= total;
            self.curiosity_weight /= total;
            self.efficiency_weight /= total;
            self.goal_weight /= total;
        }
    }
}

impl Default for ADNAParameters {
    fn default() -> Self {
        Self::from_profile(ADNAProfile::Balanced)
    }
}

// ============================================================================
// Default Implementations
// ============================================================================

impl Default for EvolutionMetrics {
    fn default() -> Self {
        Self {
            generation: 0,
            fitness_score: 0.5,
            confidence: 0.5,
            learning_rate: 0.0,
            activation_count: 0,
            success_rate: 0.0,
            rollback_count: 0,
            _reserved: [0; 36],
        }
    }
}

impl Default for PolicyPointer {
    fn default() -> Self {
        Self {
            policy_path_hash: 0,
            policy_checksum: 0,
            policy_size: 0,
            compression_type: 0,
            encryption_flag: 0,
            cache_strategy: 0,
            _reserved1: 0,
            _reserved2: [0; 40],
        }
    }
}

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug, Clone, PartialEq)]
pub enum ADNAError {
    InvalidMagic(u32),
    IncompatibleVersion {
        found: (u16, u16),
        expected: (u16, u16),
    },
    InvalidParameter(String),
    HashMismatch,
    PolicyLoadError(String),
}

impl std::fmt::Display for ADNAError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ADNAError::InvalidMagic(magic) => {
                write!(
                    f,
                    "Invalid ADNA magic number: 0x{:08X} (expected 0x{:08X})",
                    magic, ADNA_MAGIC
                )
            }
            ADNAError::IncompatibleVersion { found, expected } => {
                write!(
                    f,
                    "Incompatible ADNA version: {}.{} (expected {}.{})",
                    found.0, found.1, expected.0, expected.1
                )
            }
            ADNAError::InvalidParameter(msg) => write!(f, "Invalid parameter: {}", msg),
            ADNAError::HashMismatch => write!(f, "ADNA hash mismatch"),
            ADNAError::PolicyLoadError(msg) => write!(f, "Policy load error: {}", msg),
        }
    }
}

impl std::error::Error for ADNAError {}

// ============================================================================
// Compile-time Size Verification
// ============================================================================

const _: () = {
    assert!(
        std::mem::size_of::<ADNA>() == 256,
        "ADNA must be exactly 256 bytes"
    );
    assert!(
        std::mem::size_of::<ADNAHeader>() == 64,
        "ADNAHeader must be 64 bytes"
    );
    assert!(
        std::mem::size_of::<EvolutionMetrics>() == 64,
        "EvolutionMetrics must be 64 bytes"
    );
    assert!(
        std::mem::size_of::<PolicyPointer>() == 64,
        "PolicyPointer must be 64 bytes"
    );
    assert!(
        std::mem::size_of::<ADNAParameters>() == 64,
        "ADNAParameters must be 64 bytes"
    );
};

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adna_size() {
        assert_eq!(std::mem::size_of::<ADNA>(), 256);
        assert_eq!(ADNA::size_bytes(), 256);
    }

    #[test]
    fn test_adna_creation() {
        let adna = ADNA::new();
        assert_eq!(adna.header.magic, ADNA_MAGIC);
        assert_eq!(adna.header.version_major, 1);
        assert_eq!(adna.header.version_minor, 0);
        assert!(adna.validate().is_ok());
    }

    #[test]
    fn test_profiles() {
        let balanced = ADNA::from_profile(ADNAProfile::Balanced);
        assert_eq!(balanced.parameters.homeostasis_weight, 0.25);
        assert_eq!(balanced.parameters.exploration_rate, 0.3);
        assert!(balanced.validate().is_ok());

        let cautious = ADNA::from_profile(ADNAProfile::Cautious);
        assert_eq!(cautious.parameters.homeostasis_weight, 0.5);
        assert_eq!(cautious.parameters.exploration_rate, 0.1);
        assert!(cautious.validate().is_ok());

        let curious = ADNA::from_profile(ADNAProfile::Curious);
        assert_eq!(curious.parameters.curiosity_weight, 0.5);
        assert_eq!(curious.parameters.exploration_rate, 0.7);
        assert!(curious.validate().is_ok());

        let adaptive = ADNA::from_profile(ADNAProfile::Adaptive);
        assert_eq!(adaptive.parameters.homeostasis_weight, 0.3);
        assert_eq!(adaptive.parameters.exploration_rate, 0.4);
        assert!(adaptive.validate().is_ok());
    }

    #[test]
    fn test_parameter_validation() {
        let mut params = ADNAParameters::default();
        assert!(params.validate().is_ok());

        // Test invalid weight
        params.homeostasis_weight = 1.5;
        assert!(params.validate().is_err());
        params.homeostasis_weight = 0.25;

        // Test invalid timeout
        params.decision_timeout_ms = 0;
        assert!(params.validate().is_err());
        params.decision_timeout_ms = 100000;
        assert!(params.validate().is_err());
        params.decision_timeout_ms = 1000;
        assert!(params.validate().is_ok());

        // Test invalid action limit
        params.max_actions_per_cycle = 0;
        assert!(params.validate().is_err());
        params.max_actions_per_cycle = 2000;
        assert!(params.validate().is_err());
        params.max_actions_per_cycle = 10;
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_weight_normalization() {
        let mut params = ADNAParameters::default();
        params.homeostasis_weight = 2.0;
        params.curiosity_weight = 2.0;
        params.efficiency_weight = 2.0;
        params.goal_weight = 2.0;

        params.normalize_weights();

        assert_eq!(params.homeostasis_weight, 0.25);
        assert_eq!(params.curiosity_weight, 0.25);
        assert_eq!(params.efficiency_weight, 0.25);
        assert_eq!(params.goal_weight, 0.25);
    }

    #[test]
    fn test_hash_update() {
        let mut adna = ADNA::new();
        let initial_hash = adna.header.parent_hash;

        std::thread::sleep(std::time::Duration::from_millis(10));
        adna.parameters.exploration_rate = 0.5;
        adna.update_hash();

        assert_ne!(adna.header.parent_hash, initial_hash);
    }

    #[test]
    fn test_generation_tracking() {
        let mut adna = ADNA::new();
        assert_eq!(adna.metrics.generation, 0);

        adna.increment_generation();
        assert_eq!(adna.metrics.generation, 1);

        adna.increment_generation();
        assert_eq!(adna.metrics.generation, 2);
    }

    #[test]
    fn test_activation_tracking() {
        let mut adna = ADNA::new();
        assert_eq!(adna.metrics.activation_count, 0);

        adna.record_activation();
        assert_eq!(adna.metrics.activation_count, 1);

        adna.record_activation();
        assert_eq!(adna.metrics.activation_count, 2);
    }

    #[test]
    fn test_invalid_magic() {
        let mut adna = ADNA::new();
        adna.header.magic = 0xDEADBEEF;

        let result = adna.validate();
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), ADNAError::InvalidMagic(_)));
    }

    #[test]
    fn test_incompatible_version() {
        let mut adna = ADNA::new();
        adna.header.version_major = 99;

        let result = adna.validate();
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            ADNAError::IncompatibleVersion { .. }
        ));
    }
}
