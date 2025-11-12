/// L1-L8 Coordinate System for NeuroGraph OS
///
/// This module defines the semantic meaning of the 8-dimensional state space
/// used throughout NeuroGraph OS. Each coordinate (L1-L8) represents a specific
/// aspect of system state.
///
/// # Architecture
///
/// - `state: [f32; 8]` in ExperienceEvent maps to L1-L8 coordinates
/// - `action: [f32; 8]` represents actions in the same 8D space
/// - All coordinates normalized to [0.0, 1.0] except L7 Valence [-1.0, 1.0]

use crate::experience_stream::ExperienceEvent;

/// L1-L8 Coordinate indices in state[]/action[] arrays
///
/// # Semantic Meanings:
///
/// - **L1 Existence**: Physical presence, activation level, "being there"
/// - **L2 Novelty**: Sensory novelty (0.0 = known/familiar, 1.0 = completely new)
/// - **L3 Velocity**: Motor activity, speed of change, movement
/// - **L4 Attention**: Focus level, attentional resource allocation
/// - **L5 Cognitive Load**: Mental effort, complexity, working memory usage
/// - **L6 Certainty**: Confidence, predictability, epistemic certainty
/// - **L7 Valence**: Emotional value (negative to positive)
/// - **L8 Coherence**: Logical consistency, structural integrity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CoordinateIndex {
    /// L1: Physical presence/activation (0.0 = absent, 1.0 = fully present)
    L1Existence = 0,

    /// L2: Sensory novelty (0.0 = known, 1.0 = completely new)
    L2Novelty = 1,

    /// L3: Motor activity/velocity (0.0 = static, 1.0 = maximum speed)
    L3Velocity = 2,

    /// L4: Attention/focus level (0.0 = no attention, 1.0 = full focus)
    L4Attention = 3,

    /// L5: Cognitive load/complexity (0.0 = minimal, 1.0 = maximum)
    L5CognitiveLoad = 4,

    /// L6: Certainty/predictability (0.0 = uncertain, 1.0 = certain)
    L6Certainty = 5,

    /// L7: Emotional valence (-1.0 = negative, 0.0 = neutral, 1.0 = positive)
    L7Valence = 6,

    /// L8: Logical coherence (0.0 = incoherent, 1.0 = fully coherent)
    L8Coherence = 7,
}

impl CoordinateIndex {
    /// Convert coordinate enum to array index
    #[inline]
    pub const fn as_usize(self) -> usize {
        self as usize
    }

    /// Get all coordinate indices in order
    pub const fn all() -> [CoordinateIndex; 8] {
        [
            CoordinateIndex::L1Existence,
            CoordinateIndex::L2Novelty,
            CoordinateIndex::L3Velocity,
            CoordinateIndex::L4Attention,
            CoordinateIndex::L5CognitiveLoad,
            CoordinateIndex::L6Certainty,
            CoordinateIndex::L7Valence,
            CoordinateIndex::L8Coherence,
        ]
    }

    /// Get coordinate name for logging/debugging
    pub const fn name(self) -> &'static str {
        match self {
            CoordinateIndex::L1Existence => "L1_Existence",
            CoordinateIndex::L2Novelty => "L2_Novelty",
            CoordinateIndex::L3Velocity => "L3_Velocity",
            CoordinateIndex::L4Attention => "L4_Attention",
            CoordinateIndex::L5CognitiveLoad => "L5_CognitiveLoad",
            CoordinateIndex::L6Certainty => "L6_Certainty",
            CoordinateIndex::L7Valence => "L7_Valence",
            CoordinateIndex::L8Coherence => "L8_Coherence",
        }
    }

    /// Get expected range for this coordinate
    pub const fn range(self) -> (f32, f32) {
        match self {
            CoordinateIndex::L7Valence => (-1.0, 1.0), // Only valence can be negative
            _ => (0.0, 1.0),
        }
    }
}

