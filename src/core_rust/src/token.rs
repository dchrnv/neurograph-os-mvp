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

/// Token V2.0 - Core data structure for NeuroGraph OS
///
/// A Token is the fundamental unit of information in NeuroGraph OS,
/// representing a discrete element in 8-dimensional semantic space.
///
/// Binary layout: 64 bytes total
/// - coordinates: 48 bytes (8 spaces × 3 axes × i16)
/// - id: 4 bytes (u32)
/// - flags: 2 bytes (u16)
/// - weight: 4 bytes (f32)
/// - field_radius: 1 byte (u8)
/// - field_strength: 1 byte (u8)
/// - timestamp: 4 bytes (u32)

use std::time::{SystemTime, UNIX_EPOCH};

/// Coordinate space identifiers
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoordinateSpace {
    L1Physical = 0,    // Physical 3D space
    L2Sensory = 1,     // Sensory perception
    L3Motor = 2,       // Motor control
    L4Emotional = 3,   // Emotional state (VAD model)
    L5Cognitive = 4,   // Cognitive processing
    L6Social = 5,      // Social interaction
    L7Temporal = 6,    // Temporal localization
    L8Abstract = 7,    // Abstract semantics
}

/// Entity types (stored in flags, bits 8-11)
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntityType {
    Undefined = 0x0000,
    Object = 0x0100,
    Event = 0x0200,
    State = 0x0300,
    Process = 0x0400,
    Concept = 0x0500,
    Relation = 0x0600,
    Pattern = 0x0700,
    Rule = 0x0800,
    Goal = 0x0900,
    Memory = 0x0A00,
    Sensor = 0x0B00,
    Actuator = 0x0C00,
    Controller = 0x0D00,
    Buffer = 0x0E00,
    Reserved = 0x0F00,
}

/// System flags (bits 0-7)
pub mod flags {
    pub const ACTIVE: u16 = 0x0001;        // Bit 0: Token is active
    pub const PERSISTENT: u16 = 0x0002;     // Bit 1: Token should be persisted
    pub const MUTABLE: u16 = 0x0004;        // Bit 2: Token can be modified
    pub const SYNCHRONIZED: u16 = 0x0008;   // Bit 3: Token is synchronized
    pub const COMPRESSED: u16 = 0x0010;     // Bit 4: Referenced data is compressed
    pub const ENCRYPTED: u16 = 0x0020;      // Bit 5: Referenced data is encrypted
    pub const DIRTY: u16 = 0x0040;          // Bit 6: Token modified but not saved
    pub const LOCKED: u16 = 0x0080;         // Bit 7: Token is locked

    /// Mask for entity type bits (8-11)
    pub const ENTITY_TYPE_MASK: u16 = 0x0F00;

    /// User flags (bits 12-15)
    pub const USER_1: u16 = 0x1000;
    pub const USER_2: u16 = 0x2000;
    pub const USER_3: u16 = 0x4000;
    pub const USER_4: u16 = 0x8000;
}

/// Coordinate scaling factors for each space
pub const SCALE_FACTORS: [f32; 8] = [
    100.0,    // L1: Physical (±327.67m)
    10000.0,  // L2: Sensory (±3.27)
    1000.0,   // L3: Motor (±32.7 m/s)
    10000.0,  // L4: Emotional (±3.27 VAD)
    10000.0,  // L5: Cognitive (±3.27)
    10000.0,  // L6: Social (±3.27)
    100.0,    // L7: Temporal (±327s for X,Y; 1000 for Z)
    10000.0,  // L8: Abstract (±3.27)
];

/// Token V2.0 structure (64 bytes, packed)
#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Token {
    /// 8 coordinate spaces × 3 axes (X, Y, Z) = 24 × i16 = 48 bytes
    /// Encoded as fixed-point integers with space-specific scaling
    pub coordinates: [[i16; 3]; 8],

    /// Unique identifier (4 bytes)
    /// Bits 0-23: local_id
    /// Bits 24-27: entity_type (embedded, redundant with flags)
    /// Bits 28-31: domain
    pub id: u32,

    /// Flags (2 bytes)
    /// Bits 0-7: system flags
    /// Bits 8-11: entity type
    /// Bits 12-15: user flags
    pub flags: u16,

    /// Weight/intensity (4 bytes, IEEE 754 f32)
    /// Range: 0.0 - 1.0 (typically)
    pub weight: f32,

    /// Field radius (1 byte, fixed-point)
    /// Scale: 100 → range 0.00 - 2.55
    pub field_radius: u8,

    /// Field strength (1 byte, fixed-point)
    /// Scale: 255 → range 0.0 - 1.0
    pub field_strength: u8,

    /// Unix timestamp (4 bytes)
    pub timestamp: u32,
}

