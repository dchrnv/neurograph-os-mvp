// Python bindings for IntuitionEngine

use pyo3::prelude::*;
use crate::intuition_engine::{IntuitionEngine, IntuitionConfig as RustIntuitionConfig};
use std::sync::{Arc, Mutex};

/// Python wrapper for IntuitionConfig
///
/// Configuration for the Intuition Engine v3.0 (Hybrid Reflex System).
///
/// # Example
///
/// ```python
/// config = IntuitionConfig(
///     analysis_interval_secs=30,
///     min_confidence=0.8,
///     enable_fast_path=True
/// )
/// ```
#[pyclass(name = "IntuitionConfig")]
#[derive(Clone)]
pub struct PyIntuitionConfig {
    pub(crate) inner: RustIntuitionConfig,
}

#[pymethods]
impl PyIntuitionConfig {
    /// Create new IntuitionConfig with defaults
    ///
    /// Args:
    ///     analysis_interval_secs: Analysis cycle interval (default: 60)
    ///     batch_size: Batch size for analysis (default: 1000)
    ///     min_confidence: Minimum confidence threshold 0-1 (default: 0.7)
    ///     max_proposals_per_cycle: Max proposals per cycle (default: 5)
    ///     enable_fast_path: Enable fast path reflexes (default: True)
    #[new]
    #[pyo3(signature = (
        analysis_interval_secs=None,
        batch_size=None,
        min_confidence=None,
        max_proposals_per_cycle=None,
        enable_fast_path=None
    ))]
    pub fn new(
        analysis_interval_secs: Option<u64>,
        batch_size: Option<usize>,
        min_confidence: Option<f64>,
        max_proposals_per_cycle: Option<usize>,
        enable_fast_path: Option<bool>,
    ) -> Self {
        let mut config = RustIntuitionConfig::default();

        if let Some(val) = analysis_interval_secs {
            config.analysis_interval_secs = val;
        }
        if let Some(val) = batch_size {
            config.batch_size = val;
        }
        if let Some(val) = min_confidence {
            config.min_confidence = val;
        }
        if let Some(val) = max_proposals_per_cycle {
            config.max_proposals_per_cycle = val;
        }
        if let Some(val) = enable_fast_path {
            config.enable_fast_path = val;
        }

        PyIntuitionConfig { inner: config }
    }

    /// Get analysis interval
    #[getter]
    pub fn analysis_interval_secs(&self) -> u64 {
        self.inner.analysis_interval_secs
    }

    /// Get batch size
    #[getter]
    pub fn batch_size(&self) -> usize {
        self.inner.batch_size
    }

    /// Get minimum confidence
    #[getter]
    pub fn min_confidence(&self) -> f64 {
        self.inner.min_confidence
    }

    /// Get enable fast path
    #[getter]
    pub fn enable_fast_path(&self) -> bool {
        self.inner.enable_fast_path
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "IntuitionConfig(interval={}s, batch={}, confidence={:.2}, fast_path={})",
            self.inner.analysis_interval_secs,
            self.inner.batch_size,
            self.inner.min_confidence,
            self.inner.enable_fast_path
        )
    }
}

/// Python wrapper for IntuitionEngine
///
/// Hybrid reflex + analytic system combining:
/// - System 1 (Fast Path): ~30-50ns reflex lookup
/// - System 2 (Slow Path): Pattern analysis from experience
///
/// # Example (Simple)
///
/// ```python
/// import neurograph
///
/// # Create with defaults - ONE LINE!
/// intuition = neurograph.IntuitionEngine.with_defaults()
///
/// # Get statistics
/// stats = intuition.stats()
/// print(f"Reflexes: {stats['total_reflexes']}")
/// ```
///
/// # Example (Custom Config)
///
/// ```python
/// # Custom configuration
/// intuition = neurograph.IntuitionEngine.builder()
///     .with_capacity(100_000)
///     .with_channel_size(10_000)
///     .build()
/// ```
#[pyclass(name = "IntuitionEngine")]
pub struct PyIntuitionEngine {
    // Use Arc<Mutex<>> for thread-safe interior mutability
    inner: Arc<Mutex<IntuitionEngine>>,
}

