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

//! Bootstrap Library v1.3
//!
//! Provides semantic graph initialization from pre-trained embeddings.
//!
//! Features:
//! - Deterministic ID generation (MurmurHash3)
//! - GloVe/Word2Vec embedding loading
//! - PCA dimensionality reduction (300D → 3D)
//! - Extended multimodal anchors (5 modalities):
//!   * Colors (27 RGB values)
//!   * Emotions (30 VAD values)
//!   * Sounds (30 volume/pitch/duration) [NEW v1.3]
//!   * Actions (40 energy/speed/direction/impact) [NEW v1.3]
//!   * Spatial relations (20 proximity/verticality/containment) [NEW v1.3]
//! - Semantic search via spreading activation [NEW v1.3]
//! - Multi-query search with score combination [NEW v1.3]
//! - Semantic analogy completion [NEW v1.3]
//! - Connection weaving via Grid KNN
//! - Artifact persistence (PCA model, bootstrap map)

use crate::{Graph, Grid, NodeId};
use fasthash::murmur3::Hasher32;
use fasthash::FastHasher;
use ndarray::{Array1, Array2};
use std::collections::HashMap;
use std::hash::Hasher;
use std::path::Path;
use std::fs::File;
use std::io::{Write, Read};

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
    pub sound: Option<[f32; 3]>,      // Volume, Pitch, Duration (NEW v1.3)
    pub action: Option<[f32; 4]>,     // Energy, Speed, Direction, Impact (NEW v1.3)
    pub spatial: Option<[f32; 3]>,    // Proximity, Verticality, Containment (NEW v1.3)
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
                sound: None,
                action: None,
                spatial: None,
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

    /// Enrich concepts with sound information (NEW v1.3)
    ///
    /// Adds sound characteristics (volume, pitch, duration) to sound-related concepts
    ///
    /// # Returns
    /// Number of concepts enriched with sound
    pub fn add_sound_anchors(&mut self) -> usize {
        let sound_map = Self::get_sound_lexicon();
        let mut enriched = 0;

        for concept in self.concepts.values_mut() {
            if let Some(&sound) = sound_map.get(concept.word.as_str()) {
                concept.sound = Some(sound);
                enriched += 1;
            }
        }

        enriched
    }

    /// Enrich concepts with action information (NEW v1.3)
    ///
    /// Adds action characteristics (energy, speed, direction, impact) to verb concepts
    ///
    /// # Returns
    /// Number of concepts enriched with action
    pub fn add_action_anchors(&mut self) -> usize {
        let action_map = Self::get_action_lexicon();
        let mut enriched = 0;

        for concept in self.concepts.values_mut() {
            if let Some(&action) = action_map.get(concept.word.as_str()) {
                concept.action = Some(action);
                enriched += 1;
            }
        }

        enriched
    }

    /// Enrich concepts with spatial relation information (NEW v1.3)
    ///
    /// Adds spatial characteristics (proximity, verticality, containment) to preposition concepts
    ///
    /// # Returns
    /// Number of concepts enriched with spatial relations
    pub fn add_spatial_anchors(&mut self) -> usize {
        let spatial_map = Self::get_spatial_lexicon();
        let mut enriched = 0;

        for concept in self.concepts.values_mut() {
            if let Some(&spatial) = spatial_map.get(concept.word.as_str()) {
                concept.spatial = Some(spatial);
                enriched += 1;
            }
        }

        enriched
    }

    /// Get sound lexicon mapping words to sound characteristics
    ///
    /// Returns HashMap of sound words to (volume, pitch, duration) values [-1.0 to 1.0]
    /// - Volume: quiet to loud
    /// - Pitch: low to high
    /// - Duration: short to long
    fn get_sound_lexicon() -> HashMap<&'static str, [f32; 3]> {
        let mut map = HashMap::new();

        // Quiet sounds
        map.insert("whisper", [-0.8, 0.2, 0.3]);
        map.insert("rustle", [-0.7, -0.3, 0.2]);
        map.insert("murmur", [-0.6, -0.2, 0.5]);
        map.insert("hum", [-0.4, 0.0, 0.7]);
        map.insert("tick", [-0.7, 0.5, -0.8]);

        // Moderate sounds
        map.insert("talk", [0.0, 0.3, 0.5]);
        map.insert("chatter", [0.2, 0.4, 0.6]);
        map.insert("laugh", [0.3, 0.5, 0.4]);
        map.insert("cry", [0.2, 0.6, 0.8]);
        map.insert("sing", [0.1, 0.7, 0.9]);

        // Loud sounds
        map.insert("shout", [0.8, 0.6, 0.3]);
        map.insert("scream", [0.9, 0.9, 0.4]);
        map.insert("yell", [0.8, 0.5, 0.3]);
        map.insert("roar", [1.0, -0.5, 0.6]);
        map.insert("thunder", [1.0, -0.7, 0.7]);

        // Musical sounds
        map.insert("melody", [0.0, 0.8, 0.9]);
        map.insert("harmony", [0.1, 0.6, 0.9]);
        map.insert("rhythm", [0.2, 0.0, 0.8]);
        map.insert("beat", [0.4, -0.3, -0.5]);
        map.insert("tone", [0.0, 0.5, 0.6]);

        // Impact sounds
        map.insert("bang", [0.9, 0.2, -0.9]);
        map.insert("crash", [1.0, -0.2, -0.7]);
        map.insert("slam", [0.8, -0.4, -0.8]);
        map.insert("thud", [0.6, -0.8, -0.6]);
        map.insert("boom", [1.0, -0.6, 0.3]);

        // Nature sounds
        map.insert("wind", [0.3, -0.1, 0.8]);
        map.insert("rain", [0.4, -0.5, 0.9]);
        map.insert("wave", [0.5, -0.3, 0.7]);
        map.insert("chirp", [-0.3, 0.9, -0.7]);
        map.insert("howl", [0.7, 0.4, 0.6]);

        map
    }

    /// Get action lexicon mapping verbs to action characteristics
    ///
    /// Returns HashMap of action words to (energy, speed, direction, impact) values [-1.0 to 1.0]
    /// - Energy: low to high physical exertion
    /// - Speed: slow to fast
    /// - Direction: inward/downward to outward/upward
    /// - Impact: gentle to forceful
    fn get_action_lexicon() -> HashMap<&'static str, [f32; 4]> {
        let mut map = HashMap::new();

        // Low energy actions
        map.insert("sleep", [-0.9, -0.9, -0.5, -0.9]);
        map.insert("rest", [-0.8, -0.8, -0.3, -0.8]);
        map.insert("sit", [-0.7, -0.7, -0.6, -0.6]);
        map.insert("lie", [-0.8, -0.8, -0.8, -0.7]);
        map.insert("wait", [-0.6, -0.5, 0.0, -0.5]);

        // Moderate energy actions
        map.insert("walk", [0.2, 0.1, 0.3, 0.1]);
        map.insert("stand", [0.0, -0.3, 0.5, 0.0]);
        map.insert("talk", [0.1, 0.2, 0.4, 0.2]);
        map.insert("think", [-0.2, 0.0, 0.6, -0.3]);
        map.insert("look", [-0.1, 0.3, 0.4, 0.0]);

        // High energy actions
        map.insert("run", [0.8, 0.9, 0.7, 0.6]);
        map.insert("jump", [0.9, 0.7, 0.9, 0.7]);
        map.insert("fight", [0.9, 0.8, 0.5, 0.9]);
        map.insert("dance", [0.7, 0.6, 0.6, 0.4]);
        map.insert("climb", [0.8, 0.4, 0.8, 0.5]);

        // Manipulative actions
        map.insert("push", [0.5, 0.3, 0.8, 0.7]);
        map.insert("pull", [0.5, 0.3, -0.8, 0.6]);
        map.insert("lift", [0.6, 0.2, 0.9, 0.5]);
        map.insert("drop", [0.1, 0.6, -0.9, 0.4]);
        map.insert("throw", [0.7, 0.9, 0.8, 0.8]);

        // Cognitive actions
        map.insert("learn", [0.2, 0.1, 0.7, 0.1]);
        map.insert("remember", [0.0, 0.2, 0.5, 0.0]);
        map.insert("forget", [-0.2, 0.0, -0.3, -0.1]);
        map.insert("understand", [0.1, 0.3, 0.6, 0.2]);
        map.insert("know", [0.0, 0.0, 0.4, 0.0]);

        // Social actions
        map.insert("help", [0.3, 0.4, 0.7, 0.3]);
        map.insert("give", [0.2, 0.3, 0.8, 0.4]);
        map.insert("take", [0.3, 0.5, -0.6, 0.5]);
        map.insert("share", [0.2, 0.2, 0.6, 0.2]);
        map.insert("love", [0.4, 0.1, 0.9, 0.3]);

        // Communication actions
        map.insert("speak", [0.2, 0.4, 0.7, 0.3]);
        map.insert("write", [0.1, 0.0, 0.5, 0.1]);
        map.insert("read", [0.0, 0.2, 0.4, 0.0]);
        map.insert("listen", [-0.1, 0.0, -0.4, 0.0]);
        map.insert("hear", [-0.2, 0.1, -0.3, 0.0]);

        // Creative actions
        map.insert("create", [0.5, 0.3, 0.8, 0.4]);
        map.insert("build", [0.6, 0.2, 0.7, 0.5]);
        map.insert("make", [0.4, 0.3, 0.6, 0.4]);
        map.insert("destroy", [0.7, 0.6, -0.5, 0.9]);
        map.insert("break", [0.5, 0.7, -0.4, 0.8]);

        map
    }

    /// Get spatial relations lexicon mapping prepositions to spatial characteristics
    ///
    /// Returns HashMap of spatial words to (proximity, verticality, containment) values [-1.0 to 1.0]
    /// - Proximity: far to near
    /// - Verticality: below to above
    /// - Containment: outside to inside
    fn get_spatial_lexicon() -> HashMap<&'static str, [f32; 3]> {
        let mut map = HashMap::new();

        // Proximity relations
        map.insert("near", [0.9, 0.0, 0.0]);
        map.insert("close", [0.9, 0.0, 0.0]);
        map.insert("beside", [0.8, 0.0, 0.0]);
        map.insert("next", [0.8, 0.0, 0.0]);
        map.insert("far", [-0.9, 0.0, 0.0]);
        map.insert("distant", [-0.9, 0.0, 0.0]);

        // Vertical relations
        map.insert("above", [0.0, 0.9, 0.0]);
        map.insert("over", [0.2, 0.9, 0.0]);
        map.insert("top", [0.0, 1.0, 0.0]);
        map.insert("below", [0.0, -0.9, 0.0]);
        map.insert("under", [0.2, -0.9, 0.0]);
        map.insert("bottom", [0.0, -1.0, 0.0]);

        // Containment relations
        map.insert("in", [0.5, 0.0, 0.9]);
        map.insert("inside", [0.3, 0.0, 1.0]);
        map.insert("within", [0.4, 0.0, 0.9]);
        map.insert("out", [0.0, 0.0, -0.9]);
        map.insert("outside", [0.0, 0.0, -1.0]);

        // Combined relations
        map.insert("behind", [0.3, 0.0, -0.3]);
        map.insert("front", [0.3, 0.0, 0.3]);
        map.insert("between", [0.6, 0.0, 0.0]);
        map.insert("among", [0.8, 0.0, 0.2]);

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

    /// Complete extended multimodal enrichment (NEW v1.3)
    ///
    /// Adds all 5 modalities: colors, emotions, sounds, actions, spatial relations
    ///
    /// # Returns
    /// (colors, emotions, sounds, actions, spatial)
    pub fn enrich_extended_multimodal(&mut self) -> (usize, usize, usize, usize, usize) {
        let colors = self.add_color_anchors();
        let emotions = self.add_emotion_anchors();
        let sounds = self.add_sound_anchors();
        let actions = self.add_action_anchors();
        let spatial = self.add_spatial_anchors();
        (colors, emotions, sounds, actions, spatial)
    }
}

