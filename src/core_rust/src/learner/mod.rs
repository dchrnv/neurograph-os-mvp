use crate::{ExperienceEvent, ADNA};
/// Learner Module v1.0 - Hebbian Learning for NeuroGraph OS
///
/// Implements basic Hebbian learning rule: "Neurons that fire together, wire together"
/// Updates connection weights based on rewards from ExperienceStream events.
///
/// # Architecture
///
/// - External weight storage (HashMap<EdgeId, f32>)
/// - Three Hebbian rule variants: Classic, BCM, Oja
/// - Online and Batch learning modes
/// - Integration with ADNA for learning rate
/// - Metrics for monitoring learning progress
///
/// # Design Decisions (v1.0)
///
/// - Weights stored separately from Connection v1.0 (32 bytes)
/// - Future v2.0 will embed weights in Connection v2.0 (40 bytes)
/// - Learning rate modulated by connection type
/// - BCM rule for stability (prevents weight explosion)
///
/// # Example
///
/// ```
/// use neurograph_core::learner::{Learner, LearningConfig, HebbianRule};
/// use neurograph_core::ADNA;
///
/// let mut learner = Learner::with_config(LearningConfig {
///     base_rate: 0.01,
///     rule: HebbianRule::BCM,
///     ..Default::default()
/// });
///
/// // Process events from ExperienceStream
/// let updates = learner.learn(&event, &adna);
/// ```
use std::collections::HashMap;

/// Edge identifier (hash of connection)
pub type EdgeId = u64;

/// Learning mode configuration
#[derive(Debug, Clone, Copy)]
pub enum LearningMode {
    /// Online learning (update after each event)
    Online {
        /// Minimum reward magnitude to trigger update
        update_threshold: f32,
        /// Eligibility trace decay rate (0.9-0.99)
        trace_decay: f32,
    },
    /// Batch learning (accumulate then consolidate)
    Batch {
        /// Events per batch
        batch_size: usize,
        /// How fast to apply batched updates
        consolidation_rate: f32,
    },
}

impl Default for LearningMode {
    fn default() -> Self {
        Self::Online {
            update_threshold: 0.01,
            trace_decay: 0.95,
        }
    }
}

/// Learning configuration
#[derive(Debug, Clone)]
pub struct LearningConfig {
    /// Base learning rate (from ADNA)
    pub base_rate: f32,

    /// Adaptive factor (can be modulated by attention/context)
    pub adaptive_factor: f32,

    /// Learning mode (online vs batch)
    pub mode: LearningMode,

    /// Weight constraints
    pub min_weight: f32,
    pub max_weight: f32,

    /// Hebbian rule variant
    pub rule: HebbianRule,
}

impl Default for LearningConfig {
    fn default() -> Self {
        Self {
            base_rate: 0.01,
            adaptive_factor: 1.0,
            mode: LearningMode::default(),
            min_weight: 0.01,
            max_weight: 1.0,
            rule: HebbianRule::BCM,
        }
    }
}

/// Hebbian learning rule variants
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HebbianRule {
    /// Classic Hebbian: Δw = η * x * y
    /// Simple but can lead to weight explosion
    Classic,

    /// BCM (Bienenstock-Cooper-Munro): Δw = η * x * y * (y - θ)
    /// More stable, prevents runaway weights with sliding threshold
    BCM,

    /// Oja's rule: Δw = η * y * (x - w * y)
    /// Normalizing, prevents weight explosion through self-regulation
    Oja,
}

/// Weight update record
#[derive(Debug, Clone)]
pub struct WeightUpdate {
    /// Connection ID
    pub edge_id: EdgeId,
    /// Weight before update
    pub old_weight: f32,
    /// Weight after update
    pub new_weight: f32,
    /// Change in weight
    pub delta: f32,
    /// When update occurred
    pub timestamp: u64,
}

/// Learner metrics for monitoring
#[derive(Debug, Clone, Default)]
pub struct LearnerMetrics {
    /// Total weight updates performed
    pub total_updates: u64,

