//! Appraisers - Reward System для KEY Architecture
//!
//! Appraisers оценивают действия системы и начисляют reward в ExperienceEvents.
//! Каждый Appraiser смотрит на событие с разных точек зрения:
//!
//! - **HomeostasisAppraiser**: Штраф за отклонение от целевых параметров (cognitive load, certainty)
//! - **CuriosityAppraiser**: Reward за новизну и исследование
//! - **EfficiencyAppraiser**: Штраф за затраты ресурсов
//! - **GoalDirectedAppraiser**: Reward за достижение целей
//!
//! # Architecture
//!
//! ```
//! ExperienceEvent (reward=0)
//!     ↓
//! HomeostasisAppraiser → reward += homeostasis_component * weight
//! CuriosityAppraiser   → reward += curiosity_component * weight
//! EfficiencyAppraiser  → reward += efficiency_component * weight
//! GoalDirectedAppraiser→ reward += goal_component * weight
//!     ↓
//! ExperienceEvent (reward=final_sum)
//! ```
//!
//! Веса для каждого Appraiser берутся из ADNA parameters.

use crate::adna::ADNA;
use crate::experience_stream::{ExperienceEvent, EventType};

pub mod homeostasis;
pub mod curiosity;
pub mod efficiency;
pub mod goal_directed;

pub use homeostasis::HomeostasisAppraiser;
pub use curiosity::CuriosityAppraiser;
pub use efficiency::EfficiencyAppraiser;
pub use goal_directed::GoalDirectedAppraiser;

/// Appraiser trait - базовый интерфейс для всех оценщиков
///
/// Каждый Appraiser анализирует событие и возвращает reward компонент.
/// Финальный reward = сумма всех компонентов * веса из ADNA.
pub trait Appraiser: Send + Sync {
    /// Calculate reward component for this appraiser
    ///
    /// # Arguments
    /// * `event` - Experience event to evaluate
    /// * `adna` - ADNA parameters (содержит веса и setpoints)
    ///
    /// # Returns
    /// Reward component (может быть отрицательным)
    fn calculate_reward(&self, event: &ExperienceEvent, adna: &ADNA) -> f32;

    /// Get appraiser name
    fn name(&self) -> &str;

    /// Get appraiser weight from ADNA
    fn weight(&self, adna: &ADNA) -> f32;
}

/// Appraiser manager - координирует работу всех appraisers
///
/// # Example
///
/// ```rust
/// use neurograph_core::{AppraisersManager, ADNA, ADNAProfile, ExperienceEvent};
///
/// let manager = AppraisersManager::new();
/// let adna = ADNA::from_profile(ADNAProfile::Balanced);
///
/// let mut event = ExperienceEvent::new(vec![0.5; 8], 0);
/// manager.appraise_event(&mut event, &adna);
///
/// // event.reward now contains weighted sum of all appraisers
/// ```
pub struct AppraisersManager {
    homeostasis: HomeostasisAppraiser,
    curiosity: CuriosityAppraiser,
    efficiency: EfficiencyAppraiser,
    goal_directed: GoalDirectedAppraiser,
}

impl AppraisersManager {
    /// Create new AppraisersManager with all appraisers
    pub fn new() -> Self {
        Self {
            homeostasis: HomeostasisAppraiser::new(),
            curiosity: CuriosityAppraiser::new(),
            efficiency: EfficiencyAppraiser::new(),
            goal_directed: GoalDirectedAppraiser::new(),
        }
    }

    /// Appraise event with all appraisers and update reward
    ///
    /// Calculates weighted sum: reward = Σ(component_i * weight_i)
    pub fn appraise_event(&self, event: &mut ExperienceEvent, adna: &ADNA) {
        let mut total_reward = 0.0;

        // HomeostasisAppraiser
        let homeostasis_component = self.homeostasis.calculate_reward(event, adna);
        let homeostasis_weight = self.homeostasis.weight(adna);
        total_reward += homeostasis_component * homeostasis_weight;

        // CuriosityAppraiser
        let curiosity_component = self.curiosity.calculate_reward(event, adna);
        let curiosity_weight = self.curiosity.weight(adna);
        total_reward += curiosity_component * curiosity_weight;

        // EfficiencyAppraiser
        let efficiency_component = self.efficiency.calculate_reward(event, adna);
        let efficiency_weight = self.efficiency.weight(adna);
        total_reward += efficiency_component * efficiency_weight;

        // GoalDirectedAppraiser
        let goal_component = self.goal_directed.calculate_reward(event, adna);
        let goal_weight = self.goal_directed.weight(adna);
        total_reward += goal_component * goal_weight;

        // Update event reward
        event.reward = total_reward;
    }

    /// Get all appraisers as trait objects
    pub fn all_appraisers(&self) -> Vec<&dyn Appraiser> {
        vec![
            &self.homeostasis as &dyn Appraiser,
            &self.curiosity as &dyn Appraiser,
            &self.efficiency as &dyn Appraiser,
            &self.goal_directed as &dyn Appraiser,
        ]
    }
}

impl Default for AppraisersManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adna::ADNAProfile;

    #[test]
    fn test_appraisers_manager_creation() {
        let manager = AppraisersManager::new();
        assert_eq!(manager.all_appraisers().len(), 4);
    }

    #[test]
    fn test_appraise_event() {
        let manager = AppraisersManager::new();
        let adna = ADNA::from_profile(ADNAProfile::Balanced);

        let mut event = ExperienceEvent::new(EventType::SystemStartup)
            .with_state([0.5; 8]);
        assert_eq!(event.reward, 0.0);

        manager.appraise_event(&mut event, &adna);

        // Reward should be calculated (non-zero for most events)
        // Exact value depends on appraiser implementations
    }

    #[test]
    fn test_all_appraisers_have_names() {
        let manager = AppraisersManager::new();

        for appraiser in manager.all_appraisers() {
            assert!(!appraiser.name().is_empty());
        }
    }
}
