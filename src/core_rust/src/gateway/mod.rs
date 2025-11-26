pub mod channels;
pub mod config;
pub mod normalizer;
pub mod signals;
pub mod stats;

use crate::action_executor::ActionResult;
use crate::bootstrap::BootstrapLibrary;
use channels::{create_result_channel, PendingRequests, ResultReceiver, SignalReceipt};
use config::GatewayConfig;
use normalizer::{NormalizationError, Normalizer};
use signals::{
    InputSignal, ProcessedMetadata, ProcessedSignal, SignalSource, SignalType, SystemCommand,
};
use stats::GatewayStats;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{SystemTime, UNIX_EPOCH};
use tokio::sync::mpsc;

/// Gateway errors
#[derive(Debug)]
pub enum GatewayError {
    EmptyInput,
    InputTooLong(usize),
    QueueFull,
    NormalizationFailed(String),
    NotImplemented(String),
    InvalidCommand(String),
    SendFailed,
}

impl std::fmt::Display for GatewayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GatewayError::EmptyInput => write!(f, "Input is empty"),
            GatewayError::InputTooLong(len) => write!(f, "Input too long: {} characters", len),
            GatewayError::QueueFull => write!(f, "Processing queue is full"),
            GatewayError::NormalizationFailed(msg) => write!(f, "Normalization failed: {}", msg),
            GatewayError::NotImplemented(msg) => write!(f, "Not implemented: {}", msg),
            GatewayError::InvalidCommand(msg) => write!(f, "Invalid command: {}", msg),
            GatewayError::SendFailed => write!(f, "Failed to send signal to queue"),
        }
    }
}

impl std::error::Error for GatewayError {}

/// Gateway - единая точка входа для всех сигналов
pub struct Gateway {
    /// Sender to ActionController queue
    sender: mpsc::Sender<ProcessedSignal>,

    /// Normalizer for converting text to states
    normalizer: Normalizer,

    /// Configuration
    config: GatewayConfig,

    /// Pending requests waiting for results
    pending_requests: Arc<PendingRequests>,

    /// Statistics
    stats: Arc<RwLock<GatewayStats>>,

    /// Signal counter for generating IDs
    signal_counter: AtomicU64,
}

impl Gateway {
    /// Create new Gateway
    pub fn new(
        sender: mpsc::Sender<ProcessedSignal>,
        bootstrap: Arc<RwLock<BootstrapLibrary>>,
        config: GatewayConfig,
    ) -> Self {
        let normalizer = Normalizer::new(bootstrap, config.clone());

        Self {
            sender,
            normalizer,
            config,
            pending_requests: Arc::new(PendingRequests::new()),
            stats: Arc::new(RwLock::new(GatewayStats::new())),
            signal_counter: AtomicU64::new(0),
        }
    }

    /// Generate unique signal ID
    fn generate_signal_id(&self) -> u64 {
        self.signal_counter.fetch_add(1, Ordering::SeqCst)
    }

