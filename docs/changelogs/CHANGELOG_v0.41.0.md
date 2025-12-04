# Changelog v0.41.0 - Production Reliability & Data Persistence

**Дата релиза:** 3 декабря 2025
**Статус:** Production-Ready (Core Features) ✅

---

## 🎯 Основные улучшения

v0.41.0 добавляет критически важные возможности для production deployment:
- **Panic Recovery** - защита от крахов при единичных ошибках
- **GIL Release** - параллелизм в Python приложениях
- **WAL (Write-Ahead Log)** - persistence данных с crash recovery
- **Resource Quotas** - защита от OOM и resource exhaustion

Совместно с v0.40.0 (Python Bindings) формирует **production-ready core**.

---

## 🛡️ Part 1: Panic Recovery System v1.0

### Новые возможности

#### Panic Handler Module (`src/panic_handler.rs`)
- **267 LOC** модуль для восстановления после паник
- Предотвращает краш всего процесса при single panic
- Structured logging с полными backtraces

#### API для Panic Recovery

**Sync recovery:**
```rust
use neurograph_core::panic_handler::catch_panic;

let result = catch_panic("token_creation", || {
    // Code that might panic
    create_tokens(1000)
});

match result {
    Ok(tokens) => println!("Success!"),
    Err(panic_err) => eprintln!("Recovered from panic: {}", panic_err.message),
}
```

**Async recovery:**
```rust
use neurograph_core::panic_handler::catch_panic_async;

let result = catch_panic_async("api_request", async {
    // Async code that might panic
    process_request().await
}).await;
```

**Macros:**
```rust
use neurograph_core::recover_panic;

let result = recover_panic!("operation_name", {
    risky_operation()
});
```

#### Global Panic Hook

**Installation:**
```rust
use neurograph_core::panic_handler::install_panic_hook;

fn main() {
    install_panic_hook();
    // Now all panics are logged with full backtraces
}
```

**Output format:**
```
ERROR PANIC OCCURRED
  location: src/main.rs:42:5
  message: index out of bounds: the len is 5 but the index is 10
  backtrace:
    0: rust_begin_unwind
    1: core::panicking::panic_fmt
    ...
```

#### Integration with ActionController

**Safe Intent Execution:**
```rust
impl ActionController {
    pub async fn execute_intent_safe(&self, intent: Intent) -> Result<ActionResult, ActionError> {
        // Wraps execute_intent() with panic recovery
        // Returns ActionError::PanicRecovered on panic
    }
}
```

**Error variant:**
```rust
pub enum ActionError {
    // ... existing variants
    PanicRecovered(String),  // NEW (v0.41.0)
}
```

#### Тестирование

- Unit tests в `src/panic_handler.rs`
- Integration tests в ActionController
- ✅ Все тесты проходят

### Технические детали

**Dependencies:**
```rust
use std::panic;
use std::panic::UnwindSafe;
use tracing::{error, warn};
```

**PanicError structure:**
```rust
pub struct PanicError {
    pub message: String,
    pub location: Option<String>,
}
```

---

## 🐍 Part 2: GIL Release for Python Bindings

### Новые возможности

#### GIL Release в Token Operations

**Before v0.41.0:**
```python
# Python GIL held during entire creation
# Other threads blocked
tokens = Token.create_batch(1_000_000)
```

**After v0.41.0:**
```python
# GIL released during Rust computation
# Other threads can run concurrently
tokens = Token.create_batch(1_000_000)
```

**Implementation:**
```rust
#[staticmethod]
pub fn create_batch(py: Python, count: usize) -> Vec<PyToken> {
    py.allow_threads(|| {
        // GIL released here
        // Heavy Rust computation
        // Other Python threads run freely
        (0..count).map(|_| PyToken::new(Token::new())).collect()
    })
}
```

#### GIL Release в IntuitionEngine

**All methods release GIL:**
- `IntuitionEngine.with_defaults()` - GIL released during initialization
- `IntuitionEngine.create()` - GIL released during builder pattern
- `IntuitionEngine.stats()` - GIL released during lock acquisition
- `IntuitionEngine.process()` - GIL released during processing

