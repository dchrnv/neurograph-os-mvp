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

    // ==================== ADNA Policy Management ====================

    /// Save an ADNA policy (creates new version or updates existing)
    async fn save_policy(
        &self,
        state_bin_id: &str,
        rule_id: &str,
        action_weights: &std::collections::HashMap<u16, f64>,
        metadata: Option<serde_json::Value>,
        parent_policy_id: Option<i32>,
    ) -> Result<i32, PersistenceError>;

    /// Get active policy for a state bin
    async fn get_active_policy(
        &self,
        state_bin_id: &str,
    ) -> Result<Option<ADNAPolicy>, PersistenceError>;

    /// Get all active policies
    async fn get_all_active_policies(&self) -> Result<Vec<ADNAPolicy>, PersistenceError>;

    /// Deactivate a policy (soft delete)
    async fn deactivate_policy(&self, policy_id: i32) -> Result<(), PersistenceError>;

    /// Update policy performance metrics
    async fn update_policy_metrics(
        &self,
        policy_id: i32,
        total_executions: i64,
        avg_reward: f32,
    ) -> Result<(), PersistenceError>;

    // ==================== Configuration Management ====================

    /// Save a configuration value (creates new version)
    async fn save_config(
        &self,
        component_name: &str,
        config_key: &str,
        config_value: serde_json::Value,
        parent_config_id: Option<i32>,
    ) -> Result<i32, PersistenceError>;

    /// Get active configuration value
    async fn get_config(
        &self,
        component_name: &str,
        config_key: &str,
    ) -> Result<Option<Configuration>, PersistenceError>;

    /// Get all active configurations for a component
    async fn get_component_configs(
        &self,
        component_name: &str,
    ) -> Result<Vec<Configuration>, PersistenceError>;

    /// Deactivate a configuration (soft delete)
    async fn deactivate_config(&self, config_id: i32) -> Result<(), PersistenceError>;
}

/// ADNA Policy representation
#[derive(Debug, Clone)]
pub struct ADNAPolicy {
    pub policy_id: i32,
    pub state_bin_id: String,
    pub rule_id: String,
    pub action_weights: std::collections::HashMap<u16, f64>,
    pub metadata: Option<serde_json::Value>,
    pub version: i32,
    pub parent_policy_id: Option<i32>,
    pub total_executions: i64,
    pub avg_reward: f32,
}

/// Configuration representation
#[derive(Debug, Clone)]
pub struct Configuration {
    pub config_id: i32,
    pub component_name: String,
    pub config_key: String,
    pub config_value: serde_json::Value,
    pub version: i32,
    pub parent_config_id: Option<i32>,
}