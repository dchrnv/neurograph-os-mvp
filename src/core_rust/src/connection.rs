/// Connection V1.0 - Link between Tokens in NeuroGraph OS
///
/// A Connection represents a directed or undirected relationship between two tokens,
/// acting as a physical force that influences token positions in multidimensional space.
///
/// Binary layout: 32 bytes total
/// - token_a_id: 4 bytes (u32)
/// - token_b_id: 4 bytes (u32)
/// - connection_type: 1 byte (u8)
/// - rigidity: 1 byte (u8)
/// - active_levels: 1 byte (u8 bitmask)
/// - flags: 1 byte (u8)
/// - activation_count: 4 bytes (u32)
/// - pull_strength: 4 bytes (f32)
/// - preferred_distance: 4 bytes (f32)
/// - created_at: 4 bytes (u32)
/// - last_activation: 4 bytes (u32)
use std::time::{SystemTime, UNIX_EPOCH};

/// Connection types organized in hierarchical categories
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    // Semantic relations (0x00-0x0F)
    Undefined = 0x00,
    Synonym = 0x01,
    Antonym = 0x02,
    Hypernym = 0x03,
    Hyponym = 0x04,
    Meronym = 0x05,
    Holonym = 0x06,

    // Causal relations (0x10-0x1F)
    Cause = 0x10,
    Effect = 0x11,
    EnabledBy = 0x12,
    PreventedBy = 0x13,

    // Temporal relations (0x20-0x2F)
    Before = 0x20,
    After = 0x21,
    During = 0x22,
    Simultaneous = 0x23,

    // Spatial relations (0x30-0x3F)
    Near = 0x30,
    Far = 0x31,
    Inside = 0x32,
    Outside = 0x33,
    Above = 0x34,
    Below = 0x35,

    // Logical relations (0x40-0x4F)
    And = 0x40,
    Or = 0x41,
    Not = 0x42,
    Implies = 0x43,
    Equivalent = 0x44,

    // Associative links (0x50-0x5F)
    AssociatedWith = 0x50,
    SimilarTo = 0x51,
    RelatedTo = 0x52,

    // Structural links (0x60-0x6F)
    PartOf = 0x60,
    HasPart = 0x61,
    MemberOf = 0x62,
    HasMember = 0x63,

    // Functional links (0x70-0x7F)
    UsedFor = 0x70,
    CapableOf = 0x71,
    RequiredFor = 0x72,

    // Emotional links (0x80-0x8F)
    Likes = 0x80,
    Dislikes = 0x81,
    Fears = 0x82,
    Desires = 0x83,

    // Rules and metaphors (0x90-0x9F)
    Rule = 0x90,
    Metaphor = 0x91,
    Analogy = 0x92,

    // Dynamic links (0xA0-0xAF)
    Transition = 0xA0,
    Transform = 0xA1,
    Evolve = 0xA2,

    // User-defined (0xF0-0xFF)
    UserDefined = 0xF0,
}

impl ConnectionType {
    /// Convert u8 to ConnectionType
    pub fn from_u8(value: u8) -> Self {
        match value {
            0x00 => Self::Undefined,
            0x01 => Self::Synonym,
            0x02 => Self::Antonym,
            0x03 => Self::Hypernym,
            0x04 => Self::Hyponym,
            0x05 => Self::Meronym,
            0x06 => Self::Holonym,
            0x10 => Self::Cause,
            0x11 => Self::Effect,
            0x12 => Self::EnabledBy,
            0x13 => Self::PreventedBy,
            0x20 => Self::Before,
            0x21 => Self::After,
            0x22 => Self::During,
            0x23 => Self::Simultaneous,
            0x30 => Self::Near,
            0x31 => Self::Far,
            0x32 => Self::Inside,
            0x33 => Self::Outside,
            0x34 => Self::Above,
            0x35 => Self::Below,
            0x40 => Self::And,
            0x41 => Self::Or,
            0x42 => Self::Not,
            0x43 => Self::Implies,
            0x44 => Self::Equivalent,
            0x50 => Self::AssociatedWith,
            0x51 => Self::SimilarTo,
            0x52 => Self::RelatedTo,
            0x60 => Self::PartOf,
            0x61 => Self::HasPart,
            0x62 => Self::MemberOf,
            0x63 => Self::HasMember,
            0x70 => Self::UsedFor,
            0x71 => Self::CapableOf,
            0x72 => Self::RequiredFor,
            0x80 => Self::Likes,
            0x81 => Self::Dislikes,
            0x82 => Self::Fears,
            0x83 => Self::Desires,
            0x90 => Self::Rule,
            0x91 => Self::Metaphor,
            0x92 => Self::Analogy,
            0xA0 => Self::Transition,
            0xA1 => Self::Transform,
            0xA2 => Self::Evolve,
            _ => Self::UserDefined,
        }
    }
}

