// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2025 Chernov Denys

//! Hybrid Learning System - IntuitionEngine v2.2
//!
//! Integrates ADNA behavioral learning with Connection causal learning:
//! - ADNA learns what actions work (behavioral patterns)
//! - Connections learn why actions work (causal relationships)
//! - Unified proposal pipeline coordinates both systems
//! - Cross-system feedback loops strengthen learning
//!
//! # Architecture
//!
//! ```text
//! ┌─────────────────────────────────────┐
//! │    IntuitionEngine v2.2 Hybrid      │
//! │                                     │
//! │  ┌──────────────┐  ┌─────────────┐ │
//! │  │  ADNA        │  │ Connection  │ │
//! │  │  Behavioral  │  │ Causal      │ │
//! │  └──────┬───────┘  └──────┬──────┘ │
//! │         │                 │         │
//! │         └────┬────────┬───┘         │
//! │              ▼        ▼             │
//! │     ┌────────────────────────┐     │
//! │     │  Proposal Router       │     │
//! │     └──────────┬─────────────┘     │
//! │                ▼                    │
//! │     ┌────────────────────────┐     │
//! │     │  Guardian Validator    │     │
//! │     └──────────┬─────────────┘     │
//! │                ▼                    │
//! │     ┌────────────────────────┐     │
//! │     │  Executor              │     │
//! │     └────────────────────────┘     │
//! └─────────────────────────────────────┘
//! ```

use std::sync::Arc;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use parking_lot::RwLock;
use thiserror::Error;

use crate::adna::Proposal as ADNAProposal;
use crate::connection_v3::{
    ConnectionV3, ConnectionProposal, ConnectionType, ConnectionField,
    ConnectionMutability,
};
use crate::guardian::Guardian;
use crate::intuition_engine::IdentifiedPattern;

// ============================================================================
// Unified Proposal Types
// ============================================================================

/// Unified proposal enum for hybrid learning system
#[derive(Debug, Clone)]
pub enum HybridProposal {
    /// Behavioral learning (ADNA policy update)
    Behavioral(ADNAProposal),

    /// Causal learning (Connection update)
    Causal(ConnectionProposal),

    /// Cross-system feedback: ADNA → Connection
    /// ADNA discovers successful action → boost Connection confidence
    BehavioralToCausal {
        adna_pattern: IdentifiedPattern,
        target_connection_id: u64,
        confidence_boost: f32,
        evidence_count: u16,
    },

    /// Cross-system hint: Connection → ADNA
    /// High-confidence Connection → suggest ADNA exploration
    CausalToBehavioral {
        connection_id: u64,
        state_token: u32,
        action_token: u32,
        exploration_weight: f32,
        causal_confidence: f32,
    },
}

/// Outcome of proposal application
#[derive(Debug, Clone)]
pub enum ProposalOutcome {
    /// Behavioral proposal applied successfully
    BehavioralApplied {
        adna_generation: u32,
        fitness_delta: f32,
    },

    /// Causal proposal applied successfully
    CausalApplied {
        connection_id: u64,
        new_confidence: u8,
        new_evidence_count: u16,
    },

    /// Cross-system feedback applied
    CrossSystemFeedback {
        connections_updated: usize,
        total_confidence_boost: f32,
    },

    /// Cross-system hint sent
    CrossSystemHint {
        adna_weights_updated: usize,
    },
}

/// Errors that can occur during hybrid learning
#[derive(Debug, Error)]
pub enum HybridLearningError {
    #[error("Guardian rejected proposal: {0}")]
    GuardianRejected(String),

    #[error("Connection not found: {0}")]
    ConnectionNotFound(u64),

    #[error("Invalid proposal: {0}")]
    InvalidProposal(String),

    #[error("ADNA error: {0}")]
    ADNAError(String),

    #[error("Lock error")]
    LockError,
}

// ============================================================================
// Proposal Router
// ============================================================================

/// Central router for hybrid learning proposals
pub struct ProposalRouter {
    /// Connections storage (keyed by connection_id)
    connections: Arc<RwLock<HashMap<u64, ConnectionV3>>>,

    /// Guardian for validation
    guardian: Arc<Guardian>,

    /// Statistics tracking
    stats: Arc<RwLock<HybridLearningStats>>,
}

/// Statistics for hybrid learning system
#[derive(Debug, Clone, Default)]
pub struct HybridLearningStats {
    /// Total proposals processed
    pub total_proposals: u64,

    /// Behavioral proposals applied
    pub behavioral_applied: u64,

    /// Causal proposals applied
    pub causal_applied: u64,

    /// Cross-system feedbacks applied
    pub feedbacks_applied: u64,

    /// Cross-system hints sent
    pub hints_sent: u64,

    /// Guardian rejections
    pub guardian_rejections: u64,
}

