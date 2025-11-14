//! PostgreSQL backend implementation for NeuroGraph OS v0.26.0
//!
//! Provides async PostgreSQL persistence for ExperienceEvents with ActionMetadata

use super::backend::{PersistenceBackend, PersistenceError, QueryOptions};
use crate::experience_stream::{ExperienceEvent, ActionMetadata, ExperienceBatch};
use async_trait::async_trait;
use sqlx::postgres::{PgPool, PgPoolOptions};
use sqlx::Row;
use std::time::Duration;

/// PostgreSQL backend configuration
#[derive(Debug, Clone)]
pub struct PostgresConfig {
    /// PostgreSQL connection URL
    pub database_url: String,

    /// Maximum number of connections in the pool
    pub max_connections: u32,

    /// Connection timeout in seconds
    pub connect_timeout: u64,

    /// Statement timeout in seconds (0 = no timeout)
    pub statement_timeout: u64,
}

impl Default for PostgresConfig {
    fn default() -> Self {
        Self {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://neurograph_user:password@localhost/neurograph_db".to_string()),
            max_connections: 10,
            connect_timeout: 30,
            statement_timeout: 30,
        }
    }
}

impl PostgresConfig {
    /// Load configuration from environment or .env file
    pub fn from_env() -> Result<Self, PersistenceError> {
        #[cfg(feature = "persistence")]
        {
            dotenv::dotenv().ok(); // Load .env file if present
        }

        let database_url = std::env::var("DATABASE_URL")
            .map_err(|_| PersistenceError::ConfigError(
                "DATABASE_URL environment variable not set".to_string()
            ))?;

        Ok(Self {
            database_url,
            max_connections: std::env::var("DB_MAX_CONNECTIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
            connect_timeout: std::env::var("DB_CONNECT_TIMEOUT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(30),
            statement_timeout: std::env::var("DB_STATEMENT_TIMEOUT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(30),
        })
    }
}

/// PostgreSQL backend implementation
pub struct PostgresBackend {
    pool: PgPool,
}

impl PostgresBackend {
    /// Create new PostgreSQL backend with configuration
    pub async fn new(config: PostgresConfig) -> Result<Self, PersistenceError> {
        let pool = PgPoolOptions::new()
            .max_connections(config.max_connections)
            .acquire_timeout(Duration::from_secs(config.connect_timeout))
            .connect(&config.database_url)
            .await
            .map_err(|e| PersistenceError::ConnectionError(e.to_string()))?;

        // Set statement timeout if configured
        if config.statement_timeout > 0 {
            sqlx::query(&format!("SET statement_timeout = {}", config.statement_timeout * 1000))
                .execute(&pool)
                .await
                .map_err(|e| PersistenceError::QueryError(e.to_string()))?;
        }

        Ok(Self { pool })
    }

    /// Create from environment variables
    pub async fn from_env() -> Result<Self, PersistenceError> {
        let config = PostgresConfig::from_env()?;
        Self::new(config).await
    }

    /// Helper: Convert u128 event_id to bytes for PostgreSQL BYTEA
    fn event_id_to_bytes(event_id: u128) -> Vec<u8> {
        event_id.to_be_bytes().to_vec()
    }

    /// Helper: Convert bytes from PostgreSQL BYTEA to u128 event_id
    fn bytes_to_event_id(bytes: &[u8]) -> Result<u128, PersistenceError> {
        if bytes.len() != 16 {
            return Err(PersistenceError::SerializationError(
                format!("Invalid event_id length: expected 16 bytes, got {}", bytes.len())
            ));
        }
        let mut array = [0u8; 16];
        array.copy_from_slice(bytes);
        Ok(u128::from_be_bytes(array))
    }
}

#[async_trait]
impl PersistenceBackend for PostgresBackend {
    async fn write_event(&self, event: &ExperienceEvent) -> Result<(), PersistenceError> {
        let event_id_bytes = Self::event_id_to_bytes(event.event_id);

        sqlx::query(
            r#"
            INSERT INTO experience_events (
                event_id, timestamp, episode_id, step_number, event_type, flags,
                state_l1, state_l2, state_l3, state_l4, state_l5, state_l6, state_l7, state_l8,
                action_l1, action_l2, action_l3, action_l4, action_l5, action_l6, action_l7, action_l8,
                reward_homeostasis, reward_curiosity, reward_efficiency, reward_goal,
                adna_version_hash, sequence_number
            ) VALUES (
                $1, $2, $3, $4, $5, $6,
                $7, $8, $9, $10, $11, $12, $13, $14,
                $15, $16, $17, $18, $19, $20, $21, $22,
                $23, $24, $25, $26, $27, $28
            )
            ON CONFLICT (event_id) DO NOTHING
            "#
        )
        .bind(&event_id_bytes)
        .bind(event.timestamp as i64)
        .bind(event.episode_id as i64)
        .bind(event.step_number as i32)
        .bind(event.event_type as i16)
        .bind(event.flags as i16)
        .bind(event.state[0])
        .bind(event.state[1])
        .bind(event.state[2])
        .bind(event.state[3])
        .bind(event.state[4])
        .bind(event.state[5])
        .bind(event.state[6])
        .bind(event.state[7])
        .bind(event.action[0])
        .bind(event.action[1])
        .bind(event.action[2])
        .bind(event.action[3])
        .bind(event.action[4])
        .bind(event.action[5])
        .bind(event.action[6])
        .bind(event.action[7])
        .bind(event.reward_homeostasis)
        .bind(event.reward_curiosity)
        .bind(event.reward_efficiency)
        .bind(event.reward_goal)
        .bind(event.adna_version_hash as i32)
        .bind(event.sequence_number as i32)
        .execute(&self.pool)
        .await
        .map_err(|e| PersistenceError::QueryError(e.to_string()))?;

        Ok(())
    }

    async fn write_event_with_metadata(
        &self,
        event: &ExperienceEvent,
        metadata: &ActionMetadata,
    ) -> Result<(), PersistenceError> {
        // Start transaction
        let mut tx = self.pool.begin()
            .await
            .map_err(|e| PersistenceError::QueryError(e.to_string()))?;

        let event_id_bytes = Self::event_id_to_bytes(event.event_id);

        // Write event
        sqlx::query(
            r#"
            INSERT INTO experience_events (
                event_id, timestamp, episode_id, step_number, event_type, flags,
                state_l1, state_l2, state_l3, state_l4, state_l5, state_l6, state_l7, state_l8,
                action_l1, action_l2, action_l3, action_l4, action_l5, action_l6, action_l7, action_l8,
                reward_homeostasis, reward_curiosity, reward_efficiency, reward_goal,
                adna_version_hash, sequence_number
            ) VALUES (
                $1, $2, $3, $4, $5, $6,
                $7, $8, $9, $10, $11, $12, $13, $14,
                $15, $16, $17, $18, $19, $20, $21, $22,
                $23, $24, $25, $26, $27, $28
            )
            ON CONFLICT (event_id) DO NOTHING
            "#
        )
        .bind(&event_id_bytes)
        .bind(event.timestamp as i64)
        .bind(event.episode_id as i64)
        .bind(event.step_number as i32)
        .bind(event.event_type as i16)
        .bind(event.flags as i16)
        .bind(event.state[0])
        .bind(event.state[1])
        .bind(event.state[2])
        .bind(event.state[3])
        .bind(event.state[4])
        .bind(event.state[5])
        .bind(event.state[6])
        .bind(event.state[7])
        .bind(event.action[0])
        .bind(event.action[1])
        .bind(event.action[2])
        .bind(event.action[3])
        .bind(event.action[4])
        .bind(event.action[5])
        .bind(event.action[6])
        .bind(event.action[7])
        .bind(event.reward_homeostasis)
        .bind(event.reward_curiosity)
        .bind(event.reward_efficiency)
        .bind(event.reward_goal)
        .bind(event.adna_version_hash as i32)
        .bind(event.sequence_number as i32)
        .execute(&mut *tx)
        .await
        .map_err(|e| PersistenceError::QueryError(e.to_string()))?;

        // Write metadata
        let params_json = serde_json::to_value(&metadata.parameters)
            .map_err(|e| PersistenceError::SerializationError(e.to_string()))?;

        sqlx::query(
            r#"
            INSERT INTO action_metadata (event_id, intent_type, executor_id, parameters)
            VALUES ($1, $2, $3, $4)
            ON CONFLICT (event_id) DO NOTHING
            "#
        )
        .bind(&event_id_bytes)
        .bind(&metadata.intent_type)
        .bind(&metadata.executor_id)
        .bind(params_json)
        .execute(&mut *tx)
        .await
        .map_err(|e| PersistenceError::QueryError(e.to_string()))?;

        // Commit transaction
        tx.commit()
            .await
            .map_err(|e| PersistenceError::QueryError(e.to_string()))?;

        Ok(())
    }

    async fn write_batch(&self, batch: &ExperienceBatch) -> Result<(), PersistenceError> {
        // For now, write events sequentially
        // TODO: Optimize with bulk insert
        for event in &batch.events {
            self.write_event(event).await?;
        }
        Ok(())
    }

    async fn read_event(&self, event_id: u128) -> Result<ExperienceEvent, PersistenceError> {
        let event_id_bytes = Self::event_id_to_bytes(event_id);

        let row = sqlx::query(
            r#"
            SELECT
                event_id, timestamp, episode_id, step_number, event_type, flags,
                state_l1, state_l2, state_l3, state_l4, state_l5, state_l6, state_l7, state_l8,
                action_l1, action_l2, action_l3, action_l4, action_l5, action_l6, action_l7, action_l8,
                reward_homeostasis, reward_curiosity, reward_efficiency, reward_goal,
                adna_version_hash, sequence_number
            FROM experience_events
            WHERE event_id = $1
            "#
        )
        .bind(&event_id_bytes)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| PersistenceError::QueryError(e.to_string()))?
        .ok_or_else(|| PersistenceError::NotFound(format!("Event {} not found", event_id)))?;

        Ok(ExperienceEvent {
            event_id,
            timestamp: row.get::<i64, _>("timestamp") as u64,
            episode_id: row.get::<i64, _>("episode_id") as u64,
            step_number: row.get::<i32, _>("step_number") as u32,
            event_type: row.get::<i16, _>("event_type") as u16,
            flags: row.get::<i16, _>("flags") as u16,
            state: [
                row.get::<f32, _>("state_l1"),
                row.get::<f32, _>("state_l2"),
                row.get::<f32, _>("state_l3"),
                row.get::<f32, _>("state_l4"),
                row.get::<f32, _>("state_l5"),
                row.get::<f32, _>("state_l6"),
                row.get::<f32, _>("state_l7"),
                row.get::<f32, _>("state_l8"),
            ],
            action: [
                row.get::<f32, _>("action_l1"),
                row.get::<f32, _>("action_l2"),
                row.get::<f32, _>("action_l3"),
                row.get::<f32, _>("action_l4"),
                row.get::<f32, _>("action_l5"),
                row.get::<f32, _>("action_l6"),
                row.get::<f32, _>("action_l7"),
                row.get::<f32, _>("action_l8"),
            ],
            reward_homeostasis: row.get("reward_homeostasis"),
            reward_curiosity: row.get("reward_curiosity"),
            reward_efficiency: row.get("reward_efficiency"),
            reward_goal: row.get("reward_goal"),
            adna_version_hash: row.get::<i32, _>("adna_version_hash") as u32,
            sequence_number: row.get::<i32, _>("sequence_number") as u32,
        })
    }

    async fn read_event_with_metadata(
        &self,
        event_id: u128,
    ) -> Result<(ExperienceEvent, Option<ActionMetadata>), PersistenceError> {
        let event = self.read_event(event_id).await?;
        let event_id_bytes = Self::event_id_to_bytes(event_id);

        // Try to fetch metadata
        let metadata_row = sqlx::query(
            r#"
            SELECT intent_type, executor_id, parameters
            FROM action_metadata
            WHERE event_id = $1
            "#
        )
        .bind(&event_id_bytes)
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| PersistenceError::QueryError(e.to_string()))?;

        let metadata = if let Some(row) = metadata_row {
            Some(ActionMetadata {
                intent_type: row.get("intent_type"),
                executor_id: row.get("executor_id"),
                parameters: row.get("parameters"),
            })
        } else {
            None
        };

        Ok((event, metadata))
    }

    async fn query_events(
        &self,
        options: QueryOptions,
    ) -> Result<Vec<ExperienceEvent>, PersistenceError> {
        let mut query = String::from(
            r#"
            SELECT
                event_id, timestamp, episode_id, step_number, event_type, flags,
                state_l1, state_l2, state_l3, state_l4, state_l5, state_l6, state_l7, state_l8,
                action_l1, action_l2, action_l3, action_l4, action_l5, action_l6, action_l7, action_l8,
                reward_homeostasis, reward_curiosity, reward_efficiency, reward_goal,
                adna_version_hash, sequence_number
            FROM experience_events
            WHERE 1=1
            "#
        );

        // Build WHERE clause
        if !options.include_archived {
            query.push_str(" AND archived = FALSE");
        }

        if let Some(event_type) = options.event_type {
            query.push_str(&format!(" AND event_type = {}", event_type));
        }

        if let Some(episode_id) = options.episode_id {
            query.push_str(&format!(" AND episode_id = {}", episode_id));
        }

        if let Some(ts_start) = options.timestamp_start {
            query.push_str(&format!(" AND timestamp >= {}", ts_start));
        }

        if let Some(ts_end) = options.timestamp_end {
            query.push_str(&format!(" AND timestamp <= {}", ts_end));
        }

        if let Some(min_reward) = options.min_reward {
            query.push_str(&format!(
                " AND (reward_homeostasis + reward_curiosity + reward_efficiency + reward_goal) >= {}",
                min_reward
            ));
        }

        // Add ORDER BY
        if options.order_asc {
            query.push_str(" ORDER BY timestamp ASC");
        } else {
            query.push_str(" ORDER BY timestamp DESC");
        }

        // Add LIMIT and OFFSET
        if let Some(limit) = options.limit {
            query.push_str(&format!(" LIMIT {}", limit));
        }

        if let Some(offset) = options.offset {
            query.push_str(&format!(" OFFSET {}", offset));
        }

        let rows = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| PersistenceError::QueryError(e.to_string()))?;

        let mut events = Vec::with_capacity(rows.len());
        for row in rows {
            let event_id_bytes: Vec<u8> = row.get("event_id");
            let event_id = Self::bytes_to_event_id(&event_id_bytes)?;

            events.push(ExperienceEvent {
                event_id,
                timestamp: row.get::<i64, _>("timestamp") as u64,
                episode_id: row.get::<i64, _>("episode_id") as u64,
                step_number: row.get::<i32, _>("step_number") as u32,
                event_type: row.get::<i16, _>("event_type") as u16,
                flags: row.get::<i16, _>("flags") as u16,
                state: [
                    row.get::<f32, _>("state_l1"),
                    row.get::<f32, _>("state_l2"),
                    row.get::<f32, _>("state_l3"),
                    row.get::<f32, _>("state_l4"),
                    row.get::<f32, _>("state_l5"),
                    row.get::<f32, _>("state_l6"),
                    row.get::<f32, _>("state_l7"),
                    row.get::<f32, _>("state_l8"),
                ],
                action: [
                    row.get::<f32, _>("action_l1"),
                    row.get::<f32, _>("action_l2"),
                    row.get::<f32, _>("action_l3"),
                    row.get::<f32, _>("action_l4"),
                    row.get::<f32, _>("action_l5"),
                    row.get::<f32, _>("action_l6"),
                    row.get::<f32, _>("action_l7"),
                    row.get::<f32, _>("action_l8"),
                ],
                reward_homeostasis: row.get("reward_homeostasis"),
                reward_curiosity: row.get("reward_curiosity"),
                reward_efficiency: row.get("reward_efficiency"),
                reward_goal: row.get("reward_goal"),
                adna_version_hash: row.get::<i32, _>("adna_version_hash") as u32,
                sequence_number: row.get::<i32, _>("sequence_number") as u32,
            });
        }

        Ok(events)
    }