**Example:**
```rust
#[staticmethod]
pub fn with_defaults(py: Python) -> PyResult<Self> {
    py.allow_threads(|| {
        // Complex initialization
        // Python threads don't block
        Ok(Self { engine: Arc::new(Mutex::new(IntuitionEngine::new())) })
    })
}
```

### Преимущества

**Multi-threading:**
```python
import threading
import neurograph

def worker():
    # Each thread can run concurrently
    tokens = neurograph.Token.create_batch(100_000)
    engine = neurograph.IntuitionEngine.with_defaults()
    stats = engine.stats()

threads = [threading.Thread(target=worker) for _ in range(4)]
for t in threads: t.start()
for t in threads: t.join()
```

**Web servers:**
```python
from flask import Flask
import neurograph

app = Flask(__name__)

@app.route("/create")
def create_tokens():
    # GIL released during creation
    # Other requests processed concurrently
    tokens = neurograph.Token.create_batch(1_000_000)
    return {"count": len(tokens)}
```

### Производительность

- **Zero overhead** - same Rust code, just GIL management
- **True parallelism** - multiple Python threads run concurrently
- **Better CPU utilization** - multi-core systems fully utilized

---

## 💾 Part 3: WAL (Write-Ahead Log) v1.0

### Новые возможности

#### WAL Module (`src/wal.rs`)
- **414 LOC** модуль для data persistence
- **Binary format** - compact append-only log
- **CRC32 checksums** - data integrity verification
- **Crash recovery** - replay mechanism

#### Binary Format

**Entry structure:**
```
[Header: 24 bytes] [Payload: N bytes] [CRC32: 4 bytes]
```

**Header layout:**
```rust
pub struct WalEntryHeader {
    pub timestamp_us: u64,        // 8 bytes - Unix timestamp (microseconds)
    pub entry_type: WalEntryType, // 1 byte - entry type tag
    pub payload_size: u32,        // 4 bytes - payload length
    pub sequence: u64,            // 8 bytes - monotonic sequence number
    pub flags: u8,                // 1 byte - future use
    pub _padding: [u8; 2],        // 2 bytes - alignment padding
}
```

#### Entry Types

```rust
pub enum WalEntryType {
    TokenCreated = 1,      // Token was created
    ExperienceAdded = 2,   // Experience added to token
    ConnectionUpdated = 3, // Connection weight updated
    Snapshot = 4,          // Full state snapshot
}
```

#### WAL Writer API

**Create and append:**
```rust
use neurograph_core::wal::{WalWriter, WalEntry, WalEntryType};

// Open WAL file
let mut writer = WalWriter::open("data.wal")?;

// Create entry
let entry = WalEntry::new(
    WalEntryType::TokenCreated,
    b"token_data".to_vec()
);

// Append (with CRC32 checksum)
writer.append(&entry)?;

// Stats
let stats = writer.stats();
println!("Written: {} entries, {} bytes", stats.entries_written, stats.bytes_written);
```

#### WAL Reader API

**Replay log:**
```rust
use neurograph_core::wal::{WalReader, WalEntry};

// Open for reading
let mut reader = WalReader::open("data.wal")?;

// Replay all entries
let count = reader.replay(|entry| {
    match entry.header.entry_type {
        WalEntryType::TokenCreated => {
            // Restore token from payload
            println!("Restoring token...");
        }
        WalEntryType::Snapshot => {
            // Restore full state
            println!("Restoring snapshot...");
        }
        _ => {}
    }
    Ok(())
})?;

println!("Replayed {} entries", count);
```

#### Entry Creation Helpers

```rust
// Token creation
let entry = WalEntry::token_created(token_id, serde_json::to_vec(&token_data)?);

// Experience added
let entry = WalEntry::experience_added(token_id, experience_data);

// Connection updated
let entry = WalEntry::connection_updated(from_id, to_id, new_weight);

// Snapshot
let entry = WalEntry::snapshot(full_state_bytes);
```

