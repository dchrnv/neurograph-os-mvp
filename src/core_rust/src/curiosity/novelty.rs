// NeuroGraph OS - Novelty Tracking v0.38.0
//
// Tracks when cells were last seen to measure novelty

use crate::curiosity::uncertainty::CellKey;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use serde::{Deserialize, Serialize};

/// Tracks novelty of states based on recency
pub struct NoveltyTracker {
    /// Map from cell key to last seen timestamp
    last_seen: HashMap<CellKey, SystemTime>,

    /// Total unique states seen
    total_unique: usize,

    /// Total state observations
    total_observations: usize,
}

impl NoveltyTracker {
    /// Create new novelty tracker
    pub fn new() -> Self {
        Self {
            last_seen: HashMap::new(),
            total_unique: 0,
            total_observations: 0,
        }
    }

    /// Calculate novelty for a state (0.0 to 1.0)
    /// Higher values = more novel (not seen recently or never seen)
    pub fn calculate_novelty(&mut self, state: &[f64; 8]) -> f32 {
        let key = CellKey::from_state(state);
        let now = SystemTime::now();

        let novelty = match self.last_seen.get(&key) {
            Some(last_time) => {
                // Calculate time since last seen
                let duration = now.duration_since(*last_time).unwrap_or(Duration::from_secs(0));

                // Novelty increases with time
                // Formula: 1 - exp(-seconds/3600)
                // After 1 hour, novelty ≈ 0.63
                // After 1 day, novelty ≈ 1.0
                let seconds = duration.as_secs() as f32;
                1.0 - (-seconds / 3600.0).exp()
            }
            None => {
                // Never seen before = maximum novelty
                self.total_unique += 1;
                1.0
            }
        };

        // Update last seen
        self.last_seen.insert(key, now);
        self.total_observations += 1;

        novelty
    }

    /// Get time since last seen for a state
    pub fn time_since_seen(&self, state: &[f64; 8]) -> Option<Duration> {
        let key = CellKey::from_state(state);
        self.last_seen.get(&key).and_then(|last_time| {
            SystemTime::now().duration_since(*last_time).ok()
        })
    }

    /// Check if state has been seen before
    pub fn has_seen(&self, state: &[f64; 8]) -> bool {
        let key = CellKey::from_state(state);
        self.last_seen.contains_key(&key)
    }

    /// Get count of unique states seen
    pub fn unique_count(&self) -> usize {
        self.last_seen.len()
    }

    /// Cleanup states not seen in a while
    pub fn cleanup_old(&mut self, max_age: Duration) -> usize {
        let now = SystemTime::now();
        let before_count = self.last_seen.len();

        self.last_seen.retain(|_, last_time| {
            now.duration_since(*last_time).unwrap_or(Duration::from_secs(0)) < max_age
        });

        before_count - self.last_seen.len()
    }

    /// Get statistics
    pub fn stats(&self) -> NoveltyStats {
        NoveltyStats {
            unique_states: self.last_seen.len(),
            total_observations: self.total_observations,
            total_unique_seen: self.total_unique,
        }
    }

    /// Clear all history
    pub fn clear(&mut self) {
        self.last_seen.clear();
        self.total_unique = 0;
        self.total_observations = 0;
    }
}

impl Default for NoveltyTracker {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics for novelty tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoveltyStats {
    pub unique_states: usize,
    pub total_observations: usize,
    pub total_unique_seen: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_novelty_first_time() {
        let mut tracker = NoveltyTracker::new();
        let state = [0.0; 8];

        // First time seeing state = max novelty
        let novelty = tracker.calculate_novelty(&state);
        assert_eq!(novelty, 1.0);
    }

    #[test]
    fn test_novelty_immediate_repeat() {
        let mut tracker = NoveltyTracker::new();
        let state = [0.0; 8];

        tracker.calculate_novelty(&state);

        // Immediate repeat = low novelty
        let novelty = tracker.calculate_novelty(&state);
        assert!(novelty < 0.1);
    }

    #[test]
    fn test_novelty_after_delay() {
        let mut tracker = NoveltyTracker::new();
        let state = [0.0; 8];

        let first_novelty = tracker.calculate_novelty(&state);
        assert_eq!(first_novelty, 1.0);

        thread::sleep(Duration::from_millis(100));

        // After 100ms delay, novelty should be very low but >= 0
        // Formula: 1 - exp(-0.1 / 3600) ≈ 0.000027 ≈ 0.0 in f32
        // The formula is designed for hour-scale changes, not milliseconds
        let novelty = tracker.calculate_novelty(&state);
        assert!(novelty >= 0.0 && novelty < 0.1,
            "novelty {} should be very low (near 0) after 100ms", novelty);
    }

    #[test]
    fn test_has_seen() {
        let mut tracker = NoveltyTracker::new();
        let state = [1.0; 8];

        assert!(!tracker.has_seen(&state));

        tracker.calculate_novelty(&state);

        assert!(tracker.has_seen(&state));
    }

    #[test]
    fn test_unique_count() {
        let mut tracker = NoveltyTracker::new();

        tracker.calculate_novelty(&[0.0; 8]);
        tracker.calculate_novelty(&[1.0; 8]);
        tracker.calculate_novelty(&[0.0; 8]); // Repeat

        assert_eq!(tracker.unique_count(), 2);
    }

    #[test]
    fn test_cleanup_old() {
        let mut tracker = NoveltyTracker::new();

        tracker.calculate_novelty(&[0.0; 8]);
        thread::sleep(Duration::from_millis(100));

        // Cleanup with very short max_age
        let removed = tracker.cleanup_old(Duration::from_millis(50));

        assert_eq!(removed, 1);
        assert_eq!(tracker.unique_count(), 0);
    }

    #[test]
    fn test_stats() {
        let mut tracker = NoveltyTracker::new();

        tracker.calculate_novelty(&[0.0; 8]);
        tracker.calculate_novelty(&[1.0; 8]);
        tracker.calculate_novelty(&[0.0; 8]); // Repeat

        let stats = tracker.stats();
        assert_eq!(stats.unique_states, 2);
        assert_eq!(stats.total_observations, 3);
        assert_eq!(stats.total_unique_seen, 2);
    }

    #[test]
    fn test_clear() {
        let mut tracker = NoveltyTracker::new();

        tracker.calculate_novelty(&[0.0; 8]);
        tracker.clear();

        assert_eq!(tracker.unique_count(), 0);
        assert_eq!(tracker.stats().total_observations, 0);
    }
}
