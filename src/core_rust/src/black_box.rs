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

/// Black Box Recorder v1.0 - Flight recorder for production debugging
///
/// Records last N system events in a circular buffer and dumps to disk on panic.
/// Similar to aircraft black box recorders - helps debug production crashes.
///
/// # Architecture
///
/// - **Circular Buffer**: Fixed-size ring buffer (default: 1000 events)
/// - **Thread-Safe**: Arc<Mutex<>> for multi-threaded access
/// - **Automatic Dump**: Writes to disk on panic via panic hook
/// - **JSON Format**: Human-readable event log
///
/// # Event Types
///
/// - TokenCreated, ConnectionCreated
/// - ValidationFailed, QuotaExceeded
/// - PanicRecovered, WalWritten
/// - Custom events with arbitrary data
///
/// # Usage
///
/// ```rust
/// use neurograph_core::black_box::{BlackBox, Event, EventType};
///
/// // Initialize global black box
/// let bb = BlackBox::new(1000);
///
/// // Record events
/// bb.record(Event::new(EventType::TokenCreated)
///     .with_data("token_id", "42"));
///
/// // Dump to file (on panic or manually)
/// bb.dump_to_file("crash_dump.json").unwrap();
/// ```

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, error, info};

/// Event types that can be recorded
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    /// Token was created
    TokenCreated,
    /// Connection was created
    ConnectionCreated,
    /// Token validation failed
    TokenValidationFailed,
    /// Connection validation failed
    ConnectionValidationFailed,
    /// Resource quota was exceeded
    QuotaExceeded,
    /// Aggressive memory cleanup triggered
    AggressiveCleanup,
    /// Panic was caught and recovered
    PanicRecovered,
    /// WAL entry was written
    WalWritten,
    /// WAL replay completed
    WalReplayed,
    /// System started
    SystemStarted,
    /// System stopped
    SystemStopped,
    /// Custom event with type name
    Custom(String),
}

/// Single event record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    /// Event type
    pub event_type: EventType,
    /// Unix timestamp in microseconds
    pub timestamp_us: u64,
    /// Optional event data (key-value pairs)
    pub data: Vec<(String, String)>,
}

impl Event {
    /// Create new event with current timestamp
    pub fn new(event_type: EventType) -> Self {
        let timestamp_us = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_micros() as u64;

        Self {
            event_type,
            timestamp_us,
            data: Vec::new(),
        }
    }

    /// Add data field to event (builder pattern)
    pub fn with_data(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.data.push((key.into(), value.into()));
        self
    }

    /// Add multiple data fields
    pub fn with_data_vec(mut self, data: Vec<(String, String)>) -> Self {
        self.data.extend(data);
        self
    }
}

/// Black Box Recorder - circular buffer for system events
#[derive(Clone)]
pub struct BlackBox {
    inner: Arc<Mutex<BlackBoxInner>>,
}

struct BlackBoxInner {
    /// Circular buffer of events
    events: VecDeque<Event>,
    /// Maximum buffer size
    capacity: usize,
    /// Total events recorded (including dropped)
    total_recorded: u64,
    /// Total events dropped (when buffer full)
    total_dropped: u64,
}

impl BlackBox {
    /// Create new Black Box Recorder
    ///
    /// # Arguments
    ///
    /// * `capacity` - Maximum number of events to keep in buffer
    ///
    /// # Example
    ///
    /// ```rust
    /// use neurograph_core::black_box::BlackBox;
    ///
    /// let bb = BlackBox::new(1000); // Keep last 1000 events
    /// ```
    pub fn new(capacity: usize) -> Self {
        info!("Black Box Recorder initialized: capacity={}", capacity);

        Self {
            inner: Arc::new(Mutex::new(BlackBoxInner {
                events: VecDeque::with_capacity(capacity),
                capacity,
                total_recorded: 0,
                total_dropped: 0,
            })),
        }
    }

    /// Record an event
    ///
    /// If buffer is full, oldest event is dropped.
    ///
    /// # Example
    ///
    /// ```rust
    /// use neurograph_core::black_box::{BlackBox, Event, EventType};
    ///
    /// let bb = BlackBox::new(1000);
    /// bb.record(Event::new(EventType::TokenCreated)
    ///     .with_data("token_id", "42"));
    /// ```
    pub fn record(&self, event: Event) {
        let mut inner = self.inner.lock().unwrap();

        // If buffer full, drop oldest
        if inner.events.len() >= inner.capacity {
            inner.events.pop_front();
            inner.total_dropped += 1;
        }

        inner.events.push_back(event.clone());
        inner.total_recorded += 1;

        debug!(
            event_type = ?event.event_type,
            total_recorded = inner.total_recorded,
            "Black Box event recorded"
        );
    }

    /// Get current number of events in buffer
    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().events.len()
    }

    /// Check if buffer is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get statistics
    pub fn stats(&self) -> BlackBoxStats {
        let inner = self.inner.lock().unwrap();
        BlackBoxStats {
            capacity: inner.capacity,
            current_size: inner.events.len(),
            total_recorded: inner.total_recorded,
            total_dropped: inner.total_dropped,
        }
    }

    /// Get all events (clone)
    pub fn get_events(&self) -> Vec<Event> {
        self.inner.lock().unwrap().events.iter().cloned().collect()
    }

    /// Dump events to JSON file
    ///
    /// # Arguments
    ///
    /// * `path` - File path to write dump
    ///
    /// # Returns
    ///
    /// Number of events written
    ///
    /// # Example
    ///
    /// ```rust
    /// use neurograph_core::black_box::BlackBox;
    ///
    /// let bb = BlackBox::new(1000);
    /// // ... record events ...
    /// bb.dump_to_file("crash_dump.json").unwrap();
    /// ```
    pub fn dump_to_file<P: AsRef<Path>>(&self, path: P) -> Result<usize, std::io::Error> {
        let events = self.get_events();
        let stats = self.stats();

        let dump = BlackBoxDump {
            timestamp_us: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_micros() as u64,
            stats,
            events,
        };

        let json = serde_json::to_string_pretty(&dump)?;

        let mut file = File::create(path.as_ref())?;
        file.write_all(json.as_bytes())?;
        file.sync_all()?;

        info!(
            path = %path.as_ref().display(),
            events_count = dump.events.len(),
            "Black Box dump written"
        );

        Ok(dump.events.len())
    }

    /// Clear all events
    pub fn clear(&self) {
        let mut inner = self.inner.lock().unwrap();
        inner.events.clear();
        debug!("Black Box cleared");
    }
}

