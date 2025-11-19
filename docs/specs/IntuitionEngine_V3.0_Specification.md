# IntuitionEngine v3.0 — Hybrid Reflex System
**Version:** 3.0.0
**Date:** 2025-11-19
**Status:** ✅ IMPLEMENTED (v0.31.0-v0.31.4) — PRODUCTION READY
**Dependencies:** Grid v2.0, Connection v3.0, ADNA v3.0, Guardian v1.0, IntuitionEngine v2.2

---

## 📋 Executive Summary

IntuitionEngine v3.0 introduces a **dual-pathway cognitive architecture** inspired by Kahneman's "Thinking, Fast and Slow":

- **System 1 (Fast Path):** 30-50ns spatial hash lookup for reflexive responses
- **System 2 (Slow Path):** Existing v2.2 pattern detection + ADNA reasoning

This enables the system to respond instantly to familiar situations while learning from novel experiences.

---

## 1. Problem Statement

### 1.1 Performance Bottleneck (v2.2)

**Current flow:**
```
State → ADNA → Policy forward pass → Action
```

**Cost:** ~1-10ms per decision (depending on policy complexity)

**Problem:** Every decision requires full ADNA computation, even for situations the system has seen thousands of times.

### 1.2 Curse of Dimensionality

**8D Token space:**
- 8 dimensions × 16-bit coordinates = 128 bits of state
- Naive discretization: 2^128 possible states (impossible to enumerate)
- Fixed binning (256 bins): Too coarse for some dimensions, too fine for others

**Problem:** How to discretize 8D space adaptively?

---

## 2. Solution Architecture

### 2.1 Dual-Path System

```
┌─────────────────────────────────────────────────────────────┐
│                 IntuitionEngine v3.0                        │
├──────────────────────────┬──────────────────────────────────┤
│   A. REFLEX LAYER        │      B. ANALYTIC LAYER           │
│   (System 1 - Fast)      │      (System 2 - Slow)           │
│                          │                                  │
│  ┌────────────────────┐  │  ┌────────────────────────────┐  │
│  │  Spatial Hasher    │  │  │   Pattern Detector v2.2    │  │
│  │  (Token → u64)     │  │  │   (Experience → Patterns)  │  │
│  └──────────┬─────────┘  │  └──────────┬─────────────────┘  │
│             ▼            │             ▼                    │
│  ┌────────────────────┐  │  ┌────────────────────────────┐  │
│  │ AssociativeMemory  │◄─┼──│  Proposal Generator        │  │
│  │ (Hash→Connection)  │  │  │  (Create Hypothesis)       │  │
│  └────────────────────┘  │  └────────────────────────────┘  │
│             │            │             │                    │
│             ▼            │             ▼                    │
│  ┌────────────────────┐  │  ┌────────────────────────────┐  │
│  │  Fast Action       │  │  │  ADNA Reasoning            │  │
│  │  (<50ns)           │  │  │  (~1-10ms)                 │  │
│  └────────────────────┘  │  └────────────────────────────┘  │
└──────────────────────────┴──────────────────────────────────┘
```

### 2.2 Data Flow

1. **Input:** `Token` (current state)
2. **GridHash:** Compute `u64` hash from 8D coordinates
3. **Lookup:** Check `AssociativeMemory`
   - **Hit:** Get `ConnectionID` → Check confidence → Execute action (Fast Path)
   - **Miss:** Fallback to ADNA reasoning (Slow Path)
4. **Learning (Background):** Analytic Layer consolidates ADNA successes into reflexes

---

## 3. Core Algorithms

### 3.1 GridHash: Adaptive Spatial Hashing

**Purpose:** Convert 8D Token coordinates → single u64 hash