/// Extension trait for ExperienceEvent to access L1-L8 coordinates
///
/// # Examples
///
/// ```
/// use neurograph_core::{ExperienceEvent, CoordinateExt};
///
/// let event = ExperienceEvent::default();
/// let novelty = event.l2_novelty();
/// let cognitive_load = event.l5_cognitive_load();
/// ```
pub trait CoordinateExt {
    // === State Coordinates (from state[]) ===

    /// L1: Existence - physical presence/activation level
    fn l1_existence(&self) -> f32;

    /// L2: Novelty - how new/unknown this stimulus is
    fn l2_novelty(&self) -> f32;

    /// L3: Velocity - speed of movement/change
    fn l3_velocity(&self) -> f32;

    /// L4: Attention - current focus level
    fn l4_attention(&self) -> f32;

    /// L5: Cognitive Load - mental effort/complexity
    fn l5_cognitive_load(&self) -> f32;

    /// L6: Certainty - confidence/predictability
    fn l6_certainty(&self) -> f32;

    /// L7: Valence - emotional value (can be negative)
    fn l7_valence(&self) -> f32;

    /// L8: Coherence - logical consistency
    fn l8_coherence(&self) -> f32;

    // === Action Coordinates (from action[]) ===

    /// L3 Action: Acceleration - desired change in velocity
    fn l3_acceleration(&self) -> f32;

    /// L4 Action: Attention shift - change in focus
    fn l4_attention_shift(&self) -> f32;

    /// L5 Action: Cognitive effort - intended mental investment
    fn l5_cognitive_effort(&self) -> f32;

    // === Utility Methods ===

    /// Get any coordinate by index from state
    fn get_state_coordinate(&self, coord: CoordinateIndex) -> f32;

    /// Get any coordinate by index from action
    fn get_action_coordinate(&self, coord: CoordinateIndex) -> f32;

    /// Get all state coordinates as array
    fn get_all_state_coordinates(&self) -> [f32; 8];

    /// Get all action coordinates as array
    fn get_all_action_coordinates(&self) -> [f32; 8];

    /// Check if state coordinate is within expected range
    fn is_valid_state_coordinate(&self, coord: CoordinateIndex) -> bool;

    /// Check if action coordinate is within expected range
    fn is_valid_action_coordinate(&self, coord: CoordinateIndex) -> bool;
}

impl CoordinateExt for ExperienceEvent {
    // === State Coordinates Implementation ===

    #[inline]
    fn l1_existence(&self) -> f32 {
        self.state[CoordinateIndex::L1Existence.as_usize()]
    }

    #[inline]
    fn l2_novelty(&self) -> f32 {
        self.state[CoordinateIndex::L2Novelty.as_usize()]
    }

    #[inline]
    fn l3_velocity(&self) -> f32 {
        self.state[CoordinateIndex::L3Velocity.as_usize()]
    }

    #[inline]
    fn l4_attention(&self) -> f32 {
        self.state[CoordinateIndex::L4Attention.as_usize()]
    }

    #[inline]
    fn l5_cognitive_load(&self) -> f32 {
        self.state[CoordinateIndex::L5CognitiveLoad.as_usize()]
    }

    #[inline]
    fn l6_certainty(&self) -> f32 {
        self.state[CoordinateIndex::L6Certainty.as_usize()]
    }

    #[inline]
    fn l7_valence(&self) -> f32 {
        self.state[CoordinateIndex::L7Valence.as_usize()]
    }

    #[inline]
    fn l8_coherence(&self) -> f32 {
        self.state[CoordinateIndex::L8Coherence.as_usize()]
    }

    // === Action Coordinates Implementation ===

    #[inline]
    fn l3_acceleration(&self) -> f32 {
        self.action[CoordinateIndex::L3Velocity.as_usize()]
    }

    #[inline]
    fn l4_attention_shift(&self) -> f32 {
        self.action[CoordinateIndex::L4Attention.as_usize()]
    }

    #[inline]
    fn l5_cognitive_effort(&self) -> f32 {
        self.action[CoordinateIndex::L5CognitiveLoad.as_usize()]
    }

    // === Utility Methods Implementation ===

    #[inline]
    fn get_state_coordinate(&self, coord: CoordinateIndex) -> f32 {
        self.state[coord.as_usize()]
    }

