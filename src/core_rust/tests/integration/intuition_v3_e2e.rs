// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2025 Chernov Denys

//! IntuitionEngine v3.0 End-to-End Integration Tests
//!
//! Tests the complete reflex learning cycle:
//! 1. Unknown situation → Slow Path (ADNA)
//! 2. Experience consolidation → Reflex creation
//! 3. Known situation → Fast Path (Reflex hit)
//! 4. Performance verification

use neurograph_core::token::Token;
use neurograph_core::reflex_layer::{ShiftConfig, AssociativeMemory, compute_grid_hash};
use neurograph_core::connection_v3::{ConnectionV3, ConnectionMutability};

#[test]
fn test_reflex_learning_cycle() {
    // Setup: Create shift config and memory
    let shift_config = ShiftConfig::default();
    let memory = AssociativeMemory::new();

    // Step 1: Unknown situation (new state)
    let mut state = Token::new(1000);
    state.coordinates[0] = [100, 200, 300];  // Physical location
    state.coordinates[4] = [500, 600, 700];  // Cognitive state

    let hash = compute_grid_hash(&state, &shift_config);

    // Step 2: No reflex exists yet (miss)
    assert!(memory.lookup(hash).is_none(), "New situation should have no reflex");

    // Step 3: Simulate ADNA decision → Create Connection
    let mut connection = ConnectionV3::new(1000, 2000);
    connection.mutability = ConnectionMutability::Learnable as u8;
    connection.confidence = 128;  // 0.5 (medium confidence)

    let conn_id = 42u64;  // Simulated connection ID

    // Step 4: Consolidate reflex (Experience → AssociativeMemory)
    memory.insert(hash, conn_id);

    // Step 5: Same situation again → Fast Path hit
    let candidates = memory.lookup(hash);
    assert!(candidates.is_some(), "Known situation should hit reflex");

    let candidates = candidates.unwrap();
    assert_eq!(candidates.len(), 1, "Should have exactly one reflex");
    assert_eq!(candidates[0], conn_id, "Should return correct connection ID");

    // Step 6: Verify stats
    let stats = memory.stats();
    assert_eq!(stats.total_lookups, 2, "Should have 2 lookups");
    assert_eq!(stats.hits, 1, "Should have 1 hit");
    assert_eq!(stats.misses, 1, "Should have 1 miss");
    assert_eq!(stats.total_entries, 1, "Should have 1 reflex");
}

#[test]
fn test_collision_resolution() {
    // Test multiple reflexes with same hash (collision)
    let shift_config = ShiftConfig::default();
    let memory = AssociativeMemory::new();

    let mut state1 = Token::new(100);
    state1.coordinates[0] = [1000, 2000, 3000];

    // Create collision: two different connections with same grid hash
    // (This happens when states are close but not identical)
    let hash = compute_grid_hash(&state1, &shift_config);

    memory.insert(hash, 100);  // First reflex
    memory.insert(hash, 200);  // Second reflex (collision)
    memory.insert(hash, 300);  // Third reflex

    // Lookup should return all candidates
    let candidates = memory.lookup(hash).expect("Should find reflexes");
    assert_eq!(candidates.len(), 3, "Should have 3 candidates");

    // Verify all candidates present
    assert!(candidates.contains(&100));
    assert!(candidates.contains(&200));
    assert!(candidates.contains(&300));

    // Stats should track collision
    let stats = memory.stats();
    assert_eq!(stats.collisions, 1, "Should track collision");
}

#[test]
fn test_spatial_locality() {
    // Test that nearby states produce same hash (spatial locality)
    let shift_config = ShiftConfig::uniform(4);  // shift=4 means 16-unit grid cells

    let mut state_a = Token::new(1);
    state_a.coordinates[0] = [1000, 2000, 3000];

    // State B is 5 units away (within same grid cell if shift=4)
    let mut state_b = Token::new(2);
    state_b.coordinates[0] = [1005, 2005, 3005];

    let hash_a = compute_grid_hash(&state_a, &shift_config);
    let hash_b = compute_grid_hash(&state_b, &shift_config);

    assert_eq!(hash_a, hash_b, "Nearby states should produce same hash (spatial locality)");

    // State C is 20 units away (different grid cell)
    let mut state_c = Token::new(3);
    state_c.coordinates[0] = [1020, 2020, 3020];

    let hash_c = compute_grid_hash(&state_c, &shift_config);
    assert_ne!(hash_a, hash_c, "Far states should produce different hash");
}

