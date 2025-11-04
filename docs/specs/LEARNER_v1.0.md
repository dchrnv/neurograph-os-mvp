# Learner Module v1.0 - Hebbian Learning

**Версия:** 1.0.0
**Дата:** 2025-11-04
**Статус:** 🎯 В разработке (v0.26.0)
**Зависимости:** ExperienceStream v2.0, ADNA v1.0, Guardian v1.1, Connection v1.0, Graph v2.0
**Цель:** Реализовать Hebbian learning для обновления connection weights на основе rewards

---

## 1. Философия

### 1.1 Основные принципы

**"Neurons that fire together, wire together"** - Donald Hebb

Learner Module реализует базовое синаптическое обучение:
- Connections, которые активируются вместе с positive rewards, усиливаются
- Connections без активации или с negative rewards ослабляются
- Learning rate берется из ADNA parameters (адаптивная политика)

### 1.2 Роль в KEY Architecture

```text
┌─────────────────────────────────────────────────┐
│            ExperienceStream v2.0                │
│  (события с rewards от Appraisers)              │
└─────────────────────────────────────────────────┘
                    ↓ subscribe
┌─────────────────────────────────────────────────┐
│            Learner Module v1.0                  │
│  (Hebbian learning + weight updates)            │
└─────────────────────────────────────────────────┘
                    ↓ update weights
┌─────────────────────────────────────────────────┐
│            Connection Weights Storage            │
│  (HashMap<EdgeId, f32> - external)              │
└─────────────────────────────────────────────────┘
```

### 1.3 Отложенные функции (v2.0+)

- ❌ Deep learning models (requires IntuitionEngine)
- ❌ Policy gradient methods
- ❌ Q-learning для action selection
- ❌ Meta-learning для learning rates

---

## 2. Архитектура

### 2.1 Core Structures

```rust
// src/core_rust/src/learner/mod.rs

use std::collections::HashMap;
use crate::{EdgeId, ExperienceEvent, ADNA};

/// Learning mode configuration
#[derive(Debug, Clone, Copy)]
pub enum LearningMode {
    /// Online learning (update after each event)
    Online {
        update_threshold: f32,  // Min reward magnitude to trigger update
        trace_decay: f32,       // Eligibility trace decay rate (0.9-0.99)
    },
    /// Batch learning (accumulate then consolidate)
    Batch {
        batch_size: usize,      // Events per batch
        consolidation_rate: f32, // How fast to apply batched updates
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
    Classic,

    /// BCM (Bienenstock-Cooper-Munro): Δw = η * x * y * (y - θ)
    /// More stable, prevents runaway weights
    BCM,

    /// Oja's rule: Δw = η * y * (x - w * y)
    /// Normalizing, prevents weight explosion
    Oja,
}

/// Weight update record
#[derive(Debug, Clone)]
pub struct WeightUpdate {
    pub edge_id: EdgeId,
    pub old_weight: f32,
    pub new_weight: f32,
    pub delta: f32,
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
```

### 2.2 Core Methods

```rust
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

    /// Process experience event and update weights
    pub fn learn(&mut self, event: &ExperienceEvent, adna: &ADNA) -> Vec<WeightUpdate> {
        // Extract learning rate from ADNA
        let base_rate = adna.parameters.learning_rate;
        let effective_rate = base_rate * self.config.adaptive_factor;

        // Get reward from event
        let reward = event.reward;

        // Check if reward is significant enough
        if let LearningMode::Online { update_threshold, .. } = self.config.mode {
            if reward.abs() < update_threshold {
                return Vec::new(); // Skip insignificant updates
            }
        }

        // Extract connections from event metadata
        // TODO: Event should store EdgeIds that were involved
        let affected_edges = self.extract_edges_from_event(event);

        let mut updates = Vec::new();

        for edge_id in affected_edges {
            let old_weight = self.get_weight(edge_id);

            // Calculate delta based on Hebbian rule
            let delta = match self.config.rule {
                HebbianRule::Classic => {
                    self.classic_hebbian(edge_id, reward, effective_rate)
                }
                HebbianRule::BCM => {
                    self.bcm_rule(edge_id, reward, effective_rate)
                }
                HebbianRule::Oja => {
                    self.oja_rule(edge_id, old_weight, reward, effective_rate)
                }
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

        updates
    }

    /// Classic Hebbian rule: Δw = η * x * y
    fn classic_hebbian(&self, edge_id: EdgeId, reward: f32, learning_rate: f32) -> f32 {
        // x = pre-synaptic activation (assume 1.0 if event triggered)
        // y = post-synaptic activation = reward (normalized to 0-1)
        let x = 1.0;
        let y = (reward + 1.0) / 2.0; // Map [-1, 1] to [0, 1]

        learning_rate * x * y
    }

    /// BCM rule: Δw = η * x * y * (y - θ)
    /// θ = sliding threshold (average activation)
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
    /// Normalizing, prevents weight explosion
    fn oja_rule(&self, edge_id: EdgeId, current_weight: f32, reward: f32, learning_rate: f32) -> f32 {
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
    /// TODO: This needs proper implementation when Event format is finalized
    fn extract_edges_from_event(&self, event: &ExperienceEvent) -> Vec<EdgeId> {
        // Placeholder: extract from event metadata
        // In real implementation, event should carry EdgeIds
        Vec::new()
    }

    /// Normalize all weights (for stability)
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
    pub fn prune_dead_connections(&mut self, threshold: f32) -> usize {
        let initial_count = self.weights.len();
        self.weights.retain(|_, w| *w >= threshold);
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
        let variance: f32 = self.weights.values()
            .map(|w| (w - avg).powi(2))
            .sum::<f32>() / self.weights.len() as f32;
        self.metrics.weight_variance = variance;

        // Dead connections
        self.metrics.dead_connections = self.weights.values()
            .filter(|&&w| w < self.config.min_weight * 2.0)
            .count();

        // Saturated connections
        self.metrics.saturated_connections = self.weights.values()
            .filter(|&&w| w > self.config.max_weight * 0.95)
            .count();
    }
}
```