/// Active levels bitmask (which of 8 semantic spaces the connection affects)
pub mod active_levels {
    pub const NONE: u8 = 0x00;
    pub const L1_PHYSICAL: u8 = 0x01;
    pub const L2_SENSORY: u8 = 0x02;
    pub const L3_MOTOR: u8 = 0x04;
    pub const L4_EMOTIONAL: u8 = 0x08;
    pub const L5_COGNITIVE: u8 = 0x10;
    pub const L6_SOCIAL: u8 = 0x20;
    pub const L7_TEMPORAL: u8 = 0x40;
    pub const L8_ABSTRACT: u8 = 0x80;
    pub const ALL: u8 = 0xFF;

    // Predefined combinations
    pub const PHYSICAL_MOTOR: u8 = 0x05; // L1 + L3
    pub const EMOTIONAL_SOCIAL: u8 = 0x28; // L4 + L6
    pub const COGNITIVE_ABSTRACT: u8 = 0x90; // L5 + L8
    pub const TEMPORAL_ABSTRACT: u8 = 0xC0; // L7 + L8
    pub const LOWER_LEVELS: u8 = 0x0F; // L1-L4
    pub const UPPER_LEVELS: u8 = 0xF0; // L5-L8
}

/// Connection flags
pub mod flags {
    pub const ACTIVE: u8 = 0x01; // Connection is active
    pub const BIDIRECTIONAL: u8 = 0x02; // Can traverse in both directions
    pub const PERSISTENT: u8 = 0x04; // Should be saved
    pub const MUTABLE: u8 = 0x08; // Can be modified
    pub const REINFORCED: u8 = 0x10; // Recently reinforced
    pub const DECAYING: u8 = 0x20; // Losing strength over time
    pub const USER_1: u8 = 0x40; // User-defined flag 1
    pub const USER_2: u8 = 0x80; // User-defined flag 2
}

/// Connection V1.0 structure (32 bytes, naturally aligned)
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Connection {
    /// ID of first token
    pub token_a_id: u32,

    /// ID of second token
    pub token_b_id: u32,

    /// Type of connection (0-255)
    pub connection_type: u8,

    /// Rigidity (0-255, represents 0.0-1.0)
    pub rigidity: u8,

    /// Bitmask of active levels (which spaces affected)
    pub active_levels: u8,

    /// Connection flags
    pub flags: u8,

    /// Number of times this connection was activated
    pub activation_count: u32,

    /// Pull strength (positive = attraction, negative = repulsion)
    pub pull_strength: f32,

    /// Preferred distance between tokens
    pub preferred_distance: f32,

    /// Unix timestamp when created
    pub created_at: u32,

    /// Unix timestamp of last activation
    pub last_activation: u32,
}

// Compile-time size check
const _: () = assert!(std::mem::size_of::<Connection>() == 32);

impl Connection {
    /// Create a new connection between two tokens
    pub fn new(token_a_id: u32, token_b_id: u32) -> Self {
        let now = Self::current_timestamp();
        Self {
            token_a_id,
            token_b_id,
            connection_type: ConnectionType::Undefined as u8,
            rigidity: 128, // 0.5 in fixed-point
            active_levels: active_levels::ALL,
            flags: flags::ACTIVE | flags::BIDIRECTIONAL,
            activation_count: 0,
            pull_strength: 0.50,
            preferred_distance: 1.00,
            created_at: now,
            last_activation: now,
        }
    }

