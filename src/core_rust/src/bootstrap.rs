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
// Embedding Loading
// ============================================================================

impl BootstrapLibrary {
    /// Load embeddings from GloVe/Word2Vec text format
    ///
    /// Format: word dim1 dim2 ... dimN
    /// Example: cat 0.123 -0.456 0.789 ...
    ///
    /// # Arguments
    /// * `path` - Path to embeddings file
    ///
    /// # Returns
    /// Result with number of loaded embeddings
    pub fn load_embeddings<P: AsRef<Path>>(&mut self, path: P) -> Result<usize, BootstrapError> {
        use std::fs::File;
        use std::io::{BufRead, BufReader};

        let file = File::open(path.as_ref())
            .map_err(|e| BootstrapError::IoError(e.to_string()))?;

        let reader = BufReader::new(file);
        let mut loaded = 0;

        for (line_num, line) in reader.lines().enumerate() {
            let line = line.map_err(|e| BootstrapError::IoError(e.to_string()))?;
            let line = line.trim();

            // Skip empty lines
            if line.is_empty() {
                continue;
            }

            // Parse line: word dim1 dim2 ... dimN
            let parts: Vec<&str> = line.split_whitespace().collect();

            if parts.len() < 2 {
                return Err(BootstrapError::ParseError(
                    format!("Line {}: too few columns", line_num + 1)
                ));
            }

            let word = parts[0].to_string();

            // Parse embedding dimensions
            let embedding: Result<Vec<f32>, _> = parts[1..]
                .iter()
                .map(|s| s.parse::<f32>())
                .collect();

            let embedding = embedding.map_err(|e| {
                BootstrapError::ParseError(format!("Line {}: {}", line_num + 1, e))
            })?;

            // Validate dimension
            if embedding.len() != self.config.embedding_dim {
                return Err(BootstrapError::DimensionMismatch {
                    expected: self.config.embedding_dim,
                    got: embedding.len(),
                });
            }

            // Create concept
            let id = Self::generate_id(&word, self.config.seed);
            let concept = SemanticConcept {
                id,
                word: word.clone(),
                embedding: Array1::from_vec(embedding),
                coords: [0.0, 0.0, 0.0], // Will be filled by PCA
                color: None,
                emotion: None,
            };

            self.concepts.insert(word, concept);
            loaded += 1;

            // Check max_words limit
            if self.config.max_words > 0 && loaded >= self.config.max_words {
                break;
            }
        }

        Ok(loaded)
    }

    /// Get all loaded embeddings as a matrix (rows = words, cols = dimensions)
    fn get_embedding_matrix(&self) -> Array2<f32> {
        let n = self.concepts.len();
        let d = self.config.embedding_dim;

        let mut matrix = Array2::zeros((n, d));

        for (i, concept) in self.concepts.values().enumerate() {
            for (j, &val) in concept.embedding.iter().enumerate() {
                matrix[[i, j]] = val;
            }
        }

        matrix
    }
}

// ============================================================================
// PCA Training and Projection
// ============================================================================

impl BootstrapLibrary {
    /// Train PCA model on loaded embeddings
    ///
    /// Simplified version: takes first N dimensions and normalizes
    /// TODO: Implement proper PCA with SVD for production
    ///
    /// # Returns
    /// Result with explained variance ratio
    pub fn train_pca(&mut self) -> Result<f32, BootstrapError> {
        if self.concepts.is_empty() {
            return Err(BootstrapError::NoData("No embeddings loaded".to_string()));
        }

        // Get embedding matrix
        let data = self.get_embedding_matrix();

        // Compute mean for each dimension
        let n_samples = data.nrows() as f32;
        let mut mean = Array1::zeros(self.config.embedding_dim);

        for i in 0..self.config.embedding_dim {
            let mut sum = 0.0f32;
            for j in 0..data.nrows() {
                sum += data[[j, i]];
            }
            mean[i] = sum / n_samples;
        }

        // Simple truncation: use identity matrix for first target_dim dimensions
        // This is a simplified approach - production would use SVD
        let components = Array2::from_shape_fn(
            (self.config.target_dim, self.config.embedding_dim),
            |(i, j)| {
                if i == j && i < self.config.target_dim {
                    1.0f32
                } else {
                    0.0f32
                }
            }
        );

        // Simplified explained variance (equal distribution)
        let explained_variance = Array1::from_vec(
            vec![1.0f32 / self.config.target_dim as f32; self.config.target_dim]
        );
        let total_variance: f32 = explained_variance.sum();

        // Store PCA model
        self.pca_model = Some(PCAModel {
            mean,
            components,
            explained_variance,
            original_dim: self.config.embedding_dim,
            target_dim: self.config.target_dim,
        });

        Ok(total_variance)
    }

