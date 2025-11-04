//! Python bindings for Connection V1.0 structure

use crate::connection::{Connection, ConnectionType};
use pyo3::prelude::*;
use pyo3::types::PyBytes;

/// Python wrapper for ConnectionType enum
#[pyclass(name = "ConnectionType")]
#[derive(Clone, Copy)]
pub struct PyConnectionType {
    inner: ConnectionType,
}

#[pymethods]
impl PyConnectionType {
    // Semantic connections (0x00-0x0F)
    #[staticmethod]
    fn Synonym() -> Self {
        PyConnectionType {
            inner: ConnectionType::Synonym,
        }
    }

    #[staticmethod]
    fn Antonym() -> Self {
        PyConnectionType {
            inner: ConnectionType::Antonym,
        }
    }

    #[staticmethod]
    fn Hypernym() -> Self {
        PyConnectionType {
            inner: ConnectionType::Hypernym,
        }
    }

    #[staticmethod]
    fn Hyponym() -> Self {
        PyConnectionType {
            inner: ConnectionType::Hyponym,
        }
    }

    #[staticmethod]
    fn Meronym() -> Self {
        PyConnectionType {
            inner: ConnectionType::Meronym,
        }
    }

    #[staticmethod]
    fn Holonym() -> Self {
        PyConnectionType {
            inner: ConnectionType::Holonym,
        }
    }

    // Causal connections (0x10-0x1F)
    #[staticmethod]
    fn Cause() -> Self {
        PyConnectionType {
            inner: ConnectionType::Cause,
        }
    }

    #[staticmethod]
    fn Effect() -> Self {
        PyConnectionType {
            inner: ConnectionType::Effect,
        }
    }

    #[staticmethod]
    fn EnabledBy() -> Self {
        PyConnectionType {
            inner: ConnectionType::EnabledBy,
        }
    }

    #[staticmethod]
    fn PreventedBy() -> Self {
        PyConnectionType {
            inner: ConnectionType::PreventedBy,
        }
    }

    // Temporal connections (0x20-0x2F)
    #[staticmethod]
    fn Before() -> Self {
        PyConnectionType {
            inner: ConnectionType::Before,
        }
    }

    #[staticmethod]
    fn After() -> Self {
        PyConnectionType {
            inner: ConnectionType::After,
        }
    }

    #[staticmethod]
    fn During() -> Self {
        PyConnectionType {
            inner: ConnectionType::During,
        }
    }

    #[staticmethod]
    fn Simultaneous() -> Self {
        PyConnectionType {
            inner: ConnectionType::Simultaneous,
        }
    }

    // Spatial connections (0x30-0x3F)
    #[staticmethod]
    fn Near() -> Self {
        PyConnectionType {
            inner: ConnectionType::Near,
        }
    }

    #[staticmethod]
    fn Far() -> Self {
        PyConnectionType {
            inner: ConnectionType::Far,
        }
    }

    #[staticmethod]
    fn Inside() -> Self {
        PyConnectionType {
            inner: ConnectionType::Inside,
        }
    }

    #[staticmethod]
    fn Outside() -> Self {
        PyConnectionType {
            inner: ConnectionType::Outside,
        }
    }

    #[staticmethod]
    fn Above() -> Self {
        PyConnectionType {
            inner: ConnectionType::Above,
        }
    }

    #[staticmethod]
    fn Below() -> Self {
        PyConnectionType {
            inner: ConnectionType::Below,
        }
    }

    // Logical connections (0x40-0x4F)
    #[staticmethod]
    fn And() -> Self {
        PyConnectionType {
            inner: ConnectionType::And,
        }
    }

    #[staticmethod]
    fn Or() -> Self {
        PyConnectionType {
            inner: ConnectionType::Or,
        }
    }

    #[staticmethod]
    fn Not() -> Self {
        PyConnectionType {
            inner: ConnectionType::Not,
        }
    }

    #[staticmethod]
    fn Implies() -> Self {
        PyConnectionType {
            inner: ConnectionType::Implies,
        }
    }

    #[staticmethod]
    fn Equivalent() -> Self {
        PyConnectionType {
            inner: ConnectionType::Equivalent,
        }
    }

    // Associative connections (0x50-0x5F)
    #[staticmethod]
    fn Similar() -> Self {
        PyConnectionType {
            inner: ConnectionType::Similar,
        }
    }

    #[staticmethod]
    fn Different() -> Self {
        PyConnectionType {
            inner: ConnectionType::Different,
        }
    }

