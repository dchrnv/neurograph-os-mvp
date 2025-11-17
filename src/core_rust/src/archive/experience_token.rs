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

//! Experience Token - State-Action-Reward tuples for learning
//!
//! **Version:** 3.0.0
//! **Size:** 128 bytes (cache-aligned)
//!
//! Experience tokens capture state-action-reward tuples for policy learning.
//! They form the training data for the Intuition Engine to improve ADNA policies.

use std::time::{SystemTime, UNIX_EPOCH};

/// Magic number for ExperienceToken validation: 'EXPE' in ASCII
pub const EXPERIENCE_TOKEN_MAGIC: u32 = 0x45585045;

// ============================================================================
// ExperienceToken Structure (128 bytes)
// ============================================================================

/// Experience token capturing state-action-reward tuple
///
/// This structure records everything needed for policy learning:
/// - Current state (8D representation)
/// - Action taken (8D action space)
/// - Resulting reward
/// - Next state (compressed)
/// - Episode/step metadata
///
/// Exactly 128 bytes
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ExperienceToken {
    // Header (32 bytes)
    /// Token type magic number (0x45585045 'EXPE')
    pub token_type: u32,                // 4 bytes
    /// Timestamp (Unix epoch seconds)
    pub timestamp: u64,                 // 8 bytes
    /// Episode identifier
    pub episode_id: u64,                // 8 bytes
    /// Step number within episode
    pub step_number: u32,               // 4 bytes
    /// Flags (terminal, truncated, etc.)
    pub flags: u16,                     // 2 bytes
    /// Reserved for alignment
    pub _reserved1: [u8; 6],            // 6 bytes (total: 32)

    // State vector (32 bytes)
    /// Current state (8D semantic space representation)
    /// Maps to L1-L8 coordinate spaces compressed into continuous values
    pub state: [f32; 8],                // 32 bytes (total: 64)

    // Action vector (32 bytes)
    /// Action taken in this state (8D action space)
    pub action: [f32; 8],               // 32 bytes (total: 96)

    // Result (32 bytes)
    /// Reward received
    pub reward: f32,                    // 4 bytes
    /// Next state (6D compressed representation)
    /// Compressed for space efficiency, full state in next token
    pub next_state: [f32; 6],           // 24 bytes
    /// ADNA version hash (first 4 bytes) that generated this action
    pub adna_version_hash: [u8; 4],     // 4 bytes (total: 128)
}

// Compile-time assertion: ExperienceToken must be exactly 128 bytes
const _: () = assert!(std::mem::size_of::<ExperienceToken>() == 128);

// ============================================================================
// Flags
// ============================================================================

/// Experience token flags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExperienceFlags;

impl ExperienceFlags {
    /// Episode terminated naturally
    pub const TERMINAL: u16 = 0x0001;
    /// Episode truncated (timeout, error, etc.)
    pub const TRUNCATED: u16 = 0x0002;
    /// High-value experience (for prioritized replay)
    pub const HIGH_VALUE: u16 = 0x0004;
    /// Low-value experience
    pub const LOW_VALUE: u16 = 0x0008;
    /// Exploration action (vs exploitation)
    pub const EXPLORATION: u16 = 0x0010;
    /// User feedback provided
    pub const USER_FEEDBACK: u16 = 0x0020;
    // Info flags (combined into upper byte)
    /// Success
    pub const SUCCESS: u16 = 0x0100;
    /// Failure
    pub const FAILURE: u16 = 0x0200;
    /// Timeout
    pub const TIMEOUT: u16 = 0x0400;
    /// Error occurred
    pub const ERROR: u16 = 0x0800;
    /// Novel state encountered
    pub const NOVEL: u16 = 0x1000;
}

/// Information flags (deprecated - use ExperienceFlags)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InfoFlags;

impl InfoFlags {
    /// Success (now in ExperienceFlags::SUCCESS)
    pub const SUCCESS: u8 = 0x01;
    /// Failure
    pub const FAILURE: u8 = 0x02;
    /// Timeout
    pub const TIMEOUT: u8 = 0x04;
    /// Error occurred
    pub const ERROR: u8 = 0x08;
    /// Novel state encountered
    pub const NOVEL: u8 = 0x10;
}

// ============================================================================
// Implementation
// ============================================================================

