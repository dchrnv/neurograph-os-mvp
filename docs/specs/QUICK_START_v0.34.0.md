# Quick Start Guide - v0.34.0 Implementation

**For developers: Target Vector Storage + ADNA Integration + Bootstrap v1.3**

---

## ⚡ Priority Order (Start Here)

### 🎯 Phase 1: Target Vector Storage (Priority 1)

**Status:** 🚧 Planned

**Problem:**
ConnectionV3 currently doesn't store target vectors, so Fast Path actions just mirror input states (not useful). We need to store what action the connection should produce.

**Goals:**
- Add target vector storage to ConnectionV3
- Store targets when creating reflexes
- Extract targets in Fast Path
- Enable proper action generation

**Impact:** ActionController will produce real goal-directed actions instead of copying inputs

---

#### Step 1.1: Extend ConnectionV3 Structure

**File:** `src/core_rust/src/connection_v3.rs`

**Current structure (64 bytes):**
```rust
pub struct ConnectionV3 {
    // Core fields (32 bytes)
    pub token_a_id: u32,
    pub token_b_id: u32,
    pub connection_type: u8,
    pub rigidity: u8,
    pub active_levels: u8,
    pub flags: u8,
    pub activation_count: u32,
    pub pull_strength: f32,
    pub preferred_distance: f32,
    pub created_at: u32,
    pub last_activation: u32,

    // Learning extension (32 bytes)
    pub mutability: u8,
    pub confidence: u8,
    pub evidence_count: u16,
    pub last_update: u32,
    pub learning_rate: u8,
    pub decay_rate: u8,
    pub _padding1: u16,
    pub source_id: u32,
    pub reserved: [u8; 16],  // ← We have 16 bytes available here!
}
```

**Action:**
1. Replace `reserved: [u8; 16]` with meaningful fields
2. Add `target_vector: [i16; 8]` (16 bytes) - stores 8D action target
3. Update size assertion tests

**New structure:**
```rust
pub struct ConnectionV3 {
    // ... existing fields ...

    // Learning extension (32 bytes)
    pub mutability: u8,
    pub confidence: u8,
    pub evidence_count: u16,
    pub last_update: u32,
    pub learning_rate: u8,
    pub decay_rate: u8,
    pub _padding1: u16,
    pub source_id: u32,
    pub target_vector: [i16; 8],  // NEW: 16 bytes for action target (8D compressed)
}
```

**Notes:**
- We use `[i16; 8]` instead of full `[i16; 24]` to fit in 16 bytes
- This stores compressed 8D target (one coordinate per dimension)
- Expansion to full 3D coordinates happens at action execution time

---

#### Step 1.2: Update IntuitionEngine - Store Targets

**File:** `src/core_rust/src/intuition_engine.rs`

**Current code (consolidate_reflex):**
```rust
pub fn consolidate_reflex(
    &mut self,
    pattern_token: &Token,
    outcome_token: &Token,
    success: bool,
) -> Result<(), String> {
    // Creates connection but doesn't store target_vector!
    let conn = ConnectionV3::new(pattern_token.id, outcome_token.id);
    // ...
}
```

**Action:**
1. Extract target from `outcome_token` coordinates
2. Compress to 8D format (take X-axis from each dimension)
3. Store in connection's `target_vector` field

**Updated code:**
```rust
pub fn consolidate_reflex(
    &mut self,
    pattern_token: &Token,
    outcome_token: &Token,
    success: bool,
) -> Result<(), String> {
    let mut conn = ConnectionV3::new(pattern_token.id, outcome_token.id);

    // NEW: Extract and store target vector (8D compressed)
    conn.target_vector = extract_8d_target(outcome_token);

    // Set confidence based on success
    if success {
        conn.confidence = 200;
    } else {
        conn.confidence = 50;
    }

    // ... rest of logic ...
}

// NEW: Helper function
fn extract_8d_target(token: &Token) -> [i16; 8] {
    let mut target = [0i16; 8];
    for i in 0..8 {
        target[i] = token.coordinates[i][0];  // Take X-axis from each dimension
    }
    target
}
```

---

#### Step 1.3: Update ActionController - Use Targets in Fast Path

**File:** `src/core_rust/src/action_controller.rs`

**Current Fast Path code:**
```rust
DecisionSource::FastPath(conn_id) => {
    // Get connection from IntuitionEngine
    if let Some(conn) = self.intuition.get_connection(conn_id) {
        // Currently: just copy input state (wrong!)
        let output_state = input_state.clone();

        return Ok(ActionIntent {
            action_type: ActionType::Move,
            target_state: output_state,
            source: DecisionSource::FastPath(conn_id),
            confidence: conn.confidence,
        });
    }
}
```

**Action:**
1. Extract `target_vector` from connection
2. Expand 8D compressed target to full Token coordinates
3. Use as action target instead of copying input

