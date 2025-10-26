/// CDNA V2.1 - Cognitive DNA for NeuroGraph OS
///
/// CDNA is the constitutional framework of NeuroGraph OS - a 384-byte structure
/// that defines the fundamental laws of the cognitive universe.
///
/// # Binary Layout
///
/// Total: 384 bytes (6 CPU cache lines)
/// - Block 1: Header (64 bytes) - version, timestamps, checksums
/// - Block 2: Grid Physics (128 bytes) - 8D semantic space constants
/// - Block 3: Graph Topology (64 bytes) - connection rules, limits
/// - Block 4: Token Properties (32 bytes) - token constraints
/// - Block 5: Connection Constraints (64 bytes) - connection parameters
/// - Block 6: Evolution & Subscription (32 bytes) - evolution rules
///
/// # Design Principles
///
/// - **Immutability**: CDNA is the LAW - only user can change it
/// - **Cache-friendly**: Exactly 6 cache lines (64 bytes each)
/// - **Zero-copy**: Direct memory mapping
/// - **Versioned**: History tracking with rollback support
/// - **Validated**: All parameters have strict bounds

use std::time::{SystemTime, UNIX_EPOCH};

/// CDNA magic number: "CDNA" in ASCII
pub const CDNA_MAGIC: u32 = 0x434E4441; // "CDNA"

/// Current CDNA version
pub const CDNA_VERSION_MAJOR: u16 = 2;
pub const CDNA_VERSION_MINOR: u16 = 1;

/// Profile IDs for predefined configurations
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileId {
    Default = 0,
    Explorer = 1,    // High connectivity, low constraints
    Analyst = 2,     // Strict validation, high precision
    Creative = 3,    // Loose constraints, high mutation
    Custom = 255,    // User-defined profile
}

impl From<u32> for ProfileId {
    fn from(value: u32) -> Self {
        match value {
            0 => ProfileId::Default,
            1 => ProfileId::Explorer,
            2 => ProfileId::Analyst,
            3 => ProfileId::Creative,
            255 => ProfileId::Custom,
            _ => ProfileId::Default,
        }
    }
}

/// Profile state flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProfileState {
    bits: u32,
}

impl ProfileState {
    pub const ACTIVE: u32 = 0x0001;
    pub const QUARANTINE: u32 = 0x0002;
    pub const VALIDATED: u32 = 0x0004;
    pub const READONLY: u32 = 0x0008;

    pub fn new(bits: u32) -> Self {
        Self { bits }
    }

    pub fn is_active(&self) -> bool {
        self.bits & Self::ACTIVE != 0
    }

    pub fn is_quarantine(&self) -> bool {
        self.bits & Self::QUARANTINE != 0
    }

    pub fn is_validated(&self) -> bool {
        self.bits & Self::VALIDATED != 0
    }

    pub fn is_readonly(&self) -> bool {
        self.bits & Self::READONLY != 0
    }
}

/// CDNA flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CDNAFlags {
    bits: u32,
}

impl CDNAFlags {
    pub const ENABLE_VALIDATION: u32 = 0x0001;
    pub const ENABLE_EVENTS: u32 = 0x0002;
    pub const ENABLE_MUTATION: u32 = 0x0004;
    pub const STRICT_MODE: u32 = 0x0008;

    pub fn new(bits: u32) -> Self {
        Self { bits }
    }

    pub fn default() -> Self {
        Self {
            bits: Self::ENABLE_VALIDATION | Self::ENABLE_EVENTS,
        }
    }

    pub fn validation_enabled(&self) -> bool {
        self.bits & Self::ENABLE_VALIDATION != 0
    }

    pub fn events_enabled(&self) -> bool {
        self.bits & Self::ENABLE_EVENTS != 0
    }

    pub fn mutation_enabled(&self) -> bool {
        self.bits & Self::ENABLE_MUTATION != 0
    }

    pub fn strict_mode(&self) -> bool {
        self.bits & Self::STRICT_MODE != 0
    }
}

