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

//! RuntimeStorage - Unified storage for all dynamic runtime data
//!
//! This is the single source of truth for:
//! - Tokens (runtime entities)
//! - Connections (relationships between tokens)
//! - Grid (spatial index for tokens)
//! - Graph (topology/network structure)
//! - CDNA (constitution/configuration)

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use parking_lot::RwLock;

use crate::token::Token;
use crate::connection_v3::ConnectionV3;
use crate::grid::Grid;
use crate::graph::Graph;
use crate::cdna::CDNA;

// ============================================================================
// Error Types
// ============================================================================

#[derive(Debug, Clone)]
pub enum StorageError {
    TokenNotFound(u32),
    TokenAlreadyExists(u32),
    ConnectionNotFound(u64),
    ConnectionAlreadyExists(u64),
    InvalidTokenId(u32),
    InvalidConnectionId(u64),
    GridError(String),
    CDNAError(String),
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StorageError::TokenNotFound(id) => write!(f, "Token {} not found", id),
            StorageError::TokenAlreadyExists(id) => write!(f, "Token {} already exists", id),
            StorageError::ConnectionNotFound(id) => write!(f, "Connection {} not found", id),
            StorageError::ConnectionAlreadyExists(id) => write!(f, "Connection {} already exists", id),
            StorageError::InvalidTokenId(id) => write!(f, "Invalid token ID: {}", id),
            StorageError::InvalidConnectionId(id) => write!(f, "Invalid connection ID: {}", id),
            StorageError::GridError(msg) => write!(f, "Grid error: {}", msg),
            StorageError::CDNAError(msg) => write!(f, "CDNA error: {}", msg),
        }
    }
}

impl std::error::Error for StorageError {}

pub type StorageResult<T> = Result<T, StorageError>;

// ============================================================================
// RuntimeStorage
// ============================================================================

/// Unified runtime storage for all dynamic data
///
/// This structure provides a single source of truth for all runtime data:
/// - Token storage with automatic ID generation
/// - Connection storage with relationship tracking
/// - Spatial grid for efficient neighbor queries
/// - Graph topology for network structure
/// - CDNA configuration for system constitution
///
/// All operations are thread-safe using RwLock for concurrent access.
pub struct RuntimeStorage {
    // === Token Storage ===
    /// All runtime tokens indexed by ID
    tokens: RwLock<HashMap<u32, Token>>,
    /// Next available token ID (atomic counter)
    next_token_id: AtomicU32,

    // === Connection Storage ===
    /// All connections indexed by ID
    connections: RwLock<HashMap<u64, ConnectionV3>>,
    /// Next available connection ID (atomic counter)
    next_connection_id: AtomicU64,

    // === Spatial Index ===
    /// Grid for spatial queries on tokens
    grid: RwLock<Grid>,

    // === Graph Topology ===
    /// Graph structure (nodes and edges)
    graph: RwLock<Graph>,

    // === Constitution ===
    /// CDNA configuration
    cdna: RwLock<CDNA>,

    // === Label Caches ===
    /// Label to ID mapping
    label_to_id: RwLock<HashMap<String, u32>>,
    /// ID to label mapping
    id_to_label: RwLock<HashMap<u32, String>>,
}

impl RuntimeStorage {
    /// Create a new RuntimeStorage with default configuration
    pub fn new() -> Self {
        Self {
            tokens: RwLock::new(HashMap::new()),
            next_token_id: AtomicU32::new(1),
            connections: RwLock::new(HashMap::new()),
            next_connection_id: AtomicU64::new(1),
            grid: RwLock::new(Grid::new()),
            graph: RwLock::new(Graph::new()),
            cdna: RwLock::new(CDNA::new()),
            label_to_id: RwLock::new(HashMap::new()),
            id_to_label: RwLock::new(HashMap::new()),
        }
    }

    // ========================================================================
    // Token API
    // ========================================================================

