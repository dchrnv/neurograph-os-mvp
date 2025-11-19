# IntuitionEngine v2.2 — Hybrid Learning Integration

**Version:** 2.2.0
**Status:** 🔄 IN PROGRESS (v0.30.2)
**Date:** 2025-11-19
**Prerequisite:** Connection v3.0 (v0.29.5 ✅), ADNA v3.1 ✅

---

## Цель

Интеграция **ADNA behavioral learning** (что работает) с **Connection causal learning** (почему работает) в единую систему.

**До v2.2:**
- ADNA учится действиям через IntuitionEngine (statistical patterns)
- Connections статичны или учатся независимо
- Нет обмена знаниями между системами

**После v2.2:**
- ADNA успехи → повышают confidence каузальных Connections
- Connection patterns → подсказывают ADNA новые действия
- Unified proposal pipeline с координированной валидацией
- Hybrid learning: behavioral + causal взаимно усиливают друг друга

---

## Архитектура интеграции

### 1. Unified Proposal System

```
┌─────────────────────────────────────┐
│    IntuitionEngine v2.2 Hybrid      │
│                                     │
│  ┌──────────────┐  ┌─────────────┐ │
│  │  ADNA        │  │ Connection  │ │
│  │  Behavioral  │  │ Causal      │ │
│  │  Learning    │  │ Learning    │ │
│  └──────┬───────┘  └──────┬──────┘ │
│         │                 │         │
│         └────┬────────┬───┘         │
│              ▼        ▼             │
│     ┌────────────────────────┐     │
│     │  Proposal Router       │     │
│     │  (behavioral/causal)   │     │
│     └──────────┬─────────────┘     │
│                ▼                    │
│     ┌────────────────────────┐     │
│     │  Guardian Validator    │     │
│     │  (CDNA constraints)    │     │
│     └──────────┬─────────────┘     │
│                ▼                    │
│     ┌────────────────────────┐     │
│     │  Proposal Executor     │     │
│     └────────────────────────┘     │
└─────────────────────────────────────┘
```

### 2. Feedback Loops

#### ADNA → Connection Feedback
```rust
// ADNA discovers: Action X works well in state S
// → Update Connection: Token(S) --[Cause]--> Token(X)
//    Increase confidence based on ADNA success_rate

if adna_action_success_rate > 0.7 {
    let causal_proposal = ConnectionProposal::Modify {
        connection_id: find_causal_connection(state, action),
        field: ConnectionField::Confidence,
        old_value: current_confidence,
        new_value: confidence + 0.1, // ADNA evidence
        justification: format!("ADNA validated: {} successes", count),
        evidence_count: adna_sample_count,
    };
}
```

#### Connection → ADNA Hints
```rust
// Connection pattern: Token(A) --[EnabledBy]--> Token(B)
// → Suggest to ADNA: Try action B when A is present

if connection.confidence > 0.8 && connection.connection_type == EnabledBy {
    let adna_hint = ADNAProposal {
        proposed_change: {
            "op": "increase_weight",
            "state": token_a_id,
            "action": token_b_id,
            "justification": "Causal connection confirmed"
        },
        confidence: connection.confidence,
        evidence_count: connection.evidence_count,
    };
}
```

---

## Proposal Types

### Unified Proposal Enum

```rust
pub enum HybridProposal {
    /// Behavioral learning (ADNA policy update)
    Behavioral(ADNAProposal),

    /// Causal learning (Connection update)
    Causal(ConnectionProposal),

    /// Cross-system feedback (ADNA → Connection)
    BehavioralToCausal {
        adna_pattern: IdentifiedPattern,
        target_connection: ConnectionId,
        confidence_boost: f32,
    },

    /// Cross-system hint (Connection → ADNA)
    CausalToBehavioral {
        connection: ConnectionV3,
        suggested_action: u16,
        exploration_weight: f32,
    },
}
```

---

## Implementation Plan

### Phase 1: Proposal Router (1-2 hours)

**File:** `src/core_rust/src/hybrid_learning.rs`

