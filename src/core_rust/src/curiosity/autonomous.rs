// NeuroGraph OS - Autonomous Exploration v0.38.0
//
// Background exploration loop for curiosity-driven discovery

use crate::curiosity::{CuriosityDrive, ExplorationTarget, ExplorationMode};
use crate::action_controller::ActionController;
use std::sync::Arc;
use std::time::Duration;
use tokio::time;

/// Configuration for autonomous exploration loop
#[derive(Debug, Clone)]
pub struct AutonomousConfig {
    /// Interval between exploration cycles
    pub exploration_interval: Duration,

    /// Interval between cleanup operations
    pub cleanup_interval: Duration,

    /// Whether to log exploration events
    pub verbose: bool,
}

impl Default for AutonomousConfig {
    fn default() -> Self {
        Self {
            exploration_interval: Duration::from_secs(5),
            cleanup_interval: Duration::from_secs(60),
            verbose: false,
        }
    }
}

/// Result of an exploration cycle
#[derive(Debug, Clone)]
pub struct ExplorationCycle {
    /// Target that was explored
    pub target: ExplorationTarget,

    /// Whether exploration was successful
    pub success: bool,

    /// Duration of exploration
    pub duration: Duration,
}

/// Autonomous exploration loop
pub struct AutonomousExplorer {
    /// Curiosity drive
    curiosity: Arc<CuriosityDrive>,

    /// Configuration
    config: AutonomousConfig,

    /// Running state
    running: Arc<tokio::sync::RwLock<bool>>,
}

impl AutonomousExplorer {
    /// Create new autonomous explorer
    pub fn new(curiosity: Arc<CuriosityDrive>, config: AutonomousConfig) -> Self {
        Self {
            curiosity,
            config,
            running: Arc::new(tokio::sync::RwLock::new(false)),
        }
    }

    /// Start autonomous exploration loop
    pub async fn start(&self, controller: Arc<ActionController>) {
        *self.running.write().await = true;

        let mut exploration_ticker = time::interval(self.config.exploration_interval);
        let mut cleanup_ticker = time::interval(self.config.cleanup_interval);

        loop {
            tokio::select! {
                _ = exploration_ticker.tick() => {
                    if !*self.running.read().await {
                        break;
                    }

                    if !self.curiosity.is_autonomous_enabled() {
                        continue;
                    }

                    // Run exploration cycle
                    if let Some(result) = self.explore_cycle(&controller).await {
                        if self.config.verbose {
                            self.log_exploration(&result);
                        }
                    }
                }

                _ = cleanup_ticker.tick() => {
                    if !*self.running.read().await {
                        break;
                    }

                    // Periodic cleanup
                    self.curiosity.cleanup();

                    if self.config.verbose {
                        println!("[CuriosityDrive] Cleanup completed");
                    }
                }
            }
        }
    }

    /// Stop autonomous exploration
    pub async fn stop(&self) {
        *self.running.write().await = false;
    }

    /// Check if currently running
    pub async fn is_running(&self) -> bool {
        *self.running.read().await
    }

    /// Execute single exploration cycle
    async fn explore_cycle(&self, controller: &ActionController) -> Option<ExplorationCycle> {
        let start = std::time::Instant::now();

        // Get next exploration target
        let target = self.get_next_target()?;

        // Execute exploration action
        let success = self.execute_exploration(controller, &target).await;

        let duration = start.elapsed();

        Some(ExplorationCycle {
            target,
            success,
            duration,
        })
    }

    /// Get next exploration target (from queue or suggestion)
    fn get_next_target(&self) -> Option<ExplorationTarget> {
        // First try queue
        if let Some(target) = self.curiosity.get_next_target() {
            return Some(target);
        }

        // Otherwise ask for suggestion
        self.curiosity.suggest_exploration()
    }

    /// Execute exploration action
    async fn execute_exploration(&self, controller: &ActionController, target: &ExplorationTarget) -> bool {
        // TODO: Integration with ActionController
        // For now, just mark as explored
        // In full implementation:
        // 1. Convert exploration target to action
        // 2. Submit to ActionController
        // 3. Wait for result
        // 4. Update curiosity metrics based on result

        let _ = controller;
        let _ = target;

        true
    }

    /// Log exploration event
    fn log_exploration(&self, cycle: &ExplorationCycle) {
        println!(
            "[CuriosityDrive] Explored {:?} target (score: {:.3}) in {:?} - {}",
            cycle.target.reason,
            cycle.target.score,
            cycle.duration,
            if cycle.success { "success" } else { "failed" }
        );
    }
}

/// Run autonomous exploration loop (convenience function)
pub async fn run_autonomous_exploration(
    curiosity: Arc<CuriosityDrive>,
    controller: Arc<ActionController>,
    config: AutonomousConfig,
) {
    let explorer = AutonomousExplorer::new(curiosity, config);
    explorer.start(controller).await;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::curiosity::CuriosityConfig;
    use crate::action_controller::{ActionControllerConfig, ArbiterConfig};
    use crate::graph::{Graph, GraphConfig};
    use crate::experience_stream::ExperienceStream;

    #[tokio::test]
    async fn test_autonomous_explorer_creation() {
        let curiosity = Arc::new(CuriosityDrive::new(CuriosityConfig::default()));
        let config = AutonomousConfig::default();

        let explorer = AutonomousExplorer::new(curiosity, config);
        assert!(!explorer.is_running().await);
    }

    #[tokio::test]
    async fn test_autonomous_start_stop() {
        let curiosity = Arc::new(CuriosityDrive::new(CuriosityConfig::default()));
        let config = AutonomousConfig {
            exploration_interval: Duration::from_millis(100),
            cleanup_interval: Duration::from_secs(10),
            verbose: false,
        };

        let explorer = Arc::new(AutonomousExplorer::new(curiosity.clone(), config));

        // Create minimal ActionController
        let graph = Arc::new(tokio::sync::RwLock::new(Graph::new(GraphConfig::default())));
        let stream = Arc::new(parking_lot::RwLock::new(ExperienceStream::new()));
        let controller_config = ActionControllerConfig {
            arbiter: ArbiterConfig::default(),
            max_pending_actions: 10,
        };
        let controller = Arc::new(ActionController::new(controller_config, graph, stream));

        // Start in background
        let explorer_clone = explorer.clone();
        let controller_clone = controller.clone();
        let handle = tokio::spawn(async move {
            explorer_clone.start(controller_clone).await;
        });

        // Wait a bit
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Should be running
        assert!(explorer.is_running().await);

        // Stop
        explorer.stop().await;

        // Wait for shutdown
        tokio::time::sleep(Duration::from_millis(200)).await;

        handle.abort();
    }
}