    #[staticmethod]
    fn Related() -> Self {
        PyConnectionType {
            inner: ConnectionType::Related,
        }
    }

    #[staticmethod]
    fn Cooccurs() -> Self {
        PyConnectionType {
            inner: ConnectionType::Cooccurs,
        }
    }

    // Structural connections (0x60-0x6F)
    #[staticmethod]
    fn PartOf() -> Self {
        PyConnectionType {
            inner: ConnectionType::PartOf,
        }
    }

    #[staticmethod]
    fn Contains() -> Self {
        PyConnectionType {
            inner: ConnectionType::Contains,
        }
    }

    #[staticmethod]
    fn MemberOf() -> Self {
        PyConnectionType {
            inner: ConnectionType::MemberOf,
        }
    }

    #[staticmethod]
    fn SubclassOf() -> Self {
        PyConnectionType {
            inner: ConnectionType::SubclassOf,
        }
    }

    #[staticmethod]
    fn InstanceOf() -> Self {
        PyConnectionType {
            inner: ConnectionType::InstanceOf,
        }
    }

    // Functional connections (0x70-0x7F)
    #[staticmethod]
    fn Uses() -> Self {
        PyConnectionType {
            inner: ConnectionType::Uses,
        }
    }

    #[staticmethod]
    fn UsedBy() -> Self {
        PyConnectionType {
            inner: ConnectionType::UsedBy,
        }
    }

    #[staticmethod]
    fn Requires() -> Self {
        PyConnectionType {
            inner: ConnectionType::Requires,
        }
    }

    #[staticmethod]
    fn Produces() -> Self {
        PyConnectionType {
            inner: ConnectionType::Produces,
        }
    }

    #[staticmethod]
    fn Consumes() -> Self {
        PyConnectionType {
            inner: ConnectionType::Consumes,
        }
    }

    // Emotional connections (0x80-0x8F)
    #[staticmethod]
    fn Likes() -> Self {
        PyConnectionType {
            inner: ConnectionType::Likes,
        }
    }

    #[staticmethod]
    fn Dislikes() -> Self {
        PyConnectionType {
            inner: ConnectionType::Dislikes,
        }
    }

    #[staticmethod]
    fn Fears() -> Self {
        PyConnectionType {
            inner: ConnectionType::Fears,
        }
    }

    #[staticmethod]
    fn Desires() -> Self {
        PyConnectionType {
            inner: ConnectionType::Desires,
        }
    }

    // Rule-based connections (0x90-0x9F)
    #[staticmethod]
    fn Constraint() -> Self {
        PyConnectionType {
            inner: ConnectionType::Constraint,
        }
    }

    #[staticmethod]
    fn Condition() -> Self {
        PyConnectionType {
            inner: ConnectionType::Condition,
        }
    }

    #[staticmethod]
    fn Consequence() -> Self {
        PyConnectionType {
            inner: ConnectionType::Consequence,
        }
    }

    // Metaphorical connections (0xA0-0xAF)
    #[staticmethod]
    fn MetaphorFor() -> Self {
        PyConnectionType {
            inner: ConnectionType::MetaphorFor,
        }
    }

    #[staticmethod]
    fn Symbolizes() -> Self {
        PyConnectionType {
            inner: ConnectionType::Symbolizes,
        }
    }

    // Dynamic connections (0xB0-0xBF)
    #[staticmethod]
    fn Activates() -> Self {
        PyConnectionType {
            inner: ConnectionType::Activates,
        }
    }

    #[staticmethod]
    fn Inhibits() -> Self {
        PyConnectionType {
            inner: ConnectionType::Inhibits,
        }
    }

    #[staticmethod]
    fn Modulates() -> Self {
        PyConnectionType {
            inner: ConnectionType::Modulates,
        }
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }

    fn value(&self) -> u8 {
        self.inner as u8
    }
}

/// Python wrapper for Connection V1.0
#[pyclass(name = "Connection")]
pub struct PyConnection {
    inner: Connection,
}

#[pymethods]
impl PyConnection {
    /// Create a new Connection between two tokens
    #[new]
    fn new(token_a_id: u32, token_b_id: u32, connection_type: PyConnectionType) -> Self {
        PyConnection {
            inner: Connection::new(token_a_id, token_b_id, connection_type.inner),
        }
    }

    /// Get token A ID
    #[getter]
    fn token_a_id(&self) -> u32 {
        self.inner.token_a_id
    }

    /// Get token B ID
    #[getter]
    fn token_b_id(&self) -> u32 {
        self.inner.token_b_id
    }

