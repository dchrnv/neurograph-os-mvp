use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::{SystemTime, UNIX_EPOCH};

/// Source of the input signal
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalSource {
    Console,
    RestApi,
    WebSocket,
    InternalTimer,
    InternalCuriosity,
    File,
    Unknown,
}

/// Type of token operation for DirectToken signals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TokenOperation {
    Activate,
    Query,
    Connect { target_id: u32, strength: f32 },
    Modify { field: String, value: Value },
}

/// System commands
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SystemCommand {
    Status,
    Stats,
    Save,
    Load,
    Reset,
    SetConfig,
    Shutdown,
}

/// Feedback type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedbackType {
    Positive,
    Negative,
    Correction,
    Ignore,
}

/// Input signal - what comes INTO the Gateway
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InputSignal {
    Text {
        content: String,
        source: SignalSource,
        metadata: Option<Value>,
    },
    SystemTick {
        tick_number: u64,
        timestamp: u64,
    },
    DirectToken {
        token_id: u32,
        operation: TokenOperation,
    },
    DirectState {
        state: [f32; 8],
        label: Option<String>,
    },
    Command {
        command: SystemCommand,
        args: Vec<String>,
    },
    Feedback {
        reference_id: u64,
        feedback_type: FeedbackType,
        content: Option<String>,
    },
}

/// Type of processed signal - semantic interpretation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SignalType {
    SemanticQuery,
    ActionRequest,
    FeedbackSignal,
    SystemSignal,
    CuriosityTrigger,
    Unknown,
}

/// Metadata for processed signals
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedMetadata {
    pub original_text: Option<String>,
    pub matched_tokens: Vec<(String, u32, f32)>, // (word, token_id, confidence)
    pub unknown_words: Vec<String>,
    pub processing_time_ns: u64,
}

impl Default for ProcessedMetadata {
    fn default() -> Self {
        Self {
            original_text: None,
            matched_tokens: Vec::new(),
            unknown_words: Vec::new(),
            processing_time_ns: 0,
        }
    }
}

/// Processed signal - what goes to ActionController
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedSignal {
    pub signal_id: u64,
    pub received_at: u64,
    pub processed_at: u64,
    pub state: [f32; 8],
    pub signal_type: SignalType,
    pub source: SignalSource,
    pub related_tokens: Vec<u32>,
    pub interpretation_confidence: f32,
    pub metadata: ProcessedMetadata,
}

impl ProcessedSignal {
    pub fn new(
        signal_id: u64,
        state: [f32; 8],
        signal_type: SignalType,
        source: SignalSource,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;

        Self {
            signal_id,
            received_at: now,
            processed_at: now,
            state,
            signal_type,
            source,
            related_tokens: Vec::new(),
            interpretation_confidence: 1.0,
            metadata: ProcessedMetadata::default(),
        }
    }

    pub fn with_metadata(mut self, metadata: ProcessedMetadata) -> Self {
        self.metadata = metadata;
        self
    }

    pub fn with_confidence(mut self, confidence: f32) -> Self {
        self.interpretation_confidence = confidence;
        self
    }

    pub fn with_tokens(mut self, tokens: Vec<u32>) -> Self {
        self.related_tokens = tokens;
        self
    }
}