```rust
pub struct ProposalRouter {
    adna: Arc<RwLock<ADNA>>,
    connections: Arc<RwLock<HashMap<u64, ConnectionV3>>>,
    guardian: Arc<Guardian>,
}

impl ProposalRouter {
    /// Route proposal to appropriate system
    pub async fn route_proposal(&self, proposal: HybridProposal)
        -> Result<ProposalOutcome, ProposalError>
    {
        match proposal {
            HybridProposal::Behavioral(p) => {
                self.guardian.validate_adna_proposal(&p)?;
                self.apply_behavioral_proposal(p).await
            }
            HybridProposal::Causal(p) => {
                self.guardian.validate_connection_proposal(&p)?;
                self.apply_causal_proposal(p).await
            }
            HybridProposal::BehavioralToCausal { .. } => {
                self.apply_cross_system_feedback(proposal).await
            }
            HybridProposal::CausalToBehavioral { .. } => {
                self.apply_causal_hint(proposal).await
            }
        }
    }
}
```

**Tests:** 5 unit tests
- Route behavioral proposal correctly
- Route causal proposal correctly
- Validate Guardian integration
- Handle cross-system feedback
- Error handling

---

### Phase 2: ADNA → Connection Feedback (1-2 hours)

**File:** `src/core_rust/src/hybrid_learning.rs` (extend)

```rust
impl ProposalRouter {
    /// Convert ADNA success pattern to Connection confidence update
    pub fn adna_to_connection_feedback(
        &self,
        pattern: &IdentifiedPattern,
    ) -> Option<ConnectionProposal> {
        // Find causal connection matching this pattern
        let state_token = pattern.state_bin_id as u32;
        let action_token = pattern.better_action as u32;

        // Look for: State --[Cause/EnabledBy]--> Action
        let conn_id = self.find_causal_connection(state_token, action_token)?;
        let current_conn = self.connections.read().get(&conn_id)?;

        // Calculate confidence boost based on ADNA evidence
        let confidence_boost = (pattern.confidence * 0.1).min(0.2);
        let new_confidence = (current_conn.confidence as f32 / 255.0 + confidence_boost)
            .min(1.0);

        Some(ConnectionProposal::Modify {
            connection_id: conn_id,
            field: ConnectionField::Confidence,
            old_value: current_conn.confidence as f32 / 255.0,
            new_value: new_confidence,
            justification: format!(
                "ADNA behavioral evidence: {:.2} confidence, {} samples",
                pattern.confidence, pattern.sample_count
            ),
            evidence_count: pattern.sample_count as u16,
        })
    }
}
```

**Tests:** 7 unit tests
- Find causal connection correctly
- Calculate confidence boost
- Respect min/max bounds
- Handle missing connections
- Multi-connection scenarios
- Evidence count tracking
- Justification formatting

---

### Phase 3: Connection → ADNA Hints (1 hour)

**File:** `src/core_rust/src/hybrid_learning.rs` (extend)

```rust
impl ProposalRouter {
    /// Convert high-confidence Connection to ADNA exploration hint
    pub fn connection_to_adna_hint(
        &self,
        connection: &ConnectionV3,
    ) -> Option<ADNAProposal> {
        // Only send hints for:
        // - High confidence (>0.8)
        // - Causal types (Cause, Effect, EnabledBy)
        // - Learnable tier

        if connection.confidence < 204 { // 0.8 * 255
            return None;
        }

        match ConnectionType::from(connection.connection_type) {
            ConnectionType::Cause |
            ConnectionType::EnabledBy |
            ConnectionType::Effect => {
                // Suggest increasing weight for this state-action pair
                Some(ADNAProposal {
                    proposal_id: uuid::Uuid::new_v4(),
                    target_entity_id: format!(
                        "state_{}_action_{}",
                        connection.token_a_id,
                        connection.token_b_id
                    ),
                    proposed_change: serde_json::json!({
                        "op": "increase_weight",
                        "path": "/policy/weights",
                        "state": connection.token_a_id,
                        "action": connection.token_b_id,
                        "delta": connection.confidence as f32 / 255.0 * 0.1,
                    }),
                    confidence: connection.confidence as f32 / 255.0,
                    evidence_count: connection.evidence_count,
                    timestamp: SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                })
            }
            _ => None,
        }
    }
}
```

**Tests:** 6 unit tests
- High confidence connections create hints
- Low confidence ignored
- Only causal types
- Correct weight calculation
- Multiple connections
- JSON format validation