#[test]
fn test_adaptive_shift_configuration() {
    // Test that different shift configs produce different granularity
    let mut state = Token::new(100);
    state.coordinates[0] = [1000, 2000, 3000];

    // Fine-grained (shift=2): small grid cells, many unique hashes
    let config_fine = ShiftConfig::uniform(2);
    let hash_fine = compute_grid_hash(&state, &config_fine);

    // Coarse-grained (shift=8): large grid cells, fewer unique hashes
    let config_coarse = ShiftConfig::uniform(8);
    let hash_coarse = compute_grid_hash(&state, &config_coarse);

    // Hashes should differ (different quantization)
    assert_ne!(hash_fine, hash_coarse, "Different shift configs should produce different hashes");

    // Test per-dimension configuration
    let mut config_mixed = ShiftConfig::default();
    config_mixed.set_shift_for_dimension(0, 4);  // Physical: fine
    config_mixed.set_shift_for_dimension(7, 8);  // Abstract: coarse

    let hash_mixed = compute_grid_hash(&state, &config_mixed);
    assert_ne!(hash_mixed, hash_fine);
    assert_ne!(hash_mixed, hash_coarse);
}

#[test]
fn test_reflex_performance_target() {
    // Verify Fast Path meets performance targets
    let shift_config = ShiftConfig::default();
    let memory = AssociativeMemory::new();

    let mut state = Token::new(999);
    state.coordinates[0] = [7000, 8000, 9000];

    let hash = compute_grid_hash(&state, &shift_config);
    memory.insert(hash, 999);

    // Measure Fast Path latency (hash + lookup)
    let start = std::time::Instant::now();

    for _ in 0..10_000 {
        let h = compute_grid_hash(&state, &shift_config);
        let _candidates = memory.lookup(h);
    }

    let elapsed = start.elapsed();
    let avg_ns = elapsed.as_nanos() / 10_000;

    // Target: <100ns for Fast Path (hash + lookup) in release mode
    // Note: This is a rough check, use benchmarks for precise measurement
    // In debug mode (~2000ns is normal, release mode achieves ~70ns)
    println!("Average Fast Path latency: {} ns", avg_ns);
    assert!(avg_ns < 5000, "Fast Path should be reasonable even in debug mode (measured: {} ns)", avg_ns);
}

#[test]
fn test_memory_growth() {
    // Test that memory grows correctly as reflexes are added
    let memory = AssociativeMemory::new();

    assert_eq!(memory.stats().total_entries, 0);

    // Add 100 unique reflexes
    for i in 0..100 {
        memory.insert(i as u64, i as u64);
    }

    assert_eq!(memory.stats().total_entries, 100);

    // Add duplicate (same hash)
    memory.insert(50, 999);
    assert_eq!(memory.stats().total_entries, 100, "Duplicate hash should not increase memory count");

    // Verify lookup returns both candidates for hash=50
    let candidates = memory.lookup(50).unwrap();
    assert_eq!(candidates.len(), 2);
    assert!(candidates.contains(&50));
    assert!(candidates.contains(&999));
}

#[test]
fn test_multidimensional_hash() {
    // Test that different dimensions contribute to hash
    let shift_config = ShiftConfig::default();

    let mut state_base = Token::new(1);
    for i in 0..8 {
        state_base.coordinates[i] = [0, 0, 0];
    }
    let hash_base = compute_grid_hash(&state_base, &shift_config);

    // Change only L1 (Physical) - should change hash
    let mut state_l1 = state_base.clone();
    state_l1.coordinates[0] = [1000, 0, 0];
    let hash_l1 = compute_grid_hash(&state_l1, &shift_config);
    assert_ne!(hash_base, hash_l1, "L1 Physical change should affect hash");

    // Change only L5 (Cognitive) - should change hash
    let mut state_l5 = state_base.clone();
    state_l5.coordinates[4] = [0, 1000, 0];
    let hash_l5 = compute_grid_hash(&state_l5, &shift_config);
    assert_ne!(hash_base, hash_l5, "L5 Cognitive change should affect hash");

    // Change only L8 (Abstract) - should change hash
    let mut state_l8 = state_base.clone();
    state_l8.coordinates[7] = [0, 0, 1000];
    let hash_l8 = compute_grid_hash(&state_l8, &shift_config);
    assert_ne!(hash_base, hash_l8, "L8 Abstract change should affect hash");

    // All hashes should be unique
    assert_ne!(hash_l1, hash_l5);
    assert_ne!(hash_l1, hash_l8);
    assert_ne!(hash_l5, hash_l8);
}

#[test]
fn test_concurrent_access() {
    // Test lock-free concurrent access to AssociativeMemory
    use std::sync::Arc;
    use std::thread;

    let memory = Arc::new(AssociativeMemory::new());

    // Spawn 4 threads inserting reflexes concurrently
    let mut handles = vec![];
    for thread_id in 0..4 {
        let mem = Arc::clone(&memory);
        let handle = thread::spawn(move || {
            for i in 0..100 {
                let hash = (thread_id * 1000 + i) as u64;
                mem.insert(hash, hash);
            }
        });
        handles.push(handle);
    }

    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all reflexes inserted
    let stats = memory.stats();
    assert_eq!(stats.total_entries, 400, "Should have 400 reflexes from 4 threads");
}
