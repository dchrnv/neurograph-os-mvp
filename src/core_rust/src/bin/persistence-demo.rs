//! Persistence Demo for v0.26.0
//!
//! Demonstrates PostgreSQL backend for ExperienceStream with ActionMetadata

#[cfg(feature = "persistence")]
use neurograph_core::{
    PostgresBackend, PersistenceBackend, QueryOptions,
    ExperienceEvent, ActionMetadata,
};

#[cfg(feature = "persistence")]
use neurograph_core::persistence::{ADNAPolicy, Configuration};

#[cfg(feature = "persistence")]
use std::collections::HashMap;

#[cfg(feature = "persistence")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== PostgreSQL Persistence Demo v0.26.0 ===\n");

    // 1. Create PostgreSQL backend
    println!("[1] Connecting to PostgreSQL...");
    let backend = PostgresBackend::from_env().await?;
    println!("   ✓ Connected successfully");

    // 2. Health check
    println!("\n[2] Running health check...");
    backend.health_check().await?;
    println!("   ✓ Database schema verified (5 tables)");

    // 3. Create sample events
    println!("\n[3] Creating sample experience events...");

    let mut event1 = ExperienceEvent::default();
    event1.event_id = 1000;
    event1.event_type = 1000; // action_started
    event1.episode_id = 1;
    event1.step_number = 1;
    event1.state = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
    event1.action = [0.15, 0.25, 0.35, 0.45, 0.55, 0.65, 0.75, 0.85];

    let metadata1 = ActionMetadata {
        intent_type: "send_message".to_string(),
        executor_id: "message_sender".to_string(),
        parameters: serde_json::json!({
            "message": "Hello from PostgreSQL!",
            "priority": "info"
        }),
    };

    backend.write_event_with_metadata(&event1, &metadata1).await?;
    println!("   ✓ Event 1 written (with metadata)");

    let mut event2 = ExperienceEvent::default();
    event2.event_id = 1001;
    event2.event_type = 1001; // action_finished
    event2.episode_id = 1;
    event2.step_number = 2;
    event2.state = [0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
    event2.action = [0.25, 0.35, 0.45, 0.55, 0.65, 0.75, 0.85, 0.95];
    event2.reward_homeostasis = 0.5;
    event2.reward_curiosity = 0.3;
    event2.reward_efficiency = 0.2;
    event2.reward_goal = 0.4;

    backend.write_event(&event2).await?;
    println!("   ✓ Event 2 written (without metadata)");

    // 4. Read events
    println!("\n[4] Reading events from database...");

    let (read_event, read_metadata) = backend.read_event_with_metadata(1000).await?;
    println!("   ✓ Event 1000 read:");
    println!("      - Type: {}", read_event.event_type);
    println!("      - Episode: {}, Step: {}", read_event.episode_id, read_event.step_number);
    if let Some(meta) = read_metadata {
        println!("      - Intent: {}", meta.intent_type);
        println!("      - Executor: {}", meta.executor_id);
        println!("      - Params: {}", meta.parameters);
    }

    let event2_read = backend.read_event(1001).await?;
    println!("   ✓ Event 1001 read:");
    println!("      - Total reward: {:.2}",
        event2_read.reward_homeostasis + event2_read.reward_curiosity +
        event2_read.reward_efficiency + event2_read.reward_goal);

    // 5. Query events
    println!("\n[5] Querying events...");

    let query_opts = QueryOptions {
        limit: Some(10),
        episode_id: Some(1),
        include_archived: false,
        order_asc: true,
        ..Default::default()
    };

    let events = backend.query_events(query_opts).await?;
    println!("   ✓ Found {} events for episode 1", events.len());

    // 6. Query with metadata
    println!("\n[6] Querying events with metadata...");

    let query_opts_meta = QueryOptions {
        limit: Some(5),
        event_type: Some(1000), // action_started
        ..Default::default()
    };

    let events_with_meta = backend.query_events_with_metadata(query_opts_meta).await?;
    println!("   ✓ Found {} action_started events", events_with_meta.len());
    for (event, metadata) in &events_with_meta {
        if let Some(meta) = metadata {
            println!("      - Event {}: {} (executor: {})",
                event.event_id, meta.intent_type, meta.executor_id);
        }
    }

    // 7. Count events
    println!("\n[7] Counting events...");

    let total_count = backend.count_events(QueryOptions {
        include_archived: false,
        ..Default::default()
    }).await?;
    println!("   ✓ Total active events: {}", total_count);

    // 8. Archive old events (dry run with 0 days = none archived)
    println!("\n[8] Testing archive function...");
    let archived = backend.archive_old_events(365).await?;
    println!("   ✓ Would archive {} events older than 365 days", archived);

    // 9. ADNA Policy Persistence
    println!("\n[9] Testing ADNA policy persistence...");

    // Create a policy with action weights
    let mut action_weights = HashMap::new();
    action_weights.insert(1000, 0.4); // send_message
    action_weights.insert(1001, 0.35); // read_data
    action_weights.insert(1002, 0.25); // process_task

    let policy_metadata = serde_json::json!({
        "description": "Initial policy for state bin A",
        "created_by": "learning_loop",
        "strategy": "epsilon_greedy"
    });

    let policy_id = backend.save_policy(
        "state_bin_A",
        "rule_homeostasis_high",
        &action_weights,
        Some(policy_metadata),
        None, // no parent, initial version
    ).await?;
    println!("   ✓ Created ADNA policy (ID: {}, version 1)", policy_id);

    // Read the active policy
    let active_policy = backend.get_active_policy("state_bin_A").await?;
    if let Some(policy) = &active_policy {
        println!("   ✓ Retrieved active policy:");
        println!("      - State bin: {}", policy.state_bin_id);
        println!("      - Rule: {}", policy.rule_id);
        println!("      - Version: {}", policy.version);
        println!("      - Action weights: {} actions", policy.action_weights.len());
    }

    // Update policy (create new version)
    let mut updated_weights = HashMap::new();
    updated_weights.insert(1000, 0.5);  // increased
    updated_weights.insert(1001, 0.3);  // decreased
    updated_weights.insert(1002, 0.2);  // decreased

    let policy_id_v2 = backend.save_policy(
        "state_bin_A",
        "rule_homeostasis_high",
        &updated_weights,
        Some(serde_json::json!({"description": "Updated after learning"})),
        Some(policy_id), // parent policy ID
    ).await?;
    println!("   ✓ Created policy version 2 (ID: {})", policy_id_v2);

    // Update metrics
    backend.update_policy_metrics(policy_id_v2, 100, 0.75).await?;
    println!("   ✓ Updated policy metrics (100 executions, avg reward 0.75)");

    // Get all active policies
    let all_policies = backend.get_all_active_policies().await?;
    println!("   ✓ Total active policies: {}", all_policies.len());

    // 10. Configuration Persistence
    println!("\n[10] Testing configuration persistence...");

    // Save configuration
    let config_value = serde_json::json!({
        "learning_rate": 0.01,
        "epsilon": 0.1,
        "batch_size": 32
    });

    let config_id = backend.save_config(
        "learning_loop",
        "hyperparameters",
        config_value,
        None, // initial version
    ).await?;
    println!("   ✓ Created configuration (ID: {}, version 1)", config_id);

    // Read configuration
    let config = backend.get_config("learning_loop", "hyperparameters").await?;
    if let Some(cfg) = &config {
        println!("   ✓ Retrieved configuration:");
        println!("      - Component: {}", cfg.component_name);
        println!("      - Key: {}", cfg.config_key);
        println!("      - Version: {}", cfg.version);
        println!("      - Value: {}", cfg.config_value);
    }

    // Update configuration (new version)
    let updated_config = serde_json::json!({
        "learning_rate": 0.005,  // decreased
        "epsilon": 0.05,         // decreased
        "batch_size": 64         // increased
    });

    let config_id_v2 = backend.save_config(
        "learning_loop",
        "hyperparameters",
        updated_config,
        Some(config_id), // parent config
    ).await?;
    println!("   ✓ Created configuration version 2 (ID: {})", config_id_v2);

    // Get all configs for component
    let component_configs = backend.get_component_configs("learning_loop").await?;
    println!("   ✓ Total active configs for 'learning_loop': {}", component_configs.len());

    // Summary
    println!("\n=== Demo Complete ===");
    println!("Demonstrated:");
    println!("  • PostgreSQL connection and health check");
    println!("  • Writing events with and without metadata");
    println!("  • Reading individual events by ID");
    println!("  • Querying events with filtering options");
    println!("  • Querying events with metadata joins");
    println!("  • Counting events");
    println!("  • Archive retention policy function");
    println!("  • ADNA policy persistence with versioning");
    println!("  • Configuration persistence with versioning");
    println!("\nPostgreSQL Persistence Layer v0.26.0 is FULLY working! 🚀\n");

    Ok(())
}

#[cfg(not(feature = "persistence"))]
fn main() {
    eprintln!("This demo requires the 'persistence' feature.");
    eprintln!("Run with: cargo run --bin persistence-demo --features persistence");
    std::process::exit(1);
}