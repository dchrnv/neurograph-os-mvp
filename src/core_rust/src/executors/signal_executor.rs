// NeuroGraph - Высокопроизводительная система пространственных вычислений на основе токенов.
// Copyright (C) 2024-2025 Chernov Denys
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! SignalExecutor - Spreading activation through Graph
//!
//! Executes neural signal propagation using spreading activation algorithm.
//! Part of SignalSystem v1.0.

use crate::action_executor::{ActionExecutor, ActionResult};
use crate::graph::{Graph, SignalConfig};
use async_trait::async_trait;
use serde_json::{json, Value};
use std::sync::{Arc, RwLock};
use std::time::Instant;

/// Executor for spreading activation signals through graph
///
/// Integrates SignalSystem v1.0 with ActionController.
///
/// # Parameters (JSON)
///
/// ```json
/// {
///   "source_id": 123,           // Node ID to start activation
///   "initial_energy": 1.0,      // Starting energy [0.0, inf]
///   "decay_rate": 0.2,          // Optional: energy decay [0.0, 1.0]
///   "max_depth": 5,             // Optional: max hops
///   "min_energy": 0.01,         // Optional: cutoff threshold
///   "accumulation_mode": "sum"  // Optional: "sum", "max", "weighted_average"
/// }
/// ```
pub struct SignalExecutor {
    graph: Arc<RwLock<Graph>>,
}

impl SignalExecutor {
    /// Create new SignalExecutor with graph reference
    pub fn new(graph: Arc<RwLock<Graph>>) -> Self {
        Self { graph }
    }

    /// Parse accumulation mode from string
    fn parse_accumulation_mode(mode_str: &str) -> Result<crate::graph::AccumulationMode, String> {
        match mode_str.to_lowercase().as_str() {
            "sum" => Ok(crate::graph::AccumulationMode::Sum),
            "max" => Ok(crate::graph::AccumulationMode::Max),
            "weighted_average" | "weighted" => Ok(crate::graph::AccumulationMode::WeightedAverage),
            _ => Err(format!("Invalid accumulation mode: {}", mode_str)),
        }
    }
}

#[async_trait]
impl ActionExecutor for SignalExecutor {
    fn id(&self) -> &str {
        "signal_executor"
    }

    fn description(&self) -> &str {
        "Spreading activation executor (SignalSystem v1.0)"
    }

    async fn execute(&self, params: Value) -> ActionResult {
        let start = Instant::now();

        // Extract required parameters
        let source_id = match params.get("source_id") {
            Some(Value::Number(n)) => n.as_u64().unwrap_or(0) as u32,
            _ => {
                return ActionResult::failure(
                    "Missing or invalid 'source_id' parameter".to_string(),
                    start.elapsed().as_millis() as u64,
                );
            }
        };

        let initial_energy = match params.get("initial_energy") {
            Some(Value::Number(n)) => n.as_f64().unwrap_or(1.0) as f32,
            _ => 1.0, // Default
        };

        // Build custom config if parameters provided
        let custom_config = if params.get("decay_rate").is_some()
            || params.get("max_depth").is_some()
            || params.get("min_energy").is_some()
            || params.get("accumulation_mode").is_some()
        {
            let mut config = SignalConfig::default();

            if let Some(Value::Number(n)) = params.get("decay_rate") {
                config.decay_rate = n.as_f64().unwrap_or(0.2) as f32;
            }

            if let Some(Value::Number(n)) = params.get("max_depth") {
                config.max_depth = n.as_u64().unwrap_or(5) as usize;
            }

            if let Some(Value::Number(n)) = params.get("min_energy") {
                config.min_energy = n.as_f64().unwrap_or(0.01) as f32;
            }

            if let Some(Value::String(s)) = params.get("accumulation_mode") {
                match Self::parse_accumulation_mode(s) {
                    Ok(mode) => config.accumulation_mode = mode,
                    Err(e) => {
                        return ActionResult::failure(
                            e,
                            start.elapsed().as_millis() as u64,
                        );
                    }
                }
            }

            Some(config)
        } else {
            None
        };

        // Execute spreading activation
        let result = {
            let mut graph = self.graph.write().unwrap();
            graph.spreading_activation(source_id, initial_energy, custom_config)
        };

        // Format output
        let activated_count = result.activated_nodes.len();
        let max_energy = result.activated_nodes
            .first()
            .map(|n| n.energy)
            .unwrap_or(0.0);

        let strongest_path_nodes = result.strongest_path
            .as_ref()
            .map(|p| p.nodes.clone())
            .unwrap_or_default();

        let output = json!({
            "source_id": source_id,
            "initial_energy": initial_energy,
            "activated_count": activated_count,
            "nodes_visited": result.nodes_visited,
            "max_depth_reached": result.max_depth_reached,
            "max_energy": max_energy,
            "execution_time_us": result.execution_time_us,
            "strongest_path": strongest_path_nodes,
            "activated_nodes": result.activated_nodes.iter().map(|n| {
                json!({
                    "node_id": n.node_id,
                    "energy": n.energy,
                    "depth": n.depth,
                })
            }).collect::<Vec<_>>(),
        });

        let duration_ms = start.elapsed().as_millis() as u64;

        ActionResult::success(output, duration_ms)
    }

