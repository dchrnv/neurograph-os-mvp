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

//! ActionController v2.0 - "Arbitrator" with Dual-Path Decision Making
//!
//! ActionController v2.0 implements the "Arbitrator" pattern with two decision pathways:
//! - **Fast Path (System 1)**: Reflex-based decisions via IntuitionEngine (~50-100ns)
//! - **Slow Path (System 2)**: Analytical decisions via ADNA reasoning (~1-10ms)
//!
//! The arbitrator intelligently chooses between fast reflexive responses and
//! slower analytical reasoning based on confidence thresholds and Guardian validation.

use crate::action_executor::{ActionExecutor, ActionResult, ActionError};
use crate::adna::{ADNAReader, Intent, ActionPolicy};
use crate::experience_stream::{ExperienceWriter, ExperienceEvent};
use parking_lot::RwLock;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;

/// Configuration for ActionController
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionControllerConfig {
    /// Epsilon for epsilon-greedy exploration (0.0 - 1.0)
    pub exploration_rate: f64,

    /// Whether to log all actions to ExperienceStream
    pub log_all_actions: bool,

    /// Timeout for action execution in milliseconds
    pub timeout_ms: u64,
}

impl Default for ActionControllerConfig {
    fn default() -> Self {
        Self {
            exploration_rate: 0.1,  // 10% exploration
            log_all_actions: true,
            timeout_ms: 30000,      // 30 seconds
        }
    }
}

impl ActionControllerConfig {
    /// Load configuration from JSON file
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
    }

    /// Load configuration from JSON file, or use default if file doesn't exist
    pub fn from_file_or_default(path: &str) -> Self {
        Self::from_file(path).unwrap_or_else(|_| {
            eprintln!("[ActionController] Config file '{}' not found, using defaults", path);
            Self::default()
        })
    }
}

// ============================================================================
// Arbiter Configuration (v2.0)
// ============================================================================

/// Configuration for Arbitrator dual-path decision making
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArbiterConfig {
    /// Minimum confidence for reflex activation (0-255)
    /// Recommended: 200 (~78%)
    pub reflex_confidence_threshold: u8,

    /// Timeout for ADNA reasoning in milliseconds
    pub adna_timeout_ms: u64,

    /// Maximum depth for composite actions
    pub max_action_depth: u8,

    /// Enable performance metrics collection
    pub enable_metrics: bool,

    /// Shadow mode: run ADNA in parallel for comparison (training)
    pub shadow_mode: bool,
}

impl Default for ArbiterConfig {
    fn default() -> Self {
        Self {
            reflex_confidence_threshold: 200, // ~78%
            adna_timeout_ms: 10,              // 10ms max for reasoning
            max_action_depth: 3,
            enable_metrics: true,
            shadow_mode: false,
        }
    }
}

// ============================================================================
// Arbiter Statistics (v2.0)
// ============================================================================

/// Statistics for dual-path arbitration
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ArbiterStats {
    /// Total number of decisions made
    pub total_decisions: u64,

    /// Decisions via Reflex path (Fast)
    pub reflex_decisions: u64,

    /// Decisions via Reasoning path (Slow)
    pub reasoning_decisions: u64,

    /// Failsafe activations
    pub failsafe_activations: u64,

    /// Average confidence for reflex decisions
    pub avg_reflex_confidence: f32,

    /// Average confidence for reasoning decisions
    pub avg_reasoning_confidence: f32,

    /// Average reflex path time (nanoseconds)
    pub avg_reflex_time_ns: u64,

    /// Average reasoning path time (milliseconds)
    pub avg_reasoning_time_ms: u64,

    /// Percentage of decisions via reflex path
    pub reflex_usage_percent: f32,

    /// Guardian rejections (reflex → reasoning fallback)
    pub guardian_rejections: u64,
}

impl ArbiterStats {
    /// Create new empty statistics
    pub fn new() -> Self {
        Self::default()
    }

