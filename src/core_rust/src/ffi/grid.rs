//! Python bindings for Grid V2.0 structure

use crate::grid::{Grid, GridConfig};
use crate::token::CoordinateSpace;
use crate::ffi::token::PyToken;
use pyo3::prelude::*;

/// Python wrapper for GridConfig
#[pyclass(name = "GridConfig")]
#[derive(Clone)]
pub struct PyGridConfig {
    pub(crate) inner: GridConfig,
}

#[pymethods]
impl PyGridConfig {
    #[new]
    #[pyo3(signature = (bucket_size=10.0, density_threshold=0.5, min_field_nodes=3))]
    fn new(bucket_size: f32, density_threshold: f32, min_field_nodes: usize) -> Self {
        PyGridConfig {
            inner: GridConfig {
                bucket_size,
                density_threshold,
                min_field_nodes,
            }
        }
    }

    /// Get bucket size
    #[getter]
    fn bucket_size(&self) -> f32 {
        self.inner.bucket_size
    }

    /// Set bucket size
    #[setter]
    fn set_bucket_size(&mut self, value: f32) {
        self.inner.bucket_size = value;
    }

    /// Get density threshold
    #[getter]
    fn density_threshold(&self) -> f32 {
        self.inner.density_threshold
    }

    /// Set density threshold
    #[setter]
    fn set_density_threshold(&mut self, value: f32) {
        self.inner.density_threshold = value;
    }

    /// Get minimum field nodes
    #[getter]
    fn min_field_nodes(&self) -> usize {
        self.inner.min_field_nodes
    }

    /// Set minimum field nodes
    #[setter]
    fn set_min_field_nodes(&mut self, value: usize) {
        self.inner.min_field_nodes = value;
    }

    fn __repr__(&self) -> String {
        format!(
            "GridConfig(bucket_size={:.2}, density_threshold={:.2}, min_field_nodes={})",
            self.inner.bucket_size,
            self.inner.density_threshold,
            self.inner.min_field_nodes
        )
    }
}

/// Python wrapper for Grid V2.0
#[pyclass(name = "Grid")]
pub struct PyGrid {
    inner: Grid,
}

#[pymethods]
impl PyGrid {
    /// Create a new Grid with default configuration
    #[new]
    #[pyo3(signature = (config=None))]
    fn new(config: Option<PyGridConfig>) -> Self {
        PyGrid {
            inner: match config {
                Some(cfg) => Grid::with_config(cfg.inner),
                None => Grid::new(),
            }
        }
    }

    /// Add a token to the grid
    fn add(&mut self, token: &PyToken) -> PyResult<()> {
        // Clone the inner token
        let token_copy = token.inner;
        self.inner.add(token_copy)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
    }

    /// Remove a token from the grid
    fn remove(&mut self, token_id: u32) -> Option<PyToken> {
        self.inner.remove(token_id).map(|token| PyToken { inner: token })
    }

    /// Get a token by ID
    fn get(&self, token_id: u32) -> Option<PyToken> {
        self.inner.get(token_id).map(|token| PyToken { inner: *token })
    }

    /// Get number of tokens in the grid
    fn __len__(&self) -> usize {
        self.inner.len()
    }

    /// Check if grid is empty
    fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    /// Find neighbors within radius in a specific space
    #[pyo3(signature = (center_token_id, space, radius, max_results=10))]
    fn find_neighbors(
        &self,
        center_token_id: u32,
        space: u8,
        radius: f32,
        max_results: usize,
    ) -> PyResult<Vec<(u32, f32)>> {
        let coord_space = match space {
            0 => CoordinateSpace::L1Physical,
            1 => CoordinateSpace::L2Sensory,
            2 => CoordinateSpace::L3Motor,
            3 => CoordinateSpace::L4Emotional,
            4 => CoordinateSpace::L5Cognitive,
            5 => CoordinateSpace::L6Social,
            6 => CoordinateSpace::L7Temporal,
            7 => CoordinateSpace::L8Abstract,
            _ => return Err(pyo3::exceptions::PyValueError::new_err("Invalid space index (0-7)")),
        };

        Ok(self.inner.find_neighbors(center_token_id, coord_space, radius, max_results))
    }

    /// Range query: find all tokens within radius of a point in a space
    #[pyo3(signature = (space, x, y, z, radius))]
    fn range_query(
        &self,
        space: u8,
        x: f32,
        y: f32,
        z: f32,
        radius: f32,
    ) -> PyResult<Vec<(u32, f32)>> {
        let coord_space = match space {
            0 => CoordinateSpace::L1Physical,
            1 => CoordinateSpace::L2Sensory,
            2 => CoordinateSpace::L3Motor,
            3 => CoordinateSpace::L4Emotional,
            4 => CoordinateSpace::L5Cognitive,
            5 => CoordinateSpace::L6Social,
            6 => CoordinateSpace::L7Temporal,
            7 => CoordinateSpace::L8Abstract,
            _ => return Err(pyo3::exceptions::PyValueError::new_err("Invalid space index (0-7)")),
        };

        Ok(self.inner.range_query(coord_space, x, y, z, radius))
    }

    /// Calculate field influence at a point in a space
    #[pyo3(signature = (space, x, y, z, radius))]
    fn calculate_field_influence(
        &self,
        space: u8,
        x: f32,
        y: f32,
        z: f32,
        radius: f32,
    ) -> PyResult<f32> {
        let coord_space = match space {
            0 => CoordinateSpace::L1Physical,
            1 => CoordinateSpace::L2Sensory,
            2 => CoordinateSpace::L3Motor,
            3 => CoordinateSpace::L4Emotional,
            4 => CoordinateSpace::L5Cognitive,
            5 => CoordinateSpace::L6Social,
            6 => CoordinateSpace::L7Temporal,
            7 => CoordinateSpace::L8Abstract,
            _ => return Err(pyo3::exceptions::PyValueError::new_err("Invalid space index (0-7)")),
        };

        Ok(self.inner.calculate_field_influence(coord_space, x, y, z, radius))
    }

    /// Calculate node density at a point in a space
    #[pyo3(signature = (space, x, y, z, radius))]
    fn calculate_density(
        &self,
        space: u8,
        x: f32,
        y: f32,
        z: f32,
        radius: f32,
    ) -> PyResult<f32> {
        let coord_space = match space {
            0 => CoordinateSpace::L1Physical,
            1 => CoordinateSpace::L2Sensory,
            2 => CoordinateSpace::L3Motor,
            3 => CoordinateSpace::L4Emotional,
            4 => CoordinateSpace::L5Cognitive,
            5 => CoordinateSpace::L6Social,
            6 => CoordinateSpace::L7Temporal,
            7 => CoordinateSpace::L8Abstract,
            _ => return Err(pyo3::exceptions::PyValueError::new_err("Invalid space index (0-7)")),
        };

        Ok(self.inner.calculate_density(coord_space, x, y, z, radius))
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "Grid(tokens={}, spaces=8)",
            self.inner.len()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_py_grid_config() {
        let config = PyGridConfig::new(20.0, 0.8, 5);
        assert_eq!(config.bucket_size(), 20.0);
        assert_eq!(config.density_threshold(), 0.8);
        assert_eq!(config.min_field_nodes(), 5);
    }
}