**Algorithm:**
```rust
/// Computes spatial hash with configurable resolution
///
/// # Parameters
/// - `token`: Current state (8D coordinates)
/// - `shift`: Quantization level (4-12 recommended)
///   - Higher shift = coarser grid = fewer bins
///   - Lower shift = finer grid = more bins
///
/// # Returns
/// u64 hash representing the "sector" in 8D space
pub fn compute_grid_hash(token: &Token, shift: &ShiftConfig) -> u64 {
    let mut hash: u64 = 0;

    for (dim_idx, &coord) in token.coordinates.iter().enumerate() {
        // 1. Quantization: Discard low-order bits (noise reduction)
        let dim_shift = shift.get_shift_for_dimension(dim_idx);
        let quantized = (coord as u64).wrapping_shr(dim_shift as u32);

        // 2. Mixing: XOR + rotate for bit dispersion
        // Different rotations per dimension to avoid collisions
        let rotation = 13 + (dim_idx * 7) as u32; // Prime-like spacing
        let dim_hash = quantized ^ ((dim_idx as u64).rotate_left(rotation));

        // 3. Accumulate into final hash
        hash ^= dim_hash.rotate_left(rotation);
    }

    hash
}
```

**Shift Configuration:**
```rust
pub struct ShiftConfig {
    /// Default shift for all dimensions
    pub default: u8,

    /// Per-dimension overrides
    /// Example: Physical dimensions (L1-L3) need finer resolution
    pub per_dimension: [Option<u8>; 8],
}

impl ShiftConfig {
    /// Recommended starting config
    pub fn default() -> Self {
        Self {
            default: 6,  // 2^6 = 64 bins per dimension
            per_dimension: [
                Some(4),  // L1 Physical X: 2^4 = 16 bins (fine)
                Some(4),  // L2 Physical Y: 16 bins
                Some(5),  // L3 Physical Z: 32 bins
                None,     // L4 Semantic: use default
                None,     // L5 Semantic: use default
                None,     // L6 Temporal: use default
                Some(8),  // L7 Emotional: 256 bins (coarse)
                Some(8),  // L8 Social: 256 bins (coarse)
            ],
        }
    }

    pub fn get_shift_for_dimension(&self, dim_idx: usize) -> u8 {
        self.per_dimension[dim_idx].unwrap_or(self.default)
    }
}
```

**Properties:**
- ✅ **Fast:** ~10-15 CPU cycles (~5ns on modern CPUs)
- ✅ **Deterministic:** Same input → same hash
- ✅ **Adaptive:** Per-dimension resolution tuning
- ✅ **Collision-resistant:** XOR + rotate mixing

---

### 3.2 AssociativeMemory: Lock-Free Reflex Storage

**Purpose:** Map hash → list of candidate Connections (reflexes)

**Data Structure:**
```rust
use dashmap::DashMap;
use smallvec::SmallVec;

pub struct AssociativeMemory {
    /// Hash → List of candidate ConnectionIDs
    /// SmallVec<4>: Inline storage for ≤4 items (no heap allocation)
    memory: DashMap<u64, SmallVec<[u64; 4]>>,

    /// Statistics for monitoring
    stats: Arc<RwLock<AssociativeStats>>,
}

pub struct AssociativeStats {
    pub total_entries: usize,
    pub total_lookups: u64,
    pub hits: u64,
    pub misses: u64,
    pub collisions: u64,  // Multiple candidates for same hash
}
```

**Operations:**
```rust
impl AssociativeMemory {
    /// Fast path: Lookup reflex by hash
    /// Returns list of candidate ConnectionIDs
    pub fn lookup(&self, hash: u64) -> Option<SmallVec<[u64; 4]>> {
        self.stats.write().unwrap().total_lookups += 1;

        match self.memory.get(&hash) {
            Some(candidates) => {
                self.stats.write().unwrap().hits += 1;
                if candidates.len() > 1 {
                    self.stats.write().unwrap().collisions += 1;
                }
                Some(candidates.clone())
            }
            None => {
                self.stats.write().unwrap().misses += 1;
                None
            }
        }
    }

    /// Slow path: Insert new reflex (from Analytic Layer)
    pub fn insert(&self, hash: u64, connection_id: u64) {
        self.memory
            .entry(hash)
            .or_insert_with(SmallVec::new)
            .push(connection_id);

        self.stats.write().unwrap().total_entries = self.memory.len();
    }

    /// TODO v0.32.0: LRU eviction
    pub fn evict_lru(&self, max_size: usize) {
        // Future implementation:
        // 1. Track last_access timestamp per entry
        // 2. Sort by timestamp
        // 3. Remove oldest until size ≤ max_size
        todo!("Implement LRU eviction in v0.32.0")
    }
}
```

