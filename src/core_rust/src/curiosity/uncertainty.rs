// NeuroGraph OS - Uncertainty Tracking v0.38.0
//
// Tracks confidence and uncertainty for 8D state space cells

use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use serde::{Deserialize, Serialize};

/// Key for 8D grid cell (discretized coordinates)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CellKey {
    /// Discretized 8D coordinates [i32; 8]
    pub coords: [i32; 8],
}

impl CellKey {
    /// Create cell key from continuous 8D state
    /// Discretizes by rounding to nearest integer
    pub fn from_state(state: &[f64; 8]) -> Self {
        Self {
            coords: [
                state[0].round() as i32,
                state[1].round() as i32,
                state[2].round() as i32,
                state[3].round() as i32,
                state[4].round() as i32,
                state[5].round() as i32,
                state[6].round() as i32,
                state[7].round() as i32,
            ],
        }
    }

    /// Create cell key from discretized coordinates
    pub fn from_coords(coords: [i32; 8]) -> Self {
        Self { coords }
    }
}

/// Confidence information for a cell
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CellConfidence {
    /// Confidence level (0.0 to 1.0, higher = more certain)
    pub confidence: f32,

    /// Number of times this cell has been visited
    pub visit_count: usize,

    /// Last time this cell was visited
    pub last_visit: SystemTime,

    /// Running average of prediction accuracy
    pub accuracy: f32,
}

impl CellConfidence {
    /// Create new cell confidence with initial visit
    pub fn new() -> Self {
        Self {
            confidence: 0.0, // Start with zero confidence
            visit_count: 1,
            last_visit: SystemTime::now(),
            accuracy: 0.0,
        }
    }

    /// Update confidence based on new observation
    pub fn update(&mut self, prediction_accuracy: f32) {
        self.visit_count += 1;
        self.last_visit = SystemTime::now();

        // Update running average accuracy
        let alpha = 0.1; // Learning rate
        self.accuracy = self.accuracy * (1.0 - alpha) + prediction_accuracy * alpha;

        // Confidence increases with visit count and accuracy
        // Formula: conf = accuracy * (1 - exp(-visits/10))
        let visit_factor = 1.0 - (-(self.visit_count as f32) / 10.0).exp();
        self.confidence = self.accuracy * visit_factor;
        self.confidence = self.confidence.clamp(0.0, 1.0);
    }

    /// Check if cell is old (not visited recently)
    pub fn is_old(&self, max_age: Duration) -> bool {
        SystemTime::now()
            .duration_since(self.last_visit)
            .unwrap_or(Duration::from_secs(0))
            > max_age
    }
}

impl Default for CellConfidence {
    fn default() -> Self {
        Self::new()
    }
}

/// Tracks uncertainty across 8D state space
pub struct UncertaintyTracker {
    /// Map from cell key to confidence
    cells: HashMap<CellKey, CellConfidence>,

    /// Total cells tracked
    total_cells: usize,

    /// Total visits across all cells
    total_visits: usize,
}

impl UncertaintyTracker {
    /// Create new uncertainty tracker
    pub fn new() -> Self {
        Self {
            cells: HashMap::new(),
            total_cells: 0,
            total_visits: 0,
        }
    }

    /// Get uncertainty for a state (1.0 - confidence)
    pub fn get_uncertainty(&self, state: &[f64; 8]) -> f32 {
        let key = CellKey::from_state(state);
        self.cells
            .get(&key)
            .map(|conf| 1.0 - conf.confidence)
            .unwrap_or(1.0) // Unvisited cells have maximum uncertainty
    }

    /// Get confidence for a state
    pub fn get_confidence(&self, state: &[f64; 8]) -> f32 {
        let key = CellKey::from_state(state);
        self.cells
            .get(&key)
            .map(|conf| conf.confidence)
            .unwrap_or(0.0)
    }

    /// Update confidence for a state based on prediction accuracy
    pub fn update(&mut self, state: &[f64; 8], prediction_accuracy: f32) {
        let key = CellKey::from_state(state);

        self.cells
            .entry(key)
            .and_modify(|conf| conf.update(prediction_accuracy))
            .or_insert_with(|| {
                self.total_cells += 1;
                let mut conf = CellConfidence::new();
                conf.accuracy = prediction_accuracy;
                conf.confidence = prediction_accuracy * 0.1; // Initial confidence scaled down
                conf
            });

        self.total_visits += 1;
    }