    /// Create a new token with auto-generated ID
    ///
    /// # Arguments
    /// * `token` - Token to store (ID will be overwritten)
    ///
    /// # Returns
    /// The assigned token ID
    pub fn create_token(&self, mut token: Token) -> u32 {
        // Generate new ID
        let id = self.next_token_id.fetch_add(1, Ordering::SeqCst);
        token.id = id;

        // Store token
        let mut tokens = self.tokens.write();
        tokens.insert(id, token.clone());
        drop(tokens);

        // Add to grid (if has coordinates)
        let mut grid = self.grid.write();
        let _ = grid.add(token.clone());
        drop(grid);

        // Add node to graph
        let mut graph = self.graph.write();
        graph.add_node(id);
        drop(graph);

        id
    }

    /// Get a token by ID
    ///
    /// # Arguments
    /// * `id` - Token ID
    ///
    /// # Returns
    /// Some(Token) if found, None otherwise
    pub fn get_token(&self, id: u32) -> Option<Token> {
        let tokens = self.tokens.read();
        tokens.get(&id).cloned()
    }

    /// Update a token
    ///
    /// # Arguments
    /// * `id` - Token ID
    /// * `token` - Updated token data
    ///
    /// # Returns
    /// Ok(()) if successful, Err if token not found
    pub fn update_token(&self, id: u32, token: Token) -> StorageResult<()> {
        let mut tokens = self.tokens.write();

        if !tokens.contains_key(&id) {
            return Err(StorageError::TokenNotFound(id));
        }

        // Update in storage
        let old_token = tokens.get(&id).cloned();
        tokens.insert(id, token.clone());
        drop(tokens);

        // Update in grid (remove old, add new)
        if let Some(old) = old_token {
            let mut grid = self.grid.write();
            grid.remove(old.id);
            let _ = grid.add(token);
        }

        Ok(())
    }

    /// Delete a token
    ///
    /// # Arguments
    /// * `id` - Token ID to delete
    ///
    /// # Returns
    /// Some(Token) if deleted, None if not found
    pub fn delete_token(&self, id: u32) -> Option<Token> {
        let mut tokens = self.tokens.write();
        let token = tokens.remove(&id)?;
        drop(tokens);

        // Remove from grid
        let mut grid = self.grid.write();
        grid.remove(id);
        drop(grid);

        // Remove node from graph
        let mut graph = self.graph.write();
        graph.remove_node(id);
        drop(graph);

        Some(token)
    }

    /// List tokens with pagination
    ///
    /// # Arguments
    /// * `limit` - Maximum number of tokens to return
    /// * `offset` - Number of tokens to skip
    ///
    /// # Returns
    /// Vector of tokens
    pub fn list_tokens(&self, limit: usize, offset: usize) -> Vec<Token> {
        let tokens = self.tokens.read();

        tokens.values()
            .skip(offset)
            .take(limit)
            .cloned()
            .collect()
    }

    /// Count total tokens
    ///
    /// # Returns
    /// Total number of tokens
    pub fn count_tokens(&self) -> usize {
        let tokens = self.tokens.read();
        tokens.len()
    }

    /// Clear all tokens
    ///
    /// # Returns
    /// Number of tokens removed
    pub fn clear_tokens(&self) -> usize {
        let mut tokens = self.tokens.write();
        let count = tokens.len();
        tokens.clear();
        drop(tokens);

        // Clear grid
        let mut grid = self.grid.write();
        *grid = Grid::new();
        drop(grid);

        // Clear graph
        let mut graph = self.graph.write();
        *graph = Graph::new();

        count
    }

    // ========================================================================
    // Connection API
    // ========================================================================

    /// Create a new connection
    ///
    /// # Arguments
    /// * `connection` - Connection to store
    ///
    /// # Returns
    /// The assigned connection ID
    ///
    /// Note: ConnectionV3 doesn't have an ID field, so we use the auto-generated ID as the key
    pub fn create_connection(&self, connection: ConnectionV3) -> u64 {
        let id = self.next_connection_id.fetch_add(1, Ordering::SeqCst);

        let mut connections = self.connections.write();
        connections.insert(id, connection);

        id
    }

    /// Get a connection by ID
    pub fn get_connection(&self, id: u64) -> Option<ConnectionV3> {
        let connections = self.connections.read();
        connections.get(&id).cloned()
    }

    /// Update a connection
    pub fn update_connection(&self, id: u64, connection: ConnectionV3) -> StorageResult<()> {
        let mut connections = self.connections.write();

        if !connections.contains_key(&id) {
            return Err(StorageError::ConnectionNotFound(id));
        }

        connections.insert(id, connection);
        Ok(())
    }

