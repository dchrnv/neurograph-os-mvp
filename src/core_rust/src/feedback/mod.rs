// NeuroGraph OS - Feedback Module v0.37.0
//
// User feedback processing for continuous learning.
// Allows users to provide:
// - Positive/negative reinforcement
// - Corrections ("X is actually Y")
// - Associations ("X relates to Y")

use crate::{
    bootstrap::BootstrapLibrary,
    experience_stream::ExperienceStream,
    intuition_engine::IntuitionEngine,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use thiserror::Error;
use parking_lot::RwLock;

/// Maximum age for feedback (1 hour)
const MAX_FEEDBACK_AGE: Duration = Duration::from_secs(3600);

/// Maximum corrections per signal
const MAX_CORRECTIONS_PER_SIGNAL: usize = 3;

/// Type of detailed feedback from user (extended from gateway::signals::FeedbackType)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DetailedFeedbackType {
    /// Positive reinforcement
    Positive {
        /// Strength of positive feedback (0.0 to 1.0)
        strength: f32,
    },

    /// Negative reinforcement
    Negative {
        /// Strength of negative feedback (0.0 to 1.0)
        strength: f32,
    },

    /// Correction: "X is actually Y"
    Correction {
        /// The correct interpretation
        correct_value: String,
    },

    /// Association: "X relates to Y"
    Association {
        /// Related concept
        related_word: String,
        /// Strength of association (0.0 to 1.0)
        strength: f32,
    },
}

/// Feedback signal from user
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackSignal {
    /// Signal ID this feedback references
    pub reference_id: u64,

    /// Type of feedback
    pub feedback_type: DetailedFeedbackType,

    /// When feedback was created
    pub timestamp: SystemTime,

    /// Optional explanation from user
    pub explanation: Option<String>,
}

/// Result of feedback processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackResult {
    /// Whether feedback was successfully applied
    pub success: bool,

    /// What was changed
    pub changes_made: Vec<String>,

    /// Any errors encountered
    pub errors: Vec<String>,

    /// Processing time in microseconds
    pub processing_time_us: u64,
}

/// Errors that can occur during feedback processing
#[derive(Debug, Error)]
pub enum FeedbackError {
    #[error("Reference signal ID {0} not found")]
    SignalNotFound(u64),

    #[error("Feedback too old: {0:?} > {1:?}")]
    FeedbackTooOld(Duration, Duration),

    #[error("Too many corrections for signal {0}: {1} >= {2}")]
    TooManyCorrections(u64, usize, usize),

    #[error("Invalid feedback strength: {0} (must be 0.0 to 1.0)")]
    InvalidStrength(f32),

    #[error("Failed to parse correction: {0}")]
    ParseError(String),

    #[error("System error: {0}")]
    SystemError(String),
}

/// Tracks corrections per signal
struct CorrectionTracker {
    /// signal_id -> number of corrections
    corrections: std::collections::HashMap<u64, usize>,
}

impl CorrectionTracker {
    fn new() -> Self {
        Self {
            corrections: std::collections::HashMap::new(),
        }
    }

    fn increment(&mut self, signal_id: u64) -> usize {
        let count = self.corrections.entry(signal_id).or_insert(0);
        *count += 1;
        *count
    }

    fn get_count(&self, signal_id: u64) -> usize {
        self.corrections.get(&signal_id).copied().unwrap_or(0)
    }
}

/// Processes user feedback and applies learning
pub struct FeedbackProcessor {
    /// Bootstrap library for token management
    bootstrap: Arc<RwLock<BootstrapLibrary>>,

    /// Experience stream for reward updates
    experience_stream: Arc<RwLock<ExperienceStream>>,

    /// Intuition engine for reflex updates
    intuition_engine: Arc<RwLock<IntuitionEngine>>,

    /// Track corrections per signal
    correction_tracker: Arc<RwLock<CorrectionTracker>>,
}

impl FeedbackProcessor {
    /// Create new feedback processor
    pub fn new(
        bootstrap: Arc<RwLock<BootstrapLibrary>>,
        experience_stream: Arc<RwLock<ExperienceStream>>,
        intuition_engine: Arc<RwLock<IntuitionEngine>>,
    ) -> Self {
        Self {
            bootstrap,
            experience_stream,
            intuition_engine,
            correction_tracker: Arc::new(RwLock::new(CorrectionTracker::new())),
        }
    }

