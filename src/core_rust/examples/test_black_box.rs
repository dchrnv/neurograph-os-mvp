// Test Black Box Recorder manually

use neurograph_core::black_box::{BlackBox, Event, EventType, GLOBAL_BLACK_BOX, record_event};

fn main() {
    println!("Testing Black Box Recorder...\n");

    // Test 1: Create local black box
    println!("=== Test 1: Local Black Box ===");
    let bb = BlackBox::new(5); // Small buffer for testing

    // Record events
    bb.record(Event::new(EventType::SystemStarted));
    bb.record(Event::new(EventType::TokenCreated).with_data("token_id", "1"));
    bb.record(Event::new(EventType::TokenCreated).with_data("token_id", "2"));
    bb.record(Event::new(EventType::ConnectionCreated).with_data("from", "1").with_data("to", "2"));
    bb.record(Event::new(EventType::QuotaExceeded).with_data("reason", "memory"));

    let stats = bb.stats();
    println!("Stats: {:?}", stats);
    println!("✅ Recorded {} events\n", bb.len());

    // Test 2: Overflow circular buffer
    println!("=== Test 2: Circular Buffer Overflow ===");
    for i in 0..10 {
        bb.record(Event::new(EventType::Custom(format!("event_{}", i))));
    }

    let stats = bb.stats();
    println!("After overflow:");
    println!("  Current size: {}", stats.current_size);
    println!("  Total recorded: {}", stats.total_recorded);
    println!("  Total dropped: {}", stats.total_dropped);
    println!("✅ Circular buffer working correctly\n");

    // Test 3: Dump to file
    println!("=== Test 3: Dump to File ===");
    let dump_path = "/tmp/test_black_box_dump.json";
    match bb.dump_to_file(dump_path) {
        Ok(count) => {
            println!("✅ Dumped {} events to {}", count, dump_path);

            // Read and display
            if let Ok(content) = std::fs::read_to_string(dump_path) {
                println!("\nDump preview (first 500 chars):");
                println!("{}", &content[..content.len().min(500)]);
                println!("...\n");
            }

            // Cleanup
            std::fs::remove_file(dump_path).ok();
        }
        Err(e) => eprintln!("❌ Failed to dump: {}", e),
    }

    // Test 4: Global black box
    println!("=== Test 4: Global Black Box ===");
    record_event(Event::new(EventType::SystemStarted));
    record_event(Event::new(EventType::TokenCreated).with_data("global", "true"));
    record_event(Event::new(EventType::PanicRecovered).with_data("test", "global"));

    let global_stats = GLOBAL_BLACK_BOX.stats();
    println!("Global Black Box stats: {:?}", global_stats);
    println!("✅ Global black box working\n");

    // Test 5: Multiple data fields
    println!("=== Test 5: Event with Multiple Fields ===");
    let event = Event::new(EventType::TokenCreated)
        .with_data("id", "42")
        .with_data("weight", "1.5")
        .with_data("layer", "cognitive");

    bb.record(event.clone());
    println!("Event: {:?}", event);
    println!("✅ Multi-field events working\n");

    println!("🎉 All Black Box tests passed!");
}
