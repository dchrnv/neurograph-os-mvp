// NeuroGraph OS - Curiosity Drive System v0.38.0
//
// Autonomous exploration driven by uncertainty, surprise, and novelty

pub mod config;
pub mod uncertainty;
pub mod surprise;
pub mod novelty;
pub mod exploration;
pub mod autonomous;

// Re-export key types
pub use config::{CuriosityConfig, ExplorationMode};
pub use exploration::{ExplorationTarget, ExplorationReason, ExplorationPriority, ExplorationQueue};
pub use autonomous::{AutonomousExplorer, AutonomousConfig, run_autonomous_exploration};

// Internal imports
use uncertainty::UncertaintyTracker;
use surprise::SurpriseHistory;
use novelty::NoveltyTracker;

use std::sync::Arc;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};

/// Combined curiosity score
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuriosityScore {
    /// Overall curiosity (0.0 to 1.0+)
    pub overall: f32,

    /// Contribution from uncertainty
    pub uncertainty: f32,

    /// Contribution from surprise
    pub surprise: f32,

    /// Contribution from novelty
    pub novelty: f32,

    /// Whether this triggers exploration (overall >= threshold)
    pub triggers_exploration: bool,
}

/// Context for curiosity calculation
#[derive(Debug, Clone)]
pub struct CuriosityContext {
    /// Current 8D state
    pub current_state: [f64; 8],

    /// Predicted next state (optional)
    pub predicted_state: Option<[f64; 8]>,

    /// Actual next state (optional, for surprise calculation)
    pub actual_state: Option<[f64; 8]>,

    /// Prediction accuracy (if known)
    pub prediction_accuracy: Option<f32>,
}

/// Curiosity-driven exploration system
pub struct CuriosityDrive {
    /// Configuration
    config: Arc<RwLock<CuriosityConfig>>,

    /// Uncertainty tracker
    uncertainty: Arc<RwLock<UncertaintyTracker>>,

    /// Surprise history
    surprise: Arc<RwLock<SurpriseHistory>>,

    /// Novelty tracker
    novelty: Arc<RwLock<NoveltyTracker>>,

    /// Exploration queue
    exploration_queue: Arc<RwLock<ExplorationQueue>>,

    /// Is autonomous exploration enabled
    autonomous_enabled: Arc<RwLock<bool>>,
}

impl CuriosityDrive {
    /// Create new curiosity drive
    pub fn new(config: CuriosityConfig) -> Self {
        let autonomous_enabled = config.enable_autonomous;
        let max_targets = config.max_exploration_targets;
        let history_size = config.surprise_history_size;

        Self {
            config: Arc::new(RwLock::new(config)),
            uncertainty: Arc::new(RwLock::new(UncertaintyTracker::new())),
            surprise: Arc::new(RwLock::new(SurpriseHistory::new(history_size))),
            novelty: Arc::new(RwLock::new(NoveltyTracker::new())),
            exploration_queue: Arc::new(RwLock::new(ExplorationQueue::new(max_targets))),
            autonomous_enabled: Arc::new(RwLock::new(autonomous_enabled)),
        }
    }

    /// Calculate curiosity score for a context
    pub fn calculate_curiosity(&self, context: &CuriosityContext) -> CuriosityScore {
        let config = self.config.read();

        // Calculate individual components
        let uncertainty = self.uncertainty.read().get_uncertainty(&context.current_state);

        let surprise = if let (Some(predicted), Some(actual)) = (context.predicted_state, context.actual_state) {
            self.surprise.write().calculate_surprise(predicted, actual)
        } else {
            self.surprise.read().current_surprise()
        };

        let novelty = self.novelty.write().calculate_novelty(&context.current_state);

        // Update uncertainty if we have prediction accuracy
        if let Some(accuracy) = context.prediction_accuracy {
            self.uncertainty.write().update(&context.current_state, accuracy);
        }

        // Weighted combination
        let overall = uncertainty * config.uncertainty_weight
            + surprise * config.surprise_weight
            + novelty * config.novelty_weight;

        let triggers_exploration = overall >= config.min_curiosity_score;

        CuriosityScore {
            overall,
            uncertainty,
            surprise,
            novelty,
            triggers_exploration,
        }
    }

    /// Add exploration target to queue
    pub fn add_exploration_target(&self, target: ExplorationTarget) {
        self.exploration_queue.write().push(target);
    }

    /// Get next exploration target
    pub fn get_next_target(&self) -> Option<ExplorationTarget> {
        self.exploration_queue.write().pop()
    }

    /// Peek at next target without removing
    pub fn peek_next_target(&self) -> Option<ExplorationTarget> {
        self.exploration_queue.read().peek().cloned()
    }

    /// Find uncertain regions for exploration
    pub fn find_uncertain_regions(&self, limit: usize) -> Vec<([f64; 8], f32)> {
        let uncertain_cells = self.uncertainty.read().get_most_uncertain(limit);

        // Convert cell keys to continuous states (center of cell)
        uncertain_cells
            .into_iter()
            .map(|(key, uncertainty)| {
                let state = [
                    key.coords[0] as f64,
                    key.coords[1] as f64,
                    key.coords[2] as f64,
                    key.coords[3] as f64,
                    key.coords[4] as f64,
                    key.coords[5] as f64,
                    key.coords[6] as f64,
                    key.coords[7] as f64,
                ];
                (state, uncertainty)
            })
            .collect()
    }