    /// Get current Unix timestamp
    pub fn current_timestamp() -> u32 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as u32
    }

    /// Set connection type
    pub fn set_connection_type(&mut self, conn_type: ConnectionType) {
        self.connection_type = conn_type as u8;
    }

    /// Get connection type
    pub fn get_connection_type(&self) -> ConnectionType {
        ConnectionType::from_u8(self.connection_type)
    }

    /// Set rigidity (0.0-1.0)
    pub fn set_rigidity(&mut self, rigidity: f32) {
        self.rigidity = (rigidity * 255.0).clamp(0.0, 255.0) as u8;
    }

    /// Get rigidity (0.0-1.0)
    pub fn get_rigidity(&self) -> f32 {
        (self.rigidity as f32) / 255.0
    }

    /// Check if a flag is set
    pub fn has_flag(&self, flag: u8) -> bool {
        (self.flags & flag) != 0
    }

    /// Set a flag
    pub fn set_flag(&mut self, flag: u8) {
        self.flags |= flag;
    }

    /// Clear a flag
    pub fn clear_flag(&mut self, flag: u8) {
        self.flags &= !flag;
    }

    /// Check if level is active
    pub fn is_level_active(&self, level: u8) -> bool {
        (self.active_levels & level) != 0
    }

    /// Activate a level
    pub fn activate_level(&mut self, level: u8) {
        self.active_levels |= level;
    }

    /// Deactivate a level
    pub fn deactivate_level(&mut self, level: u8) {
        self.active_levels &= !level;
    }

    /// Activate the connection (increment counter, update timestamp)
    pub fn activate(&mut self) {
        if self.activation_count < u32::MAX {
            self.activation_count += 1;
        }
        self.last_activation = Self::current_timestamp();
    }

    /// Get age in seconds
    pub fn age(&self) -> u32 {
        Self::current_timestamp().saturating_sub(self.created_at)
    }

    /// Get time since last activation in seconds
    pub fn time_since_activation(&self) -> u32 {
        Self::current_timestamp().saturating_sub(self.last_activation)
    }

    /// Serialize to bytes (32 bytes)
    pub fn to_bytes(&self) -> [u8; 32] {
        unsafe { std::mem::transmute(*self) }
    }

    /// Deserialize from bytes (32 bytes)
    pub fn from_bytes(bytes: &[u8; 32]) -> Self {
        unsafe { std::mem::transmute(*bytes) }
    }

    /// Validate connection
    pub fn validate(&self) -> Result<(), &'static str> {
        // Check token IDs are non-zero
        if self.token_a_id == 0 {
            return Err("token_a_id cannot be zero");
        }
        if self.token_b_id == 0 {
            return Err("token_b_id cannot be zero");
        }

        // Check self-connections
        if self.token_a_id == self.token_b_id {
            return Err("Self-connections not allowed in v1.0");
        }

        // Check timestamps
        let now = Self::current_timestamp();
        if self.created_at > now + 3600 {
            return Err("created_at is too far in the future");
        }
        if self.last_activation > now + 3600 {
            return Err("last_activation is too far in the future");
        }
        if self.last_activation < self.created_at {
            return Err("last_activation cannot be before created_at");
        }

        // Check for NaN or Inf in floats
        if !self.pull_strength.is_finite() {
            return Err("pull_strength must be finite");
        }
        if !self.preferred_distance.is_finite() {
            return Err("preferred_distance must be finite");
        }

        // Check preferred_distance is non-negative
        if self.preferred_distance < 0.0 {
            return Err("preferred_distance must be non-negative");
        }

        Ok(())
    }
}

