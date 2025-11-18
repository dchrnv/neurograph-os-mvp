// NeuroGraph - Высокопроизводительная система пространственных вычислений на основе токенов.
// Copyright (C) 2024-2025 Chernov Denys

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

/// Connection V3.0 - Learning-Capable Links in NeuroGraph OS
///
/// A Connection represents a relationship between two tokens with three-tier mutability:
/// - Immutable: Ontological facts (IsA, Synonym) that never change
/// - Learnable: Causal hypotheses (Cause, Effect) refined through experience
/// - Hypothesis: Experimental patterns with fast learning and decay
///
/// Binary layout: 64 bytes total (2× cache line aligned)
///
/// Core fields (32 bytes, v1.0 compatible):
/// - token_a_id: 4 bytes (u32)
/// - token_b_id: 4 bytes (u32)
/// - connection_type: 1 byte (u8)
/// - rigidity: 1 byte (u8, 0-255 = 0.0-1.0)
/// - active_levels: 1 byte (u8 bitmask)
/// - flags: 1 byte (u8)
/// - activation_count: 4 bytes (u32)
/// - pull_strength: 4 bytes (f32)
/// - preferred_distance: 4 bytes (f32)
/// - created_at: 4 bytes (u32 Unix timestamp)
/// - last_activation: 4 bytes (u32 Unix timestamp)
///
/// Learning extension (32 bytes, NEW in v3.0):
/// - mutability: 1 byte (u8: 0=Immutable, 1=Learnable, 2=Hypothesis)
/// - confidence: 1 byte (u8, 0-255 = 0.0-1.0)
/// - evidence_count: 2 bytes (u16)
/// - last_update: 4 bytes (u32 Unix timestamp)
/// - learning_rate: 1 byte (u8, 0-255 = 0.0-1.0)
/// - decay_rate: 1 byte (u8, 0-255 = 0.0-1.0)
/// - _padding1: 2 bytes (alignment)
/// - source_id: 4 bytes (u32, 0=manual, >0=IntuitionEngine proposal ID)
/// - reserved: 16 bytes (future extensions)

use std::time::{SystemTime, UNIX_EPOCH};

/// Three-tier mutability system (synaptic plasticity analogy)
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionMutability {
    /// Ontological facts that never change (IsA, Synonym, PartOf)
    /// Confidence always 255. Cannot be modified by IntuitionEngine.
    Immutable = 0,

    /// Causal hypotheses refined through experience (Cause, Effect, EnabledBy)
    /// Confidence updated based on observations. Learning rate ~0.125, decay ~0.0625.
    Learnable = 1,

    /// Experimental patterns for testing (fast learning + decay)
    /// High learning rate (~0.5), moderate decay (~0.125). Deleted if confidence < 10%.
    Hypothesis = 2,
}

