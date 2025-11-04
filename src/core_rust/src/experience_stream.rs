/// ExperienceStream v2.0 - Event Memory System for NeuroGraph OS
///
/// This module provides the fundamental memory infrastructure for the KEY architecture:
/// - ExperienceEvent: 128-byte event structure
/// - HotBuffer: Circular buffer for in-memory storage (1M events)
/// - ExperienceStream: Pub-sub event distribution
/// - Sampling strategies for batch processing
use parking_lot::RwLock;
use std::sync::Arc;
use tokio::sync::broadcast;
use uuid::Uuid;

// ==================== EVENT STRUCTURE ====================

/// ExperienceEvent - unified 128-byte structure for all events
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct ExperienceEvent {
    /// Unique event identifier (UUID v4)
    pub event_id: Uuid, // 16 bytes

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
    /// Can contain: [L1..L8] coordinates, system metrics, etc.
    pub state: [f32; 8], // 32 bytes

    /// Action vector (8D representation)
    /// For action events: action parameters
    /// For other events: may be unused
    pub action: [f32; 8], // 32 bytes

    /// Accumulated reward (updated by Appraisers)
    pub reward: f32, // 4 bytes

    /// ADNA version hash (first 4 bytes)
    pub adna_version_hash: u32, // 4 bytes

    /// Reserved for future use
    _reserved: [u8; 16], // 16 bytes
}

// Compile-time assertion: ExperienceEvent must be exactly 128 bytes
const _: () = assert!(std::mem::size_of::<ExperienceEvent>() == 128);

impl Default for ExperienceEvent {
    fn default() -> Self {
        Self {
            event_id: Uuid::nil(),
            timestamp: 0,
            episode_id: 0,
            step_number: 0,
            event_type: 0,
            flags: 0,
            state: [0.0; 8],
            action: [0.0; 8],
            reward: 0.0,
            adna_version_hash: 0,
            _reserved: [0; 16],
        }
    }
}

impl ExperienceEvent {
    /// Create new event with current timestamp and unique ID
    pub fn new(event_type: EventType) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            timestamp: current_timestamp_micros(),
            event_type: event_type as u16,
            ..Default::default()
        }
    }

    /// Create event with specific episode and step
    pub fn with_episode(mut self, episode_id: u64, step_number: u32) -> Self {
        self.episode_id = episode_id;
        self.step_number = step_number;
        self
    }

    /// Set state vector
    pub fn with_state(mut self, state: [f32; 8]) -> Self {
        self.state = state;
        self
    }

    /// Set action vector
    pub fn with_action(mut self, action: [f32; 8]) -> Self {
        self.action = action;
        self
    }

    /// Set flags
    pub fn with_flags(mut self, flags: u16) -> Self {
        self.flags = flags;
        self
    }
}

// ==================== EVENT TYPES ====================

/// Event type discriminator
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    // System Events (0x00xx)
    SystemStartup = 0x0001,
    SystemShutdown = 0x0002,
    CDNAUpdated = 0x0010,
    ADNAUpdated = 0x0011,

    // Token/Connection Events (0x01xx)
    TokenCreated = 0x0100,
    TokenDeleted = 0x0101,
    TokenActivated = 0x0102,
    ConnectionCreated = 0x0110,
    ConnectionDeleted = 0x0111,
    ConnectionActivated = 0x0112,

    // Action Events (0x02xx)
    ActionStarted = 0x0200,
    ActionCompleted = 0x0201,
    ActionFailed = 0x0202,

    // Appraisal Events (0x03xx)
    HomeostasisReward = 0x0300,
    CuriosityReward = 0x0301,
    EfficiencyReward = 0x0302,
    GoalReward = 0x0303,

    // Learning Events (0x04xx)
    ProposalGenerated = 0x0400,
    ProposalAccepted = 0x0401,
    ProposalRejected = 0x0402,

    // Custom Events (0xF0xx)
    CustomUserEvent = 0xF000,
}

