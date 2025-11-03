//! HomeostasisAppraiser - Оценщик гомеостаза
//!
//! Задача: поддерживать стабильность системы, штрафуя за отклонения от целевых параметров.
//!
//! # Принцип работы
//!
//! Использует **квадратичный штраф** за отклонение от setpoint:
//! ```text
//! penalty = -k * (value - target)²
//! ```
//!
//! Оценивает:
//! - **Cognitive Load** (L4): целевой диапазон [0.3, 0.7]
//! - **Certainty** (L6): целевой диапазон [0.5, 0.9]
//!
//! # Example
//!
//! ```rust
//! use neurograph_core::{HomeostasisAppraiser, Appraiser, ADNA, ADNAProfile, ExperienceEvent};
//!
//! let appraiser = HomeostasisAppraiser::new();
//! let adna = ADNA::from_profile(ADNAProfile::Balanced);
//!
//! let mut event = ExperienceEvent::new(EventType::SystemStartup).with_state([0.5; 8]);
//! event.state[3] = 0.9; // High cognitive load (L4)
//!
//! let reward = appraiser.calculate_reward(&event, &adna);
//! // reward < 0 (penalty for deviation from target)
//! ```

use super::Appraiser;
use crate::adna::ADNA;
use crate::experience_stream::{ExperienceEvent, EventType};

/// HomeostasisAppraiser - maintains system stability
pub struct HomeostasisAppraiser {
    /// Penalty coefficient for deviations
    penalty_coefficient: f32,
}

impl HomeostasisAppraiser {
    /// Create new HomeostasisAppraiser
    pub fn new() -> Self {
        Self {
            penalty_coefficient: 1.0,
        }
    }

    /// Calculate cognitive load penalty
    ///
    /// Target: [0.3, 0.7] (comfortable cognitive load)
    /// L4 in state vector (index 3)
    fn cognitive_load_penalty(&self, event: &ExperienceEvent) -> f32 {
        if event.state.len() <= 3 {
            return 0.0;
        }

        let cognitive_load = event.state[3];
        let target_low = 0.3;
        let target_high = 0.7;

        // No penalty if within target range
        if cognitive_load >= target_low && cognitive_load <= target_high {
            return 0.0;
        }

        // Quadratic penalty for deviation
        let deviation = if cognitive_load < target_low {
            target_low - cognitive_load
        } else {
            cognitive_load - target_high
        };

        -self.penalty_coefficient * deviation * deviation
    }

    /// Calculate certainty penalty
    ///
    /// Target: [0.5, 0.9] (high certainty preferred)
    /// L6 in state vector (index 5)
    fn certainty_penalty(&self, event: &ExperienceEvent) -> f32 {
        if event.state.len() <= 5 {
            return 0.0;
        }

        let certainty = event.state[5];
        let target_low = 0.5;
        let target_high = 0.9;

        // No penalty if within target range
        if certainty >= target_low && certainty <= target_high {
            return 0.0;
        }

        // Quadratic penalty for deviation
        let deviation = if certainty < target_low {
            target_low - certainty
        } else {
            certainty - target_high
        };

        -self.penalty_coefficient * deviation * deviation
    }
}

impl Default for HomeostasisAppraiser {
    fn default() -> Self {
        Self::new()
    }
}

impl Appraiser for HomeostasisAppraiser {
    fn calculate_reward(&self, event: &ExperienceEvent, _adna: &ADNA) -> f32 {
        let cognitive_penalty = self.cognitive_load_penalty(event);
        let certainty_penalty = self.certainty_penalty(event);

        // Average of both penalties
        (cognitive_penalty + certainty_penalty) / 2.0
    }

    fn name(&self) -> &str {
        "HomeostasisAppraiser"
    }

