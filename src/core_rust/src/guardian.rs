/// Guardian V1.1 - System Coordinator for NeuroGraph OS
///
/// Guardian is the central orchestrator that:
/// - Enforces CDNA constitutional rules
/// - Manages ADNA (Adaptive DNA) lifecycle and evolution
/// - Validates Token and Connection operations
/// - Manages event pub/sub system
/// - Tracks CDNA and ADNA version history
/// - Coordinates module interactions
///
/// # Architecture
///
/// Guardian is NOT a dictator - it's a constitutional court:
/// - Does NOT implement module functionality
/// - DOES validate operations against CDNA
/// - DOES coordinate module communication
/// - DOES ensure system consistency
///
/// # Design Principles
///
/// - **Minimal intervention**: Only validate critical operations
/// - **Delegation**: Modules validate their own frequent operations
/// - **Transparency**: All actions are logged and tracked
/// - **Immutability**: CDNA changes are versioned and reversible

use crate::cdna::{CDNA, ProfileId};
use crate::adna::ADNA;
use crate::{Token, Connection};
use std::collections::{HashMap, VecDeque};

/// Event types that can be emitted by Guardian
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    /// CDNA was updated
    CDNAUpdated,
    /// ADNA was loaded
    ADNALoaded,
    /// ADNA parameter was updated
    ADNAUpdated,
    /// ADNA was rolled back to previous version
    ADNARolledBack,
    /// Token was created
    TokenCreated,
    /// Token was deleted
    TokenDeleted,
    /// Connection was created
    ConnectionCreated,
    /// Connection was deleted
    ConnectionDeleted,
    /// Validation failed
    ValidationFailed,
    /// System state changed
    SystemStateChanged,
}

/// Event emitted by Guardian
#[derive(Debug, Clone)]
pub struct Event {
    /// Event type
    pub event_type: EventType,
    /// Optional token ID
    pub token_id: Option<u32>,
    /// Optional connection info (from_id, to_id)
    pub connection_info: Option<(u32, u32)>,
    /// Timestamp (Unix epoch)
    pub timestamp: u64,
    /// Event-specific data
    pub data: String,
}

impl Event {
    pub fn new(event_type: EventType) -> Self {
        Self {
            event_type,
            token_id: None,
            connection_info: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            data: String::new(),
        }
    }

    pub fn with_token(mut self, token_id: u32) -> Self {
        self.token_id = Some(token_id);
        self
    }

    pub fn with_connection(mut self, from_id: u32, to_id: u32) -> Self {
        self.connection_info = Some((from_id, to_id));
        self
    }

    pub fn with_data(mut self, data: String) -> Self {
        self.data = data;
        self
    }
}

/// Module identifier for subscriptions
pub type ModuleId = String;

/// Subscription handle
#[derive(Debug, Clone)]
pub struct Subscription {
    module_id: ModuleId,
    event_types: Vec<EventType>,
    active: bool,
}

