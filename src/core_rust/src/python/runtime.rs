// Python bindings for Runtime
// PyO3 FFI wrapper for neurograph Runtime

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::sync::{Arc, Mutex};
use crate::graph::Graph;
use crate::bootstrap::{BootstrapLibrary, BootstrapConfig};
use std::path::PathBuf;
use std::collections::HashMap;

/// Python wrapper for neurograph Runtime
///
/// Main runtime interface managing Graph, IntuitionEngine, and Bootstrap.
///
/// # Example
///
/// ```python
/// runtime = PyRuntime(config)
/// runtime.bootstrap("glove.6B.50d.txt", limit=50000)
/// result = runtime.query("cat", top_k=10)
/// ```
#[pyclass(name = "PyRuntime")]
pub struct PyRuntime {
    graph: Arc<Mutex<Graph>>,
    bootstrap: Option<BootstrapLibrary>,
    word_to_id: HashMap<String, u32>,
    id_to_word: HashMap<u32, String>,
    initialized: bool,
    dimensions: usize,
}

#[pymethods]
impl PyRuntime {
    /// Create new Runtime with configuration
    ///
    /// Args:
    ///     config (dict): Configuration dictionary with keys:
    ///         - grid_size (int): Size of spatial grid
    ///         - dimensions (int): Embedding dimensions
    ///         - learning_rate (float): Learning rate
    ///         - max_connections (int): Max connections per token
    ///
    /// Returns:
    ///     PyRuntime: New runtime instance
    #[new]
    pub fn new(config: &Bound<'_, PyDict>) -> PyResult<Self> {
        let _grid_size = config
            .get_item("grid_size")?
            .and_then(|v| v.extract::<usize>().ok())
            .unwrap_or(1000);

        let dimensions = config
            .get_item("dimensions")?
            .and_then(|v| v.extract::<usize>().ok())
            .unwrap_or(50);

        let graph = Graph::new();

        Ok(PyRuntime {
            graph: Arc::new(Mutex::new(graph)),
            bootstrap: None,
            word_to_id: HashMap::new(),
            id_to_word: HashMap::new(),
            initialized: false,
            dimensions,
        })
    }

