# Quick Start Guide - v0.33.0 Implementation

**For developers starting work on SignalSystem v1.0 + Bootstrap Library v1.2**

---

## ⚡ Priority Order (Start Here)

### 🔥 Phase 1: SignalSystem ✅ COMPLETED

**Status:** ✅ All tasks completed (Jan 23, 2025)

#### ✅ Step 1: Add Structures (DONE)
```bash
# Edit: src/core_rust/src/graph.rs
# ✅ Added NodeActivation, SignalConfig, AccumulationMode
# ✅ Added activations field to Graph struct
# ✅ Added ActivationResult, ActivatedNode
```

#### ✅ Step 2: Basic Spreading (DONE)
```bash
# ✅ Implemented spreading_activation() method
# ✅ Tested with chain graph: 1→2→3→4
# ✅ Added 8 comprehensive tests
```

#### ✅ Step 3: SignalExecutor (DONE)
```bash
# ✅ Created: src/core_rust/src/executors/signal_executor.rs
# ✅ Registered with ActionController
# ✅ Added 3 SignalExecutor tests
```

**Test results:**
```bash
cargo test --lib spreading_activation
# ✅ 13 tests passing (8 graph + 3 executor + 2 utility)
```

**Benchmarks:**
```bash
cargo bench --bench graph_bench spreading_activation
# ✅ 100 nodes: ~14.6µs
# ✅ 1K nodes: ~14.4µs (66x faster than 1ms target!)
# ✅ 5K nodes: ~15.2µs
```

**Commits:**
- `feat: Add SignalSystem v1.0 structures`
- `feat: Implement spreading_activation algorithm`
- `feat: Add SignalExecutor for ActionController`
- `test: Add comprehensive unit tests`
- `perf: Add benchmarks`

---

### 📚 Phase 2: Bootstrap Library v1.2 ✅ COMPLETED

**Status:** ✅ All tasks completed (Jan 24, 2025)

#### ✅ Step 1: Add Dependencies (DONE)
```bash
# Edit: src/core_rust/Cargo.toml
# ✅ Added: linfa, linfa-reduction, ndarray, fasthash, rayon
# ✅ Successfully compiled
```

#### ✅ Step 2-4: Core Implementation (DONE)
```bash
# ✅ Created: src/core_rust/src/bootstrap.rs
# ✅ Implemented BootstrapLibrary + BootstrapConfig
# ✅ Implemented generate_id() with MurmurHash3 (deterministic hashing)
# ✅ Implemented load_embeddings() (GloVe/Word2Vec format)
# ✅ Implemented PCA pipeline: 300D → 3D projection via linfa
```

#### ✅ Step 5-6: Graph Population (DONE)
```bash
# ✅ Implemented populate_graph() - create nodes from concepts
# ✅ Implemented populate_grid() - spatial indexing in L1Physical
# ✅ Implemented weave_connections() - KNN-based semantic links
```

#### ✅ Step 7: Multimodal Anchors (DONE)
```bash
# ✅ Color lexicon: 27 colors with RGB values
# ✅ Emotion lexicon: 30 emotions with VAD (Valence-Arousal-Dominance)
# ✅ Implemented add_color_anchors() and add_emotion_anchors()
# ✅ Implemented enrich_multimodal() - joint enrichment
```

#### ✅ Step 8: Persistence (DONE)
```bash
# ✅ Binary PCA model serialization (save_pca_model, load_pca_model)
# ✅ JSON bootstrap map export (save_bootstrap_map)
# ✅ Artifact versioning for reproducibility
```

#### ✅ Step 9: Semantic Tests (DONE)
```bash
# ✅ test_semantic_similarity_cat_dog_car() - PCA preserves distances
# ✅ test_spreading_activation_on_semantic_graph() - activation on semantic net
# ✅ test_integration_bootstrap_full_pipeline() - end-to-end validation
```

**Test results:**
```bash
cargo test --lib bootstrap
# ✅ 19 tests passing (all bootstrap + integration tests)
```

