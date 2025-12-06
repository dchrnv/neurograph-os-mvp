// Example: Distributed Tracing with OpenTelemetry and Jaeger
//
// This example demonstrates how to use OpenTelemetry distributed tracing
// with Jaeger backend for NeuroGraph OS.
//
// Usage:
//   1. Start Jaeger: docker run -d -p16686:16686 -p14268:14268 jaegertracing/all-in-one:1.51
//   2. Run: cargo run --example test_tracing
//   3. Open Jaeger UI: http://localhost:16686

use neurograph_core::tracing_otel;
use tracing::{info, info_span, warn};
use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== OpenTelemetry Distributed Tracing Example ===\n");

    // Initialize tracing with Jaeger
    println!("Initializing Jaeger tracing...");
    match tracing_otel::init_tracing_with_jaeger(
        "neurograph-example",
        "http://localhost:14268/api/traces",
        "info"
    ) {
        Ok(_) => println!("✅ Jaeger tracing initialized\n"),
        Err(e) => {
            eprintln!("❌ Failed to initialize Jaeger: {}", e);
            eprintln!("Make sure Jaeger is running: docker run -d -p16686:16686 -p14268:14268 jaegertracing/all-in-one:1.51");
            return Ok(());
        }
    }

    // Simulate some operations with traces
    simulate_api_request().await;
    simulate_token_creation().await;
    simulate_error_handling().await;

    // Give time for traces to be exported
    println!("\n⏳ Waiting for traces to be exported to Jaeger...");
    thread::sleep(Duration::from_secs(2));

    // Shutdown tracer
    println!("🛑 Shutting down tracer...");
    tracing_otel::shutdown_tracer();

    println!("\n✅ Example complete!");
    println!("📊 View traces at: http://localhost:16686");
    println!("   - Service: neurograph-example");
    println!("   - Look for operations: api_request, token_creation, error_handling\n");

    Ok(())
}

/// Simulate an API request with nested spans
async fn simulate_api_request() {
    let span = info_span!(
        "api_request",
        method = "POST",
        endpoint = "/api/v1/query",
        user_id = "user123"
    );
    let _enter = span.enter();

    info!("Processing API request");

    // Simulate authentication
    {
        let auth_span = info_span!("authenticate", user = "user123");
        let _auth_enter = auth_span.enter();
        info!("Authenticating user");
        thread::sleep(Duration::from_millis(50));
    }

    // Simulate query processing
    {
        let query_span = info_span!("process_query", query = "hello world");
        let _query_enter = query_span.enter();
        info!("Processing query");
        thread::sleep(Duration::from_millis(100));

        // Nested: token lookup
        {
            let lookup_span = info_span!("token_lookup", count = 5);
            let _lookup_enter = lookup_span.enter();
            info!("Looking up tokens");
            thread::sleep(Duration::from_millis(30));
        }
    }

    // Simulate response generation
    {
        let response_span = info_span!("generate_response");
        let _response_enter = response_span.enter();
        info!("Generating response");
        thread::sleep(Duration::from_millis(20));
    }

    info!("API request completed");
}

/// Simulate token creation with attributes
async fn simulate_token_creation() {
    let span = info_span!(
        "token_creation",
        batch_size = 1000,
        operation = "batch_create"
    );
    let _enter = span.enter();

    info!("Starting batch token creation");

    for i in 0..5 {
        let create_span = info_span!("create_token", token_id = i, weight = 0.5);
        let _create_enter = create_span.enter();
        info!(token_id = i, "Creating token");
        thread::sleep(Duration::from_millis(10));
    }

    info!(tokens_created = 5, "Batch token creation completed");
}

/// Simulate error handling with warnings
async fn simulate_error_handling() {
    let span = info_span!(
        "error_handling",
        operation = "validate_connection"
    );
    let _enter = span.enter();

    info!("Validating connection");

    // Simulate validation failure
    {
        let validate_span = info_span!("validate", from = "token1", to = "token2");
        let _validate_enter = validate_span.enter();
        warn!(
            reason = "weight_too_low",
            weight = 0.01,
            min_weight = 0.1,
            "Validation failed"
        );
    }

    info!("Error handling completed");
}