/// Connection types - organized in 11 categories (176 total types)
/// Based on Connection_V3_UNIFIED.md specification
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionType {
    // ===== Semantic (0x00-0x0F) - IMMUTABLE =====
    Synonym = 0x00,
    Antonym = 0x01,
    Hypernym = 0x02,        // IsA
    Hyponym = 0x03,         // TypeOf
    Meronym = 0x04,         // PartOf
    Holonym = 0x05,         // HasPart
    Troponym = 0x06,        // MannerOf
    Entailment = 0x07,
    Similar = 0x08,
    Also = 0x09,
    Attribute = 0x0A,
    Derivation = 0x0B,
    Domain = 0x0C,
    Topic = 0x0D,
    Usage = 0x0E,
    Region = 0x0F,

    // ===== Causal (0x10-0x1F) - LEARNABLE =====
    Cause = 0x10,
    Effect = 0x11,
    Precondition = 0x12,
    Postcondition = 0x13,
    EnabledBy = 0x14,
    DisabledBy = 0x15,
    PreventedBy = 0x16,
    Triggered = 0x17,
    Influences = 0x18,
    Correlates = 0x19,
    Depends = 0x1A,
    Produces = 0x1B,
    Consumes = 0x1C,
    Modifies = 0x1D,
    Maintains = 0x1E,
    Destroys = 0x1F,

    // ===== Temporal (0x20-0x2F) - LEARNABLE =====
    Before = 0x20,
    After = 0x21,
    During = 0x22,
    Overlaps = 0x23,
    Starts = 0x24,
    Finishes = 0x25,
    Meets = 0x26,
    Equals = 0x27,
    Simultaneous = 0x28,
    Sequential = 0x29,
    Parallel = 0x2A,
    Periodic = 0x2B,
    Continuous = 0x2C,
    Discrete = 0x2D,
    Instant = 0x2E,
    Extended = 0x2F,

    // ===== Spatial (0x30-0x3F) - IMMUTABLE =====
    Near = 0x30,
    Far = 0x31,
    Above = 0x32,
    Below = 0x33,
    Left = 0x34,
    Right = 0x35,
    Inside = 0x36,
    Outside = 0x37,
    Adjacent = 0x38,
    Overlapping = 0x39,
    Touching = 0x3A,
    Containing = 0x3B,
    Crossing = 0x3C,
    Behind = 0x3D,
    Front = 0x3E,
    Between = 0x3F,

    // ===== Logical (0x40-0x4F) - IMMUTABLE =====
    And = 0x40,
    Or = 0x41,
    Not = 0x42,
    Xor = 0x43,
    Implies = 0x44,
    Equivalent = 0x45,
    Contradicts = 0x46,
    Consistent = 0x47,
    Proves = 0x48,
    Disproves = 0x49,
    Assumes = 0x4A,
    Concludes = 0x4B,
    Necessary = 0x4C,
    Sufficient = 0x4D,
    Possible = 0x4E,
    Impossible = 0x4F,

    // ===== Associative (0x50-0x5F) - LEARNABLE =====
    AssociatedWith = 0x50,
    RelatedTo = 0x51,
    SimilarTo = 0x52,
    ContrastedWith = 0x53,
    ComparedTo = 0x54,
    DistinguishedFrom = 0x55,
    AlternativeTo = 0x56,
    SubstituteFor = 0x57,
    ComplementOf = 0x58,
    VariantOf = 0x59,
    VersionOf = 0x5A,
    ExampleOf = 0x5B,
    InstanceOf = 0x5C,
    KindOf = 0x5D,
    FormOf = 0x5E,
    ManifestationOf = 0x5F,

    // ===== Structural (0x60-0x6F) - IMMUTABLE =====
    PartOf = 0x60,
    HasPart = 0x61,
    MemberOf = 0x62,
    HasMember = 0x63,
    SubclassOf = 0x64,
    SuperclassOf = 0x65,
    Contains = 0x66,
    ContainedBy = 0x67,
    Comprises = 0x68,
    ComposedOf = 0x69,
    ElementOf = 0x6A,
    HasElement = 0x6B,
    CollectionOf = 0x6C,
    ItemIn = 0x6D,
    SegmentOf = 0x6E,
    Whole = 0x6F,

    // ===== Functional (0x70-0x7F) - LEARNABLE =====
    UsedFor = 0x70,
    UsedBy = 0x71,
    ToolFor = 0x72,
    MethodFor = 0x73,
    InputTo = 0x74,
    OutputFrom = 0x75,
    ResourceFor = 0x76,
    RequiredBy = 0x77,
    ProvidedBy = 0x78,
    CapableOf = 0x79,
    SupportsFunction = 0x7A,
    ImplementsFunction = 0x7B,
    InterfaceFor = 0x7C,
    ProtocolFor = 0x7D,
    StandardFor = 0x7E,
    OptimizedFor = 0x7F,

    // ===== Emotional (0x80-0x8F) - LEARNABLE =====
    Likes = 0x80,
    Dislikes = 0x81,
    Loves = 0x82,
    Hates = 0x83,
    Fears = 0x84,
    Trusts = 0x85,
    Distrusts = 0x86,
    Respects = 0x87,
    Admires = 0x88,
    Envies = 0x89,
    Sympathizes = 0x8A,
    Empathizes = 0x8B,
    Resents = 0x8C,
    Forgives = 0x8D,
    Blames = 0x8E,
    Grateful = 0x8F,

    // ===== Rule/Metaphor (0x90-0x9F) - IMMUTABLE =====
    Rule = 0x90,
    Exception = 0x91,
    Constraint = 0x92,
    Permission = 0x93,
    Prohibition = 0x94,
    Obligation = 0x95,
    Metaphor = 0x96,
    Analogy = 0x97,
    Symbol = 0x98,
    Represents = 0x99,
    Signifies = 0x9A,
    Indicates = 0x9B,
    Suggests = 0x9C,
    Connotes = 0x9D,
    Denotes = 0x9E,
    References = 0x9F,

    // ===== Dynamic (0xA0-0xAF) - LEARNABLE =====
    Becomes = 0xA0,
    Transforms = 0xA1,
    Evolves = 0xA2,
    Develops = 0xA3,
    Grows = 0xA4,
    Decays = 0xA5,
    Improves = 0xA6,
    Degrades = 0xA7,
    Strengthens = 0xA8,
    Weakens = 0xA9,
    Accelerates = 0xAA,
    Decelerates = 0xAB,
    Stabilizes = 0xAC,
    Destabilizes = 0xAD,
    Cycles = 0xAE,
    Alternates = 0xAF,
}

