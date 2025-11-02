# ExperienceStream v2.0 — Система памяти событий

**Версия:** 2.0.0
**Дата:** 2025-11-02
**Статус:** Спецификация для реализации
**Зависимости:** None (фундаментальный модуль)
**Язык:** Rust + tokio async runtime
**Цель:** Единая система сбора, хранения и распространения событий для всей архитектуры KEY

---

## 1. Философия и назначение

### 1.1 Центральная роль в архитектуре

ExperienceStream — это **нервная система** NeuroGraph OS. Все события, действия, изменения состояния записываются в единый поток и становятся доступными для:

- **Appraisers** (Группа II) - оценивают события и генерируют reward
- **IntuitionEngine** (Группа III) - анализирует паттерны для обучения
- **EvolutionManager** (Группа III) - логирует изменения ADNA
- **ActionController** (Группа IV) - записывает выполненные действия
- **Мониторинг и отладка** - полная аудит-история системы

### 1.2 Ключевые принципы

- **Write-Ahead Log:** Все события записываются до их обработки
- **Pub-Sub модель:** Множественные подписчики на один поток
- **Асинхронность:** Запись не блокирует операции
- **Структурированность:** Единый формат для всех типов событий
- **Эффективность:** Circular buffer в памяти + опциональная персистентность

---

## 2. Архитектура

### 2.1 Компоненты

```
┌─────────────────────────────────────────────────┐
│           ExperienceWriter (трейт)              │
│  write_event() - запись новых событий           │
└─────────────────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────┐
│         ExperienceStream (реализация)           │
│  ┌───────────────────────────────────────────┐  │
│  │  Hot Buffer (Circular, in-memory)         │  │
│  │  Capacity: 1M events (~128 MB)            │  │
│  └───────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────┐  │
│  │  Broadcast Channel (tokio::sync)          │  │
│  │  Real-time event distribution             │  │
│  └───────────────────────────────────────────┘  │
│  ┌───────────────────────────────────────────┐  │
│  │  Cold Storage (Optional, on-disk)         │  │
│  │  Format: Parquet/MessagePack              │  │
│  └───────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
                      ↓
┌─────────────────────────────────────────────────┐
│           ExperienceReader (трейт)              │
│  - get_event(id)                                │
│  - query_range(start, end)                      │
│  - sample_batch(size, strategy)                 │
└─────────────────────────────────────────────────┘
```

### 2.2 Data Flow

```
[ActionController] ──┐
[Guardian]         ──┤
[Appraisers]       ──┤─→ write_event() ─→ [Hot Buffer] ─→ [Broadcast] ─→ [Appraisers]
[Evolution Mgr]    ──┤                                  │                  [Intuition]
                     │                                  │                  [Monitoring]
                     │                                  ↓
                     └──────────────────────── [Cold Storage (async)]
```

---

## 3. ExperienceEvent структура (128 байт)

### 3.1 Binary Layout

```
Offset | Size  | Field
-------|-------|------------------
0-15   | 16    | event_id (UUID v4)
16-23  | 8     | timestamp (Unix μs)
24-31  | 8     | episode_id
32-35  | 4     | step_number
36-37  | 2     | event_type
38-39  | 2     | flags
40-71  | 32    | state_vector [f32; 8]
72-103 | 32    | action_vector [f32; 8]
104-107| 4     | reward
108-111| 4     | adna_version_hash
112-127| 16    | reserved
-------|-------|------------------
TOTAL  | 128   | bytes
```

### 3.2 Rust Definition

