// NeuroGraph - Высокопроизводительная система пространственных вычислений на основе токенов.
// Copyright (C) 2024-2025 Chernov Denys

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Bootstrap Library v1.2
//!
//! Provides semantic graph initialization from pre-trained embeddings.
//!
//! Features:
//! - Deterministic ID generation (MurmurHash3)
//! - GloVe/Word2Vec embedding loading
//! - PCA dimensionality reduction (300D → 3D)
//! - Multimodal anchor creation (concept + color + emotion)
//! - Connection weaving via Grid KNN
//! - Artifact persistence (PCA model, bootstrap map)

use crate::{Graph, Grid, NodeId};
use fasthash::murmur3::Hasher32;
use fasthash::FastHasher;
use ndarray::{Array1, Array2};
use std::collections::HashMap;
use std::hash::Hasher;
use std::path::Path;

// ============================================================================
// Configuration
// ============================================================================

/// Configuration for Bootstrap Library
#[derive(Debug, Clone)]
pub struct BootstrapConfig {
    /// Path to embeddings file (GloVe/Word2Vec format)
    pub embeddings_path: String,

    /// Original embedding dimension (e.g., 300 for GloVe-300d)
    pub embedding_dim: usize,

    /// Target dimension after PCA (default: 3 for spatial coords)
    pub target_dim: usize,

    /// Number of words to load (0 = all)
    pub max_words: usize,

    /// K for KNN connection weaving
    pub knn_k: usize,

    /// Connection weight decay factor
    pub connection_decay: f32,

    /// Seed for deterministic operations
    pub seed: u32,
}

impl Default for BootstrapConfig {
    fn default() -> Self {
        Self {
            embeddings_path: String::new(),
            embedding_dim: 300,
            target_dim: 3,
            max_words: 0, // Load all
            knn_k: 5,
            connection_decay: 0.1,
            seed: 42,
        }
    }
}

// ============================================================================
// Core Structures
// ============================================================================

/// Represents a semantic concept in the bootstrap process
#[derive(Debug, Clone)]
pub struct SemanticConcept {
    /// Unique identifier (deterministic hash of word)
    pub id: NodeId,

    /// Word/concept text
    pub word: String,

    /// Original high-dimensional embedding
    pub embedding: Array1<f32>,

    /// 3D coordinates after PCA projection
    pub coords: [f32; 3],

    /// Multimodal anchors (optional)
    pub color: Option<[f32; 3]>,      // RGB
    pub emotion: Option<[f32; 3]>,    // Valence, Arousal, Dominance
}

/// PCA model for dimensionality reduction
#[derive(Debug, Clone)]
pub struct PCAModel {
    /// Mean vector for centering
    pub mean: Array1<f32>,

    /// Principal components (rows are components)
    pub components: Array2<f32>,

    /// Explained variance ratio per component
    pub explained_variance: Array1<f32>,

    /// Original dimension
    pub original_dim: usize,

    /// Target dimension
    pub target_dim: usize,
}

/// Main Bootstrap Library
pub struct BootstrapLibrary {
    /// Configuration
    config: BootstrapConfig,

    /// Semantic concepts (word -> concept)
    concepts: HashMap<String, SemanticConcept>,

    /// Trained PCA model
    pca_model: Option<PCAModel>,

    /// Target graph for population
    graph: Graph,

    /// Grid for spatial queries
    grid: Grid,
}

// ============================================================================
// Implementation
// ============================================================================

impl BootstrapLibrary {
    /// Create new Bootstrap Library
    pub fn new(config: BootstrapConfig) -> Self {
        Self {
            config,
            concepts: HashMap::new(),
            pca_model: None,
            graph: Graph::new(),
            grid: Grid::new(),
        }
    }

    /// Get reference to underlying graph
    pub fn graph(&self) -> &Graph {
        &self.graph
    }

    /// Get mutable reference to underlying graph
    pub fn graph_mut(&mut self) -> &mut Graph {
        &mut self.graph
    }

    /// Get reference to grid
    pub fn grid(&self) -> &Grid {
        &self.grid
    }

    /// Get number of loaded concepts
    pub fn concept_count(&self) -> usize {
        self.concepts.len()
    }

    /// Get concept by word
    pub fn get_concept(&self, word: &str) -> Option<&SemanticConcept> {
        self.concepts.get(word)
    }
}

// ============================================================================
// ID Generation (Deterministic Hashing)
// ============================================================================

impl BootstrapLibrary {
    /// Generate deterministic NodeId from word using MurmurHash3
    ///
    /// # Arguments
    /// * `word` - Input word/concept
    /// * `seed` - Hash seed for reproducibility
    ///
    /// # Returns
    /// Deterministic 32-bit NodeId
    pub fn generate_id(word: &str, seed: u32) -> NodeId {
        let mut hasher = Hasher32::with_seed(seed);
        hasher.write(word.as_bytes());
        hasher.finish() as u32
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bootstrap_creation() {
        let config = BootstrapConfig::default();
        let bootstrap = BootstrapLibrary::new(config);

        assert_eq!(bootstrap.concept_count(), 0);
    }

    #[test]
    fn test_generate_id_deterministic() {
        let id1 = BootstrapLibrary::generate_id("cat", 42);
        let id2 = BootstrapLibrary::generate_id("cat", 42);
        let id3 = BootstrapLibrary::generate_id("dog", 42);

        // Same word+seed = same ID
        assert_eq!(id1, id2);

        // Different word = different ID
        assert_ne!(id1, id3);
    }

    #[test]
    fn test_generate_id_different_seeds() {
        let id1 = BootstrapLibrary::generate_id("cat", 42);
        let id2 = BootstrapLibrary::generate_id("cat", 123);

        // Different seed = different ID
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_config_default() {
        let config = BootstrapConfig::default();

        assert_eq!(config.embedding_dim, 300);
        assert_eq!(config.target_dim, 3);
        assert_eq!(config.knn_k, 5);
        assert_eq!(config.seed, 42);
    }
}