    /// Updates per second (rolling average)
    pub updates_per_second: f32,

    /// Average weight across all connections
    pub average_weight: f32,

    /// Variance in weights
    pub weight_variance: f32,

    /// Number of dead connections (weight < threshold)
    pub dead_connections: usize,

    /// Number of saturated connections (weight ≈ max)
    pub saturated_connections: usize,

    /// Average learning rate used
    pub average_learning_rate: f32,
}

/// Main Learner structure
///
/// Implements Hebbian learning for connection weight updates.
/// Weights are stored externally in HashMap (Connection v1.0 doesn't have weight field).
pub struct Learner {
    /// Hebbian weights (external storage)
    weights: HashMap<EdgeId, f32>,

    /// Eligibility traces for temporal credit assignment
    eligibility_traces: HashMap<EdgeId, f32>,

    /// BCM threshold (sliding average of activation)
    bcm_thresholds: HashMap<EdgeId, f32>,

    /// Learning configuration
    config: LearningConfig,

    /// Metrics
    metrics: LearnerMetrics,

    /// Batch accumulator (for batch mode)
    batch_updates: Vec<WeightUpdate>,
}

impl Default for Learner {
    fn default() -> Self {
        Self::new()
    }
}

impl Learner {
    /// Create new Learner with default config
    pub fn new() -> Self {
        Self {
            weights: HashMap::new(),
            eligibility_traces: HashMap::new(),
            bcm_thresholds: HashMap::new(),
            config: LearningConfig::default(),
            metrics: LearnerMetrics::default(),
            batch_updates: Vec::new(),
        }
    }

    /// Create Learner with custom config
    pub fn with_config(config: LearningConfig) -> Self {
        let mut learner = Self::new();
        learner.config = config;
        learner
    }

    /// Get weight for connection (returns 0.5 if not learned yet)
    pub fn get_weight(&self, edge_id: EdgeId) -> f32 {
        *self.weights.get(&edge_id).unwrap_or(&0.5)
    }

    /// Set weight for connection (clamped to min/max)
    pub fn set_weight(&mut self, edge_id: EdgeId, weight: f32) {
        let clamped = weight.clamp(self.config.min_weight, self.config.max_weight);
        self.weights.insert(edge_id, clamped);
    }

    /// Get number of learned weights
    pub fn weight_count(&self) -> usize {
        self.weights.len()
    }

    /// Clear all weights (reset learner)
    pub fn clear_weights(&mut self) {
        self.weights.clear();
        self.eligibility_traces.clear();
        self.bcm_thresholds.clear();
        self.metrics = LearnerMetrics::default();
    }

    /// Process experience event and update weights
    ///
    /// # Arguments
    ///
    /// * `event` - Experience event with reward
    /// * `adna` - ADNA parameters (for learning rate)
    ///
    /// # Returns
    ///
    /// Vector of weight updates performed
    pub fn learn(&mut self, event: &ExperienceEvent, adna: &ADNA) -> Vec<WeightUpdate> {
        // Extract learning rate from ADNA
        let base_rate = adna.parameters.learning_rate;
        let effective_rate = base_rate * self.config.adaptive_factor;

        // Get reward from event
        let reward = event.reward;

        // Check if reward is significant enough
        if let LearningMode::Online {
            update_threshold, ..
        } = self.config.mode
        {
            if reward.abs() < update_threshold {
                return Vec::new(); // Skip insignificant updates
            }
        }

        // Extract connections from event metadata
        // TODO: Event should store EdgeIds that were involved
        // For now, we'll use a placeholder implementation
        let affected_edges = self.extract_edges_from_event(event);

        let mut updates = Vec::new();

        for edge_id in affected_edges {
            let old_weight = self.get_weight(edge_id);

            // Calculate delta based on Hebbian rule
            let delta = match self.config.rule {
                HebbianRule::Classic => self.classic_hebbian(reward, effective_rate),
                HebbianRule::BCM => self.bcm_rule(edge_id, reward, effective_rate),
                HebbianRule::Oja => self.oja_rule(old_weight, reward, effective_rate),
            };

            let new_weight = old_weight + delta;
            self.set_weight(edge_id, new_weight);

            updates.push(WeightUpdate {
                edge_id,
                old_weight,
                new_weight,
                delta,
                timestamp: event.timestamp,
            });

            // Update eligibility trace
            self.update_eligibility_trace(edge_id);
        }

        // Update metrics
        self.metrics.total_updates += updates.len() as u64;
        self.metrics.average_learning_rate = effective_rate;

        updates
    }