impl From<u16> for EventType {
    fn from(value: u16) -> Self {
        match value {
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

// ==================== EVENT FLAGS ====================

/// Event flags
pub struct EventFlags;

impl EventFlags {
    pub const URGENT: u16 = 0x0001;
    pub const TRAJECTORY: u16 = 0x0002;
    pub const ERROR: u16 = 0x0004;
    pub const PERSIST: u16 = 0x0008;
    pub const FULLY_APPRAISED: u16 = 0x0010;
}

// ==================== HOT BUFFER ====================

/// Circular buffer for hot storage of events
pub struct HotBuffer {
    /// Fixed-size buffer (heap-allocated)
    events: Box<[ExperienceEvent]>,

    /// Capacity (typically 1M events = 128 MB)
    capacity: usize,

    /// Write position (wraps around)
    write_pos: Arc<RwLock<usize>>,

    /// Total events written (monotonically increasing)
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

    /// Write event to buffer (single writer assumption)
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

    /// Get current buffer size
    pub fn size(&self) -> usize {
        let total = *self.total_written.read();
        std::cmp::min(total as usize, self.capacity)
    }

    /// Get total events written (including overwritten)
    pub fn total_written(&self) -> u64 {
        *self.total_written.read()
    }
}

// ==================== SAMPLING STRATEGIES ====================

/// Sampling strategy for batch processing
#[derive(Debug, Clone)]
pub enum SamplingStrategy {
    /// Uniform random sampling
    Uniform,

    /// Prioritized by |reward|
    PrioritizedReward,

    /// Recent events (last N)
    Recent(usize),

    /// Specific event types
    FilteredByType(Vec<EventType>),
}

// ==================== EXPERIENCE STREAM ====================

/// Main ExperienceStream with pub-sub capabilities
pub struct ExperienceStream {
    /// Hot buffer for event storage
    buffer: Arc<HotBuffer>,

    /// Broadcast channel for real-time distribution
    tx: broadcast::Sender<ExperienceEvent>,
}

impl ExperienceStream {
    /// Create new ExperienceStream
    ///
    /// # Arguments
    /// * `capacity` - Hot buffer capacity (e.g., 1_000_000)
    /// * `channel_size` - Broadcast channel buffer size (e.g., 1000)
    pub fn new(capacity: usize, channel_size: usize) -> Self {
        let buffer = Arc::new(HotBuffer::new(capacity));
        let (tx, _rx) = broadcast::channel(channel_size);

        Self { buffer, tx }
    }

    /// Write event to stream
    pub fn write_event(&self, event: ExperienceEvent) -> Result<u64, StreamError> {
        // 1. Write to hot buffer
        let seq = self.buffer.write(event);

        // 2. Broadcast to subscribers (non-blocking, ignore if no subscribers)
        let _ = self.tx.send(event);

        Ok(seq)
    }

    /// Get single event by sequence number
    pub fn get_event(&self, seq: u64) -> Option<ExperienceEvent> {
        self.buffer.read(seq)
    }

    /// Query range of events [start, end)
    pub fn query_range(&self, start: u64, end: u64) -> Vec<ExperienceEvent> {
        self.buffer.query_range(start, end)
    }

    /// Subscribe to real-time events
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

    /// Update reward for existing event (eventual consistency)
    pub fn update_reward(&self, seq: u64, delta: f32) -> Result<(), StreamError> {
        // Simple implementation: read-modify-write with lock
        // Note: This is not atomic, but acceptable for MVP
        let total = self.buffer.total_written();

        if seq >= total {
            return Err(StreamError::InvalidSequence(seq));
        }

        // Check if still in buffer
        if seq + (self.buffer.capacity as u64) < total {
            return Err(StreamError::EventOverwritten(seq));
        }

        let idx = (seq as usize) % self.buffer.capacity;
        unsafe {
            let ptr = self.buffer.events.as_ptr() as *mut ExperienceEvent;
            let event = &mut *ptr.add(idx);
            event.reward += delta;
        }

        Ok(())
    }

    /// Sample batch of events with given strategy
    pub fn sample_batch(&self, size: usize, strategy: SamplingStrategy) -> Vec<ExperienceEvent> {
        let total = self.buffer.size();

        match strategy {
            SamplingStrategy::Uniform => {
                // Uniform random sampling
                use rand::seq::SliceRandom;
                let indices: Vec<u64> = (0..total as u64).collect();
                indices
                    .choose_multiple(&mut rand::thread_rng(), size)
                    .filter_map(|&seq| self.buffer.read(seq))
                    .collect()
            }

            SamplingStrategy::PrioritizedReward => {
                // Sample with probability proportional to |reward|
                let all_events: Vec<_> = (0..total as u64)
                    .filter_map(|seq| self.buffer.read(seq))
                    .collect();

                // For MVP: simple top-k by |reward|
                let mut events_with_reward: Vec<_> = all_events
                    .into_iter()
                    .map(|e| (e.reward.abs(), e))
                    .collect();

                events_with_reward.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
                events_with_reward
                    .into_iter()
                    .take(size)
                    .map(|(_, e)| e)
                    .collect()
            }

            SamplingStrategy::Recent(n) => {
                // Last N events
                let start = total.saturating_sub(n) as u64;
                self.buffer.query_range(start, total as u64)
            }

            SamplingStrategy::FilteredByType(types) => {
                // Filter by event types
                (0..total as u64)
                    .filter_map(|seq| self.buffer.read(seq))
                    .filter(|e| types.contains(&EventType::from(e.event_type)))
                    .take(size)
                    .collect()
            }
        }
    }
}

// ==================== ERRORS ====================

#[derive(Debug)]
pub enum StreamError {
    InvalidSequence(u64),
    EventOverwritten(u64),
    Other(String),
}

impl std::fmt::Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamError::InvalidSequence(seq) => {
                write!(f, "Invalid sequence number: {}", seq)
            }
            StreamError::EventOverwritten(seq) => {
                write!(f, "Event {} has been overwritten", seq)
            }
            StreamError::Other(msg) => write!(f, "Stream error: {}", msg),
        }
    }
}

impl std::error::Error for StreamError {}

// ==================== UTILITIES ====================

/// Get current timestamp in microseconds
fn current_timestamp_micros() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_micros() as u64
}