```rust
use uuid::Uuid;

/// ExperienceEvent - единая структура для всех событий (128 байт)
#[repr(C, align(16))]
#[derive(Debug, Clone, Copy)]
pub struct ExperienceEvent {
    /// Unique event identifier
    pub event_id: Uuid, // 16 bytes

    /// Timestamp (Unix epoch microseconds)
    pub timestamp: u64, // 8 bytes

    /// Episode ID (для группировки связанных событий)
    pub episode_id: u64, // 8 bytes

    /// Step number within episode
    pub step_number: u32, // 4 bytes

    /// Event type discriminator
    pub event_type: u16, // 2 bytes (см. EventType enum)

    /// Event flags
    pub flags: u16, // 2 bytes (см. EventFlags)

    /// State vector (8D representation)
    /// Может содержать: [L1..L8] coordinates, system metrics, etc.
    pub state: [f32; 8], // 32 bytes

    /// Action vector (8D representation)
    /// Для action events: параметры действия
    /// Для других: может быть неиспользовано
    pub action: [f32; 8], // 32 bytes

    /// Accumulated reward (updated by Appraisers)
    pub reward: f32, // 4 bytes

    /// ADNA version hash (first 4 bytes)
    pub adna_version_hash: u32, // 4 bytes

    /// Reserved for future use
    pub _reserved: [u8; 16], // 16 bytes
}

// Compile-time assertion
const _: () = assert!(std::mem::size_of::<ExperienceEvent>() == 128);
```

### 3.3 EventType Enum

```rust
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    // === System Events (0x00xx) ===
    SystemStartup         = 0x0001,
    SystemShutdown        = 0x0002,
    CDNAUpdated          = 0x0010,
    ADNAUpdated          = 0x0011,

    // === Token/Connection Events (0x01xx) ===
    TokenCreated         = 0x0100,
    TokenDeleted         = 0x0101,
    TokenActivated       = 0x0102,
    ConnectionCreated    = 0x0110,
    ConnectionDeleted    = 0x0111,
    ConnectionActivated  = 0x0112,

    // === Action Events (0x02xx) ===
    ActionStarted        = 0x0200,
    ActionCompleted      = 0x0201,
    ActionFailed         = 0x0202,

    // === Appraisal Events (0x03xx) ===
    HomeostasisReward    = 0x0300,
    CuriosityReward      = 0x0301,
    EfficiencyReward     = 0x0302,
    GoalReward           = 0x0303,

    // === Learning Events (0x04xx) ===
    ProposalGenerated    = 0x0400,
    ProposalAccepted     = 0x0401,
    ProposalRejected     = 0x0402,

    // === Custom Events (0xF0xx) ===
    CustomUserEvent      = 0xF000,
}
```

### 3.4 EventFlags

```rust
pub struct EventFlags;

impl EventFlags {
    /// Event требует немедленной обработки
    pub const URGENT: u16           = 0x0001;

    /// Event является частью траектории
    pub const TRAJECTORY: u16       = 0x0002;

    /// Event содержит ошибку
    pub const ERROR: u16            = 0x0004;

    /// Event должен быть персистентен на диск
    pub const PERSIST: u16          = 0x0008;

    /// Event прошёл все Appraisers
    pub const FULLY_APPRAISED: u16  = 0x0010;

    /// Reserved flags
    pub const _RESERVED: u16        = 0xFFE0;
}
```

---

## 4. Hot Buffer (Circular Buffer)

### 4.1 Реализация

```rust
use std::sync::Arc;
use parking_lot::RwLock;

/// Circular buffer для горячего хранения событий
pub struct HotBuffer {
    /// Fixed-size buffer
    events: Box<[ExperienceEvent]>,

    /// Capacity (обычно 1M events)
    capacity: usize,

    /// Write position (wraps around)
    write_pos: Arc<RwLock<usize>>,

    /// Total events written (never wraps)
    total_written: Arc<RwLock<u64>>,
}

impl HotBuffer {
    /// Create new buffer with given capacity
    pub fn new(capacity: usize) -> Self {
        let events = vec![ExperienceEvent::default(); capacity]
            .into_boxed_slice();

        Self {
            events,
            capacity,
            write_pos: Arc::new(RwLock::new(0)),
            total_written: Arc::new(RwLock::new(0)),
        }
    }

    /// Write event to buffer (lock-free read, single writer)
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

    /// Query range of events
    pub fn query_range(&self, start: u64, end: u64) -> Vec<ExperienceEvent> {
        (start..end)
            .filter_map(|seq| self.read(seq))
            .collect()
    }

    /// Get current size
    pub fn size(&self) -> usize {
        let total = *self.total_written.read();
        std::cmp::min(total as usize, self.capacity)
    }
}
```