    /// Classic Hebbian rule: Δw = η * x * y
    ///
    /// Simple but can lead to weight explosion without bounds
    fn classic_hebbian(&self, reward: f32, learning_rate: f32) -> f32 {
        // x = pre-synaptic activation (assume 1.0 if event triggered)
        // y = post-synaptic activation = reward (normalized to 0-1)
        let x = 1.0;
        let y = (reward + 1.0) / 2.0; // Map [-1, 1] to [0, 1]

        learning_rate * x * y
    }

    /// BCM rule: Δw = η * x * y * (y - θ)
    ///
    /// θ = sliding threshold (average activation)
    /// Prevents weight explosion by penalizing weak activations
    fn bcm_rule(&mut self, edge_id: EdgeId, reward: f32, learning_rate: f32) -> f32 {
        let x = 1.0;
        let y = (reward + 1.0) / 2.0;

        // Get or initialize BCM threshold
        let threshold = self.bcm_thresholds.entry(edge_id).or_insert(0.5);

        // BCM delta
        let delta = learning_rate * x * y * (y - *threshold);

        // Update threshold (sliding average)
        *threshold = 0.99 * (*threshold) + 0.01 * y;

        delta
    }

    /// Oja's rule: Δw = η * y * (x - w * y)
    ///
    /// Normalizing, prevents weight explosion through self-regulation
    fn oja_rule(&self, current_weight: f32, reward: f32, learning_rate: f32) -> f32 {
        let x = 1.0;
        let y = (reward + 1.0) / 2.0;

        learning_rate * y * (x - current_weight * y)
    }

    /// Update eligibility trace for temporal credit assignment
    fn update_eligibility_trace(&mut self, edge_id: EdgeId) {
        if let LearningMode::Online { trace_decay, .. } = self.config.mode {
            let trace = self.eligibility_traces.entry(edge_id).or_insert(0.0);
            *trace = (*trace * trace_decay) + 1.0;
        }
    }

    /// Extract edge IDs from event
    ///
    /// TODO: This needs proper implementation when Event format is finalized
    /// For now, returns empty vec as placeholder
    fn extract_edges_from_event(&self, _event: &ExperienceEvent) -> Vec<EdgeId> {
        // Placeholder: extract from event metadata
        // In real implementation, event should carry EdgeIds
        Vec::new()
    }

    /// Normalize all weights (for stability)
    ///
    /// Ensures sum of squared weights = 1.0
    pub fn normalize_weights(&mut self) {
        if self.weights.is_empty() {
            return;
        }

        // Calculate sum of squares
        let sum_squares: f32 = self.weights.values().map(|w| w * w).sum();
        let norm = sum_squares.sqrt();

        if norm > 0.0 {
            for weight in self.weights.values_mut() {
                *weight /= norm;
                *weight = weight.clamp(self.config.min_weight, self.config.max_weight);
            }
        }
    }

    /// Prune dead connections (weight below threshold)
    ///
    /// Returns number of connections pruned
    pub fn prune_dead_connections(&mut self, threshold: f32) -> usize {
        let initial_count = self.weights.len();
        self.weights.retain(|_, w| *w >= threshold);

        // Also clean up traces and thresholds
        let remaining_edges: Vec<EdgeId> = self.weights.keys().copied().collect();
        self.eligibility_traces
            .retain(|k, _| remaining_edges.contains(k));
        self.bcm_thresholds
            .retain(|k, _| remaining_edges.contains(k));

        initial_count - self.weights.len()
    }

