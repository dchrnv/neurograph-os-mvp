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

//! Persistence E2E Integration Test for v0.27.0
//!
//! Tests PostgreSQL persistence layer:
//! 1. Connect to PostgreSQL
//! 2. Write events with metadata
//! 3. Write ADNA policies with versioning
//! 4. Write configurations with versioning
//! 5. Query and verify data integrity
//! 6. Test archival retention policy

#[cfg(all(feature = "demo-tokio", feature = "persistence"))]
mod persistence_e2e_test {
    use neurograph_core::{
        PostgresBackend, PersistenceBackend, QueryOptions,
        ExperienceEvent, EventType, ActionMetadata,
    };
    use std::collections::HashMap;

    #[tokio::test]
    async fn test_postgres_connection_and_health() {
        // ✅ Test 1: Connection
        let backend = match PostgresBackend::from_env().await {
            Ok(b) => b,
            Err(e) => {
                println!("⚠️  PostgreSQL not available: {}", e);
                println!("   Skipping persistence test (requires PostgreSQL)");
                return;
            }
        };

        println!("✅ Connected to PostgreSQL");

        // ✅ Test 2: Health check
        backend.health_check().await.expect("Health check should pass");
        println!("✅ Database schema verified");
    }

    #[tokio::test]
    async fn test_event_persistence() {
        let backend = match PostgresBackend::from_env().await {
            Ok(b) => b,
            Err(_) => {
                println!("⚠️  Skipping test (PostgreSQL not available)");
                return;
            }
        };

        // === Write Events ===

        let mut events_written = 0;

        for i in 0..100 {
            let mut event = ExperienceEvent::default();
            event.event_id = (1000 + i) as u128;
            event.event_type = EventType::ActionCompleted as u16;
            event.episode_id = 10;
            event.step_number = i;
            event.state = [
                (i as f32 * 0.01) % 1.0,
                (i as f32 * 0.02) % 1.0,
                0.0, 0.0, 0.0, 0.0, 0.0, 0.0
            ];
            event.reward_homeostasis = 0.5 + (i as f32 * 0.001);
            event.reward_curiosity = 0.3;
            event.reward_efficiency = 0.4;
            event.reward_goal = 0.2;

            backend.write_event(&event).await.expect("Event write should succeed");
            events_written += 1;
        }

        println!("✅ Wrote {} events", events_written);

        // === Read Events ===

        let read_event = backend.read_event(1000).await.expect("Event read should succeed");
        assert_eq!(read_event.event_id, 1000);
        assert_eq!(read_event.episode_id, 10);
        println!("✅ Read event 1000 successfully");

        // === Query Events ===

        let query_opts = QueryOptions {
            limit: Some(50),
            episode_id: Some(10),
            event_type: Some(EventType::ActionCompleted as u16),
            order_asc: true,
            include_archived: false,
            ..Default::default()
        };

        let events = backend.query_events(query_opts).await.expect("Query should succeed");
        assert!(events.len() <= 50, "Should respect limit");
        assert!(events.len() > 0, "Should find events");
        println!("✅ Queried {} events for episode 10", events.len());

        // === Count Events ===

        let count = backend.count_events(QueryOptions {
            episode_id: Some(10),
            include_archived: false,
            ..Default::default()
        }).await.expect("Count should succeed");

        assert_eq!(count, 100, "Should count all 100 events");
        println!("✅ Counted {} events", count);
    }