### 4.2 Memory Layout

```
Capacity: 1M events × 128 bytes = 128 MB

[0]────────→[Event 0]
[1]────────→[Event 1]
...
[999,999]──→[Event 999,999]
             ↓ wrap around
[0]────────→[Event 1,000,000]  (overwrites Event 0)
```

---

## 5. Pub-Sub система (Broadcast Channel)

### 5.1 Реализация

```rust
use tokio::sync::broadcast;

pub struct ExperienceStream {
    /// Hot buffer
    buffer: Arc<HotBuffer>,

    /// Broadcast channel для реального времени
    tx: broadcast::Sender<ExperienceEvent>,

    /// Cold storage (опционально)
    cold_storage: Option<Arc<ColdStorage>>,
}

impl ExperienceStream {
    pub fn new(capacity: usize, channel_size: usize) -> Self {
        let buffer = Arc::new(HotBuffer::new(capacity));
        let (tx, _rx) = broadcast::channel(channel_size);

        Self {
            buffer,
            tx,
            cold_storage: None,
        }
    }

    /// Enable cold storage
    pub fn with_cold_storage(mut self, storage: Arc<ColdStorage>) -> Self {
        self.cold_storage = Some(storage);
        self
    }
}

impl ExperienceWriter for ExperienceStream {
    fn write_event(&self, event: ExperienceEvent) -> Result<u64, StreamError> {
        // 1. Write to hot buffer
        let seq = self.buffer.write(event);

        // 2. Broadcast to subscribers (non-blocking)
        let _ = self.tx.send(event); // Игнорируем ошибку если нет подписчиков

        // 3. Асинхронно записываем в cold storage (если включено)
        if let Some(storage) = &self.cold_storage {
            let storage = Arc::clone(storage);
            let event = event;
            tokio::spawn(async move {
                if let Err(e) = storage.write(event).await {
                    eprintln!("Cold storage write failed: {}", e);
                }
            });
        }

        Ok(seq)
    }
}

impl ExperienceReader for ExperienceStream {
    fn get_event(&self, seq: u64) -> Option<ExperienceEvent> {
        self.buffer.read(seq)
    }

    fn query_range(&self, start: u64, end: u64) -> Vec<ExperienceEvent> {
        self.buffer.query_range(start, end)
    }

    fn subscribe(&self) -> broadcast::Receiver<ExperienceEvent> {
        self.tx.subscribe()
    }
}
```

### 5.2 Subscriber Example

```rust
// Appraiser subscribing to events
pub async fn run_appraiser(stream: Arc<ExperienceStream>) {
    let mut rx = stream.subscribe();

    loop {
        match rx.recv().await {
            Ok(event) => {
                // Process event
                process_event(event).await;
            }
            Err(broadcast::error::RecvError::Lagged(n)) => {
                eprintln!("Appraiser lagged by {} events", n);
            }
            Err(_) => break,
        }
    }
}
```

---

## 6. Sampling Strategies

### 6.1 SamplingStrategy Enum

```rust
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

    /// Custom predicate
    Custom(fn(&ExperienceEvent) -> bool),
}
```

### 6.2 Batch Sampling

```rust
impl ExperienceStream {
    /// Sample batch of events for analysis
    pub fn sample_batch(
        &self,
        size: usize,
        strategy: SamplingStrategy,
    ) -> Vec<ExperienceEvent> {
        let total = self.buffer.size();

        match strategy {
            SamplingStrategy::Uniform => {
                // Random sampling
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

                // Calculate priorities
                let priorities: Vec<f32> = all_events
                    .iter()
                    .map(|e| e.reward.abs())
                    .collect();

                // Weighted sampling
                weighted_sample(&all_events, &priorities, size)
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

            SamplingStrategy::Custom(predicate) => {
                (0..total as u64)
                    .filter_map(|seq| self.buffer.read(seq))
                    .filter(predicate)
                    .take(size)
                    .collect()
            }
        }
    }
}
```

