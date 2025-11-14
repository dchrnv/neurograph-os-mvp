# Persistence Layer v0.26.0 - PostgreSQL Backend

**Статус**: 🟡 Planned
**Дата**: 2025-01-14
**Автор**: Claude Code + Денис Чернов

---

## Обзор

Production-ready слой персистентности для NeuroGraph OS с использованием PostgreSQL.

### Цели

- ✅ Долговременное хранение ExperienceStream событий
- ✅ Персистентность ActionMetadata для анализа причинно-следственных связей
- ✅ Версионное хранилище ADNA политик с lineage tracking
- ✅ Configuration store для runtime параметров
- ✅ Эффективный query интерфейс для batch retrieval
- ✅ Retention policies и автоматическое архивирование

---

## Архитектура

### Принципы дизайна

1. **Hot/Cold Separation**:
   - Hot: In-memory circular buffer (ExperienceStream)
   - Warm: PostgreSQL для недавних данных (последние 7 дней)
   - Cold: Архивирование старых данных (>7 дней) в сжатом формате

2. **Write-Ahead Pattern**:
   - Быстрая запись в memory buffer
   - Асинхронный batch flush в PostgreSQL
   - Гарантия durability через WAL

3. **Read Optimization**:
   - Индексы на часто используемые поля
   - Materialized views для аналитики
   - Connection pooling для производительности

---

## Схема БД

### 1. experience_events

Основная таблица для всех событий опыта (1:1 mapping с ExperienceEvent struct).

```sql
CREATE TABLE experience_events (
    -- Primary key
    event_id BYTEA PRIMARY KEY,  -- u128 as 16 bytes

    -- Temporal attributes
    timestamp BIGINT NOT NULL,  -- Unix epoch microseconds
    episode_id BIGINT NOT NULL,
    step_number INTEGER NOT NULL,

    -- Event classification
    event_type SMALLINT NOT NULL,
    flags SMALLINT NOT NULL,

    -- State space (8D)
    state_l1 REAL NOT NULL,  -- Existence
    state_l2 REAL NOT NULL,  -- Novelty
    state_l3 REAL NOT NULL,  -- Velocity
    state_l4 REAL NOT NULL,  -- Attention
    state_l5 REAL NOT NULL,  -- Cognitive Load
    state_l6 REAL NOT NULL,  -- Certainty
    state_l7 REAL NOT NULL,  -- Valence
    state_l8 REAL NOT NULL,  -- Coherence

    -- Action space (8D)
    action_l1 REAL NOT NULL,
    action_l2 REAL NOT NULL,
    action_l3 REAL NOT NULL,
    action_l4 REAL NOT NULL,
    action_l5 REAL NOT NULL,
    action_l6 REAL NOT NULL,
    action_l7 REAL NOT NULL,
    action_l8 REAL NOT NULL,

    -- Reward components
    reward_homeostasis REAL NOT NULL DEFAULT 0.0,
    reward_curiosity REAL NOT NULL DEFAULT 0.0,
    reward_efficiency REAL NOT NULL DEFAULT 0.0,
    reward_goal REAL NOT NULL DEFAULT 0.0,

    -- ADNA tracking
    adna_version_hash INTEGER NOT NULL,

    -- Sequence tracking
    sequence_number INTEGER NOT NULL,

    -- Metadata
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    archived BOOLEAN DEFAULT FALSE
);

-- Indexes for common queries
CREATE INDEX idx_events_timestamp ON experience_events(timestamp DESC);
CREATE INDEX idx_events_episode ON experience_events(episode_id, step_number);
CREATE INDEX idx_events_type ON experience_events(event_type);
CREATE INDEX idx_events_adna_version ON experience_events(adna_version_hash);
CREATE INDEX idx_events_archived ON experience_events(archived) WHERE NOT archived;

-- Composite index for reward-based queries
CREATE INDEX idx_events_total_reward ON experience_events(
    (reward_homeostasis + reward_curiosity + reward_efficiency + reward_goal) DESC
);
```

### 2. action_metadata

Таблица метаданных для action events (связана с experience_events по event_id).

```sql
CREATE TABLE action_metadata (
    -- Foreign key to experience_events
    event_id BYTEA PRIMARY KEY REFERENCES experience_events(event_id) ON DELETE CASCADE,

    -- Action context
    intent_type VARCHAR(255) NOT NULL,
    executor_id VARCHAR(255) NOT NULL,

    -- Parameters as JSONB for flexible querying
    parameters JSONB NOT NULL,

    -- Metadata
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for analytics
CREATE INDEX idx_metadata_intent_type ON action_metadata(intent_type);
CREATE INDEX idx_metadata_executor ON action_metadata(executor_id);
CREATE INDEX idx_metadata_parameters ON action_metadata USING GIN(parameters);
```