// Compile-time size check
const _: () = assert!(std::mem::size_of::<Token>() == 64);

impl Token {
    /// Create a new Token with default values
    pub fn new(id: u32) -> Self {
        Self {
            coordinates: [[0; 3]; 8],
            id,
            flags: flags::ACTIVE,
            weight: 0.0,
            field_radius: 0,
            field_strength: 0,
            timestamp: Self::current_timestamp(),
        }
    }

    /// Create Token from 8D state vector (simplified for ActionController)
    ///
    /// This method creates a Token from a simple 8D float vector by placing
    /// each value in the X-axis of its corresponding coordinate space.
    /// Y and Z axes are set to 0.
    ///
    /// # Arguments
    /// * `id` - Token identifier
    /// * `state` - 8D state vector (one value per coordinate space)
    ///
    /// # Returns
    /// Token with state[i] mapped to coordinates[i][0] (X-axis)
    ///
    /// # Example
    /// ```ignore
    /// let state = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
    /// let token = Token::from_state_f32(0, &state);
    /// ```
    pub fn from_state_f32(id: u32, state: &[f32; 8]) -> Self {
        let mut token = Self::new(id);

        // Map each state value to the X-axis of its coordinate space
        for i in 0..8 {
            let space = match i {
                0 => CoordinateSpace::L1Physical,
                1 => CoordinateSpace::L2Sensory,
                2 => CoordinateSpace::L3Motor,
                3 => CoordinateSpace::L4Emotional,
                4 => CoordinateSpace::L5Cognitive,
                5 => CoordinateSpace::L6Social,
                6 => CoordinateSpace::L7Temporal,
                7 => CoordinateSpace::L8Abstract,
                _ => unreachable!(),
            };

            // Encode value to X-axis, Y=0, Z=0
            token.coordinates[i][0] = Self::encode_coordinate(state[i], space);
            token.coordinates[i][1] = 0;
            token.coordinates[i][2] = 0;
        }

        token
    }

    /// Extract 8D state vector from Token (inverse of from_state_f32)
    ///
    /// Extracts the X-axis value from each coordinate space.
    ///
    /// # Returns
    /// 8D state vector with decoded X-axis values
    pub fn to_state_f32(&self) -> [f32; 8] {
        let mut state = [0.0f32; 8];

        for i in 0..8 {
            let space = match i {
                0 => CoordinateSpace::L1Physical,
                1 => CoordinateSpace::L2Sensory,
                2 => CoordinateSpace::L3Motor,
                3 => CoordinateSpace::L4Emotional,
                4 => CoordinateSpace::L5Cognitive,
                5 => CoordinateSpace::L6Social,
                6 => CoordinateSpace::L7Temporal,
                7 => CoordinateSpace::L8Abstract,
                _ => unreachable!(),
            };

            state[i] = Self::decode_coordinate(self.coordinates[i][0], space);
        }

        state
    }

