//! MessageSenderExecutor - sends log messages

use crate::action_executor::{ActionExecutor, ActionResult};
use async_trait::async_trait;
use serde_json::Value;
use std::time::Instant;

/// Message sender executor
///
/// Sends messages to stdout with optional priority levels.
pub struct MessageSenderExecutor;

impl MessageSenderExecutor {
    pub fn new() -> Self {
        Self
    }

    fn get_priority(params: &Value) -> String {
        params
            .get("priority")
            .and_then(|v| v.as_str())
            .unwrap_or("info")
            .to_string()
    }

    fn get_message(params: &Value) -> Option<String> {
        params
            .get("message")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    }
}

#[async_trait]
impl ActionExecutor for MessageSenderExecutor {
    fn id(&self) -> &str {
        "message_sender"
    }

    fn description(&self) -> &str {
        "Sends log messages with configurable priority"
    }

    async fn execute(&self, params: Value) -> ActionResult {
        let start = Instant::now();

        let message = match Self::get_message(&params) {
            Some(msg) => msg,
            None => {
                return ActionResult::failure(
                    "Missing 'message' parameter".to_string(),
                    start.elapsed().as_millis() as u64
                );
            }
        };

        let priority = Self::get_priority(&params);

        // Send message to stdout with priority
        let formatted = format!("[{}] {}", priority.to_uppercase(), message);
        println!("{}", formatted);

        let duration_ms = start.elapsed().as_millis() as u64;

        ActionResult::success(
            serde_json::json!({
                "action": "message_sent",
                "message": message,
                "priority": priority,
                "formatted": formatted
            }),
            duration_ms
        )
    }

    fn validate_params(&self, params: &Value) -> Result<(), String> {
        // Check that 'message' field exists and is a string
        if params.get("message").and_then(|v| v.as_str()).is_none() {
            return Err("Missing or invalid 'message' field (must be string)".to_string());
        }

        // Optional: validate priority if provided
        if let Some(priority) = params.get("priority") {
            if let Some(p_str) = priority.as_str() {
                let valid_priorities = ["debug", "info", "warn", "error"];
                if !valid_priorities.contains(&p_str) {
                    return Err(format!(
                        "Invalid priority '{}'. Must be one of: {:?}",
                        p_str, valid_priorities
                    ));
                }
            } else {
                return Err("'priority' field must be a string".to_string());
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_message_sender_success() {
        let executor = MessageSenderExecutor::new();

        let params = serde_json::json!({
            "message": "Hello from test",
            "priority": "info"
        });

        let result = executor.execute(params).await;
        assert!(result.success);
        assert!(result.error.is_none());

        let output = result.output.as_object().unwrap();
        assert_eq!(output.get("message").unwrap(), "Hello from test");
        assert_eq!(output.get("priority").unwrap(), "info");
    }

    #[tokio::test]
    async fn test_message_sender_missing_message() {
        let executor = MessageSenderExecutor::new();

        let params = serde_json::json!({
            "priority": "info"
        });

        let result = executor.execute(params).await;
        assert!(!result.success);
        assert!(result.error.is_some());
        assert!(result.error.unwrap().contains("Missing 'message'"));
    }

    #[tokio::test]
    async fn test_message_sender_default_priority() {
        let executor = MessageSenderExecutor::new();

        let params = serde_json::json!({
            "message": "Test message"
        });

        let result = executor.execute(params).await;
        assert!(result.success);

        let output = result.output.as_object().unwrap();
        assert_eq!(output.get("priority").unwrap(), "info"); // default
    }

    #[test]
    fn test_message_sender_validate() {
        let executor = MessageSenderExecutor::new();

        // Valid
        assert!(executor.validate_params(&serde_json::json!({
            "message": "test"
        })).is_ok());

        assert!(executor.validate_params(&serde_json::json!({
            "message": "test",
            "priority": "warn"
        })).is_ok());

        // Invalid: missing message
        assert!(executor.validate_params(&serde_json::json!({})).is_err());

        // Invalid: wrong priority
        assert!(executor.validate_params(&serde_json::json!({
            "message": "test",
            "priority": "invalid"
        })).is_err());
    }
}