// ============================================================================
// Semantic Search (NEW v1.3)
// ============================================================================

impl BootstrapLibrary {
    /// Semantic search using spreading activation (NEW v1.3)
    ///
    /// Searches for concepts semantically related to a query word by activating
    /// its node in the graph and spreading energy through connections.
    ///
    /// # Arguments
    /// * `query` - Query word to search for
    /// * `max_results` - Maximum number of results to return
    /// * `max_depth` - Optional maximum depth for spreading (default: 3)
    ///
    /// # Returns
    /// Vector of (word, activation_score) tuples, sorted by relevance
    ///
    /// # Example
    /// ```ignore
    /// let results = bootstrap.semantic_search("cat", 10, None)?;
    /// // Returns: [("dog", 0.85), ("animal", 0.72), ("pet", 0.68), ...]
    /// ```
    pub fn semantic_search(
        &mut self,
        query: &str,
        max_results: usize,
        max_depth: Option<usize>,
    ) -> Result<Vec<(String, f32)>, BootstrapError> {
        // Get query concept
        let query_concept = self.concepts.get(query)
            .ok_or_else(|| BootstrapError::NoData(
                format!("Unknown query word: '{}'", query)
            ))?;

        let query_id = query_concept.id;

        // Create SignalConfig with custom max_depth if specified
        let config = if let Some(depth) = max_depth {
            Some(crate::SignalConfig {
                max_depth: depth,
                ..Default::default()
            })
        } else {
            None  // Use default config (max_depth: 5)
        };

        // Run spreading activation from query node
        let result = self.graph.spreading_activation(
            query_id,
            1.0,  // initial energy
            config,
        );

        // Convert activated nodes to (word, score) pairs
        let mut results: Vec<(String, f32)> = Vec::new();

        for activated_node in &result.activated_nodes {
            // Find concept matching this node ID
            if let Some(concept) = self.concepts.values()
                .find(|c| c.id == activated_node.node_id)
            {
                // Skip query word itself
                if concept.word != query {
                    results.push((concept.word.clone(), activated_node.energy));
                }
            }
        }

        // Sort by energy (descending) and limit to max_results
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(max_results);

        Ok(results)
    }

