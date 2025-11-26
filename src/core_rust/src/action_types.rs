// NeuroGraph - Высокопроизводительная система пространственных вычислений на основе токенов.
// Copyright (C) 2024-2025 Chernov Denys
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Action types and decision tracking for ActionController v2.0 "Arbitrator"
//!
//! This module defines the core types for dual-pathway decision making:
//! - ActionIntent: High-level description of desired action
//! - DecisionSource: Tracks whether decision came from Reflex (Fast) or Reasoning (Slow)
//! - ActionType: Enumeration of all possible action types in the system

use serde::{Deserialize, Serialize};

/// Types of actions available in NeuroGraph OS
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActionType {
    // Token manipulation
    CreateToken,
    ModifyToken,
    DeleteToken,
    MoveToken,

    // Connection manipulation
    CreateConnection,
    ModifyConnection,
    DeleteConnection,

    // Activation and propagation
    ActivateToken,
    PropagateSignal,

    // System actions
    UpdatePolicy,
    TriggerLearning,
    SaveState,

    // Exploration (v0.38.0 Curiosity Drive)
    Explore,

    // External actions (extensible)
    External(u32),
}

impl ActionType {
    /// Convert action type to string representation
    pub fn as_str(&self) -> &str {
        match self {
            ActionType::CreateToken => "create_token",
            ActionType::ModifyToken => "modify_token",
            ActionType::DeleteToken => "delete_token",
            ActionType::MoveToken => "move_token",
            ActionType::CreateConnection => "create_connection",
            ActionType::ModifyConnection => "modify_connection",
            ActionType::DeleteConnection => "delete_connection",
            ActionType::ActivateToken => "activate_token",
            ActionType::PropagateSignal => "propagate_signal",
            ActionType::UpdatePolicy => "update_policy",
            ActionType::TriggerLearning => "trigger_learning",
            ActionType::SaveState => "save_state",
            ActionType::Explore => "explore",
            ActionType::External(_) => "external",
        }
    }
}

/// Source of the decision - tracks which pathway was used
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DecisionSource {
    /// Reflexive decision (System 1 - Fast Path)
    Reflex {
        /// Connection ID from IntuitionEngine
        connection_id: u64,
        /// Lookup time in nanoseconds
        lookup_time_ns: u64,
        /// Similarity score (0.0-1.0)
        similarity: f32,
    },

    /// Analytical decision (System 2 - Slow Path via ADNA)
    Reasoning {
        /// ADNA policy version used
        policy_version: u32,
        /// Reasoning time in milliseconds
        reasoning_time_ms: u64,
    },

    /// Failsafe decision (emergency fallback)
    Failsafe {
        /// Reason for failsafe activation
        reason: String,
    },

    /// Curiosity-driven exploration (v0.38.0)
    Curiosity {
        /// Curiosity score that triggered exploration
        curiosity_score: f32,
        /// Reason for exploration (uncertainty/surprise/novelty)
        exploration_reason: String,
    },
}

impl DecisionSource {
    /// Check if decision came from reflex path
    pub fn is_reflex(&self) -> bool {
        matches!(self, DecisionSource::Reflex { .. })
    }

    /// Check if decision came from reasoning path
    pub fn is_reasoning(&self) -> bool {
        matches!(self, DecisionSource::Reasoning { .. })
    }

    /// Check if decision is failsafe
    pub fn is_failsafe(&self) -> bool {
        matches!(self, DecisionSource::Failsafe { .. })
    }

    /// Check if decision is curiosity-driven
    pub fn is_curiosity(&self) -> bool {
        matches!(self, DecisionSource::Curiosity { .. })
    }

    /// Get execution time in nanoseconds (for metrics)
    pub fn execution_time_ns(&self) -> u64 {
        match self {
            DecisionSource::Reflex { lookup_time_ns, .. } => *lookup_time_ns,
            DecisionSource::Reasoning { reasoning_time_ms, .. } => reasoning_time_ms * 1_000_000,
            DecisionSource::Failsafe { .. } => 0,
            DecisionSource::Curiosity { .. } => 0, // Exploration doesn't have decision time
        }
    }
}

/// High-level action intent with decision metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionIntent {
    /// Unique ID for this action
    pub action_id: u64,

    /// Type of action to perform
    pub action_type: ActionType,

    /// Action parameters (8D vector)
    pub params: [f32; 8],

    /// Source of the decision (Reflex/Reasoning/Failsafe)
    pub source: DecisionSource,

    /// Confidence in this action (0.0 - 1.0)
    pub confidence: f32,

    /// Expected reward from performing this action
    pub estimated_reward: f32,

    /// Unix timestamp (milliseconds)
    pub timestamp: u64,
}