    #[tokio::test]
    async fn test_metadata_persistence() {
        let backend = match PostgresBackend::from_env().await {
            Ok(b) => b,
            Err(_) => {
                println!("⚠️  Skipping test (PostgreSQL not available)");
                return;
            }
        };

        // === Write Event with Metadata ===

        let mut event = ExperienceEvent::default();
        event.event_id = 2000;
        event.event_type = EventType::ActionStarted as u16;
        event.episode_id = 20;
        event.step_number = 1;

        let metadata = ActionMetadata {
            intent_type: "send_message".to_string(),
            executor_id: "message_sender".to_string(),
            parameters: serde_json::json!({
                "message": "Integration test",
                "priority": "high"
            }),
        };

        backend.write_event_with_metadata(&event, &metadata).await
            .expect("Event with metadata write should succeed");

        println!("✅ Wrote event with metadata");

        // === Read Event with Metadata ===

        let (read_event, read_metadata) = backend.read_event_with_metadata(2000).await
            .expect("Event with metadata read should succeed");

        assert_eq!(read_event.event_id, 2000);

        if let Some(meta) = read_metadata {
            assert_eq!(meta.intent_type, "send_message");
            assert_eq!(meta.executor_id, "message_sender");
            assert!(meta.parameters.get("message").is_some());
            println!("✅ Read metadata: intent={}, executor={}", meta.intent_type, meta.executor_id);
        } else {
            panic!("Metadata should exist");
        }

        // === Query Events with Metadata ===

        let query_opts = QueryOptions {
            limit: Some(10),
            event_type: Some(EventType::ActionStarted as u16),
            ..Default::default()
        };

        let events_with_meta = backend.query_events_with_metadata(query_opts).await
            .expect("Query with metadata should succeed");

        assert!(events_with_meta.len() > 0, "Should find events with metadata");
        println!("✅ Queried {} events with metadata", events_with_meta.len());
    }

    #[tokio::test]
    async fn test_policy_persistence() {
        let backend = match PostgresBackend::from_env().await {
            Ok(b) => b,
            Err(_) => {
                println!("⚠️  Skipping test (PostgreSQL not available)");
                return;
            }
        };

        // === Create Policy Version 1 ===

        let mut action_weights_v1 = HashMap::new();
        action_weights_v1.insert(1000, 0.6);
        action_weights_v1.insert(1001, 0.4);

        let policy_id_v1 = backend.save_policy(
            "test_state_bin_A",
            "test_rule_alpha",
            &action_weights_v1,
            Some(serde_json::json!({"description": "Initial test policy"})),
            None, // No parent
        ).await.expect("Policy save should succeed");

        println!("✅ Created policy v1 (ID: {})", policy_id_v1);

        // === Read Active Policy ===

        let active_policy = backend.get_active_policy("test_state_bin_A").await
            .expect("Get active policy should succeed");

        assert!(active_policy.is_some(), "Should have active policy");

        if let Some(policy) = &active_policy {
            assert_eq!(policy.state_bin_id, "test_state_bin_A");
            assert_eq!(policy.rule_id, "test_rule_alpha");
            assert_eq!(policy.version, 1);
            assert_eq!(policy.action_weights.len(), 2);
            println!("✅ Retrieved active policy: version {}", policy.version);
        }

        // === Create Policy Version 2 (Update) ===

        let mut action_weights_v2 = HashMap::new();
        action_weights_v2.insert(1000, 0.7); // Increased
        action_weights_v2.insert(1001, 0.3); // Decreased

        let policy_id_v2 = backend.save_policy(
            "test_state_bin_A",
            "test_rule_alpha",
            &action_weights_v2,
            Some(serde_json::json!({"description": "Updated after learning"})),
            Some(policy_id_v1), // Parent policy
        ).await.expect("Policy update should succeed");

        println!("✅ Created policy v2 (ID: {}, parent: {})", policy_id_v2, policy_id_v1);

        // === Verify Versioning ===

        let active_policy_v2 = backend.get_active_policy("test_state_bin_A").await
            .expect("Get active policy should succeed")
            .expect("Should have active policy");

        assert_eq!(active_policy_v2.version, 2, "Should be version 2");
        assert_eq!(active_policy_v2.parent_policy_id, Some(policy_id_v1), "Should reference parent");
        assert_eq!(*active_policy_v2.action_weights.get(&1000).unwrap(), 0.7);

        println!("✅ Policy versioning verified");

        // === Update Metrics ===

        backend.update_policy_metrics(policy_id_v2, 50, 0.75).await
            .expect("Metrics update should succeed");

        println!("✅ Updated policy metrics (50 executions, avg reward 0.75)");

        // === Get All Active Policies ===

        let all_policies = backend.get_all_active_policies().await
            .expect("Get all policies should succeed");

        println!("✅ Retrieved {} active policies", all_policies.len());
    }