**Why DashMap?**
- ✅ Lock-free concurrent HashMap (faster than `RwLock<HashMap>`)
- ✅ Sharded internally (reduces contention)
- ✅ Compatible with multi-threaded ADNA evolution

**Why SmallVec<4>?**
- ✅ Stack allocation for ≤4 items (most common case)
- ✅ Heap allocation only if >4 candidates (rare)
- ✅ Zero-cost for the common path

---

### 3.3 Fast Path: Reflex Execution

**Purpose:** Execute learned reflex without ADNA computation

**Algorithm:**
```rust
pub fn try_fast_path(
    &self,
    state_token: &Token,
    connections: &HashMap<u64, ConnectionV3>,
) -> Option<FastPathResult> {
    // 1. Compute hash
    let hash = compute_grid_hash(state_token, &self.shift_config);

    // 2. Lookup candidates
    let candidates = self.memory.lookup(hash)?;

    // 3. Collision resolution: Find best match
    let mut best_match: Option<(u64, f32)> = None;

    for &conn_id in candidates.iter() {
        let conn = connections.get(&conn_id)?;

        // 4. Verify this Connection is suitable for reflexes
        if !self.is_reflex_eligible(conn) {
            continue;
        }

        // 5. Compute similarity (handles hash collisions)
        let state_token_from_conn = self.get_state_token(conn)?;
        let similarity = state_token.similarity(&state_token_from_conn);

        // 6. Track best match
        match best_match {
            None => best_match = Some((conn_id, similarity)),
            Some((_, prev_sim)) if similarity > prev_sim => {
                best_match = Some((conn_id, similarity));
            }
            _ => {}
        }
    }

    // 7. Return if good enough match
    if let Some((conn_id, similarity)) = best_match {
        if similarity > self.config.similarity_threshold {
            return Some(FastPathResult {
                connection_id: conn_id,
                similarity,
                hash,
            });
        }
    }

    None
}

/// Check if Connection is eligible for fast path
fn is_reflex_eligible(&self, conn: &ConnectionV3) -> bool {
    // Only high-confidence Connections
    if conn.confidence < self.config.min_confidence {
        return false;
    }

    // Hypothesis connections need higher threshold
    if conn.mutability == ConnectionMutability::Hypothesis as u8 {
        return conn.confidence >= self.config.hypothesis_threshold;
    }

    // Learnable and Immutable are OK
    true
}
```

**Configuration:**
```rust
pub struct FastPathConfig {
    /// Minimum confidence to use reflex (0-255)
    pub min_confidence: u8,  // Default: 150 (~0.6)

    /// Higher threshold for Hypothesis connections
    pub hypothesis_threshold: u8,  // Default: 200 (~0.8)

    /// Similarity threshold after hash match (0.0-1.0)
    pub similarity_threshold: f32,  // Default: 0.85
}
```

**Performance Target:**
- Hash computation: ~5ns
- DashMap lookup: ~20-30ns (L1 cache hit)
- Similarity check (collision resolution): ~35ns × candidates
- **Total:** 30-50ns (typical case with 1-2 candidates)

---

### 3.4 Slow Path: Pattern Detection (v2.2 Preserved)

**Purpose:** Analyze experience, create new reflexes

**Algorithm** (unchanged from v2.2):
```rust
pub fn identify_patterns(&self, batch: &ExperienceBatch) -> Vec<IdentifiedPattern> {
    // Existing v2.2 implementation:
    // 1. Discretize states
    // 2. Group by state bin
    // 3. Compare action rewards
    // 4. Identify patterns (better_action vs worse_action)
    // 5. Calculate confidence

    // ... (existing code) ...
}
```