### Технические детали

**Durability guarantees:**
- `fsync()` called on **Snapshot** entries only (performance vs durability trade-off)
- Regular entries buffered by OS (faster writes)
- Configurable sync policy

**CRC32 verification:**
```rust
pub fn verify_checksum(&self) -> bool {
    let expected = self.checksum;
    let actual = self.calculate_checksum();
    expected == actual
}
```

**Replay mechanism:**
```rust
pub fn replay<F>(&mut self, mut callback: F) -> Result<usize, WalError>
where F: FnMut(&WalEntry) -> Result<(), WalError>
{
    self.file.seek(SeekFrom::Start(0))?;

    let mut count = 0;
    while let Some(entry) = self.read_entry()? {
        // Verify checksum
        if !entry.verify_checksum() {
            return Err(WalError::ChecksumMismatch);
        }

        // Replay
        callback(&entry)?;
        count += 1;
    }

    Ok(count)
}
```

#### Тестирование

- Unit tests в `src/wal.rs` (3 test cases)
- Tests: write/read, replay, checksum verification
- ✅ Все тесты проходят

---

## 🛡️ Part 4: Guardian Resource Quotas v1.0

### Новые возможности

#### Resource Quotas в Guardian

**Purpose:** Prevent OOM (Out-Of-Memory) and resource exhaustion in production

**Default limits:**
- **Token quota:** 10,000,000 tokens (≈ 640MB estimated)
- **Memory quota:** 1GB (1,073,741,824 bytes)
- **Cleanup threshold:** 80% of quota

#### ResourceStats Structure

```rust
pub struct ResourceStats {
    pub tokens_created: usize,
    pub connections_created: usize,
    pub tokens_removed: usize,
    pub connections_removed: usize,
    pub memory_used_bytes: usize,
    pub quota_exceeded_count: usize,
}
```

#### Quota Check API

**Token creation:**
```rust
let mut guardian = Guardian::new();

if guardian.can_create_token() {
    // Safe to create
    let token = Token::new();
    guardian.record_token_created();
} else {
    // Quota exceeded
    eprintln!("Token quota exceeded!");
}
```

**Connection creation:**
```rust
if guardian.can_create_connection() {
    // Safe to create
    let conn = Connection::new(from, to, weight);
    guardian.record_connection_created();
} else {
    eprintln!("Connection quota exceeded!");
}
```

#### Memory Monitoring

**Linux (accurate):**
```rust
// Reads /proc/self/status -> VmRSS
let memory = guardian.get_current_memory_usage();
```

**Fallback (estimation):**
```rust
// 64 bytes per token + 64 bytes per connection
let estimated = (tokens * 64) + (connections * 64);
```

#### Aggressive Cleanup Detection

```rust
if guardian.should_trigger_aggressive_cleanup() {
    // Usage > 80% of quota
    warn!("Triggering aggressive cleanup!");
    cleanup_old_data();
}
```

#### Resource Statistics

```rust
let stats = guardian.resource_stats();
println!("Tokens: {}", stats.tokens_created);
println!("Memory: {} bytes", stats.memory_used_bytes);
println!("Quota exceeded: {} times", stats.quota_exceeded_count);
```

### Технические детали

**Guardian initialization:**
```rust
// Unlimited quotas
let guardian = Guardian::new();

// Custom quotas
let guardian = Guardian::with_config(GuardianConfig {
    max_tokens: Some(5_000_000),
    max_memory_bytes: Some(512 * 1024 * 1024), // 512MB
    aggressive_cleanup_threshold: 0.75, // 75%
    ..Default::default()
});
```

**Platform-specific memory:**
```rust
#[cfg(target_os = "linux")]
fn read_memory_from_proc() -> Option<usize> {
    let status = std::fs::read_to_string("/proc/self/status").ok()?;
    // Parse VmRSS line
    // ...
}
```

#### Тестирование

- Unit tests в `src/guardian.rs` (9 test cases)
- Tests: unlimited quotas, token quota enforcement, memory quota, cleanup triggers, stats
- ✅ Все тесты проходят