impl ExperienceToken {
    /// Create new experience token
    pub fn new(episode_id: u64, step_number: u32) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        Self {
            token_type: EXPERIENCE_TOKEN_MAGIC,
            timestamp: now,
            episode_id,
            step_number,
            flags: 0,
            _reserved1: [0; 6],
            state: [0.0; 8],
            action: [0.0; 8],
            reward: 0.0,
            next_state: [0.0; 6],
            adna_version_hash: [0; 4],
        }
    }

    /// Create experience token with full data
    pub fn with_data(
        episode_id: u64,
        step_number: u32,
        state: [f32; 8],
        action: [f32; 8],
        reward: f32,
        next_state: [f32; 6],
        adna_hash: [u8; 4],
    ) -> Self {
        let mut token = Self::new(episode_id, step_number);
        token.state = state;
        token.action = action;
        token.reward = reward;
        token.next_state = next_state;
        token.adna_version_hash = adna_hash;
        token
    }

    /// Check if token is valid
    pub fn is_valid(&self) -> bool {
        self.token_type == EXPERIENCE_TOKEN_MAGIC
    }

    /// Check if episode is done
    pub fn is_done(&self) -> bool {
        self.has_flag(ExperienceFlags::TERMINAL) || self.has_flag(ExperienceFlags::TRUNCATED)
    }

    /// Check if episode was truncated
    pub fn is_truncated(&self) -> bool {
        self.has_flag(ExperienceFlags::TRUNCATED)
    }

    /// Check if this was an exploration action
    pub fn is_exploration(&self) -> bool {
        self.has_flag(ExperienceFlags::EXPLORATION)
    }

    /// Set flag
    pub fn set_flag(&mut self, flag: u16) {
        self.flags |= flag;
    }

    /// Clear flag
    pub fn clear_flag(&mut self, flag: u16) {
        self.flags &= !flag;
    }

    /// Check if flag is set
    pub fn has_flag(&self, flag: u16) -> bool {
        (self.flags & flag) != 0
    }

    /// Mark as terminal
    pub fn mark_terminal(&mut self) {
        self.set_flag(ExperienceFlags::TERMINAL);
    }

    /// Mark as truncated
    pub fn mark_truncated(&mut self) {
        self.set_flag(ExperienceFlags::TRUNCATED);
    }

    /// Mark as high value (for prioritized replay)
    pub fn mark_high_value(&mut self) {
        self.set_flag(ExperienceFlags::HIGH_VALUE);
    }

    /// Get priority for replay (based on reward magnitude and flags)
    pub fn priority(&self) -> f32 {
        let mut priority = self.reward.abs();

        if self.has_flag(ExperienceFlags::HIGH_VALUE) {
            priority *= 2.0;
        }
        if self.has_flag(ExperienceFlags::LOW_VALUE) {
            priority *= 0.5;
        }
        if self.has_flag(ExperienceFlags::NOVEL) {
            priority *= 1.5;
        }

        priority
    }
}

impl Default for ExperienceToken {
    fn default() -> Self {
        Self::new(0, 0)
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_experience_token_size() {
        assert_eq!(std::mem::size_of::<ExperienceToken>(), 128);
    }

    #[test]
    fn test_experience_token_creation() {
        let token = ExperienceToken::new(1, 0);
        assert!(token.is_valid());
        // Copy values to avoid references to packed fields
        let ep_id = token.episode_id;
        let step = token.step_number;
        assert_eq!(ep_id, 1);
        assert_eq!(step, 0);
        assert!(!token.is_done());
    }

    #[test]
    fn test_experience_token_with_data() {
        let state = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let action = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let next_state = [0.2, 0.3, 0.4, 0.5, 0.6, 0.7];
        let adna_hash = [0xAA, 0xBB, 0xCC, 0xDD];

        let token = ExperienceToken::with_data(
            1,
            5,
            state,
            action,
            10.5,
            next_state,
            adna_hash,
        );

        assert!(token.is_valid());
        // Copy values to avoid taking references to packed fields
        let ep_id = token.episode_id;
        let step = token.step_number;
        let rew = token.reward;
        let token_state = token.state;
        let token_action = token.action;
        let token_hash = token.adna_version_hash;
        assert_eq!(ep_id, 1);
        assert_eq!(step, 5);
        assert_eq!(rew, 10.5);
        assert_eq!(token_state, state);
        assert_eq!(token_action, action);
        assert_eq!(token_hash, adna_hash);
    }

    #[test]
    fn test_flags() {
        let mut token = ExperienceToken::new(1, 0);

        assert!(!token.is_done());
        assert!(!token.is_truncated());
        assert!(!token.is_exploration());

        token.mark_terminal();
        assert!(token.is_done());
        assert!(token.has_flag(ExperienceFlags::TERMINAL));

        let mut token2 = ExperienceToken::new(2, 0);
        token2.mark_truncated();
        assert!(token2.is_done());
        assert!(token2.is_truncated());

        let mut token3 = ExperienceToken::new(3, 0);
        token3.set_flag(ExperienceFlags::EXPLORATION);
        assert!(token3.is_exploration());
    }

    #[test]
    fn test_info_flags() {
        let mut token = ExperienceToken::new(1, 0);

        token.set_flag(ExperienceFlags::SUCCESS);
        assert!(token.has_flag(ExperienceFlags::SUCCESS));
        assert!(!token.has_flag(ExperienceFlags::FAILURE));

        token.set_flag(ExperienceFlags::NOVEL);
        assert!(token.has_flag(ExperienceFlags::SUCCESS));
        assert!(token.has_flag(ExperienceFlags::NOVEL));
    }

    #[test]
    fn test_priority() {
        let mut token = ExperienceToken::new(1, 0);
        token.reward = 5.0;

        let base_priority = token.priority();
        assert_eq!(base_priority, 5.0);

        token.mark_high_value();
        assert_eq!(token.priority(), 10.0); // 2x multiplier

        token.clear_flag(ExperienceFlags::HIGH_VALUE);
        token.set_flag(ExperienceFlags::LOW_VALUE);
        assert_eq!(token.priority(), 2.5); // 0.5x multiplier

        token.clear_flag(ExperienceFlags::LOW_VALUE);
        token.set_flag(ExperienceFlags::NOVEL);
        assert_eq!(token.priority(), 7.5); // 1.5x multiplier
    }
}
