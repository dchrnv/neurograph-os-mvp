//! Grid V2.0 - 8-Dimensional Semantic Space
//!
//! Grid provides:
//! - 8 independent coordinate spaces (L1-L8)
//! - Spatial indexing for fast neighbor search
//! - Field calculations (density, influence)
//! - Token storage and retrieval
//!
//! Version: 2.0 (MVP implementation)

use crate::token::{CoordinateSpace, Token};
use std::collections::HashMap;

/// Grid configuration
#[derive(Clone, Debug)]
pub struct GridConfig {
    /// Bucket size for spatial index (in decoded units)
    pub bucket_size: f32,

    /// Density threshold for field detection
    pub density_threshold: f32,

    /// Minimum nodes to form a field
    pub min_field_nodes: usize,
}

impl Default for GridConfig {
    fn default() -> Self {
        GridConfig {
            bucket_size: 10.0,
            density_threshold: 0.5,
            min_field_nodes: 3,
        }
    }
}

/// Spatial bucket key for indexing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct BucketKey {
    x: i32,
    y: i32,
    z: i32,
}

impl BucketKey {
    fn from_coords(x: f32, y: f32, z: f32, bucket_size: f32) -> Self {
        BucketKey {
            x: (x / bucket_size).floor() as i32,
            y: (y / bucket_size).floor() as i32,
            z: (z / bucket_size).floor() as i32,
        }
    }

    fn neighbors(&self) -> Vec<BucketKey> {
        let mut result = Vec::with_capacity(27);
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    result.push(BucketKey {
                        x: self.x + dx,
                        y: self.y + dy,
                        z: self.z + dz,
                    });
                }
            }
        }
        result
    }
}

/// Spatial index for one coordinate space
struct SpatialIndex {
    /// Buckets: BucketKey -> Vec<token_id>
    buckets: HashMap<BucketKey, Vec<u32>>,

    /// Bucket size
    bucket_size: f32,
}

impl SpatialIndex {
    fn new(bucket_size: f32) -> Self {
        SpatialIndex {
            buckets: HashMap::new(),
            bucket_size,
        }
    }

    fn add(&mut self, token_id: u32, x: f32, y: f32, z: f32) {
        let key = BucketKey::from_coords(x, y, z, self.bucket_size);
        self.buckets
            .entry(key)
            .or_insert_with(Vec::new)
            .push(token_id);
    }

    fn remove(&mut self, token_id: u32, x: f32, y: f32, z: f32) {
        let key = BucketKey::from_coords(x, y, z, self.bucket_size);
        if let Some(bucket) = self.buckets.get_mut(&key) {
            bucket.retain(|&id| id != token_id);
            if bucket.is_empty() {
                self.buckets.remove(&key);
            }
        }
    }

    fn find_candidates(&self, x: f32, y: f32, z: f32, radius: f32) -> Vec<u32> {
        let center_key = BucketKey::from_coords(x, y, z, self.bucket_size);
        let search_range = (radius / self.bucket_size).ceil() as i32;

        let mut candidates = Vec::new();
        for dx in -search_range..=search_range {
            for dy in -search_range..=search_range {
                for dz in -search_range..=search_range {
                    let key = BucketKey {
                        x: center_key.x + dx,
                        y: center_key.y + dy,
                        z: center_key.z + dz,
                    };
                    if let Some(bucket) = self.buckets.get(&key) {
                        candidates.extend_from_slice(bucket);
                    }
                }
            }
        }
        candidates
    }
}

/// Grid V2.0 - 8-dimensional semantic space
pub struct Grid {
    /// Configuration
    config: GridConfig,

    /// Token storage: token_id -> Token
    tokens: HashMap<u32, Token>,

    /// Spatial indexes (one per coordinate space)
    indexes: [Option<SpatialIndex>; 8],
}

impl Grid {
    /// Create a new Grid with default configuration
    pub fn new() -> Self {
        Grid::with_config(GridConfig::default())
    }

    /// Create a new Grid with custom configuration
    pub fn with_config(config: GridConfig) -> Self {
        Grid {
            tokens: HashMap::new(),
            indexes: [
                Some(SpatialIndex::new(config.bucket_size)),
                Some(SpatialIndex::new(config.bucket_size)),
                Some(SpatialIndex::new(config.bucket_size)),
                Some(SpatialIndex::new(config.bucket_size)),
                Some(SpatialIndex::new(config.bucket_size)),
                Some(SpatialIndex::new(config.bucket_size)),
                Some(SpatialIndex::new(config.bucket_size)),
                Some(SpatialIndex::new(config.bucket_size)),
            ],
            config,
        }
    }

    /// Add a token to the grid
    pub fn add(&mut self, token: Token) -> Result<(), &'static str> {
        let token_id = token.id;

        // Check if token already exists
        if self.tokens.contains_key(&token_id) {
            return Err("Token with this ID already exists");
        }

