use serde::{Deserialize, Serialize};

/// Strategy for handling unknown words
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnknownWordStrategy {
    /// Ignore unknown words
    Ignore,
    /// Create empty state [0.0; 8]
    CreateEmpty,
    /// Trigger curiosity system to learn about the word
    TriggerCuriosity,
    /// Find nearest known word and use its state
    UseNearest,
}

/// Gateway configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    /// Maximum queue capacity for pending signals
    pub queue_capacity: usize,

    /// Processing timeout in milliseconds
    pub processing_timeout_ms: u64,

    /// Enable system tick generation
    pub enable_system_ticks: bool,

    /// Tick interval in milliseconds
    pub tick_interval_ms: u64,

    /// Maximum text length for input signals
    pub max_text_length: usize,

    /// Strategy for handling unknown words
    pub unknown_word_strategy: UnknownWordStrategy,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            queue_capacity: 10_000,
            processing_timeout_ms: 100,
            enable_system_ticks: true,
            tick_interval_ms: 1000,
            max_text_length: 4096,
            unknown_word_strategy: UnknownWordStrategy::TriggerCuriosity,
        }
    }
}

impl GatewayConfig {
    /// Validate configuration
    pub fn validate(&self) -> Result<(), String> {
        if self.queue_capacity == 0 {
            return Err("queue_capacity must be > 0".to_string());
        }

        if self.processing_timeout_ms == 0 {
            return Err("processing_timeout_ms must be > 0".to_string());
        }

        if self.enable_system_ticks && self.tick_interval_ms == 0 {
            return Err("tick_interval_ms must be > 0 when system ticks enabled".to_string());
        }

        if self.max_text_length == 0 {
            return Err("max_text_length must be > 0".to_string());
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config_valid() {
        let config = GatewayConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_invalid_queue_capacity() {
        let mut config = GatewayConfig::default();
        config.queue_capacity = 0;
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_invalid_timeout() {
        let mut config = GatewayConfig::default();
        config.processing_timeout_ms = 0;
        assert!(config.validate().is_err());
    }
}
