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

/// Fields modifiable in Connection via proposals
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ConnectionField {
    Confidence,
    PullStrength,
    PreferredDistance,
    LearningRate,
    DecayRate,
}

/// Proposal for modifying a Connection (from IntuitionEngine)
#[derive(Debug, Clone)]
pub enum ConnectionProposal {
    /// Modify existing Connection field
    Modify {
        connection_id: u64,
        field: ConnectionField,
        old_value: f32,
        new_value: f32,
        justification: String,
        evidence_count: u16,
    },

    /// Create new hypothesis Connection
    Create {
        token_a_id: u32,
        token_b_id: u32,
        connection_type: u8,
        initial_strength: f32,
        initial_confidence: u8,
        justification: String,
    },

    /// Delete hypothesis Connection (low confidence)
    Delete {
        connection_id: u64,
        reason: String,
    },

    /// Promote Hypothesis → Learnable
    Promote {
        connection_id: u64,
        evidence_count: u16,
        justification: String,
    },
}

/// Proposal application error
#[derive(Debug, Clone, PartialEq)]
pub enum ProposalError {
    /// Cannot modify immutable connection
    ImmutableConnection,

    /// Invalid field value
    InvalidValue { field: String, value: f32 },

    /// Cannot promote non-hypothesis
    NotHypothesis,

    /// Cannot delete non-hypothesis
    CannotDelete,

    /// Evidence threshold not met
    InsufficientEvidence { required: u16, provided: u16 },

    /// Guardian rejected proposal (CDNA constraints violated)
    GuardianRejected { reason: String },
}

impl ConnectionV3 {
    /// Apply a proposal from IntuitionEngine (with validation)
    pub fn apply_proposal(&mut self, proposal: &ConnectionProposal) -> Result<(), ProposalError> {
        match proposal {
            ConnectionProposal::Modify {
                field,
                new_value,
                evidence_count,
                ..
            } => {
                // Check mutability
                if !self.can_modify() {
                    return Err(ProposalError::ImmutableConnection);
                }

                // Require minimum evidence for modifications
                if *evidence_count < 5 {
                    return Err(ProposalError::InsufficientEvidence {
                        required: 5,
                        provided: *evidence_count,
                    });
                }

                // Apply field change
                match field {
                    ConnectionField::Confidence => {
                        let conf_u8 = (*new_value * 255.0) as u8;
                        if conf_u8 > 255 {
                            return Err(ProposalError::InvalidValue {
                                field: "confidence".to_string(),
                                value: *new_value,
                            });
                        }
                        self.confidence = conf_u8;
                        self.evidence_count = self.evidence_count.saturating_add(*evidence_count);
                    }
                    ConnectionField::PullStrength => {
                        if new_value.abs() > 10.0 {
                            return Err(ProposalError::InvalidValue {
                                field: "pull_strength".to_string(),
                                value: *new_value,
                            });
                        }
                        self.pull_strength = *new_value;
                    }
                    ConnectionField::PreferredDistance => {
                        if *new_value < 0.01 || *new_value > 100.0 {
                            return Err(ProposalError::InvalidValue {
                                field: "preferred_distance".to_string(),
                                value: *new_value,
                            });
                        }
                        self.preferred_distance = *new_value;
                    }
                    ConnectionField::LearningRate => {
                        let lr_u8 = (*new_value * 255.0) as u8;
                        self.learning_rate = lr_u8;
                    }
                    ConnectionField::DecayRate => {
                        let dr_u8 = (*new_value * 255.0) as u8;
                        self.decay_rate = dr_u8;
                    }
                }

                self.last_update = current_timestamp();
                self.flags |= connection_flags::MODIFIED;
                Ok(())
            }

            ConnectionProposal::Promote { evidence_count, .. } => {
                // Only hypotheses can be promoted
                if self.mutability != ConnectionMutability::Hypothesis as u8 {
                    return Err(ProposalError::NotHypothesis);
                }

                // Require significant evidence
                if *evidence_count < 20 {
                    return Err(ProposalError::InsufficientEvidence {
                        required: 20,
                        provided: *evidence_count,
                    });
                }

                // Promote to Learnable
                self.mutability = ConnectionMutability::Learnable as u8;
                self.learning_rate = 32; // Slower learning
                self.decay_rate = 8; // Slower decay
                self.evidence_count = *evidence_count;
                self.last_update = current_timestamp();
                self.flags |= connection_flags::MODIFIED;

                Ok(())
            }

            ConnectionProposal::Delete { .. } => {
                // Only hypotheses can be deleted via proposal
                if self.mutability != ConnectionMutability::Hypothesis as u8 {
                    return Err(ProposalError::CannotDelete);
                }

                // Mark inactive (actual deletion handled by caller)
                self.flags &= !connection_flags::ACTIVE;
                Ok(())
            }

            ConnectionProposal::Create { .. } => {
                // Create is handled externally (not applied to existing connection)
                Ok(())
            }
        }
    }

