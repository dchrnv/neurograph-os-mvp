// NeuroGraph OS - REST API Handlers v0.39.0
//
// HTTP request handlers for REST API

use super::models::*;
use super::state::ApiState;
use axum::{
    extract::{Json, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
};
use crate::{InputSignal, SignalSource};
use crate::feedback::{DetailedFeedbackType, FeedbackSignal};
use std::time::SystemTime;
use std::collections::HashMap;

// ============================================================================
// Error Handling
// ============================================================================

/// API error type
pub enum ApiError {
    Unauthorized,
    BadRequest(String),
    Timeout,
    InternalError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, error_response) = match self {
            ApiError::Unauthorized => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse::new("unauthorized", "Invalid or missing API key"),
            ),
            ApiError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse::new("bad_request", msg),
            ),
            ApiError::Timeout => (
                StatusCode::REQUEST_TIMEOUT,
                ErrorResponse::new("timeout", "Request timed out"),
            ),
            ApiError::InternalError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse::new("internal_error", msg),
            ),
        };

        (status, Json(error_response)).into_response()
    }
}

/// Extract API key from headers
fn extract_api_key(headers: &HeaderMap) -> Option<String> {
    headers
        .get("X-API-Key")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string())
}

// ============================================================================
// Query Handler
// ============================================================================

/// POST /api/v1/query
///
/// Process a text query through the system
pub async fn handle_query(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Json(req): Json<QueryRequest>,
) -> Result<Json<QueryResponse>, ApiError> {
    // Validate API key
    let api_key = extract_api_key(&headers);
    if !state.validate_api_key(api_key.as_deref()) {
        return Err(ApiError::Unauthorized);
    }

    // Validate request
    if req.query.trim().is_empty() {
        return Err(ApiError::BadRequest("Query cannot be empty".to_string()));
    }

    let start = std::time::Instant::now();

    // Create input signal
    let signal = InputSignal::Text {
        content: req.query.clone(),
        source: SignalSource::RestApi,
        metadata: None,
    };

    // Inject into gateway
    let (receipt, receiver) = state
        .gateway
        .inject(signal)
        .await
        .map_err(|e| ApiError::InternalError(format!("Gateway error: {}", e)))?;

    // Wait for result with timeout
    let timeout_duration = std::time::Duration::from_millis(
        req.timeout_ms.unwrap_or(state.config.request_timeout_ms),
    );

    let result = tokio::time::timeout(timeout_duration, receiver)
        .await
        .map_err(|_| ApiError::Timeout)?
        .map_err(|_| ApiError::InternalError("Response channel closed".to_string()))?;

    let processing_time = start.elapsed().as_micros() as u64;

    // Extract data from ActionResult
    if !result.success {
        return Err(ApiError::InternalError(
            result.error.unwrap_or_else(|| "Unknown error".to_string())
        ));
    }

    // Parse output JSON to extract signal data
    let state: [f32; 8] = result.output
        .get("state")
        .and_then(|v| serde_json::from_value(v.clone()).ok())
        .unwrap_or([0.0; 8]);

    let signal_type = result.output
        .get("signal_type")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown")
        .to_string();

    let confidence = result.output
        .get("confidence")
        .and_then(|v| v.as_f64())
        .map(|c| c as f32);

    let response_text = result.output
        .get("response")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let matched_tokens = result.output
        .get("matched_tokens")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;

    let unknown_words = result.output
        .get("unknown_words")
        .and_then(|v| v.as_u64())
        .unwrap_or(0) as usize;

    let decision_source = result.output
        .get("decision_source")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    // Build response
    let response = QueryResponse {
        signal_id: receipt.signal_id,
        state,
        signal_type,
        response: response_text,
        metadata: QueryMetadata {
            processing_time_us: processing_time,
            matched_tokens,
            unknown_words,
            decision_source,
            confidence,
        },
    };

    Ok(Json(response))
}

// ============================================================================
// Feedback Handler
// ============================================================================

/// POST /api/v1/feedback
///
/// Submit feedback for a previous query
pub async fn handle_feedback(
    State(state): State<ApiState>,
    headers: HeaderMap,
    Json(req): Json<FeedbackRequest>,
) -> Result<Json<FeedbackResponse>, ApiError> {
    // Validate API key
    let api_key = extract_api_key(&headers);
    if !state.validate_api_key(api_key.as_deref()) {
        return Err(ApiError::Unauthorized);
    }

    // Convert API feedback type to internal type
    let feedback_type = match req.feedback {
        FeedbackType::Positive { strength } => DetailedFeedbackType::Positive { strength },
        FeedbackType::Negative { strength } => DetailedFeedbackType::Negative { strength },
        FeedbackType::Correction { correct_value } => {
            DetailedFeedbackType::Correction { correct_value }
        }
    };

    // Create feedback signal
    let feedback_signal = FeedbackSignal {
        reference_id: req.signal_id,
        feedback_type,
        timestamp: SystemTime::now(),
        explanation: req.explanation,
    };

    // Process feedback
    let result = state
        .feedback_processor
        .process(feedback_signal)
        .await
        .map_err(|e| ApiError::InternalError(format!("Feedback error: {}", e)))?;

    let response = FeedbackResponse {
        success: result.success,
        changes_made: result.changes_made,
        errors: result.errors,
    };

    Ok(Json(response))
}

// ============================================================================
// Status Handler
// ============================================================================