impl ProposalRouter {
    /// Create new ProposalRouter
    pub fn new(guardian: Arc<Guardian>) -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            guardian,
            stats: Arc::new(RwLock::new(HybridLearningStats::default())),
        }
    }

    /// Add connection to router's storage
    pub fn add_connection(&self, id: u64, connection: ConnectionV3) {
        self.connections.write().insert(id, connection);
    }

    /// Get connection by ID
    pub fn get_connection(&self, id: u64) -> Option<ConnectionV3> {
        self.connections.read().get(&id).copied()
    }

    /// Route proposal to appropriate system
    pub fn route_proposal(
        &self,
        proposal: HybridProposal,
    ) -> Result<ProposalOutcome, HybridLearningError> {
        self.stats.write().total_proposals += 1;

        match proposal {
            HybridProposal::Behavioral(p) => {
                self.apply_behavioral_proposal(p)
            }
            HybridProposal::Causal(p) => {
                self.apply_causal_proposal(p)
            }
            HybridProposal::BehavioralToCausal {
                adna_pattern,
                target_connection_id,
                confidence_boost,
                evidence_count,
            } => {
                self.apply_behavioral_to_causal_feedback(
                    &adna_pattern,
                    target_connection_id,
                    confidence_boost,
                    evidence_count,
                )
            }
            HybridProposal::CausalToBehavioral {
                connection_id,
                state_token,
                action_token,
                exploration_weight,
                causal_confidence,
            } => {
                self.apply_causal_to_behavioral_hint(
                    connection_id,
                    state_token,
                    action_token,
                    exploration_weight,
                    causal_confidence,
                )
            }
        }
    }

    /// Apply behavioral (ADNA) proposal
    fn apply_behavioral_proposal(
        &self,
        _proposal: ADNAProposal,
    ) -> Result<ProposalOutcome, HybridLearningError> {
        // TODO: Implement ADNA proposal application
        // For now, just track stats
        self.stats.write().behavioral_applied += 1;

        Ok(ProposalOutcome::BehavioralApplied {
            adna_generation: 0,
            fitness_delta: 0.0,
        })
    }

    /// Apply causal (Connection) proposal
    fn apply_causal_proposal(
        &self,
        proposal: ConnectionProposal,
    ) -> Result<ProposalOutcome, HybridLearningError> {
        match &proposal {
            ConnectionProposal::Modify {
                connection_id,
                field,
                old_value: _,
                new_value,
                justification: _,
                evidence_count,
            } => {
                let mut connections = self.connections.write();
                let conn = connections
                    .get_mut(connection_id)
                    .ok_or(HybridLearningError::ConnectionNotFound(*connection_id))?;

                // Validate with Guardian
                crate::connection_v3::guardian_validation::validate_proposal(conn, &proposal)
                    .map_err(|e| {
                        self.stats.write().guardian_rejections += 1;
                        HybridLearningError::GuardianRejected(format!("{:?}", e))
                    })?;

                // Apply modification
                match field {
                    ConnectionField::Confidence => {
                        let new_conf = (*new_value * 255.0) as u8;
                        conn.confidence = new_conf;
                        conn.evidence_count = conn.evidence_count.saturating_add(*evidence_count);
                    }
                    ConnectionField::PullStrength => {
                        conn.pull_strength = *new_value;
                    }
                    ConnectionField::PreferredDistance => {
                        conn.preferred_distance = *new_value;
                    }
                    ConnectionField::LearningRate => {
                        conn.learning_rate = (*new_value * 255.0) as u8;
                    }
                    ConnectionField::DecayRate => {
                        conn.decay_rate = (*new_value * 255.0) as u8;
                    }
                }

                // Update timestamp
                conn.last_update = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as u32;

                self.stats.write().causal_applied += 1;

                Ok(ProposalOutcome::CausalApplied {
                    connection_id: *connection_id,
                    new_confidence: conn.confidence,
                    new_evidence_count: conn.evidence_count,
                })
            }
            _ => {
                // TODO: Implement Create, Delete, Promote
                Err(HybridLearningError::InvalidProposal(
                    "Only Modify proposals supported currently".to_string(),
                ))
            }
        }
    }

    /// Apply ADNA → Connection feedback
    fn apply_behavioral_to_causal_feedback(
        &self,
        _pattern: &IdentifiedPattern,
        connection_id: u64,
        confidence_boost: f32,
        evidence_count: u16,
    ) -> Result<ProposalOutcome, HybridLearningError> {
        let mut connections = self.connections.write();
        let conn = connections
            .get_mut(&connection_id)
            .ok_or(HybridLearningError::ConnectionNotFound(connection_id))?;

        // Only update Learnable connections
        if conn.mutability != ConnectionMutability::Learnable as u8 {
            return Err(HybridLearningError::InvalidProposal(
                "Can only apply feedback to Learnable connections".to_string(),
            ));
        }

        // Calculate new confidence
        let current_conf = conn.confidence as f32 / 255.0;
        let new_conf = (current_conf + confidence_boost).min(1.0);
        let total_boost = new_conf - current_conf;

        // Update connection
        conn.confidence = (new_conf * 255.0) as u8;
        conn.evidence_count = conn.evidence_count.saturating_add(evidence_count);
        conn.last_update = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as u32;

        self.stats.write().feedbacks_applied += 1;

        Ok(ProposalOutcome::CrossSystemFeedback {
            connections_updated: 1,
            total_confidence_boost: total_boost,
        })
    }

    /// Apply Connection → ADNA hint
    fn apply_causal_to_behavioral_hint(
        &self,
        _connection_id: u64,
        _state_token: u32,
        _action_token: u32,
        _exploration_weight: f32,
        _causal_confidence: f32,
    ) -> Result<ProposalOutcome, HybridLearningError> {
        // TODO: Implement ADNA weight update
        // For now, just track stats
        self.stats.write().hints_sent += 1;

        Ok(ProposalOutcome::CrossSystemHint {
            adna_weights_updated: 1,
        })
    }

    /// Get current statistics
    pub fn get_stats(&self) -> HybridLearningStats {
        self.stats.read().clone()
    }
}

