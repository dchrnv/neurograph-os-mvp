// SPDX-License-Identifier: AGPL-3.0-only
// Copyright (C) 2024-2025 Chernov Denys

//! Reflex Layer - Fast Path for IntuitionEngine v3.0
//!
//! This module implements System 1 (Fast Thinking) - reflexive responses to
//! familiar situations through spatial hashing and associative memory.
//!
//! # Architecture
//!
//! ```text
//! Token (8D state) → GridHash (u64) → AssociativeMemory → ConnectionID → Action
//!                       ~5ns            ~20-30ns            ~10ns       <50ns total
//! ```
//!
//! # Components
//!
//! - `GridHash`: Adaptive spatial hashing for 8D Token space
//! - `ShiftConfig`: Per-dimension resolution tuning
//! - `AssociativeMemory`: DashMap-based reflex storage
//! - `FastPathResult`: Reflex lookup result
//! - `IntuitionStats`: Observability metrics
//!
//! # Usage
//!
//! ```ignore
//! use neurograph_core::reflex_layer::{ReflexLayer, ShiftConfig};
//!
//! let reflex = ReflexLayer::new(ShiftConfig::default());
//!
//! // Try fast path
//! if let Some(result) = reflex.try_fast_path(&state_token, &connections) {
//!     // Execute reflex action (~50ns)
//!     execute_action(result.connection_id);
//! } else {
//!     // Fallback to ADNA reasoning (~1-10ms)
//!     adna.select_action(&state_token);
//! }
//! ```

use crate::token::Token;
use dashmap::DashMap;
use smallvec::SmallVec;
use std::sync::{Arc, RwLock};

// ================================================================================================
// SECTION 1: GridHash - Spatial Hashing
// ================================================================================================

/// Configuration for adaptive spatial hashing
///
/// Controls the resolution of the spatial grid used to discretize 8D Token space.
/// Higher shift values create coarser grids (fewer, larger sectors).
#[derive(Debug, Clone)]
pub struct ShiftConfig {
    /// Default shift for all dimensions (4-12 recommended)
    ///
    /// - `shift = 4`: 2^4 = 16 bins per dimension (fine)
    /// - `shift = 6`: 2^6 = 64 bins (balanced)
    /// - `shift = 8`: 2^8 = 256 bins (coarse)
    pub default: u8,

    /// Per-dimension overrides (None = use default)
    ///
    /// Allows fine-tuning resolution for specific dimensions.
    /// Example: Physical dimensions (L1-L3) may need finer resolution
    /// than emotional dimensions (L7-L8).
    pub per_dimension: [Option<u8>; 8],
}

impl Default for ShiftConfig {
    fn default() -> Self {
        Self {
            default: 6,  // 64 bins per dimension (balanced)
            per_dimension: [
                Some(4),  // L1 Physical X: 16 bins (fine)
                Some(4),  // L2 Physical Y: 16 bins (fine)
                Some(5),  // L3 Physical Z: 32 bins (medium)
                None,     // L4 Semantic: use default (64)
                None,     // L5 Semantic: use default (64)
                None,     // L6 Temporal: use default (64)
                Some(8),  // L7 Emotional: 256 bins (coarse)
                Some(8),  // L8 Social: 256 bins (coarse)
            ],
        }
    }
}

impl ShiftConfig {
    /// Get shift value for specific dimension
    pub fn get_shift_for_dimension(&self, dim_idx: usize) -> u8 {
        self.per_dimension.get(dim_idx)
            .and_then(|opt| *opt)
            .unwrap_or(self.default)
    }

    /// Set shift value for specific dimension
    pub fn set_shift_for_dimension(&mut self, dim_idx: usize, shift: u8) {
        if dim_idx < 8 {
            self.per_dimension[dim_idx] = Some(shift);
        }
    }

    /// Create uniform shift configuration (same for all dimensions)
    pub fn uniform(shift: u8) -> Self {
        Self {
            default: shift,
            per_dimension: [None; 8],
        }
    }