/// Connection flags (bit field)
pub mod connection_flags {
    pub const ACTIVE: u8 = 0x01;
    pub const PERSISTENT: u8 = 0x02;
    pub const BIDIRECTIONAL: u8 = 0x04;
    pub const INHIBITORY: u8 = 0x08;
    pub const MODIFIED: u8 = 0x10;
    pub const REINFORCED: u8 = 0x20;
    pub const DECAYING: u8 = 0x40;
    pub const USER_FLAG: u8 = 0x80;
}

/// Active level bitmask
pub mod active_levels {
    pub const L1_PHYSICAL: u8 = 0x01;
    pub const L2_SENSORY: u8 = 0x02;
    pub const L3_MOTOR: u8 = 0x04;
    pub const L4_EMOTIONAL: u8 = 0x08;
    pub const L5_COGNITIVE: u8 = 0x10;
    pub const L6_SOCIAL: u8 = 0x20;
    pub const L7_TEMPORAL: u8 = 0x40;
    pub const L8_ABSTRACT: u8 = 0x80;
}

/// Connection V3.0 - 64-byte learning-capable structure
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct ConnectionV3 {
    // ===== CORE FIELDS (32 bytes, v1.0 compatible) =====
    pub token_a_id: u32,
    pub token_b_id: u32,
    pub connection_type: u8,
    pub rigidity: u8,
    pub active_levels: u8,
    pub flags: u8,
    pub activation_count: u32,
    pub pull_strength: f32,
    pub preferred_distance: f32,
    pub created_at: u32,
    pub last_activation: u32,

    // ===== LEARNING EXTENSION (32 bytes, NEW in v3.0) =====
    pub mutability: u8,  // ConnectionMutability as u8
    pub confidence: u8,
    pub evidence_count: u16,
    pub last_update: u32,
    pub learning_rate: u8,
    pub decay_rate: u8,
    pub _padding1: u16,
    pub source_id: u32,
    pub reserved: [u8; 16],
}