### 3. adna_policies

Версионное хранилище ADNA политик с lineage tracking.

```sql
CREATE TABLE adna_policies (
    -- Primary key
    policy_id SERIAL PRIMARY KEY,

    -- State bin identifier
    state_bin_id VARCHAR(255) NOT NULL,

    -- Policy identification
    rule_id VARCHAR(255) NOT NULL,

    -- Action weights (JSONB for flexibility)
    action_weights JSONB NOT NULL,  -- HashMap<u16, f64>

    -- Policy metadata
    metadata JSONB,

    -- Versioning
    version INTEGER NOT NULL DEFAULT 1,
    parent_policy_id INTEGER REFERENCES adna_policies(policy_id),

    -- Temporal tracking
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE,

    -- Performance metrics
    total_executions BIGINT DEFAULT 0,
    avg_reward REAL DEFAULT 0.0,

    -- Unique constraint: only one active policy per state_bin
    UNIQUE(state_bin_id, is_active) WHERE is_active
);

-- Indexes
CREATE INDEX idx_policies_state_bin ON adna_policies(state_bin_id) WHERE is_active;
CREATE INDEX idx_policies_lineage ON adna_policies(parent_policy_id);
CREATE INDEX idx_policies_performance ON adna_policies(avg_reward DESC);
```

### 4. configuration_store

Версионное хранилище конфигураций для runtime параметров.

```sql
CREATE TABLE configuration_store (
    -- Primary key
    config_id SERIAL PRIMARY KEY,

    -- Configuration identification
    component_name VARCHAR(255) NOT NULL,  -- e.g., 'action_controller', 'intuition_engine'
    config_key VARCHAR(255) NOT NULL,

    -- Configuration value (JSONB for type flexibility)
    config_value JSONB NOT NULL,

    -- Versioning
    version INTEGER NOT NULL DEFAULT 1,
    parent_config_id INTEGER REFERENCES configuration_store(config_id),

    -- Temporal tracking
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    is_active BOOLEAN DEFAULT TRUE,

    -- Unique constraint: only one active config per component/key
    UNIQUE(component_name, config_key, is_active) WHERE is_active
);

-- Indexes
CREATE INDEX idx_config_component ON configuration_store(component_name) WHERE is_active;
CREATE INDEX idx_config_key ON configuration_store(component_name, config_key) WHERE is_active;
CREATE INDEX idx_config_lineage ON configuration_store(parent_config_id);
```

### 5. learning_metrics

Таблица для отслеживания прогресса обучения и метрик производительности.

```sql
CREATE TABLE learning_metrics (
    -- Primary key
    metric_id SERIAL PRIMARY KEY,

    -- Temporal
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,

    -- Metric type
    metric_type VARCHAR(255) NOT NULL,  -- 'proposal_accepted', 'reward_trend', 'policy_performance'

    -- Metric data (flexible JSONB)
    metric_data JSONB NOT NULL,

    -- Related entities
    related_policy_id INTEGER REFERENCES adna_policies(policy_id),
    related_event_id BYTEA REFERENCES experience_events(event_id)
);

-- Indexes
CREATE INDEX idx_metrics_timestamp ON learning_metrics(timestamp DESC);
CREATE INDEX idx_metrics_type ON learning_metrics(metric_type);
CREATE INDEX idx_metrics_policy ON learning_metrics(related_policy_id);
```

---

## Rust Implementation

### Dependencies

```toml
[dependencies]
# PostgreSQL async driver
tokio-postgres = "0.7"

# Connection pooling
deadpool-postgres = "0.10"

# SQL query builder (optional, for type safety)
sqlx = { version = "0.7", features = ["postgres", "runtime-tokio", "macros", "uuid", "chrono", "json"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# UUID support
uuid = { version = "1.0", features = ["v4", "serde"] }

# Time handling
chrono = "0.4"
```

### Trait Design