    /// Create new Connection from CreateConnection proposal
    pub fn from_proposal(proposal: &ConnectionProposal) -> Option<Self> {
        match proposal {
            ConnectionProposal::Create {
                token_a_id,
                token_b_id,
                connection_type,
                initial_strength,
                initial_confidence,
                ..
            } => {
                let mut conn = Self::new(*token_a_id, *token_b_id);
                conn.connection_type = *connection_type;
                conn.mutability = ConnectionMutability::Hypothesis as u8;
                conn.pull_strength = *initial_strength;
                conn.confidence = *initial_confidence;
                conn.learning_rate = 128; // Fast learning for hypothesis
                conn.decay_rate = 32; // Moderate decay
                conn.source_id = 1; // Mark as IntuitionEngine-generated

                Some(conn)
            }
            _ => None,
        }
    }
}

/// Guardian validation helpers for ConnectionV3
pub mod guardian_validation {
    use super::*;

    /// Validate proposal against CDNA constraints (simplified for Phase 3)
    ///
    /// This is a lightweight validation that checks basic constraints.
    /// Full Guardian integration would check against actual CDNA instance.
    pub fn validate_proposal(
        connection: &ConnectionV3,
        proposal: &ConnectionProposal,
    ) -> Result<(), String> {
        match proposal {
            ConnectionProposal::Modify {
                field, new_value, ..
            } => {
                // Check pull_strength range (CDNA typical: -10.0 to +10.0)
                if matches!(field, ConnectionField::PullStrength) {
                    if new_value.abs() > 10.0 {
                        return Err(format!(
                            "Pull strength {} exceeds CDNA limit ±10.0",
                            new_value
                        ));
                    }
                }

                // Check preferred_distance range (CDNA typical: 0.01 to 100.0)
                if matches!(field, ConnectionField::PreferredDistance) {
                    if *new_value < 0.01 || *new_value > 100.0 {
                        return Err(format!(
                            "Preferred distance {} outside CDNA range 0.01-100.0",
                            new_value
                        ));
                    }
                }

                // Check confidence bounds
                if matches!(field, ConnectionField::Confidence) {
                    if *new_value < 0.0 || *new_value > 1.0 {
                        return Err(format!(
                            "Confidence {} outside valid range 0.0-1.0",
                            new_value
                        ));
                    }
                }

                Ok(())
            }

            ConnectionProposal::Create {
                connection_type,
                initial_strength,
                ..
            } => {
                // Validate connection type is known (0x00-0xAF range)
                if *connection_type > 0xAF {
                    return Err(format!(
                        "Unknown connection type 0x{:02X}",
                        connection_type
                    ));
                }

                // Validate initial strength
                if initial_strength.abs() > 10.0 {
                    return Err(format!(
                        "Initial strength {} exceeds CDNA limit ±10.0",
                        initial_strength
                    ));
                }

                Ok(())
            }

            ConnectionProposal::Delete { .. } => {
                // Deletion allowed only for hypotheses (already checked in apply_proposal)
                Ok(())
            }

            ConnectionProposal::Promote { .. } => {
                // Promotion validation (already handled in apply_proposal)
                Ok(())
            }
        }
    }

    /// Check if connection type is allowed by typical CDNA rules
    ///
    /// In full implementation, this would check against CDNA.allowed_connection_types
    /// For Phase 3, we accept all types in the 0x00-0xAF range
    pub fn is_connection_type_allowed(connection_type: u8) -> bool {
        connection_type <= 0xAF
    }

    /// Validate connection meets CDNA constraints after modification
    ///
    /// This is called after proposal is applied to ensure result is valid
    pub fn validate_connection_state(connection: &ConnectionV3) -> Result<(), String> {
        // Check pull_strength bounds
        if connection.pull_strength.abs() > 10.0 {
            return Err(format!(
                "Pull strength {} exceeds CDNA limit",
                connection.pull_strength
            ));
        }

        // Check preferred_distance bounds
        if connection.preferred_distance < 0.01 || connection.preferred_distance > 100.0 {
            return Err(format!(
                "Preferred distance {} outside CDNA range",
                connection.preferred_distance
            ));
        }

        // Check connection type is known
        if !is_connection_type_allowed(connection.connection_type) {
            return Err(format!(
                "Unknown connection type 0x{:02X}",
                connection.connection_type
            ));
        }

        Ok(())
    }
}

impl ConnectionV3 {
    /// Apply proposal with Guardian validation
    ///
    /// This extends apply_proposal() with CDNA constraint checking
    pub fn apply_proposal_with_guardian(
        &mut self,
        proposal: &ConnectionProposal,
    ) -> Result<(), ProposalError> {
        // Step 1: Guardian validation before changes
        guardian_validation::validate_proposal(self, proposal).map_err(|reason| {
            ProposalError::GuardianRejected { reason }
        })?;

        // Step 2: Apply proposal (includes mutability/evidence checks)
        self.apply_proposal(proposal)?;

        // Step 3: Validate final state against CDNA
        guardian_validation::validate_connection_state(self).map_err(|reason| {
            ProposalError::GuardianRejected { reason }
        })?;

        Ok(())
    }