---

## 7. Cold Storage (Optional)

### 7.1 ColdStorage Trait

```rust
use async_trait::async_trait;

#[async_trait]
pub trait ColdStorage: Send + Sync {
    /// Write event to persistent storage
    async fn write(&self, event: ExperienceEvent) -> Result<(), StorageError>;

    /// Query events by time range
    async fn query_time_range(
        &self,
        start: u64,
        end: u64,
    ) -> Result<Vec<ExperienceEvent>, StorageError>;

    /// Query events by episode
    async fn query_episode(
        &self,
        episode_id: u64,
    ) -> Result<Vec<ExperienceEvent>, StorageError>;

    /// Flush pending writes
    async fn flush(&self) -> Result<(), StorageError>;
}
```

### 7.2 File-based Implementation

```rust
use std::path::PathBuf;

pub struct FileColdStorage {
    /// Base directory
    base_dir: PathBuf,

    /// Batch size for flushing
    batch_size: usize,

    /// Pending events
    pending: Arc<RwLock<Vec<ExperienceEvent>>>,
}

impl FileColdStorage {
    pub fn new(base_dir: PathBuf, batch_size: usize) -> Self {
        std::fs::create_dir_all(&base_dir).unwrap();

        Self {
            base_dir,
            batch_size,
            pending: Arc::new(RwLock::new(Vec::new())),
        }
    }

    async fn flush_to_disk(&self) -> Result<(), StorageError> {
        let events = {
            let mut pending = self.pending.write();
            std::mem::take(&mut *pending)
        };

        if events.is_empty() {
            return Ok(());
        }

        // Group by day
        let timestamp = events[0].timestamp;
        let day = timestamp / (86400 * 1_000_000); // microseconds per day

        let filename = self.base_dir.join(format!("{}.parquet", day));

        // Serialize to Parquet (или MessagePack)
        // TODO: реальная сериализация
        tokio::fs::write(filename, serialize_events(&events)?).await?;

        Ok(())
    }
}

#[async_trait]
impl ColdStorage for FileColdStorage {
    async fn write(&self, event: ExperienceEvent) -> Result<(), StorageError> {
        let should_flush = {
            let mut pending = self.pending.write();
            pending.push(event);
            pending.len() >= self.batch_size
        };

        if should_flush {
            self.flush_to_disk().await?;
        }

        Ok(())
    }

    async fn flush(&self) -> Result<(), StorageError> {
        self.flush_to_disk().await
    }

    // ... остальные методы ...
}
```

---

## 8. Reward Updates

### 8.1 Mechanism

Appraisers могут обновлять reward существующих событий:

```rust
impl ExperienceStream {
    /// Update reward for existing event
    pub fn update_reward(&self, seq: u64, delta: f32) -> Result<(), StreamError> {
        // This is tricky in circular buffer - need atomic f32 add
        // Option 1: Separate reward accumulator
        // Option 2: Lock-based update
        // Option 3: Accept eventual consistency

        // MVP: Simple implementation with lock
        let idx = (seq as usize) % self.buffer.capacity;
        unsafe {
            let ptr = self.buffer.events.as_ptr() as *mut ExperienceEvent;
            let event = &mut *ptr.add(idx);
            event.reward += delta;
        }

        Ok(())
    }
}
```

### 8.2 Appraiser Integration

```rust
// In HomeostasisAppraiser
async fn process_event(&self, event: ExperienceEvent) {
    let reward = calculate_homeostasis_reward(&event);

    // Update event reward
    self.stream.update_reward(event.sequence_number, reward).await;
}
```

---

## 9. API Summary

### 9.1 ExperienceWriter Trait

```rust
pub trait ExperienceWriter: Send + Sync {
    /// Write new event
    fn write_event(&self, event: ExperienceEvent) -> Result<u64, StreamError>;

    /// Write multiple events
    fn write_batch(&self, events: Vec<ExperienceEvent>) -> Result<Vec<u64>, StreamError> {
        events.into_iter().map(|e| self.write_event(e)).collect()
    }

    /// Update reward for existing event
    fn update_reward(&self, seq: u64, delta: f32) -> Result<(), StreamError>;
}
```

