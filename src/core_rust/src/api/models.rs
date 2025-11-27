// NeuroGraph OS - REST API Models v0.39.0
//
// Request and response types for HTTP API

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Query API Models
// ============================================================================

/// Request to query the system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryRequest {
    /// Input text query
    pub query: String,

    /// Optional context metadata
    #[serde(default)]
    pub context: HashMap<String, String>,

    /// Optional timeout in milliseconds
    #[serde(default)]
    pub timeout_ms: Option<u64>,
}

/// Response from query
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResponse {
    /// Unique signal ID for this query
    pub signal_id: u64,

    /// Processed state vector [f32; 8]
    pub state: [f32; 8],

    /// Signal type classification
    pub signal_type: String,

    /// Response text (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,

    /// Processing metadata
    pub metadata: QueryMetadata,
}

/// Metadata about query processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryMetadata {
    /// Processing time in microseconds
    pub processing_time_us: u64,

    /// Number of matched tokens
    pub matched_tokens: usize,

    /// Number of unknown words
    pub unknown_words: usize,

    /// Decision source (Reflex/Reasoning/Curiosity/Failsafe)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decision_source: Option<String>,

    /// Confidence score (0.0-1.0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<f32>,
}

// ============================================================================
// Feedback API Models
// ============================================================================

/// Feedback types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum FeedbackType {
    /// Positive feedback
    Positive {
        /// Strength 0.0-1.0
        #[serde(default = "default_strength")]
        strength: f32,
    },

    /// Negative feedback
    Negative {
        /// Strength 0.0-1.0
        #[serde(default = "default_strength")]
        strength: f32,
    },

    /// Correction with correct value
    Correction {
        /// Correct response
        correct_value: String,
    },
}

fn default_strength() -> f32 {
    1.0
}

/// Request to submit feedback
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackRequest {
    /// Signal ID to provide feedback for
    pub signal_id: u64,

    /// Type of feedback
    pub feedback: FeedbackType,

    /// Optional explanation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

/// Response from feedback submission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackResponse {
    /// Whether feedback was successfully applied
    pub success: bool,

    /// Changes made
    pub changes_made: Vec<String>,

    /// Errors encountered (if any)
    #[serde(default)]
    pub errors: Vec<String>,
}

// ============================================================================
// Status API Models
// ============================================================================

/// System status response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    /// System version
    pub version: String,

    /// Uptime in seconds
    pub uptime_seconds: u64,

    /// Gateway status
    pub gateway: GatewayStatus,

    /// Curiosity drive status (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curiosity: Option<CuriosityStatus>,
}

/// Gateway status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayStatus {
    /// Number of pending requests
    pub pending_requests: usize,

    /// Total signals processed
    pub total_signals: u64,

    /// Unknown words encountered
    pub unknown_words: u64,

    /// Success rate (0.0-1.0)
    pub success_rate: f32,
}

/// Curiosity drive status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuriosityStatus {
    /// Whether autonomous exploration is enabled
    pub autonomous_enabled: bool,

    /// Total cells explored
    pub total_cells: usize,

    /// Average confidence (0.0-1.0)
    pub avg_confidence: f32,

    /// Current average surprise
    pub avg_surprise: f32,

    /// Exploration queue size
    pub queue_size: usize,
}

// ============================================================================
// Statistics API Models
// ============================================================================

/// Detailed statistics response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResponse {
    /// Gateway statistics
    pub gateway: GatewayStats,

    /// Curiosity statistics (if available)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub curiosity: Option<CuriosityStats>,
}

/// Gateway statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayStats {
    pub total_signals: u64,
    pub text_signals: u64,
    pub tick_signals: u64,
    pub command_signals: u64,
    pub feedback_signals: u64,
    pub unknown_words: u64,
    pub queue_overflows: u64,
    pub timeouts: u64,
    pub errors: u64,
    pub avg_processing_time_us: f64,
    pub success_rate: f32,
}

/// Curiosity statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CuriosityStats {
    pub uncertainty: UncertaintyStats,
    pub surprise: SurpriseStats,
    pub novelty: NoveltyStats,
    pub exploration: ExplorationStats,
    pub autonomous_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyStats {
    pub total_cells: usize,
    pub total_visits: usize,
    pub avg_confidence: f32,
    pub avg_visits: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurpriseStats {
    pub current_surprise: f32,
    pub avg_surprise: f32,
    pub max_recent_surprise: f32,
    pub history_size: usize,
    pub total_events: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoveltyStats {
    pub unique_states: usize,
    pub total_observations: usize,
    pub total_unique_seen: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplorationStats {
    pub queue_size: usize,
    pub total_added: usize,
    pub total_explored: usize,
}

// ============================================================================
// Health Check Models
// ============================================================================

/// Health check response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    /// Status: "healthy" or "unhealthy"
    pub status: String,

    /// Detailed checks
    pub checks: HashMap<String, bool>,
}

// ============================================================================
// Error Models
// ============================================================================

/// API error response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorResponse {
    /// Error code
    pub error: String,

    /// Human-readable message
    pub message: String,

    /// Optional details
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

impl ErrorResponse {
    pub fn new(error: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            message: message.into(),
            details: None,
        }
    }

    pub fn with_details(mut self, details: impl Into<String>) -> Self {
        self.details = Some(details.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query_request_serialization() {
        let req = QueryRequest {
            query: "hello world".to_string(),
            context: HashMap::new(),
            timeout_ms: Some(5000),
        };

        let json = serde_json::to_string(&req).unwrap();
        let deserialized: QueryRequest = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.query, "hello world");
        assert_eq!(deserialized.timeout_ms, Some(5000));
    }

    #[test]
    fn test_feedback_type_serialization() {
        let positive = FeedbackType::Positive { strength: 0.8 };
        let json = serde_json::to_string(&positive).unwrap();
        assert!(json.contains("\"type\":\"positive\""));
        assert!(json.contains("\"strength\":0.8"));

        let correction = FeedbackType::Correction {
            correct_value: "correct answer".to_string(),
        };
        let json = serde_json::to_string(&correction).unwrap();
        assert!(json.contains("\"type\":\"correction\""));
    }

    #[test]
    fn test_error_response() {
        let err = ErrorResponse::new("bad_request", "Invalid query")
            .with_details("Query cannot be empty");

        assert_eq!(err.error, "bad_request");
        assert_eq!(err.message, "Invalid query");
        assert_eq!(err.details, Some("Query cannot be empty".to_string()));
    }
}
