// NeuroGraph OS - Python Bindings v0.40.0
// PyO3-based bindings for NeuroGraph OS core

#![cfg(feature = "python")]

use pyo3::prelude::*;

mod token;
mod intuition;

use token::PyToken;
use intuition::{PyIntuitionEngine, PyIntuitionConfig};

/// NeuroGraph OS Python Module
#[pymodule]
fn neurograph(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Module metadata
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add("__author__", "Chernov Denys")?;
    m.add("__license__", "AGPL-3.0")?;

    // Core types
    m.add_class::<PyToken>()?;

    // Intuition Engine
    m.add_class::<PyIntuitionEngine>()?;
    m.add_class::<PyIntuitionConfig>()?;

    Ok(())
}
