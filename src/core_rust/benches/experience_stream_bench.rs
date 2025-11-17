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

//! ExperienceStream Benchmarks for v0.27.0
//!
//! Performance measurements for ExperienceStream operations:
//! - write_event: <200 ns target (lock-free circular buffer)
//! - write_event_with_metadata: <500 ns target (HashMap insert)
//! - read_event: <100 ns target (direct array access)
//! - sample_batch_uniform: <50 μs target (100 from 10k)
//! - sample_batch_prioritized: <100 μs target (with sorting)
//! - broadcast_latency: <1 μs target

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use neurograph_core::{ExperienceStream, ExperienceEvent, EventType, ActionMetadata};
use serde_json::json;

/// Benchmark: Write event (target: <200 ns for lock-free circular buffer)
fn bench_write_event(c: &mut Criterion) {
    let stream = ExperienceStream::new(1_000_000);

    c.bench_function("write_event", |b| {
        let mut counter = 0u128;
        b.iter(|| {
            counter += 1;
            let mut event = ExperienceEvent::default();
            event.event_id = counter;
            event.event_type = EventType::ActionCompleted as u16;
            event.episode_id = 1;
            event.step_number = counter as u32;
            event.state = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
            event.action = [0.15, 0.25, 0.35, 0.45, 0.55, 0.65, 0.75, 0.85];
            stream.write_event(black_box(event))
        })
    });
}

/// Benchmark: Write event with metadata (target: <500 ns with HashMap insert)
fn bench_write_event_with_metadata(c: &mut Criterion) {
    let stream = ExperienceStream::new(1_000_000);

    c.bench_function("write_event_with_metadata", |b| {
        let mut counter = 0u128;
        b.iter(|| {
            counter += 1;
            let mut event = ExperienceEvent::default();
            event.event_id = counter;
            event.event_type = EventType::ActionStarted as u16;
            event.episode_id = 1;
            event.step_number = counter as u32;

            let metadata = ActionMetadata {
                intent_type: "send_message".to_string(),
                executor_id: "message_sender".to_string(),
                parameters: json!({
                    "message": "Hello",
                    "priority": "info"
                }),
            };

            stream.write_event_with_metadata(black_box(event), black_box(metadata))
        })
    });
}

/// Benchmark: Read event (target: <100 ns for direct array access)
fn bench_read_event(c: &mut Criterion) {
    let stream = ExperienceStream::new(1_000_000);

    // Pre-populate with 1000 events
    for i in 0..1000 {
        let mut event = ExperienceEvent::default();
        event.event_id = i;
        event.event_type = EventType::ActionCompleted as u16;
        stream.write_event(event);
    }

    c.bench_function("read_event", |b| {
        let mut seq = 0u64;
        b.iter(|| {
            seq = (seq + 1) % 1000;
            stream.read_event(black_box(seq))
        })
    });
}

/// Benchmark: Uniform sampling (target: <50 μs for 100 samples from 10k events)
fn bench_sample_batch_uniform(c: &mut Criterion) {
    let stream = ExperienceStream::new(1_000_000);

    // Pre-populate with 10,000 events
    for i in 0..10_000 {
        let mut event = ExperienceEvent::default();
        event.event_id = i;
        event.event_type = EventType::ActionCompleted as u16;
        event.reward_homeostasis = 0.5;
        event.reward_curiosity = 0.3;
        event.reward_efficiency = 0.2;
        event.reward_goal = 0.4;
        stream.write_event(event);
    }

    let mut group = c.benchmark_group("sample_batch_uniform");

    for batch_size in [10, 50, 100, 200].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(batch_size),
            batch_size,
            |b, &batch_size| {
                b.iter(|| stream.sample_uniform(black_box(batch_size)))
            },
        );
    }

    group.finish();
}

/// Benchmark: Prioritized sampling (target: <100 μs with sorting by reward)
fn bench_sample_batch_prioritized(c: &mut Criterion) {
    let stream = ExperienceStream::new(1_000_000);

    // Pre-populate with 10,000 events with varying rewards
    for i in 0..10_000 {
        let mut event = ExperienceEvent::default();
        event.event_id = i;
        event.event_type = EventType::ActionCompleted as u16;
        event.reward_homeostasis = (i as f32 * 0.0001) % 1.0;
        event.reward_curiosity = (i as f32 * 0.0002) % 1.0;
        event.reward_efficiency = (i as f32 * 0.0003) % 1.0;
        event.reward_goal = (i as f32 * 0.0004) % 1.0;
        stream.write_event(event);
    }

    let mut group = c.benchmark_group("sample_batch_prioritized");

    for batch_size in [10, 50, 100, 200].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(batch_size),
            batch_size,
            |b, &batch_size| {
                b.iter(|| stream.sample_prioritized(black_box(batch_size)))
            },
        );
    }

    group.finish();
}

/// Benchmark: Appraiser reward update (lock-free write to dedicated slot)
fn bench_set_appraiser_reward(c: &mut Criterion) {
    let stream = ExperienceStream::new(1_000_000);

    // Pre-populate with 1000 events
    for i in 0..1000 {
        let mut event = ExperienceEvent::default();
        event.event_id = i;
        event.event_type = EventType::ActionCompleted as u16;
        stream.write_event(event);
    }

    let mut group = c.benchmark_group("set_appraiser_reward");

    group.bench_function("homeostasis", |b| {
        let mut seq = 0u64;
        b.iter(|| {
            seq = (seq + 1) % 1000;
            stream
                .hot_buffer
                .set_appraiser_reward(
                    black_box(seq),
                    black_box(neurograph_core::AppraiserType::Homeostasis),
                    black_box(0.5),
                )
        })
    });

    group.bench_function("curiosity", |b| {
        let mut seq = 0u64;
        b.iter(|| {
            seq = (seq + 1) % 1000;
            stream
                .hot_buffer
                .set_appraiser_reward(
                    black_box(seq),
                    black_box(neurograph_core::AppraiserType::Curiosity),
                    black_box(0.3),
                )
        })
    });

    group.finish();
}

/// Benchmark: Query range of events
fn bench_query_range(c: &mut Criterion) {
    let stream = ExperienceStream::new(1_000_000);

    // Pre-populate with 10,000 events
    for i in 0..10_000 {
        let mut event = ExperienceEvent::default();
        event.event_id = i;
        event.event_type = EventType::ActionCompleted as u16;
        stream.write_event(event);
    }

    let mut group = c.benchmark_group("query_range");

    for range_size in [10, 50, 100, 500].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(range_size),
            range_size,
            |b, &range_size| {
                b.iter(|| {
                    stream
                        .hot_buffer
                        .query_range(black_box(0), black_box(range_size as u64))
                })
            },
        );
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_write_event,
    bench_write_event_with_metadata,
    bench_read_event,
    bench_sample_batch_uniform,
    bench_sample_batch_prioritized,
    bench_set_appraiser_reward,
    bench_query_range
);
criterion_main!(benches);