**Updated code:**
```rust
DecisionSource::FastPath(conn_id) => {
    if let Some(conn) = self.intuition.get_connection(conn_id) {
        // NEW: Expand target_vector to full coordinates
        let target_token = expand_target_to_token(&conn.target_vector);

        // Convert to state format for action
        let target_state = target_token.to_state_f32();

        return Ok(ActionIntent {
            action_type: ActionType::Move,
            target_state,  // Now using real target!
            source: DecisionSource::FastPath(conn_id),
            confidence: conn.confidence,
        });
    }
}

// NEW: Helper function
fn expand_target_to_token(target_8d: &[i16; 8]) -> Token {
    let mut token = Token::default();
    for i in 0..8 {
        token.coordinates[i][0] = target_8d[i];  // Set X-axis
        token.coordinates[i][1] = 0;             // Y = 0
        token.coordinates[i][2] = 0;             // Z = 0
    }
    token
}
```

---

#### Step 1.4: Add Tests

**File:** `src/core_rust/src/action_controller.rs` (test module)

**Tests to add:**

1. **test_target_vector_storage** - Verify ConnectionV3 stores targets
2. **test_fast_path_uses_target** - Verify Fast Path extracts and uses targets
3. **test_consolidate_reflex_with_target** - Verify IntuitionEngine stores targets correctly

**Example test:**
```rust
#[test]
fn test_fast_path_uses_target() {
    let mut controller = ActionController::new(/* ... */);

    // Train reflex: state A → target B
    let state_a = vec![1.0, 0.0, 0.0, ...];
    let target_b = vec![5.0, 3.0, 2.0, ...];

    controller.consolidate_reflex(
        &Token::from_state_f32(&state_a),
        &Token::from_state_f32(&target_b),
        true,
    ).unwrap();

    // Act on state A via Fast Path
    let intent = controller.act(&state_a).unwrap();

    // Should produce target_b, NOT state_a!
    assert_eq!(intent.source, DecisionSource::FastPath(_));
    assert_ne!(intent.target_state, state_a);  // NOT copying input
    assert_close(intent.target_state, target_b);  // Using real target
}
```

---

### 🧠 Phase 2: ADNA Integration (Priority 2)

**Status:** 🚧 Planned

**Problem:**
Slow Path currently uses a stub - returns default ActionPolicy. Need real ADNA policy lookup.

**Goals:**
- Implement async ADNA policy lookup in Slow Path
- Compute policy confidence
- Add shadow mode (parallel Fast+Slow for comparison)

---

#### Step 2.1: ADNA Reader Integration

**File:** `src/core_rust/src/action_controller.rs`

**Current Slow Path stub:**
```rust
DecisionSource::SlowPath => {
    // TODO: Real ADNA integration
    let policy = ActionPolicy::default();

    return Ok(ActionIntent {
        action_type: ActionType::Move,
        target_state: input_state.clone(),
        source: DecisionSource::SlowPath,
        confidence: 128,
    });
}
```

**Action:**
1. Add `adna_reader: Arc<ADNAReader>` to ActionController
2. Implement async ADNA policy lookup
3. Convert policy to ActionIntent
4. Compute confidence from policy strength

**Updated code:**
```rust
DecisionSource::SlowPath => {
    // Query ADNA for policy matching input state
    let input_token = Token::from_state_f32(input_state);
    let policy = self.adna_reader.get_policy_for_state(&input_token).await?;

    // Extract action from policy
    let target_state = policy.target_vector.to_state_f32();

    return Ok(ActionIntent {
        action_type: policy.action_type,
        target_state,
        source: DecisionSource::SlowPath,
        confidence: compute_policy_confidence(&policy),
    });
}

fn compute_policy_confidence(policy: &ActionPolicy) -> u8 {
    // Confidence based on policy strength and activation count
    let base = (policy.strength * 255.0) as u8;
    let evidence_bonus = (policy.activations as f32).min(50.0) as u8;
    (base + evidence_bonus).min(255)
}
```

---

#### Step 2.2: Shadow Mode

**File:** `src/core_rust/src/action_controller.rs`

**Goal:** Run both Fast Path and Slow Path in parallel, compare results for learning.

**Action:**
1. Add `shadow_mode: bool` to ArbiterConfig
2. When enabled, always run both paths
3. Log disagreements for analysis
4. Return Fast Path result (Slow Path for monitoring only)

**New code:**
```rust
pub async fn act_with_shadow(
    &mut self,
    input_state: &[f32],
) -> Result<(ActionIntent, Option<ActionIntent>), String> {
    if !self.config.shadow_mode {
        return Ok((self.act(input_state)?, None));
    }

    // Run both paths in parallel
    let fast_future = self.try_fast_path(input_state);
    let slow_future = self.try_slow_path(input_state);

    let (fast_result, slow_result) = tokio::join!(fast_future, slow_future);

    // Log if they disagree
    if let (Ok(fast), Ok(slow)) = (&fast_result, &slow_result) {
        if vector_distance(&fast.target_state, &slow.target_state) > 1.0 {
            self.stats.shadow_disagreements += 1;
        }
    }

    // Return Fast Path as primary, Slow Path as shadow
    Ok((fast_result?, Some(slow_result?)))
}
```