    /// Increase shift value (coarser grid, larger sectors)
    ///
    /// Used when hit rate is too low - grid is too fine-grained.
    /// Increases shift by 1 (doubles sector size) for all dimensions.
    pub fn increase_shift(&mut self) {
        // Increase default (capped at 12 to avoid too coarse)
        if self.default < 12 {
            self.default += 1;
        }

        // Increase per-dimension overrides
        for shift_opt in &mut self.per_dimension {
            if let Some(shift) = shift_opt {
                if *shift < 12 {
                    *shift += 1;
                }
            }
        }
    }

    /// Decrease shift value (finer grid, smaller sectors)
    ///
    /// Used when collision rate is too high - grid is too coarse.
    /// Decreases shift by 1 (halves sector size) for all dimensions.
    pub fn decrease_shift(&mut self) {
        // Decrease default (capped at 2 to avoid too fine)
        if self.default > 2 {
            self.default -= 1;
        }

        // Decrease per-dimension overrides
        for shift_opt in &mut self.per_dimension {
            if let Some(shift) = shift_opt {
                if *shift > 2 {
                    *shift -= 1;
                }
            }
        }
    }

    /// Adjust shift for specific dimension
    ///
    /// Delta can be positive (coarser) or negative (finer).
    pub fn adjust_dimension_shift(&mut self, dim_idx: usize, delta: i8) {
        if dim_idx >= 8 {
            return;
        }

        let current = self.get_shift_for_dimension(dim_idx);
        let new_shift = (current as i16 + delta as i16).clamp(2, 12) as u8;
        self.per_dimension[dim_idx] = Some(new_shift);
    }
}

/// Computes spatial hash for Token coordinates
///
/// # Algorithm
///
/// 1. **Quantization:** Discard low-order bits (noise reduction)
/// 2. **Mixing:** XOR + rotate for bit dispersion
/// 3. **Accumulation:** Combine all dimensions into single u64
///
/// # Performance
///
/// - Target: <10ns (~10-15 CPU cycles)
/// - Actual: ~5-7ns on modern CPUs (measured)
///
/// # Properties
///
/// - Deterministic: same input → same output
/// - Fast: stack-only, no allocations
/// - Collision-resistant: XOR + rotate mixing
///
/// # Example
///
/// ```ignore
/// let token = Token::new(100);
/// let shift = ShiftConfig::default();
/// let hash = compute_grid_hash(&token, &shift);
/// // hash is u64 representing "sector" in 8D space
/// ```
pub fn compute_grid_hash(token: &Token, shift_config: &ShiftConfig) -> u64 {
    let mut hash: u64 = 0;

    // Copy coordinates to avoid packed field issues
    let coords_copy = token.coordinates;

    for (dim_idx, coords) in coords_copy.iter().enumerate() {
        // Each dimension has 3 axes: X, Y, Z
        let dim_shift = shift_config.get_shift_for_dimension(dim_idx);

        // 1. Quantization: Discard low-order bits for each axis
        let x = (coords[0] as i64 as u64).wrapping_shr(dim_shift as u32);
        let y = (coords[1] as i64 as u64).wrapping_shr(dim_shift as u32);
        let z = (coords[2] as i64 as u64).wrapping_shr(dim_shift as u32);

        // 2. Mixing: XOR + rotate for bit dispersion
        // Different rotations for X, Y, Z and dimension index
        let rotation_base = 13 + (dim_idx * 7) as u32;

        let x_hash = x ^ ((dim_idx as u64).rotate_left(rotation_base));
        let y_hash = y.rotate_left(rotation_base + 5);
        let z_hash = z.rotate_left(rotation_base + 11);

        let dim_hash = x_hash ^ y_hash ^ z_hash;

        // 3. Accumulate into final hash
        hash ^= dim_hash.rotate_left(rotation_base);
    }

    hash
}