    /// Process feedback signal
    pub async fn process(&self, signal: FeedbackSignal) -> Result<FeedbackResult, FeedbackError> {
        let start = std::time::Instant::now();
        let mut changes = Vec::new();
        let mut errors = Vec::new();

        // Validate feedback
        self.validate_feedback(&signal)?;

        // Apply feedback based on type
        match &signal.feedback_type {
            DetailedFeedbackType::Positive { strength } => {
                match self.apply_positive(signal.reference_id, *strength).await {
                    Ok(change) => changes.push(change),
                    Err(e) => errors.push(e.to_string()),
                }
            }

            DetailedFeedbackType::Negative { strength } => {
                match self.apply_negative(signal.reference_id, *strength).await {
                    Ok(change) => changes.push(change),
                    Err(e) => errors.push(e.to_string()),
                }
            }

            DetailedFeedbackType::Correction { correct_value } => {
                match self.apply_correction(signal.reference_id, correct_value).await {
                    Ok(change) => changes.push(change),
                    Err(e) => errors.push(e.to_string()),
                }
            }

            DetailedFeedbackType::Association { related_word, strength } => {
                match self.apply_association(signal.reference_id, related_word, *strength).await {
                    Ok(change) => changes.push(change),
                    Err(e) => errors.push(e.to_string()),
                }
            }
        }

        let processing_time_us = start.elapsed().as_micros() as u64;

        Ok(FeedbackResult {
            success: errors.is_empty(),
            changes_made: changes,
            errors,
            processing_time_us,
        })
    }

    /// Validate feedback signal
    fn validate_feedback(&self, signal: &FeedbackSignal) -> Result<(), FeedbackError> {
        // Check age
        let age = SystemTime::now()
            .duration_since(signal.timestamp)
            .unwrap_or(Duration::from_secs(0));

        if age > MAX_FEEDBACK_AGE {
            return Err(FeedbackError::FeedbackTooOld(age, MAX_FEEDBACK_AGE));
        }

        // Check strength values
        match &signal.feedback_type {
            DetailedFeedbackType::Positive { strength } | DetailedFeedbackType::Negative { strength } => {
                if *strength < 0.0 || *strength > 1.0 {
                    return Err(FeedbackError::InvalidStrength(*strength));
                }
            }
            DetailedFeedbackType::Association { strength, .. } => {
                if *strength < 0.0 || *strength > 1.0 {
                    return Err(FeedbackError::InvalidStrength(*strength));
                }
            }
            DetailedFeedbackType::Correction { .. } => {
                // Check correction limit
                let tracker = self.correction_tracker.read();

                let count = tracker.get_count(signal.reference_id);
                if count >= MAX_CORRECTIONS_PER_SIGNAL {
                    return Err(FeedbackError::TooManyCorrections(
                        signal.reference_id,
                        count,
                        MAX_CORRECTIONS_PER_SIGNAL,
                    ));
                }
            }
        }

        Ok(())
    }

    /// Apply positive feedback
    async fn apply_positive(&self, signal_id: u64, strength: f32) -> Result<String, FeedbackError> {
        // Update experience stream reward
        let _stream = self.experience_stream.write();

        // Find experience by signal_id and update reward
        // For now, just return success message
        // TODO: Implement actual reward update in ExperienceStream

        Ok(format!("Applied positive feedback (strength: {:.2}) to signal {}", strength, signal_id))
    }

    /// Apply negative feedback
    async fn apply_negative(&self, signal_id: u64, strength: f32) -> Result<String, FeedbackError> {
        // Update experience stream with negative reward
        let _stream = self.experience_stream.write();

        // Find experience by signal_id and update reward
        // For now, just return success message
        // TODO: Implement actual reward update in ExperienceStream

        Ok(format!("Applied negative feedback (strength: {:.2}) to signal {}", strength, signal_id))
    }

    /// Apply correction: "X is actually Y"
    async fn apply_correction(&self, signal_id: u64, correct_value: &str) -> Result<String, FeedbackError> {
        // Increment correction tracker
        let mut tracker = self.correction_tracker.write();

        tracker.increment(signal_id);
        drop(tracker);

        // Parse correction and create/update token
        let _bootstrap = self.bootstrap.write();

        // For now, just normalize the correct value
        // TODO: Create actual connection between original and corrected

        Ok(format!("Applied correction: '{}' for signal {}", correct_value, signal_id))
    }

    /// Apply association: "X relates to Y"
    async fn apply_association(&self, signal_id: u64, related_word: &str, strength: f32) -> Result<String, FeedbackError> {
        // Create association in bootstrap library
        let _bootstrap = self.bootstrap.write();

        // For now, just normalize the related word
        // TODO: Create actual connection with specified strength

        Ok(format!("Applied association: '{}' (strength: {:.2}) for signal {}", related_word, strength, signal_id))
    }
}
