/// ExperienceStream Demo
///
/// This demo shows the complete ExperienceStream v2.1 workflow:
/// 1. Creating an ExperienceStream
/// 2. Writing events
/// 3. Multiple appraisers subscribing and processing events concurrently
/// 4. Lock-free parallel reward updates
/// 5. Reading back enriched events
/// 6. Performance benchmarks

use neurograph_core::{
    ExperienceStream, ExperienceEvent, EventType, AppraiserType,
};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// Simulates a Homeostasis Appraiser
/// Evaluates events based on internal state stability
async fn homeostasis_appraiser(stream: Arc<ExperienceStream>) {
    println!("[Homeostasis] Starting appraiser...");
    let mut rx = stream.subscribe();

    let mut processed = 0;
    loop {
        match rx.recv().await {
            Ok(event) => {
                // Simulate processing time (state analysis)
                sleep(Duration::from_micros(100)).await;

                // Calculate homeostasis reward based on state vector
                let state_change: f32 = event.state.iter().map(|x| x.abs()).sum();
                let reward = if state_change < 1.0 {
                    1.0 - state_change  // Reward stability
                } else {
                    -0.5  // Penalize instability
                };

                // Update reward in event (lock-free!)
                let seq = processed as u64;  // In real system, track sequence from write
                if let Err(e) = stream.set_appraiser_reward(seq, AppraiserType::Homeostasis, reward) {
                    println!("[Homeostasis] Error setting reward: {}", e);
                }

                processed += 1;
                println!("[Homeostasis] Event {} → reward: {:.3}", event.event_id, reward);
            }
            Err(_) => break,
        }
    }
}

/// Simulates a Curiosity Appraiser
/// Rewards novel, unexpected situations
async fn curiosity_appraiser(stream: Arc<ExperienceStream>) {
    println!("[Curiosity] Starting appraiser...");
    let mut rx = stream.subscribe();

    let mut processed = 0;
    loop {
        match rx.recv().await {
            Ok(event) => {
                sleep(Duration::from_micros(150)).await;

                // Calculate curiosity reward based on action magnitude
                let action_magnitude: f32 = event.action.iter().map(|x| x * x).sum::<f32>().sqrt();
                let reward = (action_magnitude / 3.0).min(1.0);  // Novel actions = higher curiosity

                let seq = processed as u64;
                if let Err(e) = stream.set_appraiser_reward(seq, AppraiserType::Curiosity, reward) {
                    println!("[Curiosity] Error setting reward: {}", e);
                }

                processed += 1;
                println!("[Curiosity] Event {} → reward: {:.3}", event.event_id, reward);
            }
            Err(_) => break,
        }
    }
}

/// Simulates an Efficiency Appraiser
/// Evaluates resource usage and optimization
async fn efficiency_appraiser(stream: Arc<ExperienceStream>) {
    println!("[Efficiency] Starting appraiser...");
    let mut rx = stream.subscribe();

    let mut processed = 0;
    loop {
        match rx.recv().await {
            Ok(event) => {
                sleep(Duration::from_micros(80)).await;

                // Calculate efficiency based on action economy
                let action_sum: f32 = event.action.iter().map(|x| x.abs()).sum();
                let reward = if action_sum < 2.0 {
                    0.8  // Efficient action
                } else {
                    0.2  // Wasteful action
                };

                let seq = processed as u64;
                if let Err(e) = stream.set_appraiser_reward(seq, AppraiserType::Efficiency, reward) {
                    println!("[Efficiency] Error setting reward: {}", e);
                }

                processed += 1;
                println!("[Efficiency] Event {} → reward: {:.3}", event.event_id, reward);
            }
            Err(_) => break,
        }
    }
}

/// Simulates a Goal-Directed Appraiser
/// Evaluates progress toward explicit goals
async fn goal_directed_appraiser(stream: Arc<ExperienceStream>) {
    println!("[Goal] Starting appraiser...");
    let mut rx = stream.subscribe();

    let goal_state = [1.0f32, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];  // Target state

    let mut processed = 0;
    loop {
        match rx.recv().await {
            Ok(event) => {
                sleep(Duration::from_micros(120)).await;

                // Calculate goal progress (distance to target state)
                let distance: f32 = event.state.iter()
                    .zip(goal_state.iter())
                    .map(|(s, g)| (s - g) * (s - g))
                    .sum::<f32>()
                    .sqrt();

                let reward = (2.0 - distance).max(0.0).min(1.0);  // Closer to goal = higher reward

                let seq = processed as u64;
                if let Err(e) = stream.set_appraiser_reward(seq, AppraiserType::Goal, reward) {
                    println!("[Goal] Error setting reward: {}", e);
                }

                processed += 1;
                println!("[Goal] Event {} → reward: {:.3}", event.event_id, reward);
            }
            Err(_) => break,
        }
    }
}

