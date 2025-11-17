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

//! Learning Loop Demo - Full Integration
//!
//! Demonstrates complete learning cycle:
//! 1. Events → ExperienceStream
//! 2. Appraisers → Rewards
//! 3. IntuitionEngine → Pattern Analysis → Proposals
//! 4. EvolutionManager → Validation → ADNA Updates
//! 5. Meta-learning feedback loop

use std::sync::Arc;
use tokio::sync::mpsc;
use neurograph_core::*;

#[tokio::main]
async fn main() {
    println!("=== Learning Loop Demo ===\n");

    // 1. Create core components
    println!("[1] Initializing core components...");

    let experience_stream = Arc::new(ExperienceStream::new(10_000, 100));
    let adna_reader = Arc::new(InMemoryADNAReader::with_defaults());
    let adna_state = Arc::new(ADNAState::new());
    let cdna = Arc::new(CDNA::default());

    println!("   ✓ ExperienceStream created (10k capacity)");
    println!("   ✓ ADNA reader with default params");
    println!("   ✓ ADNA state (in-memory)");
    println!("   ✓ CDNA with default rules\n");

    // 2. Start Appraisers
    println!("[2] Starting 4 appraisers...");

    let appraiser_set = AppraiserSet::start(
        Arc::clone(&adna_reader) as Arc<dyn ADNAReader>,
        Arc::clone(&experience_stream) as Arc<dyn ExperienceWriter>,
        experience_stream.subscribe(),
        experience_stream.subscribe(),
        experience_stream.subscribe(),
        experience_stream.subscribe(),
    );

    println!("   ✓ All 4 appraisers running in parallel\n");

    // 3. Setup IntuitionEngine → EvolutionManager channel
    println!("[3] Setting up learning loop...");

    let (proposal_tx, proposal_rx) = mpsc::channel(100);

    // Create IntuitionEngine
    let intuition_config = IntuitionConfig {
        analysis_interval_secs: 2, // Analyze every 2 seconds for demo
        batch_size: 50,
        sampling_strategy: SamplingStrategy::PrioritizedByReward { alpha: 1.0 },
        min_confidence: 0.6, // Lower threshold for demo
        max_proposals_per_cycle: 3,
        state_bins_per_dim: 4,
        min_samples: 5, // Lower for demo
        min_reward_delta: 0.3,
    };

    let intuition_engine = IntuitionEngine::new(
        intuition_config,
        Arc::clone(&experience_stream),
        Arc::clone(&adna_reader) as Arc<dyn ADNAReader>,
        proposal_tx,
    );

    // Create EvolutionManager
    let evolution_config = EvolutionConfig {
        max_proposals_per_sec: 5,
        min_confidence_threshold: 0.6,
        strict_validation: true,
    };

    let evolution_manager = EvolutionManager::new(
        evolution_config,
        Arc::clone(&adna_state),
        Arc::clone(&cdna),
        Arc::clone(&experience_stream),
        proposal_rx,
    );

    println!("   ✓ IntuitionEngine configured (2s interval)");
    println!("   ✓ EvolutionManager configured\n");

    // 4. Start background tasks
    println!("[4] Starting background learning tasks...");

    tokio::spawn(async move {
        intuition_engine.run().await;
    });

    tokio::spawn(async move {
        evolution_manager.run().await;
    });

    println!("   ✓ IntuitionEngine running");
    println!("   ✓ EvolutionManager running\n");

    // 5. Simulate experience events
    println!("[5] Simulating experience events...\n");

    simulate_learning_scenario(Arc::clone(&experience_stream)).await;

    // 6. Wait for analysis cycles
    println!("\n[6] Waiting for learning cycles...");
    tokio::time::sleep(tokio::time::Duration::from_secs(8)).await;

    // 7. Show results
    println!("\n[7] Learning Loop Results:");
    println!("   Total events: {}", experience_stream.total_written());
    println!("   ADNA policies learned: {}", adna_state.policy_count());

    if adna_state.policy_count() > 0 {
        println!("\n   ✓ Learning successful! ADNA has been updated with new policies.");
    }

    println!("\n=== Demo Complete ===");
    println!("The system has demonstrated:");
    println!("  • Event generation → ExperienceStream");
    println!("  • Reward appraisal (4 concurrent appraisers)");
    println!("  • Pattern analysis (IntuitionEngine)");
    println!("  • Proposal validation (EvolutionManager)");
    println!("  • Policy evolution (ADNA updates)");
    println!("\nFull learning loop is operational! 🚀\n");
}

/// Simulate learning scenario with clear action-reward patterns
async fn simulate_learning_scenario(stream: Arc<ExperienceStream>) {
    println!("   Generating 100 events with clear patterns:\n");

    println!("   Pattern 1: In state ~[0.5, 0.5, ...], action 100 → high reward");
    println!("   Pattern 2: In state ~[0.5, 0.5, ...], action 200 → low reward");
    println!("   Pattern 3: In state ~[-0.5, -0.5, ...], action 300 → high reward\n");

    for i in 0..100 {
        let mut event = ExperienceEvent::default();
        event.episode_id = 1;
        event.step_number = i;
        event.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;

        // Create 3 distinct state-action patterns
        if i < 40 {
            // Pattern 1: State ~[0.5, 0.5, ...] + action 100 → high reward
            event.state = [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5];
            event.event_type = 100;
            event.reward_homeostasis = 3.0;
            event.reward_curiosity = 2.0;
        } else if i < 70 {
            // Pattern 2: State ~[0.5, 0.5, ...] + action 200 → low reward
            event.state = [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5];
            event.event_type = 200;
            event.reward_homeostasis = -1.0;
            event.reward_efficiency = -0.5;
        } else {
            // Pattern 3: State ~[-0.5, -0.5, ...] + action 300 → high reward
            event.state = [-0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5, -0.5];
            event.event_type = 300;
            event.reward_goal = 4.0;
            event.reward_curiosity = 1.0;
        }

        stream.write_event(event).unwrap();

        // Small delay for realistic simulation
        if i % 20 == 0 {
            println!("   Generated {} events...", i + 1);
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }
    }

    println!("   ✓ 100 events generated");
}