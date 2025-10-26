//! Python bindings for Guardian V1.0 structure

use crate::guardian::{Guardian, GuardianConfig, Event, EventType, ValidationError};
use crate::cdna::CDNA;
use crate::token::Token;
use crate::connection::Connection;
use super::cdna::PyCDNA;
use super::token::PyToken;
use super::connection::PyConnection;
use pyo3::prelude::*;

/// Python wrapper for GuardianConfig
#[pyclass(name = "GuardianConfig")]
#[derive(Clone)]
pub struct PyGuardianConfig {
    pub(crate) inner: GuardianConfig,
}

#[pymethods]
impl PyGuardianConfig {
    #[new]
    #[pyo3(signature = (max_history=10, validate_on_update=true, enable_events=true))]
    fn new(max_history: usize, validate_on_update: bool, enable_events: bool) -> Self {
        PyGuardianConfig {
            inner: GuardianConfig {
                max_history,
                validate_on_update,
                enable_events,
            }
        }
    }

    #[getter]
    fn max_history(&self) -> usize {
        self.inner.max_history
    }

    #[setter]
    fn set_max_history(&mut self, value: usize) {
        self.inner.max_history = value;
    }

    #[getter]
    fn validate_on_update(&self) -> bool {
        self.inner.validate_on_update
    }

    #[setter]
    fn set_validate_on_update(&mut self, value: bool) {
        self.inner.validate_on_update = value;
    }

    #[getter]
    fn enable_events(&self) -> bool {
        self.inner.enable_events
    }

    #[setter]
    fn set_enable_events(&mut self, value: bool) {
        self.inner.enable_events = value;
    }

    fn __repr__(&self) -> String {
        format!(
            "GuardianConfig(max_history={}, validate_on_update={}, enable_events={})",
            self.inner.max_history,
            self.inner.validate_on_update,
            self.inner.enable_events
        )
    }
}

/// Python wrapper for EventType
#[pyclass(name = "EventType")]
#[derive(Clone, Copy)]
pub struct PyEventType {
    inner: EventType,
}

#[pymethods]
impl PyEventType {
    #[staticmethod]
    fn cdna_updated() -> Self {
        PyEventType { inner: EventType::CDNAUpdated }
    }

    #[staticmethod]
    fn token_created() -> Self {
        PyEventType { inner: EventType::TokenCreated }
    }

    #[staticmethod]
    fn token_deleted() -> Self {
        PyEventType { inner: EventType::TokenDeleted }
    }

    #[staticmethod]
    fn connection_created() -> Self {
        PyEventType { inner: EventType::ConnectionCreated }
    }

    #[staticmethod]
    fn connection_deleted() -> Self {
        PyEventType { inner: EventType::ConnectionDeleted }
    }

    #[staticmethod]
    fn validation_failed() -> Self {
        PyEventType { inner: EventType::ValidationFailed }
    }

    #[staticmethod]
    fn system_state_changed() -> Self {
        PyEventType { inner: EventType::SystemStateChanged }
    }

    fn __repr__(&self) -> String {
        match self.inner {
            EventType::CDNAUpdated => "EventType.CDNAUpdated".to_string(),
            EventType::TokenCreated => "EventType.TokenCreated".to_string(),
            EventType::TokenDeleted => "EventType.TokenDeleted".to_string(),
            EventType::ConnectionCreated => "EventType.ConnectionCreated".to_string(),
            EventType::ConnectionDeleted => "EventType.ConnectionDeleted".to_string(),
            EventType::ValidationFailed => "EventType.ValidationFailed".to_string(),
            EventType::SystemStateChanged => "EventType.SystemStateChanged".to_string(),
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        std::mem::discriminant(&self.inner) == std::mem::discriminant(&other.inner)
    }
}

/// Python wrapper for Event
#[pyclass(name = "Event")]
#[derive(Clone)]
pub struct PyEvent {
    inner: Event,
}

#[pymethods]
impl PyEvent {
    #[getter]
    fn event_type(&self) -> PyEventType {
        PyEventType { inner: self.inner.event_type }
    }

    #[getter]
    fn timestamp(&self) -> u64 {
        self.inner.timestamp
    }

    #[getter]
    fn module_id(&self) -> u32 {
        self.inner.module_id
    }

    #[getter]
    fn entity_id(&self) -> u64 {
        self.inner.entity_id
    }

    #[getter]
    fn metadata(&self) -> u64 {
        self.inner.metadata
    }