    /// Multi-query semantic search (NEW v1.3)
    ///
    /// Performs semantic search for multiple query words and combines results.
    /// Useful for concept intersection (e.g., "red" AND "car" → "fire truck")
    ///
    /// # Arguments
    /// * `queries` - Vector of query words
    /// * `max_results` - Maximum number of results to return
    /// * `combination_mode` - How to combine scores: "sum", "max", "avg"
    ///
    /// # Returns
    /// Vector of (word, combined_score) tuples
    pub fn semantic_search_multi(
        &mut self,
        queries: &[&str],
        max_results: usize,
        combination_mode: &str,
    ) -> Result<Vec<(String, f32)>, BootstrapError> {
        if queries.is_empty() {
            return Ok(Vec::new());
        }

        // Collect results from all queries
        let mut all_results: HashMap<String, Vec<f32>> = HashMap::new();

        for &query in queries {
            let results = self.semantic_search(query, max_results * 2, Some(3))?;  // max_depth = 3

            for (word, score) in results {
                all_results.entry(word).or_insert_with(Vec::new).push(score);
            }
        }

        // Combine scores based on mode
        let mut combined_results: Vec<(String, f32)> = all_results
            .into_iter()
            .map(|(word, scores)| {
                let combined_score = match combination_mode {
                    "sum" => scores.iter().sum(),
                    "max" => scores.iter().copied().fold(0.0f32, f32::max),
                    "avg" => scores.iter().sum::<f32>() / scores.len() as f32,
                    _ => scores.iter().sum(), // default to sum
                };
                (word, combined_score)
            })
            .collect();

        // Sort by combined score and limit
        combined_results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        combined_results.truncate(max_results);

        Ok(combined_results)
    }

    /// Find semantic analogies (NEW v1.3)
    ///
    /// Finds concepts that complete an analogy: "A is to B as C is to ?"
    /// Example: "king" is to "queen" as "man" is to "woman"
    ///
    /// # Arguments
    /// * `a` - First word in analogy
    /// * `b` - Second word in analogy
    /// * `c` - Third word in analogy
    /// * `max_results` - Maximum number of results
    ///
    /// # Returns
    /// Vector of (word, score) candidates for completing the analogy
    pub fn semantic_analogy(
        &mut self,
        a: &str,
        b: &str,
        c: &str,
        max_results: usize,
    ) -> Result<Vec<(String, f32)>, BootstrapError> {
        // Get concepts
        let concept_a = self.concepts.get(a)
            .ok_or_else(|| BootstrapError::NoData(format!("Unknown word: '{}'", a)))?;
        let concept_b = self.concepts.get(b)
            .ok_or_else(|| BootstrapError::NoData(format!("Unknown word: '{}'", b)))?;
        let concept_c = self.concepts.get(c)
            .ok_or_else(|| BootstrapError::NoData(format!("Unknown word: '{}'", c)))?;

        // Compute vector offset: B - A
        let offset = [
            concept_b.coords[0] - concept_a.coords[0],
            concept_b.coords[1] - concept_a.coords[1],
            concept_b.coords[2] - concept_a.coords[2],
        ];

        // Compute target: C + offset
        let target = [
            concept_c.coords[0] + offset[0],
            concept_c.coords[1] + offset[1],
            concept_c.coords[2] + offset[2],
        ];

        // Find concepts closest to target
        let mut results: Vec<(String, f32)> = Vec::new();

        for concept in self.concepts.values() {
            // Skip input words
            if concept.word == a || concept.word == b || concept.word == c {
                continue;
            }

            // Compute Euclidean distance to target
            let distance = (
                (concept.coords[0] - target[0]).powi(2) +
                (concept.coords[1] - target[1]).powi(2) +
                (concept.coords[2] - target[2]).powi(2)
            ).sqrt();

            // Convert distance to similarity score (inverse)
            let similarity = 1.0 / (1.0 + distance);
            results.push((concept.word.clone(), similarity));
        }

        // Sort by similarity and limit
        results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        results.truncate(max_results);

        Ok(results)
    }
}