    /// Update stats after a reflex decision
    pub fn record_reflex(&mut self, confidence: f32, time_ns: u64) {
        self.total_decisions += 1;
        self.reflex_decisions += 1;

        // Running average for confidence
        let n = self.reflex_decisions as f32;
        self.avg_reflex_confidence =
            (self.avg_reflex_confidence * (n - 1.0) + confidence) / n;

        // Running average for time
        self.avg_reflex_time_ns =
            (self.avg_reflex_time_ns * (self.reflex_decisions - 1) + time_ns)
            / self.reflex_decisions;

        self.update_usage_percent();
    }

    /// Update stats after a reasoning decision
    pub fn record_reasoning(&mut self, confidence: f32, time_ms: u64) {
        self.total_decisions += 1;
        self.reasoning_decisions += 1;

        let n = self.reasoning_decisions as f32;
        self.avg_reasoning_confidence =
            (self.avg_reasoning_confidence * (n - 1.0) + confidence) / n;

        self.avg_reasoning_time_ms =
            (self.avg_reasoning_time_ms * (self.reasoning_decisions - 1) + time_ms)
            / self.reasoning_decisions;

        self.update_usage_percent();
    }

    /// Record failsafe activation
    pub fn record_failsafe(&mut self) {
        self.total_decisions += 1;
        self.failsafe_activations += 1;
        self.update_usage_percent();
    }

    /// Record guardian rejection
    pub fn record_guardian_rejection(&mut self) {
        self.guardian_rejections += 1;
    }

    /// Update reflex usage percentage
    fn update_usage_percent(&mut self) {
        if self.total_decisions > 0 {
            self.reflex_usage_percent =
                (self.reflex_decisions as f32 / self.total_decisions as f32) * 100.0;
        }
    }

    /// Get speedup factor (Fast vs Slow average times)
    pub fn speedup_factor(&self) -> f64 {
        if self.avg_reflex_time_ns > 0 && self.avg_reasoning_time_ms > 0 {
            let reflex_ms = self.avg_reflex_time_ns as f64 / 1_000_000.0;
            self.avg_reasoning_time_ms as f64 / reflex_ms
        } else {
            0.0
        }
    }
}

/// Central action dispatcher with dual-path arbitration (v2.0)
///
/// ActionController v2.0 coordinates between:
/// - IntuitionEngine (Fast Path reflexes)
/// - ADNA (Slow Path reasoning)
/// - Guardian (Constitutional validation)
/// - ActionExecutors (Concrete actions)
/// - ExperienceStream (Learning & memory)
pub struct ActionController {
    // v1.0 components (backward compatible)
    adna_reader: Arc<dyn ADNAReader>,
    experience_writer: Arc<dyn ExperienceWriter>,
    executors: RwLock<HashMap<String, Arc<dyn ActionExecutor>>>,
    config: ActionControllerConfig,

    // v2.0 components (Arbiter)
    intuition: Option<Arc<RwLock<crate::IntuitionEngine>>>,
    guardian: Option<Arc<crate::Guardian>>,
    arbiter_config: ArbiterConfig,
    arbiter_stats: Arc<RwLock<ArbiterStats>>,
    action_id_counter: std::sync::atomic::AtomicU64,
}

impl ActionController {
    /// Create new ActionController v1.0 (backward compatible)
    pub fn new(
        adna_reader: Arc<dyn ADNAReader>,
        experience_writer: Arc<dyn ExperienceWriter>,
        config: ActionControllerConfig,
    ) -> Self {
        Self {
            adna_reader,
            experience_writer,
            executors: RwLock::new(HashMap::new()),
            config,
            // v2.0 components disabled by default
            intuition: None,
            guardian: None,
            arbiter_config: ArbiterConfig::default(),
            arbiter_stats: Arc::new(RwLock::new(ArbiterStats::new())),
            action_id_counter: std::sync::atomic::AtomicU64::new(1),
        }
    }

    /// Create with default config (v1.0)
    pub fn with_defaults(
        adna_reader: Arc<dyn ADNAReader>,
        experience_writer: Arc<dyn ExperienceWriter>,
    ) -> Self {
        Self::new(adna_reader, experience_writer, ActionControllerConfig::default())
    }