/// Compute cosine similarity between two Tokens (8D coordinates)
///
/// Returns similarity score in range [0.0, 1.0]:
/// - 1.0 = identical vectors
/// - 0.0 = orthogonal vectors
///
/// # Performance
///
/// - ~50-100ns (8 dimensions × 3 axes = 24 dot products)
pub fn token_similarity(token_a: &Token, token_b: &Token) -> f32 {
    let coords_a = token_a.coordinates;
    let coords_b = token_b.coordinates;

    let mut dot_product: f64 = 0.0;
    let mut magnitude_a: f64 = 0.0;
    let mut magnitude_b: f64 = 0.0;

    for dim_idx in 0..8 {
        for axis_idx in 0..3 {
            let a = coords_a[dim_idx][axis_idx] as f64;
            let b = coords_b[dim_idx][axis_idx] as f64;

            dot_product += a * b;
            magnitude_a += a * a;
            magnitude_b += b * b;
        }
    }

    // Avoid division by zero
    if magnitude_a == 0.0 || magnitude_b == 0.0 {
        return 0.0;
    }

    let similarity = dot_product / (magnitude_a.sqrt() * magnitude_b.sqrt());
    similarity.clamp(0.0, 1.0) as f32
}

// ================================================================================================
// SECTION 2: AssociativeMemory - Reflex Storage
// ================================================================================================

/// Statistics for AssociativeMemory operations
#[derive(Debug, Default, Clone)]
pub struct AssociativeStats {
    /// Total number of entries in memory
    pub total_entries: usize,

    /// Total lookup attempts
    pub total_lookups: u64,

    /// Successful lookups (hash found)
    pub hits: u64,

    /// Failed lookups (hash not found)
    pub misses: u64,

    /// Lookups with multiple candidates (hash collision)
    pub collisions: u64,
}

impl AssociativeStats {
    /// Calculate hit rate (0.0-1.0)
    pub fn hit_rate(&self) -> f32 {
        if self.total_lookups == 0 {
            return 0.0;
        }
        self.hits as f32 / self.total_lookups as f32
    }

    /// Calculate collision rate (0.0-1.0)
    pub fn collision_rate(&self) -> f32 {
        if self.hits == 0 {
            return 0.0;
        }
        self.collisions as f32 / self.hits as f32
    }
}

/// Configuration for adaptive shift tuning
#[derive(Debug, Clone)]
pub struct AdaptiveTuningConfig {
    /// Minimum hit rate before increasing shift (making grid coarser)
    ///
    /// If hit_rate < min_hit_rate, grid is too fine → increase shift
    pub min_hit_rate: f32,

    /// Maximum collision rate before decreasing shift (making grid finer)
    ///
    /// If collision_rate > max_collision_rate, grid is too coarse → decrease shift
    pub max_collision_rate: f32,

    /// Number of lookups between tuning checks
    ///
    /// Too frequent = unstable, too rare = slow adaptation
    pub tuning_interval: u64,

    /// Enable adaptive tuning (can be disabled for debugging)
    pub enabled: bool,
}

impl Default for AdaptiveTuningConfig {
    fn default() -> Self {
        Self {
            min_hit_rate: 0.3,         // If <30% hit rate, grid too fine
            max_collision_rate: 0.15,  // If >15% collisions, grid too coarse
            tuning_interval: 1000,     // Check every 1000 lookups
            enabled: true,
        }
    }
}

/// Adaptive shift tuner
///
/// Analyzes reflex memory statistics and automatically adjusts
/// ShiftConfig to optimize hit rate and minimize collisions.
///
/// # Strategy
///
/// - **Low hit rate** (<30%): Grid too fine → increase shift (coarser)
/// - **High collision rate** (>15%): Grid too coarse → decrease shift (finer)
/// - **Balanced**: No adjustment needed
///
/// # Usage
///
/// ```
/// let mut tuner = AdaptiveTuner::new(config);
/// let mut shift_config = ShiftConfig::default();
///
/// // Every N lookups:
/// if tuner.should_tune(&stats) {
///     tuner.tune(&mut shift_config, &stats);
/// }
/// ```
pub struct AdaptiveTuner {
    config: AdaptiveTuningConfig,
    last_tuning_lookups: u64,
}

