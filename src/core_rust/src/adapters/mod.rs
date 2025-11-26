pub mod console;

use crate::action_executor::ActionResult;
pub use crate::{SignalSource, SignalType};
use serde_json::Value;
use std::fmt;

/// Error during output operations
#[derive(Debug)]
pub enum OutputError {
    SendFailed(String),
    FormatError(String),
    IoError(String),
}

impl fmt::Display for OutputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputError::SendFailed(msg) => write!(f, "Send failed: {}", msg),
            OutputError::FormatError(msg) => write!(f, "Format error: {}", msg),
            OutputError::IoError(msg) => write!(f, "IO error: {}", msg),
        }
    }
}

impl std::error::Error for OutputError {}

/// Context for formatting output
#[derive(Debug, Clone)]
pub struct OutputContext {
    /// Signal ID that generated this output
    pub signal_id: u64,

    /// Original input text (if any)
    pub original_input: Option<String>,

    /// Type of signal that was processed
    pub signal_type: SignalType,

    /// Source of the signal
    pub source: SignalSource,
}

impl OutputContext {
    pub fn new(
        signal_id: u64,
        original_input: Option<String>,
        signal_type: SignalType,
        source: SignalSource,
    ) -> Self {
        Self {
            signal_id,
            original_input,
            signal_type,
            source,
        }
    }
}

/// Formatted output ready for display
#[derive(Debug, Clone)]
pub struct FormattedOutput {
    /// Text representation (for console/logs)
    pub text: Option<String>,

    /// Structured data (for APIs)
    pub data: Option<Value>,
}

impl FormattedOutput {
    pub fn text(text: String) -> Self {
        Self {
            text: Some(text),
            data: None,
        }
    }

    pub fn data(data: Value) -> Self {
        Self {
            text: None,
            data: Some(data),
        }
    }

    pub fn both(text: String, data: Value) -> Self {
        Self {
            text: Some(text),
            data: Some(data),
        }
    }
}

/// Trait for output adapters (Console, REST API, WebSocket, etc.)
#[async_trait::async_trait]
pub trait OutputAdapter: Send + Sync {
    /// Name of this adapter
    fn name(&self) -> &str;

    /// Format action result for output
    async fn format_output(
        &self,
        result: &ActionResult,
        context: &OutputContext,
    ) -> Result<FormattedOutput, OutputError>;

    /// Send formatted output
    async fn send(&self, output: FormattedOutput) -> Result<(), OutputError>;
}
