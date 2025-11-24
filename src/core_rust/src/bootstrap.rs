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
// Graph Population and Connection Weaving
// ============================================================================

impl BootstrapLibrary {
    /// Populate Graph with all loaded concepts as nodes
    ///
    /// Adds each SemanticConcept as a node in the Graph
    ///
    /// # Returns
    /// Result with number of nodes added
    pub fn populate_graph(&mut self) -> Result<usize, BootstrapError> {
        if self.concepts.is_empty() {
            return Err(BootstrapError::NoData("No concepts loaded".to_string()));
        }

        let mut added = 0;

        for concept in self.concepts.values() {
            self.graph.add_node(concept.id);
            added += 1;
        }

        Ok(added)
    }

    /// Populate Grid with concept coordinates for spatial queries
    ///
    /// Adds each concept's 3D coordinates to the Grid for KNN lookup
    ///
    /// # Returns
    /// Result with number of tokens added to grid
    pub fn populate_grid(&mut self) -> Result<usize, BootstrapError> {
        use crate::Token;

        if self.concepts.is_empty() {
            return Err(BootstrapError::NoData("No concepts loaded".to_string()));
        }

        // Check that PCA has been run (coords are set)
        if self.pca_model.is_none() {
            return Err(BootstrapError::NoData(
                "PCA model not trained - run PCA first".to_string()
            ));
        }

        let mut added = 0;

        for concept in self.concepts.values() {
            // Create token with concept's coordinates
            let token = Token::from_state_f32(concept.id, &[
                concept.coords[0],
                concept.coords[1],
                concept.coords[2],
                0.0, 0.0, 0.0, 0.0, 0.0, // Fill remaining with zeros
            ]);

            if let Ok(_) = self.grid.add(token) {
                added += 1;
            }
        }

        Ok(added)
    }

    /// Weave connections between concepts using Grid KNN
    ///
    /// For each concept, finds K nearest neighbors and creates edges
    ///
    /// # Returns
    /// Result with number of edges created
    pub fn weave_connections(&mut self) -> Result<usize, BootstrapError> {
        if self.concepts.is_empty() {
            return Err(BootstrapError::NoData("No concepts loaded".to_string()));
        }

        if self.pca_model.is_none() {
            return Err(BootstrapError::NoData(
                "PCA model not trained - run PCA first".to_string()
            ));
        }

        let mut edges_created = 0;
        let k = self.config.knn_k;
        let decay = self.config.connection_decay;

        // For each concept, find KNN and create edges
        for concept in self.concepts.values() {
            // Find K nearest neighbors using Grid
            // Use large radius to get all neighbors, then limit by max_results
            let neighbors = self.grid.find_neighbors(
                concept.id,
                crate::CoordinateSpace::L1Physical, // Use L1 physical coordinate space
                100.0, // Large radius to include all
                k + 1, // +1 to exclude self potentially
            );

            // Create edges to neighbors
            for (i, &(neighbor_id, distance)) in neighbors.iter().enumerate() {
                // Skip self
                if neighbor_id == concept.id {
                    continue;
                }

                // Calculate weight based on distance
                // Closer neighbors (smaller distance) get higher weight
                let weight = 1.0 / (1.0 + distance * decay);

                // Create bidirectional edge
                let edge_id = crate::Graph::compute_edge_id(concept.id, neighbor_id, 0);

                if let Ok(_) = self.graph.add_edge(
                    edge_id,
                    concept.id,
                    neighbor_id,
                    0, // layer
                    weight,
                    false, // not directed
                ) {
                    edges_created += 1;
                }
            }
        }

        Ok(edges_created)
    }

    /// Complete bootstrap pipeline: load → PCA → populate → weave
    ///
    /// # Arguments
    /// * `embeddings_path` - Path to embeddings file
    ///
    /// # Returns
    /// Result with (num_concepts, num_edges)
    pub fn bootstrap_from_embeddings<P: AsRef<Path>>(
        &mut self,
        embeddings_path: P,
    ) -> Result<(usize, usize), BootstrapError> {
        // Load embeddings
        let loaded = self.load_embeddings(embeddings_path)?;

        // Run PCA
        let (_variance, _projected) = self.run_pca_pipeline()?;

        // Populate graph
        let _nodes = self.populate_graph()?;

        // Populate grid
        let _grid_items = self.populate_grid()?;

        // Weave connections
        let edges = self.weave_connections()?;

        Ok((loaded, edges))
    }
}

// ============================================================================
// Multimodal Anchors
// ============================================================================

