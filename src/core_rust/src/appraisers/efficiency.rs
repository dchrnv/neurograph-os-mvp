//! EfficiencyAppraiser - Оценщик эффективности
//!
//! Задача: штрафовать за затраты ресурсов (энергия, время).
//!
//! # Принцип работы
//!
//! Использует **линейный штраф** за затраты энергии:
//! ```text
//! penalty = -k * energy_cost
//! ```
//!
//! Оценивает:
//! - **Energy** (L7): чем выше затраты, тем больше штраф
//!
//! # Example
//!
//! ```rust
//! use neurograph_core::{EfficiencyAppraiser, Appraiser, ADNA, ADNAProfile, ExperienceEvent};
//!
//! let appraiser = EfficiencyAppraiser::new();
//! let adna = ADNA::from_profile(ADNAProfile::Balanced);
//!
//! let mut event = ExperienceEvent::new(EventType::SystemStartup).with_state([0.0; 8]);
//! event.state[6] = 0.8; // High energy cost (L7)
//!
//! let reward = appraiser.calculate_reward(&event, &adna);
//! // reward < 0 (penalty for high energy cost)
//! ```

use super::Appraiser;
use crate::adna::ADNA;
use crate::experience_stream::{ExperienceEvent, EventType};

/// EfficiencyAppraiser - penalizes resource waste
pub struct EfficiencyAppraiser {
    /// Penalty coefficient for energy cost
    penalty_coefficient: f32,
}

impl EfficiencyAppraiser {
    /// Create new EfficiencyAppraiser
    pub fn new() -> Self {
        Self {
            penalty_coefficient: 1.0,
        }
    }

    /// Calculate energy cost penalty
    ///
    /// L7 (Energy) in state vector (index 6)
    /// Linear penalty: higher energy cost = higher penalty
    fn energy_penalty(&self, event: &ExperienceEvent) -> f32 {
        if event.state.len() <= 6 {
            return 0.0;
        }

        let energy_cost = event.state[6];
        -self.penalty_coefficient * energy_cost
    }
}

impl Default for EfficiencyAppraiser {
    fn default() -> Self {
        Self::new()
    }
}

impl Appraiser for EfficiencyAppraiser {
    fn calculate_reward(&self, event: &ExperienceEvent, _adna: &ADNA) -> f32 {
        self.energy_penalty(event)
    }

    fn name(&self) -> &str {
        "EfficiencyAppraiser"
    }

    fn weight(&self, adna: &ADNA) -> f32 {
        adna.parameters.efficiency_weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adna::ADNAProfile;

    #[test]
    fn test_efficiency_appraiser_creation() {
        let appraiser = EfficiencyAppraiser::new();
        assert_eq!(appraiser.name(), "EfficiencyAppraiser");
    }

    #[test]
    fn test_energy_zero() {
        let appraiser = EfficiencyAppraiser::new();
        let mut event = ExperienceEvent::new(EventType::SystemStartup)
            .with_state([0.0; 8]);
        event.state[6] = 0.0; // No energy cost

        let penalty = appraiser.energy_penalty(&event);
        assert_eq!(penalty, 0.0);
    }

    #[test]
    fn test_energy_high() {
        let appraiser = EfficiencyAppraiser::new();
        let mut event = ExperienceEvent::new(EventType::SystemStartup)
            .with_state([0.0; 8]);
        event.state[6] = 0.8; // High energy cost

        let penalty = appraiser.energy_penalty(&event);
        assert!(penalty < 0.0); // Should be negative
        assert!((penalty + 0.8).abs() < 0.001);
    }

    #[test]
    fn test_energy_linear_scaling() {
        let appraiser = EfficiencyAppraiser::new();

        let mut event1 = ExperienceEvent::new(EventType::SystemStartup)
            .with_state([0.0; 8]);
        event1.state[6] = 0.2;

        let mut event2 = ExperienceEvent::new(EventType::SystemStartup)
            .with_state([0.0; 8]);
        event2.state[6] = 0.4;

        let penalty1 = appraiser.energy_penalty(&event1);
        let penalty2 = appraiser.energy_penalty(&event2);

        // Should be linear: penalty2 = 2 * penalty1
        assert!((penalty2 / penalty1 - 2.0).abs() < 0.001);
    }

    #[test]
    fn test_calculate_reward_with_adna() {
        let appraiser = EfficiencyAppraiser::new();
        let adna = ADNA::from_profile(ADNAProfile::Balanced);

        let mut event = ExperienceEvent::new(EventType::SystemStartup)
            .with_state([0.0; 8]);
        event.state[6] = 0.5; // Medium energy cost

        let reward = appraiser.calculate_reward(&event, &adna);
        assert!(reward < 0.0); // Should be negative (penalty)
    }

    #[test]
    fn test_weight_from_adna() {
        let appraiser = EfficiencyAppraiser::new();
        let adna = ADNA::from_profile(ADNAProfile::Balanced);

        let weight = appraiser.weight(&adna);
        assert_eq!(weight, 0.25); // Balanced profile
    }

    #[test]
    fn test_short_state_vector() {
        let appraiser = EfficiencyAppraiser::new();
        let event = ExperienceEvent::new(EventType::SystemStartup)
            .with_state([0.5, 0.5, 0.5, 0.5, 0.5, 0.0, 0.0, 0.0]); // Only 5 elements

        let penalty = appraiser.energy_penalty(&event);
        assert_eq!(penalty, 0.0); // Graceful handling
    }

    #[test]
    fn test_low_energy_small_penalty() {
        let appraiser = EfficiencyAppraiser::new();

        let mut event = ExperienceEvent::new(EventType::SystemStartup)
            .with_state([0.0; 8]);
        event.state[6] = 0.1; // Low energy cost

        let penalty = appraiser.energy_penalty(&event);
        assert!(penalty < 0.0);
        assert!(penalty > -0.2); // Small penalty
    }
}
