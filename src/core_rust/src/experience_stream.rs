//! ExperienceStream v2.1 - Event-based memory system
//!
//! This module provides a unified event stream for the entire NeuroGraph OS architecture.
//! All events, actions, and state changes are recorded in a single stream and made available
//! to Appraisers, IntuitionEngine, and other components.
//!
//! Key features:
//! - 128-byte cache-friendly ExperienceEvent structure
//! - Circular buffer for hot storage (1M events = 128 MB)
//! - Pub-sub system via tokio::broadcast
//! - Separate reward components for each appraiser (no race conditions)
//! - Optional cold storage for long-term persistence

use std::sync::Arc;
use std::collections::HashMap;
use parking_lot::RwLock;
use tokio::sync::broadcast;
use serde_json::Value;

/// ExperienceEvent - unified structure for all events (128 bytes)
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct ExperienceEvent {
    /// Unique event identifier
    pub event_id: u128, // 16 bytes (using u128 as simple UUID)

    /// Timestamp (Unix epoch microseconds)
    pub timestamp: u64, // 8 bytes

    /// Episode ID (for grouping related events)
    pub episode_id: u64, // 8 bytes

    /// Step number within episode
    pub step_number: u32, // 4 bytes

    /// Event type discriminator
    pub event_type: u16, // 2 bytes

    /// Event flags
    pub flags: u16, // 2 bytes

    /// State vector (8D representation)
    pub state: [f32; 8], // 32 bytes

    /// Action vector (8D representation)
    pub action: [f32; 8], // 32 bytes

    /// Reward components (each appraiser writes to its own slot)
    pub reward_homeostasis: f32, // 4 bytes
    pub reward_curiosity: f32,   // 4 bytes
    pub reward_efficiency: f32,  // 4 bytes
    pub reward_goal: f32,        // 4 bytes

    /// ADNA version hash (first 4 bytes)
    pub adna_version_hash: u32, // 4 bytes

    /// Sequence number in buffer (for appraisers to update rewards)
    pub sequence_number: u32, // 4 bytes
}

// Compile-time size assertion
const _: () = assert!(std::mem::size_of::<ExperienceEvent>() == 128);

impl ExperienceEvent {
    /// Calculate total reward from all components
    pub fn total_reward(&self) -> f32 {
        self.reward_homeostasis
            + self.reward_curiosity
            + self.reward_efficiency
            + self.reward_goal
    }

    /// Check if event has been fully appraised by all 4 appraisers
    pub fn is_fully_appraised(&self) -> bool {
        self.flags & EventFlags::FULLY_APPRAISED != 0
    }
}

impl Default for ExperienceEvent {
    fn default() -> Self {
        Self {
            event_id: 0,
            timestamp: 0,
            episode_id: 0,
            step_number: 0,
            event_type: 0,
            flags: 0,
            state: [0.0; 8],
            action: [0.0; 8],
            reward_homeostasis: 0.0,
            reward_curiosity: 0.0,
            reward_efficiency: 0.0,
            reward_goal: 0.0,
            adna_version_hash: 0,
            sequence_number: 0,
        }
    }
}

/// Event type discriminator
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    // === System Events (0x00xx) ===
    SystemStartup = 0x0001,
    SystemShutdown = 0x0002,
    CDNAUpdated = 0x0010,
    ADNAUpdated = 0x0011,

    // === Token/Connection Events (0x01xx) ===
    TokenCreated = 0x0100,
    TokenDeleted = 0x0101,
    TokenActivated = 0x0102,
    ConnectionCreated = 0x0110,
    ConnectionDeleted = 0x0111,
    ConnectionActivated = 0x0112,

    // === Action Events (0x02xx) ===
    ActionStarted = 0x0200,
    ActionCompleted = 0x0201,
    ActionFailed = 0x0202,

    // === Appraisal Events (0x03xx) ===
    HomeostasisReward = 0x0300,
    CuriosityReward = 0x0301,
    EfficiencyReward = 0x0302,
    GoalReward = 0x0303,

    // === Learning Events (0x04xx) ===
    ProposalGenerated = 0x0400,
    ProposalAccepted = 0x0401,
    ProposalRejected = 0x0402,

    // === Custom Events (0xF0xx) ===
    CustomUserEvent = 0xF000,
}