    fn __repr__(&self) -> String {
        format!(
            "Event(type={:?}, module={}, entity={}, timestamp={})",
            self.inner.event_type,
            self.inner.module_id,
            self.inner.entity_id,
            self.inner.timestamp
        )
    }
}

/// Python wrapper for Guardian V1.0
#[pyclass(name = "Guardian")]
pub struct PyGuardian {
    inner: Guardian,
}

#[pymethods]
impl PyGuardian {
    /// Create a new Guardian with default CDNA and configuration
    #[new]
    #[pyo3(signature = (cdna=None, config=None))]
    fn new(cdna: Option<&PyCDNA>, config: Option<PyGuardianConfig>) -> Self {
        let cdna_inner = cdna.map(|c| c.inner).unwrap_or_else(CDNA::new);
        let config_inner = config.map(|c| c.inner);

        PyGuardian {
            inner: match config_inner {
                Some(cfg) => Guardian::with_config(cdna_inner, cfg),
                None => Guardian::new(cdna_inner),
            }
        }
    }

    // ==================== CDNA MANAGEMENT ====================

    /// Get current CDNA
    fn get_cdna(&self) -> PyCDNA {
        PyCDNA {
            inner: *self.inner.get_cdna(),
        }
    }

    /// Update CDNA
    fn update_cdna(&mut self, cdna: &PyCDNA) -> PyResult<()> {
        self.inner.update_cdna(cdna.inner)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
    }

    /// Rollback CDNA to previous version
    fn rollback_cdna(&mut self) -> PyResult<()> {
        self.inner.rollback_cdna()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err("No CDNA history available"))
    }

    /// Get CDNA history count
    fn cdna_history_count(&self) -> usize {
        self.inner.cdna_history_count()
    }

    /// Clear CDNA history
    fn clear_cdna_history(&mut self) {
        self.inner.clear_cdna_history();
    }

    // ==================== VALIDATION ====================

    /// Validate a token against current CDNA
    fn validate_token(&self, token: &PyToken) -> PyResult<()> {
        self.inner.validate_token(&token.inner)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))
    }

    /// Validate a connection against current CDNA
    fn validate_connection(&self, connection: &PyConnection) -> PyResult<()> {
        self.inner.validate_connection(&connection.inner)
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(format!("{:?}", e)))
    }

    // ==================== EVENT SYSTEM ====================

    /// Subscribe to event type
    fn subscribe(&mut self, module_id: u32, event_type: &PyEventType) {
        self.inner.subscribe(module_id, event_type.inner);
    }

    /// Unsubscribe from event type
    fn unsubscribe(&mut self, module_id: u32, event_type: &PyEventType) {
        self.inner.unsubscribe(module_id, event_type.inner);
    }

    /// Emit an event
    fn emit_event(&mut self, event_type: &PyEventType, module_id: u32, entity_id: u64, metadata: u64) {
        self.inner.emit_event(event_type.inner, module_id, entity_id, metadata);
    }

    /// Poll events for a specific module
    fn poll_events(&mut self, module_id: u32) -> Vec<PyEvent> {
        self.inner.poll_events(module_id)
            .into_iter()
            .map(|e| PyEvent { inner: e })
            .collect()
    }

    /// Clear event queue
    fn clear_events(&mut self) {
        self.inner.clear_events();
    }

    /// Get event queue size
    fn event_count(&self) -> usize {
        self.inner.event_count()
    }

    // ==================== STATISTICS ====================

    /// Get total validation count
    fn total_validations(&self) -> u64 {
        self.inner.total_validations()
    }

    /// Get successful validation count
    fn successful_validations(&self) -> u64 {
        self.inner.successful_validations()
    }

    /// Get failed validation count
    fn failed_validations(&self) -> u64 {
        self.inner.failed_validations()
    }

    /// Get validation success rate (0.0 to 1.0)
    fn validation_success_rate(&self) -> f64 {
        self.inner.validation_success_rate()
    }

    /// Get total events emitted
    fn total_events_emitted(&self) -> u64 {
        self.inner.total_events_emitted()
    }

    /// Get total CDNA updates
    fn total_cdna_updates(&self) -> u64 {
        self.inner.total_cdna_updates()
    }

    /// Reset statistics
    fn reset_stats(&mut self) {
        self.inner.reset_stats();
    }

    // ==================== CONFIGURATION ====================

    /// Get current configuration
    fn get_config(&self) -> PyGuardianConfig {
        PyGuardianConfig {
            inner: self.inner.get_config().clone(),
        }
    }

    /// Update configuration
    fn update_config(&mut self, config: PyGuardianConfig) {
        self.inner.update_config(config.inner);
    }

    // ==================== UTILITY ====================

    fn __repr__(&self) -> String {
        format!(
            "Guardian(validations={}/{}, events={}, cdna_history={})",
            self.inner.successful_validations(),
            self.inner.total_validations(),
            self.inner.total_events_emitted(),
            self.inner.cdna_history_count()
        )
    }
}
