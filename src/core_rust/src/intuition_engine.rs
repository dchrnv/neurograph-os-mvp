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

//! IntuitionEngine v2.1 - Learning Loop Integration
//!
//! Analyzes accumulated experience from ExperienceStream, finds correlations
//! between actions and rewards, and generates Proposals to improve ADNA policies.
//!
//! # v1.0 Implementation: Statistical Analysis
//!
//! - State space quantization using simple grid binning
//! - Action-reward aggregation per state bin
//! - Statistical significance testing (t-test)
//! - Proposal generation based on significant patterns

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::mpsc;
use crate::experience_stream::{ExperienceStream, ExperienceBatch, SamplingStrategy};
use crate::adna::{ADNAReader, Proposal};

/// Configuration for IntuitionEngine
#[derive(Debug, Clone)]
pub struct IntuitionConfig {
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
}

impl Default for IntuitionConfig {
    fn default() -> Self {
        Self {
            analysis_interval_secs: 60, // Analyze every minute
            batch_size: 1000,
            sampling_strategy: SamplingStrategy::PrioritizedByReward { alpha: 1.0 },
            min_confidence: 0.7,
            max_proposals_per_cycle: 5,
            state_bins_per_dim: 4, // 4^8 = 65536 state bins (manageable)
            min_samples: 10,
            min_reward_delta: 0.5,
        }
    }
}

/// Identified pattern from batch analysis
#[derive(Debug, Clone)]
struct IdentifiedPattern {
    /// State cluster/bin ID
    state_bin_id: u64,

    /// Action type with better reward
    better_action: u16,

    /// Action type with worse reward
    worse_action: u16,

    /// Difference in mean rewards
    reward_delta: f64,

    /// Statistical confidence [0.0, 1.0]
    confidence: f64,

    /// Number of samples used
    sample_count: usize,
}

/// IntuitionEngine - Analyzes experience and generates improvement proposals
pub struct IntuitionEngine {
    config: IntuitionConfig,
    experience_stream: Arc<ExperienceStream>,
    _dna_reader: Arc<dyn ADNAReader>,
    proposal_sender: mpsc::Sender<Proposal>,
}

impl IntuitionEngine {
    /// Create new IntuitionEngine
    pub fn new(
        config: IntuitionConfig,
        experience_stream: Arc<ExperienceStream>,
        dna_reader: Arc<dyn ADNAReader>,
        proposal_sender: mpsc::Sender<Proposal>,
    ) -> Self {
        Self {
            config,
            experience_stream,
            _dna_reader: dna_reader,
            proposal_sender,
        }
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
}