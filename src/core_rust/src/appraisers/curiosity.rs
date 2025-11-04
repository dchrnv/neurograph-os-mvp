//! CuriosityAppraiser - Оценщик любопытства
//!
//! Задача: поощрять исследование и новизну.
//!
//! # Принцип работы
//!
//! Использует **линейный reward** за новизну:
//! ```text
//! reward = k * novelty_score
//! ```
//!
//! Оценивает:
//! - **Novelty** (L2): чем выше, тем больше reward
//!
//! # Example
//!
//! ```rust
//! use neurograph_core::{CuriosityAppraiser, Appraiser, ADNA, ADNAProfile, ExperienceEvent};
//!
//! let appraiser = CuriosityAppraiser::new();
//! let adna = ADNA::from_profile(ADNAProfile::Curious);
//!
//! let mut event = ExperienceEvent::new(EventType::SystemStartup).with_state([0.0; 8]);
//! event.state[1] = 0.9; // High novelty (L2)
//!
//! let reward = appraiser.calculate_reward(&event, &adna);
//! // reward > 0 (reward for high novelty)
//! ```

use super::Appraiser;
use crate::adna::ADNA;
use crate::experience_stream::{EventType, ExperienceEvent};

/// CuriosityAppraiser - encourages exploration and novelty
pub struct CuriosityAppraiser {
    /// Reward coefficient for novelty
    reward_coefficient: f32,
}

impl CuriosityAppraiser {
    /// Create new CuriosityAppraiser
    pub fn new() -> Self {
        Self {
            reward_coefficient: 1.0,
        }
    }

    /// Calculate novelty reward
    ///
    /// L2 (Novelty) in state vector (index 1)
    /// Linear reward: higher novelty = higher reward
    fn novelty_reward(&self, event: &ExperienceEvent) -> f32 {
        if event.state.len() <= 1 {
            return 0.0;
        }

        let novelty = event.state[1];
        self.reward_coefficient * novelty
    }
}

impl Default for CuriosityAppraiser {
    fn default() -> Self {
        Self::new()
    }
}

impl Appraiser for CuriosityAppraiser {
    fn calculate_reward(&self, event: &ExperienceEvent, _adna: &ADNA) -> f32 {
        self.novelty_reward(event)
    }

    fn name(&self) -> &str {
        "CuriosityAppraiser"
    }

    fn weight(&self, adna: &ADNA) -> f32 {
        adna.parameters.curiosity_weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adna::ADNAProfile;

    #[test]
    fn test_curiosity_appraiser_creation() {
        let appraiser = CuriosityAppraiser::new();
        assert_eq!(appraiser.name(), "CuriosityAppraiser");
    }

    #[test]
    fn test_novelty_zero() {
        let appraiser = CuriosityAppraiser::new();
        let mut event = ExperienceEvent::new(EventType::SystemStartup).with_state([0.0; 8]);
        event.state[1] = 0.0; // No novelty

        let reward = appraiser.novelty_reward(&event);
        assert_eq!(reward, 0.0);
    }

    #[test]
    fn test_novelty_high() {
        let appraiser = CuriosityAppraiser::new();
        let mut event = ExperienceEvent::new(EventType::SystemStartup).with_state([0.0; 8]);
        event.state[1] = 0.9; // High novelty

        let reward = appraiser.novelty_reward(&event);
        assert!(reward > 0.0);
        assert!((reward - 0.9).abs() < 0.001);
    }

    #[test]
    fn test_novelty_linear_scaling() {
        let appraiser = CuriosityAppraiser::new();

        let mut event1 = ExperienceEvent::new(EventType::SystemStartup).with_state([0.0; 8]);
        event1.state[1] = 0.3;

        let mut event2 = ExperienceEvent::new(EventType::SystemStartup).with_state([0.0; 8]);
        event2.state[1] = 0.6;

        let reward1 = appraiser.novelty_reward(&event1);
        let reward2 = appraiser.novelty_reward(&event2);

        // Should be linear: reward2 = 2 * reward1
        assert!((reward2 / reward1 - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_calculate_reward_with_adna() {
        let appraiser = CuriosityAppraiser::new();
        let adna = ADNA::from_profile(ADNAProfile::Curious);

        let mut event = ExperienceEvent::new(EventType::SystemStartup).with_state([0.0; 8]);
        event.state[1] = 0.8; // High novelty

        let reward = appraiser.calculate_reward(&event, &adna);
        assert!(reward > 0.0);
    }

    #[test]
    fn test_weight_from_adna_balanced() {
        let appraiser = CuriosityAppraiser::new();
        let adna = ADNA::from_profile(ADNAProfile::Balanced);

        let weight = appraiser.weight(&adna);
        assert_eq!(weight, 0.25); // Balanced profile
    }

    #[test]
    fn test_weight_from_adna_curious() {
        let appraiser = CuriosityAppraiser::new();
        let adna = ADNA::from_profile(ADNAProfile::Curious);

        let weight = appraiser.weight(&adna);
        assert_eq!(weight, 0.5); // Curious profile has higher curiosity weight
    }

    #[test]
    fn test_short_state_vector() {
        let appraiser = CuriosityAppraiser::new();
        let event = ExperienceEvent::new(EventType::SystemStartup)
            .with_state([0.5, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0]); // Only 1 element

        let reward = appraiser.novelty_reward(&event);
        assert_eq!(reward, 0.0); // Graceful handling
    }
}