```rust
/// Trait for persistent storage backend
#[async_trait]
pub trait PersistenceBackend: Send + Sync {
    /// Write single event
    async fn write_event(&self, event: &ExperienceEvent) -> Result<(), PersistenceError>;

    /// Write event with metadata
    async fn write_event_with_metadata(
        &self,
        event: &ExperienceEvent,
        metadata: &ActionMetadata,
    ) -> Result<(), PersistenceError>;

    /// Write batch of events (optimized)
    async fn write_batch(&self, events: &[ExperienceEvent]) -> Result<(), PersistenceError>;

    /// Query events by time range
    async fn query_events(
        &self,
        start_time: u64,
        end_time: u64,
        limit: usize,
    ) -> Result<Vec<ExperienceEvent>, PersistenceError>;

    /// Query events with metadata
    async fn query_events_with_metadata(
        &self,
        start_time: u64,
        end_time: u64,
        limit: usize,
    ) -> Result<Vec<(ExperienceEvent, Option<ActionMetadata>)>, PersistenceError>;

    /// Save ADNA policy
    async fn save_policy(
        &self,
        state_bin_id: &str,
        policy: &ActionPolicy,
    ) -> Result<i32, PersistenceError>; // Returns policy_id

    /// Load active policy for state bin
    async fn load_policy(
        &self,
        state_bin_id: &str,
    ) -> Result<Option<ActionPolicy>, PersistenceError>;

    /// Save configuration
    async fn save_config(
        &self,
        component: &str,
        key: &str,
        value: serde_json::Value,
    ) -> Result<(), PersistenceError>;

    /// Load configuration
    async fn load_config(
        &self,
        component: &str,
        key: &str,
    ) -> Result<Option<serde_json::Value>, PersistenceError>;

    /// Archive old events (move to cold storage)
    async fn archive_events(&self, older_than: u64) -> Result<usize, PersistenceError>;
}
```

---

## Retention Policies

### Automatic Archiving

```sql
-- Function to archive old events
CREATE OR REPLACE FUNCTION archive_old_events(days_threshold INTEGER)
RETURNS INTEGER AS $$
DECLARE
    archived_count INTEGER;
    cutoff_timestamp BIGINT;
BEGIN
    -- Calculate cutoff (Unix epoch microseconds)
    cutoff_timestamp := EXTRACT(EPOCH FROM (NOW() - INTERVAL '1 day' * days_threshold)) * 1000000;

    -- Update archived flag
    UPDATE experience_events
    SET archived = TRUE
    WHERE timestamp < cutoff_timestamp
      AND archived = FALSE;

    GET DIAGNOSTICS archived_count = ROW_COUNT;

    RETURN archived_count;
END;
$$ LANGUAGE plpgsql;

-- Scheduled job (using pg_cron or external scheduler)
-- SELECT archive_old_events(7);  -- Archive events older than 7 days
```

---

## Query Examples

### 1. Batch Retrieval for IntuitionEngine

```sql
-- Get recent high-reward events for analysis
SELECT e.*, m.intent_type, m.executor_id, m.parameters
FROM experience_events e
LEFT JOIN action_metadata m ON e.event_id = m.event_id
WHERE e.timestamp > $1
  AND e.archived = FALSE
  AND (e.reward_homeostasis + e.reward_curiosity + e.reward_efficiency + e.reward_goal) > 0.5
ORDER BY e.timestamp DESC
LIMIT 1000;
```

### 2. Policy Performance Analytics

```sql
-- Analyze policy performance over time
SELECT
    state_bin_id,
    rule_id,
    version,
    total_executions,
    avg_reward,
    updated_at
FROM adna_policies
WHERE is_active = TRUE
ORDER BY avg_reward DESC
LIMIT 100;
```

### 3. Configuration History

```sql
-- View configuration evolution
SELECT
    component_name,
    config_key,
    config_value,
    version,
    created_at
FROM configuration_store
WHERE component_name = 'action_controller'
  AND config_key = 'exploration_rate'
ORDER BY version DESC;
```

---

## Migration Strategy

### Phase 1: Dual-Write (v0.26.0)
- ExperienceStream пишет в **память + PostgreSQL**
- Чтение из памяти (fast path)
- PostgreSQL для долговременного хранения

### Phase 2: Read Optimization (v0.27.0)
- Intelligent caching
- Prefetch для predictable patterns
- Async batch loading

### Phase 3: Full PostgreSQL (v1.0.0)
- Memory buffer как L1 cache
- PostgreSQL как source of truth
- Automatic failover и recovery

---

## Benchmarks (Target)

- **Write latency**: < 1ms (async batch)
- **Query latency**: < 10ms (10k events)
- **Throughput**: > 10k events/sec
- **Storage efficiency**: ~200 bytes/event (compressed)

---

## Next Steps

1. ✅ Create PostgreSQL schema
2. ⏳ Implement `PostgresBackend` struct
3. ⏳ Implement async writer with batching
4. ⏳ Implement query interface
5. ⏳ Add retention policy automation
6. ⏳ Write integration tests
7. ⏳ Create migration tool (memory → PostgreSQL)

---

**Примечание**: Эта спецификация будет реализована инкрементально в v0.26.0.