    /// Suggest exploration based on boredom threshold
    pub fn suggest_exploration(&self) -> Option<ExplorationTarget> {
        let config = self.config.read();

        // Check if current average confidence is below boredom threshold
        let stats = self.uncertainty.read().stats();
        if stats.avg_confidence < config.boredom_threshold {
            // Find most uncertain region
            let uncertain = self.find_uncertain_regions(1);
            if let Some((state, uncertainty)) = uncertain.first() {
                return Some(ExplorationTarget::new(
                    *state,
                    *uncertainty,
                    ExplorationReason::HighUncertainty,
                ));
            }
        }

        None
    }

    /// Cleanup old data
    pub fn cleanup(&self) {
        let config = self.config.read();

        let max_age = std::time::Duration::from_secs(config.max_cell_age_secs);
        let min_visits = config.min_cell_visits;

        // Cleanup uncertainty tracker
        self.uncertainty.write().cleanup_old_cells(max_age, min_visits);

        // Cleanup novelty tracker
        self.novelty.write().cleanup_old(max_age);
    }

    /// Enable/disable autonomous exploration
    pub fn set_autonomous(&self, enabled: bool) {
        *self.autonomous_enabled.write() = enabled;
    }

    /// Check if autonomous exploration is enabled
    pub fn is_autonomous_enabled(&self) -> bool {
        *self.autonomous_enabled.read()
    }

    /// Get comprehensive statistics
    pub fn stats(&self) -> CuriosityStats {
        CuriosityStats {
            uncertainty: self.uncertainty.read().stats(),
            surprise: self.surprise.read().stats(),
            novelty: self.novelty.read().stats(),
            exploration: self.exploration_queue.read().stats(),
            autonomous_enabled: *self.autonomous_enabled.read(),
        }
    }
}

impl Default for CuriosityDrive {
    fn default() -> Self {
        Self::new(CuriosityConfig::default())
    }
}

/// Combined statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuriosityStats {
    pub uncertainty: uncertainty::UncertaintyStats,
    pub surprise: surprise::SurpriseStats,
    pub novelty: novelty::NoveltyStats,
    pub exploration: exploration::ExplorationStats,
    pub autonomous_enabled: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_curiosity_drive_creation() {
        let drive = CuriosityDrive::default();
        assert!(!drive.is_autonomous_enabled());
    }

    #[test]
    fn test_calculate_curiosity_first_time() {
        let drive = CuriosityDrive::default();

        let context = CuriosityContext {
            current_state: [0.0; 8],
            predicted_state: None,
            actual_state: None,
            prediction_accuracy: None,
        };

        let score = drive.calculate_curiosity(&context);

        // First time seeing state = high novelty and uncertainty
        assert!(score.novelty > 0.9);
        assert!(score.uncertainty > 0.9);
    }

    #[test]
    fn test_calculate_curiosity_with_surprise() {
        let drive = CuriosityDrive::default();

        let context = CuriosityContext {
            current_state: [0.0; 8],
            predicted_state: Some([0.0; 8]),
            actual_state: Some([1.0; 8]), // Very different from predicted
            prediction_accuracy: Some(0.1), // Low accuracy
        };

        let score = drive.calculate_curiosity(&context);

        // Should have high surprise
        assert!(score.surprise > 0.5);
    }

    #[test]
    fn test_exploration_target_queue() {
        let drive = CuriosityDrive::default();

        let target = ExplorationTarget::new([0.0; 8], 0.8, ExplorationReason::Novel);
        drive.add_exploration_target(target);

        assert!(drive.get_next_target().is_some());
        assert!(drive.get_next_target().is_none()); // Queue empty
    }

    #[test]
    fn test_find_uncertain_regions() {
        let drive = CuriosityDrive::default();

        // Add some states
        let context = CuriosityContext {
            current_state: [0.0; 8],
            predicted_state: None,
            actual_state: None,
            prediction_accuracy: Some(0.5),
        };

        drive.calculate_curiosity(&context);

        let uncertain = drive.find_uncertain_regions(5);
        assert!(!uncertain.is_empty());
    }

    #[test]
    fn test_suggest_exploration() {
        let mut config = CuriosityConfig::default();
        config.boredom_threshold = 0.9; // Very high threshold

        let drive = CuriosityDrive::new(config);

        // Low confidence should trigger suggestion
        let suggestion = drive.suggest_exploration();
        // May or may not suggest depending on current state
        // Just checking it doesn't panic
        let _ = suggestion;
    }

    #[test]
    fn test_autonomous_toggle() {
        let drive = CuriosityDrive::default();

        assert!(!drive.is_autonomous_enabled());

        drive.set_autonomous(true);
        assert!(drive.is_autonomous_enabled());

        drive.set_autonomous(false);
        assert!(!drive.is_autonomous_enabled());
    }
}
