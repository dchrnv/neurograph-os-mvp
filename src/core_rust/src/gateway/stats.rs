use serde::{Deserialize, Serialize};

/// Gateway statistics
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GatewayStats {
    /// Total signals received
    pub total_signals: u64,

    /// Text signals processed
    pub text_signals: u64,

    /// System tick signals
    pub tick_signals: u64,

    /// Command signals
    pub command_signals: u64,

    /// Feedback signals
    pub feedback_signals: u64,

    /// Direct token signals
    pub direct_token_signals: u64,

    /// Direct state signals
    pub direct_state_signals: u64,

    /// Unknown words encountered
    pub unknown_words: u64,

    /// Total processing time in microseconds
    pub total_processing_time_us: u64,

    /// Queue overflows (signal rejected due to full queue)
    pub queue_overflows: u64,

    /// Timeouts (requests that didn't complete in time)
    pub timeouts: u64,

    /// Errors during processing
    pub errors: u64,
}

impl GatewayStats {
    pub fn new() -> Self {
        Self::default()
    }

    /// Average processing time per signal in microseconds
    pub fn avg_processing_time_us(&self) -> f64 {
        if self.total_signals == 0 {
            0.0
        } else {
            self.total_processing_time_us as f64 / self.total_signals as f64
        }
    }

    /// Success rate (1.0 = 100% success)
    pub fn success_rate(&self) -> f64 {
        if self.total_signals == 0 {
            1.0
        } else {
            let failures = self.errors + self.timeouts + self.queue_overflows;
            1.0 - (failures as f64 / self.total_signals as f64)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_stats() {
        let stats = GatewayStats::new();
        assert_eq!(stats.total_signals, 0);
        assert_eq!(stats.avg_processing_time_us(), 0.0);
        assert_eq!(stats.success_rate(), 1.0);
    }

    #[test]
    fn test_avg_processing_time() {
        let mut stats = GatewayStats::new();
        stats.total_signals = 10;
        stats.total_processing_time_us = 1000;
        assert_eq!(stats.avg_processing_time_us(), 100.0);
    }

    #[test]
    fn test_success_rate() {
        let mut stats = GatewayStats::new();
        stats.total_signals = 100;
        stats.errors = 5;
        stats.timeouts = 3;
        stats.queue_overflows = 2;
        // 90% success rate
        assert!((stats.success_rate() - 0.9).abs() < 0.001);
    }
}