    /// Get current Unix timestamp
    pub fn current_timestamp() -> u32 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as u32
    }

    /// Encode a float coordinate to i16 with scaling
    pub fn encode_coordinate(value: f32, space: CoordinateSpace) -> i16 {
        let scale = SCALE_FACTORS[space as usize];
        let scaled = value * scale;
        scaled.clamp(i16::MIN as f32, i16::MAX as f32) as i16
    }

    /// Decode an i16 coordinate to float with scaling
    pub fn decode_coordinate(encoded: i16, space: CoordinateSpace) -> f32 {
        let scale = SCALE_FACTORS[space as usize];
        (encoded as f32) / scale
    }

    /// Set coordinates for a specific space
    pub fn set_coordinates(&mut self, space: CoordinateSpace, x: f32, y: f32, z: f32) {
        let idx = space as usize;
        self.coordinates[idx][0] = Self::encode_coordinate(x, space);
        self.coordinates[idx][1] = Self::encode_coordinate(y, space);
        self.coordinates[idx][2] = Self::encode_coordinate(z, space);
    }

    /// Get decoded coordinates for a specific space
    pub fn get_coordinates(&self, space: CoordinateSpace) -> [f32; 3] {
        let idx = space as usize;
        [
            Self::decode_coordinate(self.coordinates[idx][0], space),
            Self::decode_coordinate(self.coordinates[idx][1], space),
            Self::decode_coordinate(self.coordinates[idx][2], space),
        ]
    }

    /// Set entity type in flags
    pub fn set_entity_type(&mut self, entity_type: EntityType) {
        self.flags = (self.flags & !flags::ENTITY_TYPE_MASK) | (entity_type as u16);
    }

    /// Get entity type from flags
    pub fn get_entity_type(&self) -> EntityType {
        match self.flags & flags::ENTITY_TYPE_MASK {
            0x0000 => EntityType::Undefined,
            0x0100 => EntityType::Object,
            0x0200 => EntityType::Event,
            0x0300 => EntityType::State,
            0x0400 => EntityType::Process,
            0x0500 => EntityType::Concept,
            0x0600 => EntityType::Relation,
            0x0700 => EntityType::Pattern,
            0x0800 => EntityType::Rule,
            0x0900 => EntityType::Goal,
            0x0A00 => EntityType::Memory,
            0x0B00 => EntityType::Sensor,
            0x0C00 => EntityType::Actuator,
            0x0D00 => EntityType::Controller,
            0x0E00 => EntityType::Buffer,
            _ => EntityType::Reserved,
        }
    }

    /// Check if a flag is set
    pub fn has_flag(&self, flag: u16) -> bool {
        (self.flags & flag) != 0
    }

    /// Set a flag
    pub fn set_flag(&mut self, flag: u16) {
        self.flags |= flag;
    }

    /// Clear a flag
    pub fn clear_flag(&mut self, flag: u16) {
        self.flags &= !flag;
    }

    /// Encode field_radius (0.0 - 2.55 → 0 - 255)
    pub fn set_field_radius(&mut self, radius: f32) {
        self.field_radius = (radius * 100.0).clamp(0.0, 255.0) as u8;
    }

    /// Decode field_radius (0 - 255 → 0.0 - 2.55)
    pub fn get_field_radius(&self) -> f32 {
        (self.field_radius as f32) / 100.0
    }

    /// Encode field_strength (0.0 - 1.0 → 0 - 255)
    pub fn set_field_strength(&mut self, strength: f32) {
        self.field_strength = (strength * 255.0).clamp(0.0, 255.0) as u8;
    }

    /// Decode field_strength (0 - 255 → 0.0 - 1.0)
    pub fn get_field_strength(&self) -> f32 {
        (self.field_strength as f32) / 255.0
    }

    /// Extract local_id from ID (bits 0-23)
    pub fn local_id(&self) -> u32 {
        self.id & 0x00FFFFFF
    }

    /// Extract entity_type from ID (bits 24-27)
    pub fn id_entity_type(&self) -> u8 {
        ((self.id >> 24) & 0x0F) as u8
    }

    /// Extract domain from ID (bits 28-31)
    pub fn domain(&self) -> u8 {
        ((self.id >> 28) & 0x0F) as u8
    }

    /// Create ID from components
    pub fn create_id(local_id: u32, entity_type: u8, domain: u8) -> u32 {
        ((domain as u32 & 0x0F) << 28)
            | ((entity_type as u32 & 0x0F) << 24)
            | (local_id & 0x00FFFFFF)
    }

    /// Serialize to bytes (64 bytes)
    pub fn to_bytes(&self) -> [u8; 64] {
        unsafe { std::mem::transmute(*self) }
    }

    /// Deserialize from bytes (64 bytes)
    pub fn from_bytes(bytes: &[u8; 64]) -> Self {
        unsafe { std::mem::transmute(*bytes) }
    }

    /// Validate token structure
    pub fn validate(&self) -> Result<(), &'static str> {
        // Check ID is non-zero
        if self.id == 0 {
            return Err("Token ID cannot be zero");
        }

        // Check weight is in reasonable range
        if self.weight < 0.0 || self.weight > 100.0 {
            return Err("Weight out of range (0.0 - 100.0)");
        }

        // Check timestamp is not in the future
        let now = Self::current_timestamp();
        if self.timestamp > now + 3600 {
            // Allow 1 hour clock skew
            return Err("Timestamp is too far in the future");
        }

        Ok(())
    }
}

