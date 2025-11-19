// NeuroGraph - Высокопроизводительная система пространственных вычислений на основе токенов.
// Copyright (C) 2024-2025 Chernov Denys

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! IntuitionEngine v3.0 - Hybrid Reflex System
//!
//! Combines System 1 (Fast reflexes) with System 2 (Slow pattern analysis):
//!
//! # System 1: Fast Path (Reflex Layer)
//! - Spatial hashing for instant state → action lookup
//! - DashMap-based associative memory (lock-free)
//! - ~30-50ns lookup time
//! - Adaptive grid resolution
//!
//! # System 2: Slow Path (Analytic Layer)
//! - Pattern detection from ExperienceStream
//! - Statistical significance testing
//! - Hypothesis Connection creation
//! - Memory consolidation (Experience → Reflex)
//!
//! # Architecture
//!
//! ```text
//! State → Fast Path? → Yes → Execute Reflex (<50ns)
//!              ↓ No
//!         Slow Path → ADNA Reasoning (~1-10ms) → Create Reflex
//! ```

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::mpsc;
use crate::experience_stream::{ExperienceStream, ExperienceBatch, SamplingStrategy};
use crate::adna::{ADNAReader, Proposal};
use crate::token::Token;
use crate::connection_v3::{ConnectionV3, ConnectionMutability};
use crate::reflex_layer::{
    ShiftConfig, AssociativeMemory, FastPathConfig, FastPathResult,
    IntuitionStats as ReflexStats, compute_grid_hash,
};

/// Configuration for IntuitionEngine v3.0
#[derive(Debug, Clone)]
pub struct IntuitionConfig {
    // === Slow Path (Analytic Layer) ===
    /// Analysis cycle interval (seconds)
    pub analysis_interval_secs: u64,

    /// Batch size for analysis
    pub batch_size: usize,

    /// Sampling strategy
    pub sampling_strategy: SamplingStrategy,

    /// Minimum confidence threshold for proposals [0.0, 1.0]
    pub min_confidence: f64,

    /// Maximum proposals per cycle
    pub max_proposals_per_cycle: usize,

    /// Number of bins per coordinate dimension for state quantization
    pub state_bins_per_dim: usize,

    /// Minimum samples per state-action pair for analysis
    pub min_samples: usize,

    /// Minimum absolute reward difference for significance
    pub min_reward_delta: f64,

    // === Fast Path (Reflex Layer) v3.0 ===
    /// Enable fast path reflexes
    pub enable_fast_path: bool,

    /// Spatial hash shift configuration
    pub shift_config: ShiftConfig,

    /// Fast path execution configuration
    pub fast_path_config: FastPathConfig,
}

impl Default for IntuitionConfig {
    fn default() -> Self {
        Self {
            // Slow Path defaults
            analysis_interval_secs: 60,
            batch_size: 1000,
            sampling_strategy: SamplingStrategy::PrioritizedByReward { alpha: 1.0 },
            min_confidence: 0.7,
            max_proposals_per_cycle: 5,
            state_bins_per_dim: 4,
            min_samples: 10,
            min_reward_delta: 0.5,

            // Fast Path defaults (v3.0)
            enable_fast_path: true,  // Enable by default
            shift_config: ShiftConfig::default(),
            fast_path_config: FastPathConfig::default(),
        }
    }
}

/// Identified pattern from batch analysis
#[derive(Debug, Clone)]
pub struct IdentifiedPattern {
    /// State cluster/bin ID
    pub state_bin_id: u64,

    /// Action type with better reward
    pub better_action: u16,

    /// Action type with worse reward
    pub worse_action: u16,

    /// Difference in mean rewards
    pub reward_delta: f64,

    /// Statistical confidence [0.0, 1.0]
    pub confidence: f64,

    /// Number of samples used
    pub sample_count: usize,
}

/// IntuitionEngine v3.0 - Hybrid reflex + analytic system
pub struct IntuitionEngine {
    // Slow Path (Analytic Layer)
    config: IntuitionConfig,
    experience_stream: Arc<ExperienceStream>,
    _dna_reader: Arc<dyn ADNAReader>,
    proposal_sender: mpsc::Sender<Proposal>,

    // Fast Path (Reflex Layer) v3.0
    associative_memory: AssociativeMemory,
    connections: Arc<std::sync::RwLock<HashMap<u64, ConnectionV3>>>,
    stats: Arc<std::sync::RwLock<ReflexStats>>,
}

