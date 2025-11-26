// NeuroGraph OS - Curiosity Drive Configuration v0.38.0
//
// Configuration for autonomous exploration system

use serde::{Deserialize, Serialize};

/// Exploration mode for curiosity-driven behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExplorationMode {
    /// Quiet mode - no REPL output during exploration
    Quiet,

    /// Verbose mode - detailed exploration logging
    Verbose,
}

impl Default for ExplorationMode {
    fn default() -> Self {
        Self::Quiet
    }
}

/// Configuration for CuriosityDrive system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuriosityConfig {
    /// Threshold below which boredom kicks in (0.0 to 1.0)
    /// Lower values = more easily bored, more exploration
    pub boredom_threshold: f32,

    /// Weight for uncertainty in curiosity score (0.0 to 1.0)
    pub uncertainty_weight: f32,

    /// Weight for surprise in curiosity score (0.0 to 1.0)
    pub surprise_weight: f32,

    /// Weight for novelty in curiosity score (0.0 to 1.0)
    pub novelty_weight: f32,

    /// Interval between autonomous exploration attempts (milliseconds)
    pub exploration_interval_ms: u64,

    /// Enable autonomous exploration
    pub enable_autonomous: bool,

    /// Exploration output mode
    pub exploration_mode: ExplorationMode,

    /// Maximum age for cells before cleanup (seconds)
    pub max_cell_age_secs: u64,

    /// Minimum visit count to keep a cell
    pub min_cell_visits: usize,

    /// Maximum number of exploration targets in queue
    pub max_exploration_targets: usize,

    /// Surprise history window size
    pub surprise_history_size: usize,

    /// Minimum curiosity score to trigger exploration (0.0 to 1.0)
    pub min_curiosity_score: f32,
}

impl Default for CuriosityConfig {
    fn default() -> Self {
        Self {
            // Moderate boredom - explore when confidence < 0.6
            boredom_threshold: 0.6,

            // Balanced weights (should sum to ~1.0)
            uncertainty_weight: 0.4,
            surprise_weight: 0.3,
            novelty_weight: 0.3,

            // Check every 5 seconds
            exploration_interval_ms: 5000,

            // Disabled by default
            enable_autonomous: false,

            // Quiet by default
            exploration_mode: ExplorationMode::Quiet,

            // Cleanup old cells after 1 hour
            max_cell_age_secs: 3600,

            // Keep cells visited at least twice
            min_cell_visits: 2,

            // Max 100 targets in exploration queue
            max_exploration_targets: 100,

            // Track last 50 surprise events
            surprise_history_size: 50,

            // Explore if curiosity >= 0.5
            min_curiosity_score: 0.5,
        }
    }
}

impl CuriosityConfig {
    /// Validate configuration values
    pub fn validate(&self) -> Result<(), String> {
        if self.boredom_threshold < 0.0 || self.boredom_threshold > 1.0 {
            return Err(format!(
                "boredom_threshold must be 0.0-1.0, got {}",
                self.boredom_threshold
            ));
        }

        if self.uncertainty_weight < 0.0 || self.uncertainty_weight > 1.0 {
            return Err(format!(
                "uncertainty_weight must be 0.0-1.0, got {}",
                self.uncertainty_weight
            ));
        }

        if self.surprise_weight < 0.0 || self.surprise_weight > 1.0 {
            return Err(format!(
                "surprise_weight must be 0.0-1.0, got {}",
                self.surprise_weight
            ));
        }

        if self.novelty_weight < 0.0 || self.novelty_weight > 1.0 {
            return Err(format!(
                "novelty_weight must be 0.0-1.0, got {}",
                self.novelty_weight
            ));
        }

        if self.min_curiosity_score < 0.0 || self.min_curiosity_score > 1.0 {
            return Err(format!(
                "min_curiosity_score must be 0.0-1.0, got {}",
                self.min_curiosity_score
            ));
        }

        let weight_sum = self.uncertainty_weight + self.surprise_weight + self.novelty_weight;
        if (weight_sum - 1.0).abs() > 0.1 {
            return Err(format!(
                "Weights should sum to ~1.0, got {}",
                weight_sum
            ));
        }

        Ok(())
    }

    /// Create config for high exploration (curious agent)
    pub fn high_exploration() -> Self {
        Self {
            boredom_threshold: 0.7,
            uncertainty_weight: 0.5,
            surprise_weight: 0.3,
            novelty_weight: 0.2,
            enable_autonomous: true,
            exploration_mode: ExplorationMode::Verbose,
            min_curiosity_score: 0.3,
            ..Default::default()
        }
    }

    /// Create config for low exploration (conservative agent)
    pub fn low_exploration() -> Self {
        Self {
            boredom_threshold: 0.4,
            uncertainty_weight: 0.3,
            surprise_weight: 0.4,
            novelty_weight: 0.3,
            enable_autonomous: false,
            min_curiosity_score: 0.7,
            ..Default::default()
        }
    }

    /// Create config for balanced exploration
    pub fn balanced() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_valid() {
        let config = CuriosityConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_high_exploration_valid() {
        let config = CuriosityConfig::high_exploration();
        assert!(config.validate().is_ok());
        assert!(config.enable_autonomous);
    }

    #[test]
    fn test_low_exploration_valid() {
        let config = CuriosityConfig::low_exploration();
        assert!(config.validate().is_ok());
        assert!(!config.enable_autonomous);
    }

    #[test]
    fn test_invalid_boredom_threshold() {
        let mut config = CuriosityConfig::default();
        config.boredom_threshold = 1.5;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_weights() {
        let mut config = CuriosityConfig::default();
        config.uncertainty_weight = 0.5;
        config.surprise_weight = 0.5;
        config.novelty_weight = 0.5; // Sum > 1.0
        assert!(config.validate().is_err());
    }
}
