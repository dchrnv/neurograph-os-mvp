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
pub mod cdna;

#[cfg(feature = "python")]
pub mod guardian;

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
    m.add_class::<cdna::PyCDNA>()?;
    m.add_class::<cdna::PyProfileId>()?;
    m.add_class::<guardian::PyGuardian>()?;
    m.add_class::<guardian::PyGuardianConfig>()?;
    m.add_class::<guardian::PyEventType>()?;
    m.add_class::<guardian::PyEvent>()?;
    m.add_function(wrap_pyfunction!(graph::create_simple_graph, m)?)?;
    Ok(())
}