---

### 📚 Phase 3: Extended Multimodal Anchors + Semantic Search

**Status:** 🚧 Planned (for tomorrow)

**Goal:** Extend Bootstrap Library with more modalities and semantic search capabilities.

**Note:** GloVe file loading moved to Phase 4 (separate feature).

---

#### Step 3.1: Extended Multimodal Anchors

**Goal:** Add more sensory modalities beyond colors and emotions.

**New modalities to add:**
1. **Sounds** (30 basic sounds: "whisper", "bang", "melody", etc.)
2. **Actions** (40 verbs: "run", "jump", "think", "speak", etc.)
3. **Spatial relations** (20 prepositions: "above", "below", "near", etc.)

**Implementation:**
```rust
pub fn add_sound_anchors(&mut self) -> Result<(), String> {
    const SOUNDS: [(&str, f32, f32); 30] = [
        ("whisper", 0.2, -0.5),  // (volume, pitch)
        ("shout", 0.9, 0.3),
        ("melody", 0.5, 0.8),
        // ... 27 more
    ];

    for (sound, volume, pitch) in SOUNDS.iter() {
        let embedding = self.generate_sound_embedding(*volume, *pitch);
        self.load_embeddings(vec![(sound.to_string(), embedding)])?;
    }
    Ok(())
}
```

---

#### Step 3.2: Semantic Search Integration

**Goal:** Use SignalSystem spreading activation for semantic queries.

**Example:**
```rust
impl BootstrapLibrary {
    /// Semantic search using spreading activation
    pub fn semantic_search(
        &mut self,
        query: &str,
        max_results: usize,
    ) -> Result<Vec<(String, f32)>, String> {
        // Get query concept node
        let query_concept = self.get_concept(query)
            .ok_or_else(|| format!("Unknown query: {}", query))?;

        // Run spreading activation
        let result = self.graph.spreading_activation(
            query_concept.id,
            1.0,  // initial energy
            Some(3),  // max depth
        );

        // Convert activated nodes to concept names + scores
        let mut results: Vec<(String, f32)> = result.activated_nodes
            .iter()
            .filter_map(|node| {
                self.concepts.iter()
                    .find(|c| c.id == node.node_id)
                    .map(|c| (c.name.clone(), node.energy))
            })
            .collect();

        // Sort by energy and limit
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        results.truncate(max_results);

        Ok(results)
    }
}
```

---

### 🗂️ Phase 4: GloVe Embeddings Loader (Future Enhancement)

**Status:** 🚧 Deferred

**Goal:** Load real word embeddings from GloVe/Word2Vec files.

---

#### Step 4.1: File Loader Implementation

**File:** `src/core_rust/src/bootstrap.rs`

**Current:** Loads mock embeddings from hardcoded vectors

**Action:**
1. Implement real file loader for GloVe format
2. Support large embedding files (100K+ words)
3. Stream processing to avoid OOM

**New method:**
```rust
impl BootstrapLibrary {
    /// Load embeddings from GloVe file (txt format)
    pub fn load_glove_file(
        &mut self,
        path: &str,
        max_words: usize,
    ) -> Result<(), String> {
        let file = File::open(path)
            .map_err(|e| format!("Failed to open {}: {}", path, e))?;

        let reader = BufReader::new(file);
        let mut count = 0;

        for line in reader.lines() {
            if count >= max_words { break; }

            let line = line.map_err(|e| format!("Read error: {}", e))?;
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() < 2 { continue; }

            let word = parts[0].to_string();
            let embedding: Vec<f32> = parts[1..]
                .iter()
                .filter_map(|s| s.parse().ok())
                .collect();

            if embedding.len() == self.config.embedding_dim {
                self.load_embeddings(vec![(word, embedding)])?;
                count += 1;
            }
        }

        Ok(())
    }
}
```

---

## 🧪 Testing Strategy

### Phase 1: Target Vector Storage ✅ COMPLETED

```bash
cargo test --lib test_target_vector_storage_and_extraction
cargo test --lib action_controller  # All 9 tests including target vector
```

### Phase 2: ADNA Integration ✅ COMPLETED

```bash
cargo test --lib test_shadow_mode_parallel_execution
cargo test --lib test_shadow_disagreement_tracking
cargo test --lib test_improved_confidence_calculation
cargo test --lib action_controller  # All 9 tests
```

### Phase 3: Extended Multimodal Anchors + Semantic Search (Tomorrow)

