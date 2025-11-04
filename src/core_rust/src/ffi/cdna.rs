//! Python bindings for CDNA V2.1 structure

use crate::cdna::{CDNAFlags, ProfileId, ProfileState, CDNA};
use pyo3::prelude::*;

/// Python wrapper for ProfileId
#[pyclass(name = "ProfileId")]
#[derive(Clone, Copy)]
pub struct PyProfileId {
    inner: ProfileId,
}

#[pymethods]
impl PyProfileId {
    #[new]
    fn new(value: u32) -> PyResult<Self> {
        let inner = match value {
            0 => ProfileId::Default,
            1 => ProfileId::Explorer,
            2 => ProfileId::Analyst,
            3 => ProfileId::Creative,
            v => ProfileId::Custom(v),
        };
        Ok(PyProfileId { inner })
    }

    #[staticmethod]
    fn default() -> Self {
        PyProfileId {
            inner: ProfileId::Default,
        }
    }

    #[staticmethod]
    fn explorer() -> Self {
        PyProfileId {
            inner: ProfileId::Explorer,
        }
    }

    #[staticmethod]
    fn analyst() -> Self {
        PyProfileId {
            inner: ProfileId::Analyst,
        }
    }

    #[staticmethod]
    fn creative() -> Self {
        PyProfileId {
            inner: ProfileId::Creative,
        }
    }

    #[staticmethod]
    fn custom(value: u32) -> Self {
        PyProfileId {
            inner: ProfileId::Custom(value),
        }
    }

    fn to_u32(&self) -> u32 {
        self.inner.to_u32()
    }

    fn __repr__(&self) -> String {
        match self.inner {
            ProfileId::Default => "ProfileId.Default".to_string(),
            ProfileId::Explorer => "ProfileId.Explorer".to_string(),
            ProfileId::Analyst => "ProfileId.Analyst".to_string(),
            ProfileId::Creative => "ProfileId.Creative".to_string(),
            ProfileId::Custom(v) => format!("ProfileId.Custom({})", v),
        }
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.inner.to_u32() == other.inner.to_u32()
    }
}

/// Python wrapper for CDNA V2.1
#[pyclass(name = "CDNA")]
pub struct PyCDNA {
    pub(crate) inner: CDNA,
}

#[pymethods]
impl PyCDNA {
    /// Create a new CDNA with default profile
    #[new]
    fn new() -> Self {
        PyCDNA { inner: CDNA::new() }
    }

    /// Create CDNA with specific profile
    #[staticmethod]
    fn with_profile(profile: &PyProfileId) -> Self {
        PyCDNA {
            inner: CDNA::with_profile(profile.inner),
        }
    }

    // ==================== HEADER BLOCK ====================

    #[getter]
    fn magic(&self) -> u32 {
        self.inner.magic
    }

    #[getter]
    fn version_major(&self) -> u16 {
        self.inner.version_major
    }

    #[getter]
    fn version_minor(&self) -> u16 {
        self.inner.version_minor
    }

    #[getter]
    fn created_at(&self) -> u64 {
        self.inner.created_at
    }

    #[getter]
    fn modified_at(&self) -> u64 {
        self.inner.modified_at
    }

    #[getter]
    fn profile_id(&self) -> u32 {
        self.inner.profile_id
    }

    #[getter]
    fn profile_state(&self) -> u32 {
        self.inner.profile_state
    }

    #[getter]
    fn flags(&self) -> u32 {
        self.inner.flags
    }

    #[getter]
    fn checksum(&self) -> u64 {
        self.inner.checksum
    }

    // ==================== GRID PHYSICS BLOCK ====================

    #[getter]
    fn dimension_ids(&self) -> Vec<u8> {
        self.inner.dimension_ids.to_vec()
    }