    /// Project all loaded embeddings to 3D space using trained PCA
    ///
    /// Updates coords field in all SemanticConcepts
    ///
    /// # Returns
    /// Result with number of projected concepts
    pub fn project_embeddings(&mut self) -> Result<usize, BootstrapError> {
        let pca_model = self.pca_model.as_ref()
            .ok_or_else(|| BootstrapError::NoData("PCA model not trained".to_string()))?;

        let mut projected = 0;

        for concept in self.concepts.values_mut() {
            // Center embedding
            let centered = &concept.embedding - &pca_model.mean;

            // Project: coords = centered @ components.T
            let mut coords = [0.0f32; 3];
            for i in 0..pca_model.target_dim {
                let mut sum = 0.0f32;
                for j in 0..pca_model.original_dim {
                    sum += centered[j] * pca_model.components[[i, j]];
                }
                coords[i] = sum;
            }

            concept.coords = coords;
            projected += 1;
        }

        Ok(projected)
    }

    /// Complete PCA pipeline: train + project
    ///
    /// # Returns
    /// Result with (explained_variance_ratio, num_projected)
    pub fn run_pca_pipeline(&mut self) -> Result<(f32, usize), BootstrapError> {
        let variance = self.train_pca()?;
        let projected = self.project_embeddings()?;
        Ok((variance, projected))
    }
}

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug)]
pub enum BootstrapError {
    IoError(String),
    ParseError(String),
    DimensionMismatch { expected: usize, got: usize },
    NoData(String),
    PcaError(String),
}

impl std::fmt::Display for BootstrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IoError(msg) => write!(f, "IO error: {}", msg),
            Self::ParseError(msg) => write!(f, "Parse error: {}", msg),
            Self::DimensionMismatch { expected, got } => {
                write!(f, "Dimension mismatch: expected {}, got {}", expected, got)
            }
            Self::NoData(msg) => write!(f, "No data: {}", msg),
            Self::PcaError(msg) => write!(f, "PCA error: {}", msg),
        }
    }
}

impl std::error::Error for BootstrapError {}

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

    #[test]
    fn test_load_embeddings_small() {
        use std::io::Write;
        use std::fs::File;

        // Create temporary test file with 3D embeddings
        let temp_path = "/tmp/test_embeddings.txt";
        let mut file = File::create(temp_path).unwrap();
        writeln!(file, "cat 0.1 0.2 0.3").unwrap();
        writeln!(file, "dog 0.4 0.5 0.6").unwrap();
        writeln!(file, "bird 0.7 0.8 0.9").unwrap();

        // Load embeddings
        let mut config = BootstrapConfig::default();
        config.embedding_dim = 3;
        config.target_dim = 2;

        let mut bootstrap = BootstrapLibrary::new(config);
        let loaded = bootstrap.load_embeddings(temp_path).unwrap();

        assert_eq!(loaded, 3);
        assert_eq!(bootstrap.concept_count(), 3);

        // Check concept data
        let cat = bootstrap.get_concept("cat").unwrap();
        assert_eq!(cat.word, "cat");
        assert_eq!(cat.embedding[0], 0.1);
        assert_eq!(cat.embedding[1], 0.2);
        assert_eq!(cat.embedding[2], 0.3);

        // Clean up
        std::fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_pca_pipeline() {
        use std::io::Write;
        use std::fs::File;

        // Create temporary test file with 5D embeddings
        let temp_path = "/tmp/test_pca_embeddings.txt";
        let mut file = File::create(temp_path).unwrap();

        // Generate 10 test words with 5D embeddings
        for i in 0..10 {
            let v1 = (i as f32) * 0.1;
            let v2 = (i as f32) * 0.2;
            let v3 = (i as f32) * 0.05;
            let v4 = (i as f32) * -0.1;
            let v5 = (i as f32) * 0.15;
            writeln!(file, "word{} {} {} {} {} {}", i, v1, v2, v3, v4, v5).unwrap();
        }

        // Load and run PCA
        let mut config = BootstrapConfig::default();
        config.embedding_dim = 5;
        config.target_dim = 3;

        let mut bootstrap = BootstrapLibrary::new(config);
        let loaded = bootstrap.load_embeddings(temp_path).unwrap();
        assert_eq!(loaded, 10);

        // Run PCA pipeline
        let (variance, projected) = bootstrap.run_pca_pipeline().unwrap();

        assert_eq!(projected, 10);
        assert!(variance > 0.0, "Explained variance should be positive");

        // Check that coordinates were updated
        let concept = bootstrap.get_concept("word0").unwrap();
        assert_ne!(concept.coords, [0.0, 0.0, 0.0], "Coords should be set by PCA");

        // Check PCA model exists
        assert!(bootstrap.pca_model.is_some());

        // Clean up
        std::fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_max_words_limit() {
        use std::io::Write;
        use std::fs::File;

        let temp_path = "/tmp/test_max_words.txt";
        let mut file = File::create(temp_path).unwrap();

        for i in 0..100 {
            writeln!(file, "word{} 0.1 0.2 0.3", i).unwrap();
        }

        let mut config = BootstrapConfig::default();
        config.embedding_dim = 3;
        config.max_words = 10; // Load only 10

        let mut bootstrap = BootstrapLibrary::new(config);
        let loaded = bootstrap.load_embeddings(temp_path).unwrap();

        assert_eq!(loaded, 10);
        assert_eq!(bootstrap.concept_count(), 10);

        std::fs::remove_file(temp_path).ok();
    }
}
