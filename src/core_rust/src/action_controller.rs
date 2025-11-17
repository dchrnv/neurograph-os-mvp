//! ActionController v1.0 - Central action dispatcher
//!
//! ActionController serves as the bridge between abstract Intents and concrete
//! action execution. It queries ADNA for policies, selects appropriate executors,
//! and logs all actions to ExperienceStream for learning.

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

/// Central action dispatcher
///
/// ActionController manages all action executors and coordinates between
/// ADNA policies, executor selection, and experience logging.
pub struct ActionController {
    adna_reader: Arc<dyn ADNAReader>,
    experience_writer: Arc<dyn ExperienceWriter>,
    executors: RwLock<HashMap<String, Arc<dyn ActionExecutor>>>,
    config: ActionControllerConfig,
}

impl ActionController {
    /// Create new ActionController
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
        }
    }

    /// Create with default config
    pub fn with_defaults(
        adna_reader: Arc<dyn ADNAReader>,
        experience_writer: Arc<dyn ExperienceWriter>,
    ) -> Self {
        Self::new(adna_reader, experience_writer, ActionControllerConfig::default())
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
}