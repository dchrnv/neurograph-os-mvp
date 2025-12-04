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

/// Logging Utilities v1.0 - Structured logging helpers
///
/// Provides convenience functions and macros for consistent structured logging
/// across the NeuroGraph system.
///
/// # Usage
///
/// ```rust
/// use neurograph_core::logging_utils::*;
///
/// // Initialize with env filter
/// init_logging("info");
///
/// // Log with context
/// log_operation_start("token_creation", "Creating batch of 1000 tokens");
/// log_operation_complete("token_creation", 1.5, "success");
/// ```

use tracing::{info, warn, error};
use tracing_subscriber::{self, EnvFilter};

/// Initialize logging with custom filter
///
/// # Arguments
///
/// * `filter` - Log level filter (e.g., "info", "debug", "warn")
///
/// # Example
///
/// ```rust
/// use neurograph_core::logging_utils::init_logging;
///
/// init_logging("info");
/// ```
pub fn init_logging(filter: &str) {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(filter.parse().unwrap_or(tracing::Level::INFO.into())),
        )
        .with_target(true)
        .with_thread_ids(false)
        .with_line_number(true)
        .init();
}

/// Initialize production logging (compact format, structured)
///
/// Better for production environments with log aggregation systems.
///
/// # Example
///
/// ```rust
/// use neurograph_core::logging_utils::init_production_logging;
///
/// init_production_logging();
/// ```
pub fn init_production_logging() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env()
                .add_directive(tracing::Level::INFO.into()),
        )
        .compact()
        .with_target(true)
        .with_thread_ids(true)
        .with_line_number(true)
        .init();
}

/// Log operation start with context
pub fn log_operation_start(operation: &str, details: &str) {
    info!(
        operation = operation,
        details = details,
        "Operation started"
    );
}

/// Log operation completion with duration
pub fn log_operation_complete(operation: &str, duration_secs: f64, status: &str) {
    info!(
        operation = operation,
        duration_secs = duration_secs,
        status = status,
        "Operation completed"
    );
}

/// Log operation failure
pub fn log_operation_failed(operation: &str, error: &str) {
    error!(
        operation = operation,
        error = error,
        "Operation failed"
    );
}

/// Log resource usage
pub fn log_resource_usage(resource: &str, current: usize, max: Option<usize>) {
    if let Some(max_val) = max {
        let percent = (current as f64 / max_val as f64) * 100.0;
        info!(
            resource = resource,
            current = current,
            max = max_val,
            percent = format!("{:.1}%", percent),
            "Resource usage"
        );
    } else {
        info!(
            resource = resource,
            current = current,
            "Resource usage (unlimited)"
        );
    }
}

/// Log warning about approaching resource limit
pub fn log_resource_warning(resource: &str, current: usize, max: usize, threshold: f64) {
    let percent = (current as f64 / max as f64) * 100.0;
    warn!(
        resource = resource,
        current = current,
        max = max,
        percent = format!("{:.1}%", percent),
        threshold = format!("{:.1}%", threshold * 100.0),
        "Resource usage exceeds threshold"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logging_functions() {
        // These tests just ensure the functions compile and run
        // Actual output depends on tracing subscriber configuration

        log_operation_start("test_op", "Testing logging");
        log_operation_complete("test_op", 0.5, "success");
        log_operation_failed("test_op", "test error");
        log_resource_usage("memory", 1024, Some(2048));
        log_resource_usage("tokens", 100, None);
        log_resource_warning("memory", 1800, 2048, 0.8);
    }
}
