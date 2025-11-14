-- NeuroGraph OS PostgreSQL Schema v0.26.0
-- Production-ready persistence layer
--
-- Created: 2025-01-14
-- Author: Claude Code + Денис Чернов

-- =============================================================================
-- 1. EXPERIENCE EVENTS
-- =============================================================================

CREATE TABLE IF NOT EXISTS experience_events (
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
CREATE INDEX IF NOT EXISTS idx_events_timestamp ON experience_events(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_events_episode ON experience_events(episode_id, step_number);
CREATE INDEX IF NOT EXISTS idx_events_type ON experience_events(event_type);
CREATE INDEX IF NOT EXISTS idx_events_adna_version ON experience_events(adna_version_hash);
CREATE INDEX IF NOT EXISTS idx_events_archived ON experience_events(archived) WHERE NOT archived;

-- Composite index for reward-based queries
CREATE INDEX IF NOT EXISTS idx_events_total_reward ON experience_events(
    (reward_homeostasis + reward_curiosity + reward_efficiency + reward_goal) DESC
);

-- =============================================================================
-- 2. ACTION METADATA
-- =============================================================================

CREATE TABLE IF NOT EXISTS action_metadata (
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
CREATE INDEX IF NOT EXISTS idx_metadata_intent_type ON action_metadata(intent_type);
CREATE INDEX IF NOT EXISTS idx_metadata_executor ON action_metadata(executor_id);
CREATE INDEX IF NOT EXISTS idx_metadata_parameters ON action_metadata USING GIN(parameters);

-- =============================================================================
-- 3. ADNA POLICIES
-- =============================================================================

CREATE TABLE IF NOT EXISTS adna_policies (
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
    avg_reward REAL DEFAULT 0.0
);

-- Unique constraint: only one active policy per state_bin
CREATE UNIQUE INDEX IF NOT EXISTS idx_policies_unique_active
    ON adna_policies(state_bin_id, is_active) WHERE is_active;

-- Other indexes
CREATE INDEX IF NOT EXISTS idx_policies_state_bin ON adna_policies(state_bin_id) WHERE is_active;
CREATE INDEX IF NOT EXISTS idx_policies_lineage ON adna_policies(parent_policy_id);
CREATE INDEX IF NOT EXISTS idx_policies_performance ON adna_policies(avg_reward DESC);

-- =============================================================================
-- 4. CONFIGURATION STORE
-- =============================================================================

CREATE TABLE IF NOT EXISTS configuration_store (
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
    is_active BOOLEAN DEFAULT TRUE
);

-- Unique constraint: only one active config per component/key
CREATE UNIQUE INDEX IF NOT EXISTS idx_config_unique_active
    ON configuration_store(component_name, config_key, is_active) WHERE is_active;

-- Other indexes
CREATE INDEX IF NOT EXISTS idx_config_component ON configuration_store(component_name) WHERE is_active;
CREATE INDEX IF NOT EXISTS idx_config_key ON configuration_store(component_name, config_key) WHERE is_active;
CREATE INDEX IF NOT EXISTS idx_config_lineage ON configuration_store(parent_config_id);

-- =============================================================================
-- 5. LEARNING METRICS
-- =============================================================================

CREATE TABLE IF NOT EXISTS learning_metrics (
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
CREATE INDEX IF NOT EXISTS idx_metrics_timestamp ON learning_metrics(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_metrics_type ON learning_metrics(metric_type);
CREATE INDEX IF NOT EXISTS idx_metrics_policy ON learning_metrics(related_policy_id);

-- =============================================================================
-- RETENTION POLICIES
-- =============================================================================

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

-- =============================================================================
-- HELPER VIEWS
-- =============================================================================

-- View: Recent high-value events with metadata
CREATE OR REPLACE VIEW recent_valuable_events AS
SELECT
    e.*,
    m.intent_type,
    m.executor_id,
    m.parameters,
    (e.reward_homeostasis + e.reward_curiosity + e.reward_efficiency + e.reward_goal) as total_reward
FROM experience_events e
LEFT JOIN action_metadata m ON e.event_id = m.event_id
WHERE e.archived = FALSE
  AND e.timestamp > EXTRACT(EPOCH FROM (NOW() - INTERVAL '1 day')) * 1000000
ORDER BY total_reward DESC;

-- View: Active policies with performance
CREATE OR REPLACE VIEW active_policies_performance AS
SELECT
    state_bin_id,
    rule_id,
    version,
    action_weights,
    total_executions,
    avg_reward,
    created_at,
    updated_at
FROM adna_policies
WHERE is_active = TRUE
ORDER BY avg_reward DESC;

-- View: Latest configurations
CREATE OR REPLACE VIEW latest_configurations AS
SELECT
    component_name,
    config_key,
    config_value,
    version,
    updated_at
FROM configuration_store
WHERE is_active = TRUE
ORDER BY component_name, config_key;

-- =============================================================================
-- GRANTS (Optional - adjust for your security model)
-- =============================================================================

-- Example: Grant read/write to neurograph_user
-- GRANT SELECT, INSERT, UPDATE ON ALL TABLES IN SCHEMA public TO neurograph_user;
-- GRANT USAGE, SELECT ON ALL SEQUENCES IN SCHEMA public TO neurograph_user;
-- GRANT EXECUTE ON FUNCTION archive_old_events(INTEGER) TO neurograph_user;

-- =============================================================================
-- END OF SCHEMA
-- =============================================================================

-- Usage:
-- psql -U postgres -d neurograph_db -f schema.sql
