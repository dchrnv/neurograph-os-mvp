//! Python bindings for Token V2.0 structure

use crate::token::{Token, CoordinateSpace, EntityType};
use pyo3::prelude::*;
use pyo3::types::PyBytes;

/// Python wrapper for CoordinateSpace enum
#[pyclass(name = "CoordinateSpace")]
#[derive(Clone, Copy)]
pub struct PyCoordinateSpace {
    inner: CoordinateSpace,
}

#[pymethods]
impl PyCoordinateSpace {
    #[new]
    fn new(space: u8) -> PyResult<Self> {
        let inner = match space {
            0 => CoordinateSpace::L1Physical,
            1 => CoordinateSpace::L2Sensory,
            2 => CoordinateSpace::L3Motor,
            3 => CoordinateSpace::L4Emotional,
            4 => CoordinateSpace::L5Cognitive,
            5 => CoordinateSpace::L6Social,
            6 => CoordinateSpace::L7Temporal,
            7 => CoordinateSpace::L8Abstract,
            _ => return Err(pyo3::exceptions::PyValueError::new_err("Invalid space index")),
        };
        Ok(PyCoordinateSpace { inner })
    }

    #[staticmethod]
    fn L1Physical() -> Self {
        PyCoordinateSpace { inner: CoordinateSpace::L1Physical }
    }

    #[staticmethod]
    fn L2Sensory() -> Self {
        PyCoordinateSpace { inner: CoordinateSpace::L2Sensory }
    }

    #[staticmethod]
    fn L3Motor() -> Self {
        PyCoordinateSpace { inner: CoordinateSpace::L3Motor }
    }

    #[staticmethod]
    fn L4Emotional() -> Self {
        PyCoordinateSpace { inner: CoordinateSpace::L4Emotional }
    }

    #[staticmethod]
    fn L5Cognitive() -> Self {
        PyCoordinateSpace { inner: CoordinateSpace::L5Cognitive }
    }

    #[staticmethod]
    fn L6Social() -> Self {
        PyCoordinateSpace { inner: CoordinateSpace::L6Social }
    }

    #[staticmethod]
    fn L7Temporal() -> Self {
        PyCoordinateSpace { inner: CoordinateSpace::L7Temporal }
    }

    #[staticmethod]
    fn L8Abstract() -> Self {
        PyCoordinateSpace { inner: CoordinateSpace::L8Abstract }
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

/// Python wrapper for EntityType enum
#[pyclass(name = "EntityType")]
#[derive(Clone, Copy)]
pub struct PyEntityType {
    inner: EntityType,
}

#[pymethods]
impl PyEntityType {
    #[staticmethod]
    fn Undefined() -> Self {
        PyEntityType { inner: EntityType::Undefined }
    }

    #[staticmethod]
    fn Concept() -> Self {
        PyEntityType { inner: EntityType::Concept }
    }

    #[staticmethod]
    fn Object() -> Self {
        PyEntityType { inner: EntityType::Object }
    }

    #[staticmethod]
    fn Event() -> Self {
        PyEntityType { inner: EntityType::Event }
    }

    #[staticmethod]
    fn Agent() -> Self {
        PyEntityType { inner: EntityType::Agent }
    }

    #[staticmethod]
    fn Process() -> Self {
        PyEntityType { inner: EntityType::Process }
    }

    #[staticmethod]
    fn State() -> Self {
        PyEntityType { inner: EntityType::State }
    }

    #[staticmethod]
    fn Relation() -> Self {
        PyEntityType { inner: EntityType::Relation }
    }

    #[staticmethod]
    fn Attribute() -> Self {
        PyEntityType { inner: EntityType::Attribute }
    }

    #[staticmethod]
    fn Action() -> Self {
        PyEntityType { inner: EntityType::Action }
    }

    #[staticmethod]
    fn Perception() -> Self {
        PyEntityType { inner: EntityType::Perception }
    }

    #[staticmethod]
    fn Memory() -> Self {
        PyEntityType { inner: EntityType::Memory }
    }

    #[staticmethod]
    fn Goal() -> Self {
        PyEntityType { inner: EntityType::Goal }
    }

    #[staticmethod]
    fn Rule() -> Self {
        PyEntityType { inner: EntityType::Rule }
    }

    #[staticmethod]
    fn Pattern() -> Self {
        PyEntityType { inner: EntityType::Pattern }
    }

    #[staticmethod]
    fn Cluster() -> Self {
        PyEntityType { inner: EntityType::Cluster }
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.inner)
    }
}

/// Python wrapper for Token V2.0
#[pyclass(name = "Token")]
pub struct PyToken {
    inner: Token,
}

#[pymethods]
impl PyToken {
    /// Create a new Token with given ID
    #[new]
    fn new(id: u32) -> Self {
        PyToken {
            inner: Token::new(id),
        }
    }

    /// Get token ID
    #[getter]
    fn id(&self) -> u32 {
        self.inner.id
    }

    /// Get token weight
    #[getter]
    fn weight(&self) -> f32 {
        self.inner.weight
    }