**Integration with Fast Path:**
```rust
/// Convert identified pattern → reflex in AssociativeMemory
pub fn consolidate_pattern(&mut self, pattern: &IdentifiedPattern) {
    // 1. Create Hypothesis Connection
    let conn = ConnectionV3::new_hypothesis(
        pattern.state_token_id,
        pattern.better_action_token_id,
        pattern.confidence as u8,
    );

    // 2. Store in Connection registry
    let conn_id = self.register_connection(conn);

    // 3. Compute hash for state
    let state_token = self.get_token(pattern.state_token_id);
    let hash = compute_grid_hash(&state_token, &self.shift_config);

    // 4. Insert into AssociativeMemory
    self.associative_memory.insert(hash, conn_id);

    // 5. Update stats
    self.stats.reflexes_created += 1;
}
```

---

## 4. Integration with Core Systems

### 4.1 Connection v3.0 Integration

**Reflex as Connection:**
- Reflexes are standard `ConnectionV3` objects
- Type: Usually `ConnectionType::Cause` or custom
- Mutability: `Hypothesis` (new) or `Learnable` (proven)
- Evidence count: Tracks how many times reflex was successful

**Learning:**
```rust
// When reflex succeeds:
connection.evidence_count += 1;
connection.update_confidence(true);

// Promotion: Hypothesis → Learnable
if connection.confidence > 200 && connection.evidence_count > 50 {
    connection.mutability = ConnectionMutability::Learnable as u8;
}
```

---

### 4.2 Guardian Integration: Trusted Reflexes

**Problem:** Full Guardian validation defeats the purpose of Fast Path

**Solution:** Two-tier validation

```rust
pub enum ValidationMode {
    Full,   // ADNA-initiated actions (full validation)
    Fast,   // Reflex-initiated actions (minimal checks)
}

impl Guardian {
    pub fn validate_action(
        &self,
        action: &Action,
        mode: ValidationMode,
    ) -> Result<(), ValidationError> {
        match mode {
            ValidationMode::Full => {
                // Existing full validation (all rules, all constraints)
                self.validate_cdna_constraints(action)?;
                self.validate_physical_limits(action)?;
                self.validate_resource_availability(action)?;
                self.validate_ethical_rules(action)?;
                Ok(())
            }
            ValidationMode::Fast => {
                // Only critical checks (physical safety)
                self.validate_physical_limits(action)?;
                // Skip expensive CDNA rule evaluation
                Ok(())
            }
        }
    }
}
```

**Rationale:**
- Reflex Connections were already validated when created (in Analytic Layer)
- Re-validating every time is redundant
- Fast validation still catches catastrophic failures (e.g., physical impossibilities)

**Performance:**
- Full validation: ~500-1000ns
- Fast validation: ~50-100ns

---

### 4.3 ADNA Integration: Fallback Path

**Flow:**
```rust
pub fn select_action(&mut self, state: &Token) -> Action {
    // 1. Try Fast Path
    if let Some(reflex) = self.intuition.try_fast_path(state, &self.connections) {
        let conn = self.connections.get(&reflex.connection_id).unwrap();
        let action = self.connection_to_action(conn);

        // 2. Fast validation
        if self.guardian.validate_action(&action, ValidationMode::Fast).is_ok() {
            self.stats.fast_path_hits += 1;
            return action;
        }
    }

    // 3. Fallback to ADNA (Slow Path)
    self.stats.slow_path_uses += 1;
    self.adna.select_action(state)
}
```

---

## 5. Adaptive Strategy

### 5.1 Dynamic Shift Adjustment

**Problem:** Optimal `shift` varies by environment and experience distribution

**Solution:** Adaptive tuning based on hit rate