impl ConnectionV3 {
    /// Create new connection with default learning parameters
    pub fn new(token_a: u32, token_b: u32) -> Self {
        // Ensure canonical order (a < b)
        let (a, b) = if token_a < token_b {
            (token_a, token_b)
        } else {
            (token_b, token_a)
        };

        let now = current_timestamp();

        Self {
            // Core fields
            token_a_id: a,
            token_b_id: b,
            connection_type: ConnectionType::AssociatedWith as u8,
            rigidity: 128,  // 0.5
            active_levels: 0,
            flags: 0,
            activation_count: 0,
            pull_strength: 0.0,
            preferred_distance: 1.0,
            created_at: now,
            last_activation: 0,

            // Learning fields (Learnable defaults)
            mutability: ConnectionMutability::Learnable as u8,
            confidence: 128,  // 0.5
            evidence_count: 0,
            last_update: now,
            learning_rate: 32,  // 0.125
            decay_rate: 16,     // 0.0625
            _padding1: 0,
            source_id: 0,  // Manual creation
            reserved: [0; 16],
        }
    }

    /// Set connection type (also adjusts mutability)
    pub fn set_connection_type(&mut self, conn_type: ConnectionType) {
        self.connection_type = conn_type as u8;
        self.mutability = guess_mutability(conn_type as u8) as u8;

        // Immutable connections always have full confidence
        if self.mutability == ConnectionMutability::Immutable as u8 {
            self.confidence = 255;
        }
    }

    /// Activate the connection (increment counter, update timestamp)
    pub fn activate(&mut self) {
        self.activation_count = self.activation_count.saturating_add(1);
        self.last_activation = current_timestamp();
        self.flags |= connection_flags::ACTIVE;

        // Reinforcement for learnable connections
        if self.mutability == ConnectionMutability::Learnable as u8 {
            self.flags |= connection_flags::REINFORCED;
            self.rigidity = self.rigidity.saturating_add(1);
        }
    }

    /// Check if connection can be modified by IntuitionEngine
    pub fn can_modify(&self) -> bool {
        self.mutability != ConnectionMutability::Immutable as u8
    }

    /// Update confidence based on observation outcome
    pub fn update_confidence(&mut self, success: bool) {
        if !self.can_modify() {
            return;  // Cannot modify immutable connections
        }

        let delta = self.learning_rate as f32 / 255.0;
        let current_conf = self.confidence as f32 / 255.0;

        let new_conf = if success {
            // Increase confidence (saturating at 1.0)
            (current_conf + delta).min(1.0)
        } else {
            // Decrease confidence (penalty is half of reward)
            (current_conf - delta * 0.5).max(0.0)
        };

        self.confidence = (new_conf * 255.0) as u8;

        if success {
            self.evidence_count = self.evidence_count.saturating_add(1);
        }

        self.last_update = current_timestamp();
        self.flags |= connection_flags::MODIFIED;
    }

    /// Apply decay for hypothesis connections (time-based)
    pub fn apply_decay(&mut self) {
        if self.mutability != ConnectionMutability::Hypothesis as u8 {
            return;  // Only hypotheses decay
        }

        let time_since_update = current_timestamp().saturating_sub(self.last_update);

        // Decay if no updates for more than 1 hour (3600 seconds)
        if time_since_update > 3600 {
            let decay_factor = self.decay_rate as f32 / 255.0;
            let current_conf = self.confidence as f32 / 255.0;
            let new_conf = current_conf * (1.0 - decay_factor);

            self.confidence = (new_conf * 255.0) as u8;
            self.flags |= connection_flags::DECAYING;

            // Mark for deletion if confidence drops below 10%
            if self.confidence < 25 {
                self.flags &= !connection_flags::ACTIVE;
            }
        }
    }

    /// Calculate force between tokens based on distance
    pub fn calculate_force(&self, current_distance: f32) -> f32 {
        let delta = self.preferred_distance - current_distance;
        let rigidity_factor = self.rigidity as f32 / 255.0;

        // Confidence affects force strength for learnable connections
        let confidence_factor = if self.can_modify() {
            self.confidence as f32 / 255.0
        } else {
            1.0  // Immutable connections always at full strength
        };

        delta * rigidity_factor * self.pull_strength * confidence_factor
    }
}

/// Get current Unix timestamp
fn current_timestamp() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}