    /// Get current timestamp in milliseconds
    fn now_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }

    /// Inject a signal into the system
    pub async fn inject(
        &self,
        signal: InputSignal,
    ) -> Result<(SignalReceipt, ResultReceiver), GatewayError> {
        let start = std::time::Instant::now();

        // Generate signal ID
        let signal_id = self.generate_signal_id();
        let received_at = Self::now_ms();

        // Create result channel
        let (result_tx, result_rx) = create_result_channel();

        // Store pending request
        self.pending_requests.insert(signal_id, result_tx);

        // Update stats
        if let Ok(mut stats) = self.stats.write() {
            stats.total_signals += 1;
        }

        // Process signal based on type
        let processed = match signal {
            InputSignal::Text {
                content,
                source,
                metadata: _,
            } => self.process_text(signal_id, received_at, content, source)?,

            InputSignal::SystemTick {
                tick_number,
                timestamp,
            } => self.process_tick(signal_id, received_at, tick_number, timestamp),

            InputSignal::DirectState { state, label } => {
                self.process_direct_state(signal_id, received_at, state, label)
            }

            InputSignal::DirectToken {
                token_id,
                operation: _,
            } => {
                if let Ok(mut stats) = self.stats.write() {
                    stats.direct_token_signals += 1;
                }
                return Err(GatewayError::NotImplemented(
                    "DirectToken not yet implemented".to_string(),
                ));
            }

            InputSignal::Command { command, args: _ } => {
                if let Ok(mut stats) = self.stats.write() {
                    stats.command_signals += 1;
                }
                self.process_command(signal_id, received_at, command)?
            }

            InputSignal::Feedback {
                reference_id: _,
                feedback_type: _,
                content: _,
            } => {
                if let Ok(mut stats) = self.stats.write() {
                    stats.feedback_signals += 1;
                }
                return Err(GatewayError::NotImplemented(
                    "Feedback not yet implemented".to_string(),
                ));
            }
        };

        // Send to queue
        let queue_position = self.sender.max_capacity() - self.sender.capacity();
        self.sender
            .send(processed)
            .await
            .map_err(|_| GatewayError::SendFailed)?;

        // Update processing time stats
        let processing_time_us = start.elapsed().as_micros() as u64;
        if let Ok(mut stats) = self.stats.write() {
            stats.total_processing_time_us += processing_time_us;
        }

        // Create receipt
        let receipt = SignalReceipt::new(signal_id, received_at, queue_position);

        Ok((receipt, result_rx))
    }

    /// Process text signal
    fn process_text(
        &self,
        signal_id: u64,
        received_at: u64,
        content: String,
        source: SignalSource,
    ) -> Result<ProcessedSignal, GatewayError> {
        // Update stats
        if let Ok(mut stats) = self.stats.write() {
            stats.text_signals += 1;
        }

        // Validate length
        let trimmed = content.trim();
        if trimmed.is_empty() {
            return Err(GatewayError::EmptyInput);
        }

        if trimmed.len() > self.config.max_text_length {
            return Err(GatewayError::InputTooLong(trimmed.len()));
        }

        // Classify text type
        let signal_type = self.classify_text(trimmed);

        // Normalize text to state
        let norm_result = self
            .normalizer
            .normalize_text(trimmed)
            .map_err(|e| match e {
                NormalizationError::NoWords => GatewayError::EmptyInput,
                NormalizationError::AllUnknown => {
                    GatewayError::NormalizationFailed("All words unknown".to_string())
                }
                NormalizationError::BootstrapLocked => {
                    GatewayError::NormalizationFailed("Bootstrap library locked".to_string())
                }
            })?;

        // Update unknown words stats
        if let Ok(mut stats) = self.stats.write() {
            stats.unknown_words += norm_result.unknown_words.len() as u64;
        }

        // Build metadata
        let metadata = ProcessedMetadata {
            original_text: Some(trimmed.to_string()),
            matched_tokens: norm_result.matched_tokens.clone(),
            unknown_words: norm_result.unknown_words,
            processing_time_ns: 0, // Updated by caller
        };

        // Extract token IDs
        let related_tokens: Vec<u32> = norm_result
            .matched_tokens
            .iter()
            .map(|(_, id, _)| *id)
            .collect();

        let mut signal = ProcessedSignal::new(signal_id, norm_result.state, signal_type, source);
        signal.received_at = received_at;
        signal = signal
            .with_metadata(metadata)
            .with_confidence(norm_result.confidence)
            .with_tokens(related_tokens);

        Ok(signal)
    }

    /// Process system tick
    fn process_tick(
        &self,
        signal_id: u64,
        received_at: u64,
        _tick_number: u64,
        _timestamp: u64,
    ) -> ProcessedSignal {
        if let Ok(mut stats) = self.stats.write() {
            stats.tick_signals += 1;
        }

        // Create time-based state (simple implementation)
        let state = [0.0; 8]; // TODO: Could use tick_number/timestamp to create meaningful state

        let mut signal = ProcessedSignal::new(
            signal_id,
            state,
            SignalType::CuriosityTrigger,
            SignalSource::InternalTimer,
        );
        signal.received_at = received_at;
        signal
    }

    /// Process direct state signal
    fn process_direct_state(
        &self,
        signal_id: u64,
        received_at: u64,
        state: [f32; 8],
        label: Option<String>,
    ) -> ProcessedSignal {
        if let Ok(mut stats) = self.stats.write() {
            stats.direct_state_signals += 1;
        }

        let mut metadata = ProcessedMetadata::default();
        if let Some(l) = label {
            metadata.original_text = Some(l);
        }

        let mut signal = ProcessedSignal::new(
            signal_id,
            state,
            SignalType::SemanticQuery,
            SignalSource::Console,
        )
        .with_metadata(metadata);
        signal.received_at = received_at;
        signal
    }

    /// Process command signal
    fn process_command(
        &self,
        signal_id: u64,
        received_at: u64,
        _command: SystemCommand,
    ) -> Result<ProcessedSignal, GatewayError> {
        // Commands are handled specially - they don't go through normal processing
        // For now, just create a system signal
        let state = [0.0; 8];

        let mut signal = ProcessedSignal::new(
            signal_id,
            state,
            SignalType::SystemSignal,
            SignalSource::Console,
        );
        signal.received_at = received_at;
        Ok(signal)
    }

    /// Classify text to determine signal type
    fn classify_text(&self, text: &str) -> SignalType {
        let lower = text.to_lowercase();

        // Command
        if text.starts_with('/') || text.starts_with(':') {
            return SignalType::SystemSignal;
        }

        // Question
        if text.ends_with('?')
            || lower.contains("what")
            || lower.contains("who")
            || lower.contains("where")
            || lower.contains("why")
            || lower.contains("how")
            || lower.contains("что")
            || lower.contains("кто")
            || lower.contains("где")
            || lower.contains("почему")
            || lower.contains("как")
        {
            return SignalType::SemanticQuery;
        }

        // Action request
        if lower.starts_with("create")
            || lower.starts_with("connect")
            || lower.starts_with("создай")
            || lower.starts_with("соедини")
        {
            return SignalType::ActionRequest;
        }

        // Default to semantic query
        SignalType::SemanticQuery
    }

    /// Complete a request with a result (called by ActionController)
    pub fn complete_request(&self, signal_id: u64, result: ActionResult) {
        if let Some((_, sender)) = self.pending_requests.remove(&signal_id) {
            // Send result back to waiting receiver
            let _ = sender.send(result); // Ignore error if receiver dropped
        }
    }

    /// Clean up stale requests that are too old
    pub fn cleanup_stale_requests(&self, max_age_ms: u64) {
        let now = Self::now_ms();
        let mut to_remove = Vec::new();

        // Find stale requests
        for entry in self.pending_requests.iter() {
            let signal_id = *entry.key();
            // We don't have timestamps in PendingRequests, so we'll rely on signal_id ordering
            // In production, you'd want to store timestamps with each request
            if signal_id < now.saturating_sub(max_age_ms) {
                to_remove.push(signal_id);
            }
        }

        // Remove them
        for signal_id in to_remove {
            self.pending_requests.remove(&signal_id);

            if let Ok(mut stats) = self.stats.write() {
                stats.timeouts += 1;
            }
        }
    }

    /// Get current statistics
    pub fn stats(&self) -> GatewayStats {
        self.stats
            .read()
            .map(|s| s.clone())
            .unwrap_or_default()
    }

    /// Get pending requests count
    pub fn pending_count(&self) -> usize {
        self.pending_requests.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_text_question() {
        use crate::bootstrap::BootstrapConfig;
        let bootstrap = Arc::new(RwLock::new(BootstrapLibrary::new(BootstrapConfig::default())));
        let (tx, _rx) = mpsc::channel(100);
        let gateway = Gateway::new(tx, bootstrap, GatewayConfig::default());

        assert_eq!(
            gateway.classify_text("What is this?"),
            SignalType::SemanticQuery
        );
        assert_eq!(
            gateway.classify_text("Что это?"),
            SignalType::SemanticQuery
        );
    }

    #[test]
    fn test_classify_text_command() {
        use crate::bootstrap::BootstrapConfig;
        let bootstrap = Arc::new(RwLock::new(BootstrapLibrary::new(BootstrapConfig::default())));
        let (tx, _rx) = mpsc::channel(100);
        let gateway = Gateway::new(tx, bootstrap, GatewayConfig::default());

        assert_eq!(
            gateway.classify_text("/status"),
            SignalType::SystemSignal
        );
        assert_eq!(gateway.classify_text(":help"), SignalType::SystemSignal);
    }

    #[test]
    fn test_classify_text_action() {
        use crate::bootstrap::BootstrapConfig;
        let bootstrap = Arc::new(RwLock::new(BootstrapLibrary::new(BootstrapConfig::default())));
        let (tx, _rx) = mpsc::channel(100);
        let gateway = Gateway::new(tx, bootstrap, GatewayConfig::default());

        assert_eq!(
            gateway.classify_text("create token"),
            SignalType::ActionRequest
        );
        assert_eq!(
            gateway.classify_text("connect nodes"),
            SignalType::ActionRequest
        );
    }
}