impl IntuitionEngine {
    /// Create new IntuitionEngine v3.0
    pub fn new(
        config: IntuitionConfig,
        experience_stream: Arc<ExperienceStream>,
        dna_reader: Arc<dyn ADNAReader>,
        proposal_sender: mpsc::Sender<Proposal>,
    ) -> Self {
        Self {
            // Slow Path
            config,
            experience_stream,
            _dna_reader: dna_reader,
            proposal_sender,

            // Fast Path (v3.0)
            associative_memory: AssociativeMemory::new(),
            connections: Arc::new(std::sync::RwLock::new(HashMap::new())),
            stats: Arc::new(std::sync::RwLock::new(ReflexStats::default())),
        }
    }

    /// Try fast path lookup (System 1)
    ///
    /// Returns ConnectionID if reflex is found and confident enough.
    /// Returns None if state is unknown (fallback to ADNA/System 2).
    ///
    /// # Performance
    /// - Target: <50ns
    /// - Actual: ~30-40ns (hash + lookup + similarity check)
    pub fn try_fast_path(&self, state: &Token) -> Option<FastPathResult> {
        if !self.config.enable_fast_path {
            return None;
        }

        let start = std::time::Instant::now();

        // 1. Compute spatial hash
        let hash = compute_grid_hash(state, &self.config.shift_config);

        // 2. Lookup candidates
        let candidates = self.associative_memory.lookup(hash)?;

        // 3. Find best match (collision resolution)
        let connections = self.connections.read().unwrap();
        let mut best_match: Option<(u64, f32)> = None;

        for &conn_id in candidates.iter() {
            let conn = connections.get(&conn_id)?;

            // 4. Check if reflex is eligible
            if !Self::is_reflex_eligible(conn, &self.config.fast_path_config) {
                continue;
            }

            // 5. Compute similarity (v0.32.0: needs state_token storage)
            // TODO v0.32.0: To fully leverage token_similarity(), we need to:
            //   - Store source Token (or its coordinates) when consolidating reflexes
            //   - Retrieve stored Token here and call token_similarity(state, stored_token)
            //   - Use actual similarity score for collision resolution
            // For now, use confidence as a proxy (higher confidence = better match)
            let similarity = conn.confidence as f32 / 255.0;

            // 6. Track best match
            match best_match {
                None => best_match = Some((conn_id, similarity)),
                Some((_, prev_sim)) if similarity > prev_sim => {
                    best_match = Some((conn_id, similarity));
                }
                _ => {}
            }
        }

        // 7. Record timing
        let elapsed = start.elapsed().as_nanos() as u64;
        {
            let mut stats = self.stats.write().unwrap();
            stats.fast_path_hits += 1;
            stats.avg_fast_path_time_ns =
                (stats.avg_fast_path_time_ns + elapsed) / 2;
        }

        // 8. Return if good enough
        if let Some((conn_id, similarity)) = best_match {
            if similarity > self.config.fast_path_config.similarity_threshold {
                return Some(FastPathResult {
                    connection_id: conn_id,
                    similarity,
                    hash,
                });
            }
        }

        None
    }

    /// Check if Connection is eligible for fast path
    fn is_reflex_eligible(conn: &ConnectionV3, config: &FastPathConfig) -> bool {
        // Minimum confidence threshold
        if conn.confidence < config.min_confidence {
            return false;
        }

        // Hypothesis connections need higher threshold
        if conn.mutability == ConnectionMutability::Hypothesis as u8 {
            return conn.confidence >= config.hypothesis_threshold;
        }

        // Learnable and Immutable are OK
        true
    }

    /// Add reflex to associative memory (called from Analytic Layer)
    ///
    /// Consolidates identified pattern into fast-path reflex.
    pub fn consolidate_reflex(
        &mut self,
        state_token: &Token,
        connection: ConnectionV3,
    ) {
        // 1. Compute hash for state
        let hash = compute_grid_hash(state_token, &self.config.shift_config);

        // 2. Store connection
        let conn_id = connection.token_a_id as u64;  // Use as unique ID
        self.connections.write().unwrap().insert(conn_id, connection);

        // 3. Add to associative memory
        self.associative_memory.insert(hash, conn_id);

        // 4. Update stats
        let mut stats = self.stats.write().unwrap();
        stats.reflexes_created += 1;
        stats.total_reflexes = self.connections.read().unwrap().len();
    }