    /// Create new Connection from proposal with Guardian validation
    pub fn from_proposal_with_guardian(
        proposal: &ConnectionProposal,
    ) -> Result<Self, ProposalError> {
        // Validate proposal first
        let temp_conn = Self::new(1, 2); // Temporary for validation context
        guardian_validation::validate_proposal(&temp_conn, proposal).map_err(|reason| {
            ProposalError::GuardianRejected { reason }
        })?;

        // Create connection
        let conn = Self::from_proposal(proposal).ok_or(ProposalError::GuardianRejected {
            reason: "Proposal type not Create".to_string(),
        })?;

        // Validate final state
        guardian_validation::validate_connection_state(&conn).map_err(|reason| {
            ProposalError::GuardianRejected { reason }
        })?;

        Ok(conn)
    }
}

/// Learning algorithms for Connection v3.0
///
/// Statistical analysis for automatic proposal generation based on experience.
/// These are simplified implementations for Phase 4 - full IntuitionEngine
/// would have more sophisticated pattern detection.
pub mod learning_stats {
    use super::*;

    /// Learning statistics for a connection
    #[derive(Debug, Clone)]
    pub struct ConnectionLearningStats {
        /// Total observations
        pub total_observations: u32,
        /// Successful activations
        pub successful_activations: u32,
        /// Failed activations
        pub failed_activations: u32,
        /// Current success rate (0.0-1.0)
        pub success_rate: f32,
        /// Temporal co-occurrence count
        pub temporal_cooccurrences: u32,
        /// Average time delta (for temporal connections)
        pub avg_time_delta_ms: i64,
    }

    impl ConnectionLearningStats {
        /// Create new stats
        pub fn new() -> Self {
            Self {
                total_observations: 0,
                successful_activations: 0,
                failed_activations: 0,
                success_rate: 0.0,
                temporal_cooccurrences: 0,
                avg_time_delta_ms: 0,
            }
        }

        /// Record successful activation
        pub fn record_success(&mut self) {
            self.successful_activations += 1;
            self.total_observations += 1;
            self.update_success_rate();
        }

        /// Record failed activation
        pub fn record_failure(&mut self) {
            self.failed_activations += 1;
            self.total_observations += 1;
            self.update_success_rate();
        }

        /// Update success rate
        fn update_success_rate(&mut self) {
            if self.total_observations > 0 {
                self.success_rate =
                    self.successful_activations as f32 / self.total_observations as f32;
            }
        }

        /// Record temporal co-occurrence
        pub fn record_cooccurrence(&mut self, time_delta_ms: i64) {
            self.temporal_cooccurrences += 1;
            // Running average
            let n = self.temporal_cooccurrences as i64;
            self.avg_time_delta_ms =
                ((self.avg_time_delta_ms * (n - 1)) + time_delta_ms) / n;
        }

        /// Check if sufficient evidence for proposal
        pub fn has_sufficient_evidence(&self, min_observations: u32) -> bool {
            self.total_observations >= min_observations
        }

        /// Generate confidence proposal if statistics warrant it
        pub fn generate_confidence_proposal(
            &self,
            connection: &ConnectionV3,
            min_observations: u32,
        ) -> Option<ConnectionProposal> {
            // Need minimum observations
            if !self.has_sufficient_evidence(min_observations) {
                return None;
            }

            // Only for learnable/hypothesis connections
            if connection.mutability == ConnectionMutability::Immutable as u8 {
                return None;
            }

            let current_confidence = connection.confidence as f32 / 255.0;
            let target_confidence = self.success_rate;

            // Only propose if significant difference (>10%)
            if (target_confidence - current_confidence).abs() < 0.1 {
                return None;
            }

            // Clamp to reasonable change
            let new_confidence = if target_confidence > current_confidence {
                (current_confidence + 0.2).min(target_confidence)
            } else {
                (current_confidence - 0.2).max(target_confidence)
            };

            Some(ConnectionProposal::Modify {
                connection_id: 0, // Would be set by caller
                field: ConnectionField::Confidence,
                old_value: current_confidence,
                new_value: new_confidence,
                justification: format!(
                    "Success rate {:.1}% over {} observations",
                    self.success_rate * 100.0,
                    self.total_observations
                ),
                evidence_count: self.total_observations as u16,
            })
        }

        /// Generate promote proposal if hypothesis is strong
        pub fn generate_promote_proposal(
            &self,
            connection: &ConnectionV3,
            min_observations: u32,
            min_success_rate: f32,
        ) -> Option<ConnectionProposal> {
            // Only for hypothesis connections
            if connection.mutability != ConnectionMutability::Hypothesis as u8 {
                return None;
            }

            // Need sufficient evidence
            if !self.has_sufficient_evidence(min_observations) {
                return None;
            }

            // Need good success rate
            if self.success_rate < min_success_rate {
                return None;
            }

            Some(ConnectionProposal::Promote {
                connection_id: 0, // Would be set by caller
                evidence_count: self.total_observations as u16,
                justification: format!(
                    "Strong evidence: {:.1}% success over {} trials",
                    self.success_rate * 100.0,
                    self.total_observations
                ),
            })
        }