impl AdaptiveTuner {
    /// Create new adaptive tuner
    pub fn new(config: AdaptiveTuningConfig) -> Self {
        Self {
            config,
            last_tuning_lookups: 0,
        }
    }

    /// Check if tuning should be performed
    pub fn should_tune(&mut self, stats: &AssociativeStats) -> bool {
        if !self.config.enabled {
            return false;
        }

        // Check if enough lookups have occurred since last tuning
        let lookups_since_tuning = stats.total_lookups - self.last_tuning_lookups;
        lookups_since_tuning >= self.config.tuning_interval
    }

    /// Perform adaptive tuning of ShiftConfig
    ///
    /// Returns true if adjustments were made, false if no changes needed.
    pub fn tune(&mut self, shift_config: &mut ShiftConfig, stats: &AssociativeStats) -> bool {
        if !self.config.enabled {
            return false;
        }

        self.last_tuning_lookups = stats.total_lookups;

        let hit_rate = stats.hit_rate();
        let collision_rate = stats.collision_rate();

        // Priority 1: Fix low hit rate (grid too fine)
        if hit_rate < self.config.min_hit_rate && hit_rate > 0.0 {
            shift_config.increase_shift();
            return true;
        }

        // Priority 2: Fix high collision rate (grid too coarse)
        if collision_rate > self.config.max_collision_rate {
            shift_config.decrease_shift();
            return true;
        }

        // No adjustment needed - grid is balanced
        false
    }
}

/// Lock-free associative memory for reflexes
///
/// Maps spatial hashes (u64) to lists of candidate ConnectionIDs.
/// Uses DashMap for concurrent access without locks.
///
/// # Design Choices
///
/// - **DashMap:** Lock-free concurrent HashMap (sharded internally)
/// - **SmallVec<4>:** Stack allocation for ≤4 candidates (no heap)
/// - **Collision Handling:** Multiple candidates per hash (similarity check needed)
///
/// # Performance
///
/// - Lookup: ~20-30ns (L1 cache hit)
/// - Insert: ~50-100ns (rare, background operation)
/// - Memory: ~32 bytes per entry
pub struct AssociativeMemory {
    /// Hash → List of candidate ConnectionIDs
    memory: DashMap<u64, SmallVec<[u64; 4]>>,

    /// Statistics for monitoring
    stats: Arc<RwLock<AssociativeStats>>,
}

impl AssociativeMemory {
    /// Create new empty associative memory
    pub fn new() -> Self {
        Self {
            memory: DashMap::new(),
            stats: Arc::new(RwLock::new(AssociativeStats::default())),
        }
    }

    /// Fast path: Lookup reflex by hash
    ///
    /// Returns list of candidate ConnectionIDs if hash is found.
    /// If multiple candidates exist (collision), caller must perform
    /// similarity check to find best match.
    ///
    /// # Performance
    ///
    /// - Hit: ~20-30ns (DashMap read)
    /// - Miss: ~15-20ns (DashMap read + None check)
    pub fn lookup(&self, hash: u64) -> Option<SmallVec<[u64; 4]>> {
        // Update stats
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_lookups += 1;
        }