/// Demo 1: Basic workflow with all 4 appraisers
async fn demo_basic_workflow() {
    println!("\n=== Demo 1: Basic Workflow ===\n");

    // Create ExperienceStream with small buffer for demo
    let stream = Arc::new(ExperienceStream::new(16, 32));
    println!("✓ Created ExperienceStream (capacity: 16, channel: 32)");

    // Spawn all 4 appraisers concurrently
    let stream_clone = Arc::clone(&stream);
    let h1 = tokio::spawn(async move {
        homeostasis_appraiser(stream_clone).await;
    });

    let stream_clone = Arc::clone(&stream);
    let h2 = tokio::spawn(async move {
        curiosity_appraiser(stream_clone).await;
    });

    let stream_clone = Arc::clone(&stream);
    let h3 = tokio::spawn(async move {
        efficiency_appraiser(stream_clone).await;
    });

    let stream_clone = Arc::clone(&stream);
    let h4 = tokio::spawn(async move {
        goal_directed_appraiser(stream_clone).await;
    });

    println!("✓ Spawned 4 concurrent appraisers");

    // Give appraisers time to subscribe
    sleep(Duration::from_millis(100)).await;

    // Write 5 test events
    println!("\n--- Writing Events ---\n");
    for i in 0..5 {
        let mut event = ExperienceEvent {
            event_id: (i + 1) as u128,
            timestamp: i as u64 * 1000,
            episode_id: 1,
            step_number: i,
            event_type: EventType::TokenCreated as u16,
            flags: 0,
            state: [
                (i as f32) * 0.2,
                0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7
            ],
            action: [
                (i as f32) * 0.15,
                0.05, 0.1, 0.15, 0.2, 0.25, 0.3, 0.35
            ],
            reward_homeostasis: 0.0,
            reward_curiosity: 0.0,
            reward_efficiency: 0.0,
            reward_goal: 0.0,
            adna_version_hash: 0x12345678,
            _reserved: [0; 4],
        };

        match stream.write_event(event) {
            Ok(seq) => println!("→ Wrote event {} (seq: {})", event.event_id, seq),
            Err(e) => println!("✗ Error writing event: {}", e),
        }

        // Small delay to let appraisers process
        sleep(Duration::from_millis(50)).await;
    }

    // Wait for appraisers to finish processing
    println!("\n--- Waiting for appraisers to complete ---\n");
    sleep(Duration::from_millis(500)).await;

    // Read back events and show final rewards
    println!("\n--- Final Enriched Events ---\n");
    for seq in 0..5 {
        match stream.get_event(seq) {
            Some(event) => {
                println!("Event {} (seq: {}):", event.event_id, seq);
                println!("  Homeostasis:  {:.3}", event.reward_homeostasis);
                println!("  Curiosity:    {:.3}", event.reward_curiosity);
                println!("  Efficiency:   {:.3}", event.reward_efficiency);
                println!("  Goal:         {:.3}", event.reward_goal);
                println!("  TOTAL REWARD: {:.3}", event.total_reward());
                println!();
            }
            None => println!("Error reading seq {}: event not found", seq),
        }
    }

    // Drop stream to close channels and terminate appraisers
    drop(stream);

    // Wait for all appraisers to exit
    let _ = tokio::join!(h1, h2, h3, h4);

    println!("✓ Demo 1 completed\n");
}