    async fn query_events_with_metadata(
        &self,
        options: QueryOptions,
    ) -> Result<Vec<(ExperienceEvent, Option<ActionMetadata>)>, PersistenceError> {
        let events = self.query_events(options).await?;
        let mut results = Vec::with_capacity(events.len());

        for event in events {
            let event_id_bytes = Self::event_id_to_bytes(event.event_id);

            let metadata_row = sqlx::query(
                r#"
                SELECT intent_type, executor_id, parameters
                FROM action_metadata
                WHERE event_id = $1
                "#
            )
            .bind(&event_id_bytes)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| PersistenceError::QueryError(e.to_string()))?;

            let metadata = if let Some(row) = metadata_row {
                Some(ActionMetadata {
                    intent_type: row.get("intent_type"),
                    executor_id: row.get("executor_id"),
                    parameters: row.get("parameters"),
                })
            } else {
                None
            };

            results.push((event, metadata));
        }

        Ok(results)
    }

    async fn archive_old_events(&self, days_threshold: i32) -> Result<u64, PersistenceError> {
        let result = sqlx::query("SELECT archive_old_events($1)")
            .bind(days_threshold)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| PersistenceError::QueryError(e.to_string()))?;

        let count: i32 = result.get(0);
        Ok(count as u64)
    }

    async fn count_events(&self, options: QueryOptions) -> Result<u64, PersistenceError> {
        let mut query = String::from("SELECT COUNT(*) FROM experience_events WHERE 1=1");

        // Build WHERE clause (same as query_events)
        if !options.include_archived {
            query.push_str(" AND archived = FALSE");
        }

        if let Some(event_type) = options.event_type {
            query.push_str(&format!(" AND event_type = {}", event_type));
        }

        if let Some(episode_id) = options.episode_id {
            query.push_str(&format!(" AND episode_id = {}", episode_id));
        }

        if let Some(ts_start) = options.timestamp_start {
            query.push_str(&format!(" AND timestamp >= {}", ts_start));
        }

        if let Some(ts_end) = options.timestamp_end {
            query.push_str(&format!(" AND timestamp <= {}", ts_end));
        }

        if let Some(min_reward) = options.min_reward {
            query.push_str(&format!(
                " AND (reward_homeostasis + reward_curiosity + reward_efficiency + reward_goal) >= {}",
                min_reward
            ));
        }

        let row = sqlx::query(&query)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| PersistenceError::QueryError(e.to_string()))?;

        let count: i64 = row.get(0);
        Ok(count as u64)
    }

    async fn health_check(&self) -> Result<(), PersistenceError> {
        // Try a simple query to verify connection
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await
            .map_err(|e| PersistenceError::ConnectionError(e.to_string()))?;

        // Verify schema exists
        let tables = sqlx::query(
            r#"
            SELECT table_name
            FROM information_schema.tables
            WHERE table_schema = 'public'
                AND table_name IN ('experience_events', 'action_metadata', 'adna_policies', 'configuration_store', 'learning_metrics')
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| PersistenceError::QueryError(e.to_string()))?;

        if tables.len() != 5 {
            return Err(PersistenceError::ConfigError(
                format!("Expected 5 tables, found {}", tables.len())
            ));
        }

        Ok(())
    }
}