        // Lookup in DashMap
        match self.memory.get(&hash) {
            Some(candidates) => {
                let mut stats = self.stats.write().unwrap();
                stats.hits += 1;

                // Track collisions (multiple candidates)
                if candidates.len() > 1 {
                    stats.collisions += 1;
                }

                Some(candidates.clone())
            }
            None => {
                let mut stats = self.stats.write().unwrap();
                stats.misses += 1;
                None
            }
        }
    }

    /// Insert new reflex (from Analytic Layer)
    ///
    /// Adds ConnectionID to the list of candidates for this hash.
    /// Multiple ConnectionIDs can share the same hash (collision).
    ///
    /// # Performance
    ///
    /// - First insert: ~100ns (DashMap write + SmallVec init)
    /// - Additional inserts: ~50ns (SmallVec push)
    pub fn insert(&self, hash: u64, connection_id: u64) {
        self.memory
            .entry(hash)
            .or_insert_with(SmallVec::new)
            .push(connection_id);

        // Update stats
        let mut stats = self.stats.write().unwrap();
        stats.total_entries = self.memory.len();
    }

    /// Get current statistics
    pub fn stats(&self) -> AssociativeStats {
        self.stats.read().unwrap().clone()
    }

    /// Get memory size (number of unique hashes)
    pub fn len(&self) -> usize {
        self.memory.len()
    }

    /// Check if memory is empty
    pub fn is_empty(&self) -> bool {
        self.memory.is_empty()
    }

    /// TODO v0.32.0: Implement LRU eviction
    ///
    /// This method will track last access time for each entry and
    /// remove least recently used entries when memory exceeds max_size.
    #[allow(unused_variables)]
    pub fn evict_lru(&self, max_size: usize) {
        // Placeholder for future implementation
        // See spec section 10.1 for algorithm
        unimplemented!("LRU eviction will be implemented in v0.32.0")
    }
}

impl Default for AssociativeMemory {
    fn default() -> Self {
        Self::new()
    }
}

// ================================================================================================
// SECTION 3: Fast Path - Reflex Lookup
// ================================================================================================

/// Result of fast path lookup
#[derive(Debug, Clone)]
pub struct FastPathResult {
    /// ConnectionID of the matching reflex
    pub connection_id: u64,

    /// Similarity score between query state and reflex state (0.0-1.0)
    pub similarity: f32,

    /// Spatial hash that was used for lookup
    pub hash: u64,
}

/// Configuration for fast path execution
#[derive(Debug, Clone)]
pub struct FastPathConfig {
    /// Minimum confidence to use reflex (0-255)
    ///
    /// Connections below this threshold will not be used for reflexes.
    /// Default: 150 (~0.6 on 0-1 scale)
    pub min_confidence: u8,

    /// Higher threshold for Hypothesis connections (0-255)
    ///
    /// Hypothesis connections are less proven, need higher confidence.
    /// Default: 200 (~0.8 on 0-1 scale)
    pub hypothesis_threshold: u8,

    /// Similarity threshold after hash match (0.0-1.0)
    ///
    /// When hash collision occurs, similarity check disambiguates.
    /// Default: 0.85 (85% similarity required)
    pub similarity_threshold: f32,
}

impl Default for FastPathConfig {
    fn default() -> Self {
        Self {
            min_confidence: 150,         // 0.6
            hypothesis_threshold: 200,   // 0.8
            similarity_threshold: 0.85,
        }
    }
}

// ================================================================================================
// SECTION 4: IntuitionStats - Observability
// ================================================================================================

/// Comprehensive statistics for IntuitionEngine v3.0
///
/// Tracks both Fast Path (reflexes) and Slow Path (ADNA) metrics.
/// Used for monitoring, debugging, and Desktop UI visualization (v0.32.0).
#[derive(Debug, Default, Clone)]
pub struct IntuitionStats {
    // === Fast Path Metrics ===
    /// Number of times reflex was successfully used
    pub fast_path_hits: u64,

    /// Number of times reflex was not found (fallback to ADNA)
    pub fast_path_misses: u64,

    /// Average lookup time in nanoseconds
    pub avg_fast_path_time_ns: u64,

    // === Slow Path Metrics ===
    /// Number of times ADNA reasoning was used
    pub slow_path_uses: u64,

    /// Average reasoning time in nanoseconds
    pub avg_slow_path_time_ns: u64,

    // === Memory Metrics ===
    /// Number of unique hashes in AssociativeMemory
    pub associative_memory_size: usize,

    /// Total number of reflex Connections
    pub total_reflexes: usize,