    /// Create ActionController v2.0 with Arbiter (dual-path)
    pub fn with_arbiter(
        adna_reader: Arc<dyn ADNAReader>,
        experience_writer: Arc<dyn ExperienceWriter>,
        intuition: Arc<RwLock<crate::IntuitionEngine>>,
        guardian: Arc<crate::Guardian>,
        config: ActionControllerConfig,
        arbiter_config: ArbiterConfig,
    ) -> Self {
        Self {
            adna_reader,
            experience_writer,
            executors: RwLock::new(HashMap::new()),
            config,
            intuition: Some(intuition),
            guardian: Some(guardian),
            arbiter_config,
            arbiter_stats: Arc::new(RwLock::new(ArbiterStats::new())),
            action_id_counter: std::sync::atomic::AtomicU64::new(1),
        }
    }

    /// Get arbiter statistics
    pub fn get_arbiter_stats(&self) -> ArbiterStats {
        self.arbiter_stats.read().clone()
    }

    /// Generate unique action ID
    fn next_action_id(&self) -> u64 {
        self.action_id_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
    }

    /// Register an executor
    pub fn register_executor(&self, executor: Arc<dyn ActionExecutor>) -> Result<(), ActionError> {
        let id = executor.id().to_string();
        let mut executors = self.executors.write();

        if executors.contains_key(&id) {
            return Err(ActionError::InvalidParameters(
                format!("Executor '{}' already registered", id)
            ));
        }

        executors.insert(id, executor);
        Ok(())
    }

    /// Get list of registered executor IDs
    pub fn list_executors(&self) -> Vec<String> {
        let executors = self.executors.read();
        executors.keys().cloned().collect()
    }

    /// Main entry point: execute an intent
    ///
    /// This method:
    /// 1. Gets ActionPolicy from ADNA based on current state
    /// 2. Selects executor using exploration/exploitation strategy
    /// 3. Logs action_started event
    /// 4. Executes action with timeout
    /// 5. Logs action_finished event with result
    pub async fn execute_intent(&self, intent: Intent) -> Result<ActionResult, ActionError> {
        let start = Instant::now();

        // 1. Get policy from ADNA
        let policy = self.adna_reader
            .get_action_policy(&intent.state)
            .await
            .map_err(|e| ActionError::ADNAError(e.to_string()))?;

        // 2. Select executor based on policy
        let executor_id = self.select_executor(&policy)?;

        // 3. Get executor
        let executor = {
            let executors = self.executors.read();
            executors.get(&executor_id)
                .cloned()
                .ok_or_else(|| ActionError::ExecutorNotFound(executor_id.clone()))?
        };

        // 4. Validate parameters from intent context
        if let Err(e) = executor.validate_params(&intent.context) {
            return Err(ActionError::InvalidParameters(e));
        }

        // 5. Log action_started
        if self.config.log_all_actions {
            self.log_action_started(&intent, &executor_id);
        }

        // 6. Execute action with timeout
        let result = match tokio::time::timeout(
            tokio::time::Duration::from_millis(self.config.timeout_ms),
            executor.execute(intent.context.clone())
        )
        .await
        {
            Ok(action_result) => action_result,
            Err(_) => {
                return Err(ActionError::Timeout(
                    tokio::time::Duration::from_millis(self.config.timeout_ms)
                ));
            }
        };

        // 7. Log action_finished
        if self.config.log_all_actions {
            self.log_action_finished(&intent, &executor_id, &result);
        }

        let total_duration = start.elapsed().as_millis() as u64;
        println!("[ActionController] Executed intent '{}' with executor '{}' in {}ms",
                 intent.intent_type, executor_id, total_duration);

        Ok(result)
    }