---

### Phase 4: Integration Tests (1 hour)

**File:** `src/core_rust/tests/hybrid_learning_integration_test.rs`

```rust
#[tokio::test]
async fn test_behavioral_to_causal_feedback_loop() {
    // Setup
    let router = setup_test_router();

    // 1. ADNA discovers successful action
    let adna_pattern = IdentifiedPattern {
        state_bin_id: 100,
        better_action: 5,
        worse_action: 3,
        reward_delta: 1.5,
        confidence: 0.85,
        sample_count: 50,
    };

    // 2. Create causal connection (initially low confidence)
    let mut conn = ConnectionV3::new(100, 5);
    conn.set_connection_type(ConnectionType::Cause);
    conn.mutability = ConnectionMutability::Learnable as u8;
    conn.confidence = 128; // 0.5
    router.add_connection(conn);

    // 3. Apply ADNA → Connection feedback
    let proposal = router.adna_to_connection_feedback(&adna_pattern).unwrap();
    router.route_proposal(HybridProposal::Causal(proposal)).await.unwrap();

    // 4. Verify confidence increased
    let updated_conn = router.get_connection(conn_id);
    assert!(updated_conn.confidence > 128);
    assert_eq!(updated_conn.evidence_count, 50);
}

#[tokio::test]
async fn test_causal_to_behavioral_hints() {
    // 1. Create high-confidence causal connection
    let mut conn = ConnectionV3::new(200, 10);
    conn.set_connection_type(ConnectionType::EnabledBy);
    conn.confidence = 220; // 0.86
    conn.evidence_count = 100;

    // 2. Generate ADNA hint
    let hint = router.connection_to_adna_hint(&conn).unwrap();

    // 3. Apply to ADNA
    router.route_proposal(HybridProposal::Behavioral(hint)).await.unwrap();

    // 4. Verify ADNA weight increased
    let adna = router.adna.read();
    let weight = adna.get_action_weight(200, 10);
    assert!(weight > initial_weight);
}

#[tokio::test]
async fn test_e2e_hybrid_learning_cycle() {
    // Full cycle:
    // 1. ADNA tries random action → success
    // 2. IntuitionEngine detects pattern
    // 3. Pattern → Connection confidence boost
    // 4. High confidence Connection → ADNA hint
    // 5. ADNA uses hint → more success
    // 6. Positive feedback loop established
}
```

**Total:** 12 integration tests covering:
- Behavioral → Causal feedback
- Causal → Behavioral hints
- Guardian validation
- Cross-system consistency
- Error propagation
- Performance (throughput)
- E2E learning cycles

---

### Phase 5: Documentation & Cleanup (30 min)

**Updates:**
1. This specification document
2. ROADMAP.md (v0.30.2 complete)
3. API documentation (rustdoc)
4. Usage examples

---

## Performance Targets

| Metric | Target |
|--------|--------|
| Proposal routing | <100 ns |
| ADNA → Connection feedback | <500 ns |
| Connection → ADNA hint | <300 ns |
| E2E hybrid learning cycle | <2 µs |

**Rationale:** Based on v0.30.1 benchmarks:
- Connection proposals: 83 ns
- Learning stats: <85 ns
- Guardian validation: ~6 ns
- Buffer for routing logic: ~500 ns

---

## Success Criteria

✅ **v0.30.2 Complete when:**

1. **ProposalRouter implemented** with full Guardian integration
2. **ADNA → Connection feedback** working (7/7 tests passing)
3. **Connection → ADNA hints** working (6/6 tests passing)
4. **Integration tests** passing (12/12)
5. **Performance targets** met (all <2 µs)
6. **Documentation** updated

---

## Future Extensions (v0.30.3+)

**Batch processing:**
- Process multiple proposals in single Guardian pass
- Vectorized confidence updates

**Confidence calibration:**
- Cross-validate ADNA and Connection confidences
- Bayesian fusion of behavioral + causal evidence

**Temporal coordination:**
- Synchronize ADNA exploration with Connection hypothesis testing
- Shared learning rate scheduling

---

**Version:** 2.2.0
**Date:** 2025-11-19
**Author:** Denis Chernov
**License:** AGPL-3.0
