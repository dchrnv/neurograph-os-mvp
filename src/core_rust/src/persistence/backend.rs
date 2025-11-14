//! PersistenceBackend trait and common types
//!
//! Defines the interface for persistence backends (PostgreSQL, SQLite, etc.)

use crate::experience_stream::{ExperienceEvent, ActionMetadata, ExperienceBatch};
use async_trait::async_trait;
use thiserror::Error;

/// Errors that can occur during persistence operations
#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("Database connection error: {0}")]
    ConnectionError(String),

    #[error("Query execution error: {0}")]
    QueryError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("Event not found: {0}")]
    NotFound(String),

    #[error("Invalid configuration: {0}")]
    ConfigError(String),
}

/// Query options for retrieving events
#[derive(Debug, Clone)]
pub struct QueryOptions {
    /// Maximum number of events to return
    pub limit: Option<u64>,

    /// Offset for pagination
    pub offset: Option<u64>,

    /// Filter by event type
    pub event_type: Option<u16>,

    /// Filter by episode ID
    pub episode_id: Option<u64>,

    /// Filter by timestamp range (Unix epoch microseconds)
    pub timestamp_start: Option<i64>,
    pub timestamp_end: Option<i64>,

    /// Filter by minimum total reward
    pub min_reward: Option<f32>,

    /// Include archived events
    pub include_archived: bool,

    /// Order by timestamp (true = ascending, false = descending)
    pub order_asc: bool,
}

impl Default for QueryOptions {
    fn default() -> Self {
        Self {
            limit: Some(100),
            offset: None,
            event_type: None,
            episode_id: None,
            timestamp_start: None,
            timestamp_end: None,
            min_reward: None,
            include_archived: false,
            order_asc: false,
        }
    }
}

/// Main trait for persistence backends
#[async_trait]
pub trait PersistenceBackend: Send + Sync {
    /// Write a single event to persistent storage
    async fn write_event(&self, event: &ExperienceEvent) -> Result<(), PersistenceError>;

    /// Write an event with metadata
    async fn write_event_with_metadata(
        &self,
        event: &ExperienceEvent,
        metadata: &ActionMetadata,
    ) -> Result<(), PersistenceError>;

    /// Write a batch of events
    async fn write_batch(&self, batch: &ExperienceBatch) -> Result<(), PersistenceError>;

    /// Read an event by event_id
    async fn read_event(&self, event_id: u128) -> Result<ExperienceEvent, PersistenceError>;

    /// Read an event with its metadata
    async fn read_event_with_metadata(
        &self,
        event_id: u128,
    ) -> Result<(ExperienceEvent, Option<ActionMetadata>), PersistenceError>;

    /// Query events with filtering options
    async fn query_events(
        &self,
        options: QueryOptions,
    ) -> Result<Vec<ExperienceEvent>, PersistenceError>;

    /// Query events with metadata
    async fn query_events_with_metadata(
        &self,
        options: QueryOptions,
    ) -> Result<Vec<(ExperienceEvent, Option<ActionMetadata>)>, PersistenceError>;

    /// Archive events older than a threshold (in days)
    async fn archive_old_events(&self, days_threshold: i32) -> Result<u64, PersistenceError>;

    /// Get count of events matching criteria
    async fn count_events(&self, options: QueryOptions) -> Result<u64, PersistenceError>;

    /// Health check - verify connection and schema
    async fn health_check(&self) -> Result<(), PersistenceError>;
}