/// CDNA V2.1 - Complete 384-byte structure
///
/// # Memory Layout
///
/// ```text
/// Offset | Size | Field
/// -------|------|------------------
/// 0      | 4    | magic
/// 4      | 2    | version_major
/// 6      | 2    | version_minor
/// 8      | 8    | created_at
/// 16     | 8    | modified_at
/// 24     | 4    | profile_id
/// 28     | 4    | profile_state
/// 32     | 4    | flags
/// 36     | 4    | reserved1
/// 40     | 8    | checksum
/// 48     | 16   | reserved2
/// -------|------|------------------
/// 64     | 8    | dimension_ids[8]
/// 72     | 8    | dimension_flags[8]
/// 80     | 32   | dimension_scales[8]
/// 112    | 32   | bucket_sizes[8]
/// 144    | 32   | field_strength_limits[8]
/// 176    | 16   | reserved3
/// -------|------|------------------
/// 192    | 8    | allowed_connection_types
/// 200    | 4    | max_out_degree
/// 204    | 4    | max_in_degree
/// 208    | 4    | max_total_degree
/// 212    | 4    | min_weight_threshold
/// 216    | 4    | max_weight_threshold
/// 220    | 36   | reserved4
/// -------|------|------------------
/// 256    | 4    | min_token_weight
/// 260    | 4    | max_token_weight
/// 264    | 2    | min_field_radius
/// 266    | 2    | max_field_radius
/// 268    | 2    | min_field_strength
/// 270    | 2    | max_field_strength
/// 272    | 16   | reserved5
/// -------|------|------------------
/// 288    | 4    | min_connection_weight
/// 292    | 4    | max_connection_weight
/// 296    | 4    | min_rigidity
/// 300    | 4    | max_rigidity
/// 304    | 4    | default_pull_strength
/// 308    | 4    | decay_rate
/// 312    | 40   | reserved6
/// -------|------|------------------
/// 352    | 4    | mutation_rate
/// 356    | 4    | crossover_rate
/// 360    | 4    | selection_pressure
/// 364    | 4    | evolution_flags
/// 368    | 16   | reserved7
/// -------|------|------------------
/// Total: 384 bytes
/// ```
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct CDNA {
    // ==================== BLOCK 1: HEADER (64 bytes) ====================
    /// Magic number "CDNA" (0x434E4441)
    pub magic: u32,
    /// Major version number
    pub version_major: u16,
    /// Minor version number
    pub version_minor: u16,
    /// Creation timestamp (Unix epoch seconds)
    pub created_at: u64,
    /// Last modification timestamp
    pub modified_at: u64,
    /// Profile ID (Default, Explorer, Analyst, Creative, Custom)
    pub profile_id: u32,
    /// Profile state flags
    pub profile_state: u32,
    /// CDNA flags (validation, events, mutation, strict mode)
    pub flags: u32,
    /// Reserved for future use
    reserved1: u32,
    /// Checksum (FNV-1a hash of entire structure)
    pub checksum: u64,
    /// Reserved for alignment
    reserved2: [u8; 16],

    // ==================== BLOCK 2: GRID PHYSICS (128 bytes) ====================
    /// Semantic dimension IDs (L1-L8)
    pub dimension_ids: [u8; 8],
    /// Dimension flags (enabled, normalized, etc.)
    pub dimension_flags: [u8; 8],
    /// Scale factors for each dimension
    pub dimension_scales: [f32; 8],
    /// Bucket sizes for spatial indexing per dimension
    pub bucket_sizes: [f32; 8],
    /// Field strength limits per dimension
    pub field_strength_limits: [f32; 8],
    /// Reserved
    reserved3: [u8; 16],

    // ==================== BLOCK 3: GRAPH TOPOLOGY (64 bytes) ====================
    /// Allowed connection types (bitmask)
    pub allowed_connection_types: u64,
    /// Maximum outgoing degree per node
    pub max_out_degree: u32,
    /// Maximum incoming degree per node
    pub max_in_degree: u32,
    /// Maximum total degree per node
    pub max_total_degree: u32,
    /// Minimum weight threshold for connections
    pub min_weight_threshold: f32,
    /// Maximum weight threshold for connections
    pub max_weight_threshold: f32,
    /// Reserved
    reserved4: [u8; 36],

    // ==================== BLOCK 4: TOKEN PROPERTIES (32 bytes) ====================
    /// Minimum token weight
    pub min_token_weight: f32,
    /// Maximum token weight
    pub max_token_weight: f32,
    /// Minimum field radius (encoded as u8)
    pub min_field_radius: u16,
    /// Maximum field radius (encoded as u8)
    pub max_field_radius: u16,
    /// Minimum field strength (encoded as u8)
    pub min_field_strength: u16,
    /// Maximum field strength (encoded as u8)
    pub max_field_strength: u16,
    /// Reserved
    reserved5: [u8; 16],

    // ==================== BLOCK 5: CONNECTION CONSTRAINTS (64 bytes) ====================
    /// Minimum connection weight
    pub min_connection_weight: f32,
    /// Maximum connection weight
    pub max_connection_weight: f32,
    /// Minimum rigidity value
    pub min_rigidity: f32,
    /// Maximum rigidity value
    pub max_rigidity: f32,
    /// Default pull strength
    pub default_pull_strength: f32,
    /// Decay rate for connections
    pub decay_rate: f32,
    /// Reserved
    reserved6: [u8; 40],

    // ==================== BLOCK 6: EVOLUTION & SUBSCRIPTION (32 bytes) ====================
    /// Mutation rate (0.0 - 1.0)
    pub mutation_rate: f32,
    /// Crossover rate (0.0 - 1.0)
    pub crossover_rate: f32,
    /// Selection pressure (0.0 - 1.0)
    pub selection_pressure: f32,
    /// Evolution flags
    pub evolution_flags: u32,
    /// Reserved
    reserved7: [u8; 16],
}

