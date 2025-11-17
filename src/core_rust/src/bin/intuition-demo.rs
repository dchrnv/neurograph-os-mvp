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

/// Intuition Module v2.2 Integration Demo
///
/// Demonstrates the complete Intuition Module with:
/// - ExperienceStream v2.1
/// - ADNA v3.1 with appraiser parameters
/// - 4 Appraisers running in parallel (HomeostasisAppraiser, CuriosityAppraiser,
///   EfficiencyAppraiser, GoalDirectedAppraiser)
/// - AppraiserSet coordinator
///
/// This demo simulates a series of experience events with different L1-L8
/// coordinate values and shows how the appraisers assign rewards in real-time.

use std::sync::Arc;
use tokio::time::{sleep, Duration};

use neurograph_core::{
    ExperienceStream, ExperienceEvent, CoordinateExt,
    InMemoryADNAReader, AppraiserSet,
};

#[tokio::main]
async fn main() {
    println!("=== Intuition Module v2.2 Integration Demo ===\n");

    // 1. Initialize ExperienceStream
    println!("[1] Initializing ExperienceStream (capacity=1000, channel_size=32)...");
    let stream = Arc::new(ExperienceStream::new(1000, 32));
    println!("    ✓ ExperienceStream initialized\n");

    // 2. Initialize ADNA Reader with default appraiser parameters
    println!("[2] Initializing ADNA v3.1 with default appraiser parameters...");
    let dna_reader = Arc::new(InMemoryADNAReader::with_defaults());
    println!("    ✓ ADNA v3.1 initialized\n");

    // 3. Create 4 broadcast receivers (one for each appraiser)
    println!("[3] Creating broadcast receivers for appraisers...");
    let homeostasis_rx = stream.subscribe();
    let curiosity_rx = stream.subscribe();
    let efficiency_rx = stream.subscribe();
    let goal_rx = stream.subscribe();
    println!("    ✓ Created 4 broadcast receivers\n");

    // 4. Launch AppraiserSet with all 4 appraisers
    println!("[4] Launching AppraiserSet with all 4 appraisers...");
    let appraiser_set = AppraiserSet::start(
        dna_reader.clone(),
        stream.clone(),
        homeostasis_rx,
        curiosity_rx,
        efficiency_rx,
        goal_rx,
    );
    println!("    ✓ All 4 appraisers running in parallel\n");

    // Small delay to ensure appraisers are ready
    sleep(Duration::from_millis(100)).await;

    // 5. Simulate experience events
    println!("[5] Simulating experience events...\n");

    // Event 1: High novelty (should trigger CuriosityAppraiser)
    println!("Event 1: High novelty (L2=0.9)");
    let mut event1 = ExperienceEvent::default();
    event1.state[1] = 0.9; // L2 Novelty = 0.9 (high)
    event1.state[4] = 0.5; // L5 Cognitive Load = 0.5 (normal)
    event1.state[5] = 0.7; // L6 Certainty = 0.7 (normal)
    event1.state[7] = 0.8; // L8 Coherence = 0.8 (normal)
    stream.write_event(event1).unwrap();
    println!("  → Expected: Curiosity reward (novelty above threshold)");
    sleep(Duration::from_millis(100)).await;

    // Event 2: Cognitive overload (should trigger HomeostasisAppraiser penalty)
    println!("\nEvent 2: Cognitive overload (L5=0.95, above target range)");
    let mut event2 = ExperienceEvent::default();
    event2.state[1] = 0.2; // L2 Novelty = 0.2 (low)
    event2.state[4] = 0.95; // L5 Cognitive Load = 0.95 (overload!)
    event2.state[5] = 0.7; // L6 Certainty = 0.7 (normal)
    event2.state[7] = 0.8; // L8 Coherence = 0.8 (normal)
    stream.write_event(event2).unwrap();
    println!("  → Expected: Homeostasis penalty (cognitive load too high)");
    sleep(Duration::from_millis(100)).await;

    // Event 3: High motor activity (should trigger EfficiencyAppraiser penalty)
    println!("\nEvent 3: High motor activity (L3 velocity=0.8, acceleration=0.6)");
    let mut event3 = ExperienceEvent::default();
    event3.state[1] = 0.2; // L2 Novelty = 0.2 (low)
    event3.state[2] = 0.8; // L3 Velocity = 0.8 (high!)
    event3.action[2] = 0.6; // L3 Acceleration = 0.6 (high!)
    event3.state[4] = 0.5; // L5 Cognitive Load = 0.5 (normal)
    event3.state[5] = 0.7; // L6 Certainty = 0.7 (normal)
    event3.state[7] = 0.8; // L8 Coherence = 0.8 (normal)
    stream.write_event(event3).unwrap();
    println!("  → Expected: Efficiency penalty (high resource usage)");
    sleep(Duration::from_millis(100)).await;

    // Event 4: High positive valence (should trigger GoalDirectedAppraiser reward)
    println!("\nEvent 4: High positive valence (L7=0.85, goal achievement)");
    let mut event4 = ExperienceEvent::default();
    event4.state[1] = 0.2; // L2 Novelty = 0.2 (low)
    event4.state[4] = 0.5; // L5 Cognitive Load = 0.5 (normal)
    event4.state[5] = 0.7; // L6 Certainty = 0.7 (normal)
    event4.state[6] = 0.85; // L7 Valence = 0.85 (very positive!)
    event4.state[7] = 0.8; // L8 Coherence = 0.8 (normal)
    stream.write_event(event4).unwrap();
    println!("  → Expected: Goal reward (high positive valence)");
    sleep(Duration::from_millis(100)).await;

    // Event 5: Perfect homeostasis (all coordinates in target ranges)
    println!("\nEvent 5: Perfect homeostasis (all coordinates in target ranges)");
    let mut event5 = ExperienceEvent::default();
    event5.state[1] = 0.1; // L2 Novelty = 0.1 (low, no curiosity reward)
    event5.state[4] = 0.5; // L5 Cognitive Load = 0.5 (in range [0.2, 0.7])
    event5.state[5] = 0.7; // L6 Certainty = 0.7 (in range [0.4, 0.9])
    event5.state[7] = 0.8; // L8 Coherence = 0.8 (in range [0.5, 1.0])
    stream.write_event(event5).unwrap();
    println!("  → Expected: No homeostasis penalty (perfect state)");
    sleep(Duration::from_millis(100)).await;

    // Event 6: Multiple issues (triggers multiple appraisers)
    println!("\nEvent 6: Multiple issues (high novelty + cognitive overload + motor activity)");
    let mut event6 = ExperienceEvent::default();
    event6.state[1] = 0.95; // L2 Novelty = 0.95 (very high!)
    event6.state[2] = 0.7; // L3 Velocity = 0.7 (high)
    event6.action[2] = 0.5; // L3 Acceleration = 0.5 (high)
    event6.state[4] = 0.9; // L5 Cognitive Load = 0.9 (overload!)
    event6.state[5] = 0.7; // L6 Certainty = 0.7 (normal)
    event6.state[6] = -0.3; // L7 Valence = -0.3 (negative)
    event6.state[7] = 0.8; // L8 Coherence = 0.8 (normal)
    stream.write_event(event6).unwrap();
    println!("  → Expected: Curiosity reward + Homeostasis penalty + Efficiency penalty");
    sleep(Duration::from_millis(100)).await;

    println!("\n[6] Waiting for appraisers to process all events...");
    sleep(Duration::from_millis(500)).await;

    // 6. Read back events and check rewards
    println!("\n[7] Reading back events and checking rewards:\n");
    for i in 1..=6 {
        if let Some(event) = stream.get_event(i) {
            println!("Event #{}: state=[{:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}, {:.2}]",
                i,
                event.l1_existence(),
                event.l2_novelty(),
                event.l3_velocity(),
                event.l4_attention(),
                event.l5_cognitive_load(),
                event.l6_certainty(),
                event.l7_valence(),
                event.l8_coherence(),
            );
            println!("  Rewards: H={:.4} C={:.4} E={:.4} G={:.4} | Total={:.4}",
                event.reward_homeostasis,
                event.reward_curiosity,
                event.reward_efficiency,
                event.reward_goal,
                event.total_reward(),
            );
            println!();
        }
    }

    // 7. Graceful shutdown
    println!("[8] Shutting down AppraiserSet...");
    drop(stream); // Close the broadcast channel
    appraiser_set.wait_all().await;
    println!("    ✓ All appraisers shut down gracefully\n");

    println!("=== Demo Complete ===");
}