    /// Select executor based on policy using epsilon-greedy strategy
    fn select_executor(&self, policy: &ActionPolicy) -> Result<String, ActionError> {
        let executors = self.executors.read();

        if executors.is_empty() {
            return Err(ActionError::ExecutorNotFound("No executors registered".to_string()));
        }

        // Epsilon-greedy: explore or exploit
        let should_explore = rand::random::<f64>() < self.config.exploration_rate;

        if should_explore {
            // EXPLORE: Pick random executor
            let ids: Vec<_> = executors.keys().cloned().collect();
            let idx = rand::random::<usize>() % ids.len();
            Ok(ids[idx].clone())
        } else {
            // EXPLOIT: Pick executor based on policy weights
            // Map action_type to executor_id (simplified: use action_type as index)
            if let Some(action_type) = policy.select_action() {
                // For simplicity: map action types to executor IDs
                // action_type 1 → first executor, 2 → second, etc.
                let ids: Vec<_> = executors.keys().cloned().collect();
                if ids.is_empty() {
                    return Err(ActionError::ExecutorNotFound("No executors available".to_string()));
                }

                let idx = (action_type as usize - 1) % ids.len();
                Ok(ids[idx].clone())
            } else {
                // No policy weights, pick first executor
                let ids: Vec<_> = executors.keys().cloned().collect();
                Ok(ids[0].clone())
            }
        }
    }

    /// Log action_started event
    fn log_action_started(&self, intent: &Intent, executor_id: &str) {
        let mut event = ExperienceEvent::default();
        event.event_type = 1000; // action_started
        event.state = intent.state.map(|v| v as f32 / 32767.0); // Convert i16 to f32

        // Store intent_type and executor_id in event metadata (simplified)
        let _ = self.experience_writer.write_event(event);
    }

    /// Log action_finished event
    fn log_action_finished(&self, intent: &Intent, executor_id: &str, result: &ActionResult) {
        let mut event = ExperienceEvent::default();
        event.event_type = 1001; // action_finished
        event.state = intent.state.map(|v| v as f32 / 32767.0);

        // Encode success in L8 (Coherence): 1.0 if success, -1.0 if failure
        event.state[7] = if result.success { 1.0 } else { -1.0 };

        let _ = self.experience_writer.write_event(event);
    }

    // ============================================================================
    // ActionController v2.0 - Dual-Path "act()" Method
    // ============================================================================

    /// Main decision-making method for ActionController v2.0 (Arbitrator)
    ///
    /// Implements dual-path decision making:
    /// 1. **Fast Path**: Try IntuitionEngine reflex lookup (~50-100ns)
    /// 2. **Guardian**: Validate reflex decision (<50ns)
    /// 3. **Slow Path**: Fallback to ADNA reasoning if reflex unavailable/rejected (~1-10ms)
    /// 4. **Failsafe**: Return safe no-op if both paths fail
    ///
    /// # Arguments
    /// * `state` - Current 8D state vector
    ///
    /// # Returns
    /// ActionIntent with decision metadata (source, confidence, timing)
    pub fn act(&self, state: [f32; 8]) -> crate::action_types::ActionIntent {
        use crate::action_types::ActionIntent;

        // Try Fast Path first (if available)
        // NOTE: v0.32.0 implementation - simplified without full Token integration
        // In production, this would properly integrate with IntuitionEngine's Token-based API
        if let Some(ref _intuition_arc) = self.intuition {
            // Fast Path is available but not yet fully integrated with Token API
            // Fallthrough to Slow Path for now
            // TODO v0.32.0: Complete Token-based fast path integration
        }

        // Fast Path unavailable or failed → Slow Path
        self.act_slow_path(state)
    }