// Compile-time assertion: CDNA must be exactly 384 bytes
const _: () = assert!(std::mem::size_of::<CDNA>() == 384);

impl CDNA {
    /// Create a new CDNA with default configuration
    pub fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut cdna = Self {
            magic: CDNA_MAGIC,
            version_major: CDNA_VERSION_MAJOR,
            version_minor: CDNA_VERSION_MINOR,
            created_at: now,
            modified_at: now,
            profile_id: ProfileId::Default as u32,
            profile_state: ProfileState::ACTIVE | ProfileState::VALIDATED,
            flags: CDNAFlags::default().bits,
            reserved1: 0,
            checksum: 0,
            reserved2: [0; 16],

            // Grid Physics defaults
            dimension_ids: [0, 1, 2, 3, 4, 5, 6, 7], // L1-L8
            dimension_flags: [0xFF; 8],               // All enabled
            dimension_scales: [1.0; 8],
            bucket_sizes: [10.0; 8],
            field_strength_limits: [1.0; 8],
            reserved3: [0; 16],

            // Graph Topology defaults
            allowed_connection_types: u64::MAX, // All types allowed
            max_out_degree: 1000,
            max_in_degree: 1000,
            max_total_degree: 2000,
            min_weight_threshold: 0.0,
            max_weight_threshold: 1.0,
            reserved4: [0; 36],

            // Token Properties defaults
            min_token_weight: 0.0,
            max_token_weight: 1.0,
            min_field_radius: 0,
            max_field_radius: 255,
            min_field_strength: 0,
            max_field_strength: 255,
            reserved5: [0; 16],

            // Connection Constraints defaults
            min_connection_weight: 0.0,
            max_connection_weight: 1.0,
            min_rigidity: 0.0,
            max_rigidity: 1.0,
            default_pull_strength: 0.5,
            decay_rate: 0.01,
            reserved6: [0; 40],

            // Evolution defaults
            mutation_rate: 0.01,
            crossover_rate: 0.1,
            selection_pressure: 0.5,
            evolution_flags: 0,
            reserved7: [0; 16],
        };