// ==================== TESTS ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_size() {
        assert_eq!(std::mem::size_of::<ExperienceEvent>(), 128);
        assert_eq!(std::mem::align_of::<ExperienceEvent>(), 16);
    }

    #[test]
    fn test_event_creation() {
        let event = ExperienceEvent::new(EventType::TokenCreated);
        assert_eq!(event.event_type, EventType::TokenCreated as u16);
        assert_ne!(event.event_id, Uuid::nil());
        assert!(event.timestamp > 0);
    }

    #[test]
    fn test_event_builder() {
        let event = ExperienceEvent::new(EventType::ActionStarted)
            .with_episode(42, 10)
            .with_state([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0])
            .with_action([0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8])
            .with_flags(EventFlags::TRAJECTORY);

        assert_eq!(event.episode_id, 42);
        assert_eq!(event.step_number, 10);
        assert_eq!(event.state[0], 1.0);
        assert_eq!(event.action[0], 0.1);
        assert_eq!(event.flags, EventFlags::TRAJECTORY);
    }

    #[test]
    fn test_hot_buffer_write_read() {
        let buffer = HotBuffer::new(10);
        let event = ExperienceEvent::new(EventType::TokenCreated);

        let seq = buffer.write(event);
        assert_eq!(seq, 1);

        let read_event = buffer.read(0);
        assert!(read_event.is_some());
        assert_eq!(read_event.unwrap().event_id, event.event_id);
    }

    #[test]
    fn test_circular_wrap() {
        let buffer = HotBuffer::new(10);

        // Write 15 events
        for i in 0..15 {
            let mut event = ExperienceEvent::new(EventType::TokenCreated);
            event.step_number = i;
            buffer.write(event);
        }

        // Events 0-4 should be overwritten (not readable)
        assert!(buffer.read(0).is_none());

        // Events 5-14 should still exist
        let event = buffer.read(5).unwrap();
        assert_eq!(event.step_number, 5);

        // Total written is 15
        assert_eq!(buffer.total_written(), 15);

        // But size is capped at 10
        assert_eq!(buffer.size(), 10);
    }

    #[test]
    fn test_experience_stream_write_read() {
        let stream = ExperienceStream::new(1000, 100);
        let event = ExperienceEvent::new(EventType::TokenCreated);

        let seq = stream.write_event(event).unwrap();
        assert_eq!(seq, 1);

        let read_event = stream.get_event(0).unwrap();
        assert_eq!(read_event.event_id, event.event_id);
    }

    #[test]
    fn test_query_range() {
        let stream = ExperienceStream::new(1000, 100);

        // Write 10 events
        for i in 0..10 {
            let mut event = ExperienceEvent::new(EventType::TokenCreated);
            event.step_number = i;
            stream.write_event(event).unwrap();
        }

        // Query range [2, 5)
        let events = stream.query_range(2, 5);
        assert_eq!(events.len(), 3);
        assert_eq!(events[0].step_number, 2);
        assert_eq!(events[1].step_number, 3);
        assert_eq!(events[2].step_number, 4);
    }

    #[test]
    fn test_update_reward() {
        let stream = ExperienceStream::new(1000, 100);
        let event = ExperienceEvent::new(EventType::TokenCreated);

        let seq = stream.write_event(event).unwrap();

        // Update reward
        stream.update_reward(seq - 1, 1.5).unwrap();
        stream.update_reward(seq - 1, 0.5).unwrap();

        // Check updated reward
        let updated = stream.get_event(seq - 1).unwrap();
        assert_eq!(updated.reward, 2.0);
    }

    #[tokio::test]
    async fn test_pubsub() {
        let stream = Arc::new(ExperienceStream::new(1000, 100));
        let mut rx = stream.subscribe();

        let event = ExperienceEvent::new(EventType::TokenCreated);
        let event_id = event.event_id;

        stream.write_event(event).unwrap();

        let received = rx.recv().await.unwrap();
        assert_eq!(received.event_id, event_id);
    }

    #[tokio::test]
    async fn test_multiple_subscribers() {
        let stream = Arc::new(ExperienceStream::new(1000, 100));
        let mut rx1 = stream.subscribe();
        let mut rx2 = stream.subscribe();

        let event = ExperienceEvent::new(EventType::TokenCreated);
        let event_id = event.event_id;

        stream.write_event(event).unwrap();

        let received1 = rx1.recv().await.unwrap();
        let received2 = rx2.recv().await.unwrap();

        assert_eq!(received1.event_id, event_id);
        assert_eq!(received2.event_id, event_id);
    }

    #[test]
    fn test_sample_recent() {
        let stream = ExperienceStream::new(1000, 100);

        // Write 100 events
        for i in 0..100 {
            let mut event = ExperienceEvent::new(EventType::TokenCreated);
            event.step_number = i;
            stream.write_event(event).unwrap();
        }

        // Sample recent 10
        let recent = stream.sample_batch(10, SamplingStrategy::Recent(10));
        assert_eq!(recent.len(), 10);
        assert_eq!(recent[0].step_number, 90);
        assert_eq!(recent[9].step_number, 99);
    }
}