    fn weight(&self, adna: &ADNA) -> f32 {
        adna.parameters.homeostasis_weight
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adna::ADNAProfile;

    #[test]
    fn test_homeostasis_appraiser_creation() {
        let appraiser = HomeostasisAppraiser::new();
        assert_eq!(appraiser.name(), "HomeostasisAppraiser");
    }

    #[test]
    fn test_cognitive_load_within_target() {
        let appraiser = HomeostasisAppraiser::new();
        let mut event = ExperienceEvent::new(EventType::SystemStartup)
            .with_state([0.0; 8]);
        event.state[3] = 0.5; // Within [0.3, 0.7]

        let penalty = appraiser.cognitive_load_penalty(&event);
        assert_eq!(penalty, 0.0);
    }

    #[test]
    fn test_cognitive_load_too_high() {
        let appraiser = HomeostasisAppraiser::new();
        let mut event = ExperienceEvent::new(EventType::SystemStartup)
            .with_state([0.0; 8]);
        event.state[3] = 0.9; // Above 0.7

        let penalty = appraiser.cognitive_load_penalty(&event);
        assert!(penalty < 0.0); // Penalty
        assert!(penalty.abs() > 0.0);
    }

    #[test]
    fn test_cognitive_load_too_low() {
        let appraiser = HomeostasisAppraiser::new();
        let mut event = ExperienceEvent::new(EventType::SystemStartup)
            .with_state([0.0; 8]);
        event.state[3] = 0.1; // Below 0.3

        let penalty = appraiser.cognitive_load_penalty(&event);
        assert!(penalty < 0.0); // Penalty
    }

    #[test]
    fn test_certainty_within_target() {
        let appraiser = HomeostasisAppraiser::new();
        let mut event = ExperienceEvent::new(EventType::SystemStartup)
            .with_state([0.0; 8]);
        event.state[5] = 0.7; // Within [0.5, 0.9]

        let penalty = appraiser.certainty_penalty(&event);
        assert_eq!(penalty, 0.0);
    }

    #[test]
    fn test_certainty_too_low() {
        let appraiser = HomeostasisAppraiser::new();
        let mut event = ExperienceEvent::new(EventType::SystemStartup)
            .with_state([0.0; 8]);
        event.state[5] = 0.2; // Below 0.5

        let penalty = appraiser.certainty_penalty(&event);
        assert!(penalty < 0.0); // Penalty
    }

    #[test]
    fn test_calculate_reward_with_adna() {
        let appraiser = HomeostasisAppraiser::new();
        let adna = ADNA::from_profile(ADNAProfile::Balanced);

        let mut event = ExperienceEvent::new(EventType::SystemStartup)
            .with_state([0.0; 8]);
        event.state[3] = 0.9; // High cognitive load
        event.state[5] = 0.2; // Low certainty

        let reward = appraiser.calculate_reward(&event, &adna);
        assert!(reward < 0.0); // Should be negative (penalty)
    }

    #[test]
    fn test_weight_from_adna() {
        let appraiser = HomeostasisAppraiser::new();
        let adna = ADNA::from_profile(ADNAProfile::Balanced);

        let weight = appraiser.weight(&adna);
        assert_eq!(weight, 0.25); // Balanced profile
    }

    #[test]
    fn test_quadratic_penalty() {
        let appraiser = HomeostasisAppraiser::new();

        // Small deviation
        let mut event1 = ExperienceEvent::new(EventType::SystemStartup)
            .with_state([0.0; 8]);
        event1.state[3] = 0.8; // deviation = 0.1

        // Large deviation
        let mut event2 = ExperienceEvent::new(EventType::SystemStartup)
            .with_state([0.0; 8]);
        event2.state[3] = 1.0; // deviation = 0.3

        let penalty1 = appraiser.cognitive_load_penalty(&event1);
        let penalty2 = appraiser.cognitive_load_penalty(&event2);

        // Penalty should grow quadratically
        // 0.1² = 0.01, 0.3² = 0.09
        assert!(penalty2.abs() > penalty1.abs() * 8.0);
    }
}
