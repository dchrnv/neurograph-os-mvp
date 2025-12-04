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

/// Prometheus Metrics v1.0 - System observability for production
///
/// This module provides Prometheus metrics export for:
/// - Token and connection counts
/// - Memory usage (from Guardian quotas)
/// - IntuitionEngine statistics
/// - Operation latencies
/// - Guardian validation stats
///
/// # Architecture
///
/// Uses global static metrics with lazy_static for thread-safe access.
/// Metrics are registered once and updated throughout system lifetime.
///
/// # Usage
///
/// ```rust
/// use neurograph_core::metrics;
///
/// // Record token creation
/// metrics::TOKENS_CREATED.inc();
///
/// // Record memory usage
/// metrics::MEMORY_USED_BYTES.set(1024000);
///
/// // Export metrics
/// let metrics_text = metrics::export_metrics();
/// ```

use lazy_static::lazy_static;
use prometheus::{
    register_counter, register_gauge, register_histogram, register_int_counter, register_int_gauge,
    Counter, Encoder, Gauge, Histogram, IntCounter, IntGauge, TextEncoder,
};

// ==================== COUNTERS ====================

lazy_static! {
    /// Total number of tokens created
    pub static ref TOKENS_CREATED: IntCounter = register_int_counter!(
        "neurograph_tokens_created_total",
        "Total number of tokens created"
    )
    .unwrap();

    /// Total number of connections created
    pub static ref CONNECTIONS_CREATED: IntCounter = register_int_counter!(
        "neurograph_connections_created_total",
        "Total number of connections created"
    )
    .unwrap();

    /// Total number of tokens validated
    pub static ref TOKENS_VALIDATED: IntCounter = register_int_counter!(
        "neurograph_tokens_validated_total",
        "Total number of tokens validated by Guardian"
    )
    .unwrap();

    /// Total number of tokens rejected
    pub static ref TOKENS_REJECTED: IntCounter = register_int_counter!(
        "neurograph_tokens_rejected_total",
        "Total number of tokens rejected by Guardian"
    )
    .unwrap();

    /// Total number of connections validated
    pub static ref CONNECTIONS_VALIDATED: IntCounter = register_int_counter!(
        "neurograph_connections_validated_total",
        "Total number of connections validated by Guardian"
    )
    .unwrap();

    /// Total number of connections rejected
    pub static ref CONNECTIONS_REJECTED: IntCounter = register_int_counter!(
        "neurograph_connections_rejected_total",
        "Total number of connections rejected by Guardian"
    )
    .unwrap();

    /// Total number of quota violations
    pub static ref QUOTA_EXCEEDED: IntCounter = register_int_counter!(
        "neurograph_quota_exceeded_total",
        "Total number of times resource quotas were exceeded"
    )
    .unwrap();

    /// Total number of aggressive cleanups triggered
    pub static ref AGGRESSIVE_CLEANUPS: IntCounter = register_int_counter!(
        "neurograph_aggressive_cleanups_total",
        "Total number of aggressive memory cleanups triggered"
    )
    .unwrap();

    /// Total number of panics recovered
    pub static ref PANICS_RECOVERED: IntCounter = register_int_counter!(
        "neurograph_panics_recovered_total",
        "Total number of panics caught and recovered"
    )
    .unwrap();

    /// Total number of WAL entries written
    pub static ref WAL_ENTRIES_WRITTEN: IntCounter = register_int_counter!(
        "neurograph_wal_entries_written_total",
        "Total number of WAL entries written"
    )
    .unwrap();

    /// Total number of WAL entries replayed
    pub static ref WAL_ENTRIES_REPLAYED: IntCounter = register_int_counter!(
        "neurograph_wal_entries_replayed_total",
        "Total number of WAL entries replayed during recovery"
    )
    .unwrap();
}

// ==================== GAUGES ====================