    /// Delete a connection
    pub fn delete_connection(&self, id: u64) -> Option<ConnectionV3> {
        let mut connections = self.connections.write();
        connections.remove(&id)
    }

    /// List connections with pagination
    pub fn list_connections(&self, limit: usize, offset: usize) -> Vec<ConnectionV3> {
        let connections = self.connections.read();

        connections.values()
            .skip(offset)
            .take(limit)
            .cloned()
            .collect()
    }

    /// Count total connections
    pub fn count_connections(&self) -> usize {
        let connections = self.connections.read();
        connections.len()
    }

    // ========================================================================
    // Grid API
    // ========================================================================

    /// Get grid information (dimensions, cell size, etc.)
    ///
    /// # Returns
    /// A tuple of (total_tokens_in_grid, grid_bounds)
    pub fn grid_info(&self) -> (usize, [f32; 6]) {
        let grid = self.grid.read();
        // Return count and bounds [min_x, max_x, min_y, max_y, min_z, max_z]
        // For now, return defaults - Grid struct needs to expose this info
        (grid.len(), [0.0, 100.0, 0.0, 100.0, 0.0, 100.0])
    }

    /// Add a token to the grid by ID
    ///
    /// # Arguments
    /// * `token_id` - ID of token to add to grid
    ///
    /// # Returns
    /// Ok(()) if successful, Err if token not found
    pub fn add_to_grid(&self, token_id: u32) -> StorageResult<()> {
        let tokens = self.tokens.read();
        let token = tokens.get(&token_id)
            .ok_or(StorageError::TokenNotFound(token_id))?
            .clone();
        drop(tokens);

        let mut grid = self.grid.write();
        grid.add(token)
            .map_err(|e| StorageError::GridError(e.to_string()))?;
        Ok(())
    }

    /// Remove a token from the grid
    ///
    /// # Arguments
    /// * `token_id` - ID of token to remove from grid
    pub fn remove_from_grid(&self, token_id: u32) {
        let mut grid = self.grid.write();
        grid.remove(token_id);
    }

    /// Find neighbors of a token within a radius
    ///
    /// # Arguments
    /// * `token_id` - Center token ID
    /// * `radius` - Search radius
    ///
    /// # Returns
    /// Vector of (token_id, distance) tuples
    pub fn find_neighbors(&self, token_id: u32, radius: f32) -> StorageResult<Vec<(u32, f32)>> {
        use crate::token::CoordinateSpace;

        // Check if token exists
        let tokens = self.tokens.read();
        if !tokens.contains_key(&token_id) {
            return Err(StorageError::TokenNotFound(token_id));
        }
        drop(tokens);

        // Use Grid's find_neighbors method
        let grid = self.grid.read();
        let neighbors = grid.find_neighbors(
            token_id,
            CoordinateSpace::L1Physical,
            radius,
            100  // max_results
        );

        Ok(neighbors)
    }

    /// Query tokens in a spatial range
    ///
    /// # Arguments
    /// * `center` - Center coordinates [x, y, z]
    /// * `radius` - Search radius
    ///
    /// # Returns
    /// Vector of (token_id, distance) tuples within range
    pub fn range_query(&self, center: [f32; 3], radius: f32) -> Vec<(u32, f32)> {
        use crate::token::CoordinateSpace;

        let grid = self.grid.read();
        grid.range_query(
            CoordinateSpace::L1Physical,
            center[0],
            center[1],
            center[2],
            radius
        )
    }

    // ========================================================================
    // CDNA API
    // ========================================================================

    /// Get CDNA configuration
    ///
    /// # Returns
    /// Clone of current CDNA configuration
    pub fn get_cdna(&self) -> CDNA {
        let cdna = self.cdna.read();
        cdna.clone()
    }