/// Black Box statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlackBoxStats {
    /// Maximum buffer capacity
    pub capacity: usize,
    /// Current number of events in buffer
    pub current_size: usize,
    /// Total events recorded (including dropped)
    pub total_recorded: u64,
    /// Total events dropped when buffer was full
    pub total_dropped: u64,
}

/// Complete dump structure
#[derive(Debug, Serialize, Deserialize)]
struct BlackBoxDump {
    /// Dump timestamp in microseconds
    timestamp_us: u64,
    /// Statistics at time of dump
    stats: BlackBoxStats,
    /// All recorded events
    events: Vec<Event>,
}

// ==================== GLOBAL BLACK BOX ====================

use lazy_static::lazy_static;

lazy_static! {
    /// Global Black Box Recorder instance
    ///
    /// Default capacity: 1000 events
    ///
    /// # Example
    ///
    /// ```rust
    /// use neurograph_core::black_box::{GLOBAL_BLACK_BOX, Event, EventType};
    ///
    /// GLOBAL_BLACK_BOX.record(Event::new(EventType::TokenCreated)
    ///     .with_data("token_id", "42"));
    /// ```
    pub static ref GLOBAL_BLACK_BOX: BlackBox = BlackBox::new(1000);
}

/// Convenience function to record event to global black box
///
/// # Example
///
/// ```rust
/// use neurograph_core::black_box::{record_event, Event, EventType};
///
/// record_event(Event::new(EventType::TokenCreated)
///     .with_data("token_id", "42"));
/// ```
pub fn record_event(event: Event) {
    GLOBAL_BLACK_BOX.record(event);
}

/// Dump global black box to file
///
/// # Example
///
/// ```rust
/// use neurograph_core::black_box::dump_to_file;
///
/// dump_to_file("crash_dump.json").unwrap();
/// ```
pub fn dump_to_file<P: AsRef<Path>>(path: P) -> Result<usize, std::io::Error> {
    GLOBAL_BLACK_BOX.dump_to_file(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_black_box_creation() {
        let bb = BlackBox::new(10);
        assert_eq!(bb.len(), 0);
        assert!(bb.is_empty());
    }

    #[test]
    fn test_record_event() {
        let bb = BlackBox::new(10);

        bb.record(Event::new(EventType::TokenCreated).with_data("id", "1"));
        bb.record(Event::new(EventType::ConnectionCreated).with_data("id", "2"));

        assert_eq!(bb.len(), 2);
        assert!(!bb.is_empty());
    }

    #[test]
    fn test_circular_buffer() {
        let bb = BlackBox::new(3); // Small buffer

        // Record 5 events (buffer size is 3)
        for i in 0..5 {
            bb.record(Event::new(EventType::TokenCreated).with_data("id", i.to_string()));
        }

        // Should only have last 3 events
        assert_eq!(bb.len(), 3);

        let stats = bb.stats();
        assert_eq!(stats.current_size, 3);
        assert_eq!(stats.total_recorded, 5);
        assert_eq!(stats.total_dropped, 2); // First 2 were dropped
    }

    #[test]
    fn test_get_events() {
        let bb = BlackBox::new(10);

        bb.record(Event::new(EventType::TokenCreated).with_data("id", "1"));
        bb.record(Event::new(EventType::PanicRecovered).with_data("msg", "test"));

        let events = bb.get_events();
        assert_eq!(events.len(), 2);
        assert_eq!(events[0].event_type, EventType::TokenCreated);
        assert_eq!(events[1].event_type, EventType::PanicRecovered);
    }

    #[test]
    fn test_dump_to_file() {
        let bb = BlackBox::new(10);

        bb.record(Event::new(EventType::SystemStarted));
        bb.record(Event::new(EventType::TokenCreated).with_data("id", "42"));
        bb.record(Event::new(EventType::SystemStopped));

        let temp_file = std::env::temp_dir().join("black_box_test.json");
        let count = bb.dump_to_file(&temp_file).unwrap();

        assert_eq!(count, 3);
        assert!(temp_file.exists());

        // Verify JSON is valid
        let json = std::fs::read_to_string(&temp_file).unwrap();
        let dump: BlackBoxDump = serde_json::from_str(&json).unwrap();
        assert_eq!(dump.events.len(), 3);

        // Cleanup
        std::fs::remove_file(temp_file).ok();
    }

    #[test]
    fn test_clear() {
        let bb = BlackBox::new(10);

        bb.record(Event::new(EventType::TokenCreated));
        bb.record(Event::new(EventType::ConnectionCreated));

        assert_eq!(bb.len(), 2);

        bb.clear();

        assert_eq!(bb.len(), 0);
        assert!(bb.is_empty());
    }

    #[test]
    fn test_global_black_box() {
        // Global black box should work
        record_event(Event::new(EventType::SystemStarted));

        let stats = GLOBAL_BLACK_BOX.stats();
        assert!(stats.total_recorded > 0);
    }
}