---

## 🔧 Изменённые файлы

### Новые модули
- `src/core_rust/src/panic_handler.rs` - Panic recovery (267 LOC)
- `src/core_rust/src/wal.rs` - Write-Ahead Log (414 LOC)

### Изменённые модули
- `src/core_rust/src/guardian.rs`:
  - Added ResourceStats struct
  - Added 8 resource quota methods
  - Added 9 resource quota tests
  - Platform-specific memory monitoring
  - Memory estimation fallback
- `src/core_rust/src/action_controller.rs`:
  - Added execute_intent_safe() with panic recovery
- `src/core_rust/src/action_executor.rs`:
  - Added ActionError::PanicRecovered variant
- `src/core_rust/src/python/token.rs`:
  - Added GIL release to create_batch()
- `src/core_rust/src/python/intuition.rs`:
  - Added GIL release to all methods
- `src/core_rust/src/bin/repl.rs`:
  - Installed panic hook in main()
- `src/core_rust/src/bin/api.rs`:
  - Installed panic hook in main()
- `src/core_rust/src/adapters/console.rs`:
  - Fixed import paths (GatewayConfig, SignalType)

### Зависимости
```toml
# Cargo.toml
crc32fast = "1.4"     # WAL checksums
tempfile = "3.8"      # Testing (dev)
```

### Экспорты
- `src/core_rust/src/lib.rs`:
  - Added pub mod panic_handler
  - Added pub mod wal
  - Exported WalEntry, WalWriter, WalReader, WalError, etc.

### Документация
- `README.md` - обновлён до v0.41.0
- `python/README.md` - обновлён production roadmap
- `docs/changelogs/CHANGELOG_v0.41.0.md` - этот файл

---

## 📈 Production Benefits

### До v0.41.0:
- ❌ Single panic crashes entire process
- ❌ Python threads block during Rust operations
- ❌ No data persistence
- ❌ No OOM protection

### После v0.41.0:
- ✅ Panics caught and logged gracefully
- ✅ Python threads run concurrently (true parallelism)
- ✅ WAL provides crash recovery
- ✅ Resource quotas prevent OOM
- ✅ Production-ready error handling

### Use Cases

**Web Applications:**
```python
from flask import Flask
import neurograph

app = Flask(__name__)

@app.route("/batch")
def create_batch():
    # GIL released - other requests don't block
    tokens = neurograph.Token.create_batch(1_000_000)
    return {"count": len(tokens)}

@app.route("/stats")
def get_stats():
    # Runs concurrently with /batch
    engine = neurograph.IntuitionEngine.with_defaults()
    return engine.stats()
```

**Data Persistence:**
```rust
// Persist operations to WAL
let mut wal = WalWriter::open("neurograph.wal")?;

for token in tokens {
    let entry = WalEntry::token_created(token.id, serde_json::to_vec(&token)?);
    wal.append(&entry)?;
}

// After crash - replay
let mut reader = WalReader::open("neurograph.wal")?;
reader.replay(|entry| {
    // Restore state
    restore_from_entry(entry)?;
    Ok(())
})?;
```

**Resource Protection:**
```rust
let mut guardian = Guardian::with_config(GuardianConfig {
    max_tokens: Some(10_000_000),
    max_memory_bytes: Some(1024 * 1024 * 1024), // 1GB
    ..Default::default()
});

loop {
    if guardian.can_create_token() {
        let token = Token::new();
        guardian.record_token_created();
    } else {
        warn!("Token quota exceeded!");
        break;
    }

    if guardian.should_trigger_aggressive_cleanup() {
        cleanup_old_tokens();
    }
}
```

---

## 🧪 Тестирование

### Build Tests

```bash
# Core library
$ cargo build --lib --release
   Finished `release` profile [optimized] in 39.50s ✅

# With Python bindings
$ cargo build --features python
   Finished `dev` profile in 12.44s ✅
```

### Unit Tests