impl std::fmt::Debug for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Copy packed fields to avoid unaligned references
        let id = self.id;
        let flags = self.flags;
        let weight = self.weight;
        let timestamp = self.timestamp;

        f.debug_struct("Token")
            .field("id", &id)
            .field("local_id", &self.local_id())
            .field("entity_type", &self.get_entity_type())
            .field("domain", &self.domain())
            .field("flags", &format!("{:#06x}", flags))
            .field("weight", &weight)
            .field("field_radius", &self.get_field_radius())
            .field("field_strength", &self.get_field_strength())
            .field("timestamp", &timestamp)
            .field("coordinates", &"[8 spaces × 3 axes]")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_size() {
        assert_eq!(std::mem::size_of::<Token>(), 64);
    }

    #[test]
    fn test_token_new() {
        let token = Token::new(42);
        // Copy packed fields to avoid unaligned references
        let token_id = token.id;
        let token_weight = token.weight;
        assert_eq!(token_id, 42);
        assert_eq!(token.local_id(), 42);
        assert!(token.has_flag(flags::ACTIVE));
        assert_eq!(token_weight, 0.0);
    }

    #[test]
    fn test_coordinate_encoding() {
        // L1 Physical (scale = 100)
        let encoded = Token::encode_coordinate(10.5, CoordinateSpace::L1Physical);
        assert_eq!(encoded, 1050);
        let decoded = Token::decode_coordinate(encoded, CoordinateSpace::L1Physical);
        assert!((decoded - 10.5).abs() < 0.01);
    }

    #[test]
    fn test_set_get_coordinates() {
        let mut token = Token::new(1);
        token.set_coordinates(CoordinateSpace::L1Physical, 1.0, 2.0, 3.0);

        let coords = token.get_coordinates(CoordinateSpace::L1Physical);
        assert!((coords[0] - 1.0).abs() < 0.01);
        assert!((coords[1] - 2.0).abs() < 0.01);
        assert!((coords[2] - 3.0).abs() < 0.01);
    }

    #[test]
    fn test_entity_type() {
        let mut token = Token::new(1);
        token.set_entity_type(EntityType::Process);
        assert_eq!(token.get_entity_type(), EntityType::Process);
    }

    #[test]
    fn test_flags() {
        let mut token = Token::new(1);
        assert!(token.has_flag(flags::ACTIVE));

        token.set_flag(flags::PERSISTENT);
        assert!(token.has_flag(flags::PERSISTENT));

        token.clear_flag(flags::ACTIVE);
        assert!(!token.has_flag(flags::ACTIVE));
    }

    #[test]
    fn test_field_radius() {
        let mut token = Token::new(1);
        token.set_field_radius(1.25);
        let radius = token.get_field_radius();
        assert!((radius - 1.25).abs() < 0.01);
    }

    #[test]
    fn test_field_strength() {
        let mut token = Token::new(1);
        token.set_field_strength(0.75);
        let strength = token.get_field_strength();
        assert!((strength - 0.75).abs() < 0.01);
    }

    #[test]
    fn test_create_id() {
        let id = Token::create_id(12345, 5, 3);
        assert_eq!(id & 0x00FFFFFF, 12345); // local_id
        assert_eq!((id >> 24) & 0x0F, 5);   // entity_type
        assert_eq!((id >> 28) & 0x0F, 3);   // domain
    }

    #[test]
    fn test_serialization() {
        let mut token = Token::new(42);
        token.set_coordinates(CoordinateSpace::L1Physical, 1.0, 2.0, 3.0);
        token.set_entity_type(EntityType::Object);
        token.weight = 0.5;
        token.set_field_radius(1.5);
        token.set_field_strength(0.8);

        // Serialize and deserialize
        let bytes = token.to_bytes();
        let token2 = Token::from_bytes(&bytes);

        // Copy packed fields to avoid unaligned references
        let token2_id = token2.id;
        let token2_weight = token2.weight;
        assert_eq!(token2_id, 42);
        assert_eq!(token2_weight, 0.5);
        let coords = token2.get_coordinates(CoordinateSpace::L1Physical);
        assert!((coords[0] - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_validation() {
        let token = Token::new(42);
        assert!(token.validate().is_ok());

        let mut bad_token = Token::new(0);
        assert!(bad_token.validate().is_err());
    }
}