    /// Get current metrics
    pub fn metrics(&self) -> &LearnerMetrics {
        &self.metrics
    }

    /// Update metrics (should be called periodically)
    pub fn update_metrics(&mut self) {
        if self.weights.is_empty() {
            return;
        }

        // Average weight
        let sum: f32 = self.weights.values().sum();
        self.metrics.average_weight = sum / self.weights.len() as f32;

        // Variance
        let avg = self.metrics.average_weight;
        let variance: f32 = self
            .weights
            .values()
            .map(|w| (w - avg).powi(2))
            .sum::<f32>()
            / self.weights.len() as f32;
        self.metrics.weight_variance = variance;

        // Dead connections (weight < 2x min_weight)
        self.metrics.dead_connections = self
            .weights
            .values()
            .filter(|&&w| w < self.config.min_weight * 2.0)
            .count();

        // Saturated connections (weight > 95% of max_weight)
        self.metrics.saturated_connections = self
            .weights
            .values()
            .filter(|&&w| w > self.config.max_weight * 0.95)
            .count();
    }

    /// Get configuration
    pub fn config(&self) -> &LearningConfig {
        &self.config
    }

    /// Update configuration
    pub fn set_config(&mut self, config: LearningConfig) {
        self.config = config;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ADNAProfile, ADNA};

    #[test]
    fn test_learner_creation() {
        let learner = Learner::new();
        assert_eq!(learner.get_weight(123), 0.5); // Default weight
        assert_eq!(learner.weight_count(), 0);
    }

    #[test]
    fn test_learner_with_config() {
        let config = LearningConfig {
            base_rate: 0.02,
            rule: HebbianRule::Oja,
            ..Default::default()
        };
        let learner = Learner::with_config(config);
        assert_eq!(learner.config().base_rate, 0.02);
        assert_eq!(learner.config().rule, HebbianRule::Oja);
    }

    #[test]
    fn test_weight_clamping() {
        let mut learner = Learner::new();

        // Set weight over max
        learner.set_weight(1, 1.5);
        assert_eq!(learner.get_weight(1), 1.0); // Clamped to max

        // Set weight under min
        learner.set_weight(2, -0.5);
        assert_eq!(learner.get_weight(2), 0.01); // Clamped to min

        // Set normal weight
        learner.set_weight(3, 0.7);
        assert_eq!(learner.get_weight(3), 0.7);
    }

    #[test]
    fn test_classic_hebbian() {
        let learner = Learner::new();

        // Positive reward should give positive delta
        let delta1 = learner.classic_hebbian(0.5, 0.01);
        assert!(delta1 > 0.0);

        // Negative reward should give small delta
        let delta2 = learner.classic_hebbian(-0.5, 0.01);
        assert!(delta2 < delta1);
    }

    #[test]
    fn test_bcm_rule() {
        let mut learner = Learner::new();

        // High activation above threshold → strengthening
        let delta1 = learner.bcm_rule(1, 0.8, 0.01);

        // Low activation below threshold → weakening
        let delta2 = learner.bcm_rule(1, 0.2, 0.01);

        // High activation should produce larger absolute delta
        assert!(delta1.abs() > delta2.abs());
    }

    #[test]
    fn test_oja_normalization() {
        let learner = Learner::new();

        // Oja with high weight should reduce growth
        let delta_high = learner.oja_rule(0.9, 0.5, 0.01);
        let delta_low = learner.oja_rule(0.1, 0.5, 0.01);

        // Lower weight should allow more growth
        assert!(delta_low > delta_high);
    }

    #[test]
    fn test_weight_count() {
        let mut learner = Learner::new();
        assert_eq!(learner.weight_count(), 0);

        learner.set_weight(1, 0.5);
        learner.set_weight(2, 0.6);
        assert_eq!(learner.weight_count(), 2);
    }