    #[setter]
    fn set_dimension_ids(&mut self, value: Vec<u8>) -> PyResult<()> {
        if value.len() != 8 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "dimension_ids must have exactly 8 elements",
            ));
        }
        self.inner.dimension_ids.copy_from_slice(&value);
        self.inner.touch();
        Ok(())
    }

    #[getter]
    fn dimension_flags(&self) -> Vec<u8> {
        self.inner.dimension_flags.to_vec()
    }

    #[setter]
    fn set_dimension_flags(&mut self, value: Vec<u8>) -> PyResult<()> {
        if value.len() != 8 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "dimension_flags must have exactly 8 elements",
            ));
        }
        self.inner.dimension_flags.copy_from_slice(&value);
        self.inner.touch();
        Ok(())
    }

    #[getter]
    fn dimension_scales(&self) -> Vec<f32> {
        self.inner.dimension_scales.to_vec()
    }

    #[setter]
    fn set_dimension_scales(&mut self, value: Vec<f32>) -> PyResult<()> {
        if value.len() != 8 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "dimension_scales must have exactly 8 elements",
            ));
        }
        self.inner.dimension_scales.copy_from_slice(&value);
        self.inner.touch();
        Ok(())
    }

    #[getter]
    fn bucket_sizes(&self) -> Vec<f32> {
        self.inner.bucket_sizes.to_vec()
    }

    #[setter]
    fn set_bucket_sizes(&mut self, value: Vec<f32>) -> PyResult<()> {
        if value.len() != 8 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "bucket_sizes must have exactly 8 elements",
            ));
        }
        self.inner.bucket_sizes.copy_from_slice(&value);
        self.inner.touch();
        Ok(())
    }

    #[getter]
    fn field_strength_limits(&self) -> Vec<f32> {
        self.inner.field_strength_limits.to_vec()
    }

    #[setter]
    fn set_field_strength_limits(&mut self, value: Vec<f32>) -> PyResult<()> {
        if value.len() != 8 {
            return Err(pyo3::exceptions::PyValueError::new_err(
                "field_strength_limits must have exactly 8 elements",
            ));
        }
        self.inner.field_strength_limits.copy_from_slice(&value);
        self.inner.touch();
        Ok(())
    }

    // ==================== GRAPH TOPOLOGY BLOCK ====================

    #[getter]
    fn max_connections_per_token(&self) -> u32 {
        self.inner.max_connections_per_token
    }

    #[setter]
    fn set_max_connections_per_token(&mut self, value: u32) {
        self.inner.max_connections_per_token = value;
        self.inner.touch();
    }

    #[getter]
    fn max_depth(&self) -> u32 {
        self.inner.max_depth
    }

    #[setter]
    fn set_max_depth(&mut self, value: u32) {
        self.inner.max_depth = value;
        self.inner.touch();
    }

    #[getter]
    fn max_fan_out(&self) -> u32 {
        self.inner.max_fan_out
    }

    #[setter]
    fn set_max_fan_out(&mut self, value: u32) {
        self.inner.max_fan_out = value;
        self.inner.touch();
    }

    #[getter]
    fn allow_cycles(&self) -> bool {
        self.inner.allow_cycles != 0
    }

    #[setter]
    fn set_allow_cycles(&mut self, value: bool) {
        self.inner.allow_cycles = if value { 1 } else { 0 };
        self.inner.touch();
    }

    #[getter]
    fn traversal_strategy(&self) -> u32 {
        self.inner.traversal_strategy
    }

    #[setter]
    fn set_traversal_strategy(&mut self, value: u32) {
        self.inner.traversal_strategy = value;
        self.inner.touch();
    }

    // ==================== TOKEN PROPERTIES BLOCK ====================

    #[getter]
    fn min_semantic_distance(&self) -> f32 {
        self.inner.min_semantic_distance
    }

    #[setter]
    fn set_min_semantic_distance(&mut self, value: f32) {
        self.inner.min_semantic_distance = value;
        self.inner.touch();
    }

    #[getter]
    fn max_semantic_distance(&self) -> f32 {
        self.inner.max_semantic_distance
    }

    #[setter]
    fn set_max_semantic_distance(&mut self, value: f32) {
        self.inner.max_semantic_distance = value;
        self.inner.touch();
    }

    #[getter]
    fn allowed_entity_types(&self) -> u32 {
        self.inner.allowed_entity_types
    }

    #[setter]
    fn set_allowed_entity_types(&mut self, value: u32) {
        self.inner.allowed_entity_types = value;
        self.inner.touch();
    }

    #[getter]
    fn allowed_spaces(&self) -> u32 {
        self.inner.allowed_spaces
    }

    #[setter]
    fn set_allowed_spaces(&mut self, value: u32) {
        self.inner.allowed_spaces = value;
        self.inner.touch();
    }

    // ==================== CONNECTION CONSTRAINTS BLOCK ====================

    #[getter]
    fn allowed_connection_types(&self) -> u32 {
        self.inner.allowed_connection_types
    }

    #[setter]
    fn set_allowed_connection_types(&mut self, value: u32) {
        self.inner.allowed_connection_types = value;
        self.inner.touch();
    }

    #[getter]
    fn min_connection_strength(&self) -> f32 {
        self.inner.min_connection_strength
    }

    #[setter]
    fn set_min_connection_strength(&mut self, value: f32) {
        self.inner.min_connection_strength = value;
        self.inner.touch();
    }

    #[getter]
    fn max_connection_strength(&self) -> f32 {
        self.inner.max_connection_strength
    }

    #[setter]
    fn set_max_connection_strength(&mut self, value: f32) {
        self.inner.max_connection_strength = value;
        self.inner.touch();
    }

    #[getter]
    fn decay_rate(&self) -> f32 {
        self.inner.decay_rate
    }

    #[setter]
    fn set_decay_rate(&mut self, value: f32) {
        self.inner.decay_rate = value;
        self.inner.touch();
    }

    #[getter]
    fn required_active_levels(&self) -> u8 {
        self.inner.required_active_levels
    }

    #[setter]
    fn set_required_active_levels(&mut self, value: u8) {
        self.inner.required_active_levels = value;
        self.inner.touch();
    }

    // ==================== EVOLUTION PARAMETERS BLOCK ====================

    #[getter]
    fn mutation_rate(&self) -> f32 {
        self.inner.mutation_rate
    }

    #[setter]
    fn set_mutation_rate(&mut self, value: f32) {
        self.inner.mutation_rate = value;
        self.inner.touch();
    }

    #[getter]
    fn learning_rate(&self) -> f32 {
        self.inner.learning_rate
    }

    #[setter]
    fn set_learning_rate(&mut self, value: f32) {
        self.inner.learning_rate = value;
        self.inner.touch();
    }

    #[getter]
    fn plasticity(&self) -> f32 {
        self.inner.plasticity
    }

    #[setter]
    fn set_plasticity(&mut self, value: f32) {
        self.inner.plasticity = value;
        self.inner.touch();
    }

    #[getter]
    fn fitness_threshold(&self) -> f32 {
        self.inner.fitness_threshold
    }

    #[setter]
    fn set_fitness_threshold(&mut self, value: f32) {
        self.inner.fitness_threshold = value;
        self.inner.touch();
    }

    // ==================== VALIDATION & UTILITY ====================

    /// Validate CDNA structure
    fn validate(&self) -> PyResult<()> {
        self.inner
            .validate()
            .map_err(|e| pyo3::exceptions::PyValueError::new_err(e))
    }

    /// Recompute checksum
    fn compute_checksum(&mut self) {
        self.inner.checksum = self.inner.compute_checksum();
    }

    /// Update modification timestamp
    fn touch(&mut self) {
        self.inner.touch();
    }

    /// Get profile state
    fn get_profile_state(&self) -> u32 {
        self.inner.get_profile_state().to_u32()
    }

    /// Set profile state
    fn set_profile_state(&mut self, state: u32) -> PyResult<()> {
        let profile_state = match state {
            0 => ProfileState::Active,
            1 => ProfileState::Frozen,
            2 => ProfileState::Evolving,
            3 => ProfileState::Deprecated,
            _ => {
                return Err(pyo3::exceptions::PyValueError::new_err(
                    "Invalid profile state",
                ))
            }
        };
        self.inner.set_profile_state(profile_state);
        Ok(())
    }

    /// Check if validation is enabled
    fn is_validation_enabled(&self) -> bool {
        self.inner.is_validation_enabled()
    }

    /// Enable validation
    fn enable_validation(&mut self) {
        self.inner.enable_validation();
    }

    /// Disable validation
    fn disable_validation(&mut self) {
        self.inner.disable_validation();
    }

    /// Check if evolution is enabled
    fn is_evolution_enabled(&self) -> bool {
        self.inner.is_evolution_enabled()
    }

    /// Enable evolution
    fn enable_evolution(&mut self) {
        self.inner.enable_evolution();
    }

    /// Disable evolution
    fn disable_evolution(&mut self) {
        self.inner.disable_evolution();
    }

    fn __repr__(&self) -> String {
        format!(
            "CDNA(profile_id={}, state={}, version={}.{}, validated={})",
            self.inner.profile_id,
            self.inner.profile_state,
            self.inner.version_major,
            self.inner.version_minor,
            self.inner.validate().is_ok()
        )
    }

    fn __len__(&self) -> usize {
        384 // Size in bytes
    }
}