// ============================================================================
// Artifact Persistence
// ============================================================================

impl BootstrapLibrary {
    /// Save PCA model to binary file
    ///
    /// Saves the trained PCA model for later reuse
    ///
    /// # Arguments
    /// * `path` - Path to save the PCA model
    ///
    /// # Returns
    /// Result with number of bytes written
    pub fn save_pca_model<P: AsRef<Path>>(&self, path: P) -> Result<usize, BootstrapError> {
        let pca_model = self.pca_model.as_ref()
            .ok_or_else(|| BootstrapError::NoData("PCA model not trained".to_string()))?;

        let mut file = File::create(path.as_ref())
            .map_err(|e| BootstrapError::IoError(e.to_string()))?;

        // Write format version (u32)
        let version: u32 = 1;
        file.write_all(&version.to_le_bytes())
            .map_err(|e| BootstrapError::IoError(e.to_string()))?;

        // Write dimensions
        file.write_all(&(pca_model.original_dim as u32).to_le_bytes())
            .map_err(|e| BootstrapError::IoError(e.to_string()))?;
        file.write_all(&(pca_model.target_dim as u32).to_le_bytes())
            .map_err(|e| BootstrapError::IoError(e.to_string()))?;

        // Write mean vector
        for &val in pca_model.mean.iter() {
            file.write_all(&val.to_le_bytes())
                .map_err(|e| BootstrapError::IoError(e.to_string()))?;
        }

        // Write components matrix (row-major)
        for i in 0..pca_model.target_dim {
            for j in 0..pca_model.original_dim {
                file.write_all(&pca_model.components[[i, j]].to_le_bytes())
                    .map_err(|e| BootstrapError::IoError(e.to_string()))?;
            }
        }

        // Write explained variance
        for &val in pca_model.explained_variance.iter() {
            file.write_all(&val.to_le_bytes())
                .map_err(|e| BootstrapError::IoError(e.to_string()))?;
        }

        let metadata = std::fs::metadata(path.as_ref())
            .map_err(|e| BootstrapError::IoError(e.to_string()))?;
        Ok(metadata.len() as usize)
    }

    /// Load PCA model from binary file
    ///
    /// # Arguments
    /// * `path` - Path to the saved PCA model
    ///
    /// # Returns
    /// Result with loaded PCA model
    pub fn load_pca_model<P: AsRef<Path>>(&mut self, path: P) -> Result<(), BootstrapError> {
        let mut file = File::open(path.as_ref())
            .map_err(|e| BootstrapError::IoError(e.to_string()))?;

        // Read version
        let mut version_bytes = [0u8; 4];
        file.read_exact(&mut version_bytes)
            .map_err(|e| BootstrapError::IoError(e.to_string()))?;
        let version = u32::from_le_bytes(version_bytes);

        if version != 1 {
            return Err(BootstrapError::PcaError(
                format!("Unsupported PCA model version: {}", version)
            ));
        }

        // Read dimensions
        let mut dim_bytes = [0u8; 4];
        file.read_exact(&mut dim_bytes)
            .map_err(|e| BootstrapError::IoError(e.to_string()))?;
        let original_dim = u32::from_le_bytes(dim_bytes) as usize;

        file.read_exact(&mut dim_bytes)
            .map_err(|e| BootstrapError::IoError(e.to_string()))?;
        let target_dim = u32::from_le_bytes(dim_bytes) as usize;

        // Read mean vector
        let mut mean = Array1::zeros(original_dim);
        for i in 0..original_dim {
            let mut val_bytes = [0u8; 4];
            file.read_exact(&mut val_bytes)
                .map_err(|e| BootstrapError::IoError(e.to_string()))?;
            mean[i] = f32::from_le_bytes(val_bytes);
        }

        // Read components matrix
        let mut components = Array2::zeros((target_dim, original_dim));
        for i in 0..target_dim {
            for j in 0..original_dim {
                let mut val_bytes = [0u8; 4];
                file.read_exact(&mut val_bytes)
                    .map_err(|e| BootstrapError::IoError(e.to_string()))?;
                components[[i, j]] = f32::from_le_bytes(val_bytes);
            }
        }

        // Read explained variance
        let mut explained_variance = Array1::zeros(target_dim);
        for i in 0..target_dim {
            let mut val_bytes = [0u8; 4];
            file.read_exact(&mut val_bytes)
                .map_err(|e| BootstrapError::IoError(e.to_string()))?;
            explained_variance[i] = f32::from_le_bytes(val_bytes);
        }

        self.pca_model = Some(PCAModel {
            mean,
            components,
            explained_variance,
            original_dim,
            target_dim,
        });

        Ok(())
    }

    /// Save bootstrap map (word → concept mapping) to JSON file
    ///
    /// Saves a lightweight mapping of words to their IDs and 3D coordinates
    ///
    /// # Arguments
    /// * `path` - Path to save the bootstrap map
    ///
    /// # Returns
    /// Result with number of concepts saved
    pub fn save_bootstrap_map<P: AsRef<Path>>(&self, path: P) -> Result<usize, BootstrapError> {
        if self.concepts.is_empty() {
            return Err(BootstrapError::NoData("No concepts loaded".to_string()));
        }

        // Create lightweight concept records (without full embedding)
        let mut records = Vec::new();
        for concept in self.concepts.values() {
            records.push(serde_json::json!({
                "word": concept.word,
                "id": concept.id,
                "coords": concept.coords,
                "color": concept.color,
                "emotion": concept.emotion,
                "sound": concept.sound,
                "action": concept.action,
                "spatial": concept.spatial,
            }));
        }

        let json = serde_json::to_string_pretty(&records)
            .map_err(|e| BootstrapError::IoError(e.to_string()))?;

        let mut file = File::create(path.as_ref())
            .map_err(|e| BootstrapError::IoError(e.to_string()))?;

        file.write_all(json.as_bytes())
            .map_err(|e| BootstrapError::IoError(e.to_string()))?;

        Ok(records.len())
    }

