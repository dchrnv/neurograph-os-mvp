// NeuroGraph OS - Adaptive Tracing Sampling v0.44.3
// Copyright (C) 2024-2025 Chernov Denys
//
// Intelligent trace sampling with CDNA integration to minimize observability overhead.
//
// # Architecture
//
// ```
// Request → Sampling Decision → Span Creation (conditional)
//    ↓           ↓                      ↓
//  Normal    1% sample             No-op (99%)
//  Error     100% sample           Full trace (1%)
//  Slow      50% sample            Partial trace
// ```
//
// ## Key Features
//
// - **Adaptive Sampling**: Dynamic rate based on operation outcome
// - **CDNA Integration**: Configuration driven by Constitutional DNA
// - **True No-Op**: Zero overhead when not sampling (no span creation)
// - **Error Tracking**: 100% sampling for failed operations
// - **Latency-Based**: Higher sampling for slow operations
//
// ## Performance Impact
//
// - Without sampling: 17x overhead (697ns per span)
// - With 1% sampling: ~1.2x overhead (expected)
// - Target: <1.5x overhead vs baseline

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use rand::Rng;

/// Configuration for adaptive trace sampling
#[derive(Debug, Clone)]
pub struct TraceSamplingConfig {
    /// Base sampling rate for normal operations (0.0-1.0)
    /// Default: 0.01 (1%)
    pub base_rate: f32,

    /// Sampling rate boost for operations with errors (0.0-1.0)
    /// Default: 1.0 (100% - always trace errors)
    pub error_boost: f32,

    /// Sampling rate boost for slow operations (0.0-1.0)
    /// Default: 0.5 (50% - trace half of slow ops)
    pub slow_request_boost: f32,

    /// Latency threshold to consider operation "slow" (milliseconds)
    /// Default: 100ms
    pub slow_threshold_ms: u64,

    /// Enable/disable all tracing (emergency kill switch)
    /// Default: true
    pub enabled: bool,
}

impl Default for TraceSamplingConfig {
    fn default() -> Self {
        Self {
            base_rate: 0.01,              // 1% baseline
            error_boost: 1.0,              // 100% for errors
            slow_request_boost: 0.5,       // 50% for slow ops
            slow_threshold_ms: 100,        // 100ms threshold
            enabled: true,
        }
    }
}

impl TraceSamplingConfig {
    /// Create config with custom base rate
    pub fn with_base_rate(base_rate: f32) -> Self {
        Self {
            base_rate,
            ..Default::default()
        }
    }

    /// Disable all tracing (emergency kill switch)
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Default::default()
        }
    }
}

/// Sampling decision for a trace operation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamplingDecision {
    /// Record this trace (create real span)
    Record,
    /// Skip this trace (create no-op span)
    Skip,
}

/// Context for making sampling decisions
#[derive(Debug, Clone)]
pub struct SamplingContext {
    /// Start time of the operation
    pub start_time: Instant,
    /// Whether the operation resulted in an error
    pub is_error: bool,
    /// Custom sampling rate override (if any)
    pub custom_rate: Option<f32>,
}

impl SamplingContext {
    /// Create new sampling context for an operation
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            is_error: false,
            custom_rate: None,
        }
    }

    /// Mark this operation as having an error
    pub fn with_error(mut self) -> Self {
        self.is_error = true;
        self
    }

    /// Set custom sampling rate for this operation
    pub fn with_custom_rate(mut self, rate: f32) -> Self {
        self.custom_rate = Some(rate.clamp(0.0, 1.0));
        self
    }

    /// Calculate elapsed time since operation start
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
}

/// Adaptive trace sampler with statistics
pub struct TraceSampler {
    config: TraceSamplingConfig,
    stats: Arc<SamplingStats>,
}

impl TraceSampler {
    /// Create new sampler with given configuration
    pub fn new(config: TraceSamplingConfig) -> Self {
        Self {
            config,
            stats: Arc::new(SamplingStats::default()),
        }
    }

    /// Create sampler with default configuration
    pub fn default() -> Self {
        Self::new(TraceSamplingConfig::default())
    }

