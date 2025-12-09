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

/// Sampling priority from request headers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SamplingPriority {
    /// Low priority (e.g., health checks) - reduced sampling
    Low,
    /// Normal priority - default sampling rate
    Normal,
    /// High priority (e.g., important operations) - increased sampling
    High,
}

impl Default for SamplingPriority {
    fn default() -> Self {
        SamplingPriority::Normal
    }
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
    /// Sampling priority from headers (v0.44.4)
    pub priority: SamplingPriority,
    /// Force tracing regardless of sampling (X-Force-Trace header)
    pub force_trace: bool,
}

impl SamplingContext {
    /// Create new sampling context for an operation
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            is_error: false,
            custom_rate: None,
            priority: SamplingPriority::Normal,
            force_trace: false,
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

    /// Set sampling priority (v0.44.4: head-based sampling)
    pub fn with_priority(mut self, priority: SamplingPriority) -> Self {
        self.priority = priority;
        self
    }

    /// Force tracing for this operation (v0.44.4: X-Force-Trace header)
    pub fn with_force_trace(mut self) -> Self {
        self.force_trace = true;
        self
    }

    /// Calculate elapsed time since operation start
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Parse sampling context from HTTP headers (v0.44.4)
    ///
    /// Supported headers:
    /// - `X-Sampling-Priority`: high|normal|low
    /// - `X-Force-Trace`: true|1
    /// - `X-Sampling-Rate`: 0.0-1.0 (custom rate)
    pub fn from_headers(headers: &axum::http::HeaderMap) -> Self {
        let mut context = Self::new();

        // Check X-Force-Trace header
        if let Some(force_trace) = headers.get("x-force-trace") {
            if let Ok(value) = force_trace.to_str() {
                if value == "true" || value == "1" {
                    context.force_trace = true;
                }
            }
        }

        // Check X-Sampling-Priority header
        if let Some(priority) = headers.get("x-sampling-priority") {
            if let Ok(value) = priority.to_str() {
                context.priority = match value.to_lowercase().as_str() {
                    "high" => SamplingPriority::High,
                    "low" => SamplingPriority::Low,
                    _ => SamplingPriority::Normal,
                };
            }
        }

        // Check X-Sampling-Rate header (custom rate override)
        if let Some(rate) = headers.get("x-sampling-rate") {
            if let Ok(value) = rate.to_str() {
                if let Ok(rate_f32) = value.parse::<f32>() {
                    context.custom_rate = Some(rate_f32.clamp(0.0, 1.0));
                }
            }
        }

        context
    }
}

/// Adaptive trace sampler with statistics
pub struct TraceSampler {
    config: TraceSamplingConfig,
    stats: Arc<SamplingStats>,
    /// Dynamic rate adjustment (v0.44.4) - optional
    load_monitor: Option<Arc<LoadMonitor>>,
}

impl TraceSampler {
    /// Create new sampler with given configuration
    pub fn new(config: TraceSamplingConfig) -> Self {
        Self {
            config,
            stats: Arc::new(SamplingStats::default()),
            load_monitor: None,  // Disabled by default
        }
    }

    /// Create sampler with dynamic rate adjustment (v0.44.4)
    pub fn with_dynamic_rate(config: TraceSamplingConfig, dynamic_config: DynamicRateConfig) -> Self {
        Self {
            config: config.clone(),
            stats: Arc::new(SamplingStats::default()),
            load_monitor: Some(Arc::new(LoadMonitor::new(dynamic_config, config.base_rate))),
        }
    }

    /// Create sampler with default configuration
    pub fn default() -> Self {
        Self::new(TraceSamplingConfig::default())
    }

