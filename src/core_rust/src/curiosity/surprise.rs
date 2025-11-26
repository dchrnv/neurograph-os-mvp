// NeuroGraph OS - Surprise Calculation v0.38.0
//
// Tracks prediction errors and calculates surprise

use std::collections::VecDeque;
use serde::{Deserialize, Serialize};

/// A surprise event (prediction vs actual)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurpriseEvent {
    /// Predicted 8D state
    pub predicted: [f64; 8],

    /// Actual 8D state
    pub actual: [f64; 8],

    /// Calculated surprise value (0.0 to 1.0+)
    pub surprise: f32,

    /// Timestamp
    pub timestamp: std::time::SystemTime,
}

impl SurpriseEvent {
    /// Calculate Euclidean distance between predicted and actual
    fn euclidean_distance(a: &[f64; 8], b: &[f64; 8]) -> f64 {
        let mut sum = 0.0;
        for i in 0..8 {
            let diff = a[i] - b[i];
            sum += diff * diff;
        }
        sum.sqrt()
    }

    /// Create new surprise event and calculate surprise
    pub fn new(predicted: [f64; 8], actual: [f64; 8]) -> Self {
        // Surprise = normalized distance between predicted and actual
        let distance = Self::euclidean_distance(&predicted, &actual);

        // Normalize by max possible distance (sqrt(8) for unit cube)
        // Scale to 0-1 range, but allow values > 1.0 for very large surprises
        let surprise = (distance / 2.828) as f32; // sqrt(8) ≈ 2.828

        Self {
            predicted,
            actual,
            surprise,
            timestamp: std::time::SystemTime::now(),
        }
    }
}

/// Tracks history of surprise events
pub struct SurpriseHistory {
    /// Ring buffer of recent surprise events
    events: VecDeque<SurpriseEvent>,

    /// Maximum history size
    max_size: usize,

    /// Running average surprise
    avg_surprise: f32,

    /// Total events processed
    total_events: usize,
}

impl SurpriseHistory {
    /// Create new surprise history with given capacity
    pub fn new(max_size: usize) -> Self {
        Self {
            events: VecDeque::with_capacity(max_size),
            max_size,
            avg_surprise: 0.0,
            total_events: 0,
        }
    }

    /// Add a new surprise event
    pub fn add_event(&mut self, predicted: [f64; 8], actual: [f64; 8]) -> f32 {
        let event = SurpriseEvent::new(predicted, actual);
        let surprise = event.surprise;

        // Add to history (remove oldest if at capacity)
        if self.events.len() >= self.max_size {
            self.events.pop_front();
        }
        self.events.push_back(event);

        // Update running average
        let alpha = 0.1; // Learning rate
        self.avg_surprise = self.avg_surprise * (1.0 - alpha) + surprise * alpha;

        self.total_events += 1;

        surprise
    }

    /// Calculate surprise for a prediction vs actual
    pub fn calculate_surprise(&mut self, predicted: [f64; 8], actual: [f64; 8]) -> f32 {
        self.add_event(predicted, actual)
    }

    /// Get current surprise level (recent average)
    pub fn current_surprise(&self) -> f32 {
        if self.events.is_empty() {
            return 0.0;
        }

        // Average of recent events (last 10 or all if less)
        let recent_count = self.events.len().min(10);
        let sum: f32 = self.events
            .iter()
            .rev()
            .take(recent_count)
            .map(|e| e.surprise)
            .sum();

        sum / recent_count as f32
    }

    /// Get average surprise over all history
    pub fn avg_surprise(&self) -> f32 {
        self.avg_surprise
    }

    /// Get maximum surprise in recent history
    pub fn max_recent_surprise(&self) -> f32 {
        self.events
            .iter()
            .map(|e| e.surprise)
            .fold(0.0_f32, f32::max)
    }

    /// Get statistics
    pub fn stats(&self) -> SurpriseStats {
        SurpriseStats {
            current_surprise: self.current_surprise(),
            avg_surprise: self.avg_surprise,
            max_recent_surprise: self.max_recent_surprise(),
            history_size: self.events.len(),
            total_events: self.total_events,
        }
    }

    /// Clear history
    pub fn clear(&mut self) {
        self.events.clear();
        self.avg_surprise = 0.0;
    }
}

impl Default for SurpriseHistory {
    fn default() -> Self {
        Self::new(50) // Default history size
    }
}

/// Statistics for surprise tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurpriseStats {
    pub current_surprise: f32,
    pub avg_surprise: f32,
    pub max_recent_surprise: f32,
    pub history_size: usize,
    pub total_events: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_surprise_zero() {
        let mut history = SurpriseHistory::new(10);
        let state = [1.0; 8];

        // Perfect prediction = zero surprise
        let surprise = history.calculate_surprise(state, state);
        assert_eq!(surprise, 0.0);
    }

    #[test]
    fn test_surprise_nonzero() {
        let mut history = SurpriseHistory::new(10);
        let predicted = [0.0; 8];
        let actual = [1.0; 8];

        // Distance = sqrt(8) ≈ 2.828
        // Normalized surprise ≈ 1.0
        let surprise = history.calculate_surprise(predicted, actual);
        assert!(surprise > 0.9 && surprise < 1.1);
    }

    #[test]
    fn test_surprise_history_capacity() {
        let mut history = SurpriseHistory::new(5);

        // Add 10 events
        for i in 0..10 {
            let predicted = [i as f64; 8];
            let actual = [(i + 1) as f64; 8];
            history.add_event(predicted, actual);
        }

        // Should only keep last 5
        let stats = history.stats();
        assert_eq!(stats.history_size, 5);
        assert_eq!(stats.total_events, 10);
    }

    #[test]
    fn test_current_surprise() {
        let mut history = SurpriseHistory::new(20);

        // Add some low surprise events
        for _ in 0..5 {
            history.add_event([0.0; 8], [0.1; 8]);
        }

        // Add a high surprise event
        history.add_event([0.0; 8], [5.0; 8]);

        // Current surprise should be influenced by recent high surprise
        let current = history.current_surprise();
        assert!(current > 0.5);
    }

    #[test]
    fn test_avg_surprise() {
        let mut history = SurpriseHistory::new(10);

        // Add mix of surprises
        history.add_event([0.0; 8], [0.0; 8]); // 0.0
        history.add_event([0.0; 8], [1.0; 8]); // ~1.0

        let avg = history.avg_surprise();
        assert!(avg > 0.0 && avg < 1.0);
    }

    #[test]
    fn test_clear() {
        let mut history = SurpriseHistory::new(10);

        history.add_event([0.0; 8], [1.0; 8]);
        assert!(history.stats().history_size > 0);

        history.clear();
        assert_eq!(history.stats().history_size, 0);
        assert_eq!(history.avg_surprise(), 0.0);
    }
}