        /// Generate delete proposal if hypothesis is weak
        pub fn generate_delete_proposal(
            &self,
            connection: &ConnectionV3,
            min_observations: u32,
            max_success_rate: f32,
        ) -> Option<ConnectionProposal> {
            // Only for hypothesis connections
            if connection.mutability != ConnectionMutability::Hypothesis as u8 {
                return None;
            }

            // Need sufficient evidence to conclude it's bad
            if !self.has_sufficient_evidence(min_observations) {
                return None;
            }

            // Success rate too low
            if self.success_rate > max_success_rate {
                return None;
            }

            Some(ConnectionProposal::Delete {
                connection_id: 0, // Would be set by caller
                reason: format!(
                    "Low success rate: {:.1}% over {} trials",
                    self.success_rate * 100.0,
                    self.total_observations
                ),
            })
        }
    }

    /// Temporal pattern for connection creation
    #[derive(Debug, Clone)]
    pub struct TemporalPattern {
        pub token_a_id: u32,
        pub token_b_id: u32,
        pub connection_type: u8,
        pub cooccurrence_count: u32,
        pub confidence: f32,
        pub avg_time_delta_ms: i64,
    }

    impl TemporalPattern {
        /// Generate create proposal from detected pattern
        pub fn generate_create_proposal(&self) -> Option<ConnectionProposal> {
            // Need minimum occurrences
            if self.cooccurrence_count < 5 {
                return None;
            }

            // Need reasonable confidence
            if self.confidence < 0.3 {
                return None;
            }

            Some(ConnectionProposal::Create {
                token_a_id: self.token_a_id,
                token_b_id: self.token_b_id,
                connection_type: self.connection_type,
                initial_strength: 1.0 + (self.confidence * 2.0), // 1.0-3.0 range
                initial_confidence: (self.confidence * 255.0) as u8,
                justification: format!(
                    "Temporal pattern: {} co-occurrences, avg Δt={}ms",
                    self.cooccurrence_count, self.avg_time_delta_ms
                ),
            })
        }
    }

    /// Detect temporal pattern between two tokens
    ///
    /// Simplified detection for Phase 4 - looks for tokens appearing in sequence.
    /// Full IntuitionEngine would analyze ExperienceStream events.
    pub fn detect_temporal_pattern(
        token_a_id: u32,
        token_b_id: u32,
        observations: &[(u32, u32, i64)], // (token_a, token_b, time_delta_ms)
        min_occurrences: u32,
    ) -> Option<TemporalPattern> {
        let mut count = 0;
        let mut total_delta: i64 = 0;

        for (a, b, delta) in observations {
            if *a == token_a_id && *b == token_b_id {
                count += 1;
                total_delta += delta;
            }
        }

        if count < min_occurrences {
            return None;
        }

        let avg_delta = total_delta / count as i64;
        let confidence = (count as f32 / observations.len() as f32).min(1.0);

        // Determine connection type based on time delta
        let connection_type = if avg_delta.abs() < 1000 {
            ConnectionType::Simultaneous as u8
        } else if avg_delta > 0 {
            ConnectionType::After as u8
        } else {
            ConnectionType::Before as u8
        };

        Some(TemporalPattern {
            token_a_id,
            token_b_id,
            connection_type,
            cooccurrence_count: count,
            confidence,
            avg_time_delta_ms: avg_delta,
        })
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

    // ===== Phase 2 Tests: Proposal System =====

    #[test]
    fn test_proposal_modify_confidence() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Learnable as u8;
        conn.confidence = 128;

        let proposal = ConnectionProposal::Modify {
            connection_id: 1,
            field: ConnectionField::Confidence,
            old_value: 0.5,
            new_value: 0.75,
            justification: "Test confidence increase".to_string(),
            evidence_count: 10,
        };

        let result = conn.apply_proposal(&proposal);
        assert!(result.is_ok());
        assert_eq!(conn.confidence, 191); // 0.75 * 255
        assert_eq!(conn.evidence_count, 10);
        assert_ne!(conn.flags & connection_flags::MODIFIED, 0);
    }

    #[test]
    fn test_proposal_modify_immutable_rejected() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Immutable as u8;

        let proposal = ConnectionProposal::Modify {
            connection_id: 1,
            field: ConnectionField::Confidence,
            old_value: 1.0,
            new_value: 0.5,
            justification: "Should fail".to_string(),
            evidence_count: 10,
        };

