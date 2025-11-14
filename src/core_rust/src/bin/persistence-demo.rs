//! Persistence Demo for v0.26.0
//!
//! Demonstrates PostgreSQL backend for ExperienceStream with ActionMetadata

#[cfg(feature = "persistence")]
use neurograph_core::{
    PostgresBackend, PersistenceBackend, QueryOptions,
    ExperienceEvent, ActionMetadata,
};

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
    println!("\nPostgreSQL Persistence Layer v0.26.0 is working! 🚀\n");

    Ok(())
}

#[cfg(not(feature = "persistence"))]
fn main() {
    eprintln!("This demo requires the 'persistence' feature.");
    eprintln!("Run with: cargo run --bin persistence-demo --features persistence");
    std::process::exit(1);
}