impl Subscription {
    pub fn new(module_id: ModuleId, event_types: Vec<EventType>) -> Self {
        Self {
            module_id,
            event_types,
            active: true,
        }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn subscribes_to(&self, event_type: EventType) -> bool {
        self.event_types.contains(&event_type)
    }

    pub fn activate(&mut self) {
        self.active = true;
    }

    pub fn deactivate(&mut self) {
        self.active = false;
    }
}

/// Validation error
#[derive(Debug, Clone)]
pub struct ValidationError {
    pub field: String,
    pub message: String,
    pub value: String,
}

impl ValidationError {
    pub fn new(field: &str, message: &str, value: &str) -> Self {
        Self {
            field: field.to_string(),
            message: message.to_string(),
            value: value.to_string(),
        }
    }
}

/// Guardian configuration
#[derive(Debug, Clone)]
pub struct GuardianConfig {
    /// Enable validation
    pub enable_validation: bool,
    /// Enable event system
    pub enable_events: bool,
    /// Maximum event queue size
    pub max_event_queue: usize,
    /// Maximum CDNA history size
    pub max_history_size: usize,
}

impl Default for GuardianConfig {
    fn default() -> Self {
        Self {
            enable_validation: true,
            enable_events: true,
            max_event_queue: 10000,
            max_history_size: 100,
        }
    }
}

/// Guardian V1.1 - System coordinator and validator with ADNA integration
///
/// # Example
///
/// ```rust
/// use neurograph_core::{Guardian, CDNA, ADNA, ADNAProfile, Token, Connection, ProfileId};
///
/// // Create Guardian with default CDNA
/// let mut guardian = Guardian::new();
///
/// // Load ADNA
/// let adna = ADNA::from_profile(ADNAProfile::Balanced);
/// guardian.load_adna(adna).unwrap();
///
/// // Update ADNA parameter
/// guardian.update_adna_parameter("curiosity_weight", 0.8).unwrap();
///
/// // Validate token
/// let token = Token::new(1);
/// match guardian.validate_token(&token) {
///     Ok(_) => println!("Token valid"),
///     Err(errors) => println!("Validation errors: {:?}", errors),
/// }
///
/// // Subscribe to events
/// guardian.subscribe("my_module".to_string(), vec![EventType::TokenCreated, EventType::ADNAUpdated]);
///
/// // Update CDNA
/// let new_cdna = CDNA::with_profile(ProfileId::Explorer);
/// guardian.update_cdna(new_cdna).unwrap();
/// ```
pub struct Guardian {
    /// Current active CDNA
    cdna: CDNA,
    /// CDNA version history
    cdna_history: VecDeque<CDNA>,
    /// Current active ADNA (optional)
    adna: Option<ADNA>,
    /// ADNA version history
    adna_history: VecDeque<ADNA>,
    /// Configuration
    config: GuardianConfig,
    /// Event subscribers
    subscribers: HashMap<ModuleId, Subscription>,
    /// Event queue
    event_queue: VecDeque<Event>,
    /// Validation statistics
    validation_stats: ValidationStats,
}

/// Validation statistics
#[derive(Debug, Clone, Default)]
struct ValidationStats {
    tokens_validated: u64,
    tokens_rejected: u64,
    connections_validated: u64,
    connections_rejected: u64,
}

impl Guardian {
    /// Create new Guardian with default CDNA
    pub fn new() -> Self {
        Self::with_cdna(CDNA::new())
    }

    /// Create Guardian with specific CDNA
    pub fn with_cdna(cdna: CDNA) -> Self {
        let mut history = VecDeque::new();
        history.push_back(cdna);

        Self {
            cdna,
            cdna_history: history,
            adna: None,
            adna_history: VecDeque::new(),
            config: GuardianConfig::default(),
            subscribers: HashMap::new(),
            event_queue: VecDeque::new(),
            validation_stats: ValidationStats::default(),
        }
    }

    /// Create Guardian with configuration
    pub fn with_config(cdna: CDNA, config: GuardianConfig) -> Self {
        let mut guardian = Self::with_cdna(cdna);
        guardian.config = config;
        guardian
    }

    // ==================== CDNA MANAGEMENT ====================

    /// Get current CDNA (read-only reference)
    pub fn cdna(&self) -> &CDNA {
        &self.cdna
    }

    /// Update CDNA (with validation and versioning)
    pub fn update_cdna(&mut self, new_cdna: CDNA) -> Result<(), String> {
        // Validate new CDNA
        new_cdna.validate()?;

        // Check if quarantine mode
        if !new_cdna.is_active() {
            return Err("Cannot activate CDNA in quarantine mode".to_string());
        }

        // Add current CDNA to history
        self.cdna_history.push_back(self.cdna);

        // Limit history size
        while self.cdna_history.len() > self.config.max_history_size {
            self.cdna_history.pop_front();
        }

        // Update current CDNA
        self.cdna = new_cdna;

        // Emit event
        if self.config.enable_events {
            let event = Event::new(EventType::CDNAUpdated)
                .with_data(format!("Profile: {:?}", self.cdna.profile()));
            self.emit_event(event);
        }

        Ok(())
    }

    /// Get CDNA history
    pub fn cdna_history(&self) -> &VecDeque<CDNA> {
        &self.cdna_history
    }