### 9.2 ExperienceReader Trait

```rust
pub trait ExperienceReader: Send + Sync {
    /// Get single event by sequence number
    fn get_event(&self, seq: u64) -> Option<ExperienceEvent>;

    /// Query range [start, end)
    fn query_range(&self, start: u64, end: u64) -> Vec<ExperienceEvent>;

    /// Sample batch with strategy
    fn sample_batch(&self, size: usize, strategy: SamplingStrategy) -> Vec<ExperienceEvent>;

    /// Subscribe to real-time events
    fn subscribe(&self) -> broadcast::Receiver<ExperienceEvent>;

    /// Get current stream size
    fn size(&self) -> usize;

    /// Get total events written
    fn total_written(&self) -> u64;
}
```

---

## 10. Configuration

### 10.1 ExperienceStreamConfig

```rust
#[derive(Debug, Clone, serde::Deserialize)]
pub struct ExperienceStreamConfig {
    /// Hot buffer capacity (events)
    pub hot_buffer_capacity: usize,

    /// Broadcast channel size
    pub channel_size: usize,

    /// Enable cold storage
    pub enable_cold_storage: bool,

    /// Cold storage path
    pub cold_storage_path: Option<String>,

    /// Cold storage batch size
    pub cold_storage_batch_size: usize,

    /// Auto-flush interval (seconds)
    pub auto_flush_interval_secs: u64,
}

impl Default for ExperienceStreamConfig {
    fn default() -> Self {
        Self {
            hot_buffer_capacity: 1_000_000,
            channel_size: 1000,
            enable_cold_storage: false,
            cold_storage_path: None,
            cold_storage_batch_size: 10_000,
            auto_flush_interval_secs: 300, // 5 minutes
        }
    }
}
```

### 10.2 Initialization

```rust
// Initialize ExperienceStream
let config = ExperienceStreamConfig::default();
let stream = Arc::new(ExperienceStream::new(
    config.hot_buffer_capacity,
    config.channel_size,
));

// Optional: add cold storage
if config.enable_cold_storage {
    let storage = Arc::new(FileColdStorage::new(
        PathBuf::from(config.cold_storage_path.unwrap()),
        config.cold_storage_batch_size,
    ));
    stream = Arc::new(stream.with_cold_storage(storage));
}

// Start auto-flush task
if config.enable_cold_storage {
    let stream_clone = Arc::clone(&stream);
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(
            Duration::from_secs(config.auto_flush_interval_secs)
        );
        loop {
            interval.tick().await;
            if let Some(storage) = &stream_clone.cold_storage {
                let _ = storage.flush().await;
            }
        }
    });
}
```

---

## 11. Testing