    /// Set token weight
    #[setter]
    fn set_weight(&mut self, weight: f32) {
        self.inner.weight = weight;
    }

    /// Get field radius
    #[getter]
    fn field_radius(&self) -> u8 {
        self.inner.field_radius
    }

    /// Set field radius
    #[setter]
    fn set_field_radius(&mut self, radius: u8) {
        self.inner.field_radius = radius;
    }

    /// Get field strength
    #[getter]
    fn field_strength(&self) -> u8 {
        self.inner.field_strength
    }

    /// Set field strength
    #[setter]
    fn set_field_strength(&mut self, strength: u8) {
        self.inner.field_strength = strength;
    }

    /// Get timestamp
    #[getter]
    fn timestamp(&self) -> u32 {
        self.inner.timestamp
    }

    /// Set timestamp
    #[setter]
    fn set_timestamp(&mut self, timestamp: u32) {
        self.inner.timestamp = timestamp;
    }

    /// Set coordinates for a specific space
    fn set_coordinates(&mut self, space: PyCoordinateSpace, x: f32, y: f32, z: f32) {
        self.inner.set_coordinates(space.inner, x, y, z);
    }

    /// Get coordinates for a specific space
    fn get_coordinates(&self, space: PyCoordinateSpace) -> (f32, f32, f32) {
        self.inner.get_coordinates(space.inner)
    }

    /// Set entity type
    fn set_entity_type(&mut self, entity_type: PyEntityType) {
        self.inner.set_entity_type(entity_type.inner);
    }

    /// Get entity type as string
    fn get_entity_type(&self) -> String {
        format!("{:?}", self.inner.get_entity_type())
    }

    /// Set active flag
    fn set_active(&mut self, active: bool) {
        self.inner.set_active(active);
    }

    /// Check if token is active
    fn is_active(&self) -> bool {
        self.inner.is_active()
    }

    /// Set persistent flag
    fn set_persistent(&mut self, persistent: bool) {
        self.inner.set_persistent(persistent);
    }

    /// Check if token is persistent
    fn is_persistent(&self) -> bool {
        self.inner.is_persistent()
    }

    /// Set mutable flag
    fn set_mutable(&mut self, mutable: bool) {
        self.inner.set_mutable(mutable);
    }

    /// Check if token is mutable
    fn is_mutable(&self) -> bool {
        self.inner.is_mutable()
    }

    /// Set system flag
    fn set_system(&mut self, system: bool) {
        self.inner.set_system(system);
    }

    /// Check if token is system
    fn is_system(&self) -> bool {
        self.inner.is_system()
    }

    /// Serialize to bytes
    fn to_bytes<'py>(&self, py: Python<'py>) -> &'py PyBytes {
        let bytes = self.inner.to_bytes();
        PyBytes::new(py, &bytes)
    }

    /// Deserialize from bytes
    #[staticmethod]
    fn from_bytes(bytes: &[u8]) -> PyResult<Self> {
        if bytes.len() != 64 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "Token requires exactly 64 bytes"
            ));
        }
        let mut arr = [0u8; 64];
        arr.copy_from_slice(bytes);
        Ok(PyToken {
            inner: Token::from_bytes(arr),
        })
    }

    /// String representation
    fn __repr__(&self) -> String {
        format!(
            "Token(id={}, weight={:.2}, active={}, entity_type={:?})",
            self.inner.id,
            self.inner.weight,
            self.inner.is_active(),
            self.inner.get_entity_type()
        )
    }

    /// Get size in bytes (always 64)
    #[staticmethod]
    fn size() -> usize {
        64
    }

    /// Calculate distance to another token in a specific space
    fn distance_to(&self, other: &PyToken, space: PyCoordinateSpace) -> f32 {
        self.inner.distance_to(&other.inner, space.inner)
    }

    /// Get all coordinates as a dictionary
    fn all_coordinates(&self) -> Vec<(String, (f32, f32, f32))> {
        vec![
            ("L1Physical".to_string(), self.inner.get_coordinates(CoordinateSpace::L1Physical)),
            ("L2Sensory".to_string(), self.inner.get_coordinates(CoordinateSpace::L2Sensory)),
            ("L3Motor".to_string(), self.inner.get_coordinates(CoordinateSpace::L3Motor)),
            ("L4Emotional".to_string(), self.inner.get_coordinates(CoordinateSpace::L4Emotional)),
            ("L5Cognitive".to_string(), self.inner.get_coordinates(CoordinateSpace::L5Cognitive)),
            ("L6Social".to_string(), self.inner.get_coordinates(CoordinateSpace::L6Social)),
            ("L7Temporal".to_string(), self.inner.get_coordinates(CoordinateSpace::L7Temporal)),
            ("L8Abstract".to_string(), self.inner.get_coordinates(CoordinateSpace::L8Abstract)),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_py_token_creation() {
        let token = PyToken::new(42);
        assert_eq!(token.id(), 42);
        assert_eq!(token.weight(), 1.0);
    }

    #[test]
    fn test_py_coordinate_space() {
        let space = PyCoordinateSpace::L1Physical();
        assert_eq!(format!("{:?}", space.inner), "L1Physical");
    }
}
