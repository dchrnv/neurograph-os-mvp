//! FFI module for Python bindings using PyO3
//!
//! This module provides Python bindings for the core Rust structures:
//! - Token (64-byte structure)
//! - Connection (32-byte structure)
//! - Grid (8-dimensional spatial indexing)
//! - Graph (topological navigation and pathfinding)
//!
//! To build the Python extension:
//! ```bash
//! cargo build --release --features python
//! ```

#[cfg(feature = "python")]
pub mod token;

#[cfg(feature = "python")]
pub mod connection;

#[cfg(feature = "python")]
pub mod grid;

#[cfg(feature = "python")]
pub mod graph;

#[cfg(feature = "python")]
use pyo3::prelude::*;

/// Python module initialization
#[cfg(feature = "python")]
#[pymodule]
fn neurograph_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<token::PyToken>()?;
    m.add_class::<token::PyCoordinateSpace>()?;
    m.add_class::<token::PyEntityType>()?;
    m.add_class::<connection::PyConnection>()?;
    m.add_class::<connection::PyConnectionType>()?;
    m.add_class::<grid::PyGrid>()?;
    m.add_class::<grid::PyGridConfig>()?;
    m.add_class::<graph::PyGraph>()?;
    m.add_class::<graph::PyGraphConfig>()?;
    m.add_class::<graph::PyPath>()?;
    m.add_class::<graph::PySubgraph>()?;
    m.add_function(wrap_pyfunction!(graph::create_simple_graph, m)?)?;
    Ok(())
}