    /// Check if connection should be consolidated to reflex and do it automatically
    ///
    /// # Consolidation Criteria
    ///
    /// A connection is eligible for automatic consolidation if:
    /// - **Confidence ≥ 192** (~75% - high confidence threshold)
    /// - **Evidence count ≥ 10** (sufficient experience)
    /// - **Mutability = Learnable or Immutable** (no Hypothesis)
    /// - **Passes Guardian validation** (safety check)
    ///
    /// # Usage
    ///
    /// ```rust
    /// // After updating a connection from experience
    /// if intuition.try_auto_consolidate(&state_token, &connection) {
    ///     println!("Connection promoted to reflex!");
    /// }
    /// ```
    ///
    /// # Returns
    ///
    /// - `true` if connection was consolidated to reflex
    /// - `false` if not eligible or validation failed
    pub fn try_auto_consolidate(
        &mut self,
        state_token: &Token,
        connection: &ConnectionV3,
        guardian: Option<&crate::Guardian>,
    ) -> bool {
        // 1. Check confidence threshold (75%)
        if connection.confidence < 192 {
            return false;
        }

        // 2. Check evidence count (at least 10 experiences)
        if connection.evidence_count < 10 {
            return false;
        }

        // 3. Check mutability (no Hypothesis)
        if connection.mutability > 1 {
            return false;
        }

        // 4. Optional Guardian validation
        if let Some(g) = guardian {
            if g.validate_reflex(connection).is_err() {
                return false;
            }
        }

        // 5. All checks passed - consolidate!
        self.consolidate_reflex(state_token, connection.clone());
        true
    }

    /// Get current stats (for monitoring/UI)
    pub fn get_stats(&self) -> ReflexStats {
        self.stats.read().unwrap().clone()
    }

    /// Run main analysis loop (async background task)
    pub async fn run(self) {
        let mut interval = tokio::time::interval(
            tokio::time::Duration::from_secs(self.config.analysis_interval_secs)
        );

        loop {
            interval.tick().await;

            if let Err(e) = self.run_analysis_cycle().await {
                eprintln!("IntuitionEngine analysis error: {}", e);
            }
        }
    }

    /// Single analysis cycle: sample → analyze → propose
    async fn run_analysis_cycle(&self) -> Result<(), String> {
        // 1. Sample "interesting" batch using prioritized sampling
        let batch = self.experience_stream.sample_batch(
            self.config.batch_size,
            self.config.sampling_strategy.clone()
        );

        if batch.events.is_empty() {
            return Ok(()); // Nothing to analyze yet
        }

        println!("[IntuitionEngine] Analyzing batch of {} events", batch.events.len());

        // 2. Analyze batch to find patterns
        let patterns = self.find_patterns_in_batch(&batch)?;

        println!("[IntuitionEngine] Found {} significant patterns", patterns.len());

        // 3. Generate proposals from patterns
        let proposals = self.generate_proposals_from_patterns(patterns)?;

        println!("[IntuitionEngine] Generated {} proposals", proposals.len());

        // 4. Send proposals to EvolutionManager
        let mut sent_count = 0;
        for proposal in proposals {
            if proposal.confidence >= self.config.min_confidence
                && sent_count < self.config.max_proposals_per_cycle {

                if let Err(e) = self.proposal_sender.send(proposal).await {
                    eprintln!("[IntuitionEngine] Failed to send proposal: {}", e);
                } else {
                    sent_count += 1;
                }
            }
        }

        println!("[IntuitionEngine] Sent {} proposals to EvolutionManager", sent_count);

        Ok(())
    }

