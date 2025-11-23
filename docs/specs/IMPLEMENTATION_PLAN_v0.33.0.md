# NeuroGraph OS MVP - Implementation Plan v0.33.0

**Version:** 0.33.0
**Date:** 2025-01-13
**Status:** Planning Phase
**Focus:** SignalSystem v1.0 + Bootstrap Library v1.2

---

## 🎯 Overview

v0.33.0 brings two major subsystems that work together:

1. **SignalSystem v1.0** - Neural dynamics (spreading activation) for Graph
2. **Bootstrap Library v1.2** - Semantic crystal initialization from word embeddings

### How They Connect

```
Bootstrap Library v1.2                SignalSystem v1.0
        ↓                                     ↓
   Loads GloVe            →        Graph with connections
   (300D vectors)                           ↓
        ↓                           spreading_activation()
   PCA → 3D Grid                            ↓
        ↓                        "fire" → "heat", "danger", "pain"
   Creates tokens                           ↓
   + connections                    ActionController executes
        ↓
   Semantic Crystal
```

---

## 📋 Implementation Roadmap

### Phase 1: SignalSystem v1.0 Foundation (Priority: HIGH)

**Goal:** Add neural dynamics to Graph

#### Task 1.1: Core Structures
**File:** `src/core_rust/src/graph.rs`
**Estimate:** ~2-3 hours

```rust
// Add to graph.rs

#[derive(Debug, Clone, Default)]
pub struct NodeActivation {
    pub energy: f32,
    pub last_activated: u64,
    pub activation_count: u32,
    pub source_id: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct SignalConfig {
    pub min_energy: f32,         // Default: 0.01
    pub decay_rate: f32,          // Default: 0.2
    pub max_depth: usize,         // Default: 5
    pub activation_threshold: f32, // Default: 0.1
    pub accumulation_mode: AccumulationMode,
}

#[derive(Debug, Clone)]
pub enum AccumulationMode {
    Sum,
    Max,
    WeightedAverage,
}

// Add to Graph struct
pub struct Graph {
    // ... existing fields ...
    activations: HashMap<NodeId, NodeActivation>,
    signal_config: SignalConfig,
}
```

**Tests:**
- ✅ NodeActivation creation and defaults
- ✅ SignalConfig validation

---

#### Task 1.2: Spreading Activation Algorithm
**File:** `src/core_rust/src/graph.rs`
**Estimate:** ~4-5 hours

```rust
impl Graph {
    pub fn spreading_activation(
        &mut self,
        source_id: NodeId,
        initial_energy: f32,
        custom_config: Option<SignalConfig>
    ) -> ActivationResult {
        // BFS with energy decay
        // See SignalSystem v1.0 spec section 3.1
    }

    fn compute_transmitted_energy(
        &self,
        source_energy: f32,
        edge_weight: f32,
        config: &SignalConfig
    ) -> f32 {
        source_energy * edge_weight * (1.0 - config.decay_rate)
    }

    fn activate_node(
        &mut self,
        node_id: NodeId,
        energy: f32,
        source: Option<NodeId>
    ) {
        // Handle accumulation modes
    }
}
```

**Tests:**
- ✅ Basic spreading (chain: 1→2→3→4)
- ✅ Accumulation modes (diamond: 1→2,3→4)
- ✅ Max depth limit
- ✅ Energy decay verification

**Performance Target:**
- Small graph (1K nodes): <100μs
- Medium graph (10K nodes): <1ms

---

#### Task 1.3: SignalExecutor Integration
**File:** `src/core_rust/src/executors/signal_executor.rs` (new)
**Estimate:** ~2 hours

```rust
pub struct SignalExecutor {
    graph: Arc<RwLock<Graph>>,
}

#[async_trait]
impl ActionExecutor for SignalExecutor {
    fn id(&self) -> &str { "signal_executor" }

    fn supported_actions(&self) -> Vec<ActionType> {
        vec![ActionType::ActivateToken, ActionType::PropagateSignal]
    }

    async fn execute(&self, params: [f32; 8]) -> Result<ActionResult, ExecutionError> {
        // Extract params: source_id, energy, decay, max_depth
        // Call spreading_activation()
        // Return result with activated nodes
    }
}
```

**Tests:**
- ✅ Integration with ActionController
- ✅ Parameter extraction from [f32; 8]
- ✅ Result formatting

---

### Phase 2: Bootstrap Library v1.2 (Priority: MEDIUM)

**Goal:** Load semantic knowledge from embeddings