        cdna.checksum = cdna.compute_checksum();
        cdna
    }

    /// Create CDNA with specific profile
    pub fn with_profile(profile: ProfileId) -> Self {
        let mut cdna = Self::new();
        cdna.profile_id = profile as u32;

        // Apply profile-specific settings
        match profile {
            ProfileId::Explorer => {
                cdna.max_out_degree = 2000;
                cdna.max_total_degree = 4000;
                cdna.mutation_rate = 0.05;
            }
            ProfileId::Analyst => {
                cdna.max_out_degree = 500;
                cdna.max_total_degree = 1000;
                cdna.flags |= CDNAFlags::STRICT_MODE;
            }
            ProfileId::Creative => {
                cdna.mutation_rate = 0.1;
                cdna.crossover_rate = 0.2;
                cdna.flags |= CDNAFlags::ENABLE_MUTATION;
            }
            _ => {}
        }

        cdna.checksum = cdna.compute_checksum();
        cdna
    }

    /// Compute FNV-1a checksum of CDNA structure
    /// (excluding checksum field itself)
    pub fn compute_checksum(&self) -> u64 {
        const FNV_OFFSET: u64 = 14695981039346656037;
        const FNV_PRIME: u64 = 1099511628211;

        let mut hash = FNV_OFFSET;

        // Convert struct to bytes (excluding checksum field)
        let bytes = unsafe {
            std::slice::from_raw_parts(
                self as *const CDNA as *const u8,
                std::mem::size_of::<CDNA>(),
            )
        };

        // Hash all bytes except checksum field (offset 40-47)
        for (i, &byte) in bytes.iter().enumerate() {
            if i >= 40 && i < 48 {
                continue; // Skip checksum field
            }
            hash ^= byte as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
        }

        hash
    }

    /// Validate CDNA structure
    pub fn validate(&self) -> Result<(), String> {
        // Check magic number
        if self.magic != CDNA_MAGIC {
            return Err(format!("Invalid magic number: 0x{:08X}", self.magic));
        }

        // Check version
        if self.version_major != CDNA_VERSION_MAJOR {
            return Err(format!(
                "Unsupported major version: {}",
                self.version_major
            ));
        }

        // Validate checksum
        let computed = self.compute_checksum();
        if self.checksum != computed {
            return Err(format!(
                "Checksum mismatch: expected 0x{:016X}, got 0x{:016X}",
                computed, self.checksum
            ));
        }

        // Validate timestamps
        if self.modified_at < self.created_at {
            return Err("Modified timestamp before created timestamp".to_string());
        }

        // Validate ranges
        if self.min_token_weight > self.max_token_weight {
            return Err("Invalid token weight range".to_string());
        }

        if self.min_connection_weight > self.max_connection_weight {
            return Err("Invalid connection weight range".to_string());
        }

        if self.mutation_rate < 0.0 || self.mutation_rate > 1.0 {
            return Err("Mutation rate must be in [0.0, 1.0]".to_string());
        }

        Ok(())
    }

    /// Check if CDNA is active
    pub fn is_active(&self) -> bool {
        ProfileState::new(self.profile_state).is_active()
    }

    /// Check if validation is enabled
    pub fn validation_enabled(&self) -> bool {
        CDNAFlags::new(self.flags).validation_enabled()
    }

    /// Get profile type
    pub fn profile(&self) -> ProfileId {
        self.profile_id.into()
    }

    /// Update modification timestamp and recompute checksum
    pub fn touch(&mut self) {
        self.modified_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        self.checksum = self.compute_checksum();
    }
}

impl Default for CDNA {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdna_size() {
        assert_eq!(std::mem::size_of::<CDNA>(), 384);
        assert_eq!(std::mem::align_of::<CDNA>(), 64);
    }

    #[test]
    fn test_cdna_creation() {
        let cdna = CDNA::new();
        assert_eq!(cdna.magic, CDNA_MAGIC);
        assert_eq!(cdna.version_major, CDNA_VERSION_MAJOR);
        assert_eq!(cdna.version_minor, CDNA_VERSION_MINOR);
        assert!(cdna.validate().is_ok());
    }

    #[test]
    fn test_cdna_checksum() {
        let cdna = CDNA::new();
        let checksum1 = cdna.compute_checksum();
        assert_eq!(cdna.checksum, checksum1);

        let mut cdna2 = cdna;
        cdna2.mutation_rate = 0.5;
        let checksum2 = cdna2.compute_checksum();
        assert_ne!(checksum1, checksum2);
    }

    #[test]
    fn test_cdna_profiles() {
        let default_cdna = CDNA::with_profile(ProfileId::Default);
        let explorer = CDNA::with_profile(ProfileId::Explorer);
        let analyst = CDNA::with_profile(ProfileId::Analyst);

        assert!(default_cdna.max_out_degree < explorer.max_out_degree);
        assert!(analyst.max_out_degree < default_cdna.max_out_degree);
        assert!(analyst.mutation_rate < explorer.mutation_rate);
    }

    #[test]
    fn test_cdna_validation() {
        let mut cdna = CDNA::new();
        assert!(cdna.validate().is_ok());

        // Invalid magic
        cdna.magic = 0xDEADBEEF;
        assert!(cdna.validate().is_err());

        // Fix and test invalid range
        cdna.magic = CDNA_MAGIC;
        cdna.min_token_weight = 1.0;
        cdna.max_token_weight = 0.0;
        cdna.checksum = cdna.compute_checksum();
        assert!(cdna.validate().is_err());
    }

    #[test]
    fn test_profile_state() {
        let state = ProfileState::new(ProfileState::ACTIVE | ProfileState::VALIDATED);
        assert!(state.is_active());
        assert!(state.is_validated());
        assert!(!state.is_quarantine());
        assert!(!state.is_readonly());
    }

    #[test]
    fn test_cdna_flags() {
        let flags = CDNAFlags::default();
        assert!(flags.validation_enabled());
        assert!(flags.events_enabled());
        assert!(!flags.mutation_enabled());
        assert!(!flags.strict_mode());
    }
}