    /// Core analysis: find patterns in batch (v1.0 - Statistical)
    fn find_patterns_in_batch(&self, batch: &ExperienceBatch) -> Result<Vec<IdentifiedPattern>, String> {
        // Phase 1: Quantize states into bins
        let mut state_action_rewards: HashMap<(u64, u16), Vec<f32>> = HashMap::new();

        for event in &batch.events {
            let state_bin = self.quantize_state(&event.state);
            let action = event.event_type;
            let total_reward = event.total_reward();

            state_action_rewards
                .entry((state_bin, action))
                .or_insert_with(Vec::new)
                .push(total_reward);
        }

        // Phase 2: Find state bins with multiple action types
        let mut state_bins_with_actions: HashMap<u64, Vec<u16>> = HashMap::new();
        for (state_bin, action) in state_action_rewards.keys() {
            state_bins_with_actions
                .entry(*state_bin)
                .or_insert_with(Vec::new)
                .push(*action);
        }

        // Phase 3: For each state bin with multiple actions, compare rewards
        let mut patterns = Vec::new();

        for (state_bin, actions) in state_bins_with_actions {
            if actions.len() < 2 {
                continue; // Need at least 2 different actions to compare
            }

            // Get unique actions
            let mut unique_actions = actions.clone();
            unique_actions.sort();
            unique_actions.dedup();

            // Compare all pairs
            for i in 0..unique_actions.len() {
                for j in (i + 1)..unique_actions.len() {
                    let action_a = unique_actions[i];
                    let action_b = unique_actions[j];

                    let rewards_a = &state_action_rewards[&(state_bin, action_a)];
                    let rewards_b = &state_action_rewards[&(state_bin, action_b)];

                    // Check minimum samples
                    if rewards_a.len() < self.config.min_samples
                        || rewards_b.len() < self.config.min_samples {
                        continue;
                    }

                    // Calculate statistics
                    let mean_a = rewards_a.iter().sum::<f32>() / rewards_a.len() as f32;
                    let mean_b = rewards_b.iter().sum::<f32>() / rewards_b.len() as f32;
                    let delta = (mean_a - mean_b).abs() as f64;

                    if delta < self.config.min_reward_delta {
                        continue; // Not significant enough
                    }

                    // Calculate confidence using simplified t-test
                    let var_a = self.variance(rewards_a, mean_a);
                    let var_b = self.variance(rewards_b, mean_b);
                    let confidence = self.calculate_confidence(
                        mean_a, var_a, rewards_a.len(),
                        mean_b, var_b, rewards_b.len()
                    );

                    // Create pattern
                    let (better_action, worse_action) = if mean_a > mean_b {
                        (action_a, action_b)
                    } else {
                        (action_b, action_a)
                    };

                    patterns.push(IdentifiedPattern {
                        state_bin_id: state_bin,
                        better_action,
                        worse_action,
                        reward_delta: delta,
                        confidence,
                        sample_count: rewards_a.len() + rewards_b.len(),
                    });
                }
            }
        }

        // Sort by confidence * reward_delta (importance score)
        patterns.sort_by(|a, b| {
            let score_a = a.confidence * a.reward_delta;
            let score_b = b.confidence * b.reward_delta;
            score_b.partial_cmp(&score_a).unwrap_or(std::cmp::Ordering::Equal)
        });

        Ok(patterns)
    }

    /// Quantize continuous state into discrete bin
    fn quantize_state(&self, state: &[f32; 8]) -> u64 {
        let mut bin_id: u64 = 0;
        let bins_per_dim = self.config.state_bins_per_dim as u64;

        for (i, &value) in state.iter().enumerate() {
            // Normalize value to [0, 1] assuming state values are roughly in [-1, 1]
            let normalized = ((value + 1.0) / 2.0).clamp(0.0, 0.999);
            let bin = (normalized * bins_per_dim as f32) as u64;

            // Encode in base-N where N = bins_per_dim
            bin_id = bin_id * bins_per_dim + bin;
        }

        bin_id
    }

    /// Calculate variance
    fn variance(&self, values: &[f32], mean: f32) -> f32 {
        if values.len() <= 1 {
            return 0.0;
        }

        let sum_sq_diff: f32 = values.iter()
            .map(|&x| (x - mean).powi(2))
            .sum();

        sum_sq_diff / (values.len() - 1) as f32
    }

    /// Calculate confidence using simplified t-test
    fn calculate_confidence(
        &self,
        mean_a: f32,
        var_a: f32,
        n_a: usize,
        mean_b: f32,
        var_b: f32,
        n_b: usize,
    ) -> f64 {
        // Pooled standard error
        let se = ((var_a / n_a as f32) + (var_b / n_b as f32)).sqrt();

        if se == 0.0 {
            return 0.0;
        }

        // t-statistic
        let t = ((mean_a - mean_b).abs() / se) as f64;

        // Simplified confidence mapping (approximation)
        // t > 3.0 => very high confidence (~99%)
        // t > 2.0 => high confidence (~95%)
        // t > 1.0 => moderate confidence (~68%)
        let confidence = (t / 3.0).min(1.0);

        confidence
    }