    /// Get visit count for a cell
    pub fn get_visit_count(&self, state: &[f64; 8]) -> usize {
        let key = CellKey::from_state(state);
        self.cells
            .get(&key)
            .map(|conf| conf.visit_count)
            .unwrap_or(0)
    }

    /// Cleanup old cells that haven't been visited recently
    pub fn cleanup_old_cells(&mut self, max_age: Duration, min_visits: usize) -> usize {
        let before_count = self.cells.len();

        self.cells.retain(|_, conf| {
            // Keep cell if it's been visited enough times OR it's recent
            conf.visit_count >= min_visits || !conf.is_old(max_age)
        });

        let removed = before_count - self.cells.len();
        self.total_cells = self.cells.len();

        removed
    }

    /// Get most uncertain cells (high uncertainty, low confidence)
    pub fn get_most_uncertain(&self, limit: usize) -> Vec<(CellKey, f32)> {
        let mut cells: Vec<_> = self.cells
            .iter()
            .map(|(key, conf)| (*key, 1.0 - conf.confidence))
            .collect();

        // Sort by uncertainty (descending)
        cells.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        cells.into_iter().take(limit).collect()
    }

    /// Get statistics
    pub fn stats(&self) -> UncertaintyStats {
        let avg_confidence = if self.cells.is_empty() {
            0.0
        } else {
            self.cells.values().map(|c| c.confidence).sum::<f32>() / self.cells.len() as f32
        };

        let avg_visits = if self.cells.is_empty() {
            0.0
        } else {
            self.total_visits as f32 / self.cells.len() as f32
        };

        UncertaintyStats {
            total_cells: self.total_cells,
            total_visits: self.total_visits,
            avg_confidence,
            avg_visits,
        }
    }
}

impl Default for UncertaintyTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics for uncertainty tracker
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UncertaintyStats {
    pub total_cells: usize,
    pub total_visits: usize,
    pub avg_confidence: f32,
    pub avg_visits: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cell_key_from_state() {
        let state = [1.2, 2.7, -1.5, 0.0, 3.9, -2.1, 4.5, -0.8];
        let key = CellKey::from_state(&state);
        assert_eq!(key.coords, [1, 3, -2, 0, 4, -2, 5, -1]);
    }

    #[test]
    fn test_uncertainty_unvisited() {
        let tracker = UncertaintyTracker::new();
        let state = [0.0; 8];
        assert_eq!(tracker.get_uncertainty(&state), 1.0);
        assert_eq!(tracker.get_confidence(&state), 0.0);
    }

    #[test]
    fn test_uncertainty_update() {
        let mut tracker = UncertaintyTracker::new();
        let state = [0.0; 8];

        tracker.update(&state, 0.8);
        let conf1 = tracker.get_confidence(&state);
        assert!(conf1 > 0.0 && conf1 < 1.0);

        tracker.update(&state, 0.9);
        let conf2 = tracker.get_confidence(&state);
        assert!(conf2 > conf1); // Confidence should increase
    }

    #[test]
    fn test_cleanup_old_cells() {
        let mut tracker = UncertaintyTracker::new();

        // Add some cells
        for i in 0..5 {
            let state = [i as f64; 8];
            tracker.update(&state, 0.5);
        }

        assert_eq!(tracker.stats().total_cells, 5);

        // Cleanup with very short max_age (all cells become old)
        let removed = tracker.cleanup_old_cells(Duration::from_nanos(1), 10);

        // All cells should be removed (none have 10+ visits)
        assert_eq!(removed, 5);
        assert_eq!(tracker.stats().total_cells, 0);
    }

    #[test]
    fn test_most_uncertain() {
        let mut tracker = UncertaintyTracker::new();

        // Add cells with different confidences
        tracker.update(&[1.0; 8], 0.9); // High confidence
        tracker.update(&[2.0; 8], 0.3); // Low confidence
        tracker.update(&[3.0; 8], 0.1); // Very low confidence

        let uncertain = tracker.get_most_uncertain(2);
        assert_eq!(uncertain.len(), 2);

        // Most uncertain should be [3.0; 8] (confidence ~0.1)
        assert!(uncertain[0].1 > 0.8); // Uncertainty = 1 - conf
    }
}