impl std::fmt::Debug for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Connection")
            .field("token_a_id", &self.token_a_id)
            .field("token_b_id", &self.token_b_id)
            .field("type", &self.get_connection_type())
            .field("rigidity", &self.get_rigidity())
            .field("active_levels", &format!("{:#04x}", self.active_levels))
            .field("flags", &format!("{:#04x}", self.flags))
            .field("activation_count", &self.activation_count)
            .field("pull_strength", &self.pull_strength)
            .field("preferred_distance", &self.preferred_distance)
            .field("created_at", &self.created_at)
            .field("last_activation", &self.last_activation)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_size() {
        assert_eq!(std::mem::size_of::<Connection>(), 32);
    }

    #[test]
    fn test_connection_new() {
        let conn = Connection::new(1, 2);
        assert_eq!(conn.token_a_id, 1);
        assert_eq!(conn.token_b_id, 2);
        assert!(conn.has_flag(flags::ACTIVE));
        assert!(conn.has_flag(flags::BIDIRECTIONAL));
        assert_eq!(conn.activation_count, 0);
    }

    #[test]
    fn test_connection_type() {
        let mut conn = Connection::new(1, 2);
        conn.set_connection_type(ConnectionType::Synonym);
        assert_eq!(conn.get_connection_type(), ConnectionType::Synonym);
    }

    #[test]
    fn test_rigidity() {
        let mut conn = Connection::new(1, 2);
        conn.set_rigidity(0.75);
        let rig = conn.get_rigidity();
        assert!((rig - 0.75).abs() < 0.01);
    }

    #[test]
    fn test_flags() {
        let mut conn = Connection::new(1, 2);
        assert!(conn.has_flag(flags::ACTIVE));

        conn.set_flag(flags::PERSISTENT);
        assert!(conn.has_flag(flags::PERSISTENT));

        conn.clear_flag(flags::ACTIVE);
        assert!(!conn.has_flag(flags::ACTIVE));
    }

    #[test]
    fn test_active_levels() {
        let mut conn = Connection::new(1, 2);

        conn.active_levels = active_levels::NONE;
        assert!(!conn.is_level_active(active_levels::L1_PHYSICAL));

        conn.activate_level(active_levels::L1_PHYSICAL);
        assert!(conn.is_level_active(active_levels::L1_PHYSICAL));

        conn.deactivate_level(active_levels::L1_PHYSICAL);
        assert!(!conn.is_level_active(active_levels::L1_PHYSICAL));
    }

    #[test]
    fn test_activation() {
        let mut conn = Connection::new(1, 2);
        assert_eq!(conn.activation_count, 0);

        conn.activate();
        assert_eq!(conn.activation_count, 1);

        conn.activate();
        assert_eq!(conn.activation_count, 2);
    }

    #[test]
    fn test_serialization() {
        let mut conn = Connection::new(1, 2);
        conn.set_connection_type(ConnectionType::Cause);
        conn.set_rigidity(0.80);
        conn.pull_strength = 0.70;
        conn.preferred_distance = 1.50;

        let bytes = conn.to_bytes();
        let conn2 = Connection::from_bytes(&bytes);

        assert_eq!(conn2.token_a_id, 1);
        assert_eq!(conn2.token_b_id, 2);
        assert_eq!(conn2.get_connection_type(), ConnectionType::Cause);
        assert!((conn2.get_rigidity() - 0.80).abs() < 0.01);
        assert_eq!(conn2.pull_strength, 0.70);
        assert_eq!(conn2.preferred_distance, 1.50);
    }

    #[test]
    fn test_validation() {
        let conn = Connection::new(1, 2);
        assert!(conn.validate().is_ok());

        // Test zero token_a_id
        let mut bad_conn = conn;
        bad_conn.token_a_id = 0;
        assert!(bad_conn.validate().is_err());

        // Test self-connection
        let mut bad_conn = conn;
        bad_conn.token_b_id = 1;
        assert!(bad_conn.validate().is_err());

        // Test invalid floats
        let mut bad_conn = conn;
        bad_conn.pull_strength = f32::NAN;
        assert!(bad_conn.validate().is_err());
    }

    #[test]
    fn test_age() {
        let conn = Connection::new(1, 2);
        let age = conn.age();
        assert!(age < 2); // Should be < 2 seconds old
    }
}
