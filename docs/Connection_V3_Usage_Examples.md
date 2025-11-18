# Connection V3.0 - Usage Examples

**Version:** 3.0.0
**Implementation:** [src/core_rust/src/connection_v3.rs](../src/core_rust/src/connection_v3.rs)
**Specification:** [Connection_V3_UNIFIED.md](specs/Connection_V3_UNIFIED.md)

---

## Table of Contents

1. [Basic Usage](#basic-usage)
2. [Three Mutability Tiers](#three-mutability-tiers)
3. [Proposal System](#proposal-system)
4. [Guardian Integration](#guardian-integration)
5. [Learning Algorithms](#learning-algorithms)
6. [Temporal Pattern Detection](#temporal-pattern-detection)
7. [E2E Workflow Examples](#e2e-workflow-examples)

---

## Basic Usage

### Creating Connections

```rust
use neurograph_core::connection_v3::{ConnectionV3, ConnectionType, ConnectionMutability};

// Create basic connection
let mut conn = ConnectionV3::new(token_a_id, token_b_id);

// Set connection type (automatically sets mutability tier)
conn.set_connection_type(ConnectionType::Cause);
// → mutability = Learnable (causal types are learnable)

conn.set_connection_type(ConnectionType::Synonym);
// → mutability = Immutable (semantic types are immutable)

// Manual configuration
conn.pull_strength = 5.0;       // Attraction force
conn.preferred_distance = 10.0;  // Ideal distance in 8D space
conn.confidence = 128;           // 0.5 in 0-255 range
```

---

## Three Mutability Tiers

### 1. Immutable (Ontological Facts)

```rust
// Semantic relationships that never change
let mut conn = ConnectionV3::new(token_dog, token_animal);
conn.set_connection_type(ConnectionType::Hypernym); // IsA relationship
// → mutability = Immutable
// → confidence = 255 (max)

// Attempting to modify will fail
conn.update_confidence(false); // No effect - immutable
assert_eq!(conn.confidence, 255);
```

### 2. Learnable (Causal Hypotheses)

```rust
// Causal relationships refined through experience
let mut conn = ConnectionV3::new(token_action, token_result);
conn.set_connection_type(ConnectionType::Cause);
// → mutability = Learnable
// → learning_rate = 32 (~0.125)
// → decay_rate = 8 (~0.03)

// Updates confidence based on observations
conn.update_confidence(true);   // Success → increase confidence
conn.update_confidence(false);  // Failure → decrease confidence

// Moderate learning and decay rates
conn.activate(); // Reinforces connection
```

### 3. Hypothesis (Experimental Patterns)

```rust
// Fast learning + decay for testing patterns
let mut conn = ConnectionV3::new(token_a, token_b);
conn.set_connection_type(ConnectionType::Cause);
conn.mutability = ConnectionMutability::Hypothesis as u8;
// → learning_rate = 128 (~0.5 - fast learning)
// → decay_rate = 32 (~0.125 - moderate decay)

// Quickly adapts to evidence
for _ in 0..10 {
    conn.update_confidence(true); // Rapid confidence increase
}

// Decays if not reinforced
conn.apply_decay(); // Confidence decreases over time
```

---

## Proposal System

### Modify Proposal

```rust
use neurograph_core::connection_v3::{ConnectionProposal, ConnectionField};

// Adjust confidence based on evidence
let proposal = ConnectionProposal::Modify {
    connection_id: 0,
    field: ConnectionField::Confidence,
    old_value: 0.5,
    new_value: 0.75,
    justification: "20 successful observations".to_string(),
    evidence_count: 20,
};

conn.apply_proposal(&proposal).unwrap();
// → confidence changed from 128 to 191 (0.75 * 255)
```

### Create Proposal

```rust
// Create new connection from proposal
let proposal = ConnectionProposal::Create {
    token_a_id: 100,
    token_b_id: 200,
    connection_type: ConnectionType::Cause as u8,
    initial_strength: 3.0,
    initial_confidence: 128, // 0.5
    justification: "Detected temporal pattern".to_string(),
};

let conn = ConnectionV3::from_proposal(&proposal).unwrap();
// → Created as Hypothesis
// → mutability = Hypothesis
// → learning_rate = 128 (fast learning)
```

### Promote Proposal

```rust
// Promote strong Hypothesis → Learnable
let proposal = ConnectionProposal::Promote {
    connection_id: 0,
    evidence_count: 25,
    justification: "85% success rate over 25 observations".to_string(),
};

conn.apply_proposal(&proposal).unwrap();
// → mutability = Learnable
// → learning_rate = 32 (slower learning)
// → decay_rate = 8 (slower decay)
```

### Delete Proposal

```rust
// Remove weak hypothesis
let proposal = ConnectionProposal::Delete {
    connection_id: 0,
    reason: "Low confidence after 20 observations (15% success)".to_string(),
};

conn.apply_proposal(&proposal).unwrap();
// → Hypothesis deleted (would be removed from collection)
```

---

## Guardian Integration

### Validation Pipeline

```rust
use neurograph_core::connection_v3::guardian_validation;

// 3-step validation for proposals
let proposal = ConnectionProposal::Modify {
    connection_id: 0,
    field: ConnectionField::PullStrength,
    old_value: 5.0,
    new_value: 8.0,
    justification: "Increase attraction".to_string(),
    evidence_count: 15,
};

// 1. Pre-validation (CDNA constraints)
guardian_validation::validate_proposal(&conn, &proposal).unwrap();

// 2. Apply proposal
conn.apply_proposal_with_guardian(&proposal).unwrap();

// 3. Post-validation (final state check)
// → Automatically performed by apply_proposal_with_guardian
```

### CDNA Constraint Enforcement

```rust
// Guardian rejects invalid values
let invalid_proposal = ConnectionProposal::Modify {
    connection_id: 0,
    field: ConnectionField::PullStrength,
    old_value: 5.0,
    new_value: 15.0, // Exceeds CDNA limit (±10.0)
    justification: "Too strong".to_string(),
    evidence_count: 20,
};

let result = conn.apply_proposal_with_guardian(&invalid_proposal);
assert!(matches!(result, Err(ProposalError::GuardianRejected { .. })));
// → Connection unchanged
```

---

## Learning Algorithms

### Statistical Learning

```rust
use neurograph_core::connection_v3::learning_stats::ConnectionLearningStats;

// Track connection performance
let mut stats = ConnectionLearningStats::new();

// Record observations
for _ in 0..20 {
    stats.record_success(); // Successful activation
}
for _ in 0..5 {
    stats.record_failure(); // Failed activation
}

// success_rate = 20 / 25 = 0.8 (80%)
assert_eq!(stats.success_rate, 0.8);
```

### Automatic Confidence Adjustment

```rust
// Generate proposal based on statistics
let proposal = stats.generate_confidence_proposal(&conn, 20);

if let Some(proposal) = proposal {
    // Minimum 20 observations required
    // ±10% success rate difference triggers update
    conn.apply_proposal_with_guardian(&proposal).unwrap();
}
```

### Promotion Based on Performance

```rust
// Promote strong hypotheses automatically
let proposal = stats.generate_promote_proposal(&conn, 20, 0.8);

if let Some(proposal) = proposal {
    // Requires: 20+ observations, 80%+ success rate
    conn.apply_proposal_with_guardian(&proposal).unwrap();
    // → Hypothesis promoted to Learnable
}
```

### Deletion of Weak Hypotheses

```rust
// Remove poorly performing connections
let proposal = stats.generate_delete_proposal(&conn, 20, 0.3);

if let Some(proposal) = proposal {
    // Requires: 20+ observations, <30% success rate
    conn.apply_proposal_with_guardian(&proposal).unwrap();
    // → Hypothesis marked for deletion
}
```

---

## Temporal Pattern Detection

### Co-occurrence Analysis

```rust
use neurograph_core::connection_v3::learning_stats::detect_temporal_pattern;

// Observations: (token_a_id, token_b_id, time_delta_ms)
let observations = vec![
    (100, 200, 100),  // A followed by B after 100ms
    (100, 200, 120),
    (100, 200, 90),
    (100, 200, 110),
    (100, 200, 105),
];

// Detect pattern (minimum 5 co-occurrences)
let pattern = detect_temporal_pattern(100, 200, &observations, 5).unwrap();

assert_eq!(pattern.cooccurrence_count, 5);
assert_eq!(pattern.avg_time_delta_ms, 105); // Average time delta
assert!(pattern.confidence > 0.8); // High confidence (5/5 observations)
```

### Automatic Connection Creation

```rust
use neurograph_core::connection_v3::learning_stats::TemporalPattern;

// Pattern generates creation proposal
let create_proposal = pattern.generate_create_proposal().unwrap();

// Create connection with Guardian validation
let conn = ConnectionV3::from_proposal_with_guardian(&create_proposal).unwrap();

// → Created as Hypothesis
// → connection_type = After (temporal sequence)
// → confidence = pattern.confidence * 255
// → pull_strength = 1.0 + (confidence * 2.0)
```

---

## E2E Workflow Examples

### Example 1: Complete Learning Cycle

```rust
// 1. Detect temporal pattern
let observations = vec![/* ... */];
let pattern = detect_temporal_pattern(100, 200, &observations, 5).unwrap();

// 2. Create Hypothesis connection
let create_proposal = pattern.generate_create_proposal().unwrap();
let mut conn = ConnectionV3::from_proposal_with_guardian(&create_proposal).unwrap();

// 3. Track performance
let mut stats = ConnectionLearningStats::new();
for _ in 0..25 {
    stats.record_success(); // 100% success rate
}

// 4. Adjust confidence based on evidence
let conf_proposal = stats.generate_confidence_proposal(&conn, 20).unwrap();
conn.apply_proposal_with_guardian(&conf_proposal).unwrap();

// 5. Promote to Learnable
let promote_proposal = stats.generate_promote_proposal(&conn, 20, 0.8).unwrap();
conn.apply_proposal_with_guardian(&promote_proposal).unwrap();
// → Now a stable Learnable connection
```

### Example 2: Failed Hypothesis Cleanup

```rust
// 1. Create hypothesis
let mut conn = ConnectionV3::new(100, 200);
conn.set_connection_type(ConnectionType::Cause);
conn.mutability = ConnectionMutability::Hypothesis as u8;

// 2. Poor performance
let mut stats = ConnectionLearningStats::new();
for _ in 0..5 {
    stats.record_success();
}
for _ in 0..20 {
    stats.record_failure(); // 20% success rate
}

// 3. Generate deletion proposal
let delete_proposal = stats.generate_delete_proposal(&conn, 20, 0.3).unwrap();

// 4. Remove weak hypothesis
conn.apply_proposal_with_guardian(&delete_proposal).unwrap();
// → Connection marked for deletion
```

### Example 3: Batch Processing (IntuitionEngine Simulation)

```rust
// Multiple connections with different outcomes
let mut connections = vec![
    create_hypothesis(1, 2, ConnectionType::Cause),
    create_hypothesis(3, 4, ConnectionType::EnabledBy),
    create_hypothesis(5, 6, ConnectionType::After),
];

let mut stats_map = vec![
    ConnectionLearningStats::new(),
    ConnectionLearningStats::new(),
    ConnectionLearningStats::new(),
];

// Simulate different performance
stats_map[0].record_many_successes(25); // Excellent → promote
stats_map[1].record_mixed(15, 10);      // Moderate → adjust confidence
stats_map[2].record_many_failures(20);  // Poor → delete

// Process batch
for (i, conn) in connections.iter_mut().enumerate() {
    if let Some(proposal) = stats_map[i].generate_best_proposal(conn) {
        conn.apply_proposal_with_guardian(&proposal).unwrap();
    }
}
// → Conn 0: Promoted, Conn 1: Confidence adjusted, Conn 2: Deleted
```

---

## Migration from v1.0

```rust
// Old v1.0 Connection (32 bytes)
struct ConnectionV1 {
    token_a_id: u32,
    token_b_id: u32,
    connection_type: u8,
    rigidity: u8,
    // ... 24 bytes total
}

// New v3.0 Connection (64 bytes)
let mut conn_v3 = ConnectionV3::new(old_conn.token_a_id, old_conn.token_b_id);
conn_v3.connection_type = old_conn.connection_type;
conn_v3.rigidity = old_conn.rigidity;

// New learning fields automatically initialized:
// - mutability: Determined by connection_type
// - confidence: 128 (0.5)
// - learning_rate: 32-128 (depends on mutability)
// - decay_rate: 8-32 (depends on mutability)
```

---

## Performance Considerations

### Memory Layout

- **Size:** 64 bytes (cache-line aligned)
- **Core fields:** 32 bytes (v1.0 compatible)
- **Learning extension:** 32 bytes (v3.0 new)

### Batch Operations

```rust
// Efficient batch proposal application
let proposals = vec![/* ... */];
for proposal in proposals {
    if let Ok(_) = conn.apply_proposal_with_guardian(&proposal) {
        // Successfully applied
    }
}
```

### Guardian Overhead

- **Pre-validation:** ~10-50 ns (constraint checks)
- **Post-validation:** ~10-30 ns (state verification)
- **Total overhead:** ~50-100 ns per proposal

---

## See Also

- [Connection_V3_UNIFIED.md](specs/Connection_V3_UNIFIED.md) - Full specification
- [IntuitionEngine_v2.2.md](specs/IntuitionEngine_v2.2.md) - Learning system integration
- [connection_v3.rs](../src/core_rust/src/connection_v3.rs) - Implementation (2314 lines, 53 tests)
