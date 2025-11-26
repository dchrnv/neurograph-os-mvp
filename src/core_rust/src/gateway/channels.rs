use crate::action_executor::ActionResult;
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use tokio::sync::oneshot;

/// Receipt returned after injecting a signal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignalReceipt {
    /// Unique signal ID
    pub signal_id: u64,

    /// Timestamp when signal was received (milliseconds since UNIX_EPOCH)
    pub received_at: u64,

    /// Position in the processing queue
    pub queue_position: usize,
}

impl SignalReceipt {
    pub fn new(signal_id: u64, received_at: u64, queue_position: usize) -> Self {
        Self {
            signal_id,
            received_at,
            queue_position,
        }
    }
}

/// Receiver for getting the result of a processed signal
pub type ResultReceiver = oneshot::Receiver<ActionResult>;

/// Sender for delivering results back to waiting requests
pub type ResultSender = oneshot::Sender<ActionResult>;

/// Thread-safe map of pending requests waiting for results
pub type PendingRequests = DashMap<u64, ResultSender>;

/// Create a new result channel
pub fn create_result_channel() -> (ResultSender, ResultReceiver) {
    oneshot::channel()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_signal_receipt_creation() {
        let receipt = SignalReceipt::new(123, 1000, 5);
        assert_eq!(receipt.signal_id, 123);
        assert_eq!(receipt.received_at, 1000);
        assert_eq!(receipt.queue_position, 5);
    }

    #[test]
    fn test_pending_requests() {
        let pending = PendingRequests::new();
        let (tx, _rx) = create_result_channel();

        pending.insert(1, tx);
        assert_eq!(pending.len(), 1);

        let removed = pending.remove(&1);
        assert!(removed.is_some());
        assert_eq!(pending.len(), 0);
    }
}