// ============================================================================
// Feedback Generators
// ============================================================================

/// Generate Connection confidence update from ADNA pattern
pub fn adna_to_connection_feedback(
    pattern: &IdentifiedPattern,
    connection_id: u64,
) -> HybridProposal {
    // Calculate confidence boost based on ADNA evidence
    // Pattern confidence is 0.0-1.0, we boost Connection confidence by up to 0.2
    let confidence_boost = (pattern.confidence * 0.1).min(0.2) as f32;

    HybridProposal::BehavioralToCausal {
        adna_pattern: pattern.clone(),
        target_connection_id: connection_id,
        confidence_boost,
        evidence_count: pattern.sample_count as u16,
    }
}

/// Generate ADNA exploration hint from high-confidence Connection
pub fn connection_to_adna_hint(
    connection: &ConnectionV3,
    connection_id: u64,
) -> Option<HybridProposal> {
    // Only send hints for high-confidence causal connections
    if connection.confidence < 204 {
        // 0.8 * 255
        return None;
    }

    // Only causal connection types
    let is_causal = connection.connection_type == ConnectionType::Cause as u8
        || connection.connection_type == ConnectionType::EnabledBy as u8
        || connection.connection_type == ConnectionType::Effect as u8;

    if is_causal {
        let exploration_weight = connection.confidence as f32 / 255.0 * 0.1;

        Some(HybridProposal::CausalToBehavioral {
            connection_id,
            state_token: connection.token_a_id,
            action_token: connection.token_b_id,
            exploration_weight,
            causal_confidence: connection.confidence as f32 / 255.0,
        })
    } else {
        None
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_router() -> ProposalRouter {
        let guardian = Arc::new(Guardian::new());
        ProposalRouter::new(guardian)
    }

    #[test]
    fn test_router_creation() {
        let router = setup_test_router();
        let stats = router.get_stats();
        assert_eq!(stats.total_proposals, 0);
    }

    #[test]
    fn test_add_and_get_connection() {
        let router = setup_test_router();
        let conn = ConnectionV3::new(100, 200);
        router.add_connection(1, conn);

        let retrieved = router.get_connection(1);
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().token_a_id, 100);
    }

    #[test]
    fn test_adna_to_connection_feedback_generation() {
        let pattern = IdentifiedPattern {
            state_bin_id: 100,
            better_action: 5,
            worse_action: 3,
            reward_delta: 1.5,
            confidence: 0.85,
            sample_count: 50,
        };

        let proposal = adna_to_connection_feedback(&pattern, 1);

        match proposal {
            HybridProposal::BehavioralToCausal {
                confidence_boost,
                evidence_count,
                ..
            } => {
                assert!(confidence_boost > 0.0);
                assert!(confidence_boost <= 0.2);
                assert_eq!(evidence_count, 50);
            }
            _ => panic!("Wrong proposal type"),
        }
    }

    #[test]
    fn test_connection_to_adna_hint_high_confidence() {
        let mut conn = ConnectionV3::new(100, 200);
        conn.set_connection_type(ConnectionType::Cause);
        conn.confidence = 220; // 0.86

        let hint = connection_to_adna_hint(&conn, 1);
        assert!(hint.is_some());
    }

    #[test]
    fn test_connection_to_adna_hint_low_confidence() {
        let mut conn = ConnectionV3::new(100, 200);
        conn.set_connection_type(ConnectionType::Cause);
        conn.confidence = 128; // 0.5

        let hint = connection_to_adna_hint(&conn, 1);
        assert!(hint.is_none());
    }
}
