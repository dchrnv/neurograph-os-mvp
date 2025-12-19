// Python bindings for Runtime
// PyO3 FFI wrapper for neurograph Runtime

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::sync::{Arc, Mutex};
use crate::graph::Graph;
use crate::bootstrap::{BootstrapLibrary, BootstrapConfig};
use crate::runtime_storage::RuntimeStorage;
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
    /// Runtime storage - single source of truth for all dynamic data
    storage: Arc<RuntimeStorage>,
    /// Graph topology (kept for compatibility, but data lives in storage)
    graph: Arc<Mutex<Graph>>,
    /// Bootstrap library for semantic embeddings
    bootstrap: Option<BootstrapLibrary>,
    /// Word to ID mapping (for Bootstrap)
    word_to_id: HashMap<String, u32>,
    /// ID to word mapping (for Bootstrap)
    id_to_word: HashMap<u32, String>,
    /// Initialization flag
    initialized: bool,
    /// Embedding dimensions
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
        let storage = Arc::new(RuntimeStorage::new());

        Ok(PyRuntime {
            storage,
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

    // ========================================================================
    // Token API - RuntimeStorage integration
    // ========================================================================

    /// Create a new token in runtime storage
    ///
    /// Args:
    ///     token_dict (dict): Token data with optional fields:
    ///         - weight (float): Token weight/intensity
    ///         - coordinates (list): 8D coordinates as [[x,y,z], ...]
    ///
    /// Returns:
    ///     int: Assigned token ID
    pub fn create_token(&self, _token_dict: &Bound<'_, PyDict>) -> PyResult<u32> {
        use crate::token::Token;

        // Create new token (ID will be assigned by storage)
        let token = Token::new(0);

        // TODO: Apply token_dict values if provided

        let id = self.storage.create_token(token);
        Ok(id)
    }

    /// Get token by ID
    ///
    /// Args:
    ///     token_id (int): Token ID
    ///
    /// Returns:
    ///     dict: Token data or None if not found
    pub fn get_token(&self, token_id: u32, py: Python) -> PyResult<Option<Py<PyDict>>> {
        match self.storage.get_token(token_id) {
            Some(token) => {
                let result = PyDict::new_bound(py);
                // Return proper types instead of all strings
                result.set_item("id", token.id)?;
                result.set_item("weight", token.weight)?;
                Ok(Some(result.unbind()))
            }
            None => Ok(None),
        }
    }

    /// Update token
    ///
    /// Args:
    ///     token_id (int): Token ID
    ///     token_dict (dict): Updated token data
    ///
    /// Returns:
    ///     bool: True if successful
    pub fn update_token(&self, token_id: u32, _token_dict: &Bound<'_, PyDict>) -> PyResult<bool> {
        match self.storage.get_token(token_id) {
            Some(token) => {
                // TODO: Apply updates from token_dict
                match self.storage.update_token(token_id, token) {
                    Ok(()) => Ok(true),
                    Err(_) => Ok(false),
                }
            }
            None => Ok(false),
        }
    }

    /// Delete token
    ///
    /// Args:
    ///     token_id (int): Token ID
    ///
    /// Returns:
    ///     bool: True if deleted
    pub fn delete_token(&self, token_id: u32) -> PyResult<bool> {
        Ok(self.storage.delete_token(token_id).is_some())
    }

    /// List tokens with pagination
    ///
    /// Args:
    ///     limit (int): Maximum number of tokens
    ///     offset (int): Number to skip
    ///
    /// Returns:
    ///     list: List of token IDs
    pub fn list_tokens(&self, limit: usize, offset: usize) -> PyResult<Vec<u32>> {
        let tokens = self.storage.list_tokens(limit, offset);
        Ok(tokens.iter().map(|t| t.id).collect())
    }

    /// Count total tokens
    ///
    /// Returns:
    ///     int: Total number of tokens
    pub fn count_tokens(&self) -> PyResult<usize> {
        Ok(self.storage.count_tokens())
    }

    /// Clear all tokens
    ///
    /// Returns:
    ///     int: Number of tokens removed
    pub fn clear_tokens(&self) -> PyResult<usize> {
        Ok(self.storage.clear_tokens())
    }

    // ========================================================================
    // Connection API
    // ========================================================================

    /// Create a new connection
    ///
    /// Args:
    ///     token_a_id (int): First token ID
    ///     token_b_id (int): Second token ID
    ///
    /// Returns:
    ///     int: Connection ID
    pub fn create_connection(&self, token_a_id: u32, token_b_id: u32) -> PyResult<u64> {
        use crate::connection_v3::ConnectionV3;

        let conn = ConnectionV3::new(token_a_id, token_b_id);
        let id = self.storage.create_connection(conn);
        Ok(id)
    }

    /// Get connection by ID
    ///
    /// Args:
    ///     connection_id (int): Connection ID
    ///
    /// Returns:
    ///     dict: Connection data or None
    pub fn get_connection(&self, connection_id: u64) -> PyResult<Option<HashMap<String, String>>> {
        match self.storage.get_connection(connection_id) {
            Some(conn) => {
                let mut result = HashMap::new();
                result.insert("token_a_id".to_string(), conn.token_a_id.to_string());
                result.insert("token_b_id".to_string(), conn.token_b_id.to_string());
                Ok(Some(result))
            }
            None => Ok(None),
        }
    }

    /// Delete connection
    ///
    /// Args:
    ///     connection_id (int): Connection ID
    ///
    /// Returns:
    ///     bool: True if deleted
    pub fn delete_connection(&self, connection_id: u64) -> PyResult<bool> {
        Ok(self.storage.delete_connection(connection_id).is_some())
    }

    /// List connections
    ///
    /// Args:
    ///     limit (int): Maximum number
    ///     offset (int): Number to skip
    ///
    /// Returns:
    ///     list: List of connection IDs
    pub fn list_connections(&self, limit: usize, offset: usize) -> PyResult<Vec<u64>> {
        let connections = self.storage.list_connections(limit, offset);
        // ConnectionV3 doesn't have ID field, so we can't return IDs
        // Return count instead
        Ok((0..connections.len() as u64).collect())
    }

    /// Count connections
    ///
    /// Returns:
    ///     int: Total connections
    pub fn count_connections(&self) -> PyResult<usize> {
        Ok(self.storage.count_connections())
    }

    // ========================================================================
    // Grid API
    // ========================================================================

    /// Get grid information
    ///
    /// Returns:
    ///     dict: Grid info with 'count' and 'bounds'
    pub fn get_grid_info(&self) -> PyResult<HashMap<String, String>> {
        let (count, _bounds) = self.storage.grid_info();
        let mut result = HashMap::new();
        result.insert("count".to_string(), count.to_string());
        Ok(result)
    }

    /// Find neighbors of a token
    ///
    /// Args:
    ///     token_id (int): Center token ID
    ///     radius (float): Search radius
    ///
    /// Returns:
    ///     list: List of (token_id, distance) tuples
    pub fn find_neighbors(&self, token_id: u32, radius: f32) -> PyResult<Vec<(u32, f32)>> {
        match self.storage.find_neighbors(token_id, radius) {
            Ok(neighbors) => Ok(neighbors),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Failed to find neighbors: {}", e)
            )),
        }
    }

    /// Range query for tokens
    ///
    /// Args:
    ///     center (list): Center coordinates [x, y, z]
    ///     radius (float): Search radius
    ///
    /// Returns:
    ///     list: List of (token_id, distance) tuples
    pub fn range_query(&self, center: [f32; 3], radius: f32) -> PyResult<Vec<(u32, f32)>> {
        Ok(self.storage.range_query(center, radius))
    }

    // ========================================================================
    // CDNA API
    // ========================================================================

    /// Get CDNA configuration
    ///
    /// Returns:
    ///     dict: CDNA configuration (profile_id: int, flags: int)
    pub fn get_cdna_config(&self, py: Python) -> PyResult<Py<PyDict>> {
        let cdna = self.storage.get_cdna();
        let result = PyDict::new_bound(py);
        // Convert u32 to i64 to avoid format code issues
        result.set_item("profile_id", cdna.profile_id as i64)?;
        result.set_item("flags", cdna.flags as i64)?;
        Ok(result.unbind())
    }

    /// Get CDNA dimension scales
    ///
    /// Returns:
    ///     list: 8 dimension scales [L1, L2, ..., L8]
    pub fn get_cdna_scales(&self) -> PyResult<Vec<f32>> {
        let cdna = self.storage.get_cdna();
        Ok(cdna.dimension_scales.to_vec())
    }

    /// Update CDNA scales
    ///
    /// Args:
    ///     scales (list): 8 dimension scales
    ///
    /// Returns:
    ///     bool: True if successful
    pub fn update_cdna_scales(&self, scales: [f32; 8]) -> PyResult<bool> {
        match self.storage.update_cdna_scales(scales) {
            Ok(()) => Ok(true),
            Err(e) => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
                format!("Failed to update scales: {}", e)
            )),
        }
    }

    /// Get CDNA profile ID
    ///
    /// Returns:
    ///     int: Profile ID
    pub fn get_cdna_profile(&self) -> PyResult<u32> {
        Ok(self.storage.get_cdna_profile())
    }

    /// Set CDNA profile ID
    ///
    /// Args:
    ///     profile_id (int): New profile ID
    pub fn set_cdna_profile(&self, profile_id: u32) -> PyResult<()> {
        self.storage.set_cdna_profile(profile_id);
        Ok(())
    }

    /// Get CDNA flags
    ///
    /// Returns:
    ///     int: Flags value
    pub fn get_cdna_flags(&self) -> PyResult<u32> {
        Ok(self.storage.get_cdna_flags())
    }

    /// Set CDNA flags
    ///
    /// Args:
    ///     flags (int): New flags value
    pub fn set_cdna_flags(&self, flags: u32) -> PyResult<()> {
        self.storage.set_cdna_flags(flags);
        Ok(())
    }

    /// Validate CDNA configuration
    ///
    /// Returns:
    ///     bool: True if valid
    pub fn validate_cdna(&self) -> PyResult<bool> {
        Ok(self.storage.validate_cdna())
    }
}