### 11.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_event_size() {
        assert_eq!(std::mem::size_of::<ExperienceEvent>(), 128);
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
    fn test_circular_wrap() {
        let buffer = HotBuffer::new(10);

        // Write 15 events
        for i in 0..15 {
            let mut event = ExperienceEvent::default();
            event.step_number = i;
            buffer.write(event);
        }

        // Events 0-4 should be overwritten
        assert!(buffer.read(0).is_none());

        // Events 5-14 should still exist
        let event = buffer.read(5).unwrap();
        assert_eq!(event.step_number, 5);
    }

    #[tokio::test]
    async fn test_pubsub() {
        let stream = Arc::new(ExperienceStream::new(1000, 100));
        let mut rx = stream.subscribe();

        let event = ExperienceEvent::default();
        stream.write_event(event).unwrap();

        let received = rx.recv().await.unwrap();
        assert_eq!(received.event_id, event.event_id);
    }
}
```

### 11.2 Integration Tests

```rust
#[tokio::test]
async fn test_appraiser_integration() {
    let stream = Arc::new(ExperienceStream::new(1000, 100));

    // Start mock appraiser
    let stream_clone = Arc::clone(&stream);
    tokio::spawn(async move {
        let mut rx = stream_clone.subscribe();
        while let Ok(event) = rx.recv().await {
            // Update reward
            stream_clone.update_reward(event.sequence_number, 1.0).unwrap();
        }
    });

    // Write event
    let event = ExperienceEvent::default();
    let seq = stream.write_event(event).unwrap();

    // Wait a bit for async processing
    tokio::time::sleep(Duration::from_millis(10)).await;

    // Check reward was updated
    let updated = stream.get_event(seq - 1).unwrap();
    assert_eq!(updated.reward, 1.0);
}
```

---

## 12. Performance Characteristics

### 12.1 Memory

- **Hot Buffer:** 128 MB (1M events × 128 bytes)
- **Channel Buffer:** ~128 KB (1K events × 128 bytes)
- **Per Subscriber:** Minimal (tokio channel receiver)

### 12.2 Latency

- **Write:** <100 ns (single memcpy + atomic increment)
- **Broadcast:** <1 μs (tokio channel send)
- **Read:** <50 ns (direct array access)
- **Query Range (1K events):** ~50 μs

### 12.3 Throughput

- **Writes:** ~10M events/sec (single writer)
- **Broadcast:** Limited by subscriber processing speed
- **Reads:** ~20M ops/sec (parallel readers)

---

## 13. Limitations & Future Work

### 13.1 MVP Limitations

- ❌ Single-writer assumption (no concurrent writes from multiple threads)
- ❌ Reward updates не atomic (eventual consistency)
- ❌ Cold storage реализация упрощена (batch writes)
- ❌ Нет компрессии для cold storage
- ❌ Нет индексации по event_type или episode_id

### 13.2 Future Improvements (v2.1+)

- Multi-writer support с lock-free algorithms
- Atomic reward accumulation (CAS loop)
- Parquet/Arrow format для cold storage
- Columnar indices для fast queries
- Compression (LZ4/Zstd) для холодных данных
- Distributed storage (S3, etc.)

---

## 14. Integration с ключевыми модулями

### 14.1 Guardian Integration

```rust
impl Guardian {
    pub fn with_experience_stream(mut self, stream: Arc<ExperienceStream>) -> Self {
        self.experience_stream = Some(stream);
        self
    }

    pub fn log_cdna_update(&self, old: &CDNA, new: &CDNA) {
        if let Some(stream) = &self.experience_stream {
            let event = ExperienceEvent {
                event_type: EventType::CDNAUpdated as u16,
                // ... fill fields ...
                ..Default::default()
            };
            let _ = stream.write_event(event);
        }
    }
}
```

### 14.2 Appraiser Template

```rust
pub async fn run_appraiser(
    stream: Arc<ExperienceStream>,
    appraise_fn: impl Fn(&ExperienceEvent) -> f32,
) {
    let mut rx = stream.subscribe();

    loop {
        match rx.recv().await {
            Ok(event) => {
                let reward = appraise_fn(&event);
                let _ = stream.update_reward(event.sequence_number, reward);
            }
            Err(_) => break,
        }
    }
}
```

---

## 15. Резюме

### 15.1 Deliverables для v0.22.0

1. ✅ ExperienceEvent (128-byte структура)
2. ✅ HotBuffer (circular buffer)
3. ✅ Pub-Sub система (tokio::broadcast)
4. ✅ Sampling strategies
5. ✅ Optional cold storage
6. ✅ ExperienceWriter/Reader traits
7. ✅ Comprehensive tests

### 15.2 Ценность для архитектуры

ExperienceStream v2.0 обеспечивает:
- **Unified memory** для всей системы KEY
- **Real-time feedback** для Appraisers
- **Historical analysis** для IntuitionEngine
- **Full auditability** для отладки и мониторинга
- **Scalability** для будущего роста

---

**Конец спецификации ExperienceStream v2.0**

*Эта спецификация определяет фундаментальную систему памяти для NeuroGraph OS, обеспечивая единый поток событий для всех компонентов архитектуры KEY с высокой производительностью и гибкостью.*