impl BootstrapLibrary {
    /// Enrich concepts with color information
    ///
    /// Adds RGB color values to concepts that represent colors or have strong color associations
    ///
    /// # Returns
    /// Number of concepts enriched with color
    pub fn add_color_anchors(&mut self) -> usize {
        let color_map = Self::get_color_lexicon();
        let mut enriched = 0;

        for concept in self.concepts.values_mut() {
            if let Some(&color) = color_map.get(concept.word.as_str()) {
                concept.color = Some(color);
                enriched += 1;
            }
        }

        enriched
    }

    /// Enrich concepts with emotion information
    ///
    /// Adds VAD (Valence-Arousal-Dominance) values to emotion-related concepts
    ///
    /// # Returns
    /// Number of concepts enriched with emotion
    pub fn add_emotion_anchors(&mut self) -> usize {
        let emotion_map = Self::get_emotion_lexicon();
        let mut enriched = 0;

        for concept in self.concepts.values_mut() {
            if let Some(&emotion) = emotion_map.get(concept.word.as_str()) {
                concept.emotion = Some(emotion);
                enriched += 1;
            }
        }

        enriched
    }

    /// Get color lexicon mapping words to RGB values
    ///
    /// Returns HashMap of color words to normalized RGB [0.0-1.0]
    fn get_color_lexicon() -> HashMap<&'static str, [f32; 3]> {
        let mut map = HashMap::new();

        // Basic colors
        map.insert("red", [1.0, 0.0, 0.0]);
        map.insert("green", [0.0, 1.0, 0.0]);
        map.insert("blue", [0.0, 0.0, 1.0]);
        map.insert("yellow", [1.0, 1.0, 0.0]);
        map.insert("cyan", [0.0, 1.0, 1.0]);
        map.insert("magenta", [1.0, 0.0, 1.0]);
        map.insert("white", [1.0, 1.0, 1.0]);
        map.insert("black", [0.0, 0.0, 0.0]);
        map.insert("gray", [0.5, 0.5, 0.5]);
        map.insert("grey", [0.5, 0.5, 0.5]);

        // Extended colors
        map.insert("orange", [1.0, 0.65, 0.0]);
        map.insert("purple", [0.5, 0.0, 0.5]);
        map.insert("pink", [1.0, 0.75, 0.8]);
        map.insert("brown", [0.65, 0.16, 0.16]);
        map.insert("violet", [0.93, 0.51, 0.93]);
        map.insert("indigo", [0.29, 0.0, 0.51]);
        map.insert("turquoise", [0.25, 0.88, 0.82]);
        map.insert("gold", [1.0, 0.84, 0.0]);
        map.insert("silver", [0.75, 0.75, 0.75]);
        map.insert("beige", [0.96, 0.96, 0.86]);

        // Nature-associated colors
        map.insert("sky", [0.53, 0.81, 0.92]);
        map.insert("ocean", [0.0, 0.5, 1.0]);
        map.insert("grass", [0.0, 0.8, 0.0]);
        map.insert("forest", [0.13, 0.55, 0.13]);
        map.insert("sun", [1.0, 1.0, 0.0]);
        map.insert("fire", [1.0, 0.27, 0.0]);
        map.insert("blood", [0.72, 0.0, 0.0]);

