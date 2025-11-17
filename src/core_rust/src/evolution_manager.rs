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

//! EvolutionManager v1.0 - Safe ADNA Evolution
//!
//! Receives Proposals from IntuitionEngine, validates them against CDNA rules,
//! and atomically applies safe changes to ADNA. Logs all decisions to ExperienceStream
//! for meta-learning feedback loop.
//!
//! # Safety Principles
//!
//! - All changes must pass CDNA validation
//! - Atomic "all or nothing" application
//! - Complete audit trail via ExperienceStream
//! - Rollback capability (version tracking)

use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::mpsc;
use parking_lot::RwLock;

use crate::adna::{Proposal, ActionPolicy};
use crate::cdna::CDNA;
use crate::experience_stream::{ExperienceStream, ExperienceEvent, EventType as ExperienceEventType};

/// Configuration for EvolutionManager
#[derive(Debug, Clone)]
pub struct EvolutionConfig {
    /// Maximum proposals to process per second (rate limiting)
    pub max_proposals_per_sec: usize,

    /// Require minimum confidence for acceptance
    pub min_confidence_threshold: f64,

    /// Enable strict CDNA validation
    pub strict_validation: bool,
}

impl Default for EvolutionConfig {
    fn default() -> Self {
        Self {
            max_proposals_per_sec: 10,
            min_confidence_threshold: 0.75,
            strict_validation: true,
        }
    }
}

/// Validation result
#[derive(Debug, Clone)]
pub enum ValidationResult {
    Accepted { reason: String },
    Rejected { reason: String },
}

/// In-memory ADNA state (simplified for MVP)
///
/// In production, this would interface with actual ADNA storage.
/// For now, we maintain a simple map of state_bin_id → ActionPolicy
pub struct ADNAState {
    /// Map of state bin ID to action policy
    policies: RwLock<HashMap<String, ActionPolicy>>,
}

impl ADNAState {
    pub fn new() -> Self {
        Self {
            policies: RwLock::new(HashMap::new()),
        }
    }

    /// Get policy for state
    pub fn get_policy(&self, state_id: &str) -> Option<ActionPolicy> {
        self.policies.read().get(state_id).cloned()
    }

    /// Apply proposal (atomic update)
    pub fn apply_proposal(&self, proposal: &Proposal) -> Result<(), String> {
        let mut policies = self.policies.write();

        // Parse proposed change
        let change = &proposal.proposed_change;

        // Extract action weights from JSON
        if let Some(value) = change.get("value") {
            if let Some(weights_obj) = value.as_object() {
                let mut policy = ActionPolicy::new(&proposal.target_entity_id);

                for (action_str, weight_val) in weights_obj {
                    if let Ok(action_type) = action_str.parse::<u16>() {
                        if let Some(weight) = weight_val.as_f64() {
                            policy.set_weight(action_type, weight);
                        }
                    }
                }

                policies.insert(proposal.target_entity_id.clone(), policy);
                return Ok(());
            }
        }

        Err("Invalid proposal format".to_string())
    }

    /// Get total number of policies
    pub fn policy_count(&self) -> usize {
        self.policies.read().len()
    }
}

/// EvolutionManager - Safe ADNA evolution orchestrator
pub struct EvolutionManager {
    config: EvolutionConfig,
    adna_state: Arc<ADNAState>,
    cdna: Arc<CDNA>,
    experience_stream: Arc<ExperienceStream>,
    proposal_receiver: mpsc::Receiver<Proposal>,
}

impl EvolutionManager {
    /// Create new EvolutionManager
    pub fn new(
        config: EvolutionConfig,
        adna_state: Arc<ADNAState>,
        cdna: Arc<CDNA>,
        experience_stream: Arc<ExperienceStream>,
        proposal_receiver: mpsc::Receiver<Proposal>,
    ) -> Self {
        Self {
            config,
            adna_state,
            cdna,
            experience_stream,
            proposal_receiver,
        }
    }

