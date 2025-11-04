//! GoalDirectedAppraiser - Оценщик целенаправленности
//!
//! Задача: поощрять прогресс к целям.
//!
//! # Принцип работы
//!
//! Использует **линейный reward** за прогресс к цели:
//! ```text
//! reward = k * goal_progress
//! ```
//!
//! Оценивает:
//! - **Goal Progress** (L8): чем выше, тем больше reward
//!
//! # Example
//!
//! ```rust
//! use neurograph_core::{GoalDirectedAppraiser, Appraiser, ADNA, ADNAProfile, ExperienceEvent};
//!
//! let appraiser = GoalDirectedAppraiser::new();
//! let adna = ADNA::from_profile(ADNAProfile::Balanced);
//!
//! let mut event = ExperienceEvent::new(EventType::SystemStartup).with_state([0.0; 8]);
//! event.state[7] = 0.8; // High goal progress (L8)
//!
//! let reward = appraiser.calculate_reward(&event, &adna);
//! // reward > 0 (reward for high goal progress)
//! ```

use super::Appraiser;
use crate::adna::ADNA;
use crate::experience_stream::{EventType, ExperienceEvent};

/// GoalDirectedAppraiser - rewards goal achievement
pub struct GoalDirectedAppraiser {
    /// Reward coefficient for goal progress
    reward_coefficient: f32,
}

impl GoalDirectedAppraiser {
    /// Create new GoalDirectedAppraiser
    pub fn new() -> Self {
        Self {
            reward_coefficient: 1.0,
        }
    }

    /// Calculate goal progress reward
    ///
    /// L8 (Goal Progress) in state vector (index 7)
    /// Linear reward: higher goal progress = higher reward
    fn goal_progress_reward(&self, event: &ExperienceEvent) -> f32 {
        if event.state.len() <= 7 {
            return 0.0;
        }

        let goal_progress = event.state[7];
        self.reward_coefficient * goal_progress
    }
}

impl Default for GoalDirectedAppraiser {
    fn default() -> Self {
        Self::new()
    }
}

impl Appraiser for GoalDirectedAppraiser {
    fn calculate_reward(&self, event: &ExperienceEvent, _adna: &ADNA) -> f32 {
        self.goal_progress_reward(event)
    }

    fn name(&self) -> &str {
        "GoalDirectedAppraiser"
    }

    fn weight(&self, adna: &ADNA) -> f32 {
        adna.parameters.goal_weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adna::ADNAProfile;

    #[test]
    fn test_goal_directed_appraiser_creation() {
        let appraiser = GoalDirectedAppraiser::new();
        assert_eq!(appraiser.name(), "GoalDirectedAppraiser");
    }

    #[test]
    fn test_goal_progress_zero() {
        let appraiser = GoalDirectedAppraiser::new();
        let mut event = ExperienceEvent::new(EventType::SystemStartup).with_state([0.0; 8]);
        event.state[7] = 0.0; // No goal progress

        let reward = appraiser.goal_progress_reward(&event);
        assert_eq!(reward, 0.0);
    }

    #[test]
    fn test_goal_progress_high() {
        let appraiser = GoalDirectedAppraiser::new();
        let mut event = ExperienceEvent::new(EventType::SystemStartup).with_state([0.0; 8]);
        event.state[7] = 0.8; // High goal progress

        let reward = appraiser.goal_progress_reward(&event);
        assert!(reward > 0.0);
        assert!((reward - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_goal_progress_linear_scaling() {
        let appraiser = GoalDirectedAppraiser::new();

        let mut event1 = ExperienceEvent::new(EventType::SystemStartup).with_state([0.0; 8]);
        event1.state[7] = 0.25;

        let mut event2 = ExperienceEvent::new(EventType::SystemStartup).with_state([0.0; 8]);
        event2.state[7] = 0.5;

        let reward1 = appraiser.goal_progress_reward(&event1);
        let reward2 = appraiser.goal_progress_reward(&event2);

        // Should be linear: reward2 = 2 * reward1
        assert!((reward2 / reward1 - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_calculate_reward_with_adna() {
        let appraiser = GoalDirectedAppraiser::new();
        let adna = ADNA::from_profile(ADNAProfile::Balanced);

        let mut event = ExperienceEvent::new(EventType::SystemStartup).with_state([0.0; 8]);
        event.state[7] = 0.7; // Good goal progress

        let reward = appraiser.calculate_reward(&event, &adna);
        assert!(reward > 0.0);
    }

    #[test]
    fn test_weight_from_adna_balanced() {
        let appraiser = GoalDirectedAppraiser::new();
        let adna = ADNA::from_profile(ADNAProfile::Balanced);

        let weight = appraiser.weight(&adna);
        assert_eq!(weight, 0.25); // Balanced profile
    }

    #[test]
    fn test_short_state_vector() {
        let appraiser = GoalDirectedAppraiser::new();
        let event = ExperienceEvent::new(EventType::SystemStartup)
            .with_state([0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.0, 0.0]); // Only 6 elements

        let reward = appraiser.goal_progress_reward(&event);
        assert_eq!(reward, 0.0); // Graceful handling
    }

    #[test]
    fn test_medium_goal_progress() {
        let appraiser = GoalDirectedAppraiser::new();

        let mut event = ExperienceEvent::new(EventType::SystemStartup).with_state([0.0; 8]);
        event.state[7] = 0.5; // Medium goal progress

        let reward = appraiser.goal_progress_reward(&event);
        assert!(reward > 0.0);
        assert!((reward - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_complete_goal() {
        let appraiser = GoalDirectedAppraiser::new();

        let mut event = ExperienceEvent::new(EventType::SystemStartup).with_state([0.0; 8]);
        event.state[7] = 1.0; // Complete goal progress

        let reward = appraiser.goal_progress_reward(&event);
        assert!((reward - 1.0).abs() < 0.001);
    }
}