    /// Rollback to previous CDNA version
    pub fn rollback_cdna(&mut self) -> Result<(), String> {
        if self.cdna_history.len() < 2 {
            return Err("No previous CDNA version to rollback to".to_string());
        }

        // Remove current from history and use previous
        self.cdna_history.pop_back();
        self.cdna = *self.cdna_history.back().unwrap();

        // Emit event
        if self.config.enable_events {
            let event = Event::new(EventType::CDNAUpdated)
                .with_data("Rolled back to previous version".to_string());
            self.emit_event(event);
        }

        Ok(())
    }

    // ==================== VALIDATION ====================

    /// Validate Token against CDNA rules
    pub fn validate_token(&mut self, token: &Token) -> Result<(), Vec<ValidationError>> {
        if !self.config.enable_validation || !self.cdna.validation_enabled() {
            return Ok(());
        }

        let mut errors = Vec::new();

        // Validate weight
        let token_weight = token.weight; // Copy to avoid unaligned reference
        if token_weight < self.cdna.min_token_weight {
            errors.push(ValidationError::new(
                "weight",
                "Token weight below minimum",
                &format!("{} < {}", token_weight, self.cdna.min_token_weight),
            ));
        }

        if token_weight > self.cdna.max_token_weight {
            errors.push(ValidationError::new(
                "weight",
                "Token weight above maximum",
                &format!("{} > {}", token_weight, self.cdna.max_token_weight),
            ));
        }

        // Validate field radius
        let min_radius = self.cdna.min_field_radius as u8;
        let max_radius = self.cdna.max_field_radius as u8;
        if token.field_radius < min_radius {
            errors.push(ValidationError::new(
                "field_radius",
                "Field radius below minimum",
                &format!("{} < {}", token.field_radius, min_radius),
            ));
        }

        if token.field_radius > max_radius {
            errors.push(ValidationError::new(
                "field_radius",
                "Field radius above maximum",
                &format!("{} > {}", token.field_radius, max_radius),
            ));
        }

        // Validate field strength
        let min_strength = self.cdna.min_field_strength as u8;
        let max_strength = self.cdna.max_field_strength as u8;
        if token.field_strength < min_strength {
            errors.push(ValidationError::new(
                "field_strength",
                "Field strength below minimum",
                &format!("{} < {}", token.field_strength, min_strength),
            ));
        }

        if token.field_strength > max_strength {
            errors.push(ValidationError::new(
                "field_strength",
                "Field strength above maximum",
                &format!("{} > {}", token.field_strength, max_strength),
            ));
        }

        // Update stats
        if errors.is_empty() {
            self.validation_stats.tokens_validated += 1;
        } else {
            self.validation_stats.tokens_rejected += 1;

            // Emit validation failed event
            if self.config.enable_events {
                let event = Event::new(EventType::ValidationFailed)
                    .with_token(token.id)
                    .with_data(format!("Token validation failed: {} errors", errors.len()));
                self.emit_event(event);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    /// Validate Connection against CDNA rules
    pub fn validate_connection(&mut self, connection: &Connection) -> Result<(), Vec<ValidationError>> {
        if !self.config.enable_validation || !self.cdna.validation_enabled() {
            return Ok(());
        }

        let mut errors = Vec::new();

        // Validate connection type is allowed
        let conn_type_bit = 1u64 << (connection.connection_type as u64);
        if self.cdna.allowed_connection_types & conn_type_bit == 0 {
            errors.push(ValidationError::new(
                "connection_type",
                "Connection type not allowed by CDNA",
                &format!("Type: {}", connection.connection_type),
            ));
        }

        // Validate pull_strength (weight equivalent)
        if connection.pull_strength < self.cdna.min_connection_weight {
            errors.push(ValidationError::new(
                "pull_strength",
                "Connection pull_strength below minimum",
                &format!("{} < {}", connection.pull_strength, self.cdna.min_connection_weight),
            ));
        }

        if connection.pull_strength > self.cdna.max_connection_weight {
            errors.push(ValidationError::new(
                "pull_strength",
                "Connection pull_strength above maximum",
                &format!("{} > {}", connection.pull_strength, self.cdna.max_connection_weight),
            ));
        }

        // Validate rigidity
        let rigidity = connection.rigidity as f32 / 255.0;
        if rigidity < self.cdna.min_rigidity {
            errors.push(ValidationError::new(
                "rigidity",
                "Rigidity below minimum",
                &format!("{} < {}", rigidity, self.cdna.min_rigidity),
            ));
        }

        if rigidity > self.cdna.max_rigidity {
            errors.push(ValidationError::new(
                "rigidity",
                "Rigidity above maximum",
                &format!("{} > {}", rigidity, self.cdna.max_rigidity),
            ));
        }

        // Update stats
        if errors.is_empty() {
            self.validation_stats.connections_validated += 1;
        } else {
            self.validation_stats.connections_rejected += 1;

            // Emit validation failed event
            if self.config.enable_events {
                let event = Event::new(EventType::ValidationFailed)
                    .with_connection(connection.token_a_id, connection.token_b_id)
                    .with_data(format!("Connection validation failed: {} errors", errors.len()));
                self.emit_event(event);
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    // ==================== ADNA MANAGEMENT ====================

    /// Load ADNA (Adaptive DNA) with validation against CDNA
    ///
    /// # Arguments
    /// * `adna` - ADNA instance to load
    ///
    /// # Returns
    /// * `Ok(())` if ADNA is valid and compatible with CDNA
    /// * `Err(String)` if validation fails
    ///
    /// # Example
    /// ```rust
    /// use neurograph_core::{Guardian, ADNA, ADNAProfile};
    ///
    /// let mut guardian = Guardian::new();
    /// let adna = ADNA::from_profile(ADNAProfile::Balanced);
    ///
    /// guardian.load_adna(adna).unwrap();
    /// ```
    pub fn load_adna(&mut self, adna: ADNA) -> Result<(), String> {
        // Validate ADNA structure
        adna.validate()
            .map_err(|e| format!("ADNA validation failed: {:?}", e))?;

        // Validate against CDNA
        self.validate_adna_against_cdna(&adna)?;

        // Store in history if replacing existing ADNA
        if let Some(current_adna) = self.adna.take() {
            self.adna_history.push_back(current_adna);

            // Limit history size
            while self.adna_history.len() > self.config.max_history_size {
                self.adna_history.pop_front();
            }
        }

        // Load new ADNA
        self.adna = Some(adna);

        // Emit event
        if self.config.enable_events {
            let event = Event::new(EventType::ADNALoaded)
                .with_data(format!("Generation: {}", adna.metrics.generation));
            self.emit_event(event);
        }

        Ok(())
    }

    /// Get current ADNA (read-only reference)
    pub fn adna(&self) -> Option<&ADNA> {
        self.adna.as_ref()
    }

    /// Get ADNA history
    pub fn adna_history(&self) -> &VecDeque<ADNA> {
        &self.adna_history
    }

    /// Update ADNA parameter with versioning
    ///
    /// Creates a new ADNA version (generation++) based on current ADNA
    ///
    /// # Arguments
    /// * `param_name` - Parameter name to update
    /// * `value` - New parameter value
    ///
    /// # Returns
    /// * `Ok(())` if parameter updated successfully
    /// * `Err(String)` if ADNA not loaded or parameter invalid
    pub fn update_adna_parameter(&mut self, param_name: &str, value: f32) -> Result<(), String> {
        let current_adna = self.adna
            .as_ref()
            .ok_or("No ADNA loaded")?;

        // Create evolved version
        let mut new_adna = current_adna.evolve();

        // Update parameter
        match param_name {
            "homeostasis_weight" => new_adna.parameters.homeostasis_weight = value,
            "curiosity_weight" => new_adna.parameters.curiosity_weight = value,
            "efficiency_weight" => new_adna.parameters.efficiency_weight = value,
            "goal_weight" => new_adna.parameters.goal_weight = value,
            "exploration_rate" => new_adna.parameters.exploration_rate = value,
            _ => return Err(format!("Unknown parameter: {}", param_name)),
        }

        // Update hash with new parameter values
        new_adna.update_hash();

        // Validate new ADNA
        new_adna.validate()
            .map_err(|e| format!("ADNA validation failed: {:?}", e))?;

        // Validate against CDNA
        self.validate_adna_against_cdna(&new_adna)?;

        // Save current to history
        let old_adna = self.adna.replace(new_adna);
        if let Some(old) = old_adna {
            self.adna_history.push_back(old);

            // Limit history size
            while self.adna_history.len() > self.config.max_history_size {
                self.adna_history.pop_front();
            }
        }

        // Emit event
        if self.config.enable_events {
            let event = Event::new(EventType::ADNAUpdated)
                .with_data(format!("{} = {}", param_name, value));
            self.emit_event(event);
        }

        Ok(())
    }

    /// Rollback ADNA to previous version
    pub fn rollback_adna(&mut self) -> Result<(), String> {
        if self.adna_history.is_empty() {
            return Err("No previous ADNA version to rollback to".to_string());
        }

        // Pop previous version from history
        let previous_adna = self.adna_history.pop_back().unwrap();

        // Save current to history (if exists)
        if let Some(current) = self.adna.replace(previous_adna) {
            // Don't add back to history - this is a rollback
            drop(current);
        }

        // Emit event
        if self.config.enable_events {
            let event = Event::new(EventType::ADNARolledBack)
                .with_data(format!("Generation: {}", previous_adna.metrics.generation));
            self.emit_event(event);
        }

        Ok(())
    }

    /// Validate ADNA against CDNA constitutional rules
    ///
    /// Ensures ADNA parameters don't violate CDNA constraints
    fn validate_adna_against_cdna(&self, adna: &ADNA) -> Result<(), String> {
        // Check appraiser weights are valid (0.0 - 1.0)
        let weights = &adna.parameters;

        if weights.homeostasis_weight < 0.0 || weights.homeostasis_weight > 1.0 {
            return Err(format!(
                "homeostasis_weight out of range: {}",
                weights.homeostasis_weight
            ));
        }

        if weights.curiosity_weight < 0.0 || weights.curiosity_weight > 1.0 {
            return Err(format!(
                "curiosity_weight out of range: {}",
                weights.curiosity_weight
            ));
        }

        if weights.efficiency_weight < 0.0 || weights.efficiency_weight > 1.0 {
            return Err(format!(
                "efficiency_weight out of range: {}",
                weights.efficiency_weight
            ));
        }

        if weights.goal_weight < 0.0 || weights.goal_weight > 1.0 {
            return Err(format!(
                "goal_weight out of range: {}",
                weights.goal_weight
            ));
        }

        // Check exploration rate
        if weights.exploration_rate < 0.0 || weights.exploration_rate > 1.0 {
            return Err(format!(
                "exploration_rate out of range: {}",
                weights.exploration_rate
            ));
        }

        // Optional: Check decision timeout is reasonable (1ms - 10s)
        if weights.decision_timeout_ms == 0 || weights.decision_timeout_ms > 10000 {
            return Err(format!(
                "decision_timeout_ms out of range: {}ms (expected 1-10000)",
                weights.decision_timeout_ms
            ));
        }

        // Optional: Check max actions per cycle is reasonable (1-1000)
        if weights.max_actions_per_cycle == 0 || weights.max_actions_per_cycle > 1000 {
            return Err(format!(
                "max_actions_per_cycle out of range: {} (expected 1-1000)",
                weights.max_actions_per_cycle
            ));
        }

        // Future: Add more CDNA-specific constraints
        // - Check against CDNA rate limits
        // - Validate stability requirements
        // - etc.

        Ok(())
    }

    // ==================== EVENT SYSTEM ====================

    /// Subscribe module to events
    pub fn subscribe(&mut self, module_id: ModuleId, event_types: Vec<EventType>) -> Result<(), String> {
        if !self.config.enable_events {
            return Err("Event system is disabled".to_string());
        }

        let subscription = Subscription::new(module_id.clone(), event_types);
        self.subscribers.insert(module_id, subscription);
        Ok(())
    }

    /// Unsubscribe module from events
    pub fn unsubscribe(&mut self, module_id: &ModuleId) -> bool {
        self.subscribers.remove(module_id).is_some()
    }

    /// Emit event to subscribers
    pub fn emit_event(&mut self, event: Event) {
        if !self.config.enable_events {
            return;
        }

        // Add to event queue
        self.event_queue.push_back(event.clone());

        // Limit queue size
        while self.event_queue.len() > self.config.max_event_queue {
            self.event_queue.pop_front();
        }

        // In a real implementation, this would notify subscribers
        // For now, events are just queued and can be polled
    }

    /// Poll events for a specific module
    pub fn poll_events(&mut self, module_id: &ModuleId) -> Vec<Event> {
        if let Some(subscription) = self.subscribers.get(module_id) {
            if !subscription.is_active() {
                return Vec::new();
            }

            self.event_queue
                .iter()
                .filter(|event| subscription.subscribes_to(event.event_type))
                .cloned()
                .collect()
        } else {
            Vec::new()
        }
    }

    /// Get all pending events
    pub fn pending_events(&self) -> &VecDeque<Event> {
        &self.event_queue
    }

    /// Clear event queue
    pub fn clear_events(&mut self) {
        self.event_queue.clear();
    }

    // ==================== STATISTICS ====================

    /// Get validation statistics
    pub fn validation_stats(&self) -> (u64, u64, u64, u64) {
        (
            self.validation_stats.tokens_validated,
            self.validation_stats.tokens_rejected,
            self.validation_stats.connections_validated,
            self.validation_stats.connections_rejected,
        )
    }

    /// Reset validation statistics
    pub fn reset_stats(&mut self) {
        self.validation_stats = ValidationStats::default();
    }

    // ==================== UTILITY ====================

    /// Get number of subscribers
    pub fn subscriber_count(&self) -> usize {
        self.subscribers.len()
    }

    /// Get event queue size
    pub fn event_queue_size(&self) -> usize {
        self.event_queue.len()
    }

    /// Check if module is subscribed
    pub fn is_subscribed(&self, module_id: &ModuleId) -> bool {
        self.subscribers.contains_key(module_id)
    }
}

impl Default for Guardian {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CoordinateSpace;

    #[test]
    fn test_guardian_creation() {
        let guardian = Guardian::new();
        assert!(guardian.cdna().validate().is_ok());
        assert_eq!(guardian.cdna_history().len(), 1);
    }

    #[test]
    fn test_cdna_update() {
        let mut guardian = Guardian::new();
        let new_cdna = CDNA::with_profile(ProfileId::Explorer);

        assert!(guardian.update_cdna(new_cdna).is_ok());
        assert_eq!(guardian.cdna().profile(), ProfileId::Explorer);
        assert_eq!(guardian.cdna_history().len(), 2);
    }

    #[test]
    fn test_cdna_rollback() {
        let mut guardian = Guardian::new();
        let explorer = CDNA::with_profile(ProfileId::Explorer);

        guardian.update_cdna(explorer).unwrap();
        assert_eq!(guardian.cdna().profile(), ProfileId::Explorer);

        guardian.rollback_cdna().unwrap();
        assert_eq!(guardian.cdna().profile(), ProfileId::Default);
    }

    #[test]
    fn test_token_validation() {
        let mut guardian = Guardian::new();

        // Valid token
        let mut token = Token::new(1);
        token.weight = 0.5;
        assert!(guardian.validate_token(&token).is_ok());

        // Invalid token (weight too high)
        token.weight = 10.0;
        let result = guardian.validate_token(&token);
        assert!(result.is_err());
        let errors = result.unwrap_err();
        assert!(!errors.is_empty());
    }

    #[test]
    fn test_connection_validation() {
        let mut guardian = Guardian::new();

        // Valid connection
        let mut conn = Connection::new(1, 2);
        conn.set_connection_type(crate::ConnectionType::Near);
        conn.pull_strength = 0.5;
        assert!(guardian.validate_connection(&conn).is_ok());

        // Invalid connection (pull_strength too high)
        conn.pull_strength = 10.0;
        let result = guardian.validate_connection(&conn);
        assert!(result.is_err());
    }

    #[test]
    fn test_event_subscription() {
        let mut guardian = Guardian::new();

        // Subscribe to events
        guardian.subscribe(
            "test_module".to_string(),
            vec![EventType::TokenCreated, EventType::CDNAUpdated],
        ).unwrap();

        assert!(guardian.is_subscribed(&"test_module".to_string()));
        assert_eq!(guardian.subscriber_count(), 1);

        // Unsubscribe
        assert!(guardian.unsubscribe(&"test_module".to_string()));
        assert!(!guardian.is_subscribed(&"test_module".to_string()));
    }

    #[test]
    fn test_event_emission() {
        let mut guardian = Guardian::new();

        guardian.subscribe(
            "test_module".to_string(),
            vec![EventType::TokenCreated],
        ).unwrap();

        // Emit event
        let event = Event::new(EventType::TokenCreated).with_token(42);
        guardian.emit_event(event);

        assert_eq!(guardian.event_queue_size(), 1);

        // Poll events
        let events = guardian.poll_events(&"test_module".to_string());
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].token_id, Some(42));
    }

    #[test]
    fn test_validation_stats() {
        let mut guardian = Guardian::new();

        let token = Token::new(1);
        guardian.validate_token(&token).unwrap();

        let (validated, rejected, _, _) = guardian.validation_stats();
        assert_eq!(validated, 1);
        assert_eq!(rejected, 0);

        // Invalid token
        let mut bad_token = Token::new(2);
        bad_token.weight = 100.0;
        let _ = guardian.validate_token(&bad_token);

        let (validated, rejected, _, _) = guardian.validation_stats();
        assert_eq!(validated, 1);
        assert_eq!(rejected, 1);
    }

    #[test]
    fn test_event_queue_limit() {
        let mut config = GuardianConfig::default();
        config.max_event_queue = 5;

        let mut guardian = Guardian::with_config(CDNA::new(), config);

        // Emit more events than limit
        for i in 0..10 {
            let event = Event::new(EventType::TokenCreated).with_token(i);
            guardian.emit_event(event);
        }

        // Queue should be limited to 5
        assert_eq!(guardian.event_queue_size(), 5);
    }

    // ==================== ADNA INTEGRATION TESTS ====================

    #[test]
    fn test_adna_load() {
        use crate::adna::ADNAProfile;

        let mut guardian = Guardian::new();

        // Initially no ADNA loaded
        assert!(guardian.adna().is_none());
        assert_eq!(guardian.adna_history().len(), 0);

        // Load ADNA
        let adna = ADNA::from_profile(ADNAProfile::Balanced);
        assert!(guardian.load_adna(adna).is_ok());

        // ADNA should be loaded
        assert!(guardian.adna().is_some());
        assert_eq!(guardian.adna_history().len(), 0);

        let loaded_adna = guardian.adna().unwrap();
        assert_eq!(loaded_adna.parameters.homeostasis_weight, 0.25);
        assert_eq!(loaded_adna.parameters.curiosity_weight, 0.25);
    }

    #[test]
    fn test_adna_update_parameter() {
        use crate::adna::ADNAProfile;

        let mut guardian = Guardian::new();
        let adna = ADNA::from_profile(ADNAProfile::Balanced);
        guardian.load_adna(adna).unwrap();

        let generation_before = guardian.adna().unwrap().metrics.generation;

        // Update parameter
        assert!(guardian.update_adna_parameter("curiosity_weight", 0.8).is_ok());

        // Check updated value
        let updated_adna = guardian.adna().unwrap();
        assert_eq!(updated_adna.parameters.curiosity_weight, 0.8);

        // Generation should be incremented
        assert_eq!(updated_adna.metrics.generation, generation_before + 1);

        // History should have old version
        assert_eq!(guardian.adna_history().len(), 1);
        assert_eq!(guardian.adna_history()[0].parameters.curiosity_weight, 0.25);
    }

    #[test]
    fn test_adna_parameter_validation() {
        use crate::adna::ADNAProfile;

        let mut guardian = Guardian::new();
        let adna = ADNA::from_profile(ADNAProfile::Balanced);
        guardian.load_adna(adna).unwrap();

        // Invalid parameter value (> 1.0)
        let result = guardian.update_adna_parameter("curiosity_weight", 1.5);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("out of range"));

        // Invalid parameter name
        let result = guardian.update_adna_parameter("invalid_param", 0.5);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Unknown parameter"));

        // No ADNA loaded
        let mut guardian2 = Guardian::new();
        let result = guardian2.update_adna_parameter("curiosity_weight", 0.5);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No ADNA loaded"));
    }

    #[test]
    fn test_adna_rollback() {
        use crate::adna::ADNAProfile;

        let mut guardian = Guardian::new();
        let adna = ADNA::from_profile(ADNAProfile::Balanced);
        guardian.load_adna(adna).unwrap();

        // Update parameter
        guardian.update_adna_parameter("curiosity_weight", 0.8).unwrap();
        assert_eq!(guardian.adna().unwrap().parameters.curiosity_weight, 0.8);

        // Rollback
        assert!(guardian.rollback_adna().is_ok());
        assert_eq!(guardian.adna().unwrap().parameters.curiosity_weight, 0.25);

        // History should be empty after rollback
        assert_eq!(guardian.adna_history().len(), 0);
    }

    #[test]
    fn test_adna_rollback_no_history() {
        use crate::adna::ADNAProfile;

        let mut guardian = Guardian::new();
        let adna = ADNA::from_profile(ADNAProfile::Balanced);
        guardian.load_adna(adna).unwrap();

        // Try to rollback without history
        let result = guardian.rollback_adna();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No previous ADNA version"));
    }

    #[test]
    fn test_adna_multiple_updates() {
        use crate::adna::ADNAProfile;

        let mut guardian = Guardian::new();
        let adna = ADNA::from_profile(ADNAProfile::Balanced);
        guardian.load_adna(adna).unwrap();

        // Multiple updates
        guardian.update_adna_parameter("curiosity_weight", 0.3).unwrap();
        guardian.update_adna_parameter("exploration_rate", 0.5).unwrap();
        guardian.update_adna_parameter("homeostasis_weight", 0.4).unwrap();

        let current = guardian.adna().unwrap();
        assert_eq!(current.parameters.curiosity_weight, 0.3);
        assert_eq!(current.parameters.exploration_rate, 0.5);
        assert_eq!(current.parameters.homeostasis_weight, 0.4);

        // History should have 3 versions
        assert_eq!(guardian.adna_history().len(), 3);

        // Generation should be 3 (started at 0)
        assert_eq!(current.metrics.generation, 3);
    }

    #[test]
    fn test_adna_cdna_validation() {
        use crate::adna::ADNAProfile;

        let mut guardian = Guardian::new();
        let mut adna = ADNA::from_profile(ADNAProfile::Balanced);

        // Set invalid timeout (> 10000ms)
        adna.parameters.decision_timeout_ms = 15000;
        let result = guardian.load_adna(adna);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("decision_timeout_ms"));

        // Set invalid max_actions (> 1000)
        let mut adna2 = ADNA::from_profile(ADNAProfile::Balanced);
        adna2.parameters.max_actions_per_cycle = 2000;
        let result = guardian.load_adna(adna2);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("max_actions_per_cycle"));
    }

    #[test]
    fn test_adna_events() {
        use crate::adna::ADNAProfile;

        let mut guardian = Guardian::new();

        // Subscribe to ADNA events
        guardian.subscribe(
            "test_module".to_string(),
            vec![EventType::ADNALoaded, EventType::ADNAUpdated, EventType::ADNARolledBack],
        ).unwrap();

        // Load ADNA - should emit ADNALoaded event
        let adna = ADNA::from_profile(ADNAProfile::Balanced);
        guardian.load_adna(adna).unwrap();

        let events = guardian.poll_events(&"test_module".to_string());
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EventType::ADNALoaded);
        guardian.clear_events();

        // Update parameter - should emit ADNAUpdated event
        guardian.update_adna_parameter("curiosity_weight", 0.8).unwrap();

        let events = guardian.poll_events(&"test_module".to_string());
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EventType::ADNAUpdated);
        assert!(events[0].data.contains("curiosity_weight"));
        guardian.clear_events();

        // Rollback - should emit ADNARolledBack event
        guardian.rollback_adna().unwrap();

        let events = guardian.poll_events(&"test_module".to_string());
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EventType::ADNARolledBack);
    }

    #[test]
    fn test_adna_history_limit() {
        use crate::adna::ADNAProfile;

        let mut config = GuardianConfig::default();
        config.max_history_size = 5;

        let mut guardian = Guardian::with_config(CDNA::new(), config);
        let adna = ADNA::from_profile(ADNAProfile::Balanced);
        guardian.load_adna(adna).unwrap();

        // Make 10 updates
        for i in 0..10 {
            let value = 0.1 + (i as f32 * 0.05);
            guardian.update_adna_parameter("curiosity_weight", value).unwrap();
        }

        // History should be limited to 5
        assert_eq!(guardian.adna_history().len(), 5);
        assert_eq!(guardian.adna().unwrap().metrics.generation, 10);
    }
}