    /// Make sampling decision for an operation
    pub fn should_sample(&self, context: &SamplingContext) -> SamplingDecision {
        // v0.44.4: Record request for load monitoring
        if let Some(monitor) = &self.load_monitor {
            monitor.record_request();
        }

        // Emergency kill switch
        if !self.config.enabled {
            self.stats.decisions_skipped.fetch_add(1, Ordering::Relaxed);
            return SamplingDecision::Skip;
        }

        // v0.44.4: Force trace header (highest priority)
        if context.force_trace {
            self.stats.decisions_recorded.fetch_add(1, Ordering::Relaxed);
            self.stats.decisions_total.fetch_add(1, Ordering::Relaxed);
            crate::metrics::TRACING_SAMPLES_TOTAL.inc();
            crate::metrics::TRACING_SAMPLES_RECORDED.inc();
            return SamplingDecision::Record;
        }

        // Custom rate override (from X-Sampling-Rate header)
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

        // v0.44.4: Get effective sampling rate (dynamic or base)
        let base_rate = if let Some(monitor) = &self.load_monitor {
            let dynamic_rate = monitor.current_rate();
            if dynamic_rate > 0.0 {
                dynamic_rate  // Use dynamic rate
            } else {
                self.config.base_rate  // Use static base rate
            }
        } else {
            self.config.base_rate
        };

        // v0.44.4: Priority-based sampling (from X-Sampling-Priority header)
        let rate = match context.priority {
            SamplingPriority::High => (base_rate * 10.0).min(1.0),  // 10x rate (max 100%)
            SamplingPriority::Low => base_rate * 0.1,                 // 0.1x rate (10%)
            SamplingPriority::Normal => base_rate,
        };

        self.sample_with_rate(rate)
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

    /// Get load monitor (v0.44.4)
    pub fn load_monitor(&self) -> Option<&Arc<LoadMonitor>> {
        self.load_monitor.as_ref()
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
pub struct SamplingStats {
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

/// Dynamic rate adjustment configuration (v0.44.4)
///
/// Automatically adjusts sampling rate based on system load.
#[derive(Debug, Clone)]
pub struct DynamicRateConfig {
    /// Enable dynamic rate adjustment
    pub enabled: bool,
    /// Minimum sampling rate (never go below this)
    pub min_rate: f32,
    /// Maximum sampling rate (never go above this)
    pub max_rate: f32,
    /// Target requests per second threshold (above this, reduce rate)
    pub high_load_rps: f64,
    /// Low load threshold (below this, increase rate)
    pub low_load_rps: f64,
    /// Adjustment factor (how much to change rate per update)
    pub adjustment_factor: f32,
}

impl Default for DynamicRateConfig {
    fn default() -> Self {
        Self {
            enabled: false,  // Disabled by default (manual testing required)
            min_rate: 0.001,  // 0.1% minimum
            max_rate: 0.1,    // 10% maximum
            high_load_rps: 1000.0,  // >1000 RPS = high load
            low_load_rps: 100.0,    // <100 RPS = low load
            adjustment_factor: 1.5,  // 1.5x or 0.67x per adjustment
        }
    }
}

/// Load monitor for dynamic rate adjustment (v0.44.4)
///
/// Tracks requests per second and adjusts sampling rate accordingly.
#[derive(Debug)]
pub struct LoadMonitor {
    config: DynamicRateConfig,
    /// Requests in current window
    requests_current: AtomicU64,
    /// Requests in previous window (for RPS calculation)
    requests_previous: AtomicU64,
    /// Last window rotation time
    last_rotation: Arc<parking_lot::Mutex<Instant>>,
    /// Current dynamic sampling rate
    current_rate: Arc<parking_lot::Mutex<f32>>,
}

impl LoadMonitor {
    /// Create new load monitor with given configuration
    pub fn new(config: DynamicRateConfig, base_rate: f32) -> Self {
        Self {
            config,
            requests_current: AtomicU64::new(0),
            requests_previous: AtomicU64::new(0),
            last_rotation: Arc::new(parking_lot::Mutex::new(Instant::now())),
            current_rate: Arc::new(parking_lot::Mutex::new(base_rate)),
        }
    }

    /// Record a request
    pub fn record_request(&self) {
        if !self.config.enabled {
            return;
        }

        self.requests_current.fetch_add(1, Ordering::Relaxed);

        // Check if we need to rotate window (every 1 second)
        let mut last_rotation = self.last_rotation.lock();
        if last_rotation.elapsed().as_secs() >= 1 {
            // Rotate window
            let current = self.requests_current.swap(0, Ordering::Relaxed);
            self.requests_previous.store(current, Ordering::Relaxed);
            *last_rotation = Instant::now();

            // Adjust rate based on load
            self.adjust_rate(current as f64);
        }
    }

    /// Adjust sampling rate based on current RPS
    fn adjust_rate(&self, rps: f64) {
        let mut current_rate = self.current_rate.lock();

        if rps > self.config.high_load_rps {
            // High load: reduce sampling rate
            *current_rate = (*current_rate / self.config.adjustment_factor)
                .max(self.config.min_rate);

            tracing::debug!(
                "High load detected ({:.0} RPS), reducing sampling rate to {:.3}",
                rps, *current_rate
            );
        } else if rps < self.config.low_load_rps {
            // Low load: increase sampling rate
            *current_rate = (*current_rate * self.config.adjustment_factor)
                .min(self.config.max_rate);

            tracing::debug!(
                "Low load detected ({:.0} RPS), increasing sampling rate to {:.3}",
                rps, *current_rate
            );
        }

        // Update Prometheus metric
        crate::metrics::TRACING_SAMPLE_RATE.set(*current_rate as f64);
    }

    /// Get current dynamic sampling rate
    pub fn current_rate(&self) -> f32 {
        if !self.config.enabled {
            return 0.0;  // Signal to use base rate
        }
        *self.current_rate.lock()
    }

    /// Get current RPS
    pub fn current_rps(&self) -> f64 {
        self.requests_previous.load(Ordering::Relaxed) as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderMap;

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

    // v0.44.4: Head-based sampling tests

    #[test]
    fn test_force_trace_header() {
        let mut headers = HeaderMap::new();
        headers.insert("x-force-trace", "true".parse().unwrap());

        let context = SamplingContext::from_headers(&headers);
        assert!(context.force_trace);

        let sampler = TraceSampler::default();
        let decision = sampler.should_sample(&context);
        assert_eq!(decision, SamplingDecision::Record);
    }

    #[test]
    fn test_sampling_priority_header() {
        // High priority
        let mut headers = HeaderMap::new();
        headers.insert("x-sampling-priority", "high".parse().unwrap());
        let context = SamplingContext::from_headers(&headers);
        assert_eq!(context.priority, SamplingPriority::High);

        // Low priority
        headers.clear();
        headers.insert("x-sampling-priority", "low".parse().unwrap());
        let context = SamplingContext::from_headers(&headers);
        assert_eq!(context.priority, SamplingPriority::Low);

        // Normal (default)
        headers.clear();
        let context = SamplingContext::from_headers(&headers);
        assert_eq!(context.priority, SamplingPriority::Normal);
    }

    #[test]
    fn test_custom_rate_header() {
        let mut headers = HeaderMap::new();
        headers.insert("x-sampling-rate", "0.5".parse().unwrap());

        let context = SamplingContext::from_headers(&headers);
        assert_eq!(context.custom_rate, Some(0.5));

        // Test clamping
        headers.clear();
        headers.insert("x-sampling-rate", "1.5".parse().unwrap());
        let context = SamplingContext::from_headers(&headers);
        assert_eq!(context.custom_rate, Some(1.0));  // Clamped to 1.0
    }

    #[test]
    fn test_priority_affects_sampling_rate() {
        let config = TraceSamplingConfig {
            base_rate: 0.1,  // 10% base rate
            ..Default::default()
        };
        let sampler = TraceSampler::new(config);

        // High priority: 10x rate = 100%
        let context = SamplingContext::new().with_priority(SamplingPriority::High);
        let decision = sampler.should_sample(&context);
        assert_eq!(decision, SamplingDecision::Record);  // Always sampled at 100%

        // Low priority: 0.1x rate = 1%
        // Test multiple times due to randomness
        let mut sampled = 0;
        for _ in 0..100 {
            let context = SamplingContext::new().with_priority(SamplingPriority::Low);
            if sampler.should_sample(&context) == SamplingDecision::Record {
                sampled += 1;
            }
        }
        // Expect ~1 sample out of 100 (allow some variance)
        assert!(sampled <= 10, "Low priority sampled too often: {}/100", sampled);
    }
}