impl From<u16> for EventType {
    fn from(val: u16) -> Self {
        match val {
            0x0001 => EventType::SystemStartup,
            0x0002 => EventType::SystemShutdown,
            0x0010 => EventType::CDNAUpdated,
            0x0011 => EventType::ADNAUpdated,
            0x0100 => EventType::TokenCreated,
            0x0101 => EventType::TokenDeleted,
            0x0102 => EventType::TokenActivated,
            0x0110 => EventType::ConnectionCreated,
            0x0111 => EventType::ConnectionDeleted,
            0x0112 => EventType::ConnectionActivated,
            0x0200 => EventType::ActionStarted,
            0x0201 => EventType::ActionCompleted,
            0x0202 => EventType::ActionFailed,
            0x0300 => EventType::HomeostasisReward,
            0x0301 => EventType::CuriosityReward,
            0x0302 => EventType::EfficiencyReward,
            0x0303 => EventType::GoalReward,
            0x0400 => EventType::ProposalGenerated,
            0x0401 => EventType::ProposalAccepted,
            0x0402 => EventType::ProposalRejected,
            _ => EventType::CustomUserEvent,
        }
    }
}

/// Event flags
pub struct EventFlags;

impl EventFlags {
    /// Event requires immediate processing
    pub const URGENT: u16 = 0x0001;

    /// Event is part of a trajectory
    pub const TRAJECTORY: u16 = 0x0002;

    /// Event contains an error
    pub const ERROR: u16 = 0x0004;

    /// Event should be persisted to disk
    pub const PERSIST: u16 = 0x0008;

    /// Event has been processed by all Appraisers
    pub const FULLY_APPRAISED: u16 = 0x0010;

    /// Reserved flags
    pub const _RESERVED: u16 = 0xFFE0;
}

/// Appraiser type for identifying which appraiser is updating rewards
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum AppraiserType {
    Homeostasis = 0,
    Curiosity = 1,
    Efficiency = 2,
    Goal = 3,
}

/// Circular buffer for hot storage of events
pub struct HotBuffer {
    /// Fixed-size buffer of events
    events: Box<[ExperienceEvent]>,

    /// Capacity (usually 1M events)
    capacity: usize,

    /// Write position (wraps around)
    write_pos: Arc<RwLock<usize>>,

    /// Total events written (never wraps)
    total_written: Arc<RwLock<u64>>,
}

impl HotBuffer {
    /// Create new buffer with given capacity
    pub fn new(capacity: usize) -> Self {
        let events = vec![ExperienceEvent::default(); capacity].into_boxed_slice();

        Self {
            events,
            capacity,
            write_pos: Arc::new(RwLock::new(0)),
            total_written: Arc::new(RwLock::new(0)),
        }
    }

    /// Write event to buffer (lock-free read, single writer)
    ///
    /// Returns the global sequence number of the written event
    pub fn write(&self, event: ExperienceEvent) -> u64 {
        let mut write_pos = self.write_pos.write();
        let mut total = self.total_written.write();

        // Write to circular buffer
        let idx = *write_pos % self.capacity;
        unsafe {
            let ptr = self.events.as_ptr() as *mut ExperienceEvent;
            ptr.add(idx).write(event);
        }

        // Update counters
        *write_pos = (*write_pos + 1) % self.capacity;
        *total += 1;

        *total // Return global sequence number
    }

    /// Read event by absolute sequence number
    ///
    /// Returns None if event has been overwritten or doesn't exist yet
    pub fn read(&self, seq: u64) -> Option<ExperienceEvent> {
        let total = *self.total_written.read();

        // Check if event is still in buffer
        if seq + (self.capacity as u64) < total {
            return None; // Too old, overwritten
        }
        if seq >= total {
            return None; // Future event
        }

        let idx = (seq as usize) % self.capacity;
        Some(self.events[idx])
    }

