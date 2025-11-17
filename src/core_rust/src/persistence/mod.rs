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

//! Persistence layer for NeuroGraph OS v0.26.0
//!
//! This module provides PostgreSQL backend for persisting:
//! - ExperienceEvents with ActionMetadata
//! - ADNA policies and state
//! - Configuration store
//! - Learning metrics

pub mod backend;

#[cfg(feature = "persistence")]
pub mod postgres;

pub use backend::{PersistenceBackend, PersistenceError, QueryOptions, ADNAPolicy, Configuration};

#[cfg(feature = "persistence")]
pub use postgres::{PostgresBackend, PostgresConfig};