        // Index token in all coordinate spaces where it has valid coordinates
        for level in 0..8 {
            let [x, y, z] = token.get_coordinates(match level {
                0 => CoordinateSpace::L1Physical,
                1 => CoordinateSpace::L2Sensory,
                2 => CoordinateSpace::L3Motor,
                3 => CoordinateSpace::L4Emotional,
                4 => CoordinateSpace::L5Cognitive,
                5 => CoordinateSpace::L6Social,
                6 => CoordinateSpace::L7Temporal,
                7 => CoordinateSpace::L8Abstract,
                _ => unreachable!(),
            });

            // Check if coordinates are defined (not 127 marker)
            if token.coordinates[level][0] != 127 {
                if let Some(index) = &mut self.indexes[level] {
                    index.add(token_id, x, y, z);
                }
            }
        }

        // Store token
        self.tokens.insert(token_id, token);
        Ok(())
    }

    /// Remove a token from the grid
    pub fn remove(&mut self, token_id: u32) -> Option<Token> {
        if let Some(token) = self.tokens.remove(&token_id) {
            // Remove from spatial indexes
            for level in 0..8 {
                if token.coordinates[level][0] != 127 {
                    let [x, y, z] = token.get_coordinates(match level {
                        0 => CoordinateSpace::L1Physical,
                        1 => CoordinateSpace::L2Sensory,
                        2 => CoordinateSpace::L3Motor,
                        3 => CoordinateSpace::L4Emotional,
                        4 => CoordinateSpace::L5Cognitive,
                        5 => CoordinateSpace::L6Social,
                        6 => CoordinateSpace::L7Temporal,
                        7 => CoordinateSpace::L8Abstract,
                        _ => unreachable!(),
                    });

                    if let Some(index) = &mut self.indexes[level] {
                        index.remove(token_id, x, y, z);
                    }
                }
            }
            Some(token)
        } else {
            None
        }
    }

    /// Get a token by ID
    pub fn get(&self, token_id: u32) -> Option<&Token> {
        self.tokens.get(&token_id)
    }

    /// Get number of tokens in the grid
    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    /// Check if grid is empty
    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    /// Find neighbors within radius in a specific space
    pub fn find_neighbors(
        &self,
        center_token_id: u32,
        space: CoordinateSpace,
        radius: f32,
        max_results: usize,
    ) -> Vec<(u32, f32)> {
        let center_token = match self.tokens.get(&center_token_id) {
            Some(t) => t,
            None => return Vec::new(),
        };

        let level = match space {
            CoordinateSpace::L1Physical => 0,
            CoordinateSpace::L2Sensory => 1,
            CoordinateSpace::L3Motor => 2,
            CoordinateSpace::L4Emotional => 3,
            CoordinateSpace::L5Cognitive => 4,
            CoordinateSpace::L6Social => 5,
            CoordinateSpace::L7Temporal => 6,
            CoordinateSpace::L8Abstract => 7,
        };

        // Get center coordinates
        let [cx, cy, cz] = center_token.get_coordinates(space);

        // Get candidates from spatial index
        let candidates = if let Some(index) = &self.indexes[level] {
            index.find_candidates(cx, cy, cz, radius)
        } else {
            return Vec::new();
        };

        // Calculate distances and filter
        let mut results: Vec<(u32, f32)> = candidates
            .into_iter()
            .filter(|&id| id != center_token_id)
            .filter_map(|id| {
                let token = self.tokens.get(&id)?;
                let [tx, ty, tz] = token.get_coordinates(space);
                let distance = ((tx - cx).powi(2) + (ty - cy).powi(2) + (tz - cz).powi(2)).sqrt();
                if distance <= radius {
                    Some((id, distance))
                } else {
                    None
                }
            })
            .collect();

        // Sort by distance
        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        // Limit results
        results.truncate(max_results);
        results
    }

    /// Range query: find all tokens within radius of a point in a space
    pub fn range_query(
        &self,
        space: CoordinateSpace,
        x: f32,
        y: f32,
        z: f32,
        radius: f32,
    ) -> Vec<(u32, f32)> {
        let level = match space {
            CoordinateSpace::L1Physical => 0,
            CoordinateSpace::L2Sensory => 1,
            CoordinateSpace::L3Motor => 2,
            CoordinateSpace::L4Emotional => 3,
            CoordinateSpace::L5Cognitive => 4,
            CoordinateSpace::L6Social => 5,
            CoordinateSpace::L7Temporal => 6,
            CoordinateSpace::L8Abstract => 7,
        };

        // Get candidates from spatial index
        let candidates = if let Some(index) = &self.indexes[level] {
            index.find_candidates(x, y, z, radius)
        } else {
            return Vec::new();
        };

        // Calculate distances and filter
        let mut results: Vec<(u32, f32)> = candidates
            .into_iter()
            .filter_map(|id| {
                let token = self.tokens.get(&id)?;
                let [tx, ty, tz] = token.get_coordinates(space);
                let distance = ((tx - x).powi(2) + (ty - y).powi(2) + (tz - z).powi(2)).sqrt();
                if distance <= radius {
                    Some((id, distance))
                } else {
                    None
                }
            })
            .collect();

        // Sort by distance
        results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        results
    }

    /// Calculate field influence at a point in a space
    pub fn calculate_field_influence(
        &self,
        space: CoordinateSpace,
        x: f32,
        y: f32,
        z: f32,
        radius: f32,
    ) -> f32 {
        let nearby = self.range_query(space, x, y, z, radius);

        let mut total_influence = 0.0;
        for (token_id, distance) in nearby {
            if let Some(token) = self.tokens.get(&token_id) {
                let field_radius = token.field_radius as f32 / 100.0;
                let field_strength = token.field_strength as f32 / 255.0;

                if distance <= field_radius {
                    // Linear falloff
                    let influence = field_strength * (1.0 - distance / field_radius);
                    total_influence += influence;
                }
            }
        }

        total_influence
    }

    /// Calculate node density at a point in a space
    pub fn calculate_density(
        &self,
        space: CoordinateSpace,
        x: f32,
        y: f32,
        z: f32,
        radius: f32,
    ) -> f32 {
        let nearby = self.range_query(space, x, y, z, radius);
        let volume = (4.0 / 3.0) * std::f32::consts::PI * radius.powi(3);
        nearby.len() as f32 / volume
    }
}