#### Task 2.1: Dependencies & Structures
**File:** `src/core_rust/Cargo.toml`, `src/core_rust/src/bootstrap.rs` (new)
**Estimate:** ~1 hour

**Add to Cargo.toml:**
```toml
[dependencies]
linfa = "0.7"
linfa-reduction = "0.7"  # For PCA
fasthash = "0.4"          # MurmurHash3
rayon = "1.10"            # Parallel iterators
ndarray = "0.15"          # For linfa
```

**Create structures:**
```rust
pub struct BootstrapLibrary {
    config: BootstrapConfig,
    grid: Arc<Grid>,
    pca_model: Option<PcaModel>,
    id_registry: HashSet<u32>,
}

#[derive(Deserialize)]
pub struct BootstrapConfig {
    pub embeddings_path: String,
    pub pca_components: usize,      // 3 for L8 (X, Y, Z)
    pub similarity_radius: f32,
    pub max_neighbors: usize,       // 10
    pub system_seed: u64,
}
```

---

#### Task 2.2: Deterministic ID Generation
**File:** `src/core_rust/src/bootstrap.rs`
**Estimate:** ~1 hour

```rust
fn generate_id(
    namespace: &str,
    value: &str,
    registry: &HashSet<u32>
) -> u32 {
    let base_key = format!("{}:{}", namespace, value);
    let mut hash = fasthash::murmur3::hash32(&base_key);

    // Linear probing for collision resolution
    while registry.contains(&hash) || hash < 1000 {
        hash = hash.wrapping_add(1);
    }
    hash
}
```

**Tests:**
- ✅ Deterministic (same input → same ID)
- ✅ Collision resolution
- ✅ Reserved range (<1000) protection

---

#### Task 2.3: Embeddings Loader
**File:** `src/core_rust/src/bootstrap.rs`
**Estimate:** ~2 hours

```rust
fn load_glove_embeddings(
    path: &str
) -> Result<HashMap<String, Vec<f32>>, BootstrapError> {
    // Parse GloVe format: "word 0.123 0.456 ..."
    // Return HashMap<word, vector>
}
```

**Format Support:**
- GloVe (text format)
- Word2Vec (binary format) - optional

**Tests:**
- ✅ Load small dataset (100 words)
- ✅ Validate vector dimensions
- ✅ Handle malformed lines

---

#### Task 2.4: PCA Projection
**File:** `src/core_rust/src/bootstrap.rs`
**Estimate:** ~3-4 hours

```rust
fn train_pca(
    embeddings: &HashMap<String, Vec<f32>>,
    n_components: usize
) -> Result<PcaModel, BootstrapError> {
    use linfa::traits::Fit;
    use linfa_reduction::Pca;

    // Convert to ndarray::Array2
    // Train PCA model
    // Return serializable model
}

fn project_to_3d(
    vector: &[f32],
    pca_model: &PcaModel
) -> [f32; 3] {
    // Apply PCA transform
    // Normalize to Grid range [-3.27, 3.27]
}
```

**Tests:**
- ✅ PCA reduces dimensionality correctly
- ✅ Projection preserves relative distances
- ✅ Output fits Grid coordinate range

---

#### Task 2.5: Multimodal Anchors
**File:** `src/core_rust/src/bootstrap.rs`
**Estimate:** ~2 hours

```rust
fn create_multimodal_anchor(
    concept: &str,
    l8_coords: [f32; 3],      // From PCA
    color_rgb: Option<[u8; 3]>,
    emotion_vad: Option<[f32; 3]>
) -> Token {
    let mut token = Token::new(generate_id("concept", concept, &registry));

    // L8 Abstract (from PCA)
    token.set_coordinate(CoordinateSpace::L8Abstract, l8_coords);

    // L2 Sensory (color)
    if let Some(rgb) = color_rgb {
        token.set_coordinate(CoordinateSpace::L2Sensory, rgb_to_grid(rgb));
    }

    // L4 Emotional (VAD)
    if let Some(vad) = emotion_vad {
        token.set_coordinate(CoordinateSpace::L4Emotional, vad);
    }

    token
}
```

**Examples:**
- "FIRE" → L8 (chaos, destruction), L2 (orange-red), L4 (high arousal, low valence)
- "WATER" → L8 (flow, calm), L2 (blue), L4 (low arousal, neutral valence)

---

#### Task 2.6: Connection Weaving
**File:** `src/core_rust/src/bootstrap.rs`
**Estimate:** ~2 hours