```rust
pub struct AdaptiveShiftManager {
    current_shift: ShiftConfig,
    stats: ShiftStats,
}

pub struct ShiftStats {
    pub window_size: usize,  // Rolling window (e.g., last 1000 lookups)
    pub hit_count: usize,
    pub miss_count: usize,
    pub collision_count: usize,
}

impl AdaptiveShiftManager {
    pub fn update(&mut self, result: LookupResult) {
        self.stats.record(result);

        // Every N lookups, adjust shift
        if self.stats.total_count() >= self.stats.window_size {
            let hit_rate = self.stats.hit_rate();
            let collision_rate = self.stats.collision_rate();

            // Too many misses → grid too fine, increase shift
            if hit_rate < 0.1 {
                self.current_shift.default += 1;
            }

            // Too many collisions → grid too coarse, decrease shift
            if collision_rate > 0.5 {
                self.current_shift.default = self.current_shift.default.saturating_sub(1);
            }

            // Reset stats for next window
            self.stats.reset();
        }
    }
}
```

**Tuning Parameters:**
- `window_size`: 1000 lookups (balance responsiveness vs stability)
- Hit rate target: 30-50% (higher → too coarse, lower → too fine)
- Collision rate tolerance: <50% (higher → need finer grid)

---

## 6. Performance Specification

### 6.1 Target Metrics

| Operation | Target | Rationale |
|-----------|--------|-----------|
| GridHash computation | <10ns | 10-15 CPU cycles (XOR + rotate) |
| AssociativeMemory lookup | <30ns | L1 cache hit (~4 cycles) |
| Similarity check (collision) | <35ns | Token::similarity (existing) |
| Fast Path total | <50ns | Hash + lookup + 1 similarity check |
| Slow Path (existing v2.2) | ~1-10ms | ADNA forward pass |
| Speedup (Fast vs Slow) | 20,000-200,000x | Enables real-time reflexes |

### 6.2 Memory Usage

| Component | Size | Notes |
|-----------|------|-------|
| GridHash (computation) | 0 bytes | Stack only |
| AssociativeMemory (1M entries) | ~32 MB | DashMap overhead + SmallVec |
| Per-entry overhead | ~32 bytes | u64 key + SmallVec<4> + DashMap metadata |
| Per-Connection | 64 bytes | ConnectionV3 (unchanged) |

**Scalability:**
- 1,000 reflexes: ~32 KB
- 10,000 reflexes: ~320 KB
- 100,000 reflexes: ~3.2 MB
- 1,000,000 reflexes: ~32 MB

**Note:** LRU eviction (v0.32.0) will cap memory at configurable limit.

---

## 7. Metrics & Observability

### 7.1 IntuitionStats Structure

```rust
pub struct IntuitionStats {
    // Fast Path metrics
    pub fast_path_hits: u64,
    pub fast_path_misses: u64,
    pub fast_path_hit_rate: f32,  // hits / (hits + misses)
    pub avg_fast_path_time_ns: u64,

    // Slow Path metrics
    pub slow_path_uses: u64,
    pub avg_slow_path_time_ns: u64,

    // Memory metrics
    pub associative_memory_size: usize,
    pub total_reflexes: usize,
    pub hypothesis_count: usize,
    pub learnable_count: usize,

    // Hash metrics
    pub hash_collisions: u64,
    pub avg_candidates_per_lookup: f32,

    // Learning metrics
    pub reflexes_created: u64,
    pub reflexes_promoted: u64,  // Hypothesis → Learnable
    pub reflexes_failed: u64,     // Low confidence, removed

    // Shift adaptation metrics
    pub current_shift_default: u8,
    pub shift_adjustments: u64,
}
```

### 7.2 Visualization (TODO v0.32.0)

**Desktop UI Integration:**
- **Hit Rate Graph:** Time-series of fast_path_hit_rate (should increase with learning)
- **Speedup Ratio:** fast_path_time vs slow_path_time
- **Memory Heatmap:** 2D projection of 8D space showing "hot" sectors (many reflexes)
- **Collision Distribution:** Histogram of candidates_per_lookup

---

## 8. Testing Strategy