lazy_static! {
    /// Current number of active tokens
    pub static ref TOKENS_ACTIVE: IntGauge = register_int_gauge!(
        "neurograph_tokens_active",
        "Current number of active tokens in system"
    )
    .unwrap();

    /// Current number of active connections
    pub static ref CONNECTIONS_ACTIVE: IntGauge = register_int_gauge!(
        "neurograph_connections_active",
        "Current number of active connections in system"
    )
    .unwrap();

    /// Current memory usage in bytes
    pub static ref MEMORY_USED_BYTES: IntGauge = register_int_gauge!(
        "neurograph_memory_used_bytes",
        "Current memory usage in bytes (from Guardian tracking)"
    )
    .unwrap();

    /// Memory usage percentage (0-100)
    pub static ref MEMORY_USAGE_PERCENT: Gauge = register_gauge!(
        "neurograph_memory_usage_percent",
        "Memory usage as percentage of quota (0-100)"
    )
    .unwrap();

    /// IntuitionEngine queue size
    pub static ref INTUITION_QUEUE_SIZE: IntGauge = register_int_gauge!(
        "neurograph_intuition_queue_size",
        "Current size of IntuitionEngine event queue"
    )
    .unwrap();

    /// Guardian event queue size
    pub static ref GUARDIAN_EVENT_QUEUE_SIZE: IntGauge = register_int_gauge!(
        "neurograph_guardian_event_queue_size",
        "Current size of Guardian event queue"
    )
    .unwrap();
}

// ==================== HISTOGRAMS ====================

lazy_static! {
    /// Token creation latency in seconds
    pub static ref TOKEN_CREATION_DURATION: Histogram = register_histogram!(
        "neurograph_token_creation_duration_seconds",
        "Time taken to create a token",
        vec![0.00001, 0.0001, 0.001, 0.01, 0.1, 1.0]
    )
    .unwrap();

    /// Connection creation latency in seconds
    pub static ref CONNECTION_CREATION_DURATION: Histogram = register_histogram!(
        "neurograph_connection_creation_duration_seconds",
        "Time taken to create a connection",
        vec![0.00001, 0.0001, 0.001, 0.01, 0.1, 1.0]
    )
    .unwrap();

    /// Validation latency in seconds
    pub static ref VALIDATION_DURATION: Histogram = register_histogram!(
        "neurograph_validation_duration_seconds",
        "Time taken to validate tokens/connections",
        vec![0.00001, 0.0001, 0.001, 0.01, 0.1, 1.0]
    )
    .unwrap();

    /// WAL write latency in seconds
    pub static ref WAL_WRITE_DURATION: Histogram = register_histogram!(
        "neurograph_wal_write_duration_seconds",
        "Time taken to write WAL entry",
        vec![0.0001, 0.001, 0.01, 0.1, 1.0]
    )
    .unwrap();
}

// ==================== EXPORT ====================

/// Export all metrics in Prometheus text format
///
/// Returns a String containing all metrics in Prometheus exposition format.
/// This can be served on `/metrics` endpoint for Prometheus scraping.
///
/// # Example
///
/// ```rust
/// let metrics = neurograph_core::metrics::export_metrics();
/// // Serve on HTTP endpoint
/// ```
pub fn export_metrics() -> Result<String, String> {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();

    encoder
        .encode(&metric_families, &mut buffer)
        .map_err(|e| format!("Failed to encode metrics: {}", e))?;

    String::from_utf8(buffer).map_err(|e| format!("Failed to convert metrics to UTF-8: {}", e))
}

/// Initialize metrics system (idempotent)
///
/// Called automatically when metrics are first accessed via lazy_static.
/// Can be called explicitly to ensure metrics are registered.
pub fn init() {
    // Force lazy_static initialization
    let _ = &*TOKENS_CREATED;
    let _ = &*CONNECTIONS_CREATED;
    let _ = &*TOKENS_ACTIVE;
    let _ = &*MEMORY_USED_BYTES;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_metrics_initialization() {
        init();
        // Should not panic
    }

    #[test]
    fn test_counter_increment() {
        let before = TOKENS_CREATED.get();
        TOKENS_CREATED.inc();
        assert_eq!(TOKENS_CREATED.get(), before + 1);
    }

    #[test]
    fn test_gauge_set() {
        MEMORY_USED_BYTES.set(1024);
        assert_eq!(MEMORY_USED_BYTES.get(), 1024);
    }

    #[test]
    fn test_export_metrics() {
        TOKENS_CREATED.inc();
        MEMORY_USED_BYTES.set(2048);

        let metrics = export_metrics().unwrap();
        assert!(metrics.contains("neurograph_tokens_created_total"));
        assert!(metrics.contains("neurograph_memory_used_bytes"));
    }

    #[test]
    fn test_histogram_observe() {
        TOKEN_CREATION_DURATION.observe(0.001); // 1ms
        // Should not panic
    }
}
