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

/// Guardian V1.0 - System Coordinator for NeuroGraph OS
///
/// Guardian is the central orchestrator that:
/// - Enforces CDNA constitutional rules
/// - Validates Token and Connection operations
/// - Manages event pub/sub system
/// - Tracks CDNA version history
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
use crate::{Token, Connection, ConnectionV3};
use std::collections::{HashMap, VecDeque};

/// Event types that can be emitted by Guardian
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EventType {
    /// CDNA was updated
    CDNAUpdated,
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
    /// Reflex was validated (for fast path)
    ReflexValidated,
    /// Reflex validation failed
    ReflexValidationFailed,
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

/// Guardian V1.0 - System coordinator and validator
///
/// # Example
///
/// ```rust
/// use neurograph_core::{Guardian, CDNA, Token, Connection, ProfileId};
///
/// // Create Guardian with default CDNA
/// let mut guardian = Guardian::new();
///
/// // Validate token
/// let token = Token::new(1);
/// match guardian.validate_token(&token) {
///     Ok(_) => println!("Token valid"),
///     Err(errors) => println!("Validation errors: {:?}", errors),
/// }
///
/// // Subscribe to events
/// guardian.subscribe("my_module".to_string(), vec![EventType::TokenCreated]);
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

    /// Validate reflex (ConnectionV3) for Fast Path usage
    ///
    /// This is a lightweight validation optimized for <100ns execution.
    /// It checks only critical safety properties without full CDNA validation.
    ///
    /// # Fast Path Criteria
    ///
    /// - **Confidence:** Must be ≥ 128 (50% confidence minimum)
    /// - **Mutability:** Immutable or Learnable only (no Hypothesis)
    /// - **Pull Strength:** Must be within safe bounds
    /// - **Rigidity:** Must be ≥ 0.5 (stable connection)
    ///
    /// # Performance
    ///
    /// - Target: <50ns (5-6 simple checks, no allocations)
    /// - No CDNA profile lookups (too slow for Fast Path)
    /// - No event emission (to avoid allocation overhead)
    ///
    /// # Usage
    ///
    /// ```rust
    /// use neurograph_core::{Guardian, ConnectionV3};
    ///
    /// let guardian = Guardian::new();
    /// let connection = ConnectionV3::default();
    ///
    /// if guardian.validate_reflex(&connection).is_ok() {
    ///     // Safe to use in Fast Path
    /// } else {
    ///     // Fall back to Slow Path (ADNA)
    /// }
    /// ```
    pub fn validate_reflex(&self, connection: &ConnectionV3) -> Result<(), &'static str> {
        // Skip if validation disabled (but still check critical safety properties)

        // 1. Confidence check: Must be at least 50% confident
        if connection.confidence < 128 {
            return Err("Reflex confidence too low (<50%)");
        }

        // 2. Mutability check: Only Immutable (0) or Learnable (1) allowed
        // Hypothesis (2) connections are too unstable for reflexes
        if connection.mutability > 1 {
            return Err("Reflex must be Immutable or Learnable (no Hypothesis)");
        }

        // 3. Pull strength check: Must be positive and within reasonable bounds
        if connection.pull_strength <= 0.0 {
            return Err("Reflex pull_strength must be positive");
        }

        if connection.pull_strength > 100.0 {
            return Err("Reflex pull_strength exceeds safe maximum (100.0)");
        }

        // 4. Rigidity check: Must be stable (≥50%)
        let rigidity = connection.rigidity as f32 / 255.0;
        if rigidity < 0.5 {
            return Err("Reflex rigidity too low (<0.5) - connection unstable");
        }

        // 5. Sanity check: token IDs must be different
        if connection.token_a_id == connection.token_b_id {
            return Err("Reflex cannot connect token to itself");
        }

        // All checks passed - reflex is safe for Fast Path
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
        conn.set_connection_type(crate::ConnectionType::Synonym);
        conn.mutability = crate::ConnectionMutability::Learnable as u8;
        conn.learning_rate = 32; // 0.125
        assert!(guardian.validate_connection(&conn).is_ok());

        // Another valid connection
        conn.mutability = crate::ConnectionMutability::Immutable as u8;
        let result = guardian.validate_connection(&conn);
        assert!(result.is_ok());
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

    // ==================== Reflex Validation Tests ====================

    #[test]
    fn test_validate_reflex_valid() {
        let guardian = Guardian::new();

        // Create valid reflex: high confidence, Learnable, stable
        let mut connection = ConnectionV3::new(1, 2);
        connection.confidence = 200;  // ~78% confidence
        connection.mutability = 1;    // Learnable
        connection.pull_strength = 5.0;
        connection.rigidity = 180;    // ~70% rigidity

        assert!(guardian.validate_reflex(&connection).is_ok());
    }

    #[test]
    fn test_validate_reflex_low_confidence() {
        let guardian = Guardian::new();

        // Low confidence (<50%)
        let mut connection = ConnectionV3::new(1, 2);
        connection.confidence = 100;  // ~39% - too low!
        connection.pull_strength = 5.0;
        connection.rigidity = 180;

        let result = guardian.validate_reflex(&connection);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Reflex confidence too low (<50%)");
    }

    #[test]
    fn test_validate_reflex_hypothesis() {
        let guardian = Guardian::new();

        // Hypothesis connections not allowed in reflexes
        let mut connection = ConnectionV3::new(1, 2);
        connection.confidence = 200;
        connection.mutability = 2;  // Hypothesis - not allowed!
        connection.pull_strength = 5.0;
        connection.rigidity = 180;

        let result = guardian.validate_reflex(&connection);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Reflex must be Immutable or Learnable (no Hypothesis)");
    }

    #[test]
    fn test_validate_reflex_low_rigidity() {
        let guardian = Guardian::new();

        // Low rigidity (unstable connection)
        let mut connection = ConnectionV3::new(1, 2);
        connection.confidence = 200;
        connection.pull_strength = 5.0;
        connection.rigidity = 100;  // ~39% - too low!

        let result = guardian.validate_reflex(&connection);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Reflex rigidity too low (<0.5) - connection unstable");
    }

    #[test]
    fn test_validate_reflex_self_loop() {
        let guardian = Guardian::new();

        // Self-loop (token connects to itself)
        let mut connection = ConnectionV3::new(42, 42);
        connection.confidence = 200;
        connection.pull_strength = 5.0;
        connection.rigidity = 180;

        let result = guardian.validate_reflex(&connection);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Reflex cannot connect token to itself");
    }

    #[test]
    fn test_validate_reflex_invalid_pull_strength() {
        let guardian = Guardian::new();

        // Zero pull strength
        let mut connection = ConnectionV3::new(1, 2);
        connection.confidence = 200;
        connection.pull_strength = 0.0;  // Invalid!
        connection.rigidity = 180;

        let result = guardian.validate_reflex(&connection);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Reflex pull_strength must be positive");

        // Excessive pull strength
        connection.pull_strength = 150.0;  // Too high!
        let result = guardian.validate_reflex(&connection);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Reflex pull_strength exceeds safe maximum (100.0)");
    }

    #[test]
    fn test_validate_reflex_immutable() {
        let guardian = Guardian::new();

        // Immutable connections are also valid for reflexes
        let mut connection = ConnectionV3::new(1, 2);
        connection.confidence = 255;  // 100% confidence
        connection.mutability = 0;    // Immutable
        connection.pull_strength = 10.0;
        connection.rigidity = 255;    // 100% rigidity

        assert!(guardian.validate_reflex(&connection).is_ok());
    }
}