/// Demo 2: Performance benchmark
async fn demo_performance_benchmark() {
    println!("\n=== Demo 2: Performance Benchmark ===\n");

    const BUFFER_SIZE: usize = 10_000;
    const NUM_EVENTS: usize = 50_000;

    let stream = Arc::new(ExperienceStream::new(BUFFER_SIZE, 1024));
    println!("✓ Created ExperienceStream (capacity: {}, channel: 1024)", BUFFER_SIZE);

    // Spawn passive subscriber to drain channel (prevent backpressure)
    let stream_clone = Arc::clone(&stream);
    let subscriber_handle = tokio::spawn(async move {
        let mut rx = stream_clone.subscribe();
        let mut count = 0;
        while let Ok(_) = rx.recv().await {
            count += 1;
            if count >= NUM_EVENTS {
                break;
            }
        }
    });

    // Benchmark: Write throughput
    println!("\n--- Benchmark: Write Throughput ---");
    let template_event = ExperienceEvent {
        event_id: 1,
        timestamp: 0,
        episode_id: 1,
        step_number: 0,
        event_type: EventType::TokenCreated as u16,
        flags: 0,
        state: [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8],
        action: [0.05, 0.1, 0.15, 0.2, 0.25, 0.3, 0.35, 0.4],
        reward_homeostasis: 0.0,
        reward_curiosity: 0.0,
        reward_efficiency: 0.0,
        reward_goal: 0.0,
        adna_version_hash: 0xABCDEF00,
        _reserved: [0; 4],
    };

    let start = Instant::now();
    for i in 0..NUM_EVENTS {
        let mut event = template_event.clone();
        event.event_id = i as u128;
        event.timestamp = i as u64;
        let _ = stream.write_event(event);
    }
    let duration = start.elapsed();

    let throughput = NUM_EVENTS as f64 / duration.as_secs_f64();
    println!("  Wrote {} events in {:?}", NUM_EVENTS, duration);
    println!("  Throughput: {:.2} events/sec", throughput);
    println!("  Latency: {:.2} µs/event", duration.as_micros() as f64 / NUM_EVENTS as f64);

    // Benchmark: Read throughput
    println!("\n--- Benchmark: Read Throughput ---");
    let start = Instant::now();
    let mut successful_reads = 0;
    for i in 0..NUM_EVENTS {
        let seq = i as u64;
        if stream.get_event(seq).is_some() {
            successful_reads += 1;
        }
    }
    let duration = start.elapsed();

    let throughput = successful_reads as f64 / duration.as_secs_f64();
    println!("  Read {} events in {:?}", successful_reads, duration);
    println!("  Throughput: {:.2} events/sec", throughput);
    println!("  Latency: {:.2} µs/event", duration.as_micros() as f64 / successful_reads as f64);

    // Note: successful_reads < NUM_EVENTS due to circular buffer overflow
    let overflow_events = NUM_EVENTS - successful_reads;
    println!("  Buffer overflowed: {} events lost (expected with circular buffer)", overflow_events);

    // Benchmark: Reward update throughput (lock-free writes)
    println!("\n--- Benchmark: Reward Updates (Lock-Free) ---");
    let start = Instant::now();
    let mut successful_updates = 0;
    for i in 0..NUM_EVENTS {
        let seq = i as u64;
        // All 4 appraisers update in parallel (simulated sequentially here)
        if stream.set_appraiser_reward(seq, AppraiserType::Homeostasis, 0.5).is_ok() {
            let _ = stream.set_appraiser_reward(seq, AppraiserType::Curiosity, 0.6);
            let _ = stream.set_appraiser_reward(seq, AppraiserType::Efficiency, 0.7);
            let _ = stream.set_appraiser_reward(seq, AppraiserType::Goal, 0.8);
            successful_updates += 1;
        }
    }
    let duration = start.elapsed();

    let throughput = (successful_updates * 4) as f64 / duration.as_secs_f64();
    println!("  Updated {} events (4 rewards each) in {:?}", successful_updates, duration);
    println!("  Throughput: {:.2} updates/sec", throughput);
    println!("  Latency: {:.2} µs/update", duration.as_micros() as f64 / (successful_updates * 4) as f64);

    // Wait for subscriber to finish
    let _ = subscriber_handle.await;

    println!("\n✓ Benchmark completed\n");
}

/// Demo 3: Circular buffer overflow behavior
async fn demo_overflow_behavior() {
    println!("\n=== Demo 3: Circular Buffer Overflow ===\n");

    const SMALL_BUFFER: usize = 4;
    const EVENTS_TO_WRITE: usize = 10;

    let stream = Arc::new(ExperienceStream::new(SMALL_BUFFER, 16));
    println!("✓ Created small ExperienceStream (capacity: {})", SMALL_BUFFER);

    let template_event = ExperienceEvent::default();

    println!("\n--- Writing {} events to buffer of size {} ---\n", EVENTS_TO_WRITE, SMALL_BUFFER);
    for i in 0..EVENTS_TO_WRITE {
        let mut event = template_event.clone();
        event.event_id = (i + 1) as u128;
        event.step_number = i as u32;

        match stream.write_event(event) {
            Ok(seq) => println!("→ Wrote event {} (seq: {})", event.event_id, seq),
            Err(e) => println!("✗ Error: {}", e),
        }
    }

    println!("\n--- Reading back events ---\n");
    println!("Note: Only last {} events should be readable\n", SMALL_BUFFER);

    for seq in 0..EVENTS_TO_WRITE as u64 {
        match stream.get_event(seq) {
            Some(event) => {
                println!("✓ seq {} → event {} (step {})", seq, event.event_id, event.step_number);
            }
            None => {
                println!("✗ seq {} → event not found (overwritten)", seq);
            }
        }
    }

    println!("\n--- Query recent events (last 4) ---\n");
    let events = stream.query_range(6, 10);  // start=6, end=10 (gets seq 6,7,8,9)
    println!("✓ Retrieved {} events:", events.len());
    for event in events {
        println!("  event {} (step {})", event.event_id, event.step_number);
    }

    println!("\n✓ Demo 3 completed\n");
}

#[tokio::main]
async fn main() {
    println!("╔════════════════════════════════════════════════════╗");
    println!("║   ExperienceStream v2.1 Demo                      ║");
    println!("║   NeuroGraph OS Core - Event-Based Memory         ║");
    println!("╚════════════════════════════════════════════════════╝");

    // Run all demos
    demo_basic_workflow().await;
    demo_performance_benchmark().await;
    demo_overflow_behavior().await;

    println!("╔════════════════════════════════════════════════════╗");
    println!("║   All demos completed successfully!               ║");
    println!("╚════════════════════════════════════════════════════╝\n");
}