        map
    }

    /// Get emotion lexicon mapping words to VAD (Valence-Arousal-Dominance)
    ///
    /// Returns HashMap of emotion words to VAD values [-1.0 to 1.0]
    /// - Valence: negative to positive
    /// - Arousal: calm to excited
    /// - Dominance: submissive to dominant
    fn get_emotion_lexicon() -> HashMap<&'static str, [f32; 3]> {
        let mut map = HashMap::new();

        // Basic emotions (Ekman's 6)
        map.insert("happy", [0.8, 0.6, 0.5]);      // positive, energized, moderate control
        map.insert("sad", [-0.7, -0.5, -0.4]);     // negative, low energy, low control
        map.insert("angry", [-0.6, 0.8, 0.7]);     // negative, high arousal, dominant
        map.insert("fear", [-0.8, 0.7, -0.6]);     // very negative, high arousal, submissive
        map.insert("disgust", [-0.7, 0.3, 0.3]);   // negative, moderate arousal
        map.insert("surprise", [0.2, 0.8, 0.0]);   // slightly positive, high arousal, neutral

        // Extended emotions
        map.insert("joy", [0.9, 0.7, 0.6]);
        map.insert("love", [0.9, 0.4, 0.4]);
        map.insert("hate", [-0.8, 0.7, 0.5]);
        map.insert("anxiety", [-0.6, 0.7, -0.5]);
        map.insert("calm", [0.5, -0.8, 0.2]);
        map.insert("excited", [0.7, 0.9, 0.5]);
        map.insert("bored", [-0.3, -0.7, -0.3]);
        map.insert("proud", [0.7, 0.4, 0.8]);
        map.insert("shame", [-0.6, 0.3, -0.7]);
        map.insert("guilt", [-0.7, 0.4, -0.6]);

        // Valence variations
        map.insert("depressed", [-0.9, -0.6, -0.7]);
        map.insert("cheerful", [0.8, 0.5, 0.5]);
        map.insert("melancholy", [-0.5, -0.4, -0.3]);
        map.insert("ecstatic", [1.0, 0.9, 0.7]);
        map.insert("miserable", [-0.9, -0.5, -0.6]);
        map.insert("content", [0.6, -0.2, 0.3]);
        map.insert("satisfied", [0.7, 0.0, 0.5]);

        // Arousal variations
        map.insert("relaxed", [0.6, -0.7, 0.3]);
        map.insert("tense", [-0.4, 0.8, -0.2]);
        map.insert("alert", [0.3, 0.8, 0.6]);
        map.insert("tired", [-0.2, -0.8, -0.4]);

        // Social emotions
        map.insert("lonely", [-0.6, -0.3, -0.5]);
        map.insert("grateful", [0.8, 0.2, 0.3]);
        map.insert("jealous", [-0.5, 0.6, 0.0]);
        map.insert("envious", [-0.4, 0.5, -0.2]);

        map
    }

    /// Complete multimodal enrichment: add colors and emotions
    ///
    /// # Returns
    /// (colors_added, emotions_added)
    pub fn enrich_multimodal(&mut self) -> (usize, usize) {
        let colors = self.add_color_anchors();
        let emotions = self.add_emotion_anchors();
        (colors, emotions)
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

    #[test]
    fn test_populate_graph() {
        use std::io::Write;
        use std::fs::File;

        let temp_path = "/tmp/test_populate.txt";
        let mut file = File::create(temp_path).unwrap();
        for i in 0..5 {
            writeln!(file, "word{} 0.1 0.2 0.3", i).unwrap();
        }

        let mut config = BootstrapConfig::default();
        config.embedding_dim = 3;
        config.target_dim = 3;

        let mut bootstrap = BootstrapLibrary::new(config);
        bootstrap.load_embeddings(temp_path).unwrap();
        bootstrap.run_pca_pipeline().unwrap();

        let nodes_added = bootstrap.populate_graph().unwrap();
        assert_eq!(nodes_added, 5);

        // Check that graph has nodes
        assert_eq!(bootstrap.graph().node_count(), 5);

        std::fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_complete_pipeline() {
        use std::io::Write;
        use std::fs::File;

        let temp_path = "/tmp/test_complete.txt";
        let mut file = File::create(temp_path).unwrap();

        // Create 10 words with distinct 5D embeddings
        for i in 0..10 {
            let v1 = (i as f32) * 0.1;
            let v2 = (i as f32) * 0.2;
            let v3 = (i as f32) * 0.05;
            let v4 = (i as f32) * -0.1;
            let v5 = (i as f32) * 0.15;
            writeln!(file, "word{} {} {} {} {} {}", i, v1, v2, v3, v4, v5).unwrap();
        }

        let mut config = BootstrapConfig::default();
        config.embedding_dim = 5;
        config.target_dim = 3;
        config.knn_k = 3; // Each node connects to 3 neighbors

        let mut bootstrap = BootstrapLibrary::new(config);
        let (concepts, edges) = bootstrap.bootstrap_from_embeddings(temp_path).unwrap();

        assert_eq!(concepts, 10);
        assert!(edges > 0, "Should have created edges");

        // Check graph state
        assert_eq!(bootstrap.graph().node_count(), 10);
        assert_eq!(bootstrap.graph().edge_count(), edges);

        std::fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_weave_connections_weights() {
        use std::io::Write;
        use std::fs::File;

        let temp_path = "/tmp/test_weights.txt";
        let mut file = File::create(temp_path).unwrap();

        // Create 5 words in a line (should connect to nearest neighbors)
        for i in 0..5 {
            writeln!(file, "word{} {} 0.0 0.0", i, i as f32).unwrap();
        }

        let mut config = BootstrapConfig::default();
        config.embedding_dim = 3;
        config.target_dim = 3;
        config.knn_k = 2;
        config.connection_decay = 0.1;

        let mut bootstrap = BootstrapLibrary::new(config);
        bootstrap.load_embeddings(temp_path).unwrap();
        bootstrap.run_pca_pipeline().unwrap();
        bootstrap.populate_graph().unwrap();
        bootstrap.populate_grid().unwrap();

        let edges = bootstrap.weave_connections().unwrap();

        assert!(edges > 0, "Should create connections");
        assert_eq!(bootstrap.graph().edge_count(), edges);

        std::fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_color_anchors() {
        use std::io::Write;
        use std::fs::File;

        let temp_path = "/tmp/test_colors.txt";
        let mut file = File::create(temp_path).unwrap();

        // Include color words
        writeln!(file, "red 0.1 0.2 0.3").unwrap();
        writeln!(file, "green 0.4 0.5 0.6").unwrap();
        writeln!(file, "blue 0.7 0.8 0.9").unwrap();
        writeln!(file, "cat 0.1 0.1 0.1").unwrap(); // non-color word

        let mut config = BootstrapConfig::default();
        config.embedding_dim = 3;

        let mut bootstrap = BootstrapLibrary::new(config);
        bootstrap.load_embeddings(temp_path).unwrap();

        let enriched = bootstrap.add_color_anchors();

        assert_eq!(enriched, 3, "Should enrich 3 color words");

        // Check that color was added
        let red_concept = bootstrap.get_concept("red").unwrap();
        assert!(red_concept.color.is_some());
        assert_eq!(red_concept.color.unwrap(), [1.0, 0.0, 0.0]);

        // Check that non-color word has no color
        let cat_concept = bootstrap.get_concept("cat").unwrap();
        assert!(cat_concept.color.is_none());

        std::fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_emotion_anchors() {
        use std::io::Write;
        use std::fs::File;

        let temp_path = "/tmp/test_emotions.txt";
        let mut file = File::create(temp_path).unwrap();

        // Include emotion words
        writeln!(file, "happy 0.1 0.2 0.3").unwrap();
        writeln!(file, "sad 0.4 0.5 0.6").unwrap();
        writeln!(file, "angry 0.7 0.8 0.9").unwrap();
        writeln!(file, "table 0.1 0.1 0.1").unwrap(); // non-emotion word

        let mut config = BootstrapConfig::default();
        config.embedding_dim = 3;

        let mut bootstrap = BootstrapLibrary::new(config);
        bootstrap.load_embeddings(temp_path).unwrap();

        let enriched = bootstrap.add_emotion_anchors();

        assert_eq!(enriched, 3, "Should enrich 3 emotion words");

        // Check that emotion was added (VAD values)
        let happy_concept = bootstrap.get_concept("happy").unwrap();
        assert!(happy_concept.emotion.is_some());
        let vad = happy_concept.emotion.unwrap();
        assert!(vad[0] > 0.0, "Happy should have positive valence");
        assert!(vad[1] > 0.0, "Happy should have positive arousal");

        // Check that non-emotion word has no emotion
        let table_concept = bootstrap.get_concept("table").unwrap();
        assert!(table_concept.emotion.is_none());

        std::fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_multimodal_enrichment() {
        use std::io::Write;
        use std::fs::File;

        let temp_path = "/tmp/test_multimodal.txt";
        let mut file = File::create(temp_path).unwrap();

        // Mix of color, emotion, and neutral words
        writeln!(file, "red 0.1 0.2 0.3").unwrap();
        writeln!(file, "happy 0.4 0.5 0.6").unwrap();
        writeln!(file, "blue 0.7 0.8 0.9").unwrap();
        writeln!(file, "sad 0.2 0.3 0.4").unwrap();
        writeln!(file, "table 0.1 0.1 0.1").unwrap();

        let mut config = BootstrapConfig::default();
        config.embedding_dim = 3;

        let mut bootstrap = BootstrapLibrary::new(config);
        bootstrap.load_embeddings(temp_path).unwrap();

        let (colors, emotions) = bootstrap.enrich_multimodal();

        assert_eq!(colors, 2, "Should enrich 2 color words");
        assert_eq!(emotions, 2, "Should enrich 2 emotion words");

        // Verify mixed enrichment
        let red_concept = bootstrap.get_concept("red").unwrap();
        assert!(red_concept.color.is_some());
        assert!(red_concept.emotion.is_none());

        let happy_concept = bootstrap.get_concept("happy").unwrap();
        assert!(happy_concept.color.is_none());
        assert!(happy_concept.emotion.is_some());

        let table_concept = bootstrap.get_concept("table").unwrap();
        assert!(table_concept.color.is_none());
        assert!(table_concept.emotion.is_none());

        std::fs::remove_file(temp_path).ok();
    }
}
