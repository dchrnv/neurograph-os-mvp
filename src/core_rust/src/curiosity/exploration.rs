// NeuroGraph OS - Exploration Targets v0.38.0
//
// Priority queue of exploration targets based on curiosity scores

use std::collections::BinaryHeap;
use std::cmp::Ordering;
use serde::{Deserialize, Serialize};

/// Reason for exploration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExplorationReason {
    /// High uncertainty (haven't been here much)
    HighUncertainty,

    /// High surprise (predictions were wrong)
    HighSurprise,

    /// Novel (haven't seen recently)
    Novel,

    /// Combination of factors
    Combined,

    /// Manual exploration request
    Manual,
}

/// Priority level for exploration
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ExplorationPriority {
    Low = 1,
    Medium = 2,
    High = 3,
    Critical = 4,
}

impl Default for ExplorationPriority {
    fn default() -> Self {
        Self::Medium
    }
}

/// A target for exploration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplorationTarget {
    /// 8D state to explore
    pub state: [f64; 8],

    /// Curiosity score (0.0 to 1.0+, higher = more interesting)
    pub score: f32,

    /// Reason for exploration
    pub reason: ExplorationReason,

    /// Priority level
    pub priority: ExplorationPriority,

    /// When this target was created
    pub created_at: std::time::SystemTime,

    /// Additional context/metadata
    pub context: Option<String>,
}

impl ExplorationTarget {
    /// Create new exploration target
    pub fn new(state: [f64; 8], score: f32, reason: ExplorationReason) -> Self {
        // Auto-assign priority based on score
        let priority = if score > 0.8 {
            ExplorationPriority::Critical
        } else if score > 0.6 {
            ExplorationPriority::High
        } else if score > 0.4 {
            ExplorationPriority::Medium
        } else {
            ExplorationPriority::Low
        };

        Self {
            state,
            score,
            reason,
            priority,
            created_at: std::time::SystemTime::now(),
            context: None,
        }
    }

    /// Create with explicit priority
    pub fn with_priority(
        state: [f64; 8],
        score: f32,
        reason: ExplorationReason,
        priority: ExplorationPriority,
    ) -> Self {
        Self {
            state,
            score,
            reason,
            priority,
            created_at: std::time::SystemTime::now(),
            context: None,
        }
    }

    /// Add context information
    pub fn with_context(mut self, context: String) -> Self {
        self.context = Some(context);
        self
    }
}

// Implement ordering for priority queue (higher priority and score = higher in queue)
impl PartialEq for ExplorationTarget {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && (self.score - other.score).abs() < 0.001
    }
}

impl Eq for ExplorationTarget {}

impl PartialOrd for ExplorationTarget {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ExplorationTarget {
    fn cmp(&self, other: &Self) -> Ordering {
        // First compare by priority
        match self.priority.cmp(&other.priority) {
            Ordering::Equal => {
                // If priority is equal, compare by score
                // Reverse ordering for f32 comparison (higher score = higher priority)
                other.score.partial_cmp(&self.score).unwrap_or(Ordering::Equal)
            }
            other => other,
        }
    }
}

/// Queue of exploration targets
pub struct ExplorationQueue {
    /// Priority queue (BinaryHeap)
    queue: BinaryHeap<ExplorationTarget>,

    /// Maximum queue size
    max_size: usize,

    /// Total targets added (including dropped)
    total_added: usize,

    /// Total targets explored
    total_explored: usize,
}

impl ExplorationQueue {
    /// Create new exploration queue
    pub fn new(max_size: usize) -> Self {
        Self {
            queue: BinaryHeap::with_capacity(max_size),
            max_size,
            total_added: 0,
            total_explored: 0,
        }
    }

    /// Add target to queue
    pub fn push(&mut self, target: ExplorationTarget) {
        self.total_added += 1;

        // If at capacity, drop lowest priority target
        if self.queue.len() >= self.max_size {
            // Convert to vec, sort, drop lowest, convert back
            let mut targets: Vec<_> = self.queue.drain().collect();
            targets.sort();
            targets.reverse(); // Highest priority first

            // Keep only max_size - 1 to make room
            targets.truncate(self.max_size - 1);

            self.queue = BinaryHeap::from(targets);
        }

        self.queue.push(target);
    }

