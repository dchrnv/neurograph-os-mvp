// ActionExecutor trait and related structures for ActionController v1.0

use async_trait::async_trait;
use serde_json::Value;
use std::time::Duration;

/// Result of an action execution
#[derive(Debug, Clone)]
pub struct ActionResult {
    /// Whether the action succeeded
    pub success: bool,
    /// Output data in JSON format
    pub output: Value,
    /// Duration of execution in milliseconds
    pub duration_ms: u64,
    /// Error message if action failed
    pub error: Option<String>,
}

impl ActionResult {
    /// Create a successful action result
    pub fn success(output: Value, duration_ms: u64) -> Self {
        Self {
            success: true,
            output,
            duration_ms,
            error: None,
        }
    }

    /// Create a failed action result
    pub fn failure(error: String, duration_ms: u64) -> Self {
        Self {
            success: false,
            output: Value::Null,
            duration_ms,
            error: Some(error),
        }
    }
}

/// Errors that can occur during action execution
#[derive(Debug, thiserror::Error)]
pub enum ActionError {
    #[error("Executor not found: {0}")]
    ExecutorNotFound(String),

    #[error("Policy not found for state")]
    PolicyNotFound(String),

    #[error("Execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),

    #[error("Timeout after {0:?}")]
    Timeout(Duration),

    #[error("ADNA reader error: {0}")]
    ADNAError(String),
}

/// Common trait for all action executors
///
/// Each executor implements a specific capability (moving tokens, sending messages, etc.)
/// and can be registered with ActionController.
#[async_trait]
pub trait ActionExecutor: Send + Sync {
    /// Unique identifier for this executor
    fn id(&self) -> &str;

    /// Human-readable description of what this executor does
    fn description(&self) -> &str;

    /// Execute the action with given parameters
    ///
    /// # Arguments
    /// * `params` - JSON parameters for the action
    ///
    /// # Returns
    /// * `ActionResult` with success status, output data, and duration
    async fn execute(&self, params: Value) -> ActionResult;

    /// Validate parameters before execution (optional)
    ///
    /// Default implementation accepts all parameters.
    /// Override to add validation logic.
    fn validate_params(&self, _params: &Value) -> Result<(), String> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_action_result_success() {
        let result = ActionResult::success(json!({"moved": true}), 100);
        assert!(result.success);
        assert_eq!(result.duration_ms, 100);
        assert!(result.error.is_none());
        assert_eq!(result.output, json!({"moved": true}));
    }

    #[test]
    fn test_action_result_failure() {
        let result = ActionResult::failure("Invalid token ID".to_string(), 50);
        assert!(!result.success);
        assert_eq!(result.duration_ms, 50);
        assert_eq!(result.error, Some("Invalid token ID".to_string()));
        assert_eq!(result.output, Value::Null);
    }

    #[test]
    fn test_action_error_display() {
        let err = ActionError::ExecutorNotFound("test_executor".to_string());
        assert_eq!(err.to_string(), "Executor not found: test_executor");

        let err = ActionError::InvalidParameters("missing field".to_string());
        assert_eq!(err.to_string(), "Invalid parameters: missing field");
    }
}