    /// Query range of events [start, end)
    pub fn query_range(&self, start: u64, end: u64) -> Vec<ExperienceEvent> {
        (start..end).filter_map(|seq| self.read(seq)).collect()
    }

    /// Get current size (number of events in buffer)
    pub fn size(&self) -> usize {
        let total = *self.total_written.read();
        std::cmp::min(total as usize, self.capacity)
    }

    /// Get total events written (including overwritten)
    pub fn total_written(&self) -> u64 {
        *self.total_written.read()
    }

    /// Update specific appraiser's reward component
    ///
    /// # Safety
    /// This is safe because each appraiser writes to its own dedicated slot,
    /// so there are no race conditions between appraisers.
    pub fn set_appraiser_reward(
        &self,
        seq: u64,
        appraiser: AppraiserType,
        reward: f32,
    ) -> Result<(), &'static str> {
        let total = *self.total_written.read();

        // Check if event still exists
        if seq + (self.capacity as u64) < total {
            return Err("Event too old, already overwritten");
        }
        if seq >= total {
            return Err("Event doesn't exist yet");
        }

        let idx = (seq as usize) % self.capacity;

        // Each appraiser writes to its own slot - no contention
        unsafe {
            let ptr = self.events.as_ptr() as *mut ExperienceEvent;
            let event = &mut *ptr.add(idx);

            match appraiser {
                AppraiserType::Homeostasis => event.reward_homeostasis = reward,
                AppraiserType::Curiosity => event.reward_curiosity = reward,
                AppraiserType::Efficiency => event.reward_efficiency = reward,
                AppraiserType::Goal => event.reward_goal = reward,
            }
        }

        Ok(())
    }

    /// Mark event as fully appraised (all 4 appraisers completed)
    pub fn mark_fully_appraised(&self, seq: u64) -> Result<(), &'static str> {
        let total = *self.total_written.read();

        if seq + (self.capacity as u64) < total {
            return Err("Event too old, already overwritten");
        }
        if seq >= total {
            return Err("Event doesn't exist yet");
        }

        let idx = (seq as usize) % self.capacity;

        unsafe {
            let ptr = self.events.as_ptr() as *mut ExperienceEvent;
            let event = &mut *ptr.add(idx);
            event.flags |= EventFlags::FULLY_APPRAISED;
        }

        Ok(())
    }
}

// ============================================================================
// ExperienceStream - Main API with Pub-Sub
// ============================================================================

/// Metadata for action events (intent_type, executor_id, parameters)
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActionMetadata {
    pub intent_type: String,
    pub executor_id: String,
    pub parameters: Value,
}

/// Main ExperienceStream structure with pub-sub capabilities
pub struct ExperienceStream {
    /// Hot buffer for storage
    buffer: Arc<HotBuffer>,

    /// Broadcast channel for real-time distribution
    tx: broadcast::Sender<ExperienceEvent>,

    /// Metadata store for action events (event_id → metadata)
    /// Separate from hot buffer to maintain cache-friendly 128-byte events
    metadata: Arc<RwLock<HashMap<u128, ActionMetadata>>>,
}

impl ExperienceStream {
    /// Create new ExperienceStream
    ///
    /// # Arguments
    /// * `capacity` - Hot buffer capacity (e.g., 1_000_000 for 128MB)
    /// * `channel_size` - Broadcast channel size (e.g., 1000)
    pub fn new(capacity: usize, channel_size: usize) -> Self {
        let buffer = Arc::new(HotBuffer::new(capacity));
        let (tx, _rx) = broadcast::channel(channel_size);
        let metadata = Arc::new(RwLock::new(HashMap::new()));

        Self { buffer, tx, metadata }
    }

    /// Write event to stream and broadcast to subscribers
    ///
    /// Returns the global sequence number of the written event
    pub fn write_event(&self, mut event: ExperienceEvent) -> Result<u64, &'static str> {
        // 1. Write to hot buffer
        let seq = self.buffer.write(event);