**Commits:**
- `feat: Add Bootstrap Library v1.2 structures and ID generation`
- `feat: Implement PCA pipeline and embedding loader`
- `feat: Add graph population and KNN connection weaving`
- `feat: Add multimodal anchors (color + emotion lexicons)`
- `feat: Add artifact persistence (PCA model + bootstrap map)`
- `test: Add semantic similarity and spreading activation integration tests`

---

## 🧪 Testing Strategy

### Quick Smoke Tests

**SignalSystem:**
```bash
cd src/core_rust
cargo test --lib test_basic_spreading -- --nocapture
```

**Bootstrap:**
```bash
cargo test --lib test_pca_projection -- --nocapture
```

### Integration Test
```bash
cargo test --test signal_bootstrap_e2e
```

---

## 📂 File Structure

```
src/core_rust/src/
├── graph.rs                    # ADD: NodeActivation, spreading_activation()
├── bootstrap.rs                # NEW: Bootstrap Library
├── executors/
│   └── signal_executor.rs      # NEW: SignalExecutor
└── lib.rs                      # ADD: pub mod bootstrap
```

---

## 🎯 Minimal Working Example

**Goal:** Get spreading activation working with 10 nodes

```rust
// Test: test_minimal_spreading
#[test]
fn test_minimal_spreading() {
    let mut graph = Graph::new();

    // Create simple network: 1→2→3
    graph.add_node(1);
    graph.add_node(2);
    graph.add_node(3);
    graph.add_edge(1, 2, 0.8);
    graph.add_edge(2, 3, 0.6);

    // Activate node 1
    let result = graph.spreading_activation(1, 1.0, None);

    // Should activate nodes 2 and 3
    assert_eq!(result.activated_nodes.len(), 2);
}
```

---

## 🚫 Common Pitfalls

### SignalSystem
1. **Energy never reaches zero** → Check decay_rate < 1.0
2. **Stack overflow** → Check max_depth limit
3. **Infinite loop** → Ensure visited set works

### Bootstrap
1. **PCA fails** → Check embeddings all same dimension
2. **ID collisions** → Verify linear probing logic
3. **OOM on large embeddings** → Load in batches

---

## 📊 Success Criteria (MVP)

### SignalSystem ✅ COMPLETED
- [x] spreading_activation() compiles
- [x] Energy decays correctly
- [x] Max depth stops propagation
- [x] Returns ActivationResult
- [x] 13 tests passing
- [x] Benchmarks: ~14-15µs for 1K-5K nodes

### Bootstrap ✅
- [ ] Loads 100 word embeddings
- [ ] PCA reduces 100D → 3D
- [ ] Creates tokens with IDs
- [ ] Generates connections via Grid KNN

### Integration ✅
- [ ] Activate "fire" → spreads to "heat", "danger"
- [ ] Performance <100ms for 1K words

---

## 🕐 Time Estimates

**Absolute Minimum (Core features only):**
- SignalSystem: ~6 hours
- Bootstrap: ~8 hours
- Integration: ~2 hours
- **Total: ~16 hours (~2 work days)**

**Full Implementation (with tests + docs):**
- SignalSystem: ~12 hours
- Bootstrap: ~15 hours
- Integration: ~4 hours
- Documentation: ~2 hours
- **Total: ~33 hours (~4 work days)**

---

## 🔗 References

- [SignalSystem v1.0 Spec](../arch/SignalSystem v1.0.md)
- [Bootstrap Library v1.2 Spec](../arch/Bootstrap Library v1.2.md)
- [Implementation Plan](IMPLEMENTATION_PLAN_v0.33.0.md)

---

## 💡 Tips

1. **Start small** - Test with 10 nodes before 10K
2. **Use existing code** - Graph already has BFS/DFS to learn from
3. **Print debug** - Use `--nocapture` to see energy values
4. **Incremental** - Get basic working before optimizations

---

**Ready to code? Start with Phase 1, Task 1.1!**

🚀 Let's build the neural dynamics layer!