```rust
pub fn weave_connections(
    grid: &Arc<Grid>,
    tokens: &[Token]
) -> Vec<ConnectionV3> {
    use rayon::prelude::*;

    tokens.par_iter().flat_map(|token| {
        // Use Grid KNN search
        let neighbors = grid.find_k_nearest(
            token.id,
            CoordinateSpace::L8Abstract,
            10  // max_neighbors
        );

        // Create connections based on distance
        neighbors.into_iter().map(|(neighbor_id, dist)| {
            let strength = (1.0 - dist / MAX_DIST).max(0.0);
            ConnectionV3::new_immutable(
                token.id,
                neighbor_id,
                ConnectionType::SimilarTo,
                (strength * 255.0) as u8
            )
        }).collect::<Vec<_>>()
    }).collect()
}
```

**Performance:** O(N log N) instead of O(N²)

---

#### Task 2.7: Artifact Persistence
**File:** `src/core_rust/src/bootstrap.rs`
**Estimate:** ~1 hour

```rust
fn save_artifacts(
    pca_model: &PcaModel,
    word_to_id: &HashMap<String, u32>,
    output_dir: &str
) -> Result<(), BootstrapError> {
    // Save pca_model.bin (serialized PCA)
    // Save bootstrap_map.bin (word → token ID mapping)
}
```

---

### Phase 3: Integration & Testing

#### Task 3.1: End-to-End Test
**File:** `tests/signal_bootstrap_e2e.rs`
**Estimate:** ~2 hours

```rust
#[tokio::test]
async fn test_signal_spreading_on_bootstrap_graph() {
    // 1. Bootstrap load 100 words
    let bootstrap = BootstrapLibrary::new(config);
    let (tokens, connections) = bootstrap.load().await.unwrap();

    // 2. Insert into Graph
    let graph = Arc::new(RwLock::new(Graph::new()));
    graph.write().await.batch_insert_tokens(tokens);
    graph.write().await.batch_insert_connections(connections);

    // 3. Activate "fire"
    let fire_id = bootstrap.word_to_id("fire").unwrap();
    let result = graph.write().await.spreading_activation(
        fire_id,
        1.0,
        None
    );

    // 4. Verify semantic spreading
    assert!(result.activated_nodes.iter()
        .any(|n| bootstrap.id_to_word(n.node_id) == Some("heat")));
    assert!(result.activated_nodes.iter()
        .any(|n| bootstrap.id_to_word(n.node_id) == Some("danger")));
}
```

---

## 📊 Success Metrics

### SignalSystem v1.0
- ✅ All unit tests pass (6+ tests)
- ✅ Performance: <1ms for 10K node graph
- ✅ Integration with ActionController works

### Bootstrap v1.2
- ✅ PCA projection maintains semantic structure
- ✅ "cat" and "dog" closer than "cat" and "car"
- ✅ Connection weaving completes in <10s for 50K words
- ✅ Artifacts saved and loadable

### Integration
- ✅ Spreading activation on bootstrapped graph works
- ✅ Semantic associations propagate correctly
- ✅ Performance acceptable for real-time use

---

## 🚀 Deployment Plan

### v0.33.0 Release Checklist

- [ ] All Phase 1 tasks complete (SignalSystem)
- [ ] All Phase 2 tasks complete (Bootstrap)
- [ ] Integration tests pass
- [ ] Documentation updated
- [ ] CHANGELOG_v0.33.0.md created
- [ ] Performance benchmarks run
- [ ] README.md updated

---

## 📚 Dependencies Summary

**New Rust crates:**
```toml
linfa = "0.7"           # ML framework
linfa-reduction = "0.7" # PCA implementation
fasthash = "0.4"        # Fast hashing (MurmurHash3)
rayon = "1.10"          # Data parallelism
ndarray = "0.15"        # N-dimensional arrays
```

**Data files needed:**
- `data/glove.6B.100d.txt` (~330MB, 400K words) or smaller subset
- Optional: WordNet for hierarchical relations

---

## 🎯 Next Steps (After v0.33.0)

### v0.34.0: Advanced Spreading
- Multi-source activation
- Typed spreading (filter by ConnectionType)
- Wave propagation with delays

### v0.35.0: Bootstrap Extensions
- WordNet hierarchy integration
- Custom domain embeddings
- Real-time word addition

---

**🤖 Generated with [Claude Code](https://claude.com/claude-code)**

**Co-Authored-By:** Claude <noreply@anthropic.com>