        // 2. Set sequence number for broadcast subscribers
        event.sequence_number = (seq - 1) as u32; // seq is 1-based, convert to 0-based u32

        // 3. Broadcast to subscribers (ignore error if no subscribers)
        let _ = self.tx.send(event);

        Ok(seq)
    }

    /// Get event by sequence number
    pub fn get_event(&self, seq: u64) -> Option<ExperienceEvent> {
        self.buffer.read(seq)
    }

    /// Query range of events [start, end)
    pub fn query_range(&self, start: u64, end: u64) -> Vec<ExperienceEvent> {
        self.buffer.query_range(start, end)
    }

    /// Subscribe to real-time events
    ///
    /// Returns a receiver that will get all future events
    pub fn subscribe(&self) -> broadcast::Receiver<ExperienceEvent> {
        self.tx.subscribe()
    }

    /// Get current stream size
    pub fn size(&self) -> usize {
        self.buffer.size()
    }

    /// Get total events written
    pub fn total_written(&self) -> u64 {
        self.buffer.total_written()
    }

    /// Update specific appraiser's reward component
    pub fn set_appraiser_reward(
        &self,
        seq: u64,
        appraiser: AppraiserType,
        reward: f32,
    ) -> Result<(), &'static str> {
        self.buffer.set_appraiser_reward(seq, appraiser, reward)
    }

    /// Mark event as fully appraised
    pub fn mark_fully_appraised(&self, seq: u64) -> Result<(), &'static str> {
        self.buffer.mark_fully_appraised(seq)
    }

    /// Get reference to underlying buffer (for advanced use)
    pub fn buffer(&self) -> &Arc<HotBuffer> {
        &self.buffer
    }

    /// Write event with associated action metadata
    ///
    /// This is a convenience method for ActionController to write events
    /// with rich metadata (intent_type, executor_id, parameters).
    pub fn write_event_with_metadata(
        &self,
        event: ExperienceEvent,
        metadata: ActionMetadata,
    ) -> Result<u64, &'static str> {
        // Store metadata before writing event
        self.metadata.write().insert(event.event_id, metadata);

        // Write event to stream
        self.write_event(event)
    }

    /// Get action metadata for an event by event_id
    ///
    /// Returns None if no metadata exists for this event.
    pub fn get_metadata(&self, event_id: u128) -> Option<ActionMetadata> {
        self.metadata.read().get(&event_id).cloned()
    }

    /// Get event with its metadata by sequence number
    ///
    /// Returns (event, Option<metadata>) tuple.
    pub fn get_event_with_metadata(&self, seq: u64) -> Option<(ExperienceEvent, Option<ActionMetadata>)> {
        if let Some(event) = self.get_event(seq) {
            let metadata = self.get_metadata(event.event_id);
            Some((event, metadata))
        } else {
            None
        }
    }
}

// ============================================================================
// Traits for Writer/Reader abstraction
// ============================================================================

/// Trait for writing events to the stream
pub trait ExperienceWriter: Send + Sync {
    /// Write new event and return sequence number
    fn write_event(&self, event: ExperienceEvent) -> Result<u64, &'static str>;

    /// Write event with associated action metadata (optional, default implementation does nothing)
    fn write_event_with_metadata(
        &self,
        event: ExperienceEvent,
        _metadata: ActionMetadata,
    ) -> Result<u64, &'static str> {
        // Default implementation ignores metadata (for backwards compatibility)
        self.write_event(event)
    }

    /// Write multiple events
    fn write_batch(&self, events: Vec<ExperienceEvent>) -> Result<Vec<u64>, &'static str> {
        events.into_iter().map(|e| self.write_event(e)).collect()
    }

    /// Update specific appraiser's reward component
    fn set_appraiser_reward(
        &self,
        seq: u64,
        appraiser: AppraiserType,
        reward: f32,
    ) -> Result<(), &'static str>;

    /// Mark event as fully appraised
    fn mark_fully_appraised(&self, seq: u64) -> Result<(), &'static str>;
}