        let result = conn.apply_proposal(&proposal);
        assert_eq!(result, Err(ProposalError::ImmutableConnection));
    }

    #[test]
    fn test_proposal_insufficient_evidence() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Learnable as u8;

        let proposal = ConnectionProposal::Modify {
            connection_id: 1,
            field: ConnectionField::Confidence,
            old_value: 0.5,
            new_value: 0.75,
            justification: "Too few evidence".to_string(),
            evidence_count: 3, // < 5 minimum
        };

        let result = conn.apply_proposal(&proposal);
        assert_eq!(
            result,
            Err(ProposalError::InsufficientEvidence {
                required: 5,
                provided: 3
            })
        );
    }

    #[test]
    fn test_proposal_modify_pull_strength() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Learnable as u8;

        let proposal = ConnectionProposal::Modify {
            connection_id: 1,
            field: ConnectionField::PullStrength,
            old_value: 0.0,
            new_value: 5.0,
            justification: "Increase attraction".to_string(),
            evidence_count: 8,
        };

        let result = conn.apply_proposal(&proposal);
        assert!(result.is_ok());
        assert_eq!(conn.pull_strength, 5.0);
    }

    #[test]
    fn test_proposal_invalid_pull_strength() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Learnable as u8;

        let proposal = ConnectionProposal::Modify {
            connection_id: 1,
            field: ConnectionField::PullStrength,
            old_value: 0.0,
            new_value: 15.0, // > 10.0 max
            justification: "Too strong".to_string(),
            evidence_count: 8,
        };

        let result = conn.apply_proposal(&proposal);
        assert!(matches!(result, Err(ProposalError::InvalidValue { .. })));
    }

    #[test]
    fn test_proposal_promote_hypothesis() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Hypothesis as u8;
        conn.learning_rate = 128; // Fast learning
        conn.decay_rate = 32;

        let proposal = ConnectionProposal::Promote {
            connection_id: 1,
            evidence_count: 25,
            justification: "Strong evidence".to_string(),
        };

        let result = conn.apply_proposal(&proposal);
        assert!(result.is_ok());
        assert_eq!(conn.mutability, ConnectionMutability::Learnable as u8);
        assert_eq!(conn.learning_rate, 32); // Slower learning
        assert_eq!(conn.decay_rate, 8); // Slower decay
        assert_eq!(conn.evidence_count, 25);
    }

    #[test]
    fn test_proposal_promote_insufficient_evidence() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Hypothesis as u8;

        let proposal = ConnectionProposal::Promote {
            connection_id: 1,
            evidence_count: 15, // < 20 minimum
            justification: "Not enough".to_string(),
        };

        let result = conn.apply_proposal(&proposal);
        assert_eq!(
            result,
            Err(ProposalError::InsufficientEvidence {
                required: 20,
                provided: 15
            })
        );
    }

    #[test]
    fn test_proposal_promote_non_hypothesis() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Learnable as u8;

        let proposal = ConnectionProposal::Promote {
            connection_id: 1,
            evidence_count: 25,
            justification: "Should fail".to_string(),
        };

        let result = conn.apply_proposal(&proposal);
        assert_eq!(result, Err(ProposalError::NotHypothesis));
    }

    #[test]
    fn test_proposal_delete_hypothesis() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Hypothesis as u8;
        conn.flags = connection_flags::ACTIVE;

        let proposal = ConnectionProposal::Delete {
            connection_id: 1,
            reason: "Low confidence".to_string(),
        };

        let result = conn.apply_proposal(&proposal);
        assert!(result.is_ok());
        assert_eq!(conn.flags & connection_flags::ACTIVE, 0);
    }

    #[test]
    fn test_proposal_delete_non_hypothesis() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Learnable as u8;

        let proposal = ConnectionProposal::Delete {
            connection_id: 1,
            reason: "Should fail".to_string(),
        };

        let result = conn.apply_proposal(&proposal);
        assert_eq!(result, Err(ProposalError::CannotDelete));
    }

    #[test]
    fn test_from_proposal_create() {
        let proposal = ConnectionProposal::Create {
            token_a_id: 10,
            token_b_id: 20,
            connection_type: ConnectionType::Cause as u8,
            initial_strength: 3.5,
            initial_confidence: 64, // 0.25
            justification: "Pattern detected".to_string(),
        };

        let conn = ConnectionV3::from_proposal(&proposal);
        assert!(conn.is_some());

        let conn = conn.unwrap();
        assert_eq!(conn.token_a_id, 10);
        assert_eq!(conn.token_b_id, 20);
        assert_eq!(conn.connection_type, ConnectionType::Cause as u8);
        assert_eq!(conn.mutability, ConnectionMutability::Hypothesis as u8);
        assert_eq!(conn.pull_strength, 3.5);
        assert_eq!(conn.confidence, 64);
        assert_eq!(conn.learning_rate, 128); // Fast learning
        assert_eq!(conn.decay_rate, 32); // Moderate decay
        assert_eq!(conn.source_id, 1); // IntuitionEngine
    }

    #[test]
    fn test_from_proposal_non_create() {
        let proposal = ConnectionProposal::Delete {
            connection_id: 1,
            reason: "Test".to_string(),
        };

        let conn = ConnectionV3::from_proposal(&proposal);
        assert!(conn.is_none());
    }

    // ===== Phase 3 Tests: Guardian Integration =====

    #[test]
    fn test_guardian_validate_pull_strength_within_bounds() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Learnable as u8;

        let proposal = ConnectionProposal::Modify {
            connection_id: 1,
            field: ConnectionField::PullStrength,
            old_value: 0.0,
            new_value: 8.0, // Within ±10.0
            justification: "Within CDNA bounds".to_string(),
            evidence_count: 10,
        };

        let result = conn.apply_proposal_with_guardian(&proposal);
        assert!(result.is_ok());
        assert_eq!(conn.pull_strength, 8.0);
    }

    #[test]
    fn test_guardian_reject_excessive_pull_strength() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Learnable as u8;

        let proposal = ConnectionProposal::Modify {
            connection_id: 1,
            field: ConnectionField::PullStrength,
            old_value: 0.0,
            new_value: 12.0, // > 10.0 CDNA limit
            justification: "Too strong".to_string(),
            evidence_count: 10,
        };

        let result = conn.apply_proposal_with_guardian(&proposal);
        assert!(matches!(
            result,
            Err(ProposalError::GuardianRejected { .. })
        ));
    }

    #[test]
    fn test_guardian_validate_preferred_distance() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Learnable as u8;

        let proposal = ConnectionProposal::Modify {
            connection_id: 1,
            field: ConnectionField::PreferredDistance,
            old_value: 1.0,
            new_value: 50.0, // Within 0.01-100.0
            justification: "Valid distance".to_string(),
            evidence_count: 8,
        };

        let result = conn.apply_proposal_with_guardian(&proposal);
        assert!(result.is_ok());
        assert_eq!(conn.preferred_distance, 50.0);
    }

    #[test]
    fn test_guardian_reject_invalid_preferred_distance() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Learnable as u8;

        let proposal = ConnectionProposal::Modify {
            connection_id: 1,
            field: ConnectionField::PreferredDistance,
            old_value: 1.0,
            new_value: 150.0, // > 100.0 CDNA limit
            justification: "Too far".to_string(),
            evidence_count: 8,
        };

        let result = conn.apply_proposal_with_guardian(&proposal);
        assert!(matches!(
            result,
            Err(ProposalError::GuardianRejected { .. })
        ));
    }

    #[test]
    fn test_guardian_validate_confidence_bounds() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Learnable as u8;

        let proposal = ConnectionProposal::Modify {
            connection_id: 1,
            field: ConnectionField::Confidence,
            old_value: 0.5,
            new_value: 0.85, // Within 0.0-1.0
            justification: "Valid confidence".to_string(),
            evidence_count: 15,
        };

        let result = conn.apply_proposal_with_guardian(&proposal);
        assert!(result.is_ok());
    }

    #[test]
    fn test_guardian_reject_invalid_confidence() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Learnable as u8;

        let proposal = ConnectionProposal::Modify {
            connection_id: 1,
            field: ConnectionField::Confidence,
            old_value: 0.5,
            new_value: 1.5, // > 1.0 invalid
            justification: "Too high".to_string(),
            evidence_count: 10,
        };

        let result = conn.apply_proposal_with_guardian(&proposal);
        assert!(matches!(
            result,
            Err(ProposalError::GuardianRejected { .. })
        ));
    }

    #[test]
    fn test_guardian_validate_create_proposal() {
        let proposal = ConnectionProposal::Create {
            token_a_id: 5,
            token_b_id: 10,
            connection_type: ConnectionType::Cause as u8,
            initial_strength: 4.5, // Within ±10.0
            initial_confidence: 100,
            justification: "Valid creation".to_string(),
        };

        let result = ConnectionV3::from_proposal_with_guardian(&proposal);
        assert!(result.is_ok());

        let conn = result.unwrap();
        assert_eq!(conn.token_a_id, 5);
        assert_eq!(conn.token_b_id, 10);
        assert_eq!(conn.pull_strength, 4.5);
    }

    #[test]
    fn test_guardian_reject_invalid_connection_type() {
        let proposal = ConnectionProposal::Create {
            token_a_id: 5,
            token_b_id: 10,
            connection_type: 0xFF, // > 0xAF invalid
            initial_strength: 2.0,
            initial_confidence: 100,
            justification: "Invalid type".to_string(),
        };

        let result = ConnectionV3::from_proposal_with_guardian(&proposal);
        assert!(matches!(
            result,
            Err(ProposalError::GuardianRejected { .. })
        ));
    }

    #[test]
    fn test_guardian_reject_excessive_initial_strength() {
        let proposal = ConnectionProposal::Create {
            token_a_id: 5,
            token_b_id: 10,
            connection_type: ConnectionType::Cause as u8,
            initial_strength: 15.0, // > 10.0 CDNA limit
            initial_confidence: 100,
            justification: "Too strong".to_string(),
        };

        let result = ConnectionV3::from_proposal_with_guardian(&proposal);
        assert!(matches!(
            result,
            Err(ProposalError::GuardianRejected { .. })
        ));
    }

    #[test]
    fn test_guardian_validation_preserves_mutability_check() {
        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Immutable as u8;

        let proposal = ConnectionProposal::Modify {
            connection_id: 1,
            field: ConnectionField::Confidence,
            old_value: 1.0,
            new_value: 0.8, // Valid value
            justification: "Should fail on mutability".to_string(),
            evidence_count: 10,
        };

        // Should fail on mutability check (before Guardian)
        let result = conn.apply_proposal_with_guardian(&proposal);
        assert_eq!(result, Err(ProposalError::ImmutableConnection));
    }

    #[test]
    fn test_guardian_validation_state_check() {
        use guardian_validation::*;

        let mut conn = ConnectionV3::new(1, 2);
        conn.pull_strength = 5.0;
        conn.preferred_distance = 10.0;
        conn.connection_type = ConnectionType::Cause as u8;

        let result = validate_connection_state(&conn);
        assert!(result.is_ok());

        // Now violate constraint
        conn.pull_strength = 15.0;
        let result = validate_connection_state(&conn);
        assert!(result.is_err());
    }

    // ===== Phase 4 Tests: Learning Algorithms =====

    #[test]
    fn test_learning_stats_success_rate() {
        use learning_stats::*;

        let mut stats = ConnectionLearningStats::new();
        assert_eq!(stats.success_rate, 0.0);

        stats.record_success();
        stats.record_success();
        stats.record_failure();

        assert_eq!(stats.total_observations, 3);
        assert_eq!(stats.successful_activations, 2);
        assert_eq!(stats.failed_activations, 1);
        assert!((stats.success_rate - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_learning_stats_temporal_cooccurrence() {
        use learning_stats::*;

        let mut stats = ConnectionLearningStats::new();
        stats.record_cooccurrence(100);
        stats.record_cooccurrence(200);
        stats.record_cooccurrence(300);

        assert_eq!(stats.temporal_cooccurrences, 3);
        assert_eq!(stats.avg_time_delta_ms, 200);
    }

    #[test]
    fn test_generate_confidence_proposal_increase() {
        use learning_stats::*;

        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Learnable as u8;
        conn.confidence = 128; // 0.5

        let mut stats = ConnectionLearningStats::new();
        // Simulate 80% success rate
        for _ in 0..8 {
            stats.record_success();
        }
        for _ in 0..2 {
            stats.record_failure();
        }

        let proposal = stats.generate_confidence_proposal(&conn, 10);
        assert!(proposal.is_some());

        let proposal = proposal.unwrap();
        match proposal {
            ConnectionProposal::Modify {
                field, new_value, ..
            } => {
                assert!(matches!(field, ConnectionField::Confidence));
                assert!(new_value > 0.5); // Should increase
                assert!(new_value <= 0.8); // But not exceed success rate
            }
            _ => panic!("Expected Modify proposal"),
        }
    }

    #[test]
    fn test_generate_confidence_proposal_decrease() {
        use learning_stats::*;

        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Learnable as u8;
        conn.confidence = 200; // 0.78

        let mut stats = ConnectionLearningStats::new();
        // Simulate 40% success rate
        for _ in 0..4 {
            stats.record_success();
        }
        for _ in 0..6 {
            stats.record_failure();
        }

        let proposal = stats.generate_confidence_proposal(&conn, 10);
        assert!(proposal.is_some());

        let proposal = proposal.unwrap();
        match proposal {
            ConnectionProposal::Modify {
                field, new_value, ..
            } => {
                assert!(matches!(field, ConnectionField::Confidence));
                assert!(new_value < 0.78); // Should decrease
            }
            _ => panic!("Expected Modify proposal"),
        }
    }

    #[test]
    fn test_no_proposal_insufficient_evidence() {
        use learning_stats::*;

        let conn = ConnectionV3::new(1, 2);
        let mut stats = ConnectionLearningStats::new();
        stats.record_success();
        stats.record_success();

        let proposal = stats.generate_confidence_proposal(&conn, 10);
        assert!(proposal.is_none()); // Only 2 observations, need 10
    }

    #[test]
    fn test_no_proposal_small_difference() {
        use learning_stats::*;

        let mut conn = ConnectionV3::new(1, 2);
        conn.confidence = 128; // 0.5

        let mut stats = ConnectionLearningStats::new();
        // 55% success - only 5% difference
        for _ in 0..55 {
            stats.record_success();
        }
        for _ in 0..45 {
            stats.record_failure();
        }

        let proposal = stats.generate_confidence_proposal(&conn, 10);
        assert!(proposal.is_none()); // Difference < 10% threshold
    }

    #[test]
    fn test_no_proposal_for_immutable() {
        use learning_stats::*;

        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Immutable as u8;

        let mut stats = ConnectionLearningStats::new();
        for _ in 0..10 {
            stats.record_success();
        }

        let proposal = stats.generate_confidence_proposal(&conn, 10);
        assert!(proposal.is_none()); // Cannot modify immutable
    }

    #[test]
    fn test_generate_promote_proposal() {
        use learning_stats::*;

        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Hypothesis as u8;

        let mut stats = ConnectionLearningStats::new();
        for _ in 0..18 {
            stats.record_success();
        }
        for _ in 0..2 {
            stats.record_failure();
        }

        let proposal = stats.generate_promote_proposal(&conn, 20, 0.7);
        assert!(proposal.is_some());

        match proposal.unwrap() {
            ConnectionProposal::Promote { evidence_count, .. } => {
                assert_eq!(evidence_count, 20);
            }
            _ => panic!("Expected Promote proposal"),
        }
    }

    #[test]
    fn test_no_promote_low_success_rate() {
        use learning_stats::*;

        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Hypothesis as u8;

        let mut stats = ConnectionLearningStats::new();
        for _ in 0..10 {
            stats.record_success();
        }
        for _ in 0..10 {
            stats.record_failure();
        }

        let proposal = stats.generate_promote_proposal(&conn, 20, 0.7);
        assert!(proposal.is_none()); // Success rate only 50%, need 70%
    }

    #[test]
    fn test_generate_delete_proposal() {
        use learning_stats::*;

        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Hypothesis as u8;

        let mut stats = ConnectionLearningStats::new();
        for _ in 0..2 {
            stats.record_success();
        }
        for _ in 0..18 {
            stats.record_failure();
        }

        let proposal = stats.generate_delete_proposal(&conn, 20, 0.2);
        assert!(proposal.is_some());

        match proposal.unwrap() {
            ConnectionProposal::Delete { reason, .. } => {
                assert!(reason.contains("10.0%")); // 2/20 = 10%
            }
            _ => panic!("Expected Delete proposal"),
        }
    }

    #[test]
    fn test_no_delete_good_performance() {
        use learning_stats::*;

        let mut conn = ConnectionV3::new(1, 2);
        conn.mutability = ConnectionMutability::Hypothesis as u8;

        let mut stats = ConnectionLearningStats::new();
        for _ in 0..15 {
            stats.record_success();
        }
        for _ in 0..5 {
            stats.record_failure();
        }

        let proposal = stats.generate_delete_proposal(&conn, 20, 0.2);
        assert!(proposal.is_none()); // Success rate 75%, above 20% threshold
    }

    #[test]
    fn test_temporal_pattern_detection() {
        use learning_stats::*;

        let observations = vec![
            (1, 2, 100),
            (1, 2, 150),
            (1, 2, 120),
            (3, 4, 200),
            (1, 2, 130),
            (1, 2, 140),
        ];

        let pattern = detect_temporal_pattern(1, 2, &observations, 5);
        assert!(pattern.is_some());

        let pattern = pattern.unwrap();
        assert_eq!(pattern.token_a_id, 1);
        assert_eq!(pattern.token_b_id, 2);
        assert_eq!(pattern.cooccurrence_count, 5);
        assert_eq!(pattern.avg_time_delta_ms, 128); // (100+150+120+130+140)/5
    }

    #[test]
    fn test_temporal_pattern_connection_type() {
        use learning_stats::*;

        // Simultaneous (< 1000ms)
        let observations = vec![(1, 2, 50), (1, 2, 100), (1, 2, 150)];
        let pattern = detect_temporal_pattern(1, 2, &observations, 3).unwrap();
        assert_eq!(pattern.connection_type, ConnectionType::Simultaneous as u8);

        // After (> 0)
        let observations = vec![(3, 4, 2000), (3, 4, 3000), (3, 4, 2500)];
        let pattern = detect_temporal_pattern(3, 4, &observations, 3).unwrap();
        assert_eq!(pattern.connection_type, ConnectionType::After as u8);

        // Before (< 0)
        let observations = vec![(5, 6, -2000), (5, 6, -3000), (5, 6, -2500)];
        let pattern = detect_temporal_pattern(5, 6, &observations, 3).unwrap();
        assert_eq!(pattern.connection_type, ConnectionType::Before as u8);
    }

    #[test]
    fn test_temporal_pattern_create_proposal() {
        use learning_stats::*;

        let pattern = TemporalPattern {
            token_a_id: 10,
            token_b_id: 20,
            connection_type: ConnectionType::After as u8,
            cooccurrence_count: 8,
            confidence: 0.6,
            avg_time_delta_ms: 1500,
        };

        let proposal = pattern.generate_create_proposal();
        assert!(proposal.is_some());

        match proposal.unwrap() {
            ConnectionProposal::Create {
                token_a_id,
                token_b_id,
                connection_type,
                initial_strength,
                ..
            } => {
                assert_eq!(token_a_id, 10);
                assert_eq!(token_b_id, 20);
                assert_eq!(connection_type, ConnectionType::After as u8);
                assert!(initial_strength > 1.0); // Should be > 1.0
                assert!(initial_strength < 3.0); // Should be < 3.0
            }
            _ => panic!("Expected Create proposal"),
        }
    }

    #[test]
    fn test_no_create_insufficient_occurrences() {
        use learning_stats::*;

        let pattern = TemporalPattern {
            token_a_id: 10,
            token_b_id: 20,
            connection_type: ConnectionType::After as u8,
            cooccurrence_count: 3, // < 5 minimum
            confidence: 0.6,
            avg_time_delta_ms: 1500,
        };

        let proposal = pattern.generate_create_proposal();
        assert!(proposal.is_none());
    }

    #[test]
    fn test_no_create_low_confidence() {
        use learning_stats::*;

        let pattern = TemporalPattern {
            token_a_id: 10,
            token_b_id: 20,
            connection_type: ConnectionType::After as u8,
            cooccurrence_count: 8,
            confidence: 0.2, // < 0.3 threshold
            avg_time_delta_ms: 1500,
        };

        let proposal = pattern.generate_create_proposal();
        assert!(proposal.is_none());
    }
}
