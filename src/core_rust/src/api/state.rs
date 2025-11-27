// NeuroGraph OS - REST API State v0.39.0
//
// Shared state for API handlers

use crate::gateway::Gateway;
use crate::curiosity::CuriosityDrive;
use crate::feedback::FeedbackProcessor;
use std::sync::Arc;
use std::time::Instant;

/// API configuration
#[derive(Debug, Clone)]
pub struct ApiConfig {
    /// Host to bind to
    pub host: String,

    /// Port to bind to
    pub port: u16,

    /// Enable CORS
    pub enable_cors: bool,

    /// API key for authentication (optional)
    pub api_key: Option<String>,

    /// Request timeout in milliseconds
    pub request_timeout_ms: u64,

    /// Rate limit: requests per minute
    pub rate_limit_per_minute: Option<u32>,
}

impl Default for ApiConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 3000,
            enable_cors: true,
            api_key: None,
            request_timeout_ms: 30000,
            rate_limit_per_minute: None,
        }
    }
}

impl ApiConfig {
    /// Load from environment variables
    pub fn from_env() -> Self {
        Self {
            host: std::env::var("NEUROGRAPH_HOST")
                .unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: std::env::var("NEUROGRAPH_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(3000),
            enable_cors: std::env::var("NEUROGRAPH_CORS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(true),
            api_key: std::env::var("NEUROGRAPH_API_KEY").ok(),
            request_timeout_ms: std::env::var("NEUROGRAPH_TIMEOUT_MS")
                .ok()
                .and_then(|t| t.parse().ok())
                .unwrap_or(30000),
            rate_limit_per_minute: std::env::var("NEUROGRAPH_RATE_LIMIT")
                .ok()
                .and_then(|r| r.parse().ok()),
        }
    }

    /// Get bind address
    pub fn bind_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

/// Shared API state
#[derive(Clone)]
pub struct ApiState {
    /// Gateway for processing signals
    pub gateway: Arc<Gateway>,

    /// Feedback processor
    pub feedback_processor: Arc<FeedbackProcessor>,

    /// Curiosity drive (optional)
    pub curiosity: Option<Arc<CuriosityDrive>>,

    /// API configuration
    pub config: Arc<ApiConfig>,

    /// Server start time
    pub start_time: Instant,
}

impl ApiState {
    /// Create new API state
    pub fn new(
        gateway: Arc<Gateway>,
        feedback_processor: Arc<FeedbackProcessor>,
        config: ApiConfig,
    ) -> Self {
        Self {
            gateway,
            feedback_processor,
            curiosity: None,
            config: Arc::new(config),
            start_time: Instant::now(),
        }
    }

    /// Create with curiosity drive
    pub fn with_curiosity(
        gateway: Arc<Gateway>,
        feedback_processor: Arc<FeedbackProcessor>,
        curiosity: Arc<CuriosityDrive>,
        config: ApiConfig,
    ) -> Self {
        Self {
            gateway,
            feedback_processor,
            curiosity: Some(curiosity),
            config: Arc::new(config),
            start_time: Instant::now(),
        }
    }

    /// Get uptime in seconds
    pub fn uptime_seconds(&self) -> u64 {
        self.start_time.elapsed().as_secs()
    }

    /// Check if API key is valid (if configured)
    pub fn validate_api_key(&self, provided_key: Option<&str>) -> bool {
        match (&self.config.api_key, provided_key) {
            (Some(expected), Some(provided)) => expected == provided,
            (None, _) => true, // No API key required
            (Some(_), None) => false, // API key required but not provided
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_config_default() {
        let config = ApiConfig::default();
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 3000);
        assert!(config.enable_cors);
        assert_eq!(config.bind_address(), "127.0.0.1:3000");
    }

    #[test]
    fn test_api_key_validation() {
        // Without API key
        let state = ApiState {
            gateway: Arc::new(Gateway::new(
                tokio::sync::mpsc::channel(100).0,
                Arc::new(parking_lot::RwLock::new(crate::bootstrap::BootstrapLibrary::new(Default::default()))),
                Default::default(),
            )),
            feedback_processor: Arc::new(FeedbackProcessor::new(
                Arc::new(parking_lot::RwLock::new(crate::bootstrap::BootstrapLibrary::new(Default::default()))),
                Arc::new(parking_lot::RwLock::new(crate::experience_stream::ExperienceStream::new(1000, 10))),
                Arc::new(parking_lot::RwLock::new(crate::IntuitionEngine::new(
                    Default::default(),
                    Arc::new(crate::experience_stream::ExperienceStream::new(1000, 10)),
                    Arc::new(crate::adna::InMemoryADNAReader::new(Default::default())),
                    tokio::sync::mpsc::channel(100).0,
                ))),
            )),
            curiosity: None,
            config: Arc::new(ApiConfig::default()),
            start_time: Instant::now(),
        };

        // No key required - should accept anything
        assert!(state.validate_api_key(None));
        assert!(state.validate_api_key(Some("any-key")));

        // With API key
        let mut config = ApiConfig::default();
        config.api_key = Some("secret-key".to_string());

        let state_with_key = ApiState {
            gateway: state.gateway.clone(),
            feedback_processor: state.feedback_processor.clone(),
            curiosity: None,
            config: Arc::new(config),
            start_time: Instant::now(),
        };

        // Correct key
        assert!(state_with_key.validate_api_key(Some("secret-key")));

        // Wrong key
        assert!(!state_with_key.validate_api_key(Some("wrong-key")));

        // No key provided
        assert!(!state_with_key.validate_api_key(None));
    }
}