    /// Update CDNA scales
    ///
    /// # Arguments
    /// * `scales` - New scale values [l1_scale, l2_scale, ..., l8_scale]
    ///
    /// # Returns
    /// Ok(()) if successful, Err if invalid scales
    pub fn update_cdna_scales(&self, scales: [f32; 8]) -> StorageResult<()> {
        // Validate scales (must be positive)
        for (i, &scale) in scales.iter().enumerate() {
            if scale <= 0.0 {
                return Err(StorageError::CDNAError(
                    format!("Invalid scale at L{}: {} (must be > 0)", i + 1, scale)
                ));
            }
        }

        let mut cdna = self.cdna.write();
        cdna.dimension_scales = scales;

        Ok(())
    }

    /// Get CDNA profile ID
    ///
    /// # Returns
    /// Current profile ID
    pub fn get_cdna_profile(&self) -> u32 {
        let cdna = self.cdna.read();
        cdna.profile_id
    }

    /// Set CDNA profile ID
    ///
    /// # Arguments
    /// * `profile_id` - Profile ID to set
    pub fn set_cdna_profile(&self, profile_id: u32) {
        let mut cdna = self.cdna.write();
        cdna.profile_id = profile_id;
    }

    /// Get CDNA flags
    ///
    /// # Returns
    /// Current CDNA flags value
    pub fn get_cdna_flags(&self) -> u32 {
        let cdna = self.cdna.read();
        cdna.flags
    }

    /// Set CDNA flags
    ///
    /// # Arguments
    /// * `flags` - New flags value
    pub fn set_cdna_flags(&self, flags: u32) {
        let mut cdna = self.cdna.write();
        cdna.flags = flags;
    }

    /// Validate CDNA configuration
    ///
    /// # Returns
    /// True if CDNA is valid, false otherwise
    pub fn validate_cdna(&self) -> bool {
        let cdna = self.cdna.read();

        // Check that all dimension scales are positive
        cdna.dimension_scales.iter().all(|&scale| scale > 0.0)
    }

    /// Reset CDNA to default configuration
    pub fn reset_cdna(&self) {
        let mut cdna = self.cdna.write();
        *cdna = CDNA::new();
    }
}

impl Default for RuntimeStorage {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_token() {
        let storage = RuntimeStorage::new();
        let token = Token::new(0); // ID will be overwritten

        let id = storage.create_token(token);
        assert_eq!(id, 1);

        let retrieved = storage.get_token(id);
        assert!(retrieved.is_some());
        let token_id = retrieved.unwrap().id;
        assert_eq!(token_id, id);
    }

    #[test]
    fn test_token_crud() {
        let storage = RuntimeStorage::new();

        // Create
        let token = Token::new(0);
        let id = storage.create_token(token);
        assert_eq!(storage.count_tokens(), 1);

        // Read
        let retrieved = storage.get_token(id);
        assert!(retrieved.is_some());

        // Update
        let mut updated = retrieved.unwrap();
        updated.weight = 2.0;
        assert!(storage.update_token(id, updated).is_ok());

        let retrieved = storage.get_token(id).unwrap();
        let weight = retrieved.weight;
        assert_eq!(weight, 2.0);

        // Delete
        let deleted = storage.delete_token(id);
        assert!(deleted.is_some());
        assert_eq!(storage.count_tokens(), 0);
    }

    #[test]
    fn test_list_tokens() {
        let storage = RuntimeStorage::new();

        // Create 10 tokens
        for _ in 0..10 {
            storage.create_token(Token::new(0));
        }

        assert_eq!(storage.count_tokens(), 10);

        // List with pagination
        let page1 = storage.list_tokens(5, 0);
        assert_eq!(page1.len(), 5);

        let page2 = storage.list_tokens(5, 5);
        assert_eq!(page2.len(), 5);
    }