---

## 3. Integration с ExperienceStream

### 3.1 Subscription Pattern

```rust
// Learner subscribes to ExperienceStream events
impl Learner {
    /// Subscribe to ExperienceStream and process events
    pub async fn subscribe_to_stream(
        &mut self,
        stream: &mut ExperienceStream,
        adna: &ADNA,
    ) -> Result<(), String> {
        let mut receiver = stream.subscribe();

        while let Ok(event) = receiver.recv().await {
            // Process event and update weights
            let updates = self.learn(&event, adna);

            // Log updates (optional)
            if !updates.is_empty() {
                println!("Learner: {} weight updates from event", updates.len());
            }

            // Update metrics periodically
            if self.metrics.total_updates % 1000 == 0 {
                self.update_metrics();
            }
        }

        Ok(())
    }
}
```

### 3.2 Event Format Requirements

ExperienceEvent должен содержать:
- `reward: f32` - от Appraisers (уже есть)
- `affected_edges: Vec<EdgeId>` - какие connections были задействованы (TODO)
- `state: [f32; 8]` - состояние системы (уже есть)

---

## 4. Learning Rate Strategy

### 4.1 Базовый rate из ADNA

```rust
// ADNA parameters содержат base learning rate
pub struct ADNAParameters {
    // ... existing fields
    pub learning_rate: f32,  // Already exists! (0.001 - 0.1)
}
```

### 4.2 Per-Connection Modulation

```rust
impl Learner {
    /// Get effective learning rate for connection type
    fn get_effective_rate(&self, edge_id: EdgeId, base_rate: f32, conn_type: ConnectionType) -> f32 {
        let type_multiplier = match conn_type {
            // Fast learning (associative links)
            ConnectionType::AssociatedWith |
            ConnectionType::SimilarTo |
            ConnectionType::RelatedTo => 2.0,

            // Medium learning (causal, temporal)
            ConnectionType::Cause |
            ConnectionType::Effect |
            ConnectionType::Before |
            ConnectionType::After => 1.0,

            // Slow learning (hierarchical, structural)
            ConnectionType::Hypernym |
            ConnectionType::Hyponym |
            ConnectionType::PartOf |
            ConnectionType::HasPart => 0.5,

            // No learning (definitions, rules)
            ConnectionType::Rule |
            ConnectionType::Equivalent => 0.0,

            _ => 1.0,
        };

        base_rate * type_multiplier * self.config.adaptive_factor
    }
}
```

---

## 5. Testing Strategy

### 5.1 Unit Tests (15-20 tests)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_learner_creation() {
        let learner = Learner::new();
        assert_eq!(learner.get_weight(123), 0.5); // Default weight
    }

    #[test]
    fn test_weight_clamping() {
        let mut learner = Learner::new();
        learner.set_weight(1, 1.5); // Over max
        assert_eq!(learner.get_weight(1), 1.0); // Clamped to max

        learner.set_weight(2, -0.5); // Under min
        assert_eq!(learner.get_weight(2), 0.01); // Clamped to min
    }

    #[test]
    fn test_classic_hebbian() {
        let learner = Learner::new();
        let delta = learner.classic_hebbian(1, 0.5, 0.01);
        assert!(delta > 0.0); // Positive reward → positive delta
    }

    #[test]
    fn test_bcm_rule() {
        let mut learner = Learner::new();
        let delta1 = learner.bcm_rule(1, 0.8, 0.01);
        let delta2 = learner.bcm_rule(1, 0.2, 0.01);
        // High activation above threshold → strengthening
        // Low activation below threshold → weakening
    }

    #[test]
    fn test_oja_normalization() {
        let learner = Learner::new();
        let delta = learner.oja_rule(1, 0.9, 0.5, 0.01);
        // Oja should prevent weight explosion
    }

    // ... 10-15 more tests
}
```

---

## 6. Performance Considerations

### 6.1 Memory Usage

- **Weights storage:** 8 bytes per connection (EdgeId + f32)
- **Eligibility traces:** 8 bytes per active connection
- **BCM thresholds:** 8 bytes per connection
- **Total:** ~24 bytes per connection overhead

**Example:** 1M connections = 24 MB overhead (acceptable)

### 6.2 Computational Cost

- **Per-event learning:** O(k) where k = affected connections (~5-20 typically)
- **Weight update:** ~50ns (hash lookup + arithmetic)
- **Batch consolidation:** O(n) where n = batch size

**Target:** Handle 10K events/sec with <1ms learning overhead

---

## 7. Roadmap

### 7.1 v1.0 (v0.26.0) - Current

- ✅ Basic Hebbian learning (Classic, BCM, Oja)
- ✅ External weight storage (HashMap)
- ✅ ExperienceStream integration
- ✅ Learning rate from ADNA
- ✅ Metrics and monitoring

### 7.2 v1.5 (v0.30.0+) - Enhanced Learning

- Temporal Difference learning
- Multi-step credit assignment
- Experience replay buffer
- Priority sampling

### 7.3 v2.0 (v0.31.0+) - RL Integration

- Q-learning для action selection
- Policy gradient methods
- Connection v2.0 integration (embedded weights)

---

**Конец спецификации Learner Module v1.0**

**Статус:** Спецификация готова к реализации
**Next:** Implement `src/core_rust/src/learner/mod.rs`