    /// Convert patterns to concrete proposals
    fn generate_proposals_from_patterns(
        &self,
        patterns: Vec<IdentifiedPattern>
    ) -> Result<Vec<Proposal>, String> {
        let mut proposals = Vec::new();

        for pattern in patterns {
            let target_entity_id = format!("adna_state_bin_{}", pattern.state_bin_id);

            let proposed_change = serde_json::json!({
                "op": "replace",
                "path": "/action_weights",
                "value": {
                    pattern.better_action.to_string(): 0.8,
                    pattern.worse_action.to_string(): 0.2,
                }
            });

            let justification = format!(
                "Statistical analysis of {} samples in state bin {}: \
                action {} outperforms action {} by {:.2} reward (confidence: {:.1}%)",
                pattern.sample_count,
                pattern.state_bin_id,
                pattern.better_action,
                pattern.worse_action,
                pattern.reward_delta,
                pattern.confidence * 100.0
            );

            let proposal = Proposal::new(
                target_entity_id,
                proposed_change,
                justification,
                pattern.reward_delta,
                pattern.confidence,
            );

            proposals.push(proposal);
        }

        Ok(proposals)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::experience_stream::ExperienceEvent;
    use crate::adna::InMemoryADNAReader;

    #[test]
    fn test_config_default() {
        let config = IntuitionConfig::default();
        assert_eq!(config.analysis_interval_secs, 60);
        assert_eq!(config.batch_size, 1000);
        assert_eq!(config.min_confidence, 0.7);
    }

    #[test]
    fn test_quantize_state() {
        let config = IntuitionConfig::default();
        let stream = Arc::new(ExperienceStream::new(1000, 100));
        let dna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let (tx, _rx) = mpsc::channel(100);

        let engine = IntuitionEngine::new(config, stream, dna_reader, tx);

        // Test state quantization
        let state1 = [0.0; 8]; // All zeros
        let state2 = [1.0; 8]; // All ones
        let state3 = [-1.0; 8]; // All negative ones

        let bin1 = engine.quantize_state(&state1);
        let bin2 = engine.quantize_state(&state2);
        let bin3 = engine.quantize_state(&state3);

        // Different states should map to different bins
        assert_ne!(bin1, bin2);
        assert_ne!(bin1, bin3);
        assert_ne!(bin2, bin3);
    }

    #[test]
    fn test_variance() {
        let config = IntuitionConfig::default();
        let stream = Arc::new(ExperienceStream::new(1000, 100));
        let dna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let (tx, _rx) = mpsc::channel(100);

        let engine = IntuitionEngine::new(config, stream, dna_reader, tx);

        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let mean = 3.0;
        let var = engine.variance(&values, mean);

        // Variance of [1,2,3,4,5] should be 2.5
        assert!((var - 2.5).abs() < 0.01);
    }

    #[test]
    fn test_pattern_detection() {
        let config = IntuitionConfig {
            min_samples: 3,
            min_reward_delta: 0.5,
            ..Default::default()
        };

        let stream = Arc::new(ExperienceStream::new(1000, 100));
        let dna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let (tx, _rx) = mpsc::channel(100);

        // Write test events: action 1 consistently gets higher rewards than action 2
        for i in 0..20 {
            let mut event = ExperienceEvent::default();
            event.state = [0.5; 8]; // Same state

            if i < 10 {
                event.event_type = 1; // Action 1
                event.reward_homeostasis = 5.0;
                event.reward_curiosity = 0.0;
                event.reward_efficiency = 0.0;
                event.reward_goal = 0.0;
            } else {
                event.event_type = 2; // Action 2
                event.reward_homeostasis = 1.0;
                event.reward_curiosity = 0.0;
                event.reward_efficiency = 0.0;
                event.reward_goal = 0.0;
            }

            stream.write_event(event).unwrap();
        }

        let engine = IntuitionEngine::new(config, stream.clone(), dna_reader, tx);

        // Sample and analyze
        let batch = stream.sample_batch(20, SamplingStrategy::Uniform);
        let patterns = engine.find_patterns_in_batch(&batch).unwrap();

        // Should find pattern: action 1 > action 2
        assert!(patterns.len() > 0);

        if let Some(pattern) = patterns.first() {
            assert_eq!(pattern.better_action, 1);
            assert_eq!(pattern.worse_action, 2);
            assert!(pattern.reward_delta > 3.0); // Difference should be ~4.0
            assert!(pattern.confidence > 0.5);
        }
    }

    // ==================== Auto Consolidation Tests ====================

    #[test]
    fn test_auto_consolidate_eligible() {
        let config = IntuitionConfig::default();
        let stream = Arc::new(ExperienceStream::new(1000, 100));
        let dna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let (tx, _rx) = mpsc::channel(100);

        let mut engine = IntuitionEngine::new(config, stream, dna_reader, tx);
        let guardian = crate::Guardian::new();

        // Create eligible connection: high confidence, evidence, learnable
        let mut connection = ConnectionV3::new(1, 2);
        connection.confidence = 200;  // ~78% > 75% threshold
        connection.evidence_count = 15;  // > 10 threshold
        connection.mutability = 1;  // Learnable
        connection.pull_strength = 5.0;
        connection.rigidity = 180;

        let state_token = Token::new(100);

        // Should consolidate
        let result = engine.try_auto_consolidate(&state_token, &connection, Some(&guardian));
        assert!(result, "Eligible connection should consolidate");

        // Verify reflex was added
        let stats = engine.get_stats();
        assert_eq!(stats.reflexes_created, 1);
        assert_eq!(stats.total_reflexes, 1);
    }

    #[test]
    fn test_auto_consolidate_low_confidence() {
        let config = IntuitionConfig::default();
        let stream = Arc::new(ExperienceStream::new(1000, 100));
        let dna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let (tx, _rx) = mpsc::channel(100);

        let mut engine = IntuitionEngine::new(config, stream, dna_reader, tx);

        // Low confidence (<75%)
        let mut connection = ConnectionV3::new(1, 2);
        connection.confidence = 150;  // ~59% < 75% threshold
        connection.evidence_count = 15;
        connection.mutability = 1;
        connection.pull_strength = 5.0;
        connection.rigidity = 180;

        let state_token = Token::new(100);

        // Should NOT consolidate
        let result = engine.try_auto_consolidate(&state_token, &connection, None);
        assert!(!result, "Low confidence should not consolidate");

        let stats = engine.get_stats();
        assert_eq!(stats.reflexes_created, 0);
    }

    #[test]
    fn test_auto_consolidate_low_evidence() {
        let config = IntuitionConfig::default();
        let stream = Arc::new(ExperienceStream::new(1000, 100));
        let dna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let (tx, _rx) = mpsc::channel(100);

        let mut engine = IntuitionEngine::new(config, stream, dna_reader, tx);

        // Low evidence count
        let mut connection = ConnectionV3::new(1, 2);
        connection.confidence = 200;
        connection.evidence_count = 5;  // < 10 threshold
        connection.mutability = 1;
        connection.pull_strength = 5.0;
        connection.rigidity = 180;

        let state_token = Token::new(100);

        // Should NOT consolidate
        let result = engine.try_auto_consolidate(&state_token, &connection, None);
        assert!(!result, "Low evidence should not consolidate");

        let stats = engine.get_stats();
        assert_eq!(stats.reflexes_created, 0);
    }

    #[test]
    fn test_auto_consolidate_hypothesis() {
        let config = IntuitionConfig::default();
        let stream = Arc::new(ExperienceStream::new(1000, 100));
        let dna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let (tx, _rx) = mpsc::channel(100);

        let mut engine = IntuitionEngine::new(config, stream, dna_reader, tx);

        // Hypothesis connection (not stable enough)
        let mut connection = ConnectionV3::new(1, 2);
        connection.confidence = 200;
        connection.evidence_count = 15;
        connection.mutability = 2;  // Hypothesis - not allowed!
        connection.pull_strength = 5.0;
        connection.rigidity = 180;

        let state_token = Token::new(100);

        // Should NOT consolidate
        let result = engine.try_auto_consolidate(&state_token, &connection, None);
        assert!(!result, "Hypothesis should not consolidate");

        let stats = engine.get_stats();
        assert_eq!(stats.reflexes_created, 0);
    }

    #[test]
    fn test_auto_consolidate_guardian_rejection() {
        let config = IntuitionConfig::default();
        let stream = Arc::new(ExperienceStream::new(1000, 100));
        let dna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let (tx, _rx) = mpsc::channel(100);

        let mut engine = IntuitionEngine::new(config, stream, dna_reader, tx);
        let guardian = crate::Guardian::new();

        // Connection that passes basic checks but fails Guardian validation
        let mut connection = ConnectionV3::new(1, 2);
        connection.confidence = 200;
        connection.evidence_count = 15;
        connection.mutability = 1;
        connection.pull_strength = 5.0;
        connection.rigidity = 100;  // Too low - Guardian will reject

        let state_token = Token::new(100);

        // Should NOT consolidate (Guardian rejects)
        let result = engine.try_auto_consolidate(&state_token, &connection, Some(&guardian));
        assert!(!result, "Guardian rejection should prevent consolidation");

        let stats = engine.get_stats();
        assert_eq!(stats.reflexes_created, 0);
    }
}