    /// Bootstrap runtime with embeddings from file
    ///
    /// Args:
    ///     path (str): Path to embeddings file
    ///     format (str): Format type ("glove", "word2vec", "fasttext")
    ///     limit (int, optional): Maximum number of embeddings to load
    ///     progress (bool, optional): Show progress bar
    ///
    /// Raises:
    ///     RuntimeError: If loading fails
    pub fn bootstrap(
        &mut self,
        py: Python,
        path: String,
        _format: Option<&str>,
        limit: Option<usize>,
        _progress: Option<bool>,
    ) -> PyResult<()> {
        // Release GIL during bootstrap (can be slow)
        py.allow_threads(|| {
            let path_buf = PathBuf::from(&path);

            if !path_buf.exists() {
                return Err(PyErr::new::<pyo3::exceptions::PyFileNotFoundError, _>(
                    format!("Embeddings file not found: {:?}", path_buf)
                ));
            }

            // Create bootstrap configuration
            let mut config = BootstrapConfig::default();
            config.embeddings_path = path;
            config.embedding_dim = self.dimensions;
            config.target_dim = 3; // Project to 3D coordinates

            if let Some(lim) = limit {
                config.max_words = lim;
            }

            // Create bootstrap library
            let mut bootstrap = BootstrapLibrary::new(config);

            // Load embeddings from file
            let loaded = bootstrap.load_embeddings(&path_buf)
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    format!("Failed to load embeddings: {:?}", e)
                ))?;

            if loaded == 0 {
                return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "No embeddings loaded from file"
                ));
            }

            // Run PCA pipeline to project to 3D
            let (_variance, _projected) = bootstrap.run_pca_pipeline()
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    format!("PCA projection failed: {:?}", e)
                ))?;

            // Populate graph and grid
            bootstrap.populate_graph()
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    format!("Graph population failed: {:?}", e)
                ))?;

            bootstrap.populate_grid()
                .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    format!("Grid population failed: {:?}", e)
                ))?;

            // Build word-to-id mappings for quick lookup
            self.word_to_id.clear();
            self.id_to_word.clear();

            for (word, concept) in bootstrap.concepts_iter() {
                self.word_to_id.insert(word.clone(), concept.id);
                self.id_to_word.insert(concept.id, word.clone());
            }

            // Store bootstrap library
            self.bootstrap = Some(bootstrap);
            self.initialized = true;

            Ok(())
        })
    }

    /// Execute semantic query
    ///
    /// Args:
    ///     text (str): Query text
    ///     top_k (int): Number of results to return
    ///     context (dict, optional): Query context filters
    ///
    /// Returns:
    ///     tuple: (signal_id, results) where results is list of (term, similarity)
    ///
    /// Raises:
    ///     RuntimeError: If runtime not initialized or query fails
    pub fn query(
        &self,
        py: Python,
        text: String,
        top_k: Option<usize>,
        _context: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<(String, Vec<(String, f64)>)> {
        if !self.initialized {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Runtime not initialized. Call bootstrap() first."
            ));
        }

        // Release GIL during query
        py.allow_threads(|| {
            // Generate signal ID
            let signal_id = uuid::Uuid::new_v4().to_string();

            let k = top_k.unwrap_or(10);

            // Find token ID for query text
            let query_id = match self.word_to_id.get(&text) {
                Some(&id) => id,
                None => {
                    // Word not found in vocabulary
                    return Ok((signal_id, Vec::new()));
                }
            };

            // Get grid from bootstrap library
            let bootstrap = self.bootstrap.as_ref()
                .ok_or_else(|| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    "Bootstrap library not initialized"
                ))?;

            // Find neighbors using Grid KNN
            // Using L1Physical (first 3 dimensions) for semantic similarity
            use crate::token::CoordinateSpace;
            let neighbors = bootstrap.grid().find_neighbors(
                query_id,
                CoordinateSpace::L1Physical,
                1000.0, // Large radius to get all neighbors
                k + 1,  // +1 because the query word itself will be in results
            );

            // Convert to results: filter out self, map IDs to words, normalize scores
            let mut results: Vec<(String, f64)> = neighbors
                .iter()
                .filter(|(id, _)| *id != query_id) // Skip self
                .filter_map(|(id, distance)| {
                    // Convert ID to word
                    let word = self.id_to_word.get(id)?.clone();

                    // Convert distance to similarity score (0-1 range)
                    // Smaller distance = higher similarity
                    // Using exponential decay: similarity = exp(-distance/scale)
                    let similarity = (-distance / 10.0).exp();

                    Some((word, similarity as f64))
                })
                .take(k)
                .collect();

            // Sort by similarity (descending)
            results.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

            Ok((signal_id, results))
        })
    }

    /// Provide feedback on query result
    ///
    /// Args:
    ///     signal_id (str): Signal ID from query result
    ///     feedback_type (str): "positive", "negative", or "neutral"
    ///
    /// Raises:
    ///     RuntimeError: If runtime not initialized
    pub fn feedback(
        &self,
        py: Python,
        _signal_id: String,
        feedback_type: String,
    ) -> PyResult<()> {
        if !self.initialized {
            return Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                "Runtime not initialized. Call bootstrap() first."
            ));
        }

        // Release GIL during feedback processing
        py.allow_threads(|| {
            // TODO: Implement feedback processing
            // For now, just validate the feedback type
            match feedback_type.as_str() {
                "positive" | "negative" | "neutral" => Ok(()),
                _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                    format!("Invalid feedback type: {}. Use 'positive', 'negative', or 'neutral'", feedback_type)
                )),
            }
        })
    }

    /// Export Prometheus metrics
    ///
    /// Returns:
    ///     str: Metrics in Prometheus text format
    pub fn export_metrics(&self) -> PyResult<String> {
        use crate::metrics::export_metrics;

        let metrics = export_metrics()
            .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Failed to export metrics: {}", e)
            ))?;
        Ok(metrics)
    }

    /// Check if runtime is initialized
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get embedding dimensions
    pub fn get_dimensions(&self) -> usize {
        self.dimensions
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "PyRuntime(dimensions={}, initialized={})",
            self.dimensions,
            self.initialized
        )
    }
}