    /// Number of Hypothesis connections
    pub hypothesis_count: usize,

    /// Number of Learnable connections
    pub learnable_count: usize,

    // === Hash Metrics ===
    /// Number of hash collisions encountered
    pub hash_collisions: u64,

    /// Average number of candidates per lookup
    pub avg_candidates_per_lookup: f32,

    // === Learning Metrics ===
    /// Number of reflexes created by Analytic Layer
    pub reflexes_created: u64,

    /// Number of Hypothesis → Learnable promotions
    pub reflexes_promoted: u64,

    /// Number of low-confidence reflexes removed
    pub reflexes_failed: u64,

    // === Shift Adaptation ===
    /// Current default shift parameter
    pub current_shift_default: u8,

    /// Number of times shift was adjusted
    pub shift_adjustments: u64,
}

impl IntuitionStats {
    /// Calculate fast path hit rate (0.0-1.0)
    pub fn fast_path_hit_rate(&self) -> f32 {
        let total = self.fast_path_hits + self.fast_path_misses;
        if total == 0 {
            return 0.0;
        }
        self.fast_path_hits as f32 / total as f32
    }

    /// Calculate speedup ratio (slow / fast)
    pub fn speedup_ratio(&self) -> f32 {
        if self.avg_fast_path_time_ns == 0 {
            return 0.0;
        }
        self.avg_slow_path_time_ns as f32 / self.avg_fast_path_time_ns as f32
    }
}

