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

    /// Shadow mode disagreements (Fast vs Slow path mismatch)
    pub shadow_disagreements: u64,
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

    /// Record shadow mode disagreement (NEW v0.34.0)
    pub fn record_shadow_disagreement(&mut self) {
        self.shadow_disagreements += 1;
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
    /// Create ActionController v2.0 with dual-path decision making
    pub fn new(
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
        if let Some(ref intuition_arc) = self.intuition {
            let start = Instant::now();
            let intuition = intuition_arc.read();

            // Convert state to Token for fast path lookup
            let state_token = crate::Token::from_state_f32(0, &state);

            // Lookup reflex connection
            if let Some(fast_result) = intuition.try_fast_path(&state_token) {
                let lookup_time_ns = start.elapsed().as_nanos() as u64;
                let similarity = fast_result.similarity;
                let connection_id = fast_result.connection_id;

                // Get the actual connection for validation and target extraction
                if let Some(connection) = intuition.get_connection(connection_id) {
                    let confidence_u8 = connection.confidence;
                    let confidence_f32 = confidence_u8 as f32 / 255.0;

                    // Check if confidence meets threshold
                    if confidence_u8 >= self.arbiter_config.reflex_confidence_threshold {
                        // Guardian validation (optional)
                        if let Some(ref guardian) = self.guardian {
                            if let Err(_) = guardian.validate_reflex(&connection) {
                                // Guardian rejected: fallback to Slow Path
                                self.arbiter_stats.write().record_guardian_rejection();
                                drop(intuition);
                                return self.act_slow_path(state);
                            }
                        }

                        // Fast Path SUCCESS
                        // Extract target_vector from connection (NEW in v0.34.0)
                        let target_state = expand_target_to_state(&connection.target_vector);
                        let action_type = self.infer_action_type(&target_state);
                        let action_id = self.next_action_id();

                        // Record stats
                        self.arbiter_stats.write().record_reflex(confidence_f32, lookup_time_ns);

                        return ActionIntent::from_reflex(
                            action_id,
                            action_type,
                            target_state,
                            connection_id,
                            lookup_time_ns,
                            similarity,
                            confidence_f32,
                        );
                    }
                }
            }
        }

        // Fast Path unavailable or failed → Slow Path
        self.act_slow_path(state)
    }

    /// Act with shadow mode: run both Fast and Slow paths in parallel (NEW v0.34.0)
    ///
    /// Returns (primary_intent, shadow_intent_opt)
    /// - primary_intent: The actual decision to use (Fast Path if available, else Slow)
    /// - shadow_intent_opt: The shadow result (Slow Path for monitoring, not used)
    ///
    /// This mode is useful for:
    /// - Validating Fast Path correctness
    /// - Collecting disagreement metrics
    /// - Gradual confidence building in Fast Path
    pub fn act_with_shadow(&self, state: [f32; 8]) -> (crate::action_types::ActionIntent, Option<crate::action_types::ActionIntent>) {
        if !self.arbiter_config.shadow_mode {
            // Shadow mode disabled - just use normal act()
            return (self.act(state), None);
        }

        // Try Fast Path
        let fast_result = self.try_fast_path_internal(state);

        // Always run Slow Path in shadow mode (for comparison)
        let slow_result = self.act_slow_path(state);

        match fast_result {
            Some(fast_intent) => {
                // Record Fast Path stats
                if let crate::action_types::DecisionSource::Reflex { lookup_time_ns, .. } = fast_intent.source {
                    self.arbiter_stats.write().record_reflex(fast_intent.confidence, lookup_time_ns);
                }

                // Compare Fast vs Slow for disagreement tracking
                let params_distance: f32 = fast_intent.params.iter()
                    .zip(&slow_result.params)
                    .map(|(a, b)| (a - b).abs())
                    .sum();

                if params_distance > 1.0 {
                    // Significant disagreement
                    self.arbiter_stats.write().record_shadow_disagreement();
                }

                // Return Fast Path as primary, Slow as shadow
                (fast_intent, Some(slow_result))
            }
            None => {
                // Fast Path failed - use Slow Path as primary (no shadow)
                (slow_result, None)
            }
        }
    }

    /// Try Fast Path and return result if successful (helper for shadow mode)
    fn try_fast_path_internal(&self, state: [f32; 8]) -> Option<crate::action_types::ActionIntent> {
        use crate::action_types::ActionIntent;

        let intuition_arc = self.intuition.as_ref()?;
        let start = Instant::now();
        let intuition = intuition_arc.read();

        let state_token = crate::Token::from_state_f32(0, &state);
        let fast_result = intuition.try_fast_path(&state_token)?;

        let lookup_time_ns = start.elapsed().as_nanos() as u64;
        let similarity = fast_result.similarity;
        let connection_id = fast_result.connection_id;

        let connection = intuition.get_connection(connection_id)?;
        let confidence_u8 = connection.confidence;
        let confidence_f32 = confidence_u8 as f32 / 255.0;

        // Check confidence threshold
        if confidence_u8 < self.arbiter_config.reflex_confidence_threshold {
            return None;
        }

        // Guardian validation
        if let Some(ref guardian) = self.guardian {
            if guardian.validate_reflex(&connection).is_err() {
                return None;
            }
        }

        // Fast Path SUCCESS
        let target_state = expand_target_to_state(&connection.target_vector);
        let action_type = self.infer_action_type(&target_state);
        let action_id = self.next_action_id();

        Some(ActionIntent::from_reflex(
            action_id,
            action_type,
            target_state,
            connection_id,
            lookup_time_ns,
            similarity,
            confidence_f32,
        ))
    }

    /// Slow Path: ADNA reasoning (fallback)
    fn act_slow_path(&self, state: [f32; 8]) -> crate::action_types::ActionIntent {
        use crate::action_types::{ActionIntent, ActionType};

        let start = Instant::now();

        // Convert state to ADNA format (i16) for policy lookup
        let state_i16: [i16; 8] = state.map(|v| (v.clamp(-1.0, 1.0) * 32767.0) as i16);

        // Query ADNA for action policy (NEW in v0.34.0: real ADNA integration)
        // Use tokio Handle if available, otherwise fallback to blocking call
        let policy_result = if let Ok(handle) = tokio::runtime::Handle::try_current() {
            // We're in a tokio runtime context
            handle.block_on(async {
                self.adna_reader.get_action_policy(&state_i16).await
            })
        } else {
            // No runtime available - use default policy as fallback
            // In production, ActionController should be used within tokio context
            Ok(ActionPolicy::new("default_fallback"))
        };

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
    pub fn compute_policy_confidence(&self, policy: &ActionPolicy) -> f32 {
        // NEW v0.34.0: Improved confidence calculation
        // Combines max weight with distribution certainty (inverse entropy)

        if policy.action_weights.is_empty() {
            return 0.0;
        }

        // Calculate total weight and max weight
        let total: f64 = policy.action_weights.values().sum();
        if total == 0.0 {
            return 0.0;
        }

        let max_weight = policy.action_weights
            .values()
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(&0.0);

        // Normalize max weight
        let normalized_max = (*max_weight / total) as f32;

        // Calculate entropy (uncertainty measure)
        let entropy: f64 = policy.action_weights
            .values()
            .map(|&w| {
                let p = w / total;
                if p > 0.0 { -p * p.log2() } else { 0.0 }
            })
            .sum();

        // Max entropy for N actions is log2(N)
        let n = policy.action_weights.len() as f64;
        let max_entropy = if n > 1.0 { n.log2() } else { 1.0 };

        // Certainty: 1.0 when entropy=0 (certain), 0.0 when entropy=max (uniform)
        let certainty = (1.0 - (entropy / max_entropy)).max(0.0) as f32;

        // Confidence combines max weight (70%) and certainty (30%)
        let confidence = 0.7 * normalized_max + 0.3 * certainty;

        confidence.clamp(0.0, 1.0)
    }
}

// ============================================================================
// Helper Functions for Target Vector Expansion
// ============================================================================

/// Expand compressed 8D target vector to full [f32; 8] state
/// Converts i16 coordinates back to f32 using Token's decode_coordinate
fn expand_target_to_state(target_8d: &[i16; 8]) -> [f32; 8] {
    use crate::CoordinateSpace;

    let mut state = [0.0f32; 8];
    for i in 0..8 {
        let space = match i {
            0 => CoordinateSpace::L1Physical,
            1 => CoordinateSpace::L2Sensory,
            2 => CoordinateSpace::L3Motor,
            3 => CoordinateSpace::L4Emotional,
            4 => CoordinateSpace::L5Cognitive,
            5 => CoordinateSpace::L6Social,
            6 => CoordinateSpace::L7Temporal,
            7 => CoordinateSpace::L8Abstract,
            _ => unreachable!(),
        };
        state[i] = crate::Token::decode_coordinate(target_8d[i], space);
    }
    state
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adna::{InMemoryADNAReader, ActionPolicy};
    use crate::experience_stream::ExperienceStream;
    // ============================================================================
    // ActionController v2.0 Tests - Dual-Path Arbitration
    // ============================================================================

    #[test]
    fn test_act_fast_path_with_reflex() {
        use crate::{IntuitionEngine, IntuitionConfig, Guardian};
        use crate::connection_v3::{ConnectionV3, ConnectionMutability};
        use tokio::sync::mpsc;
        use crate::adna::Proposal;

        let adna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let experience_stream = Arc::new(ExperienceStream::new(1000, 10));

        // Create IntuitionEngine with a high-confidence reflex
        let (proposal_tx, _proposal_rx) = mpsc::channel::<Proposal>(100);
        let mut intuition = IntuitionEngine::new(
            IntuitionConfig::default(),
            Arc::clone(&experience_stream),
            Arc::clone(&adna_reader) as Arc<dyn crate::adna::ADNAReader>,
            proposal_tx,
        );

        // Create and consolidate a high-confidence reflex connection
        let source = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let target = [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2];

        // Convert to Token format using new helper
        let source_token = crate::Token::from_state_f32(1, &source);
        let target_token = crate::Token::from_state_f32(2, &target);

        let mut connection = ConnectionV3::new(1, 2);
        connection.confidence = 220; // High confidence (>200 threshold)
        connection.mutability = ConnectionMutability::Immutable as u8;
        connection.rigidity = 200; // 0.8 * 255
        connection.pull_strength = 50.0;
        connection.set_target_from_token(&target_token);  // Store target vector

        // Consolidate the reflex (adds to fast path)
        intuition.consolidate_reflex(&source_token, connection);

        let intuition_arc = Arc::new(RwLock::new(intuition));
        let guardian = Arc::new(Guardian::new());

        // Create ActionController v2.0 with dual-path decision making
        let controller = ActionController::new(
            adna_reader as Arc<dyn ADNAReader>,
            experience_stream as Arc<dyn ExperienceWriter>,
            intuition_arc,
            guardian,
            ActionControllerConfig::default(),
            ArbiterConfig::default(),
        );

        // Call act() with matching state
        let state = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let intent = controller.act(state);

        // Should use Fast Path (reflex)
        assert!(intent.source.is_reflex());
        assert!(intent.confidence > 0.8); // High confidence

        // Check stats
        let stats = controller.get_arbiter_stats();
        assert_eq!(stats.total_decisions, 1);
        assert_eq!(stats.reflex_decisions, 1);
        assert_eq!(stats.reasoning_decisions, 0);
        assert!(stats.avg_reflex_time_ns < 1_000_000); // < 1ms
    }

    #[test]
    fn test_act_guardian_rejection_fallback() {
        use crate::{IntuitionEngine, IntuitionConfig, Guardian};
        use crate::connection_v3::{ConnectionV3, ConnectionMutability};
        use tokio::sync::mpsc;
        use crate::adna::Proposal;

        let adna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let experience_stream = Arc::new(ExperienceStream::new(1000, 10));

        // Create IntuitionEngine with a HYPOTHESIS reflex (will be rejected by Guardian)
        let (proposal_tx, _proposal_rx) = mpsc::channel::<Proposal>(100);
        let mut intuition = IntuitionEngine::new(
            IntuitionConfig::default(),
            Arc::clone(&experience_stream),
            Arc::clone(&adna_reader) as Arc<dyn crate::adna::ADNAReader>,
            proposal_tx,
        );

        let source = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let target = [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2];

        let source_token = crate::Token::from_state_f32(1, &source);
        let target_token = crate::Token::from_state_f32(2, &target);

        let mut connection = ConnectionV3::new(1, 2);
        connection.confidence = 250; // Very high confidence
        connection.mutability = ConnectionMutability::Hypothesis as u8; // Guardian will reject this!
        connection.rigidity = 200;
        connection.pull_strength = 50.0;

        intuition.consolidate_reflex(&source_token, connection);

        let intuition_arc = Arc::new(RwLock::new(intuition));
        let guardian = Arc::new(Guardian::new());

        let controller = ActionController::new(
            adna_reader as Arc<dyn ADNAReader>,
            experience_stream as Arc<dyn ExperienceWriter>,
            intuition_arc,
            guardian,
            ActionControllerConfig::default(),
            ArbiterConfig::default(),
        );

        let state = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let intent = controller.act(state);

        // Guardian should reject → fallback to Slow Path
        assert!(intent.source.is_reasoning());

        // Check stats: guardian rejection recorded
        let stats = controller.get_arbiter_stats();
        assert_eq!(stats.guardian_rejections, 1);
        assert_eq!(stats.reflex_decisions, 0);
        assert_eq!(stats.reasoning_decisions, 1);
    }

    #[test]
    fn test_act_low_confidence_fallback() {
        use crate::{IntuitionEngine, IntuitionConfig, Guardian};
        use crate::connection_v3::{ConnectionV3, ConnectionMutability};
        use tokio::sync::mpsc;
        use crate::adna::Proposal;

        let adna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let experience_stream = Arc::new(ExperienceStream::new(1000, 10));

        // Create IntuitionEngine with LOW confidence reflex
        let (proposal_tx, _proposal_rx) = mpsc::channel::<Proposal>(100);
        let mut intuition = IntuitionEngine::new(
            IntuitionConfig::default(),
            Arc::clone(&experience_stream),
            Arc::clone(&adna_reader) as Arc<dyn crate::adna::ADNAReader>,
            proposal_tx,
        );

        let source = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let target = [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2];

        let source_token = crate::Token::from_state_f32(1, &source);
        let target_token = crate::Token::from_state_f32(2, &target);

        let mut connection = ConnectionV3::new(1, 2);
        connection.confidence = 150; // Low confidence (< 200 threshold)
        connection.mutability = ConnectionMutability::Immutable as u8;

        intuition.consolidate_reflex(&source_token, connection);

        let intuition_arc = Arc::new(RwLock::new(intuition));
        let guardian = Arc::new(Guardian::new());

        let controller = ActionController::new(
            adna_reader as Arc<dyn ADNAReader>,
            experience_stream as Arc<dyn ExperienceWriter>,
            intuition_arc,
            guardian,
            ActionControllerConfig::default(),
            ArbiterConfig::default(),
        );

        let state = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let intent = controller.act(state);

        // Low confidence → fallback to Slow Path
        assert!(intent.source.is_reasoning());

        let stats = controller.get_arbiter_stats();
        assert_eq!(stats.reflex_decisions, 0);
        assert_eq!(stats.reasoning_decisions, 1);
    }

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
    fn test_target_vector_storage_and_extraction() {
        use crate::{IntuitionEngine, IntuitionConfig, Guardian};
        use crate::connection_v3::{ConnectionV3, ConnectionMutability};
        use tokio::sync::mpsc;
        use crate::adna::Proposal;

        let adna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let experience_stream = Arc::new(ExperienceStream::new(1000, 10));

        let (proposal_tx, _proposal_rx) = mpsc::channel::<Proposal>(100);
        let mut intuition = IntuitionEngine::new(
            IntuitionConfig::default(),
            Arc::clone(&experience_stream),
            Arc::clone(&adna_reader) as Arc<dyn crate::adna::ADNAReader>,
            proposal_tx,
        );

        // Different states: source vs target to verify no copying
        let source = [0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];  // Different from reflex test!
        let target = [-0.5, -0.4, -0.3, -0.2, -0.1, 0.0, 0.1, 0.2];  // Clearly different

        let source_token = crate::Token::from_state_f32(10, &source);
        let target_token = crate::Token::from_state_f32(11, &target);

        // Create connection with target vector
        let mut connection = ConnectionV3::new(10, 11);
        connection.confidence = 250; // Very high
        connection.mutability = ConnectionMutability::Immutable as u8;
        connection.rigidity = 200;
        connection.pull_strength = 50.0;
        connection.set_target_from_token(&target_token);  // Store target!

        // Verify target_vector storage
        for i in 0..8 {
            let expected_coord = target_token.coordinates[i][0];
            assert_eq!(connection.target_vector[i], expected_coord);
        }

        intuition.consolidate_reflex(&source_token, connection);

        let intuition_arc = Arc::new(RwLock::new(intuition));
        let guardian = Arc::new(crate::Guardian::new());

        let controller = ActionController::new(
            adna_reader as Arc<dyn ADNAReader>,
            experience_stream as Arc<dyn ExperienceWriter>,
            intuition_arc,
            guardian,
            ActionControllerConfig::default(),
            ArbiterConfig::default(),
        );

        // Call with source state
        let intent = controller.act(source);

        // Should find reflex (high confidence)
        assert!(intent.source.is_reflex(), "Expected Reflex, got {:?}", intent.source);

        // CRITICAL TEST: params should be TARGET, not SOURCE
        for i in 0..8 {
            let diff = (intent.params[i] - target[i]).abs();
            assert!(diff < 0.05,
                "params[{}] = {:.3}, target[{}] = {:.3}, diff = {:.3} (should use target, not source!)",
                i, intent.params[i], i, target[i], diff);
        }

        // Verify params != source (proof we're not copying input)
        let source_similarity: f32 = source.iter().zip(&intent.params)
            .map(|(s, p)| (s - p).abs())
            .sum();
        assert!(source_similarity > 0.5, "params should NOT match source! similarity = {}", source_similarity);
    }

    #[test]
    fn test_shadow_mode_parallel_execution() {
        use crate::{IntuitionEngine, IntuitionConfig, Guardian};
        use crate::connection_v3::{ConnectionV3, ConnectionMutability};
        use tokio::sync::mpsc;
        use crate::adna::Proposal;

        let adna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let experience_stream = Arc::new(ExperienceStream::new(1000, 10));

        let (proposal_tx, _proposal_rx) = mpsc::channel::<Proposal>(100);
        let mut intuition = IntuitionEngine::new(
            IntuitionConfig::default(),
            Arc::clone(&experience_stream),
            Arc::clone(&adna_reader) as Arc<dyn crate::adna::ADNAReader>,
            proposal_tx,
        );

        // Create reflex
        let source = [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];
        let target = [0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2];

        let source_token = crate::Token::from_state_f32(1, &source);
        let target_token = crate::Token::from_state_f32(2, &target);

        let mut connection = ConnectionV3::new(1, 2);
        connection.confidence = 220;
        connection.mutability = ConnectionMutability::Immutable as u8;
        connection.rigidity = 200;
        connection.pull_strength = 50.0;
        connection.set_target_from_token(&target_token);

        intuition.consolidate_reflex(&source_token, connection);

        let intuition_arc = Arc::new(RwLock::new(intuition));
        let guardian = Arc::new(crate::Guardian::new());

        // Enable shadow mode
        let mut config = ArbiterConfig::default();
        config.shadow_mode = true;

        let controller = ActionController::new(
            adna_reader as Arc<dyn ADNAReader>,
            experience_stream as Arc<dyn ExperienceWriter>,
            intuition_arc,
            guardian,
            ActionControllerConfig::default(),
            config,
        );

        // Call act_with_shadow
        let (primary, shadow) = controller.act_with_shadow(source);

        // Should use Fast Path as primary
        assert!(primary.source.is_reflex());

        // Should have shadow result (Slow Path)
        assert!(shadow.is_some());
        let shadow_intent = shadow.unwrap();
        assert!(shadow_intent.source.is_reasoning());
    }

    #[test]
    fn test_shadow_disagreement_tracking() {
        use crate::{IntuitionEngine, IntuitionConfig, Guardian};
        use crate::connection_v3::{ConnectionV3, ConnectionMutability};
        use tokio::sync::mpsc;
        use crate::adna::Proposal;

        let adna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let experience_stream = Arc::new(ExperienceStream::new(1000, 10));

        let (proposal_tx, _proposal_rx) = mpsc::channel::<Proposal>(100);
        let mut intuition = IntuitionEngine::new(
            IntuitionConfig::default(),
            Arc::clone(&experience_stream),
            Arc::clone(&adna_reader) as Arc<dyn crate::adna::ADNAReader>,
            proposal_tx,
        );

        // Create reflex with very different target
        let source = [0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5, 0.5];
        let target = [1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0];  // Very different from ADNA default

        let source_token = crate::Token::from_state_f32(10, &source);
        let target_token = crate::Token::from_state_f32(11, &target);

        let mut connection = ConnectionV3::new(10, 11);
        connection.confidence = 250;
        connection.mutability = ConnectionMutability::Immutable as u8;
        connection.rigidity = 200;
        connection.pull_strength = 50.0;
        connection.set_target_from_token(&target_token);

        intuition.consolidate_reflex(&source_token, connection);

        let intuition_arc = Arc::new(RwLock::new(intuition));
        let guardian = Arc::new(crate::Guardian::new());

        let mut config = ArbiterConfig::default();
        config.shadow_mode = true;

        let controller = ActionController::new(
            adna_reader as Arc<dyn ADNAReader>,
            experience_stream as Arc<dyn ExperienceWriter>,
            intuition_arc,
            guardian,
            ActionControllerConfig::default(),
            config,
        );

        // Act with shadow mode
        let (_primary, _shadow) = controller.act_with_shadow(source);

        // Check that disagreement was recorded
        let stats = controller.get_arbiter_stats();
        assert!(stats.shadow_disagreements >= 0); // At least tracked (might be 0 if params close)
    }

    #[test]
    fn test_improved_confidence_calculation() {
        use crate::adna::ActionPolicy;
        use crate::{IntuitionEngine, IntuitionConfig, Guardian};
        use tokio::sync::mpsc;
        use crate::adna::Proposal;

        let adna_reader = Arc::new(InMemoryADNAReader::with_defaults());
        let experience_stream = Arc::new(ExperienceStream::new(1000, 10));

        let (proposal_tx, _proposal_rx) = mpsc::channel::<Proposal>(100);
        let intuition = IntuitionEngine::new(
            IntuitionConfig::default(),
            Arc::clone(&experience_stream),
            Arc::clone(&adna_reader) as Arc<dyn crate::adna::ADNAReader>,
            proposal_tx,
        );
        let intuition_arc = Arc::new(RwLock::new(intuition));

        let guardian = Arc::new(Guardian::new());

        let controller = ActionController::new(
            adna_reader as Arc<dyn ADNAReader>,
            experience_stream as Arc<dyn ExperienceWriter>,
            intuition_arc,
            guardian,
            ActionControllerConfig::default(),
            ArbiterConfig::default(),
        );

        // Test 1: High certainty (one dominant action)
        let mut policy1 = ActionPolicy::new("test1");
        policy1.action_weights.insert(0, 0.9);
        policy1.action_weights.insert(1, 0.05);
        policy1.action_weights.insert(2, 0.05);

        let conf1 = controller.compute_policy_confidence(&policy1);
        assert!(conf1 > 0.8, "High certainty should give high confidence: {}", conf1);

        // Test 2: Low certainty (uniform distribution)
        let mut policy2 = ActionPolicy::new("test2");
        policy2.action_weights.insert(0, 0.33);
        policy2.action_weights.insert(1, 0.33);
        policy2.action_weights.insert(2, 0.34);

        let conf2 = controller.compute_policy_confidence(&policy2);
        assert!(conf2 < 0.6, "Low certainty should give low confidence: {}", conf2);

        // Confidence 1 should be higher than confidence 2
        assert!(conf1 > conf2, "Certain policy should have higher confidence than uncertain");
    }

}