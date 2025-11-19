// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2025 Chernov Denys

//! Integration Tests - Unified Entry Point
//!
//! This module provides a centralized entry point for all integration tests.
//! Each test module focuses on end-to-end scenarios for specific subsystems.
//!
//! ## Test Organization
//!
//! - `action_controller_e2e` - Action execution and control loop tests
//! - `learning_loop_e2e` - Full ADNA learning cycle tests
//! - `persistence_e2e` - Database persistence integration tests
//! - `hybrid_learning_e2e` - ADNA ↔ Connection feedback loop tests
//!
//! ## Running Tests
//!
//! ```bash
//! # Run all integration tests
//! cargo test --tests
//!
//! # Run specific test module
//! cargo test --test integration::hybrid_learning_e2e
//!
//! # Run with output
//! cargo test --tests -- --nocapture
//! ```

// Integration test modules
mod integration {
    pub mod action_controller_e2e;
    pub mod learning_loop_e2e;
    pub mod persistence_e2e;
    pub mod hybrid_learning_e2e;
    pub mod intuition_v3_e2e;
}

// Common test utilities
#[cfg(test)]
pub mod common {
    use std::sync::Arc;
    use neurograph_core::{Guardian, CDNA};

    /// Setup test Guardian instance
    pub fn setup_guardian() -> Arc<Guardian> {
        Arc::new(Guardian::new())
    }

    /// Setup test CDNA instance
    pub fn setup_cdna() -> CDNA {
        CDNA::default()
    }

    /// Common test constants
    pub const TEST_TIMEOUT_MS: u64 = 5000;
    pub const TEST_ITERATIONS: usize = 100;
}