impl Default for Grid {
    fn default() -> Self {
        Grid::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::EntityType;

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new();
        assert_eq!(grid.len(), 0);
        assert!(grid.is_empty());
    }

    #[test]
    fn test_add_remove_token() {
        let mut grid = Grid::new();

        let mut token = Token::new(1);
        token.set_coordinates(CoordinateSpace::L1Physical, 10.00, 20.00, 5.00);

        assert!(grid.add(token).is_ok());
        assert_eq!(grid.len(), 1);

        assert!(grid.get(1).is_some());
        assert!(grid.remove(1).is_some());
        assert_eq!(grid.len(), 0);
    }

    #[test]
    fn test_find_neighbors() {
        let mut grid = Grid::new();

        // Add center token
        let mut token1 = Token::new(1);
        token1.set_coordinates(CoordinateSpace::L1Physical, 0.00, 0.00, 0.00);
        grid.add(token1).unwrap();

        // Add nearby tokens
        let mut token2 = Token::new(2);
        token2.set_coordinates(CoordinateSpace::L1Physical, 1.00, 0.00, 0.00);
        grid.add(token2).unwrap();

        let mut token3 = Token::new(3);
        token3.set_coordinates(CoordinateSpace::L1Physical, 0.00, 1.00, 0.00);
        grid.add(token3).unwrap();

        // Add far token
        let mut token4 = Token::new(4);
        token4.set_coordinates(CoordinateSpace::L1Physical, 100.00, 0.00, 0.00);
        grid.add(token4).unwrap();

        // Find neighbors within radius 5
        let neighbors = grid.find_neighbors(1, CoordinateSpace::L1Physical, 5.00, 10);
        assert_eq!(neighbors.len(), 2); // token2 and token3
    }

    #[test]
    fn test_range_query() {
        let mut grid = Grid::new();

        for i in 0..10 {
            let mut token = Token::new(i);
            token.set_coordinates(CoordinateSpace::L1Physical, i as f32, 0.00, 0.00);
            grid.add(token).unwrap();
        }

        // Query around center (5, 0, 0) with radius 2
        let results = grid.range_query(CoordinateSpace::L1Physical, 5.00, 0.00, 0.00, 2.00);
        // Should find tokens at 3, 4, 5, 6, 7
        assert_eq!(results.len(), 5);
    }

    #[test]
    fn test_field_influence() {
        let mut grid = Grid::new();

        let mut token = Token::new(1);
        token.set_coordinates(CoordinateSpace::L1Physical, 0.00, 0.00, 0.00);
        token.field_radius = 100; // 1.0 in decoded units
        token.field_strength = 255; // 1.0 in decoded units
        grid.add(token).unwrap();

        // At center, should be max influence
        let influence_center =
            grid.calculate_field_influence(CoordinateSpace::L1Physical, 0.00, 0.00, 0.00, 2.00);
        assert!(influence_center > 0.9);

        // At edge, should be minimal influence
        let influence_edge =
            grid.calculate_field_influence(CoordinateSpace::L1Physical, 1.00, 0.00, 0.00, 2.00);
        assert!(influence_edge < 0.1);
    }

    #[test]
    fn test_density_calculation() {
        let mut grid = Grid::new();

        // Add 5 tokens in a cluster
        for i in 0..5 {
            let mut token = Token::new(i);
            token.set_coordinates(CoordinateSpace::L1Physical, (i as f32) * 0.1, 0.00, 0.00);
            grid.add(token).unwrap();
        }

        let density = grid.calculate_density(CoordinateSpace::L1Physical, 0.20, 0.00, 0.00, 1.00);
        assert!(density > 0.0);
    }
}