    /// Run main proposal processing loop
    pub async fn run(mut self) {
        println!("[EvolutionManager] Starting proposal processing loop");

        let mut rate_limiter = tokio::time::interval(
            tokio::time::Duration::from_millis(1000 / self.config.max_proposals_per_sec.max(1) as u64)
        );

        while let Some(proposal) = self.proposal_receiver.recv().await {
            rate_limiter.tick().await; // Rate limiting

            if let Err(e) = self.process_proposal(proposal).await {
                eprintln!("[EvolutionManager] Error processing proposal: {}", e);
            }
        }

        println!("[EvolutionManager] Proposal channel closed, shutting down");
    }

    /// Process single proposal: validate → apply → log
    async fn process_proposal(&self, proposal: Proposal) -> Result<(), String> {
        println!("[EvolutionManager] Processing proposal: {}", proposal.target_entity_id);

        // 1. Validate proposal
        let validation_result = self.validate_proposal(&proposal).await;

        let (accepted, reason) = match &validation_result {
            ValidationResult::Accepted { reason } => (true, reason.clone()),
            ValidationResult::Rejected { reason } => (false, reason.clone()),
        };

        println!("[EvolutionManager] Validation: {} - {}",
            if accepted { "ACCEPTED" } else { "REJECTED" },
            reason);

        if accepted {
            // 2. Apply proposal atomically
            match self.adna_state.apply_proposal(&proposal) {
                Ok(_) => {
                    println!("[EvolutionManager] Successfully applied proposal to ADNA state");

                    // 3a. Log success to ExperienceStream
                    self.log_outcome(&proposal, true, "Proposal applied successfully").await;
                }
                Err(e) => {
                    println!("[EvolutionManager] Failed to apply proposal: {}", e);

                    // 3b. Log failure
                    self.log_outcome(&proposal, false, &format!("Application failed: {}", e)).await;
                }
            }
        } else {
            // 3c. Log rejection
            self.log_outcome(&proposal, false, &reason).await;
        }

        Ok(())
    }

    /// Validate proposal against CDNA rules and internal constraints
    async fn validate_proposal(&self, proposal: &Proposal) -> ValidationResult {
        // Check 1: Confidence threshold
        if proposal.confidence < self.config.min_confidence_threshold {
            return ValidationResult::Rejected {
                reason: format!(
                    "Confidence {:.2} below threshold {:.2}",
                    proposal.confidence,
                    self.config.min_confidence_threshold
                ),
            };
        }

        // Check 2: Expected impact (must be positive or at least significant)
        if proposal.expected_impact < 0.1 {
            return ValidationResult::Rejected {
                reason: format!(
                    "Expected impact {:.2} too low",
                    proposal.expected_impact
                ),
            };
        }

        // Check 3: CDNA validation (simplified for MVP)
        if self.config.strict_validation {
            if let Err(e) = self.validate_against_cdna(proposal).await {
                return ValidationResult::Rejected {
                    reason: format!("CDNA violation: {}", e),
                };
            }
        }

        // Check 4: Format validation
        if !self.validate_proposal_format(proposal) {
            return ValidationResult::Rejected {
                reason: "Invalid proposal format".to_string(),
            };
        }

        ValidationResult::Accepted {
            reason: format!(
                "Passed all checks (confidence: {:.1}%, impact: {:.2})",
                proposal.confidence * 100.0,
                proposal.expected_impact
            ),
        }
    }

    /// Validate against CDNA constitutional rules
    ///
    /// In full implementation, this would check:
    /// - No deletion of protected rules
    /// - Action weights within allowed bounds
    /// - Consistency with constitutional constraints
    async fn validate_against_cdna(&self, _proposal: &Proposal) -> Result<(), String> {
        // Simplified MVP validation
        // In production: query CDNA for relevant rules and check compliance

        // Example checks:
        // - Weights must sum to ≤ 1.0
        // - No negative weights
        // - Action types must be valid

        Ok(()) // Accept for now (MVP)
    }

    /// Validate proposal format
    fn validate_proposal_format(&self, proposal: &Proposal) -> bool {
        // Check that proposed_change has required structure
        if let Some(op) = proposal.proposed_change.get("op") {
            if op.as_str() != Some("replace") {
                return false; // Only support "replace" operations for now
            }
        } else {
            return false;
        }

        if proposal.proposed_change.get("value").is_none() {
            return false;
        }

        true
    }