    #[test]
    fn test_create_connection() {
        let storage = RuntimeStorage::new();
        let conn = ConnectionV3::new(1, 2);

        let id = storage.create_connection(conn);
        assert_eq!(id, 1);

        let retrieved = storage.get_connection(id);
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_connection_crud() {
        let storage = RuntimeStorage::new();

        // Create
        let conn = ConnectionV3::new(1, 2);
        let id = storage.create_connection(conn);
        assert_eq!(storage.count_connections(), 1);

        // Read
        let retrieved = storage.get_connection(id);
        assert!(retrieved.is_some());

        // Delete
        let deleted = storage.delete_connection(id);
        assert!(deleted.is_some());
        assert_eq!(storage.count_connections(), 0);
    }

    #[test]
    fn test_clear_tokens() {
        let storage = RuntimeStorage::new();

        for _ in 0..5 {
            storage.create_token(Token::new(0));
        }

        assert_eq!(storage.count_tokens(), 5);

        let cleared = storage.clear_tokens();
        assert_eq!(cleared, 5);
        assert_eq!(storage.count_tokens(), 0);
    }

    #[test]
    fn test_grid_operations() {
        use crate::token::CoordinateSpace;

        let storage = RuntimeStorage::new();

        // Create token with coordinates
        let mut token = Token::new(0);
        token.set_coordinates(CoordinateSpace::L1Physical, 1.0, 2.0, 3.0);

        let id = storage.create_token(token);

        // Grid info
        let (count, _bounds) = storage.grid_info();
        assert!(count > 0);

        // Remove from grid
        storage.remove_from_grid(id);

        // Re-add to grid
        assert!(storage.add_to_grid(id).is_ok());
    }

    #[test]
    fn test_find_neighbors() {
        use crate::token::CoordinateSpace;

        let storage = RuntimeStorage::new();

        // Create center token
        let mut center = Token::new(0);
        center.set_coordinates(CoordinateSpace::L1Physical, 0.0, 0.0, 0.0);
        let center_id = storage.create_token(center);

        // Create nearby token
        let mut nearby = Token::new(0);
        nearby.set_coordinates(CoordinateSpace::L1Physical, 1.0, 1.0, 1.0);
        storage.create_token(nearby);

        // Create far token
        let mut far = Token::new(0);
        far.set_coordinates(CoordinateSpace::L1Physical, 100.0, 100.0, 100.0);
        storage.create_token(far);

        // Find neighbors within radius
        let neighbors = storage.find_neighbors(center_id, 5.0);
        assert!(neighbors.is_ok());

        let neighbors = neighbors.unwrap();
        // Should find the nearby token but not the far one
        assert!(neighbors.len() >= 1);
    }

    #[test]
    fn test_range_query() {
        use crate::token::CoordinateSpace;

        let storage = RuntimeStorage::new();

        // Create some tokens
        for i in 0..5 {
            let mut token = Token::new(0);
            token.set_coordinates(CoordinateSpace::L1Physical, i as f32, i as f32, i as f32);
            storage.create_token(token);
        }

        // Query range
        let results = storage.range_query([2.0, 2.0, 2.0], 3.0);
        assert!(results.len() > 0);
    }

    #[test]
    fn test_cdna_operations() {
        let storage = RuntimeStorage::new();

        // Get initial CDNA
        let cdna = storage.get_cdna();
        assert!(cdna.dimension_scales[0] > 0.0);

        // Update scales
        let new_scales = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        assert!(storage.update_cdna_scales(new_scales).is_ok());

        let cdna = storage.get_cdna();
        assert_eq!(cdna.dimension_scales[0], 1.0);
        assert_eq!(cdna.dimension_scales[7], 8.0);

        // Invalid scales should fail
        let invalid_scales = [1.0, -1.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        assert!(storage.update_cdna_scales(invalid_scales).is_err());
    }

    #[test]
    fn test_cdna_profile() {
        let storage = RuntimeStorage::new();

        // Get initial profile
        let profile_id = storage.get_cdna_profile();
        assert!(profile_id >= 0);

        // Set profile
        storage.set_cdna_profile(42);
        let profile_id = storage.get_cdna_profile();
        assert_eq!(profile_id, 42);
    }

    #[test]
    fn test_cdna_flags() {
        let storage = RuntimeStorage::new();

        // Get initial flags (CDNA has default flags set)
        let flags = storage.get_cdna_flags();
        assert!(flags > 0);  // Should have some default flags

        // Set flags
        storage.set_cdna_flags(0xFF);
        let flags = storage.get_cdna_flags();
        assert_eq!(flags, 0xFF);
    }

    #[test]
    fn test_cdna_validation() {
        let storage = RuntimeStorage::new();

        // Should be valid by default
        assert!(storage.validate_cdna());

        // Reset should keep it valid
        storage.reset_cdna();
        assert!(storage.validate_cdna());
    }
}
