// NeuroGraph OS - REST API Router v0.44.0
//
// HTTP routes and middleware configuration with distributed tracing

use super::{handlers, state::ApiState};
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

/// Create API router
pub fn create_router(state: ApiState) -> Router {
    // API v1 routes
    let api_v1 = Router::new()
        // Query endpoint
        .route("/query", post(handlers::handle_query))
        // Feedback endpoint
        .route("/feedback", post(handlers::handle_feedback))
        // Status endpoint
        .route("/status", get(handlers::handle_status))
        // Statistics endpoint
        .route("/stats", get(handlers::handle_stats))
        // Health check
        .route("/health", get(handlers::handle_health));

    // Root router
    let app = Router::new()
        .nest("/api/v1", api_v1)
        .route("/health", get(handlers::handle_health)) // Also at root
        .route("/metrics", get(handlers::handle_metrics)) // Prometheus metrics (v0.42.0)
        .with_state(state.clone());

    // Add CORS if enabled
    let app = if state.config.enable_cors {
        app.layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
    } else {
        app
    };

    // Add tracing middleware (v0.44.0)
    // This integrates with OpenTelemetry when enabled
    app.layer(
        TraceLayer::new_for_http()
            .make_span_with(|request: &axum::http::Request<_>| {
                tracing::info_span!(
                    "http_request",
                    method = %request.method(),
                    uri = %request.uri(),
                    version = ?request.version(),
                )
            })
            .on_response(|response: &axum::http::Response<_>, latency: std::time::Duration, _span: &tracing::Span| {
                tracing::info!(
                    status = %response.status(),
                    latency_ms = %latency.as_millis(),
                    "request completed"
                );
            })
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gateway::Gateway;
    use crate::feedback::FeedbackProcessor;
    use crate::bootstrap::BootstrapLibrary;
    use crate::experience_stream::ExperienceStream;
    use crate::intuition_engine::IntuitionEngine;
    use crate::adna::InMemoryADNAReader;
    use tokio::sync::mpsc;
    use std::sync::Arc;
    use parking_lot::RwLock;

    #[tokio::test]
    async fn test_router_creation() {
        // Create minimal components
        let (signal_tx, _signal_rx) = mpsc::channel(100);
        let bootstrap = Arc::new(RwLock::new(BootstrapLibrary::new(Default::default())));
        let gateway = Arc::new(Gateway::new(signal_tx, bootstrap.clone(), Default::default()));

        let experience_stream = Arc::new(RwLock::new(ExperienceStream::new(1000, 10)));
        let adna = Arc::new(InMemoryADNAReader::new(Default::default()));
        let (proposal_tx, _proposal_rx) = mpsc::channel(100);
        let intuition = Arc::new(RwLock::new(IntuitionEngine::new(
            Default::default(),
            Arc::new(ExperienceStream::new(1000, 10)),
            adna,
            proposal_tx,
        )));

        let feedback = Arc::new(FeedbackProcessor::new(
            bootstrap,
            experience_stream,
            intuition,
        ));

        let state = ApiState::new(gateway, feedback, Default::default());
        let _router = create_router(state);
        // Just checking it compiles and creates successfully
    }
}
