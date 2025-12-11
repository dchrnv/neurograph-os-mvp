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

//! IntuitionEngine Benchmarks for v0.27.0
//!
//! Performance measurements for IntuitionEngine operations:
//! - homeostasis_appraisal: <100 ns target
//! - curiosity_appraisal: <100 ns target
//! - efficiency_appraisal: <100 ns target
//! - goal_directed_appraisal: <100 ns target
//! - pattern_detection: <10 ms target (1k events)

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use neurograph_core::{ExperienceEvent, EventType, IntuitionConfig, ExperienceBatch};

/// Helper: Create a test batch with varying patterns
fn create_test_batch(size: usize) -> ExperienceBatch {
    let mut events = Vec::with_capacity(size);

    for i in 0..size {
        let mut event = ExperienceEvent::default();
        event.event_id = i as u128;
        event.event_type = if i % 3 == 0 {
            EventType::ActionCompleted as u16
        } else if i % 3 == 1 {
            EventType::ActionStarted as u16
        } else {
            EventType::ActionFailed as u16
        };

        event.episode_id = (i / 100) as u64;
        event.step_number = i as u32;

        // Create pattern: state values correlate with reward
        event.state = [
            (i as f32 * 0.01) % 2.0 - 1.0,
            (i as f32 * 0.02) % 2.0 - 1.0,
            (i as f32 * 0.03) % 2.0 - 1.0,
            (i as f32 * 0.04) % 2.0 - 1.0,
            (i as f32 * 0.05) % 2.0 - 1.0,
            (i as f32 * 0.06) % 2.0 - 1.0,
            (i as f32 * 0.07) % 2.0 - 1.0,
            (i as f32 * 0.08) % 2.0 - 1.0,
        ];

        // Rewards based on event type (creating a learnable pattern)
        event.reward_homeostasis = if event.event_type == EventType::ActionCompleted as u16 {
            0.8
        } else {
            0.2
        };

        event.reward_curiosity = if event.event_type == EventType::ActionStarted as u16 {
            0.7
        } else {
            0.3
        };

        event.reward_efficiency = 0.5;
        event.reward_goal = 0.4;

        events.push(event);
    }

    ExperienceBatch { events }
}

/// Benchmark: Homeostasis appraisal (single event, target: <100 ns)
fn bench_homeostasis_appraisal(c: &mut Criterion) {
    let event = ExperienceEvent::default();

    c.bench_function("homeostasis_appraisal", |b| {
        b.iter(|| {
            // Simplified appraisal logic (checking L5, L6, L8)
            let state = black_box(&event.state);
            let l5 = state[4]; // L5 Cognitive
            let l6 = state[5]; // L6 Social
            let l8 = state[7]; // L8 Abstract

            // Simple homeostasis check: balance between cognitive load
            let cognitive_balance = (l5.abs() + l6.abs() + l8.abs()) / 3.0;
            let reward = 1.0 - cognitive_balance.clamp(0.0, 1.0);
            black_box(reward)
        })
    });
}

/// Benchmark: Curiosity appraisal (novelty detection, target: <100 ns)
fn bench_curiosity_appraisal(c: &mut Criterion) {
    let event = ExperienceEvent::default();

    c.bench_function("curiosity_appraisal", |b| {
        b.iter(|| {
            // Simplified novelty check (L2 Sensory)
            let state = black_box(&event.state);
            let l2 = state[1]; // L2 Sensory

            // Higher sensory values = more novel
            let novelty = l2.abs().clamp(0.0, 1.0);
            black_box(novelty)
        })
    });
}

/// Benchmark: Efficiency appraisal (L3 + L5, target: <100 ns)
fn bench_efficiency_appraisal(c: &mut Criterion) {
    let event = ExperienceEvent::default();

    c.bench_function("efficiency_appraisal", |b| {
        b.iter(|| {
            // Simplified efficiency check (L3 Motor + L5 Cognitive)
            let state = black_box(&event.state);
            let l3 = state[2]; // L3 Motor
            let l5 = state[4]; // L5 Cognitive

            // Lower values = more efficient
            let efficiency = 1.0 - ((l3.abs() + l5.abs()) / 2.0).clamp(0.0, 1.0);
            black_box(efficiency)
        })
    });
}