    /// Make sampling decision for an operation
    pub fn should_sample(&self, context: &SamplingContext) -> SamplingDecision {
        // Emergency kill switch
        if !self.config.enabled {
            self.stats.decisions_skipped.fetch_add(1, Ordering::Relaxed);
            return SamplingDecision::Skip;
        }

        // Custom rate override
        if let Some(custom_rate) = context.custom_rate {
            return self.sample_with_rate(custom_rate);
        }

        // Error operations: use error_boost rate
        if context.is_error {
            self.stats.error_operations.fetch_add(1, Ordering::Relaxed);
            crate::metrics::TRACING_ERROR_OPERATIONS.inc();
            return self.sample_with_rate(self.config.error_boost);
        }

        // Slow operations: use slow_request_boost rate
        let elapsed_ms = context.elapsed().as_millis() as u64;
        if elapsed_ms >= self.config.slow_threshold_ms {
            self.stats.slow_operations.fetch_add(1, Ordering::Relaxed);
            crate::metrics::TRACING_SLOW_OPERATIONS.inc();
            return self.sample_with_rate(self.config.slow_request_boost);
        }

        // Normal operations: use base_rate
        self.sample_with_rate(self.config.base_rate)
    }

    /// Sample with given probability
    fn sample_with_rate(&self, rate: f32) -> SamplingDecision {
        self.stats.decisions_total.fetch_add(1, Ordering::Relaxed);

        // Update Prometheus metrics
        crate::metrics::TRACING_SAMPLES_TOTAL.inc();

        if rate >= 1.0 {
            // Always sample
            self.stats.decisions_recorded.fetch_add(1, Ordering::Relaxed);
            crate::metrics::TRACING_SAMPLES_RECORDED.inc();
            return SamplingDecision::Record;
        }

        if rate <= 0.0 {
            // Never sample
            self.stats.decisions_skipped.fetch_add(1, Ordering::Relaxed);
            crate::metrics::TRACING_SAMPLES_SKIPPED.inc();
            return SamplingDecision::Skip;
        }

        // Probabilistic sampling
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() < rate {
            self.stats.decisions_recorded.fetch_add(1, Ordering::Relaxed);
            crate::metrics::TRACING_SAMPLES_RECORDED.inc();
            SamplingDecision::Record
        } else {
            self.stats.decisions_skipped.fetch_add(1, Ordering::Relaxed);
            crate::metrics::TRACING_SAMPLES_SKIPPED.inc();
            SamplingDecision::Skip
        }
    }

    /// Get current sampling statistics
    pub fn stats(&self) -> SamplingStatsSnapshot {
        SamplingStatsSnapshot {
            decisions_total: self.stats.decisions_total.load(Ordering::Relaxed),
            decisions_recorded: self.stats.decisions_recorded.load(Ordering::Relaxed),
            decisions_skipped: self.stats.decisions_skipped.load(Ordering::Relaxed),
            error_operations: self.stats.error_operations.load(Ordering::Relaxed),
            slow_operations: self.stats.slow_operations.load(Ordering::Relaxed),
        }
    }

    /// Update configuration (hot reload)
    pub fn update_config(&mut self, config: TraceSamplingConfig) {
        self.config = config;
    }

    /// Get current configuration
    pub fn config(&self) -> &TraceSamplingConfig {
        &self.config
    }
}

/// Shared sampling statistics (lock-free atomics)
#[derive(Debug, Default)]
struct SamplingStats {
    /// Total sampling decisions made
    decisions_total: AtomicU64,
    /// Decisions that resulted in recording
    decisions_recorded: AtomicU64,
    /// Decisions that resulted in skipping
    decisions_skipped: AtomicU64,
    /// Operations with errors
    error_operations: AtomicU64,
    /// Slow operations (above threshold)
    slow_operations: AtomicU64,
}

/// Statistics snapshot
#[derive(Debug, Clone, Copy)]
pub struct SamplingStatsSnapshot {
    pub decisions_total: u64,
    pub decisions_recorded: u64,
    pub decisions_skipped: u64,
    pub error_operations: u64,
    pub slow_operations: u64,
}