/// Trait for reading events from the stream
pub trait ExperienceReader: Send + Sync {
    /// Get single event by sequence number
    fn get_event(&self, seq: u64) -> Option<ExperienceEvent>;

    /// Query range [start, end)
    fn query_range(&self, start: u64, end: u64) -> Vec<ExperienceEvent>;

    /// Subscribe to real-time events
    fn subscribe(&self) -> broadcast::Receiver<ExperienceEvent>;

    /// Get current stream size
    fn size(&self) -> usize;

    /// Get total events written
    fn total_written(&self) -> u64;
}

impl ExperienceWriter for ExperienceStream {
    fn write_event(&self, event: ExperienceEvent) -> Result<u64, &'static str> {
        self.write_event(event)
    }

    fn set_appraiser_reward(
        &self,
        seq: u64,
        appraiser: AppraiserType,
        reward: f32,
    ) -> Result<(), &'static str> {
        self.set_appraiser_reward(seq, appraiser, reward)
    }

    fn mark_fully_appraised(&self, seq: u64) -> Result<(), &'static str> {
        self.mark_fully_appraised(seq)
    }
}

impl ExperienceReader for ExperienceStream {
    fn get_event(&self, seq: u64) -> Option<ExperienceEvent> {
        self.get_event(seq)
    }

    fn query_range(&self, start: u64, end: u64) -> Vec<ExperienceEvent> {
        self.query_range(start, end)
    }

    fn subscribe(&self) -> broadcast::Receiver<ExperienceEvent> {
        self.subscribe()
    }

    fn size(&self) -> usize {
        self.size()
    }

    fn total_written(&self) -> u64 {
        self.total_written()
    }
}

// ============================================================================
// Sampling Strategy for IntuitionEngine
// ============================================================================

/// Sampling strategy for selecting "interesting" experience events
#[derive(Debug, Clone)]
pub enum SamplingStrategy {
    /// Random uniform sampling
    Uniform,

    /// Prioritized by absolute total reward
    PrioritizedByReward {
        /// Probability exponent (higher = more biased toward high |reward|)
        alpha: f64,
    },

    /// Recency-weighted (prefer recent experience)
    RecencyWeighted {
        /// Decay factor [0.0, 1.0]
        decay: f64,
    },

    /// Mixed strategy
    Mixed {
        reward_weight: f64,
        recency_weight: f64,
    },
}

/// Batch of sampled experience events
pub struct ExperienceBatch {
    pub events: Vec<ExperienceEvent>,
    pub sampled_at: std::time::SystemTime,
}

