// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2025 Chernov Denys

//! Integration tests for Hybrid Learning System (IntuitionEngine v2.2)
//!
//! Tests the full feedback loop between ADNA behavioral learning
//! and Connection causal learning.

use std::sync::Arc;
use neurograph_core::{
    ProposalRouter, HybridProposal, ProposalOutcome,
    IdentifiedPattern,
    ConnectionV3, ConnectionMutability,
    Guardian,
    adna_to_connection_feedback, connection_to_adna_hint,
};
use neurograph_core::connection_v3::ConnectionType;

// ============================================================================
// Test Setup
// ============================================================================

fn setup_router() -> ProposalRouter {
    let guardian = Arc::new(Guardian::new());
    ProposalRouter::new(guardian)
}

fn create_test_connection(token_a: u32, token_b: u32, confidence: u8) -> ConnectionV3 {
    let mut conn = ConnectionV3::new(token_a, token_b);
    conn.set_connection_type(ConnectionType::Cause);
    conn.mutability = ConnectionMutability::Learnable as u8;
    conn.confidence = confidence;
    conn
}

fn create_test_pattern(state: u64, action: u16, confidence: f64, samples: usize) -> IdentifiedPattern {
    IdentifiedPattern {
        state_bin_id: state,
        better_action: action,
        worse_action: action - 1,
        reward_delta: 1.5,
        confidence,
        sample_count: samples,
    }
}

// ============================================================================
// Test 1: Behavioral → Causal Feedback
// ============================================================================

#[test]
fn test_behavioral_to_causal_feedback_loop() {
    let router = setup_router();

    // 1. Create causal connection (initially low confidence)
    let conn = create_test_connection(100, 5, 128); // 0.5 confidence
    router.add_connection(1, conn);

    // 2. ADNA discovers successful action pattern
    let adna_pattern = create_test_pattern(100, 5, 0.85, 50);

    // 3. Generate feedback proposal
    let proposal = adna_to_connection_feedback(&adna_pattern, 1);

    // 4. Apply feedback
    let result = router.route_proposal(proposal);
    assert!(result.is_ok());

    // 5. Verify confidence increased
    let updated_conn = router.get_connection(1).unwrap();
    assert!(updated_conn.confidence > 128, "Confidence should have increased");
    assert_eq!(updated_conn.evidence_count, 50, "Evidence count should match pattern samples");
}

#[test]
fn test_behavioral_to_causal_confidence_boost_calculation() {
    let router = setup_router();

    // Initial connection with 50% confidence
    let conn = create_test_connection(100, 200, 128);
    router.add_connection(1, conn);

    // High confidence ADNA pattern (0.9)
    let pattern = create_test_pattern(100, 200, 0.9, 30);
    let proposal = adna_to_connection_feedback(&pattern, 1);

    router.route_proposal(proposal).unwrap();

    let updated = router.get_connection(1).unwrap();
    // 0.9 * 0.1 = 0.09, min(0.09, 0.2) = 0.09
    // New confidence: 0.5 + 0.09 = 0.59 → 150
    assert!(updated.confidence >= 145 && updated.confidence <= 155,
            "Confidence boost should be ~0.09: got {}", updated.confidence);
}

#[test]
fn test_behavioral_to_causal_rejects_immutable() {
    let router = setup_router();

    // Create IMMUTABLE connection
    let mut conn = ConnectionV3::new(100, 200);
    conn.set_connection_type(ConnectionType::Synonym); // Semantic = Immutable
    conn.mutability = ConnectionMutability::Immutable as u8;
    conn.confidence = 255; // Max
    router.add_connection(1, conn);

    // Try to apply feedback
    let pattern = create_test_pattern(100, 200, 0.8, 20);
    let proposal = adna_to_connection_feedback(&pattern, 1);

    let result = router.route_proposal(proposal);
    assert!(result.is_err(), "Should reject feedback to Immutable connection");
}

// ============================================================================
// Test 2: Causal → Behavioral Hints
// ============================================================================

#[test]
fn test_causal_to_behavioral_hint_generation() {
    let router = setup_router();

    // Create high-confidence causal connection
    let mut conn = ConnectionV3::new(200, 10);
    conn.set_connection_type(ConnectionType::EnabledBy);
    conn.confidence = 220; // 0.86
    conn.evidence_count = 100;
    router.add_connection(1, conn);

    // Generate hint
    let hint = connection_to_adna_hint(&conn, 1);
    assert!(hint.is_some(), "Should generate hint for high-confidence causal connection");

    // Apply hint
    let result = router.route_proposal(hint.unwrap());
    assert!(result.is_ok());

    // Verify hint was sent
    let stats = router.get_stats();
    assert_eq!(stats.hints_sent, 1);
}

#[test]
fn test_causal_to_behavioral_hint_threshold() {
    // Low confidence should NOT generate hint
    let mut conn_low = ConnectionV3::new(100, 200);
    conn_low.set_connection_type(ConnectionType::Cause);
    conn_low.confidence = 128; // 0.5 < 0.8 threshold

    let hint_low = connection_to_adna_hint(&conn_low, 1);
    assert!(hint_low.is_none(), "Low confidence should not generate hint");

    // High confidence SHOULD generate hint
    let mut conn_high = ConnectionV3::new(100, 200);
    conn_high.set_connection_type(ConnectionType::Cause);
    conn_high.confidence = 210; // 0.82 > 0.8 threshold

    let hint_high = connection_to_adna_hint(&conn_high, 1);
    assert!(hint_high.is_some(), "High confidence should generate hint");
}