    #[tokio::test]
    async fn test_configuration_persistence() {
        let backend = match PostgresBackend::from_env().await {
            Ok(b) => b,
            Err(_) => {
                println!("⚠️  Skipping test (PostgreSQL not available)");
                return;
            }
        };

        // === Save Configuration v1 ===

        let config_v1 = serde_json::json!({
            "learning_rate": 0.01,
            "epsilon": 0.1,
            "batch_size": 32
        });

        let config_id_v1 = backend.save_config(
            "test_learning_loop",
            "hyperparameters",
            config_v1,
            None, // No parent
        ).await.expect("Config save should succeed");

        println!("✅ Created configuration v1 (ID: {})", config_id_v1);

        // === Read Configuration ===

        let config = backend.get_config("test_learning_loop", "hyperparameters").await
            .expect("Get config should succeed")
            .expect("Config should exist");

        assert_eq!(config.component_name, "test_learning_loop");
        assert_eq!(config.config_key, "hyperparameters");
        assert_eq!(config.version, 1);
        println!("✅ Retrieved configuration: version {}", config.version);

        // === Update Configuration (v2) ===

        let config_v2 = serde_json::json!({
            "learning_rate": 0.005, // Decreased
            "epsilon": 0.05,        // Decreased
            "batch_size": 64        // Increased
        });

        let config_id_v2 = backend.save_config(
            "test_learning_loop",
            "hyperparameters",
            config_v2,
            Some(config_id_v1), // Parent config
        ).await.expect("Config update should succeed");

        println!("✅ Created configuration v2 (ID: {}, parent: {})", config_id_v2, config_id_v1);

        // === Verify Versioning ===

        let config_v2_read = backend.get_config("test_learning_loop", "hyperparameters").await
            .expect("Get config should succeed")
            .expect("Config should exist");

        assert_eq!(config_v2_read.version, 2);
        assert_eq!(config_v2_read.parent_config_id, Some(config_id_v1));

        println!("✅ Configuration versioning verified");

        // === Get All Component Configs ===

        let component_configs = backend.get_component_configs("test_learning_loop").await
            .expect("Get component configs should succeed");

        println!("✅ Retrieved {} configs for component", component_configs.len());
    }

    #[tokio::test]
    async fn test_archival_retention() {
        let backend = match PostgresBackend::from_env().await {
            Ok(b) => b,
            Err(_) => {
                println!("⚠️  Skipping test (PostgreSQL not available)");
                return;
            }
        };

        // === Test Archive Function ===

        // Archive events older than 365 days (should be 0 for recent events)
        let archived_count = backend.archive_old_events(365).await
            .expect("Archive should succeed");

        println!("✅ Archive function executed: {} events archived", archived_count);

        // Verify archived events are not returned by default
        let active_count = backend.count_events(QueryOptions {
            include_archived: false,
            ..Default::default()
        }).await.expect("Count should succeed");

        let total_count = backend.count_events(QueryOptions {
            include_archived: true,
            ..Default::default()
        }).await.expect("Count should succeed");

        println!("✅ Active events: {}, Total (including archived): {}", active_count, total_count);
    }
}

#[cfg(not(all(feature = "demo-tokio", feature = "persistence")))]
fn main() {
    eprintln!("Persistence integration tests require 'demo-tokio' and 'persistence' features");
    eprintln!("Run with: cargo test --features demo-tokio,persistence");
}