/// Benchmark: Goal-directed appraisal (L7, target: <100 ns)
fn bench_goal_directed_appraisal(c: &mut Criterion) {
    let event = ExperienceEvent::default();

    c.bench_function("goal_directed_appraisal", |b| {
        b.iter(|| {
            // Simplified goal check (L7 Temporal)
            let state = black_box(&event.state);
            let l7 = state[6]; // L7 Temporal

            // Temporal consistency = goal progress
            let goal_progress = l7.abs().clamp(0.0, 1.0);
            black_box(goal_progress)
        })
    });
}

/// Benchmark: State quantization (4 bins per dimension = 4^8 total bins)
fn bench_state_quantization(c: &mut Criterion) {
    let config = IntuitionConfig::default();
    let event = ExperienceEvent::default();

    c.bench_function("state_quantization", |b| {
        b.iter(|| {
            // Quantize state into bin
            let state = black_box(&event.state);
            let mut bin_id: u64 = 0;
            let bins_per_dim = config.state_bins_per_dim as u64;

            for (i, &value) in state.iter().enumerate() {
                let normalized = ((value + 1.0) / 2.0).clamp(0.0, 0.999);
                let bin = (normalized * bins_per_dim as f32) as u64;
                bin_id = bin_id * bins_per_dim + bin;
            }

            black_box(bin_id)
        })
    });
}

/// Benchmark: Pattern detection in batch (target: <10 ms for 1k events)
fn bench_pattern_detection(c: &mut Criterion) {
    use std::collections::HashMap;

    let mut group = c.benchmark_group("pattern_detection");

    for size in [100, 500, 1000].iter() {
        let batch = create_test_batch(*size);

        group.bench_with_input(
            BenchmarkId::from_parameter(size),
            size,
            |b, _| {
                b.iter(|| {
                    // Simplified pattern detection: quantize states and aggregate rewards
                    let mut state_action_rewards: HashMap<(u64, u16), Vec<f32>> = HashMap::new();
                    let bins_per_dim = 4u64;

                    for event in &batch.events {
                        // Quantize state
                        let mut bin_id: u64 = 0;
                        for &value in &event.state {
                            let normalized = ((value + 1.0) / 2.0).clamp(0.0, 0.999);
                            let bin = (normalized * bins_per_dim as f32) as u64;
                            bin_id = bin_id * bins_per_dim + bin;
                        }

                        let action = event.event_type;
                        let total_reward = event.total_reward();

                        state_action_rewards
                            .entry((bin_id, action))
                            .or_insert_with(Vec::new)
                            .push(total_reward);
                    }

                    // Count patterns found
                    let pattern_count = state_action_rewards.len();
                    black_box(pattern_count)
                })
            },
        );
    }

    group.finish();
}

/// Benchmark: Statistical comparison (t-test for action comparison)
fn bench_statistical_comparison(c: &mut Criterion) {
    // Create two reward distributions
    let rewards_a: Vec<f32> = (0..100).map(|i| 0.6 + (i as f32 * 0.001)).collect();
    let rewards_b: Vec<f32> = (0..100).map(|i| 0.4 + (i as f32 * 0.001)).collect();

    c.bench_function("statistical_comparison", |b| {
        b.iter(|| {
            // Calculate means
            let mean_a = black_box(&rewards_a).iter().sum::<f32>() / rewards_a.len() as f32;
            let mean_b = black_box(&rewards_b).iter().sum::<f32>() / rewards_b.len() as f32;

            // Calculate variances
            let var_a: f32 = rewards_a.iter()
                .map(|&x| (x - mean_a).powi(2))
                .sum::<f32>() / (rewards_a.len() - 1) as f32;

            let var_b: f32 = rewards_b.iter()
                .map(|&x| (x - mean_b).powi(2))
                .sum::<f32>() / (rewards_b.len() - 1) as f32;

            // Calculate t-statistic
            let pooled_se = ((var_a / rewards_a.len() as f32) +
                            (var_b / rewards_b.len() as f32)).sqrt();
            let t_stat = (mean_a - mean_b) / pooled_se;

            black_box(t_stat)
        })
    });
}

criterion_group!(
    benches,
    bench_homeostasis_appraisal,
    bench_curiosity_appraisal,
    bench_efficiency_appraisal,
    bench_goal_directed_appraisal,
    bench_state_quantization,
    bench_pattern_detection,
    bench_statistical_comparison
);
criterion_main!(benches);