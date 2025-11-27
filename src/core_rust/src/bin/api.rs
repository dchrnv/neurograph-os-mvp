// NeuroGraph OS - REST API Server v0.39.0
//
// HTTP API server for external access

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
    ProcessedSignal,
};
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;
use tracing_subscriber;

/// Print welcome banner
fn print_banner(config: &ApiConfig) {
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║         NeuroGraph OS v0.39.0 - REST API Server          ║");
    println!("║       Cognitive Architecture over HTTP + WebSocket        ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
    println!("🚀 Server starting on http://{}", config.bind_address());
    println!("📚 API Documentation: http://{}/api/v1", config.bind_address());
    println!("💚 Health Check: http://{}/health", config.bind_address());
    println!("📊 Status: http://{}/api/v1/status", config.bind_address());
    println!("📈 Stats: http://{}/api/v1/stats", config.bind_address());

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
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .init();

    // Load configuration
    let api_config = ApiConfig::from_env();

    print_banner(&api_config);

    // Initialize Bootstrap Library
    let bootstrap_config = BootstrapConfig::default();
    let bootstrap = Arc::new(RwLock::new(BootstrapLibrary::new(bootstrap_config)));

    // Initialize ExperienceStream
    let experience_stream_raw = Arc::new(ExperienceStream::new(100_000, 1000));
    let experience_stream = Arc::new(RwLock::new(ExperienceStream::new(100_000, 1000)));

    // Initialize ADNA
    let adna_config = AppraiserConfig::default();
    let adna = Arc::new(InMemoryADNAReader::new(adna_config));

    // Create proposal channel for IntuitionEngine
    let (proposal_tx, _proposal_rx) = mpsc::channel(100);

    // Initialize IntuitionEngine
    let intuition_engine = Arc::new(RwLock::new(IntuitionEngine::new(
        Default::default(),
        experience_stream_raw,
        adna,
        proposal_tx,
    )));

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
