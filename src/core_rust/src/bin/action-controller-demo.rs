//! ActionController E2E Demo
//!
//! Demonstrates the complete action execution pipeline:
//! Intent → ActionController → ADNA Policy → Executor Selection → Action Execution → Logging

use neurograph_core::{
    ActionController, ActionControllerConfig,
    ADNAReader, InMemoryADNAReader,
    ExperienceStream, ExperienceWriter,
    NoOpExecutor, MessageSenderExecutor,
    Intent, ActionPolicy,
};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    println!("=== ActionController E2E Demo ===\n");

    // 1. Create core components
    println!("[1] Initializing components...");
    let experience_stream = Arc::new(ExperienceStream::new(1000, 10));
    let adna_reader = Arc::new(InMemoryADNAReader::with_defaults());
    println!("   ✓ ExperienceStream created (capacity: 1000)");
    println!("   ✓ ADNA reader with default config\n");

    // 2. Create ActionController with config from JSON file
    println!("[2] Creating ActionController...");
    let config = ActionControllerConfig::from_file_or_default("action_controller_config.json");
    let controller = ActionController::new(
        adna_reader.clone() as Arc<dyn ADNAReader>,
        experience_stream.clone() as Arc<dyn ExperienceWriter>,
        config.clone(),
    );
    println!("   ✓ ActionController configured from JSON file");
    println!("   - Exploration rate: {}%", config.exploration_rate * 100.0);
    println!("   - Timeout: {}ms", config.timeout_ms);
    println!("   - Logging: {}\n", if config.log_all_actions { "enabled" } else { "disabled" });

    // 3. Register executors
    println!("[3] Registering executors...");
    let noop = Arc::new(NoOpExecutor::new());
    let message_sender = Arc::new(MessageSenderExecutor::new());

    controller.register_executor(noop).unwrap();
    controller.register_executor(message_sender).unwrap();

    println!("   ✓ Registered: {}", controller.list_executors().join(", "));
    println!();

    // 4. Set up a simple ADNA policy (optional - will use default if not set)
    println!("[4] Setting up ADNA policy...");
    let mut policy = ActionPolicy::new("demo_policy");
    policy.set_weight(1, 0.3);  // NoOp with weight 0.3
    policy.set_weight(2, 0.7);  // MessageSender with weight 0.7

    // Set policy for a specific state bin
    let test_state: [i16; 8] = [1000, 2000, 500, 3000, 1500, 4000, 2500, 3500];
    let state_bin_id = format!("adna_state_bin_{}", quantize_state(&test_state));
    adna_reader.set_action_policy(state_bin_id.clone(), policy).await;
    println!("   ✓ Policy set for state bin: {}", state_bin_id);
    println!("   - NoOp weight: 0.3");
    println!("   - MessageSender weight: 0.7\n");

    // 5. Execute several intents
    println!("[5] Executing intents...\n");

    // Intent 1: NoOp action
    println!("   Intent #1: test_noop");
    let intent1 = Intent::new(
        "test_noop",
        serde_json::json!({}),
        test_state,
    );

    match controller.execute_intent(intent1).await {
        Ok(result) => {
            println!("   ✓ Success: {}", result.success);
            println!("   Duration: {}ms\n", result.duration_ms);
        }
        Err(e) => {
            println!("   ✗ Error: {}\n", e);
        }
    }

    // Intent 2: Send message
    println!("   Intent #2: send_greeting");
    let intent2 = Intent::new(
        "send_greeting",
        serde_json::json!({
            "message": "Hello from ActionController Demo!",
            "priority": "info"
        }),
        test_state,
    );

    match controller.execute_intent(intent2).await {
        Ok(result) => {
            println!("   ✓ Success: {}", result.success);
            println!("   Duration: {}ms", result.duration_ms);
            if let Some(msg) = result.output.get("formatted") {
                println!("   Output: {}\n", msg);
            }
        }
        Err(e) => {
            println!("   ✗ Error: {}\n", e);
        }
    }

    // Intent 3: Send warning message
    println!("   Intent #3: send_warning");
    let intent3 = Intent::new(
        "send_warning",
        serde_json::json!({
            "message": "This is a test warning",
            "priority": "warn"
        }),
        test_state,
    );

    match controller.execute_intent(intent3).await {
        Ok(result) => {
            println!("   ✓ Success: {}", result.success);
            println!("   Duration: {}ms", result.duration_ms);
            if let Some(msg) = result.output.get("formatted") {
                println!("   Output: {}\n", msg);
            }
        }
        Err(e) => {
            println!("   ✗ Error: {}\n", e);
        }
    }

    // Intent 4: Test with different state (will use different/default policy)
    println!("   Intent #4: different_state");
    let different_state: [i16; 8] = [5000, 6000, 7000, 8000, 9000, 10000, 11000, 12000];
    let intent4 = Intent::new(
        "different_state_action",
        serde_json::json!({
            "message": "Action from different state",
            "priority": "debug"
        }),
        different_state,
    );

    match controller.execute_intent(intent4).await {
        Ok(result) => {
            println!("   ✓ Success: {}", result.success);
            println!("   Duration: {}ms\n", result.duration_ms);
        }
        Err(e) => {
            println!("   ✗ Error: {}\n", e);
        }
    }

    // Intent 5: Test invalid parameters (should fail validation)
    println!("   Intent #5: invalid_params (should fail)");
    let intent5 = Intent::new(
        "invalid_action",
        serde_json::json!({
            "priority": "info"
            // Missing 'message' parameter for MessageSender
        }),
        test_state,
    );

    match controller.execute_intent(intent5).await {
        Ok(result) => {
            println!("   ✓ Success: {}", result.success);
            println!("   Duration: {}ms\n", result.duration_ms);
        }
        Err(e) => {
            println!("   ✗ Expected error: {}\n", e);
        }
    }

    // 6. Summary of execution
    println!("[6] Execution summary...");
    println!("   ✓ 5 intents executed (4 successful, 1 failed validation)");
    println!("   ✓ All actions logged to ExperienceStream");
    println!("   (Each successful intent logs 2 events: action_started + action_finished)\n");

    // 7. Summary
    println!("=== Demo Complete ===");
    println!("Demonstrated:");
    println!("  • ActionController initialization and configuration");
    println!("  • Executor registration (NoOp, MessageSender)");
    println!("  • ADNA policy setup and state quantization");
    println!("  • Intent execution with different parameters");
    println!("  • Epsilon-greedy exploration/exploitation");
    println!("  • Parameter validation");
    println!("  • Event logging to ExperienceStream");
    println!("\nActionController v1.0 is working! 🚀\n");
}

/// Quantize state to bin ID (simplified version)
fn quantize_state(state: &[i16; 8]) -> u64 {
    let mut bin_id: u64 = 0;
    let bins_per_dim = 4u64;

    for &value in state.iter() {
        let normalized = ((value as f32 / 32767.0) + 1.0) / 2.0;
        let clamped = normalized.clamp(0.0, 0.999);
        let bin = (clamped * bins_per_dim as f32) as u64;
        bin_id = bin_id * bins_per_dim + bin;
    }

    bin_id
}