impl ExperienceStream {
    /// Sample batch of events using specified strategy
    ///
    /// This is used by IntuitionEngine to select "interesting" events for analysis.
    pub fn sample_batch(&self, size: usize, strategy: SamplingStrategy) -> ExperienceBatch {
        use rand::Rng;
        use rand::seq::SliceRandom;

        let total = self.total_written();
        let available = self.size();

        if available == 0 {
            return ExperienceBatch {
                events: Vec::new(),
                sampled_at: std::time::SystemTime::now(),
            };
        }

        // Get start sequence for available events
        let start_seq = if total > available as u64 {
            total - available as u64
        } else {
            0
        };

        // Collect all available events
        let all_events: Vec<_> = (start_seq..total)
            .filter_map(|seq| self.get_event(seq))
            .collect();

        if all_events.is_empty() {
            return ExperienceBatch {
                events: Vec::new(),
                sampled_at: std::time::SystemTime::now(),
            };
        }

        let sample_size = std::cmp::min(size, all_events.len());
        let mut rng = rand::thread_rng();

        let sampled_events = match strategy {
            SamplingStrategy::Uniform => {
                // Simple uniform random sampling
                let mut events = all_events.clone();
                events.shuffle(&mut rng);
                events.into_iter().take(sample_size).collect()
            }

            SamplingStrategy::PrioritizedByReward { alpha } => {
                // Prioritized sampling based on absolute total reward
                let mut indices_with_priority: Vec<_> = all_events
                    .iter()
                    .enumerate()
                    .map(|(i, event)| {
                        let total_reward = event.total_reward().abs();
                        let priority = total_reward.powf(alpha as f32);
                        (i, priority)
                    })
                    .collect();

                // Calculate total priority
                let total_priority: f32 = indices_with_priority
                    .iter()
                    .map(|(_, p)| p)
                    .sum();

                if total_priority == 0.0 {
                    // Fall back to uniform if all rewards are zero
                    let mut events = all_events.clone();
                    events.shuffle(&mut rng);
                    events.into_iter().take(sample_size).collect()
                } else {
                    // Sample proportional to priority
                    let mut selected = Vec::with_capacity(sample_size);
                    let mut remaining = indices_with_priority;

                    for _ in 0..sample_size {
                        if remaining.is_empty() {
                            break;
                        }

                        // Calculate cumulative probabilities
                        let current_total: f32 = remaining.iter().map(|(_, p)| p).sum();
                        let mut rand_val = rng.gen::<f32>() * current_total;

                        // Select based on probability
                        let mut selected_idx = 0;
                        for (j, (_, priority)) in remaining.iter().enumerate() {
                            rand_val -= priority;
                            if rand_val <= 0.0 {
                                selected_idx = j;
                                break;
                            }
                        }

                        let (event_idx, _) = remaining.remove(selected_idx);
                        selected.push(all_events[event_idx]);
                    }

                    selected
                }
            }

            SamplingStrategy::RecencyWeighted { decay } => {
                // Weight events by recency
                let mut indices_with_weight: Vec<_> = all_events
                    .iter()
                    .enumerate()
                    .map(|(i, _event)| {
                        // More recent = higher index = higher weight
                        let recency_factor = i as f64 / all_events.len().max(1) as f64;
                        let weight = decay.powf(1.0 - recency_factor);
                        (i, weight)
                    })
                    .collect();

                // Similar weighted sampling as PrioritizedByReward
                let total_weight: f64 = indices_with_weight.iter().map(|(_, w)| w).sum();

                let mut selected = Vec::with_capacity(sample_size);
                let mut remaining = indices_with_weight;

                for _ in 0..sample_size {
                    if remaining.is_empty() {
                        break;
                    }

                    let current_total: f64 = remaining.iter().map(|(_, w)| w).sum();
                    let mut rand_val = rng.gen::<f64>() * current_total;

                    let mut selected_idx = 0;
                    for (j, (_, weight)) in remaining.iter().enumerate() {
                        rand_val -= weight;
                        if rand_val <= 0.0 {
                            selected_idx = j;
                            break;
                        }
                    }

                    let (event_idx, _) = remaining.remove(selected_idx);
                    selected.push(all_events[event_idx]);
                }

                selected
            }

            SamplingStrategy::Mixed {
                reward_weight,
                recency_weight,
            } => {
                // Combine reward and recency
                let mut indices_with_weight: Vec<_> = all_events
                    .iter()
                    .enumerate()
                    .map(|(i, event)| {
                        let reward_factor = event.total_reward().abs() as f64;
                        let recency_factor = i as f64 / all_events.len().max(1) as f64;
                        let combined_weight =
                            reward_factor * reward_weight + recency_factor * recency_weight;
                        (i, combined_weight.max(0.0))
                    })
                    .collect();

                let total_weight: f64 = indices_with_weight.iter().map(|(_, w)| w).sum();

                if total_weight == 0.0 {
                    let mut events = all_events.clone();
                    events.shuffle(&mut rng);
                    events.into_iter().take(sample_size).collect()
                } else {
                    let mut selected = Vec::with_capacity(sample_size);
                    let mut remaining = indices_with_weight;

                    for _ in 0..sample_size {
                        if remaining.is_empty() {
                            break;
                        }

                        let current_total: f64 = remaining.iter().map(|(_, w)| w).sum();
                        let mut rand_val = rng.gen::<f64>() * current_total;

                        let mut selected_idx = 0;
                        for (j, (_, weight)) in remaining.iter().enumerate() {
                            rand_val -= weight;
                            if rand_val <= 0.0 {
                                selected_idx = j;
                                break;
                            }
                        }

                        let (event_idx, _) = remaining.remove(selected_idx);
                        selected.push(all_events[event_idx]);
                    }

                    selected
                }
            }
        };

        ExperienceBatch {
            events: sampled_events,
            sampled_at: std::time::SystemTime::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_size() {
        assert_eq!(std::mem::size_of::<ExperienceEvent>(), 128);
    }

    #[test]
    fn test_event_default() {
        let event = ExperienceEvent::default();
        assert_eq!(event.total_reward(), 0.0);
        assert!(!event.is_fully_appraised());
    }

    #[test]
    fn test_event_total_reward() {
        let mut event = ExperienceEvent::default();
        event.reward_homeostasis = 1.0;
        event.reward_curiosity = 2.0;
        event.reward_efficiency = -0.5;
        event.reward_goal = 1.5;
        assert_eq!(event.total_reward(), 4.0);
    }

    #[test]
    fn test_hot_buffer_write_read() {
        let buffer = HotBuffer::new(10);
        let event = ExperienceEvent::default();

        let seq = buffer.write(event);
        assert_eq!(seq, 1);

        let read_event = buffer.read(0);
        assert!(read_event.is_some());
    }

    #[test]
    fn test_hot_buffer_circular_wrap() {
        let buffer = HotBuffer::new(10);

        // Write 15 events
        for i in 0..15 {
            let mut event = ExperienceEvent::default();
            event.step_number = i;
            buffer.write(event);
        }

        // Events 0-4 should be overwritten
        assert!(buffer.read(0).is_none());
        assert!(buffer.read(4).is_none());

        // Events 5-14 should still exist
        let event = buffer.read(5).unwrap();
        assert_eq!(event.step_number, 5);

        let event = buffer.read(14).unwrap();
        assert_eq!(event.step_number, 14);
    }

    #[test]
    fn test_appraiser_reward_update() {
        let buffer = HotBuffer::new(10);
        let event = ExperienceEvent::default();
        let seq = buffer.write(event);

        // Update rewards from different appraisers
        buffer
            .set_appraiser_reward(seq - 1, AppraiserType::Homeostasis, 1.0)
            .unwrap();
        buffer
            .set_appraiser_reward(seq - 1, AppraiserType::Curiosity, 2.0)
            .unwrap();
        buffer
            .set_appraiser_reward(seq - 1, AppraiserType::Efficiency, -0.5)
            .unwrap();
        buffer
            .set_appraiser_reward(seq - 1, AppraiserType::Goal, 1.5)
            .unwrap();

        // Read back and check
        let updated = buffer.read(seq - 1).unwrap();
        assert_eq!(updated.reward_homeostasis, 1.0);
        assert_eq!(updated.reward_curiosity, 2.0);
        assert_eq!(updated.reward_efficiency, -0.5);
        assert_eq!(updated.reward_goal, 1.5);
        assert_eq!(updated.total_reward(), 4.0);
    }

    #[test]
    fn test_mark_fully_appraised() {
        let buffer = HotBuffer::new(10);
        let event = ExperienceEvent::default();
        let seq = buffer.write(event);

        assert!(!buffer.read(seq - 1).unwrap().is_fully_appraised());

        buffer.mark_fully_appraised(seq - 1).unwrap();

        assert!(buffer.read(seq - 1).unwrap().is_fully_appraised());
    }

    #[test]
    fn test_experience_stream_creation() {
        let stream = ExperienceStream::new(1000, 100);
        assert_eq!(stream.size(), 0);
        assert_eq!(stream.total_written(), 0);
    }

    #[test]
    fn test_experience_stream_write() {
        let stream = ExperienceStream::new(1000, 100);
        let event = ExperienceEvent::default();

        let seq = stream.write_event(event).unwrap();
        assert_eq!(seq, 1);
        assert_eq!(stream.size(), 1);

        let read_event = stream.get_event(0).unwrap();
        assert_eq!(read_event.event_id, event.event_id);
    }

    #[tokio::test]
    async fn test_pubsub_broadcast() {
        let stream = Arc::new(ExperienceStream::new(1000, 100));
        let mut rx = stream.subscribe();

        // Write event
        let mut event = ExperienceEvent::default();
        event.step_number = 42;
        stream.write_event(event).unwrap();

        // Receive via broadcast
        let received = rx.recv().await.unwrap();
        assert_eq!(received.step_number, 42);
    }

    #[tokio::test]
    async fn test_multiple_subscribers() {
        let stream = Arc::new(ExperienceStream::new(1000, 100));
        let mut rx1 = stream.subscribe();
        let mut rx2 = stream.subscribe();

        // Write event
        let mut event = ExperienceEvent::default();
        event.step_number = 99;
        stream.write_event(event).unwrap();

        // Both subscribers should receive
        let received1 = rx1.recv().await.unwrap();
        let received2 = rx2.recv().await.unwrap();

        assert_eq!(received1.step_number, 99);
        assert_eq!(received2.step_number, 99);
    }

    #[tokio::test]
    async fn test_appraiser_integration() {
        let stream = Arc::new(ExperienceStream::new(1000, 100));

        // Simulate appraiser
        let stream_clone = Arc::clone(&stream);
        tokio::spawn(async move {
            let mut rx = stream_clone.subscribe();
            if let Ok(_event) = rx.recv().await {
                // Get seq (in real implementation, this would be passed with event)
                let seq = stream_clone.total_written() - 1;

                // Update reward
                stream_clone
                    .set_appraiser_reward(seq, AppraiserType::Homeostasis, 1.5)
                    .unwrap();
            }
        });

        // Write event
        let event = ExperienceEvent::default();
        let seq = stream.write_event(event).unwrap();

        // Wait for appraiser to process
        tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;

        // Check reward was updated
        let updated = stream.get_event(seq - 1).unwrap();
        assert_eq!(updated.reward_homeostasis, 1.5);
    }

    #[test]
    fn test_traits() {
        let stream = ExperienceStream::new(1000, 100);

        // Test ExperienceWriter trait
        let writer: &dyn ExperienceWriter = &stream;
        let event = ExperienceEvent::default();
        let seq = writer.write_event(event).unwrap();
        assert_eq!(seq, 1);

        // Test ExperienceReader trait
        let reader: &dyn ExperienceReader = &stream;
        assert_eq!(reader.size(), 1);
        assert!(reader.get_event(0).is_some());
    }

    #[test]
    fn test_sampling_uniform() {
        let stream = ExperienceStream::new(1000, 100);

        // Write some events
        for i in 0..50 {
            let mut event = ExperienceEvent::default();
            event.reward_homeostasis = i as f32;
            stream.write_event(event).unwrap();
        }

        // Sample 10 events uniformly
        let batch = stream.sample_batch(10, SamplingStrategy::Uniform);
        assert_eq!(batch.events.len(), 10);
    }

    #[test]
    fn test_sampling_prioritized() {
        let stream = ExperienceStream::new(1000, 100);

        // Write events with different rewards
        for i in 0..50 {
            let mut event = ExperienceEvent::default();
            // Some high rewards, some low
            event.reward_homeostasis = if i % 10 == 0 { 10.0 } else { 0.1 };
            stream.write_event(event).unwrap();
        }

        // Sample prioritized by reward
        let batch = stream.sample_batch(
            10,
            SamplingStrategy::PrioritizedByReward { alpha: 1.0 }
        );
        assert_eq!(batch.events.len(), 10);

        // Most events should have higher rewards (statistical test with tolerance)
        let avg_reward: f32 = batch.events.iter()
            .map(|e| e.reward_homeostasis)
            .sum::<f32>() / batch.events.len() as f32;

        // Should be > 1.0 due to prioritization (some high-reward events selected)
        assert!(avg_reward > 0.5);
    }
}