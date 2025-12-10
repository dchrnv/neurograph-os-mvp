// NeuroGraph OS - REST API Server v0.44.0
//
// HTTP API server with distributed tracing support

use neurograph_core::{
    api::{create_router, ApiConfig, ApiState},
    bootstrap::{BootstrapConfig, BootstrapLibrary},
    curiosity::{CuriosityConfig, CuriosityDrive},
    experience_stream::ExperienceStream,
    feedback::FeedbackProcessor,
    Gateway,
    GatewayConfig,
    intuition_engine::IntuitionEngine,
    adna::{AppraiserConfig, InMemoryADNAReader},
    install_panic_hook,
    ProcessedSignal,
    logging_utils,
    black_box,
    tracing_otel,  // NEW: v0.44.0 - OpenTelemetry tracing
};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tracing::{info, error};

/// Print welcome banner
fn print_banner(config: &ApiConfig, jaeger_enabled: bool) {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║         NeuroGraph OS v0.44.0 - REST API Server          ║");
    println!("║       Cognitive Architecture over HTTP + WebSocket        ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
    println!("🚀 Server starting on http://{}", config.bind_address());
    println!("📚 API Documentation: http://{}/api/v1", config.bind_address());
    println!("💚 Health Check: http://{}/health", config.bind_address());
    println!("📊 Status: http://{}/api/v1/status", config.bind_address());
    println!("📈 Stats: http://{}/api/v1/stats", config.bind_address());
    println!("📉 Metrics: http://{}/metrics (Prometheus)", config.bind_address());

    if jaeger_enabled {
        println!("🔍 Tracing: Jaeger enabled (distributed tracing)", );
    }

    if config.api_key.is_some() {
        println!("\n🔐 API Key authentication enabled");
    }

    if config.enable_cors {
        println!("🌐 CORS enabled");
    }

    println!("\n⚡ Press Ctrl+C to stop\n");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Install panic hook for production (v0.41.0)
    install_panic_hook();

    // Check if Jaeger tracing is enabled (v0.44.0)
    let jaeger_endpoint = std::env::var("JAEGER_ENDPOINT")
        .unwrap_or_else(|_| "http://jaeger:14268/api/traces".to_string());
    let enable_tracing = std::env::var("ENABLE_TRACING")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);

    // Initialize tracing with or without Jaeger (v0.44.0)
    if enable_tracing {
        // With distributed tracing
        match tracing_otel::init_tracing_with_jaeger(
            "neurograph-api",
            &jaeger_endpoint,
            "info"
        ) {
            Ok(_) => info!("OpenTelemetry tracing initialized with Jaeger"),
            Err(e) => {
                error!("Failed to initialize Jaeger tracing: {}. Falling back to standard logging.", e);
                logging_utils::init_logging("info");
            }
        }
    } else {
        // Standard logging only (v0.42.0)
        logging_utils::init_logging("info");
    }

    // Record system start in Black Box (v0.42.0)
    black_box::record_event(
        black_box::Event::new(black_box::EventType::SystemStarted)
            .with_data("component", "api_server")
            .with_data("version", "v0.44.0")
    );

    info!("NeuroGraph OS v0.44.0 API Server starting...");

    // Load configuration
    let api_config = ApiConfig::from_env();

    print_banner(&api_config, enable_tracing);

    // Initialize Bootstrap Library
    let bootstrap_config = BootstrapConfig::default();
    let bootstrap = Arc::new(RwLock::new(BootstrapLibrary::new(bootstrap_config)));

    // Initialize ExperienceStream
    let experience_stream = Arc::new(RwLock::new(ExperienceStream::new(100_000, 1000)));

    // Initialize IntuitionEngine with builder (v0.39.2)
    let intuition_engine = Arc::new(RwLock::new(
        IntuitionEngine::builder()
            .with_capacity(100_000)
            .with_channel_size(1000)
            .build()
            .expect("Failed to build IntuitionEngine")
    ));

    // Create signal queue
    let (signal_tx, mut signal_rx) = mpsc::channel::<ProcessedSignal>(1000);

    // Initialize Gateway
    let gateway_config = GatewayConfig::default();
    let gateway = Arc::new(Gateway::new(
        signal_tx,
        bootstrap.clone(),
        gateway_config,
    ));

    // Create FeedbackProcessor
    let feedback_processor = Arc::new(FeedbackProcessor::new(
        bootstrap.clone(),
        experience_stream,
        intuition_engine,
    ));

    // Initialize Curiosity Drive
    let curiosity_config = CuriosityConfig::default();
    let curiosity = Arc::new(CuriosityDrive::new(curiosity_config));

    // Create API state
    let state = ApiState::with_curiosity(
        gateway.clone(),
        feedback_processor,
        curiosity,
        api_config.clone(),
    );

    // Create router
    let app = create_router(state);

    // Spawn background task to handle processed signals
    tokio::spawn(async move {
        while let Some(_signal) = signal_rx.recv().await {
            // Process signal (e.g., trigger actions, update state)
            // For now, we just consume them
        }
    });

    // Bind and serve
    let listener = tokio::net::TcpListener::bind(&api_config.bind_address())
        .await?;

    tracing::info!("REST API server listening on {}", api_config.bind_address());

    axum::serve(listener, app)
        .await?;

    Ok(())
}