    fn validate_params(&self, params: &Value) -> Result<(), String> {
        // Validate source_id is present
        if !params.get("source_id").is_some() {
            return Err("Missing required parameter: source_id".to_string());
        }

        // Validate source_id is a number
        match params.get("source_id") {
            Some(Value::Number(_)) => {}
            _ => return Err("source_id must be a number".to_string()),
        }

        // Validate optional parameters
        if let Some(Value::Number(n)) = params.get("initial_energy") {
            let energy = n.as_f64().unwrap_or(0.0);
            if energy < 0.0 {
                return Err("initial_energy must be >= 0.0".to_string());
            }
        }

        if let Some(Value::Number(n)) = params.get("decay_rate") {
            let rate = n.as_f64().unwrap_or(0.0);
            if rate < 0.0 || rate > 1.0 {
                return Err("decay_rate must be in [0.0, 1.0]".to_string());
            }
        }

        if let Some(Value::Number(n)) = params.get("max_depth") {
            let depth = n.as_u64().unwrap_or(0);
            if depth == 0 {
                return Err("max_depth must be > 0".to_string());
            }
        }

        if let Some(Value::String(s)) = params.get("accumulation_mode") {
            Self::parse_accumulation_mode(s)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Graph;

    #[tokio::test]
    async fn test_signal_executor_basic() {
        // Create graph with simple chain: 1 -> 2 -> 3
        let graph = Arc::new(RwLock::new(Graph::new()));
        {
            let mut g = graph.write().unwrap();
            g.add_node(1);
            g.add_node(2);
            g.add_node(3);

            let edge_id_12 = Graph::compute_edge_id(1, 2, 0);
            g.add_edge(edge_id_12, 1, 2, 0, 0.8, false);

            let edge_id_23 = Graph::compute_edge_id(2, 3, 0);
            g.add_edge(edge_id_23, 2, 3, 0, 0.8, false);
        }

        let executor = SignalExecutor::new(graph);

        let params = json!({
            "source_id": 1,
            "initial_energy": 1.0,
        });

        let result = executor.execute(params).await;

        assert!(result.success);
        assert!(result.error.is_none());

        let output = &result.output;
        assert_eq!(output["source_id"], 1);
        assert!(output["activated_count"].as_u64().unwrap() > 0);
    }

    #[tokio::test]
    async fn test_signal_executor_custom_config() {
        let graph = Arc::new(RwLock::new(Graph::new()));
        {
            let mut g = graph.write().unwrap();
            g.add_node(1);
            g.add_node(2);

            let edge_id = Graph::compute_edge_id(1, 2, 0);
            g.add_edge(edge_id, 1, 2, 0, 1.0, false);
        }

        let executor = SignalExecutor::new(graph);

        let params = json!({
            "source_id": 1,
            "initial_energy": 1.0,
            "decay_rate": 0.1,
            "max_depth": 3,
            "accumulation_mode": "max",
        });

        let result = executor.execute(params).await;

        assert!(result.success);
    }

    #[test]
    fn test_signal_executor_validate() {
        let graph = Arc::new(RwLock::new(Graph::new()));
        let executor = SignalExecutor::new(graph);

        // Valid params
        let valid = json!({
            "source_id": 1,
            "initial_energy": 1.0,
        });
        assert!(executor.validate_params(&valid).is_ok());

        // Missing source_id
        let invalid = json!({
            "initial_energy": 1.0,
        });
        assert!(executor.validate_params(&invalid).is_err());

        // Invalid decay_rate
        let invalid = json!({
            "source_id": 1,
            "decay_rate": 1.5,
        });
        assert!(executor.validate_params(&invalid).is_err());
    }
}