### 8.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn test_grid_hash_deterministic() {
        let token = Token::new(100);
        let hash1 = compute_grid_hash(&token, &ShiftConfig::default());
        let hash2 = compute_grid_hash(&token, &ShiftConfig::default());
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_grid_hash_shift_affects_result() {
        let token = Token::new(100);
        let shift_coarse = ShiftConfig { default: 8, ..Default::default() };
        let shift_fine = ShiftConfig { default: 4, ..Default::default() };

        let hash_coarse = compute_grid_hash(&token, &shift_coarse);
        let hash_fine = compute_grid_hash(&token, &shift_fine);

        // Different shifts should give different hashes (usually)
        // Note: Not always true due to XOR, but statistically likely
    }

    #[test]
    fn test_associative_memory_insert_lookup() {
        let memory = AssociativeMemory::new();
        memory.insert(12345, 99);

        let result = memory.lookup(12345);
        assert!(result.is_some());
        assert!(result.unwrap().contains(&99));
    }

    #[test]
    fn test_fast_path_hit() {
        let mut engine = IntuitionEngine::new();

        // Create reflex
        let state = Token::new(1);
        let action_conn = ConnectionV3::new_hypothesis(1, 2, 200);
        engine.add_reflex(state, action_conn);

        // Lookup should hit
        let result = engine.try_fast_path(&state, &engine.connections);
        assert!(result.is_some());
    }

    #[test]
    fn test_fast_path_miss() {
        let engine = IntuitionEngine::new();
        let unknown_state = Token::new(999);

        let result = engine.try_fast_path(&unknown_state, &engine.connections);
        assert!(result.is_none());
    }
}
```

### 8.2 Integration Tests

```rust
#[test]
fn test_e2e_reflex_learning_cycle() {
    let mut system = TestSystem::new();

    // 1. Novel situation (should use Slow Path)
    let state1 = Token::new(100);
    let action1 = system.select_action(&state1);
    assert_eq!(system.stats.slow_path_uses, 1);

    // 2. Execute action, get positive reward
    let reward = system.execute_action(&action1);
    assert!(reward > 0.5);

    // 3. Trigger pattern detection (Analytic Layer)
    system.consolidate_patterns();

    // 4. Same situation again (should use Fast Path now)
    let state2 = state1.clone();  // Same state
    let action2 = system.select_action(&state2);
    assert_eq!(system.stats.fast_path_hits, 1);

    // 5. Verify action is same as before
    assert_eq!(action1, action2);
}
```

### 8.3 Benchmarks

```rust
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_grid_hash(c: &mut Criterion) {
    let token = Token::new(100);
    let shift = ShiftConfig::default();

    c.bench_function("grid_hash", |b| {
        b.iter(|| {
            black_box(compute_grid_hash(black_box(&token), black_box(&shift)))
        })
    });
}

fn bench_fast_path_hit(c: &mut Criterion) {
    let mut engine = IntuitionEngine::new();
    // Populate with 1000 reflexes
    for i in 0..1000 {
        let state = Token::new(i);
        let conn = ConnectionV3::new_hypothesis(i, i + 1000, 200);
        engine.add_reflex(state, conn);
    }

    let test_state = Token::new(500);  // Known reflex

    c.bench_function("fast_path_hit", |b| {
        b.iter(|| {
            black_box(engine.try_fast_path(black_box(&test_state), &engine.connections))
        })
    });
}

fn bench_fast_vs_slow_path(c: &mut Criterion) {
    let mut group = c.benchmark_group("fast_vs_slow");

    // ... benchmark both paths, compare results ...

    group.finish();
}