    /// Pop highest priority target
    pub fn pop(&mut self) -> Option<ExplorationTarget> {
        let target = self.queue.pop();
        if target.is_some() {
            self.total_explored += 1;
        }
        target
    }

    /// Peek at highest priority target without removing
    pub fn peek(&self) -> Option<&ExplorationTarget> {
        self.queue.peek()
    }

    /// Get current queue size
    pub fn len(&self) -> usize {
        self.queue.len()
    }

    /// Check if queue is empty
    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }

    /// Get statistics
    pub fn stats(&self) -> ExplorationStats {
        ExplorationStats {
            queue_size: self.queue.len(),
            total_added: self.total_added,
            total_explored: self.total_explored,
        }
    }

    /// Clear queue
    pub fn clear(&mut self) {
        self.queue.clear();
    }
}

impl Default for ExplorationQueue {
    fn default() -> Self {
        Self::new(100) // Default max size
    }
}

/// Statistics for exploration queue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplorationStats {
    pub queue_size: usize,
    pub total_added: usize,
    pub total_explored: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_ordering() {
        let t1 = ExplorationTarget::new([0.0; 8], 0.5, ExplorationReason::HighUncertainty);
        let t2 = ExplorationTarget::new([1.0; 8], 0.8, ExplorationReason::Novel);

        // t2 has higher score, should be > t1
        assert!(t2 > t1);
    }

    #[test]
    fn test_target_priority() {
        let t1 = ExplorationTarget::with_priority(
            [0.0; 8],
            0.5,
            ExplorationReason::HighUncertainty,
            ExplorationPriority::Low,
        );
        let t2 = ExplorationTarget::with_priority(
            [1.0; 8],
            0.3,
            ExplorationReason::Novel,
            ExplorationPriority::High,
        );

        // t2 has higher priority, should be > t1 despite lower score
        assert!(t2 > t1);
    }

    #[test]
    fn test_queue_push_pop() {
        let mut queue = ExplorationQueue::new(10);

        let t1 = ExplorationTarget::new([0.0; 8], 0.5, ExplorationReason::HighUncertainty);
        let t2 = ExplorationTarget::new([1.0; 8], 0.8, ExplorationReason::Novel);

        queue.push(t1);
        queue.push(t2);

        assert_eq!(queue.len(), 2);

        // Pop should return highest score (t2)
        let popped = queue.pop().unwrap();
        assert!((popped.score - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_queue_capacity() {
        let mut queue = ExplorationQueue::new(3);

        // Add 5 targets
        for i in 0..5 {
            let target = ExplorationTarget::new([i as f64; 8], i as f32 / 10.0, ExplorationReason::Novel);
            queue.push(target);
        }

        // Should only keep 3 highest priority
        assert_eq!(queue.len(), 3);
        assert_eq!(queue.stats().total_added, 5);
    }

    #[test]
    fn test_queue_peek() {
        let mut queue = ExplorationQueue::new(10);

        let target = ExplorationTarget::new([0.0; 8], 0.9, ExplorationReason::HighSurprise);
        queue.push(target);

        // Peek should not remove
        let peeked = queue.peek().unwrap();
        assert!((peeked.score - 0.9).abs() < 0.001);
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn test_empty_queue() {
        let mut queue = ExplorationQueue::new(10);

        assert!(queue.is_empty());
        assert!(queue.pop().is_none());
        assert!(queue.peek().is_none());
    }

    #[test]
    fn test_auto_priority_assignment() {
        let t_low = ExplorationTarget::new([0.0; 8], 0.3, ExplorationReason::Novel);
        let t_medium = ExplorationTarget::new([0.0; 8], 0.5, ExplorationReason::Novel);
        let t_high = ExplorationTarget::new([0.0; 8], 0.7, ExplorationReason::Novel);
        let t_critical = ExplorationTarget::new([0.0; 8], 0.9, ExplorationReason::Novel);

        assert_eq!(t_low.priority, ExplorationPriority::Low);
        assert_eq!(t_medium.priority, ExplorationPriority::Medium);
        assert_eq!(t_high.priority, ExplorationPriority::High);
        assert_eq!(t_critical.priority, ExplorationPriority::Critical);
    }
}
