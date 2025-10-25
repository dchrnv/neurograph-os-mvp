//! FFI module for Python bindings using PyO3
//!
//! This module provides Python bindings for the core Rust structures:
//! - Token (64-byte structure)
//! - Connection (32-byte structure)
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
    Ok(())
}