#[test]
fn test_causal_to_behavioral_only_causal_types() {
    // Causal type → should generate hint
    let mut conn_causal = ConnectionV3::new(100, 200);
    conn_causal.set_connection_type(ConnectionType::Cause);
    conn_causal.confidence = 220;

    let hint_causal = connection_to_adna_hint(&conn_causal, 1);
    assert!(hint_causal.is_some(), "Causal type should generate hint");

    // Semantic type → should NOT generate hint
    let mut conn_semantic = ConnectionV3::new(100, 200);
    conn_semantic.set_connection_type(ConnectionType::Synonym);
    conn_semantic.confidence = 220;

    let hint_semantic = connection_to_adna_hint(&conn_semantic, 1);
    assert!(hint_semantic.is_none(), "Semantic type should not generate hint");
}

// ============================================================================
// Test 3: Statistics Tracking
// ============================================================================

#[test]
fn test_statistics_tracking() {
    let router = setup_router();

    let stats_initial = router.get_stats();
    assert_eq!(stats_initial.total_proposals, 0);
    assert_eq!(stats_initial.feedbacks_applied, 0);
    assert_eq!(stats_initial.hints_sent, 0);

    // Apply feedback
    let conn = create_test_connection(100, 200, 128);
    router.add_connection(1, conn);
    let pattern = create_test_pattern(100, 200, 0.8, 20);
    let feedback = adna_to_connection_feedback(&pattern, 1);
    router.route_proposal(feedback).unwrap();

    let stats_after_feedback = router.get_stats();
    assert_eq!(stats_after_feedback.total_proposals, 1);
    assert_eq!(stats_after_feedback.feedbacks_applied, 1);

    // Send hint
    let mut conn2 = ConnectionV3::new(200, 300);
    conn2.set_connection_type(ConnectionType::Cause);
    conn2.confidence = 220;
    let hint = connection_to_adna_hint(&conn2, 2).unwrap();
    router.route_proposal(hint).unwrap();

    let stats_final = router.get_stats();
    assert_eq!(stats_final.total_proposals, 2);
    assert_eq!(stats_final.hints_sent, 1);
}

// ============================================================================
// Test 4: Error Handling
// ============================================================================

#[test]
fn test_connection_not_found() {
    let router = setup_router();

    // Try to apply feedback to non-existent connection
    let pattern = create_test_pattern(100, 200, 0.8, 20);
    let proposal = adna_to_connection_feedback(&pattern, 999); // ID doesn't exist

    let result = router.route_proposal(proposal);
    assert!(result.is_err(), "Should fail for non-existent connection");
}

// ============================================================================
// Test 5: E2E Learning Cycle
// ============================================================================

#[test]
fn test_e2e_hybrid_learning_cycle() {
    let router = setup_router();

    // 1. Start with low-confidence learnable connection
    let mut conn = ConnectionV3::new(100, 5);
    conn.set_connection_type(ConnectionType::Cause);
    conn.mutability = ConnectionMutability::Learnable as u8;
    conn.confidence = 64; // 0.25
    conn.learning_rate = 32; // Moderate learning rate
    router.add_connection(1, conn);

    // 2. ADNA discovers this action works well
    let pattern1 = create_test_pattern(100, 5, 0.7, 15);
    router.route_proposal(adna_to_connection_feedback(&pattern1, 1)).unwrap();

    let conn_after_1 = router.get_connection(1).unwrap();
    let conf_after_1 = conn_after_1.confidence;

    // 3. More ADNA evidence
    let pattern2 = create_test_pattern(100, 5, 0.85, 25);
    router.route_proposal(adna_to_connection_feedback(&pattern2, 1)).unwrap();

    let conn_after_2 = router.get_connection(1).unwrap();
    assert!(conn_after_2.confidence > conf_after_1, "Confidence should keep increasing");
    assert_eq!(conn_after_2.evidence_count, 40, "Evidence should accumulate: 15 + 25");

    // 4. Connection becomes high-confidence, sends hint back to ADNA
    // (This would normally trigger ADNA to increase exploration of this action)
    if conn_after_2.confidence >= 204 {
        let hint = connection_to_adna_hint(&conn_after_2, 1);
        assert!(hint.is_some(), "High-confidence connection should generate ADNA hint");
        router.route_proposal(hint.unwrap()).unwrap();
    }

    // Verify full cycle statistics
    let stats = router.get_stats();
    assert_eq!(stats.feedbacks_applied, 2, "Should have applied 2 feedbacks");
    assert!(stats.total_proposals >= 2, "Should have processed at least 2 proposals");
}

#[test]
fn test_multiple_connections_same_pattern() {
    let router = setup_router();

    // Create 3 connections that could match the same ADNA pattern
    for i in 1..=3 {
        let conn = create_test_connection(100, i as u32, 100);
        router.add_connection(i, conn);
    }

    // Apply feedback to each
    let pattern1 = create_test_pattern(100, 1, 0.8, 20);
    let pattern2 = create_test_pattern(100, 2, 0.6, 10);
    let pattern3 = create_test_pattern(100, 3, 0.9, 30);

    router.route_proposal(adna_to_connection_feedback(&pattern1, 1)).unwrap();
    router.route_proposal(adna_to_connection_feedback(&pattern2, 2)).unwrap();
    router.route_proposal(adna_to_connection_feedback(&pattern3, 3)).unwrap();

    // Verify each updated correctly
    let conn1 = router.get_connection(1).unwrap();
    let conn2 = router.get_connection(2).unwrap();
    let conn3 = router.get_connection(3).unwrap();

    // conn3 should have highest confidence (pattern3 had 0.9 confidence)
    assert!(conn3.confidence > conn1.confidence);
    assert!(conn3.confidence > conn2.confidence);

    let stats = router.get_stats();
    assert_eq!(stats.feedbacks_applied, 3);
}