```bash
# Multimodal tests
cargo test --lib test_add_sound_anchors
cargo test --lib test_add_action_anchors
cargo test --lib test_add_spatial_anchors

# Semantic search tests
cargo test --lib test_semantic_search_spreading
```

### Phase 4: GloVe Loader (Future)

```bash
cargo test --lib test_load_glove_file
cargo test --lib test_stream_large_embeddings
```

---

## 📊 Success Criteria

### Phase 1 ✅ COMPLETED
- [x] ConnectionV3 has `target_vector: [i16; 8]` field
- [x] `set_target_from_token()` method for storing targets
- [x] Fast Path extracts and uses targets (not copying input)
- [x] 1 comprehensive test passing
- [x] Size assertion still correct (64 bytes)

### Phase 2 ✅ COMPLETED
- [x] Slow Path calls real ADNA reader via tokio runtime
- [x] Policy confidence computed from entropy + max weight
- [x] Shadow mode runs both paths in parallel
- [x] Disagreement tracking implemented
- [x] 3 new tests passing (total 9 ActionController tests)

### Phase 3 🚧 Planned (Tomorrow)
- [ ] 3 new modalities added (sounds, actions, spatial)
- [ ] Helper methods for generating embeddings
- [ ] Semantic search via spreading activation
- [ ] 4 new tests passing

### Phase 4 🚧 Deferred (Future)
- [ ] GloVe file loader working (100K+ words)
- [ ] Stream processing for large files
- [ ] 2 new tests passing

---

## 🕐 Time Estimates

**Phase 1: Target Vector Storage** ✅ COMPLETED (~2 hours actual)
- Structure changes: 30 min
- Helper methods: 30 min
- ActionController updates: 30 min
- Tests: 30 min

**Phase 2: ADNA Integration** ✅ COMPLETED (~2 hours actual)
- ADNA reader integration: 45 min
- Confidence computation: 30 min
- Shadow mode: 30 min
- Tests: 15 min

**Phase 3: Extended Multimodal Anchors + Semantic Search** 🚧 Planned for Tomorrow
- Extended modalities (sounds, actions, spatial): ~3 hours
- Helper embedding generators: ~1 hour
- Semantic search implementation: ~2 hours
- Tests: ~1 hour
- **Total: ~7 hours**

**Phase 4: GloVe Loader** 🚧 Deferred to Future
- File loader implementation: ~3 hours
- Stream processing: ~2 hours
- Tests: ~1 hour
- **Total: ~6 hours**

**Completed Today: ~4 hours (Phases 1+2)**
**Remaining Tomorrow: ~7 hours (Phase 3)**
**Future Work: ~6 hours (Phase 4)**

---

## 🔗 References

- [ConnectionV3 Spec](Connection_V3_UNIFIED.md)
- [ActionController v2.0 Spec](ActionController_v2.0.md)
- [ADNA v3.0 Spec](ADNA_v3.0.md)
- [Bootstrap Library v1.2 Spec](../arch/Bootstrap Library v1.2.md)
- [CHANGELOG v0.32.1](CHANGELOG_v0.32.1.md) - Known limitations

---

## 💡 Key Design Decisions

### Why 8D compressed targets instead of full 24D?

**Space constraint:** ConnectionV3 has only 16 bytes available in `reserved` field.

**Options considered:**
1. Full 24D (`[i16; 24]` = 48 bytes) - doesn't fit ❌
2. Compressed 8D (`[i16; 8]` = 16 bytes) - fits perfectly ✅
3. External storage with pointer - adds indirection overhead ❌

**Decision:** Use 8D compression (X-axis only per dimension), expand at execution time.

### Why shadow mode instead of A/B testing?

**Shadow mode benefits:**
- No user impact (Fast Path always wins)
- Collect disagreement metrics
- Validate Slow Path correctness
- Gradual confidence building

**A/B testing drawbacks:**
- Users experience inconsistent behavior
- Requires traffic splitting infrastructure
- Harder to debug issues

---

## 🚫 Common Pitfalls

### Phase 1: Target Vector Storage
1. **Size bloat** → Verify ConnectionV3 still 64 bytes after changes
2. **Endianness** → Use native byte order for i16 arrays
3. **Zero targets** → Validate targets are non-zero before storing

### Phase 2: ADNA Integration
1. **Async deadlocks** → Use tokio::spawn for ADNA calls
2. **Missing policies** → Have fallback when ADNA returns None
3. **Shadow overhead** → Add toggle to disable in production

### Phase 3: Bootstrap v1.3
1. **OOM on large files** → Stream processing, not load-all
2. **Embedding dimension mismatch** → Validate before PCA
3. **UTF-8 errors** → Handle non-ASCII words gracefully

---

**Ready to code? Start with Phase 1, Step 1.1!**

🎯 Let's make actions goal-directed!
