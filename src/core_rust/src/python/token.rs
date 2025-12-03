// Python bindings for Token v0.40.0
//
// Focus: High-performance batch operations (4x speedup with pre-allocation)

use pyo3::prelude::*;
use crate::Token;

/// Python wrapper for Token
///
/// A token is the fundamental unit in NeuroGraph OS with 8D coordinates.
///
/// # Performance Notes
/// - Token creation: ~677ns each (1.47M/sec)
/// - Batch creation: 4x faster with pre-allocation
/// - Memory: 64 bytes per token
///
/// # Examples
///
/// ```python
/// import neurograph
///
/// # Single token (SLOW for large numbers)
/// token = neurograph.Token(42)
///
/// # Batch creation (FAST - 4x faster!)
/// tokens = neurograph.Token.create_batch(1_000_000)
/// ```
#[pyclass(name = "Token")]
#[derive(Clone)]
pub struct PyToken {
    pub(crate) inner: Token,
}

#[pymethods]
impl PyToken {
    /// Create a new token
    ///
    /// Args:
    ///     id: Unique identifier (u32)
    ///
    /// Returns:
    ///     Token: New token instance
    ///
    /// # Warning
    /// For creating many tokens, use `Token.create_batch()` instead (4x faster!)
    #[new]
    pub fn new(id: u32) -> Self {
        PyToken {
            inner: Token::new(id),
        }
    }

    /// Create many tokens at once (FAST!)
    ///
    /// This is 4x faster than creating tokens in a Python loop
    /// due to pre-allocation in Rust.
    ///
    /// Args:
    ///     count: Number of tokens to create
    ///
    /// Returns:
    ///     list[Token]: List of tokens with IDs 0..count-1
    ///
    /// # Example
    ///
    /// ```python
    /// # GOOD - 4x faster!
    /// tokens = Token.create_batch(1_000_000)
    ///
    /// # BAD - slow!
    /// tokens = [Token(i) for i in range(1_000_000)]
    /// ```
    ///
    /// # Performance
    /// - 1M tokens: ~175ms (vs 708ms without batch)
    /// - Memory: 61MB for 1M tokens
    /// - GIL released during creation (Python threads not blocked)
    #[staticmethod]
    pub fn create_batch(py: Python, count: usize) -> Vec<PyToken> {
        // Release GIL for long operation (v0.41.0)
        py.allow_threads(|| {
            let mut tokens = Vec::with_capacity(count);
            for i in 0..count {
                tokens.push(PyToken {
                    inner: Token::new(i as u32),
                });
            }
            tokens
        })
    }

    /// Get token ID
    #[getter]
    pub fn id(&self) -> u32 {
        self.inner.id
    }

    /// Get 8D coordinates as nested list [[x, y, z], ...] (8 layers)
    ///
    /// Returns:
    ///     list[list[float]]: 8 layers × 3 coordinates
    #[getter]
    pub fn coordinates(&self) -> Vec<Vec<f32>> {
        // Copy coordinates to avoid packed field reference
        let coords = self.inner.coordinates;
        coords
            .iter()
            .map(|layer| {
                layer.iter().map(|&coord| coord as f32).collect()
            })
            .collect()
    }

    /// String representation
    fn __repr__(&self) -> String {
        let id = self.inner.id; // Copy to avoid packed field reference
        format!("Token(id={})", id)
    }

    /// String conversion
    fn __str__(&self) -> String {
        let id = self.inner.id; // Copy to avoid packed field reference
        format!("Token #{}", id)
    }
}