impl ActionIntent {
    /// Create new ActionIntent with Reflex source
    pub fn from_reflex(
        action_id: u64,
        action_type: ActionType,
        params: [f32; 8],
        connection_id: u64,
        lookup_time_ns: u64,
        similarity: f32,
        confidence: f32,
    ) -> Self {
        Self {
            action_id,
            action_type,
            params,
            source: DecisionSource::Reflex {
                connection_id,
                lookup_time_ns,
                similarity,
            },
            confidence,
            estimated_reward: 0.0, // Will be filled by appraisers
            timestamp: current_timestamp_ms(),
        }
    }

    /// Create new ActionIntent with Reasoning source
    pub fn from_reasoning(
        action_id: u64,
        action_type: ActionType,
        params: [f32; 8],
        policy_version: u32,
        reasoning_time_ms: u64,
        confidence: f32,
    ) -> Self {
        Self {
            action_id,
            action_type,
            params,
            source: DecisionSource::Reasoning {
                policy_version,
                reasoning_time_ms,
            },
            confidence,
            estimated_reward: 0.0,
            timestamp: current_timestamp_ms(),
        }
    }

    /// Create failsafe ActionIntent (no-op)
    pub fn failsafe(reason: String) -> Self {
        Self {
            action_id: 0,
            action_type: ActionType::SaveState, // Safe no-op action
            params: [0.0; 8],
            source: DecisionSource::Failsafe { reason },
            confidence: 0.0,
            estimated_reward: 0.0,
            timestamp: current_timestamp_ms(),
        }
    }
}

/// Get current Unix timestamp in milliseconds
pub fn current_timestamp_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_type_as_str() {
        assert_eq!(ActionType::CreateToken.as_str(), "create_token");
        assert_eq!(ActionType::ActivateToken.as_str(), "activate_token");
        assert_eq!(ActionType::External(42).as_str(), "external");
    }

    #[test]
    fn test_decision_source_checks() {
        let reflex = DecisionSource::Reflex {
            connection_id: 123,
            lookup_time_ns: 50,
            similarity: 0.95,
        };
        assert!(reflex.is_reflex());
        assert!(!reflex.is_reasoning());
        assert!(!reflex.is_failsafe());

        let reasoning = DecisionSource::Reasoning {
            policy_version: 1,
            reasoning_time_ms: 5,
        };
        assert!(!reasoning.is_reflex());
        assert!(reasoning.is_reasoning());

        let failsafe = DecisionSource::Failsafe {
            reason: "timeout".to_string(),
        };
        assert!(failsafe.is_failsafe());
    }

    #[test]
    fn test_decision_source_execution_time() {
        let reflex = DecisionSource::Reflex {
            connection_id: 123,
            lookup_time_ns: 100,
            similarity: 0.9,
        };
        assert_eq!(reflex.execution_time_ns(), 100);

        let reasoning = DecisionSource::Reasoning {
            policy_version: 1,
            reasoning_time_ms: 5,
        };
        assert_eq!(reasoning.execution_time_ns(), 5_000_000); // 5ms = 5M ns
    }

    #[test]
    fn test_action_intent_from_reflex() {
        let intent = ActionIntent::from_reflex(
            1,
            ActionType::ActivateToken,
            [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0],
            100,
            50,
            0.95,
            0.8,
        );

        assert_eq!(intent.action_id, 1);
        assert_eq!(intent.action_type, ActionType::ActivateToken);
        assert_eq!(intent.confidence, 0.8);
        assert!(intent.source.is_reflex());
    }

    #[test]
    fn test_action_intent_from_reasoning() {
        let intent = ActionIntent::from_reasoning(
            2,
            ActionType::CreateConnection,
            [0.0; 8],
            1,
            10,
            0.6,
        );

        assert_eq!(intent.action_id, 2);
        assert!(intent.source.is_reasoning());
        assert_eq!(intent.confidence, 0.6);
    }

    #[test]
    fn test_action_intent_failsafe() {
        let intent = ActionIntent::failsafe("ADNA timeout".to_string());

        assert!(intent.source.is_failsafe());
        assert_eq!(intent.confidence, 0.0);
        assert_eq!(intent.action_type, ActionType::SaveState);
    }
}