    #[inline]
    fn get_action_coordinate(&self, coord: CoordinateIndex) -> f32 {
        self.action[coord.as_usize()]
    }

    #[inline]
    fn get_all_state_coordinates(&self) -> [f32; 8] {
        self.state
    }

    #[inline]
    fn get_all_action_coordinates(&self) -> [f32; 8] {
        self.action
    }

    fn is_valid_state_coordinate(&self, coord: CoordinateIndex) -> bool {
        let value = self.get_state_coordinate(coord);
        let (min, max) = coord.range();
        value >= min && value <= max
    }

    fn is_valid_action_coordinate(&self, coord: CoordinateIndex) -> bool {
        let value = self.get_action_coordinate(coord);
        let (min, max) = coord.range();
        value >= min && value <= max
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coordinate_index_conversion() {
        assert_eq!(CoordinateIndex::L1Existence.as_usize(), 0);
        assert_eq!(CoordinateIndex::L2Novelty.as_usize(), 1);
        assert_eq!(CoordinateIndex::L8Coherence.as_usize(), 7);
    }

    #[test]
    fn test_coordinate_ranges() {
        assert_eq!(CoordinateIndex::L1Existence.range(), (0.0, 1.0));
        assert_eq!(CoordinateIndex::L7Valence.range(), (-1.0, 1.0));
        assert_eq!(CoordinateIndex::L8Coherence.range(), (0.0, 1.0));
    }

    #[test]
    fn test_coordinate_names() {
        assert_eq!(CoordinateIndex::L1Existence.name(), "L1_Existence");
        assert_eq!(CoordinateIndex::L2Novelty.name(), "L2_Novelty");
        assert_eq!(CoordinateIndex::L5CognitiveLoad.name(), "L5_CognitiveLoad");
    }

    #[test]
    fn test_all_coordinates() {
        let all = CoordinateIndex::all();
        assert_eq!(all.len(), 8);
        assert_eq!(all[0], CoordinateIndex::L1Existence);
        assert_eq!(all[7], CoordinateIndex::L8Coherence);
    }

    #[test]
    fn test_coordinate_ext_state() {
        let mut event = ExperienceEvent::default();
        event.state[0] = 0.5; // L1 Existence
        event.state[1] = 0.8; // L2 Novelty
        event.state[4] = 0.6; // L5 Cognitive Load

        assert_eq!(event.l1_existence(), 0.5);
        assert_eq!(event.l2_novelty(), 0.8);
        assert_eq!(event.l5_cognitive_load(), 0.6);
    }

    #[test]
    fn test_coordinate_ext_action() {
        let mut event = ExperienceEvent::default();
        event.action[2] = 0.3; // L3 Acceleration
        event.action[3] = 0.7; // L4 Attention shift

        assert_eq!(event.l3_acceleration(), 0.3);
        assert_eq!(event.l4_attention_shift(), 0.7);
    }

    #[test]
    fn test_validation() {
        let mut event = ExperienceEvent::default();

        // Valid positive coordinate
        event.state[0] = 0.5;
        assert!(event.is_valid_state_coordinate(CoordinateIndex::L1Existence));

        // Invalid (out of range)
        event.state[0] = 1.5;
        assert!(!event.is_valid_state_coordinate(CoordinateIndex::L1Existence));

        // Valence can be negative
        event.state[6] = -0.5;
        assert!(event.is_valid_state_coordinate(CoordinateIndex::L7Valence));

        // But not other coordinates
        event.state[0] = -0.1;
        assert!(!event.is_valid_state_coordinate(CoordinateIndex::L1Existence));
    }

    #[test]
    fn test_get_all_coordinates() {
        let mut event = ExperienceEvent::default();
        for i in 0..8 {
            event.state[i] = i as f32 * 0.1;
            event.action[i] = i as f32 * 0.05;
        }

        let states = event.get_all_state_coordinates();
        let actions = event.get_all_action_coordinates();

        assert_eq!(states[0], 0.0);
        assert_eq!(states[7], 0.7);
        assert_eq!(actions[0], 0.0);
        assert_eq!(actions[7], 0.35);
    }
}