// Test metrics export manually

use neurograph_core::metrics;

fn main() {
    println!("Testing Prometheus metrics export...\n");

    // Initialize metrics
    metrics::init();

    // Simulate some operations
    println!("Simulating operations...");
    metrics::TOKENS_CREATED.inc_by(100);
    metrics::CONNECTIONS_CREATED.inc_by(50);
    metrics::TOKENS_VALIDATED.inc_by(95);
    metrics::TOKENS_REJECTED.inc_by(5);
    metrics::MEMORY_USED_BYTES.set(1024 * 1024 * 500); // 500MB
    metrics::MEMORY_USAGE_PERCENT.set(50.0);
    metrics::WAL_ENTRIES_WRITTEN.inc_by(10);
    metrics::PANICS_RECOVERED.inc();

    // Export metrics
    println!("\nExporting metrics...\n");
    match metrics::export_metrics() {
        Ok(metrics_text) => {
            println!("=== PROMETHEUS METRICS OUTPUT ===\n");
            println!("{}", metrics_text);
            println!("=== END ===\n");
            println!("✅ Metrics export successful!");
        }
        Err(e) => {
            eprintln!("❌ Failed to export metrics: {}", e);
            std::process::exit(1);
        }
    }
}