#[pymethods]
impl PyIntuitionEngine {
    /// Create IntuitionEngine with all defaults
    ///
    /// This is the simplest way to create an IntuitionEngine.
    /// Equivalent to Rust's `IntuitionEngine::with_defaults()`.
    ///
    /// Returns:
    ///     IntuitionEngine: New engine with default configuration
    ///
    /// # Example
    ///
    /// ```python
    /// intuition = IntuitionEngine.with_defaults()
    /// ```
    #[staticmethod]
    pub fn with_defaults(py: Python) -> PyResult<Self> {
        // Release GIL during engine initialization (v0.41.0)
        let engine = py.allow_threads(|| {
            IntuitionEngine::with_defaults()
        });

        Ok(PyIntuitionEngine {
            inner: Arc::new(Mutex::new(engine)),
        })
    }

    /// Create IntuitionEngine with custom config
    ///
    /// Args:
    ///     config: IntuitionConfig instance (optional)
    ///     capacity: Experience stream capacity (optional)
    ///     channel_size: Broadcast channel size (optional)
    ///
    /// Returns:
    ///     IntuitionEngine: New engine instance
    ///
    /// # Example
    ///
    /// ```python
    /// config = IntuitionConfig(min_confidence=0.8)
    /// intuition = IntuitionEngine.create(
    ///     config=config,
    ///     capacity=50_000
    /// )
    /// ```
    #[staticmethod]
    #[pyo3(signature = (config=None, capacity=None, channel_size=None))]
    pub fn create(
        py: Python,
        config: Option<PyIntuitionConfig>,
        capacity: Option<usize>,
        channel_size: Option<usize>,
    ) -> PyResult<Self> {
        // Release GIL during engine building (v0.41.0)
        let result = py.allow_threads(|| -> Result<IntuitionEngine, String> {
            let mut builder = IntuitionEngine::builder();

            if let Some(cfg) = config {
                builder = builder.with_config(cfg.inner);
            }

            if let Some(cap) = capacity {
                builder = builder.with_capacity(cap);
            }

            if let Some(size) = channel_size {
                builder = builder.with_channel_size(size);
            }

            builder.build()
        });

        let engine = result.map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e)
        })?;

        Ok(PyIntuitionEngine {
            inner: Arc::new(Mutex::new(engine)),
        })
    }

    /// Get current engine statistics
    ///
    /// Returns:
    ///     dict: Statistics dictionary with keys:
    ///         - reflexes_created: Total reflexes created
    ///         - total_reflexes: Current number of reflexes
    ///         - fast_path_hits: Number of fast path hits
    ///         - avg_fast_path_time_ns: Average fast path time in nanoseconds
    ///
    /// # Example
    ///
    /// ```python
    /// stats = intuition.stats()
    /// print(f"Reflexes: {stats['total_reflexes']}")
    /// print(f"Fast path: {stats['avg_fast_path_time_ns']}ns")
    /// ```
    pub fn stats(&self, py: Python) -> PyResult<std::collections::HashMap<String, u64>> {
        // Release GIL during lock acquisition and stats retrieval (v0.41.0)
        py.allow_threads(|| {
            let engine = self.inner.lock().map_err(|e| {
                PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                    format!("Failed to lock engine: {}", e)
                )
            })?;

            let stats = engine.get_stats();

            let mut result = std::collections::HashMap::new();
            result.insert("reflexes_created".to_string(), stats.reflexes_created);
            result.insert("total_reflexes".to_string(), stats.total_reflexes as u64);
            result.insert("fast_path_hits".to_string(), stats.fast_path_hits);
            result.insert("avg_fast_path_time_ns".to_string(), stats.avg_fast_path_time_ns);

            Ok(result)
        })
    }

    /// String representation
    fn __repr__(&self) -> PyResult<String> {
        let engine = self.inner.lock().map_err(|e| {
            PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(
                format!("Failed to lock engine: {}", e)
            )
        })?;

        let stats = engine.get_stats();

        Ok(format!(
            "IntuitionEngine(reflexes={}, fast_path_hits={})",
            stats.total_reflexes,
            stats.fast_path_hits
        ))
    }
}