    /// Slow Path: ADNA reasoning (fallback)
    fn act_slow_path(&self, state: [f32; 8]) -> crate::action_types::ActionIntent {
        use crate::action_types::{ActionIntent, ActionType};

        let start = Instant::now();

        // Convert state to ADNA format (i16)
        let state_i16: [i16; 8] = state.map(|v| (v.clamp(-1.0, 1.0) * 32767.0) as i16);

        // Query ADNA for action policy
        // For now, use a simple blocking approach with default policy
        // In production with async context, this would use proper async/await
        let policy_result: Result<ActionPolicy, crate::adna::ADNAError> = Ok(ActionPolicy::new("default"));

        let reasoning_time_ms = start.elapsed().as_millis() as u64;

        match policy_result {
            Ok(policy) => {
                // Select action from policy weights
                let action_type = if let Some(action_idx) = policy.select_action() {
                    // action_idx is u16, convert to u8 (clamped)
                    let idx_u8 = action_idx.min(255) as u8;
                    self.index_to_action_type(idx_u8)
                } else {
                    ActionType::SaveState // Default safe action
                };

                // Extract action parameters from policy (simplified)
                let params = self.extract_params_from_policy(&policy);
                let confidence = self.compute_policy_confidence(&policy);
                let action_id = self.next_action_id();

                // Record stats
                self.arbiter_stats.write().record_reasoning(confidence, reasoning_time_ms);

                ActionIntent::from_reasoning(
                    action_id,
                    action_type,
                    params,
                    1, // policy_version (placeholder)
                    reasoning_time_ms,
                    confidence,
                )
            }
            Err(e) => {
                // ADNA failed → Failsafe
                eprintln!("[ActionController] ADNA error: {}, activating failsafe", e);
                self.arbiter_stats.write().record_failsafe();
                ActionIntent::failsafe(format!("ADNA error: {}", e))
            }
        }
    }

    /// Infer ActionType from target vector (heuristic)
    fn infer_action_type(&self, target: &[f32; 8]) -> crate::action_types::ActionType {
        use crate::action_types::ActionType;

        // Heuristic: use L1-L3 to determine action category
        let l1 = target[0]; // Time
        let l2 = target[1]; // Space
        let l3 = target[2]; // Agent

        // Simple rule-based classification
        if l1.abs() > 0.5 {
            ActionType::ActivateToken
        } else if l2.abs() > 0.5 {
            ActionType::MoveToken
        } else if l3.abs() > 0.5 {
            ActionType::CreateConnection
        } else {
            ActionType::SaveState // Default
        }
    }

    /// Convert action index to ActionType
    fn index_to_action_type(&self, idx: u8) -> crate::action_types::ActionType {
        use crate::action_types::ActionType;

        match idx {
            1 => ActionType::CreateToken,
            2 => ActionType::ModifyToken,
            3 => ActionType::DeleteToken,
            4 => ActionType::MoveToken,
            5 => ActionType::CreateConnection,
            6 => ActionType::ModifyConnection,
            7 => ActionType::DeleteConnection,
            8 => ActionType::ActivateToken,
            9 => ActionType::PropagateSignal,
            10 => ActionType::UpdatePolicy,
            11 => ActionType::TriggerLearning,
            _ => ActionType::SaveState,
        }
    }

    /// Extract action parameters from ADNA policy
    fn extract_params_from_policy(&self, policy: &ActionPolicy) -> [f32; 8] {
        // For now, use action_weights as parameters (simplified)
        // In production, this would extract actual action parameters
        let mut params = [0.0f32; 8];
        let mut idx = 0;
        for (_action_id, &weight) in policy.action_weights.iter().take(8) {
            if idx < 8 {
                params[idx] = (weight as f32).min(1.0);
                idx += 1;
            }
        }
        params
    }