    /// Get connection type as string
    fn get_connection_type(&self) -> String {
        format!("{:?}", self.inner.get_connection_type())
    }

    /// Get rigidity (0-255 mapped to 0.0-1.0)
    #[getter]
    fn rigidity(&self) -> f32 {
        self.inner.get_rigidity()
    }

    /// Set rigidity (0.0-1.0)
    #[setter]
    fn set_rigidity(&mut self, value: f32) {
        self.inner.set_rigidity(value);
    }

    /// Get pull strength
    #[getter]
    fn pull_strength(&self) -> f32 {
        self.inner.pull_strength
    }

    /// Set pull strength
    #[setter]
    fn set_pull_strength(&mut self, value: f32) {
        self.inner.pull_strength = value;
    }

    /// Get preferred distance
    #[getter]
    fn preferred_distance(&self) -> f32 {
        self.inner.preferred_distance
    }

    /// Set preferred distance
    #[setter]
    fn set_preferred_distance(&mut self, value: f32) {
        self.inner.preferred_distance = value;
    }

    /// Get activation count
    #[getter]
    fn activation_count(&self) -> u32 {
        self.inner.activation_count
    }

    /// Get created_at timestamp
    #[getter]
    fn created_at(&self) -> u32 {
        self.inner.created_at
    }

    /// Get last_activation timestamp
    #[getter]
    fn last_activation(&self) -> u32 {
        self.inner.last_activation
    }

    /// Check if connection is active
    fn is_active(&self) -> bool {
        self.inner.is_active()
    }

    /// Set active state
    fn set_active(&mut self, active: bool) {
        self.inner.set_active(active);
    }

    /// Check if connection is bidirectional
    fn is_bidirectional(&self) -> bool {
        self.inner.is_bidirectional()
    }

    /// Set bidirectional flag
    fn set_bidirectional(&mut self, bidirectional: bool) {
        self.inner.set_bidirectional(bidirectional);
    }

    /// Check if connection is persistent
    fn is_persistent(&self) -> bool {
        self.inner.is_persistent()
    }

    /// Set persistent flag
    fn set_persistent(&mut self, persistent: bool) {
        self.inner.set_persistent(persistent);
    }

    /// Activate connection (increments count, updates timestamp)
    fn activate(&mut self) {
        self.inner.activate();
    }

    /// Check if level is active
    fn is_level_active(&self, level: u8) -> PyResult<bool> {
        if level > 7 {
            return Err(pyo3::exceptions::PyValueError::new_err("Level must be 0-7"));
        }
        Ok(self.inner.is_level_active(level))
    }

    /// Set level active state
    fn set_level_active(&mut self, level: u8, active: bool) -> PyResult<()> {
        if level > 7 {
            return Err(pyo3::exceptions::PyValueError::new_err("Level must be 0-7"));
        }
        self.inner.set_level_active(level, active);
        Ok(())
    }

    /// Get active levels as list of indices
    fn get_active_levels(&self) -> Vec<u8> {
        self.inner.get_active_levels()
    }

    /// Calculate physical force
    fn calculate_force(&self, current_distance: f32) -> f32 {
        self.inner.calculate_force(current_distance)
    }

    /// Serialize to bytes
    fn to_bytes<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        let bytes = self.inner.to_bytes();
        PyBytes::new(py, &bytes)
    }

    /// Deserialize from bytes
    #[staticmethod]
    fn from_bytes(bytes: &[u8]) -> PyResult<Self> {
        if bytes.len() != 32 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Connection requires exactly 32 bytes",
            ));
        }
        let mut arr = [0u8; 32];
        arr.copy_from_slice(bytes);
        Ok(PyConnection {
            inner: Connection::from_bytes(arr),
        })
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "Connection({} -> {}, type={:?}, strength={:.2}, active={})",
            self.inner.token_a_id,
            self.inner.token_b_id,
            self.inner.get_connection_type(),
            self.inner.pull_strength,
            self.inner.is_active()
        )
    }

    /// Get size in bytes (always 32)
    #[staticmethod]
    fn size() -> usize {
        32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_py_connection_creation() {
        let conn = PyConnection::new(1, 2, PyConnectionType::Synonym());
        assert_eq!(conn.token_a_id(), 1);
        assert_eq!(conn.token_b_id(), 2);
    }

    #[test]
    fn test_py_connection_type() {
        let ct = PyConnectionType::Cause();
        assert_eq!(ct.value(), 0x10);
    }
}