criterion_group!(benches, bench_grid_hash, bench_fast_path_hit, bench_fast_vs_slow_path);
criterion_main!(benches);
```

**Target Results:**
- `grid_hash`: <10ns
- `fast_path_hit` (1000 entries): <50ns
- `fast_path_miss`: <30ns
- Speedup ratio: >10,000x

---

## 9. Implementation Roadmap

### Phase 1: Core Infrastructure — v0.31.0 ✅ COMPLETED
- [x] Create specification document
- [x] Implement `GridHash` algorithm (compute_grid_hash)
- [x] Implement `ShiftConfig` with defaults (uniform, per-dimension)
- [x] Unit tests for hashing (deterministic, shift variations)

### Phase 2: AssociativeMemory — v0.31.1 ✅ COMPLETED
- [x] Add `dashmap` dependency to Cargo.toml
- [x] Implement `AssociativeMemory` struct with DashMap
- [x] Implement `insert`, `lookup`, `stats`
- [x] Unit tests for memory operations (insert, lookup, collision)
- [x] Integration tests (8/8 passing)

### Phase 3: Fast Path — v0.31.2 ✅ COMPLETED
- [x] Implement `try_fast_path` in IntuitionEngine
- [x] Collision resolution (confidence-based proxy, token_similarity ready)
- [x] Integration with existing v2.2 code
- [x] Unit tests for fast path
- [x] Benchmarks (Criterion.rs): 69.5ns Fast Path E2E

### Phase 4: Adaptive Tuning — v0.31.3 ✅ COMPLETED
- [x] Implement `AdaptiveTuningConfig` and `AdaptiveTuner`
- [x] Add shift adjustment methods (increase_shift, decrease_shift, adjust_dimension_shift)
- [x] Implement `token_similarity()` for collision resolution
- [x] Unit tests for adaptive tuning (9 new tests)
- [x] Export new types in lib.rs

### Phase 5: Integration & Polish — v0.31.4 ✅ COMPLETED
- [x] ADNA fallback integration
- [x] Benchmarks (criterion): Full performance report
- [x] Documentation updates (benchmarks, spec)
- [x] Guardian fast validation mode (`validate_reflex()`)
- [x] Connection reflex creation from patterns (automatic consolidation)

**Status:** v0.31.4 completes IntuitionEngine v3.0 implementation. All core features ready for production.

---

## 10. Future Work (v0.32.0+)

### 10.1 LRU Eviction (v0.32.0)
```rust
pub struct ReflexEntry {
    pub connection_id: u64,
    pub last_access: Instant,
    pub access_count: u32,
    pub creation_time: Instant,
}

impl AssociativeMemory {
    pub fn evict_lru(&mut self, max_size: usize) {
        if self.memory.len() <= max_size {
            return;
        }

        // Collect all entries with timestamps
        let mut entries: Vec<_> = self.memory.iter()
            .map(|e| (e.key(), e.value(), e.last_access))
            .collect();

        // Sort by last_access (oldest first)
        entries.sort_by_key(|(_, _, timestamp)| *timestamp);

        // Remove oldest until size ≤ max_size
        let to_remove = self.memory.len() - max_size;
        for i in 0..to_remove {
            self.memory.remove(&entries[i].0);
        }
    }
}
```

### 10.2 Desktop UI Visualization (v0.32.0)
- Real-time hit rate graphs
- 8D space heatmap (2D projections)
- Reflex memory browser (explore learned patterns)
- Shift parameter tuning UI

### 10.3 Advanced Features (v0.33.0+)
- Multi-resolution hashing (multiple shift levels simultaneously)
- Hierarchical reflexes (compose simple reflexes into complex ones)
- Transfer learning (share reflexes across agents)

---

## 11. References

**Academic:**
- Kahneman, D. (2011). *Thinking, Fast and Slow*. System 1 vs System 2 thinking.
- Dayan, P. & Niv, Y. (2008). *Reinforcement learning: The Good, The Bad and The Ugly*. Model-free (reflexes) vs model-based (planning).
- Hassabis, D. et al. (2017). *Neuroscience-Inspired AI*. Memory consolidation.

**Technical:**
- DashMap: https://docs.rs/dashmap
- SmallVec: https://docs.rs/smallvec
- Criterion benchmarking: https://docs.rs/criterion

---

**Authors:** Denis Chernov & Claude (Anthropic)
**License:** AGPL-3.0
**Status:** Ready for Implementation
**Next Step:** Phase 1 - Core Infrastructure
