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

pub use backend::{PersistenceBackend, PersistenceError, QueryOptions};

#[cfg(feature = "persistence")]
pub use postgres::{PostgresBackend, PostgresConfig};