/// GET /api/v1/status
///
/// Get system status
pub async fn handle_status(
    State(state): State<ApiState>,
    headers: HeaderMap,
) -> Result<Json<StatusResponse>, ApiError> {
    // Validate API key
    let api_key = extract_api_key(&headers);
    if !state.validate_api_key(api_key.as_deref()) {
        return Err(ApiError::Unauthorized);
    }

    // Get gateway stats
    let gateway_stats = state.gateway.stats();

    let gateway_status = GatewayStatus {
        pending_requests: state.gateway.pending_count(),
        total_signals: gateway_stats.total_signals,
        unknown_words: gateway_stats.unknown_words,
        success_rate: gateway_stats.success_rate() as f32,
    };

    // Get curiosity status if available
    let curiosity_status = state.curiosity.as_ref().map(|c| {
        let stats = c.stats();
        CuriosityStatus {
            autonomous_enabled: stats.autonomous_enabled,
            total_cells: stats.uncertainty.total_cells,
            avg_confidence: stats.uncertainty.avg_confidence,
            avg_surprise: stats.surprise.avg_surprise,
            queue_size: stats.exploration.queue_size,
        }
    });

    let response = StatusResponse {
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_seconds: state.uptime_seconds(),
        gateway: gateway_status,
        curiosity: curiosity_status,
    };

    Ok(Json(response))
}

// ============================================================================
// Statistics Handler
// ============================================================================

/// GET /api/v1/stats
///
/// Get detailed statistics
pub async fn handle_stats(
    State(state): State<ApiState>,
    headers: HeaderMap,
) -> Result<Json<StatsResponse>, ApiError> {
    // Validate API key
    let api_key = extract_api_key(&headers);
    if !state.validate_api_key(api_key.as_deref()) {
        return Err(ApiError::Unauthorized);
    }

    // Get gateway stats
    let gateway_stats = state.gateway.stats();

    let gateway_stats_response = GatewayStats {
        total_signals: gateway_stats.total_signals,
        text_signals: gateway_stats.text_signals,
        tick_signals: gateway_stats.tick_signals,
        command_signals: gateway_stats.command_signals,
        feedback_signals: gateway_stats.feedback_signals,
        unknown_words: gateway_stats.unknown_words,
        queue_overflows: gateway_stats.queue_overflows,
        timeouts: gateway_stats.timeouts,
        errors: gateway_stats.errors,
        avg_processing_time_us: gateway_stats.avg_processing_time_us(),
        success_rate: gateway_stats.success_rate() as f32,
    };

    // Get curiosity stats if available
    let curiosity_stats_response = state.curiosity.as_ref().map(|c| {
        let stats = c.stats();
        CuriosityStats {
            uncertainty: UncertaintyStats {
                total_cells: stats.uncertainty.total_cells,
                total_visits: stats.uncertainty.total_visits,
                avg_confidence: stats.uncertainty.avg_confidence,
                avg_visits: stats.uncertainty.avg_visits,
            },
            surprise: SurpriseStats {
                current_surprise: stats.surprise.current_surprise,
                avg_surprise: stats.surprise.avg_surprise,
                max_recent_surprise: stats.surprise.max_recent_surprise,
                history_size: stats.surprise.history_size,
                total_events: stats.surprise.total_events,
            },
            novelty: NoveltyStats {
                unique_states: stats.novelty.unique_states,
                total_observations: stats.novelty.total_observations,
                total_unique_seen: stats.novelty.total_unique_seen,
            },
            exploration: ExplorationStats {
                queue_size: stats.exploration.queue_size,
                total_added: stats.exploration.total_added,
                total_explored: stats.exploration.total_explored,
            },
            autonomous_enabled: stats.autonomous_enabled,
        }
    });

    let response = StatsResponse {
        gateway: gateway_stats_response,
        curiosity: curiosity_stats_response,
    };

    Ok(Json(response))
}

// ============================================================================
// Health Check Handler
// ============================================================================

/// GET /api/v1/health
///
/// Health check endpoint
pub async fn handle_health(
    State(state): State<ApiState>,
) -> Result<Json<HealthResponse>, ApiError> {
    let mut checks = HashMap::new();

    // Check gateway
    checks.insert("gateway".to_string(), true);

    // Check curiosity if available
    if let Some(_curiosity) = &state.curiosity {
        checks.insert("curiosity".to_string(), true);
    }

    let all_healthy = checks.values().all(|&v| v);

    let response = HealthResponse {
        status: if all_healthy {
            "healthy".to_string()
        } else {
            "unhealthy".to_string()
        },
        checks,
    };

    Ok(Json(response))
}

// ============================================================================
// Metrics Handler (v0.42.0)
// ============================================================================

/// Prometheus metrics endpoint
///
/// Returns metrics in Prometheus exposition format for scraping.
/// No authentication required for metrics endpoint (standard practice).
///
/// # Example
///
/// ```bash
/// curl http://localhost:8080/metrics
/// ```
pub async fn handle_metrics() -> Result<impl IntoResponse, ApiError> {
    match crate::metrics::export_metrics() {
        Ok(metrics_text) => {
            // Return with proper content type for Prometheus
            Ok((
                StatusCode::OK,
                [(
                    axum::http::header::CONTENT_TYPE,
                    "text/plain; version=0.0.4; charset=utf-8",
                )],
                metrics_text,
            ))
        }
        Err(e) => Err(ApiError::InternalError(format!(
            "Failed to export metrics: {}",
            e
        ))),
    }
}