/// Guess mutability based on connection type category
fn guess_mutability(conn_type: u8) -> ConnectionMutability {
    match conn_type {
        // Semantic - Immutable
        0x00..=0x0F => ConnectionMutability::Immutable,
        // Causal - Learnable
        0x10..=0x1F => ConnectionMutability::Learnable,
        // Temporal - Learnable
        0x20..=0x2F => ConnectionMutability::Learnable,
        // Spatial - Immutable
        0x30..=0x3F => ConnectionMutability::Immutable,
        // Logical - Immutable
        0x40..=0x4F => ConnectionMutability::Immutable,
        // Associative - Learnable
        0x50..=0x5F => ConnectionMutability::Learnable,
        // Structural - Immutable
        0x60..=0x6F => ConnectionMutability::Immutable,
        // Functional - Learnable
        0x70..=0x7F => ConnectionMutability::Learnable,
        // Emotional - Learnable
        0x80..=0x8F => ConnectionMutability::Learnable,
        // Rule/Metaphor - Immutable
        0x90..=0x9F => ConnectionMutability::Immutable,
        // Dynamic - Learnable
        0xA0..=0xAF => ConnectionMutability::Learnable,
        // Default - Learnable
        _ => ConnectionMutability::Learnable,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<ConnectionV3>(), 64);
    }

    #[test]
    fn test_mutability_semantics() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Immutable as u8;
        assert!(!conn.can_modify());

        conn.mutability = ConnectionMutability::Learnable as u8;
        assert!(conn.can_modify());

        conn.mutability = ConnectionMutability::Hypothesis as u8;
        assert!(conn.can_modify());
    }

    #[test]
    fn test_confidence_update() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Learnable as u8;
        conn.confidence = 128;
        conn.learning_rate = 25;  // ~0.1

        let old_conf = conn.confidence;
        conn.update_confidence(true);  // Success
        assert!(conn.confidence > old_conf);
        assert_eq!(conn.evidence_count, 1);

        let success_conf = conn.confidence;
        conn.update_confidence(false);  // Failure
        assert!(conn.confidence < success_conf);
    }

    #[test]
    fn test_immutable_cannot_update() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Immutable as u8;
        conn.confidence = 255;

        conn.update_confidence(false);  // Try to decrease
        assert_eq!(conn.confidence, 255);  // Should remain unchanged
    }

    #[test]
    fn test_decay() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Hypothesis as u8;
        conn.confidence = 100;
        conn.decay_rate = 64;  // 0.25
        conn.last_update = current_timestamp().saturating_sub(7200);  // 2 hours ago

        conn.apply_decay();
        assert!(conn.confidence < 100);
        assert_ne!(conn.flags & connection_flags::DECAYING, 0);
    }

    #[test]
    fn test_canonical_order() {
        let conn1 = ConnectionV3::new(5, 3);
        assert_eq!(conn1.token_a_id, 3);
        assert_eq!(conn1.token_b_id, 5);

        let conn2 = ConnectionV3::new(3, 5);
        assert_eq!(conn2.token_a_id, 3);
        assert_eq!(conn2.token_b_id, 5);
    }

    #[test]
    fn test_connection_type_sets_mutability() {
        let mut conn = ConnectionV3::new(1, 2);

        conn.set_connection_type(ConnectionType::Synonym);
        assert_eq!(conn.mutability, ConnectionMutability::Immutable as u8);
        assert_eq!(conn.confidence, 255);

        conn.set_connection_type(ConnectionType::Cause);
        assert_eq!(conn.mutability, ConnectionMutability::Learnable as u8);
    }

    #[test]
    fn test_activation() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Learnable as u8;

        assert_eq!(conn.activation_count, 0);
        let initial_rigidity = conn.rigidity;

        conn.activate();
        assert_eq!(conn.activation_count, 1);
        assert_ne!(conn.flags & connection_flags::ACTIVE, 0);
        assert_ne!(conn.flags & connection_flags::REINFORCED, 0);
        assert!(conn.rigidity > initial_rigidity);
    }
}