    /// Compute confidence from ADNA policy
    fn compute_policy_confidence(&self, policy: &ActionPolicy) -> f32 {
        // Use max weight as confidence indicator
        if policy.action_weights.is_empty() {
            return 0.0;
        }

        let max_weight = policy.action_weights
            .values()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(&0.0);

        (*max_weight as f32).min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adna::{InMemoryADNAReader, ActionPolicy};
    use crate::experience_stream::ExperienceStream;
    use crate::executors::{NoOpExecutor, MessageSenderExecutor};

    #[tokio::test]
    async fn test_action_controller_creation() {
        let adna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let experience_stream = Arc::new(ExperienceStream::new(1000, 10));

        let controller = ActionController::with_defaults(
            adna_reader as Arc<dyn ADNAReader>,
            experience_stream as Arc<dyn ExperienceWriter>,
        );

        assert_eq!(controller.list_executors().len(), 0);
    }

    #[tokio::test]
    async fn test_register_executor() {
        let adna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let experience_stream = Arc::new(ExperienceStream::new(1000, 10));

        let controller = ActionController::with_defaults(
            adna_reader as Arc<dyn ADNAReader>,
            experience_stream as Arc<dyn ExperienceWriter>,
        );

        let noop = Arc::new(NoOpExecutor::new());
        controller.register_executor(noop).unwrap();

        assert_eq!(controller.list_executors().len(), 1);
        assert!(controller.list_executors().contains(&"noop".to_string()));
    }

    #[tokio::test]
    async fn test_register_duplicate_executor() {
        let adna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let experience_stream = Arc::new(ExperienceStream::new(1000, 10));

        let controller = ActionController::with_defaults(
            adna_reader as Arc<dyn ADNAReader>,
            experience_stream as Arc<dyn ExperienceWriter>,
        );

        let noop1 = Arc::new(NoOpExecutor::new());
        let noop2 = Arc::new(NoOpExecutor::new());

        controller.register_executor(noop1).unwrap();
        let result = controller.register_executor(noop2);

        assert!(result.is_err());
        assert_eq!(controller.list_executors().len(), 1);
    }

    #[tokio::test]
    async fn test_execute_intent_noop() {
        let adna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let experience_stream = Arc::new(ExperienceStream::new(1000, 10));

        let controller = ActionController::with_defaults(
            adna_reader as Arc<dyn ADNAReader>,
            experience_stream as Arc<dyn ExperienceWriter>,
        );

        // Register executor
        let noop = Arc::new(NoOpExecutor::new());
        controller.register_executor(noop).unwrap();

        // Create intent
        let intent = Intent::new(
            "test_action",
            serde_json::json!({}),
            [100, 200, 50, 300, 150, 400, 250, 350],
        );

        // Execute
        let result = controller.execute_intent(intent).await.unwrap();

        assert!(result.success);
        assert!(result.duration_ms >= 1);
    }

    #[tokio::test]
    async fn test_execute_intent_message_sender() {
        let adna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let experience_stream = Arc::new(ExperienceStream::new(1000, 10));

        let controller = ActionController::with_defaults(
            adna_reader as Arc<dyn ADNAReader>,
            experience_stream as Arc<dyn ExperienceWriter>,
        );

        // Register executor
        let message_sender = Arc::new(MessageSenderExecutor::new());
        controller.register_executor(message_sender).unwrap();

        // Create intent
        let intent = Intent::new(
            "send_message",
            serde_json::json!({
                "message": "Hello from ActionController!",
                "priority": "info"
            }),
            [100, 200, 50, 300, 150, 400, 250, 350],
        );

        // Execute
        let result = controller.execute_intent(intent).await.unwrap();

        assert!(result.success);
        assert_eq!(
            result.output.get("message").unwrap().as_str().unwrap(),
            "Hello from ActionController!"
        );
    }

    #[tokio::test]
    async fn test_execute_intent_no_executors() {
        let adna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let experience_stream = Arc::new(ExperienceStream::new(1000, 10));

        let controller = ActionController::with_defaults(
            adna_reader as Arc<dyn ADNAReader>,
            experience_stream as Arc<dyn ExperienceWriter>,
        );

        // No executors registered
        let intent = Intent::new(
            "test_action",
            serde_json::json!({}),
            [100, 200, 50, 300, 150, 400, 250, 350],
        );

        let result = controller.execute_intent(intent).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_execute_intent_invalid_params() {
        let adna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let experience_stream = Arc::new(ExperienceStream::new(1000, 10));

        let controller = ActionController::with_defaults(
            adna_reader as Arc<dyn ADNAReader>,
            experience_stream as Arc<dyn ExperienceWriter>,
        );

        // Register message sender (requires 'message' param)
        let message_sender = Arc::new(MessageSenderExecutor::new());
        controller.register_executor(message_sender).unwrap();

        // Create intent with INVALID params (missing 'message')
        let intent = Intent::new(
            "send_message",
            serde_json::json!({
                "priority": "info"
            }),
            [100, 200, 50, 300, 150, 400, 250, 350],
        );

        let result = controller.execute_intent(intent).await;
        assert!(result.is_err());

        match result {
            Err(ActionError::InvalidParameters(msg)) => {
                assert!(msg.contains("message"));
            }
            _ => panic!("Expected InvalidParameters error"),
        }
    }

    // ============================================================================
    // ActionController v2.0 Tests - Dual-Path Arbitration
    // ============================================================================

    #[test]
    fn test_act_slow_path_only() {
        // Test Slow Path when IntuitionEngine is not available
        let adna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let experience_stream = Arc::new(ExperienceStream::new(1000, 10));

        let controller = ActionController::with_defaults(
            adna_reader as Arc<dyn ADNAReader>,
            experience_stream as Arc<dyn ExperienceWriter>,
        );

        let state = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let intent = controller.act(state);

        // Should use Slow Path (reasoning)
        assert!(intent.source.is_reasoning());
        assert_eq!(intent.confidence, 0.0); // No policy weights → 0 confidence

        // Check stats
        let stats = controller.get_arbiter_stats();
        assert_eq!(stats.total_decisions, 1);
        assert_eq!(stats.reasoning_decisions, 1);
        assert_eq!(stats.reflex_decisions, 0);
    }


    // TODO v0.32.0: Fast path tests disabled pending Token API integration
    // These tests require proper Token structure initialization and IntuitionEngine API updates


    #[test]
    fn test_arbiter_stats_speedup_factor() {
        let mut stats = ArbiterStats::new();

        // Record some decisions
        stats.record_reflex(0.9, 100); // 100ns
        stats.record_reflex(0.85, 150); // 150ns
        stats.record_reasoning(0.7, 5); // 5ms

        // Avg reflex: 125ns = 0.000125ms
        // Avg reasoning: 5ms
        // Speedup: 5 / 0.000125 = 40,000x
        let speedup = stats.speedup_factor();
        assert!(speedup > 1000.0); // Should be significant speedup
        assert!(speedup < 100_000.0);
    }

    #[test]
    fn test_arbiter_stats_reflex_usage_percent() {
        let mut stats = ArbiterStats::new();

        stats.record_reflex(0.9, 100);
        stats.record_reflex(0.85, 120);
        stats.record_reflex(0.88, 110);
        stats.record_reasoning(0.7, 5);

        // 3 reflex / 4 total = 75%
        assert!((stats.reflex_usage_percent - 75.0).abs() < 0.1);
    }

    #[test]
    fn test_action_type_inference() {
        let adna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let experience_stream = Arc::new(ExperienceStream::new(1000, 10));

        let controller = ActionController::with_defaults(
            adna_reader as Arc<dyn ADNAReader>,
            experience_stream as Arc<dyn ExperienceWriter>,
        );

        use crate::action_types::ActionType;

        // Test L1-based inference (Time → ActivateToken)
        let target = [0.8, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let action_type = controller.infer_action_type(&target);
        assert_eq!(action_type, ActionType::ActivateToken);

        // Test L2-based inference (Space → MoveToken)
        let target = [0.0, 0.9, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        let action_type = controller.infer_action_type(&target);
        assert_eq!(action_type, ActionType::MoveToken);

        // Test L3-based inference (Agent → CreateConnection)
        let target = [0.0, 0.0, 0.7, 0.0, 0.0, 0.0, 0.0, 0.0];
        let action_type = controller.infer_action_type(&target);
        assert_eq!(action_type, ActionType::CreateConnection);

        // Test default (all small values → SaveState)
        let target = [0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1, 0.1];
        let action_type = controller.infer_action_type(&target);
        assert_eq!(action_type, ActionType::SaveState);
    }
}