```bash
# Panic handler tests
cargo test --lib panic_handler::tests

# WAL tests
cargo test --lib wal::tests

# Guardian resource quota tests
cargo test --lib guardian::tests::test_token_quota
cargo test --lib guardian::tests::test_memory_quota
```

### Integration Tests

**Panic recovery:**
```rust
let result = catch_panic("test", || {
    panic!("intentional panic");
});
assert!(result.is_err());
```

**WAL replay:**
```rust
let mut writer = WalWriter::open("test.wal")?;
writer.append(&entry)?;

let mut reader = WalReader::open("test.wal")?;
let count = reader.replay(|_| Ok(()))?;
assert_eq!(count, 1);
```

---

## 📊 Статистика изменений

### Lines of Code

- **Panic handler module:** 267 LOC
- **WAL module:** 414 LOC
- **Guardian resource quotas:** 361 LOC (added)
- **Python GIL release:** ~50 LOC (modified)
- **Integration:** ~100 LOC
- **Tests:** ~200 LOC

**Total:** ~1400 LOC добавлено/изменено

### Files Changed

- **New files:** 2 (panic_handler.rs, wal.rs)
- **Modified files:** 10 (guardian, action_controller, python bindings, etc.)
- **Documentation:** 3 files (README.md, python/README.md, changelog)

### Commits

1. `3d29847` - feat: Implement v0.41.0-rc1 Reliability (Panic Recovery + GIL Release)
2. `c4c167d` - feat: Implement v0.41.0 Final - WAL + Resource Quotas
3. `b2aef63` - docs: Update documentation for v0.41.0 Final release

---

## 🚀 Roadmap Updates

### Completed Milestones

- ✅ **v0.40.0** - Python Bindings (PyO3)
- ✅ **v0.41.0** - Reliability (Panic Recovery, WAL, Resource Quotas) ← **WE ARE HERE**

### Next Milestones

- ⏳ **v0.42.0** - Observability (Prometheus, Black Box, Logging)
- ⏳ **v0.43.0** - Docker Deployment
- ⏳ **v0.44.0** - Distributed Tracing

---

## 💡 Migration Guide

### Для существующих проектов

1. **Обновите зависимости:**
```toml
[dependencies]
neurograph-core = "0.41.0"
```

2. **Установите panic hook:**
```rust
use neurograph_core::panic_handler::install_panic_hook;

fn main() {
    install_panic_hook();
    // ... rest of your code
}
```

3. **Используйте safe execution:**
```rust
// Вместо:
let result = controller.execute_intent(intent).await?;

// Используйте:
let result = controller.execute_intent_safe(intent).await?;
match result {
    Ok(action_result) => { /* success */ },
    Err(ActionError::PanicRecovered(msg)) => {
        eprintln!("Recovered from panic: {}", msg);
    },
    Err(e) => { /* other errors */ },
}
```

4. **Настройте WAL:**
```rust
use neurograph_core::wal::{WalWriter, WalEntry};

let mut wal = WalWriter::open("neurograph.wal")?;

// Persist operations
wal.append(&WalEntry::token_created(id, data))?;
```

5. **Настройте resource quotas:**
```rust
use neurograph_core::guardian::{Guardian, GuardianConfig};

let guardian = Guardian::with_config(GuardianConfig {
    max_tokens: Some(10_000_000),
    max_memory_bytes: Some(1024 * 1024 * 1024), // 1GB
    aggressive_cleanup_threshold: 0.8,
    ..Default::default()
});
```

### Backwards Compatibility

- ✅ Полная обратная совместимость с v0.40.0
- ✅ GIL release прозрачен для Python кода
- ✅ Panic recovery опционален
- ✅ WAL и quotas опциональны
- ✅ Zero breaking changes

---

## 🎯 Known Issues

Нет известных проблем в v0.41.0.

---

## 👥 Contributors

- Chernov Denys (@dchrnv) - lead developer
- Claude (Anthropic) - code generation assistant

---

## 📜 License

AGPL-3.0 - Copyright (C) 2024-2025 Chernov Denys

---

**v0.41.0 Final** - Production-Ready Core! 🎉