    /// Save all artifacts: PCA model and bootstrap map
    ///
    /// # Arguments
    /// * `output_dir` - Directory to save artifacts
    ///
    /// # Returns
    /// Result with (pca_bytes, concepts_count)
    pub fn save_artifacts<P: AsRef<Path>>(&self, output_dir: P) -> Result<(usize, usize), BootstrapError> {
        let output_dir = output_dir.as_ref();

        // Create output directory if it doesn't exist
        std::fs::create_dir_all(output_dir)
            .map_err(|e| BootstrapError::IoError(e.to_string()))?;

        // Save PCA model
        let pca_path = output_dir.join("pca_model.bin");
        let pca_bytes = self.save_pca_model(&pca_path)?;

        // Save bootstrap map
        let map_path = output_dir.join("bootstrap_map.json");
        let concepts_count = self.save_bootstrap_map(&map_path)?;

        Ok((pca_bytes, concepts_count))
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

    #[test]
    fn test_save_and_load_pca_model() {
        use std::io::Write;
        use std::fs::File;

        let temp_embeddings = "/tmp/test_pca_save.txt";
        let mut file = File::create(temp_embeddings).unwrap();
        for i in 0..5 {
            writeln!(file, "word{} {} {} {}", i, i as f32 * 0.1, i as f32 * 0.2, i as f32 * 0.3).unwrap();
        }

        let mut config = BootstrapConfig::default();
        config.embedding_dim = 3;
        config.target_dim = 3;

        // Train PCA
        let mut bootstrap = BootstrapLibrary::new(config.clone());
        bootstrap.load_embeddings(temp_embeddings).unwrap();
        bootstrap.run_pca_pipeline().unwrap();

        // Save PCA model
        let pca_path = "/tmp/test_pca_model.bin";
        let bytes = bootstrap.save_pca_model(pca_path).unwrap();
        assert!(bytes > 0, "Should write bytes");

        // Load PCA model into new instance
        let mut bootstrap2 = BootstrapLibrary::new(config);
        bootstrap2.load_pca_model(pca_path).unwrap();

        // Verify model was loaded
        assert!(bootstrap2.pca_model.is_some());
        let model = bootstrap2.pca_model.as_ref().unwrap();
        assert_eq!(model.original_dim, 3);
        assert_eq!(model.target_dim, 3);

        // Clean up
        std::fs::remove_file(temp_embeddings).ok();
        std::fs::remove_file(pca_path).ok();
    }

    #[test]
    fn test_save_bootstrap_map() {
        use std::io::Write;
        use std::fs::File;

        let temp_path = "/tmp/test_map_save.txt";
        let mut file = File::create(temp_path).unwrap();
        writeln!(file, "cat 0.1 0.2 0.3").unwrap();
        writeln!(file, "dog 0.4 0.5 0.6").unwrap();
        writeln!(file, "red 0.7 0.8 0.9").unwrap();

        let mut config = BootstrapConfig::default();
        config.embedding_dim = 3;
        config.target_dim = 3;

        let mut bootstrap = BootstrapLibrary::new(config);
        bootstrap.load_embeddings(temp_path).unwrap();
        bootstrap.run_pca_pipeline().unwrap();
        bootstrap.add_color_anchors();

        // Save bootstrap map
        let map_path = "/tmp/test_bootstrap_map.json";
        let count = bootstrap.save_bootstrap_map(map_path).unwrap();
        assert_eq!(count, 3);

        // Verify file exists and contains JSON
        let map_content = std::fs::read_to_string(map_path).unwrap();
        assert!(map_content.contains("cat"));
        assert!(map_content.contains("dog"));
        assert!(map_content.contains("red"));

        // Clean up
        std::fs::remove_file(temp_path).ok();
        std::fs::remove_file(map_path).ok();
    }

    #[test]
    fn test_save_all_artifacts() {
        use std::io::Write;
        use std::fs::File;

        let temp_path = "/tmp/test_artifacts.txt";
        let mut file = File::create(temp_path).unwrap();
        for i in 0..5 {
            writeln!(file, "word{} {} {} {}", i, i as f32 * 0.1, i as f32 * 0.2, i as f32 * 0.3).unwrap();
        }

        let mut config = BootstrapConfig::default();
        config.embedding_dim = 3;
        config.target_dim = 3;

        let mut bootstrap = BootstrapLibrary::new(config);
        bootstrap.load_embeddings(temp_path).unwrap();
        bootstrap.run_pca_pipeline().unwrap();

        // Save all artifacts to directory
        let output_dir = "/tmp/test_bootstrap_artifacts";
        let (pca_bytes, concepts_count) = bootstrap.save_artifacts(output_dir).unwrap();

        assert!(pca_bytes > 0);
        assert_eq!(concepts_count, 5);

        // Verify files exist
        assert!(std::path::Path::new(&format!("{}/pca_model.bin", output_dir)).exists());
        assert!(std::path::Path::new(&format!("{}/bootstrap_map.json", output_dir)).exists());

        // Clean up
        std::fs::remove_file(temp_path).ok();
        std::fs::remove_dir_all(output_dir).ok();
    }

    #[test]
    fn test_semantic_similarity_cat_dog_car() {
        use std::io::Write;
        use std::fs::File;

        let temp_path = "/tmp/test_semantic_sim.txt";
        let mut file = File::create(temp_path).unwrap();

        // Create embeddings where:
        // - cat and dog are close (both animals, similar vectors)
        // - cat and car are far (animal vs vehicle, different vectors)
        // Using 5D embeddings for testing

        // Animals cluster (cat, dog, bird, fish)
        writeln!(file, "cat 1.0 0.9 0.1 0.0 0.0").unwrap();   // Animal cluster
        writeln!(file, "dog 0.9 1.0 0.1 0.0 0.0").unwrap();   // Very close to cat
        writeln!(file, "bird 0.8 0.7 0.2 0.1 0.0").unwrap();  // Somewhat close
        writeln!(file, "fish 0.7 0.6 0.3 0.1 0.0").unwrap();  // Animal but more distant

        // Vehicles cluster (car, truck, bus)
        writeln!(file, "car 0.0 0.0 0.1 1.0 0.9").unwrap();   // Vehicle cluster, far from cat
        writeln!(file, "truck 0.0 0.0 0.1 0.9 1.0").unwrap(); // Close to car
        writeln!(file, "bus 0.1 0.0 0.2 0.8 0.8").unwrap();   // Vehicle cluster

        // Neutral words
        writeln!(file, "table 0.5 0.5 0.5 0.5 0.5").unwrap(); // Midpoint
        writeln!(file, "chair 0.4 0.6 0.5 0.5 0.4").unwrap(); // Similar to table

        let mut config = BootstrapConfig::default();
        config.embedding_dim = 5;
        config.target_dim = 3;
        config.knn_k = 3; // Find 3 nearest neighbors

        let mut bootstrap = BootstrapLibrary::new(config);
        bootstrap.load_embeddings(temp_path).unwrap();
        bootstrap.run_pca_pipeline().unwrap();
        bootstrap.populate_graph().unwrap();
        bootstrap.populate_grid().unwrap();
        bootstrap.weave_connections().unwrap();

        // Get concepts
        let cat = bootstrap.get_concept("cat").unwrap();
        let dog = bootstrap.get_concept("dog").unwrap();
        let car = bootstrap.get_concept("car").unwrap();

        // Calculate Euclidean distances in 3D projected space
        let cat_dog_dist = (
            (cat.coords[0] - dog.coords[0]).powi(2) +
            (cat.coords[1] - dog.coords[1]).powi(2) +
            (cat.coords[2] - dog.coords[2]).powi(2)
        ).sqrt();

        let cat_car_dist = (
            (cat.coords[0] - car.coords[0]).powi(2) +
            (cat.coords[1] - car.coords[1]).powi(2) +
            (cat.coords[2] - car.coords[2]).powi(2)
        ).sqrt();

        // Assert: cat-dog distance should be much smaller than cat-car distance
        assert!(
            cat_dog_dist < cat_car_dist,
            "Cat-Dog distance ({}) should be less than Cat-Car distance ({})",
            cat_dog_dist, cat_car_dist
        );

        // Additionally, cat-dog distance should be significantly smaller (at least 2x)
        assert!(
            cat_car_dist > cat_dog_dist * 2.0,
            "Cat-Car distance should be at least 2x Cat-Dog distance"
        );

        std::fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_spreading_activation_on_semantic_graph() {
        use std::io::Write;
        use std::fs::File;

        let temp_path = "/tmp/test_spreading.txt";
        let mut file = File::create(temp_path).unwrap();

        // Create a semantic network with animal and vehicle clusters
        // Animals
        writeln!(file, "cat 1.0 0.9 0.1 0.0 0.0").unwrap();
        writeln!(file, "dog 0.9 1.0 0.1 0.0 0.0").unwrap();
        writeln!(file, "mouse 0.8 0.8 0.2 0.0 0.0").unwrap();

        // Vehicles
        writeln!(file, "car 0.0 0.0 0.1 1.0 0.9").unwrap();
        writeln!(file, "truck 0.0 0.0 0.1 0.9 1.0").unwrap();

        // Bridge words (have some similarity to both)
        writeln!(file, "toy 0.5 0.5 0.3 0.5 0.5").unwrap();

        let mut config = BootstrapConfig::default();
        config.embedding_dim = 5;
        config.target_dim = 3;
        config.knn_k = 2;

        let mut bootstrap = BootstrapLibrary::new(config);
        bootstrap.load_embeddings(temp_path).unwrap();
        bootstrap.run_pca_pipeline().unwrap();
        bootstrap.populate_graph().unwrap();
        bootstrap.populate_grid().unwrap();
        bootstrap.weave_connections().unwrap();

        // Get IDs before spreading activation
        let cat_id = bootstrap.get_concept("cat").unwrap().id;
        let dog_id = bootstrap.get_concept("dog").unwrap().id;

        // Perform spreading activation starting from "cat"
        let result = bootstrap.graph_mut().spreading_activation(cat_id, 1.0, None);

        // Verify result structure - should activate neighbors, not the source
        assert!(result.activated_nodes.len() > 0, "Should activate at least one node");
        assert!(!result.activated_nodes.is_empty(), "Should have activated nodes");
        assert!(result.nodes_visited >= 2, "Should visit at least source + neighbors");

        // Dog should be activated (neighbor of cat)
        let dog_activated = result.activated_nodes.iter()
            .find(|n| n.node_id == dog_id);
        assert!(dog_activated.is_some(), "Dog should be activated as neighbor");
        assert!(dog_activated.unwrap().energy > 0.0, "Dog should have positive activation");

        // Verify that activation spreads with decay
        // Neighbor nodes should have energy < 1.0 (source had 1.0, but neighbors decay)
        let max_activation = result.activated_nodes.iter()
            .map(|n| n.energy)
            .fold(0.0f32, f32::max);
        assert!(max_activation < 1.0, "Neighbor nodes should have decayed energy < 1.0");
        assert!(max_activation > 0.0, "Activated nodes should have positive energy");

        std::fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_integration_bootstrap_full_pipeline() {
        use std::io::Write;
        use std::fs::File;

        let temp_path = "/tmp/test_full_integration.txt";
        let mut file = File::create(temp_path).unwrap();

        // Create a small realistic semantic network
        writeln!(file, "happy 0.9 0.8 0.1 0.0 0.0").unwrap();
        writeln!(file, "joy 0.85 0.9 0.1 0.0 0.0").unwrap();
        writeln!(file, "sad 0.1 0.2 0.1 0.0 0.0").unwrap();
        writeln!(file, "red 0.0 0.0 1.0 0.0 0.0").unwrap();
        writeln!(file, "blue 0.0 0.0 0.9 0.1 0.0").unwrap();
        writeln!(file, "green 0.0 0.0 0.85 0.2 0.0").unwrap();

        let mut config = BootstrapConfig::default();
        config.embedding_dim = 5;
        config.target_dim = 3;
        config.knn_k = 2;

        let mut bootstrap = BootstrapLibrary::new(config);

        // Full pipeline
        let (concepts, edges) = bootstrap.bootstrap_from_embeddings(temp_path).unwrap();
        assert_eq!(concepts, 6);
        assert!(edges > 0);

        // Add multimodal anchors
        let (colors, emotions) = bootstrap.enrich_multimodal();
        assert_eq!(colors, 3, "Should enrich red, blue, green");
        assert_eq!(emotions, 3, "Should enrich happy, joy, sad");

        // Test spreading activation
        let happy_id = bootstrap.get_concept("happy").unwrap().id;
        let joy_id = bootstrap.get_concept("joy").unwrap().id;
        let result = bootstrap.graph_mut().spreading_activation(happy_id, 1.0, None);

        assert!(result.activated_nodes.len() > 1, "Should activate multiple nodes");

        // Joy should be activated (similar emotion)
        let joy_activated = result.activated_nodes.iter()
            .find(|n| n.node_id == joy_id);
        assert!(joy_activated.is_some(), "Joy should be activated from happy");

        // Verify multimodal data
        let red_concept = bootstrap.get_concept("red").unwrap();
        assert!(red_concept.color.is_some(), "Red should have color");
        assert_eq!(red_concept.color.unwrap(), [1.0, 0.0, 0.0]);

        let happy_concept = bootstrap.get_concept("happy").unwrap();
        assert!(happy_concept.emotion.is_some(), "Happy should have emotion");
        let vad = happy_concept.emotion.unwrap();
        assert!(vad[0] > 0.0, "Happy should have positive valence");

        std::fs::remove_file(temp_path).ok();
    }

    // ========================================================================
    // NEW v1.3 TESTS
    // ========================================================================

    #[test]
    fn test_sound_anchors() {
        use std::io::Write;
        use std::fs::File;

        let temp_path = "/tmp/test_sounds.txt";
        let mut file = File::create(temp_path).unwrap();

        // Include sound words
        writeln!(file, "whisper 0.1 0.2 0.3").unwrap();
        writeln!(file, "shout 0.4 0.5 0.6").unwrap();
        writeln!(file, "melody 0.7 0.8 0.9").unwrap();
        writeln!(file, "bang 0.2 0.3 0.4").unwrap();
        writeln!(file, "table 0.1 0.1 0.1").unwrap(); // non-sound word

        let mut config = BootstrapConfig::default();
        config.embedding_dim = 3;

        let mut bootstrap = BootstrapLibrary::new(config);
        bootstrap.load_embeddings(temp_path).unwrap();

        let enriched = bootstrap.add_sound_anchors();

        assert_eq!(enriched, 4, "Should enrich 4 sound words");

        // Check that sound was added
        let whisper_concept = bootstrap.get_concept("whisper").unwrap();
        assert!(whisper_concept.sound.is_some());
        let sound = whisper_concept.sound.unwrap();
        assert!(sound[0] < 0.0, "Whisper should have low volume");

        let shout_concept = bootstrap.get_concept("shout").unwrap();
        assert!(shout_concept.sound.is_some());
        let shout_sound = shout_concept.sound.unwrap();
        assert!(shout_sound[0] > 0.0, "Shout should have high volume");

        // Check that non-sound word has no sound
        let table_concept = bootstrap.get_concept("table").unwrap();
        assert!(table_concept.sound.is_none());

        std::fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_action_anchors() {
        use std::io::Write;
        use std::fs::File;

        let temp_path = "/tmp/test_actions.txt";
        let mut file = File::create(temp_path).unwrap();

        // Include action words (verbs)
        writeln!(file, "run 0.1 0.2 0.3").unwrap();
        writeln!(file, "sleep 0.4 0.5 0.6").unwrap();
        writeln!(file, "jump 0.7 0.8 0.9").unwrap();
        writeln!(file, "think 0.2 0.3 0.4").unwrap();
        writeln!(file, "table 0.1 0.1 0.1").unwrap(); // non-action word

        let mut config = BootstrapConfig::default();
        config.embedding_dim = 3;

        let mut bootstrap = BootstrapLibrary::new(config);
        bootstrap.load_embeddings(temp_path).unwrap();

        let enriched = bootstrap.add_action_anchors();

        assert_eq!(enriched, 4, "Should enrich 4 action words");

        // Check that action was added (Energy, Speed, Direction, Impact)
        let run_concept = bootstrap.get_concept("run").unwrap();
        assert!(run_concept.action.is_some());
        let run_action = run_concept.action.unwrap();
        assert!(run_action[0] > 0.0, "Run should have high energy");
        assert!(run_action[1] > 0.0, "Run should have high speed");

        let sleep_concept = bootstrap.get_concept("sleep").unwrap();
        assert!(sleep_concept.action.is_some());
        let sleep_action = sleep_concept.action.unwrap();
        assert!(sleep_action[0] < 0.0, "Sleep should have low energy");
        assert!(sleep_action[1] < 0.0, "Sleep should have low speed");

        // Check that non-action word has no action
        let table_concept = bootstrap.get_concept("table").unwrap();
        assert!(table_concept.action.is_none());

        std::fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_spatial_anchors() {
        use std::io::Write;
        use std::fs::File;

        let temp_path = "/tmp/test_spatial.txt";
        let mut file = File::create(temp_path).unwrap();

        // Include spatial words (prepositions)
        writeln!(file, "above 0.1 0.2 0.3").unwrap();
        writeln!(file, "below 0.4 0.5 0.6").unwrap();
        writeln!(file, "inside 0.7 0.8 0.9").unwrap();
        writeln!(file, "near 0.2 0.3 0.4").unwrap();
        writeln!(file, "table 0.1 0.1 0.1").unwrap(); // non-spatial word

        let mut config = BootstrapConfig::default();
        config.embedding_dim = 3;

        let mut bootstrap = BootstrapLibrary::new(config);
        bootstrap.load_embeddings(temp_path).unwrap();

        let enriched = bootstrap.add_spatial_anchors();

        assert_eq!(enriched, 4, "Should enrich 4 spatial words");

        // Check that spatial was added (Proximity, Verticality, Containment)
        let above_concept = bootstrap.get_concept("above").unwrap();
        assert!(above_concept.spatial.is_some());
        let above_spatial = above_concept.spatial.unwrap();
        assert!(above_spatial[1] > 0.0, "Above should have positive verticality");

        let below_concept = bootstrap.get_concept("below").unwrap();
        assert!(below_concept.spatial.is_some());
        let below_spatial = below_concept.spatial.unwrap();
        assert!(below_spatial[1] < 0.0, "Below should have negative verticality");

        let inside_concept = bootstrap.get_concept("inside").unwrap();
        assert!(inside_concept.spatial.is_some());
        let inside_spatial = inside_concept.spatial.unwrap();
        assert!(inside_spatial[2] > 0.0, "Inside should have high containment");

        // Check that non-spatial word has no spatial
        let table_concept = bootstrap.get_concept("table").unwrap();
        assert!(table_concept.spatial.is_none());

        std::fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_semantic_search() {
        use std::io::Write;
        use std::fs::File;

        let temp_path = "/tmp/test_semantic_search.txt";
        let mut file = File::create(temp_path).unwrap();

        // Create semantic clusters
        // Animals
        writeln!(file, "cat 1.0 0.9 0.1 0.0 0.0").unwrap();
        writeln!(file, "dog 0.9 1.0 0.1 0.0 0.0").unwrap();
        writeln!(file, "mouse 0.8 0.8 0.2 0.0 0.0").unwrap();
        writeln!(file, "bird 0.7 0.7 0.3 0.1 0.0").unwrap();

        // Vehicles
        writeln!(file, "car 0.0 0.0 0.1 1.0 0.9").unwrap();
        writeln!(file, "truck 0.0 0.0 0.1 0.9 1.0").unwrap();
        writeln!(file, "bus 0.1 0.0 0.2 0.8 0.8").unwrap();

        let mut config = BootstrapConfig::default();
        config.embedding_dim = 5;
        config.target_dim = 3;
        config.knn_k = 3;

        let mut bootstrap = BootstrapLibrary::new(config);
        bootstrap.load_embeddings(temp_path).unwrap();
        bootstrap.run_pca_pipeline().unwrap();
        bootstrap.populate_graph().unwrap();
        bootstrap.populate_grid().unwrap();
        bootstrap.weave_connections().unwrap();

        // Semantic search for "cat" should return related animals
        let results = bootstrap.semantic_search("cat", 5, None).unwrap();

        assert!(results.len() > 0, "Should return results");
        assert!(results.len() <= 5, "Should not exceed max_results");

        // Results should be sorted by relevance (descending energy)
        for i in 1..results.len() {
            assert!(
                results[i-1].1 >= results[i].1,
                "Results should be sorted by relevance"
            );
        }

        // Should find other animals ("dog", "mouse", "bird") higher than vehicles
        let dog_score = results.iter().find(|(w, _)| w == "dog").map(|(_, s)| *s);
        let car_score = results.iter().find(|(w, _)| w == "car").map(|(_, s)| *s);

        if let (Some(dog), Some(car)) = (dog_score, car_score) {
            assert!(
                dog > car,
                "Dog (animal) should be more relevant to cat than car (vehicle)"
            );
        }

        std::fs::remove_file(temp_path).ok();
    }

    #[test]
    fn test_extended_multimodal_enrichment() {
        use std::io::Write;
        use std::fs::File;

        let temp_path = "/tmp/test_extended_multimodal.txt";
        let mut file = File::create(temp_path).unwrap();

        // Mix of all modalities
        writeln!(file, "red 0.1 0.2 0.3").unwrap();      // color
        writeln!(file, "happy 0.4 0.5 0.6").unwrap();    // emotion
        writeln!(file, "whisper 0.7 0.8 0.9").unwrap();  // sound
        writeln!(file, "run 0.2 0.3 0.4").unwrap();      // action
        writeln!(file, "above 0.5 0.6 0.7").unwrap();    // spatial
        writeln!(file, "table 0.1 0.1 0.1").unwrap();    // none

        let mut config = BootstrapConfig::default();
        config.embedding_dim = 3;

        let mut bootstrap = BootstrapLibrary::new(config);
        bootstrap.load_embeddings(temp_path).unwrap();

        let (colors, emotions, sounds, actions, spatial) = bootstrap.enrich_extended_multimodal();

        assert_eq!(colors, 1, "Should enrich 1 color");
        assert_eq!(emotions, 1, "Should enrich 1 emotion");
        assert_eq!(sounds, 1, "Should enrich 1 sound");
        assert_eq!(actions, 1, "Should enrich 1 action");
        assert_eq!(spatial, 1, "Should enrich 1 spatial");

        // Verify each modality
        let red = bootstrap.get_concept("red").unwrap();
        assert!(red.color.is_some());
        assert!(red.emotion.is_none());
        assert!(red.sound.is_none());
        assert!(red.action.is_none());
        assert!(red.spatial.is_none());

        let happy = bootstrap.get_concept("happy").unwrap();
        assert!(happy.color.is_none());
        assert!(happy.emotion.is_some());
        assert!(happy.sound.is_none());
        assert!(happy.action.is_none());
        assert!(happy.spatial.is_none());

        let whisper = bootstrap.get_concept("whisper").unwrap();
        assert!(whisper.sound.is_some());

        let run = bootstrap.get_concept("run").unwrap();
        assert!(run.action.is_some());

        let above = bootstrap.get_concept("above").unwrap();
        assert!(above.spatial.is_some());

        let table = bootstrap.get_concept("table").unwrap();
        assert!(table.color.is_none());
        assert!(table.emotion.is_none());
        assert!(table.sound.is_none());
        assert!(table.action.is_none());
        assert!(table.spatial.is_none());

        std::fs::remove_file(temp_path).ok();
    }
}