    /// Log proposal outcome to ExperienceStream (meta-learning)
    async fn log_outcome(&self, proposal: &Proposal, accepted: bool, reason: &str) {
        let event_type = if accepted {
            ExperienceEventType::ProposalAccepted
        } else {
            ExperienceEventType::ProposalRejected
        };

        let mut event = ExperienceEvent::default();
        event.event_type = event_type as u16;
        event.timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;

        // Encode proposal info in state/action (simplified)
        event.state[0] = proposal.confidence as f32;
        event.state[1] = proposal.expected_impact as f32;
        event.action[0] = if accepted { 1.0 } else { 0.0 };

        // Write to stream
        if let Err(e) = self.experience_stream.write_event(event) {
            eprintln!("[EvolutionManager] Failed to log outcome: {}", e);
        } else {
            println!("[EvolutionManager] Logged {} event: {}",
                if accepted { "acceptance" } else { "rejection" },
                reason);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    #[test]
    fn test_config_default() {
        let config = EvolutionConfig::default();
        assert_eq!(config.max_proposals_per_sec, 10);
        assert_eq!(config.min_confidence_threshold, 0.75);
        assert!(config.strict_validation);
    }

    #[test]
    fn test_adna_state() {
        let state = ADNAState::new();
        assert_eq!(state.policy_count(), 0);

        // Create test proposal
        let proposal = Proposal::new(
            "test_state_1".to_string(),
            serde_json::json!({
                "op": "replace",
                "path": "/action_weights",
                "value": {
                    "1": 0.7,
                    "2": 0.3,
                }
            }),
            "Test proposal".to_string(),
            1.0,
            0.9,
        );

        // Apply proposal
        state.apply_proposal(&proposal).unwrap();
        assert_eq!(state.policy_count(), 1);

        // Retrieve policy
        let policy = state.get_policy("test_state_1").unwrap();
        assert_eq!(policy.get_weight(1), 0.7);
        assert_eq!(policy.get_weight(2), 0.3);
    }

    #[tokio::test]
    async fn test_validation_confidence() {
        let config = EvolutionConfig {
            min_confidence_threshold: 0.8,
            ..Default::default()
        };

        let adna_state = Arc::new(ADNAState::new());
        let cdna = Arc::new(CDNA::default());
        let stream = Arc::new(ExperienceStream::new(1000, 100));
        let (_tx, rx) = mpsc::channel(100);

        let manager = EvolutionManager::new(config, adna_state, cdna, stream, rx);

        // Test low confidence proposal
        let low_confidence = Proposal::new(
            "test".to_string(),
            serde_json::json!({"op": "replace", "value": {}}),
            "test".to_string(),
            1.0,
            0.5, // Below threshold
        );

        let result = manager.validate_proposal(&low_confidence).await;
        match result {
            ValidationResult::Rejected { reason } => {
                assert!(reason.contains("Confidence"));
            }
            _ => panic!("Expected rejection"),
        }

        // Test high confidence proposal
        let high_confidence = Proposal::new(
            "test".to_string(),
            serde_json::json!({"op": "replace", "value": {}}),
            "test".to_string(),
            1.0,
            0.9, // Above threshold
        );

        let result = manager.validate_proposal(&high_confidence).await;
        match result {
            ValidationResult::Accepted { .. } => {} // OK
            _ => panic!("Expected acceptance"),
        }
    }

    #[test]
    fn test_proposal_format_validation() {
        let config = EvolutionConfig::default();
        let adna_state = Arc::new(ADNAState::new());
        let cdna = Arc::new(CDNA::default());
        let stream = Arc::new(ExperienceStream::new(1000, 100));
        let (_tx, rx) = mpsc::channel(100);

        let manager = EvolutionManager::new(config, adna_state, cdna, stream, rx);

        // Valid format
        let valid = Proposal::new(
            "test".to_string(),
            serde_json::json!({
                "op": "replace",
                "path": "/action_weights",
                "value": {"1": 0.5}
            }),
            "test".to_string(),
            1.0,
            0.9,
        );

        assert!(manager.validate_proposal_format(&valid));

        // Invalid format (missing op)
        let invalid = Proposal::new(
            "test".to_string(),
            serde_json::json!({"value": {}}),
            "test".to_string(),
            1.0,
            0.9,
        );

        assert!(!manager.validate_proposal_format(&invalid));
    }
}