    #[test]
    fn test_clear_weights() {
        let mut learner = Learner::new();
        learner.set_weight(1, 0.5);
        learner.set_weight(2, 0.6);

        learner.clear_weights();
        assert_eq!(learner.weight_count(), 0);
        assert_eq!(learner.get_weight(1), 0.5); // Back to default
    }

    #[test]
    fn test_normalize_weights() {
        let mut learner = Learner::new();
        learner.set_weight(1, 0.8);
        learner.set_weight(2, 0.6);
        learner.set_weight(3, 0.4);

        learner.normalize_weights();

        // Check sum of squares ≈ 1.0
        let sum_squares: f32 = [1, 2, 3]
            .iter()
            .map(|&id| learner.get_weight(id).powi(2))
            .sum();

        assert!((sum_squares - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_prune_dead_connections() {
        let mut learner = Learner::new();
        learner.set_weight(1, 0.8); // Keep
        learner.set_weight(2, 0.02); // Prune (below threshold)
        learner.set_weight(3, 0.5); // Keep

        let pruned = learner.prune_dead_connections(0.1);

        assert_eq!(pruned, 1);
        assert_eq!(learner.weight_count(), 2);
        assert_eq!(learner.get_weight(2), 0.5); // Back to default after pruning
    }

    #[test]
    fn test_metrics_update() {
        let mut learner = Learner::new();
        learner.set_weight(1, 0.3);
        learner.set_weight(2, 0.6);
        learner.set_weight(3, 0.9);

        learner.update_metrics();

        let metrics = learner.metrics();
        assert!((metrics.average_weight - 0.6).abs() < 0.01);
        assert!(metrics.weight_variance > 0.0);
    }

    #[test]
    fn test_learn_with_adna() {
        let mut learner = Learner::new();
        let adna = ADNA::from_profile(ADNAProfile::Balanced);

        // Create dummy event (learn will return empty vec because extract_edges_from_event returns [])
        let event = ExperienceEvent::new(crate::experience_stream::EventType::ActionCompleted);

        let updates = learner.learn(&event, &adna);

        // Should return empty because no edges extracted yet
        assert_eq!(updates.len(), 0);
    }

    #[test]
    fn test_learning_modes() {
        let online_config = LearningConfig {
            mode: LearningMode::Online {
                update_threshold: 0.05,
                trace_decay: 0.9,
            },
            ..Default::default()
        };

        let batch_config = LearningConfig {
            mode: LearningMode::Batch {
                batch_size: 100,
                consolidation_rate: 0.1,
            },
            ..Default::default()
        };

        let learner1 = Learner::with_config(online_config);
        let learner2 = Learner::with_config(batch_config);

        // Just verify configs are set correctly
        assert!(matches!(
            learner1.config().mode,
            LearningMode::Online { .. }
        ));
        assert!(matches!(learner2.config().mode, LearningMode::Batch { .. }));
    }

    #[test]
    fn test_hebbian_rules() {
        // Test all three rules
        let learner_classic = Learner::with_config(LearningConfig {
            rule: HebbianRule::Classic,
            ..Default::default()
        });

        let mut learner_bcm = Learner::with_config(LearningConfig {
            rule: HebbianRule::BCM,
            ..Default::default()
        });

        let learner_oja = Learner::with_config(LearningConfig {
            rule: HebbianRule::Oja,
            ..Default::default()
        });

        // All should produce different deltas
        let delta_classic = learner_classic.classic_hebbian(0.5, 0.01);
        let delta_bcm = learner_bcm.bcm_rule(1, 0.5, 0.01);
        let delta_oja = learner_oja.oja_rule(0.5, 0.5, 0.01);

        // Just verify they all produce valid deltas
        assert!(delta_classic.is_finite());
        assert!(delta_bcm.is_finite());
        assert!(delta_oja.is_finite());
    }
}
