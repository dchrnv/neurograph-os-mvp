# Quick Start Guide - v0.33.0 Implementation

**For developers starting work on SignalSystem v1.0 + Bootstrap Library v1.2**

---

## ⚡ Priority Order (Start Here)

### 🔥 Phase 1: SignalSystem (Start First!)

**Why first?** No external dependencies, builds on existing Graph.

#### Step 1: Add Structures (30 min)
```bash
# Edit: src/core_rust/src/graph.rs
# Add NodeActivation, SignalConfig, AccumulationMode
# Add activations field to Graph struct
```

#### Step 2: Basic Spreading (2-3 hours)
```bash
# Implement spreading_activation() method
# Test with simple chain graph: 1→2→3→4
```

#### Step 3: SignalExecutor (1 hour)
```bash
# Create: src/core_rust/src/executors/signal_executor.rs
# Register with ActionController
```

**Test checkpoint:**
```bash
cargo test --lib spreading_activation
```

---

### 📚 Phase 2: Bootstrap Library (After Phase 1)

#### Step 1: Add Dependencies (10 min)
```bash
# Edit: src/core_rust/Cargo.toml
# Add: linfa, fasthash, rayon, ndarray
cargo build --lib  # Verify compilation
```

#### Step 2: Create Module (30 min)
```bash
# Create: src/core_rust/src/bootstrap.rs
# Add basic structures
```

#### Step 3: ID Generation (1 hour)
```bash
# Implement generate_id() with MurmurHash3
# Test determinism
```

#### Step 4: PCA Pipeline (3-4 hours)
```bash
# Implement load_embeddings()
# Implement train_pca()
# Test on small dataset (100 words)
```

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

### SignalSystem ✅
- [ ] spreading_activation() compiles
- [ ] Energy decays correctly
- [ ] Max depth stops propagation
- [ ] Returns ActivationResult

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