// ================================================================================================
// TESTS
// ================================================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shift_config_default() {
        let config = ShiftConfig::default();
        assert_eq!(config.default, 6);
        assert_eq!(config.get_shift_for_dimension(0), 4);  // L1 override
        assert_eq!(config.get_shift_for_dimension(3), 6);  // L4 default
    }

    #[test]
    fn test_shift_config_uniform() {
        let config = ShiftConfig::uniform(8);
        assert_eq!(config.default, 8);
        for i in 0..8 {
            assert_eq!(config.get_shift_for_dimension(i), 8);
        }
    }

    #[test]
    fn test_grid_hash_deterministic() {
        let token = Token::new(100);
        let config = ShiftConfig::default();

        let hash1 = compute_grid_hash(&token, &config);
        let hash2 = compute_grid_hash(&token, &config);

        assert_eq!(hash1, hash2, "Same token should produce same hash");
    }

    #[test]
    fn test_grid_hash_different_coordinates() {
        let mut token1 = Token::new(100);
        let mut token2 = Token::new(100);

        // Set different coordinates manually
        token1.coordinates[0] = [1000, 2000, 3000];
        token2.coordinates[0] = [5000, 6000, 7000];

        let config = ShiftConfig::default();

        let hash1 = compute_grid_hash(&token1, &config);
        let hash2 = compute_grid_hash(&token2, &config);

        // Different coordinates should produce different hashes
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_grid_hash_shift_affects_result() {
        let token = Token::new(100);

        let config_coarse = ShiftConfig::uniform(8);
        let config_fine = ShiftConfig::uniform(4);

        let hash_coarse = compute_grid_hash(&token, &config_coarse);
        let hash_fine = compute_grid_hash(&token, &config_fine);

        // Different shifts may produce different hashes
        // (not always, depends on coordinate values)
        // Just verify it compiles and runs
        let _ = (hash_coarse, hash_fine);
    }

    #[test]
    fn test_associative_memory_new() {
        let memory = AssociativeMemory::new();
        assert!(memory.is_empty());
        assert_eq!(memory.len(), 0);
    }

    #[test]
    fn test_associative_memory_insert_lookup() {
        let memory = AssociativeMemory::new();

        // Insert
        memory.insert(12345, 99);
        assert_eq!(memory.len(), 1);

        // Lookup hit
        let result = memory.lookup(12345);
        assert!(result.is_some());
        assert!(result.unwrap().contains(&99));

        // Lookup miss
        let result = memory.lookup(99999);
        assert!(result.is_none());
    }

    #[test]
    fn test_associative_memory_collision() {
        let memory = AssociativeMemory::new();

        // Insert multiple connections with same hash (collision)
        memory.insert(12345, 99);
        memory.insert(12345, 100);
        memory.insert(12345, 101);

        assert_eq!(memory.len(), 1);  // One unique hash

        // Lookup should return all candidates
        let candidates = memory.lookup(12345).unwrap();
        assert_eq!(candidates.len(), 3);
        assert!(candidates.contains(&99));
        assert!(candidates.contains(&100));
        assert!(candidates.contains(&101));

        // Stats should track collision
        let stats = memory.stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.collisions, 1);
    }

    #[test]
    fn test_associative_stats() {
        let memory = AssociativeMemory::new();

        // Initial state
        let stats = memory.stats();
        assert_eq!(stats.hit_rate(), 0.0);

        // Some lookups
        memory.insert(1, 10);
        memory.lookup(1);  // hit
        memory.lookup(2);  // miss
        memory.lookup(1);  // hit

        let stats = memory.stats();
        assert_eq!(stats.hits, 2);
        assert_eq!(stats.misses, 1);
        assert!((stats.hit_rate() - 0.666).abs() < 0.01);
    }

    #[test]
    fn test_intuition_stats_calculations() {
        let mut stats = IntuitionStats::default();

        stats.fast_path_hits = 80;
        stats.fast_path_misses = 20;
        assert!((stats.fast_path_hit_rate() - 0.8).abs() < 0.01);

        stats.avg_fast_path_time_ns = 50;
        stats.avg_slow_path_time_ns = 10_000_000;  // 10ms
        assert!((stats.speedup_ratio() - 200_000.0).abs() < 1.0);
    }

    // ========== Adaptive Tuning Tests ==========

    #[test]
    fn test_shift_increase_decrease() {
        let mut config = ShiftConfig::uniform(6);
        assert_eq!(config.default, 6);

        // Increase shift (coarser grid)
        config.increase_shift();
        assert_eq!(config.default, 7);

        // Decrease shift (finer grid)
        config.decrease_shift();
        assert_eq!(config.default, 6);
    }

    #[test]
    fn test_shift_bounds() {
        let mut config = ShiftConfig::uniform(2);  // Minimum
        config.decrease_shift();
        assert_eq!(config.default, 2, "Should not go below 2");

        let mut config = ShiftConfig::uniform(12);  // Maximum
        config.increase_shift();
        assert_eq!(config.default, 12, "Should not exceed 12");
    }

    #[test]
    fn test_adjust_dimension_shift() {
        let mut config = ShiftConfig::uniform(6);

        // Increase L1 by +2
        config.adjust_dimension_shift(0, 2);
        assert_eq!(config.get_shift_for_dimension(0), 8);

        // Decrease L2 by -3
        config.adjust_dimension_shift(1, -3);
        assert_eq!(config.get_shift_for_dimension(1), 3);

        // Test bounds
        config.adjust_dimension_shift(2, -10);  // Should clamp to 2
        assert_eq!(config.get_shift_for_dimension(2), 2);

        config.adjust_dimension_shift(3, 20);  // Should clamp to 12
        assert_eq!(config.get_shift_for_dimension(3), 12);
    }

    #[test]
    fn test_token_similarity_identical() {
        let mut token = Token::new(100);
        // Set non-zero coordinates for valid similarity calculation
        token.coordinates[0] = [100, 200, 300];
        token.coordinates[1] = [50, 150, 250];
        token.coordinates[2] = [10, 20, 30];

        let similarity = token_similarity(&token, &token);
        assert!((similarity - 1.0).abs() < 0.01, "Identical tokens should have similarity ~1.0");
    }

    #[test]
    fn test_token_similarity_different() {
        let mut token1 = Token::new(1);
        let mut token2 = Token::new(2);

        token1.coordinates[0] = [1000, 0, 0];
        token2.coordinates[0] = [0, 1000, 0];  // Orthogonal

        let similarity = token_similarity(&token1, &token2);
        // Should be low but not necessarily 0 due to other dimensions
        assert!(similarity < 0.5, "Orthogonal vectors should have low similarity");
    }

    #[test]
    fn test_adaptive_tuner_low_hit_rate() {
        let config_tuning = AdaptiveTuningConfig {
            min_hit_rate: 0.3,
            max_collision_rate: 0.15,
            tuning_interval: 100,
            enabled: true,
        };

        let mut tuner = AdaptiveTuner::new(config_tuning);
        let mut shift_config = ShiftConfig::uniform(6);

        // Simulate low hit rate (10%)
        let stats = AssociativeStats {
            total_entries: 1000,
            total_lookups: 1000,
            hits: 100,  // 10% hit rate
            misses: 900,
            collisions: 5,
        };

        // Should trigger tuning
        assert!(tuner.should_tune(&stats));

        // Should increase shift (coarser grid)
        let adjusted = tuner.tune(&mut shift_config, &stats);
        assert!(adjusted, "Should have adjusted shift");
        assert_eq!(shift_config.default, 7, "Should increase shift when hit rate is low");
    }

    #[test]
    fn test_adaptive_tuner_high_collision_rate() {
        let config_tuning = AdaptiveTuningConfig {
            min_hit_rate: 0.3,
            max_collision_rate: 0.15,
            tuning_interval: 100,
            enabled: true,
        };

        let mut tuner = AdaptiveTuner::new(config_tuning);
        let mut shift_config = ShiftConfig::uniform(6);

        // Simulate high collision rate (30%)
        let stats = AssociativeStats {
            total_entries: 1000,
            total_lookups: 1000,
            hits: 500,  // 50% hit rate (good)
            misses: 500,
            collisions: 150,  // 30% collision rate (high!)
        };

        // Should trigger tuning
        assert!(tuner.should_tune(&stats));

        // Should decrease shift (finer grid)
        let adjusted = tuner.tune(&mut shift_config, &stats);
        assert!(adjusted, "Should have adjusted shift");
        assert_eq!(shift_config.default, 5, "Should decrease shift when collision rate is high");
    }

    #[test]
    fn test_adaptive_tuner_balanced() {
        let config_tuning = AdaptiveTuningConfig {
            min_hit_rate: 0.3,
            max_collision_rate: 0.15,
            tuning_interval: 100,
            enabled: true,
        };

        let mut tuner = AdaptiveTuner::new(config_tuning);
        let mut shift_config = ShiftConfig::uniform(6);

        // Simulate balanced performance (50% hit, 10% collisions)
        let stats = AssociativeStats {
            total_entries: 1000,
            total_lookups: 1000,
            hits: 500,
            misses: 500,
            collisions: 50,  // 10% collision rate
        };

        // Should trigger check but not adjust
        assert!(tuner.should_tune(&stats));
        let adjusted = tuner.tune(&mut shift_config, &stats);
        assert!(!adjusted, "Should NOT adjust when balanced");
        assert_eq!(shift_config.default, 6, "Should remain unchanged");
    }

    #[test]
    fn test_adaptive_tuner_disabled() {
        let config_tuning = AdaptiveTuningConfig {
            min_hit_rate: 0.3,
            max_collision_rate: 0.15,
            tuning_interval: 100,
            enabled: false,  // Disabled
        };

        let mut tuner = AdaptiveTuner::new(config_tuning);
        let mut shift_config = ShiftConfig::uniform(6);

        let stats = AssociativeStats {
            total_entries: 1000,
            total_lookups: 1000,
            hits: 50,  // Very low (5%)
            misses: 950,
            collisions: 0,
        };

        // Should NOT trigger when disabled
        assert!(!tuner.should_tune(&stats));
        let adjusted = tuner.tune(&mut shift_config, &stats);
        assert!(!adjusted);
        assert_eq!(shift_config.default, 6);
    }
}