impl SamplingStatsSnapshot {
    /// Calculate actual sampling rate
    pub fn actual_rate(&self) -> f32 {
        if self.decisions_total == 0 {
            return 0.0;
        }
        self.decisions_recorded as f32 / self.decisions_total as f32
    }

    /// Calculate skip rate
    pub fn skip_rate(&self) -> f32 {
        if self.decisions_total == 0 {
            return 0.0;
        }
        self.decisions_skipped as f32 / self.decisions_total as f32
    }
}

/// Helper macro for conditional tracing with sampling
///
/// # Example
///
/// ```rust
/// let sampler = TraceSampler::default();
/// let context = SamplingContext::new();
///
/// sample_span!(sampler, context, "operation_name", {
///     // This code only runs if sampling decision is Record
///     expensive_operation();
/// });
/// ```
#[macro_export]
macro_rules! sample_span {
    ($sampler:expr, $context:expr, $name:expr, $body:block) => {
        match $sampler.should_sample(&$context) {
            $crate::tracing_sampling::SamplingDecision::Record => {
                let span = tracing::info_span!($name);
                let _guard = span.enter();
                $body
            }
            $crate::tracing_sampling::SamplingDecision::Skip => {
                // No-op: zero overhead
                $body
            }
        }
    };
}

// Note: For OpenTelemetry integration, use Sampler::TraceIdRatioBased(rate)
// directly in tracing_otel.rs instead of creating a custom sampler.
// This provides tail-based sampling at the OpenTelemetry SDK level.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sampling_config_default() {
        let config = TraceSamplingConfig::default();
        assert_eq!(config.base_rate, 0.01);
        assert_eq!(config.error_boost, 1.0);
        assert_eq!(config.slow_request_boost, 0.5);
        assert_eq!(config.slow_threshold_ms, 100);
        assert!(config.enabled);
    }

    #[test]
    fn test_sampling_context() {
        let context = SamplingContext::new();
        assert!(!context.is_error);
        assert!(context.custom_rate.is_none());

        let context = context.with_error();
        assert!(context.is_error);

        let context = SamplingContext::new().with_custom_rate(0.5);
        assert_eq!(context.custom_rate, Some(0.5));
    }

    #[test]
    fn test_sampler_error_boost() {
        let config = TraceSamplingConfig {
            base_rate: 0.01,
            error_boost: 1.0,
            ..Default::default()
        };
        let sampler = TraceSampler::new(config);

        // Error operations should always be sampled
        let context = SamplingContext::new().with_error();
        for _ in 0..100 {
            assert_eq!(sampler.should_sample(&context), SamplingDecision::Record);
        }

        let stats = sampler.stats();
        assert_eq!(stats.error_operations, 100);
        assert_eq!(stats.decisions_recorded, 100);
    }

    #[test]
    fn test_sampler_disabled() {
        let config = TraceSamplingConfig::disabled();
        let sampler = TraceSampler::new(config);

        let context = SamplingContext::new();
        for _ in 0..100 {
            assert_eq!(sampler.should_sample(&context), SamplingDecision::Skip);
        }

        let stats = sampler.stats();
        assert_eq!(stats.decisions_skipped, 100);
    }

    #[test]
    fn test_sampling_stats() {
        let sampler = TraceSampler::default();

        // Sample 1000 normal operations
        for _ in 0..1000 {
            let context = SamplingContext::new();
            sampler.should_sample(&context);
        }

        let stats = sampler.stats();
        assert_eq!(stats.decisions_total, 1000);

        // With 1% base rate, expect ~10 samples (±5 due to randomness)
        let recorded = stats.decisions_recorded;
        assert!(recorded >= 5 && recorded <= 15, "Expected ~10 samples, got {}", recorded);

        // Verify actual rate is close to base rate
        let actual_rate = stats.actual_rate();
        assert!(actual_rate >= 0.005 && actual_rate <= 0.015,
            "Expected rate ~0.01, got {}", actual_rate);
    }
}
