// NeuroGraph - Высокопроизводительная система пространственных вычислений на основе токенов.
// Copyright (C) 2024-2025 Chernov Denys

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! NoOpExecutor - does nothing, useful for testing

use crate::action_executor::{ActionExecutor, ActionResult};
use async_trait::async_trait;
use serde_json::Value;
use std::time::Instant;

/// NoOp executor that does nothing
///
/// Useful for testing ActionController without side effects.
pub struct NoOpExecutor;

impl NoOpExecutor {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ActionExecutor for NoOpExecutor {
    fn id(&self) -> &str {
        "noop"
    }

    fn description(&self) -> &str {
        "No-operation executor (does nothing)"
    }

    async fn execute(&self, _params: Value) -> ActionResult {
        let start = Instant::now();

        // Simulate tiny work
        tokio::time::sleep(tokio::time::Duration::from_millis(1)).await;

        let duration_ms = start.elapsed().as_millis() as u64;

        ActionResult::success(
            serde_json::json!({
                "action": "noop",
                "message": "No operation performed successfully"
            }),
            duration_ms
        )
    }

    fn validate_params(&self, _params: &Value) -> Result<(), String> {
        // NoOp accepts any parameters
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_noop_executor() {
        let executor = NoOpExecutor::new();

        assert_eq!(executor.id(), "noop");
        assert!(executor.description().contains("No-operation"));

        let result = executor.execute(serde_json::json!({})).await;
        assert!(result.success);
        assert!(result.duration_ms >= 1);
        assert!(result.error.is_none());
    }

    #[test]
    fn test_noop_validate() {
        let executor = NoOpExecutor::new();

        // Should accept any params
        assert!(executor.validate_params(&serde_json::json!({})).is_ok());
        assert!(executor.validate_params(&serde_json::json!({"foo": "bar"})).is_ok());
    }
}