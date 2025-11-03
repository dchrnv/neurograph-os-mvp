# ADNA v1.0 MVP — Adaptive DNA (Static Policy Edition)

**Версия:** 1.0.0 (MVP)
**Дата:** 2025-11-02
**Статус:** ✅ Реализовано (v0.23.0) + 4 Appraisers (v0.25.0)
**Зависимости:** CDNA v2.1, Guardian v1.1, Token v2.0, Connection v1.0, ExperienceStream v2.0
**Размер:** 256 байт (фиксированный)
**Цель:** Базовая инфраструктура для статических политик и параметров системы + Reward System

---

## 📝 Implementation Notes

### v0.25.0 - 4 Appraisers (Reward System)

**Полная реализация Reward System для KEY Architecture:**

Все 4 Appraiser'а из ADNA parameters теперь полностью реализованы:

1. **HomeostasisAppraiser** (`homeostasis_weight`)
   - Квадратичный штраф за отклонение от целевых параметров
   - Cognitive Load target: [0.3, 0.7] (L4, index 3)
   - Certainty target: [0.5, 0.9] (L6, index 5)
   - Формула: `penalty = -k * deviation²`
   - 10 unit tests

2. **CuriosityAppraiser** (`curiosity_weight`)
   - Линейная награда за новизну
   - Novelty (L2, index 1)
   - Формула: `reward = k * novelty`
   - 9 unit tests

3. **EfficiencyAppraiser** (`efficiency_weight`)
   - Линейный штраф за затраты энергии
   - Energy cost (L7, index 6)
   - Формула: `penalty = -k * energy_cost`
   - 9 unit tests

4. **GoalDirectedAppraiser** (`goal_weight`)
   - Линейная награда за прогресс к цели
   - Goal progress (L8, index 7)
   - Формула: `reward = k * goal_progress`
   - 9 unit tests

**AppraisersManager:**
- Координирует все 4 appraiser'а
- Weighted sum: `reward = Σ(component_i * weight_i)`
- Веса берутся из ADNA parameters
- 3 integration tests

**Архитектура:**
```rust
pub trait Appraiser: Send + Sync {
    fn calculate_reward(&self, event: &ExperienceEvent, adna: &ADNA) -> f32;
    fn name(&self) -> &str;
    fn weight(&self, adna: &ADNA) -> f32;
}

pub struct AppraisersManager {
    homeostasis: HomeostasisAppraiser,
    curiosity: CuriosityAppraiser,
    efficiency: EfficiencyAppraiser,
    goal_directed: GoalDirectedAppraiser,
}
```

**Использование:**
```rust
let manager = AppraisersManager::new();
let adna = ADNA::from_profile(ADNAProfile::Balanced);

let mut event = ExperienceEvent::new(EventType::ActionExecuted)
    .with_state([0.5, 0.8, 0.3, 0.6, 0.4, 0.7, 0.2, 0.9]);

// Appraiser manager вычисляет reward на основе ADNA весов
manager.appraise_event(&mut event, &adna);
// event.reward теперь содержит weighted sum всех компонентов
```

**Файлы:**
- `src/core_rust/src/appraisers/mod.rs` (204 lines) - trait + manager
- `src/core_rust/src/appraisers/homeostasis.rs` (242 lines)
- `src/core_rust/src/appraisers/curiosity.rs` (170 lines)
- `src/core_rust/src/appraisers/efficiency.rs` (173 lines)
- `src/core_rust/src/appraisers/goal_directed.rs` (186 lines)

**Тестирование:**
- 37 unit tests для appraisers (100% coverage)
- 126 total tests passing в core_rust

**Интеграция с ADNA:**
- Веса из `ADNAParameters` напрямую используются в `Appraiser::weight()`
- Разные профили (Balanced, Cautious, Curious, Adaptive) дают разные rewards
- События оцениваются с точки зрения 4 разных "мотиваций" системы

---

### v0.24.0 - Guardian v1.1 (ADNA Integration)

**Полная интеграция ADNA в систему валидации:**

Guardian v1.1 расширяет Guardian v1.0 поддержкой ADNA lifecycle management:

**Новые возможности:**

1. **ADNA Loading & Validation:**
   ```rust
   pub fn load_adna(&mut self, adna: ADNA) -> Result<(), String> {
       // 1. Validate ADNA structure
       adna.validate()?;

       // 2. Validate against CDNA constraints
       self.validate_adna_against_cdna(&adna)?;

       // 3. Store old ADNA in history
       if let Some(current) = self.adna.take() {
           self.adna_history.push_back(current);
       }

       // 4. Load new ADNA
       self.adna = Some(adna);

       // 5. Emit event
       self.emit_event(Event::new(EventType::ADNALoaded));
       Ok(())
   }
   ```

2. **ADNA Parameter Updates:**
   ```rust
   pub fn update_adna_parameter(&mut self, param_name: &str, value: f32) -> Result<(), String> {
       let current = self.adna.as_ref().ok_or("No ADNA loaded")?;

       // Create evolved version with updated parameter
       let mut new_adna = current.evolve();
       match param_name {
           "homeostasis_weight" => new_adna.parameters.homeostasis_weight = value,
           "curiosity_weight" => new_adna.parameters.curiosity_weight = value,
           "efficiency_weight" => new_adna.parameters.efficiency_weight = value,
           "goal_weight" => new_adna.parameters.goal_weight = value,
           "exploration_rate" => new_adna.parameters.exploration_rate = value,
           _ => return Err(format!("Unknown parameter: {}", param_name)),
       }

       // Validate and store
       new_adna.update_hash();
       new_adna.validate()?;
       self.validate_adna_against_cdna(&new_adna)?;

       let old = self.adna.replace(new_adna);
       if let Some(old) = old {
           self.adna_history.push_back(old);
       }

       self.emit_event(Event::new(EventType::ADNAUpdated));
       Ok(())
   }
   ```

3. **ADNA Constitutional Validation:**
   ```rust
   fn validate_adna_against_cdna(&self, adna: &ADNA) -> Result<(), String> {
       // Check all weights in [0.0, 1.0]
       if adna.parameters.homeostasis_weight < 0.0 ||
          adna.parameters.homeostasis_weight > 1.0 {
           return Err("homeostasis_weight out of range");
       }

       // Check decision_timeout in [1ms, 10s]
       if adna.parameters.decision_timeout_ms == 0 ||
          adna.parameters.decision_timeout_ms > 10000 {
           return Err("decision_timeout_ms out of range");
       }

       // Check max_actions_per_cycle in [1, 1000]
       if adna.parameters.max_actions_per_cycle == 0 ||
          adna.parameters.max_actions_per_cycle > 1000 {
           return Err("max_actions_per_cycle out of range");
       }

       Ok(())
   }
   ```

4. **ADNA History & Rollback:**
   ```rust
   pub fn rollback_adna(&mut self) -> Result<(), String> {
       let previous = self.adna_history.pop_back()
           .ok_or("No ADNA history available")?;

       if let Some(current) = self.adna.replace(previous) {
           // Don't add rolled-back version to history
       }

       self.emit_event(Event::new(EventType::ADNARolledBack));
       Ok(())
   }
   ```

**Новые Event Types:**
- `ADNALoaded` (0x0011) - ADNA successfully loaded
- `ADNAUpdated` (0x0012) - ADNA parameter updated
- `ADNARolledBack` (0x0013) - ADNA rolled back to previous version

**Структура Guardian:**
```rust
pub struct Guardian {
    cdna: CDNA,
    cdna_history: VecDeque<CDNA>,
    adna: Option<ADNA>,              // NEW in v1.1
    adna_history: VecDeque<ADNA>,    // NEW in v1.1
    config: GuardianConfig,
    event_queue: Vec<Event>,
    subscriptions: HashMap<String, Vec<EventType>>,
    validation_stats: ValidationStats,
}
```

**Интеграция с ADNA.evolve():**
- При обновлении параметров Guardian вызывает `adna.evolve()`
- Это создает новую версию ADNA с:
  - Incremented generation counter
  - Parent hash = current parameters hash
  - Updated modification timestamp

**Тестирование:**
- 9 integration tests для ADNA lifecycle
- Tests for load, update, rollback, validation
- Tests for event emission
- Tests for history management
- 89 total tests passing (включая ADNA tests)

**Файлы:**
- `src/core_rust/src/guardian.rs` - updated with ADNA support
- `src/core_rust/src/adna.rs` - added `evolve()` method

**Архитектурный смысл:**
Guardian v1.1 делает ADNA "first-class citizen":
- ADNA проходит те же проверки, что и CDNA
- ADNA изменения всегда валидируются
- ADNA история позволяет откатываться к stable configurations
- Events позволяют модулям реагировать на ADNA changes

---

### v0.23.0 - ADNA Structure Implementation

**Оптимизации структуры для точного размера 256 байт:**

1. **ADNAHeader (64 bytes):**
   - ❌ Removed `current_hash: [u8; 32]` field (saves 32 bytes)
   - ✅ Current hash computed on-demand via `compute_fnv1a_hash()` (~50ns)
   - ✅ `parent_hash[0..8]` reused for storing current parameters hash
   - ✅ Removed `#[repr(C, align(64))]` → `#[repr(C)]` for precise 64-byte size

2. **EvolutionMetrics (64 bytes):**
   - ✅ `_reserved` increased from 28 to 36 bytes for exact 64-byte alignment

3. **PolicyPointer (64 bytes):**
   - ✅ Reordered fields: u64 first, then u32, then u8 (minimizes padding)
   - ✅ `_reserved2` adjusted to 40 bytes for exact 64-byte size

4. **Hashing:**
   - ✅ FNV-1a instead of SHA256 (zero dependencies, ~50ns performance)
   - ✅ Only hashes `parameters` block (64 bytes) for version tracking

**Trade-offs:**

- ✅ Zero external dependencies for ADNA module
- ✅ Cache-friendly (exactly 4 × 64-byte cache lines)
- ✅ Fast hash computation (50ns vs ~1μs for SHA256)
- ❌ No pre-computed current_hash (computed on-demand when needed)

---

## 1. Философия MVP

### 1.1 Упрощения относительно ADNA v3.0

ADNA v1.0 MVP — это **подмножество** полной спецификации ADNA v3.0, адаптированное для поэтапной реализации:

- ✅ **Сохранено:** Структура 256 байт, версионирование, валидация через Guardian
- ❌ **Отложено:** Gradient updates, reinforcement learning, neural policies
- 🔄 **Упрощено:** Политики хранятся как статические JSON/TOML конфигурации

### 1.2 Основные принципы

- **Статические политики:** Правила изменяются только вручную или через Guardian API
- **Полная валидация:** Все изменения проверяются CDNA (конституционные ограничения)
- **Версионирование:** Каждое изменение создаёт новую версию с parent_hash
- **Читаемость:** Политики хранятся в человекочитаемом формате
- **Фундамент для эволюции:** Структура готова к расширению до ADNA v2.0/v3.0

### 1.3 Роль в системе

```
┌──────────────────────────────────────┐
│          Guardian v1.1               │
│  (оркестрирует изменения ADNA)       │
└──────────────────────────────────────┘
                ↓ update/validate
┌──────────────────────────────────────┐
│          ADNA v1.0 MVP               │
│  (статические параметры системы)     │
└──────────────────────────────────────┘
                ↓ validates
┌──────────────────────────────────────┐
│          CDNA v2.1                   │
│  (конституционные правила)           │
└──────────────────────────────────────┘
```

---

## 2. Бинарная структура (256 байт)

### 2.1 Общий layout

```
Offset | Size  | Field
-------|-------|------------------
0-63   | 64    | Header Block
64-127 | 64    | Evolution Metrics Block
128-191| 64    | Policy Pointer Block
192-255| 64    | Parameters Block
-------|-------|------------------
TOTAL  | 256   | bytes (cache-aligned)
```

### 2.2 Header Block (64 bytes)

```rust
#[repr(C)]
pub struct ADNAHeader {
    /// Magic number 'ADNA' (0x41444E41)
    pub magic: u32,

    /// Version (major.minor)
    pub version_major: u16,
    pub version_minor: u16,

    /// Policy type enum
    pub policy_type: u16,  // 0 = StaticRules, 1-255 reserved

    /// Reserved for alignment
    pub _reserved1: u16,

    /// Creation timestamp (Unix epoch seconds)
    pub created_at: u64,

    /// Last modification timestamp
    pub modified_at: u64,

    /// FNV-1a hash of parent ADNA version (for lineage tracking)
    /// First 8 bytes also store current parameters hash
    pub parent_hash: [u8; 32],
}
```

**Changes from initial design:**
- ❌ Removed `current_hash` field (32 bytes saved)
- ✅ Current hash computed on-demand via FNV-1a (~50ns)
- ✅ `parent_hash` reused for lineage + version tracking
- ✅ Removed `align(64)` to allow precise 64-byte size

**Policy Types:**
- `0x0000` - StaticRules (JSON/TOML конфигурация)
- `0x0001-0xFFFF` - Reserved для будущих типов (Neural, Tree, Hybrid)

### 2.3 Evolution Metrics Block (64 bytes)

```rust
pub struct EvolutionMetrics {
    /// Generation number (increments on each update)
    pub generation: u32,

    /// Manual quality score (0.0 - 1.0)
    pub fitness_score: f32,

    /// Confidence in current configuration (0.0 - 1.0)
    pub confidence: f32,

    /// Reserved for future learning rate
    pub learning_rate: f32,

    /// Number of times this ADNA was active
    pub activation_count: u32,

    /// Reserved for future success tracking
    pub success_rate: f32,

    /// Reserved for rollback tracking
    pub rollback_count: u32,

    /// Reserved for future use (total: 4+4+4+4+4+4+4 = 28, need 36 more for 64)
    pub _reserved: [u8; 36],
}
```

**MVP поля:**
- `fitness_score` - устанавливается вручную администратором
- `confidence` - может отражать стабильность системы
- Остальные поля зарезервированы для ADNA v2.0+

### 2.4 Policy Pointer Block (64 bytes)

```rust
#[repr(C)]
pub struct PolicyPointer {
    /// File path hash (FNV-1a for identification)
    pub policy_path_hash: u64,

    /// Checksum of policy file (FNV-1a)
    pub policy_checksum: u64,

    /// Size of external policy file (bytes)
    pub policy_size: u32,

    /// Compression type (0 = none, 1 = LZ4, 2 = Zstd)
    pub compression_type: u8,

    /// Encryption flag (0 = none, 1 = AES-256)
    pub encryption_flag: u8,

    /// Cache strategy (0 = always, 1 = on-demand)
    pub cache_strategy: u8,

    /// Reserved
    pub _reserved1: u8,

    /// Reserved for future (8+8+4+1+1+1+1 = 24, need 40 more for 64)
    pub _reserved2: [u8; 40],
}
```

**Field ordering:**
- ✅ u64 fields first (policy_path_hash, policy_checksum)
- ✅ u32 field next (policy_size)
- ✅ u8 fields last (compression_type, encryption_flag, cache_strategy, _reserved1)
- ✅ Padding adjusted to 40 bytes for exact 64-byte size

**MVP:** Политики хранятся в отдельных файлах (JSON/TOML), referenced by hash.

### 2.5 Parameters Block (64 bytes)

```rust
pub struct ADNAParameters {
    // === Appraiser Weights (16 bytes) ===
    /// Weight for HomeostasisAppraiser (0.0 - 1.0)
    pub homeostasis_weight: f32,

    /// Weight for CuriosityAppraiser (0.0 - 1.0)
    pub curiosity_weight: f32,

    /// Weight for EfficiencyAppraiser (0.0 - 1.0)
    pub efficiency_weight: f32,

    /// Weight for GoalDirectedAppraiser (0.0 - 1.0)
    pub goal_weight: f32,

    // === System Behavior (16 bytes) ===
    /// Exploration rate (0.0 = exploit, 1.0 = explore)
    pub exploration_rate: f32,

    /// Decision timeout (milliseconds)
    pub decision_timeout_ms: u32,

    /// Max actions per cycle
    pub max_actions_per_cycle: u32,

    /// Reserved
    pub _reserved1: u32,

    // === Reserved for future (32 bytes) ===
    pub _reserved2: [u8; 32],
}
```

---

## 3. Policy File Format (External Storage)

### 3.1 JSON Schema Example

```json
{
  "adna_version": "1.0.0",
  "policy_type": "static_rules",
  "created_at": "2025-11-02T12:00:00Z",

  "appraisers": {
    "homeostasis": {
      "cognitive_load_target": [0.3, 0.7],
      "certainty_target": [0.5, 0.9],
      "novelty_threshold": 0.1
    },
    "curiosity": {
      "novelty_threshold": 0.2,
      "reward_scale": 1.0
    },
    "efficiency": {
      "energy_budget": 1000.0,
      "penalty_factor": 0.5
    },
    "goal_directed": {
      "task_completion_bonus": 10.0
    }
  },

  "action_policies": {
    "generate_code": {
      "executor_id": "code_generator",
      "parameters": {
        "style": "functional",
        "max_lines": 100
      },
      "priority": 0.8
    },
    "answer_question": {
      "executor_id": "text_generator",
      "parameters": {
        "max_tokens": 500
      },
      "priority": 0.9
    }
  },

  "constraints": {
    "max_token_creation_rate": 1000,
    "max_connection_creation_rate": 5000,
    "min_system_stability": 0.7
  }
}
```

### 3.2 File Location Convention

```
data/adna/
├── current.json          # Symlink to active policy
├── v001_baseline.json    # Generation 1
├── v002_tuned.json       # Generation 2
└── archive/
    └── v000_default.json # Initial baseline
```

---

## 4. API и интеграция с Guardian

### 4.1 Rust API

```rust
// src/core_rust/src/adna.rs

use std::path::Path;

/// ADNA magic number
pub const ADNA_MAGIC: u32 = 0x41444E41; // "ADNA"

/// ADNA version
pub const ADNA_VERSION_MAJOR: u16 = 1;
pub const ADNA_VERSION_MINOR: u16 = 0;

/// Policy type
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyType {
    StaticRules = 0,
    // Future: Neural = 1, Tree = 2, Hybrid = 3
}

/// Complete ADNA structure (256 bytes, cache-aligned)
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct ADNA {
    pub header: ADNAHeader,           // 64 bytes (offset 0-63)
    pub metrics: EvolutionMetrics,    // 64 bytes (offset 64-127)
    pub pointer: PolicyPointer,       // 64 bytes (offset 128-191)
    pub parameters: ADNAParameters,   // 64 bytes (offset 192-255)
}

// Compile-time size check
const _: () = assert!(std::mem::size_of::<ADNA>() == 256);

impl ADNA {
    /// Create new ADNA with default parameters
    pub fn new() -> Self {
        Self {
            header: ADNAHeader::default(),
            metrics: EvolutionMetrics::default(),
            pointer: PolicyPointer::default(),
            parameters: ADNAParameters::default(),
        }
    }

    /// Load ADNA from binary + external policy
    pub fn load(binary_path: &Path, policy_path: &Path) -> Result<(Self, PolicyData), ADNAError> {
        // 1. Load 256-byte core
        let bytes = std::fs::read(binary_path)?;
        let adna = unsafe { std::ptr::read(bytes.as_ptr() as *const ADNA) };

        // 2. Validate
        adna.validate()?;

        // 3. Load external policy
        let policy = PolicyData::load(policy_path)?;

        // 4. Verify checksum
        if policy.compute_checksum() != adna.pointer.policy_checksum {
            return Err(ADNAError::PolicyChecksumMismatch);
        }

        Ok((adna, policy))
    }

    /// Save ADNA to disk
    pub fn save(&self, binary_path: &Path) -> Result<(), ADNAError> {
        self.validate()?;
        let bytes = unsafe {
            std::slice::from_raw_parts(self as *const ADNA as *const u8, 256)
        };
        std::fs::write(binary_path, bytes)?;
        Ok(())
    }

    /// Compute FNV-1a hash of parameters (for version tracking)
    pub fn compute_hash(&self) -> u64 {
        const FNV_OFFSET: u64 = 14695981039346656037;
        const FNV_PRIME: u64 = 1099511628211;

        let mut hash = FNV_OFFSET;

        // Hash parameters block (64 bytes at offset 192)
        let params_bytes = unsafe {
            std::slice::from_raw_parts(
                &self.parameters as *const ADNAParameters as *const u8,
                64
            )
        };

        for &byte in params_bytes {
            hash ^= byte as u64;
            hash = hash.wrapping_mul(FNV_PRIME);
        }

        hash
    }

    /// Update modification timestamp and parent hash
    pub fn update_hash(&mut self) {
        let hash = self.compute_hash();
        self.header.parent_hash[0..8].copy_from_slice(&hash.to_le_bytes());
        self.header.modified_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
    }

    /// Validate ADNA structure
    pub fn validate(&self) -> Result<(), ADNAError> {
        // Check magic
        if self.header.magic != ADNA_MAGIC {
            return Err(ADNAError::InvalidMagic(self.header.magic));
        }

        // Check version
        if self.header.version_major != ADNA_VERSION_MAJOR {
            return Err(ADNAError::UnsupportedVersion(
                self.header.version_major,
                self.header.version_minor,
            ));
        }

        // Validate parameters
        self.parameters.validate()?;

        Ok(())
    }

    /// Create new version based on current
    pub fn evolve(&self) -> Self {
        let mut new_adna = *self;

        // Store current hash in parent_hash for lineage tracking
        let current_hash = self.compute_hash();
        new_adna.header.parent_hash[0..8].copy_from_slice(&current_hash.to_le_bytes());

        new_adna.metrics.generation += 1;
        new_adna.header.modified_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        new_adna
    }
}

impl ADNAParameters {
    pub fn validate(&self) -> Result<(), ADNAError> {
        // Check weights in [0.0, 1.0]
        if self.homeostasis_weight < 0.0 || self.homeostasis_weight > 1.0 {
            return Err(ADNAError::InvalidParameter("homeostasis_weight"));
        }
        if self.curiosity_weight < 0.0 || self.curiosity_weight > 1.0 {
            return Err(ADNAError::InvalidParameter("curiosity_weight"));
        }
        if self.efficiency_weight < 0.0 || self.efficiency_weight > 1.0 {
            return Err(ADNAError::InvalidParameter("efficiency_weight"));
        }
        if self.goal_weight < 0.0 || self.goal_weight > 1.0 {
            return Err(ADNAError::InvalidParameter("goal_weight"));
        }
        if self.exploration_rate < 0.0 || self.exploration_rate > 1.0 {
            return Err(ADNAError::InvalidParameter("exploration_rate"));
        }
        Ok(())
    }
}

/// External policy data (JSON/TOML)
pub struct PolicyData {
    pub raw: String,
    pub parsed: serde_json::Value,
}

impl PolicyData {
    pub fn load(path: &Path) -> Result<Self, ADNAError> {
        let raw = std::fs::read_to_string(path)?;
        let parsed: serde_json::Value = serde_json::from_str(&raw)?;
        Ok(Self { raw, parsed })
    }

    pub fn compute_checksum(&self) -> u64 {
        fnv1a_hash(self.raw.as_bytes())
    }
}
```

### 4.2 Guardian Integration

```rust
// In Guardian v1.1

impl Guardian {
    /// Load ADNA (in addition to CDNA)
    pub fn load_adna(&mut self, binary_path: &Path, policy_path: &Path) -> Result<(), String> {
        let (adna, policy) = ADNA::load(binary_path, policy_path)?;

        // Validate against CDNA
        self.validate_adna_against_cdna(&adna, &policy)?;

        self.adna = Some(adna);
        self.adna_policy = Some(policy);

        Ok(())
    }

    /// Validate ADNA doesn't violate CDNA
    fn validate_adna_against_cdna(&self, adna: &ADNA, policy: &PolicyData) -> Result<(), String> {
        // Example checks:
        // - max_token_creation_rate <= CDNA.max_tokens
        // - appraiser weights sum to reasonable value
        // - action policies respect CDNA constraints
        Ok(())
    }

    /// Update ADNA parameter (creates new version)
    pub fn update_adna_parameter(&mut self, param: &str, value: f32) -> Result<(), String> {
        let mut new_adna = self.adna.as_ref().unwrap().evolve();

        match param {
            "homeostasis_weight" => new_adna.parameters.homeostasis_weight = value,
            "curiosity_weight" => new_adna.parameters.curiosity_weight = value,
            "efficiency_weight" => new_adna.parameters.efficiency_weight = value,
            "goal_weight" => new_adna.parameters.goal_weight = value,
            "exploration_rate" => new_adna.parameters.exploration_rate = value,
            _ => return Err(format!("Unknown parameter: {}", param)),
        }

        // Validate
        new_adna.validate()?;
        self.validate_adna_against_cdna(&new_adna, self.adna_policy.as_ref().unwrap())?;

        // Save history
        self.adna_history.push_back(self.adna.unwrap());
        self.adna = Some(new_adna);

        Ok(())
    }
}
```

---

## 5. Lifecycle и операции

### 5.1 Initialization

```rust
// Bootstrap process
let mut guardian = Guardian::new();

// Load CDNA (constitutional rules)
guardian.load_cdna(CDNA::new());

// Load ADNA (adaptive parameters)
guardian.load_adna(
    Path::new("data/adna/core_v001.bin"),
    Path::new("data/adna/policy_v001.json"),
)?;
```

### 5.2 Manual Update

```rust
// Administrator updates exploration rate
guardian.update_adna_parameter("exploration_rate", 0.3)?;

// Save new version
guardian.adna.as_ref().unwrap().save(Path::new("data/adna/core_v002.bin"))?;
```

### 5.3 Rollback

```rust
// Rollback to previous version
guardian.rollback_adna()?;
```

---

## 6. Ограничения MVP и будущие улучшения

### 6.1 Что НЕ включено в v1.0

- ❌ Gradient-based updates (требует IntuitionEngine)
- ❌ Neural policy networks
- ❌ Automatic evolution
- ❌ Reinforcement learning loop
- ❌ Policy sandboxing
- ❌ A/B testing between policies

### 6.2 Путь к ADNA v2.0

**v1.0 → v1.5 (Semi-automatic):**
- Add simple heuristics для автоматической подстройки весов
- Statistics-based parameter tuning
- Basic fitness tracking

**v1.5 → v2.0 (ML-assisted):**
- Integration с IntuitionEngine
- Proposal system для обновлений
- EvolutionManager для безопасного применения

**v2.0 → v3.0 (Full Policy Engine):**
- Neural policy networks
- Gradient descent updates
- Full reinforcement learning loop

---

## 7. Testing Strategy

### 7.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adna_size() {
        assert_eq!(std::mem::size_of::<ADNA>(), 256);
        assert_eq!(std::mem::align_of::<ADNA>(), 64);
    }

    #[test]
    fn test_adna_creation() {
        let adna = ADNA::new();
        assert_eq!(adna.header.magic, ADNA_MAGIC);
        assert_eq!(adna.header.version_major, 1);
        assert!(adna.validate().is_ok());
    }

    #[test]
    fn test_adna_hash() {
        let adna = ADNA::new();
        let hash1 = adna.compute_hash();

        let mut adna2 = adna;
        adna2.parameters.exploration_rate = 0.5;
        let hash2 = adna2.compute_hash();

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_parameter_validation() {
        let mut adna = ADNA::new();
        adna.parameters.homeostasis_weight = 1.5; // Invalid
        assert!(adna.validate().is_err());
    }

    #[test]
    fn test_evolution() {
        let adna1 = ADNA::new();
        let hash1 = adna1.compute_hash();

        let adna2 = adna1.evolve();
        let hash2 = adna2.compute_hash();

        // Parent hash should contain hash of previous version
        let stored_parent = u64::from_le_bytes(adna2.header.parent_hash[0..8].try_into().unwrap());
        assert_eq!(stored_parent, hash1);

        assert_eq!(adna2.metrics.generation, adna1.metrics.generation + 1);
        assert_eq!(hash1, hash2); // Parameters didn't change, so hash is same
    }
}
```

### 7.2 Integration Tests

```rust
#[test]
fn test_guardian_adna_integration() {
    let mut guardian = Guardian::new();

    // Load ADNA
    guardian.load_adna(
        Path::new("test_data/adna_v001.bin"),
        Path::new("test_data/policy_v001.json"),
    ).unwrap();

    // Update parameter
    guardian.update_adna_parameter("curiosity_weight", 0.8).unwrap();

    // Verify history
    assert_eq!(guardian.adna_history.len(), 1);

    // Rollback
    guardian.rollback_adna().unwrap();
    assert_eq!(guardian.adna.as_ref().unwrap().parameters.curiosity_weight, 0.5);
}
```

---

## 8. Migration Path

### 8.1 От отсутствия ADNA к v1.0

Система работает **без ADNA**:
```rust
// Hardcoded weights
const HOMEOSTASIS_WEIGHT: f32 = 0.5;
const CURIOSITY_WEIGHT: f32 = 0.3;
```

Миграция на **ADNA v1.0**:
```rust
// Load from ADNA
let weights = guardian.adna.as_ref().unwrap().parameters;
let homeostasis_weight = weights.homeostasis_weight;
let curiosity_weight = weights.curiosity_weight;
```

### 8.2 От v1.0 к v2.0

**v1.0:** Ручное обновление через Guardian API
**v2.0:** Автоматические предложения от IntuitionEngine

Код совместим, добавляется новый путь:
```rust
// v1.0 path (manual)
guardian.update_adna_parameter("curiosity_weight", 0.8)?;

// v2.0 path (automatic) - будет добавлено позже
evolution_manager.apply_proposal(proposal)?;
```

---

## 9. Производительность

### 9.1 Memory

- **Core:** 256 bytes (fixed)
- **Policy file:** ~1-10 KB (JSON)
- **History:** 256 bytes × max_history_size

### 9.2 Latency

- **Load:** <1ms (256 byte read + JSON parse)
- **Parameter read:** <100ns (direct struct access)
- **Update:** <1ms (validation + hash computation)
- **Save:** <1ms (256 byte write)

### 9.3 Cache Efficiency

- Single cache line read для header (64 bytes)
- Predictable access patterns
- No dynamic allocation in hot path

---

## 10. Резюме

### 10.1 Deliverables для v0.22.0

1. ✅ ADNA 256-byte структура (Rust)
2. ✅ Load/Save/Validate operations
3. ✅ Guardian integration (v1.1)
4. ✅ JSON policy file format
5. ✅ Version history и rollback
6. ✅ Comprehensive tests

### 10.2 Готовность к расширению

ADNA v1.0 MVP является **solid foundation** для:
- ✅ Phase 2: Reward System (Appraisers) - **DONE v0.25.0**
- 📋 Phase 3: IntuitionEngine integration (v0.26.0+)
- 📋 Phase 4: ActionController integration (v0.27.0+)
- 📋 Phase 5: Full learning loop (v2.0+)

---

## 11. Roadmap & Next Steps

### 11.1 Текущий статус (v0.25.0)

**✅ Завершено:**
- ADNA v1.0 structure (256 bytes)
- Guardian v1.1 integration
- ExperienceStream v2.0 (128-byte events)
- 4 Appraisers (Homeostasis, Curiosity, Efficiency, GoalDirected)
- Full reward calculation pipeline

**🔄 Текущая архитектура:**
```
Token/Connection → Grid/Graph → Guardian (CDNA validation) →
ExperienceStream (events) → Appraisers (reward calculation) →
[Next: Learner/Attention modules]
```

### 11.2 Следующие шаги (Priority Order)

#### Option A: Learner Module (Mini-Neuron)
**Цель:** Первый обучаемый компонент

Что делать:
1. Создать `src/core_rust/src/learner/mod.rs`
2. Реализовать простейшую Hebbian learning rule:
   - "Neurons that fire together, wire together"
   - Update connection weights based on co-activation
3. Интегрировать с ExperienceStream:
   - Subscribe to ActionExecuted events
   - Update weights when actions lead to positive reward
4. Добавить в Guardian validation
5. Написать тесты

**Входные данные:**
- ExperienceEvent with state + action + reward
- Connection weights from Graph

**Выходные данные:**
- Updated connection weights
- Learning metrics (learning_rate, weight_changes)

**Примерный объем:** ~200-300 lines + tests

#### Option B: Attention Module (Salience)
**Цель:** Selective activation of tokens

Что делать:
1. Создать `src/core_rust/src/attention/mod.rs`
2. Реализовать salience calculation:
   - Based on novelty (L2), certainty (L6), energy (L7)
   - Weighted combination from ADNA parameters
3. Add activation threshold mechanism
4. Интегрировать с Grid (spatial attention)
5. Написать тесты

**Входные данные:**
- Token state vectors (8D)
- ADNA attention weights
- Current context (recent events)

**Выходные данные:**
- Activation scores for each token
- Top-K most salient tokens

**Примерный объем:** ~250-350 lines + tests

#### Option C: Policy Executor (ADNA → Actions)
**Цель:** Bridge между ADNA policies и действиями системы

Что делать:
1. Создать `src/core_rust/src/policy/mod.rs`
2. Реализовать policy interpreter:
   - Parse policy rules (JSON/TOML)
   - Match conditions against current state
   - Execute actions (create tokens, modify connections)
3. Add action queue management
4. Интегрировать с ExperienceStream
5. Написать тесты

**Входные данные:**
- ADNA policy rules
- Current system state
- Event triggers

**Выходные данные:**
- Actions to execute
- Policy execution metrics

**Примерный объем:** ~300-400 lines + tests

### 11.3 Рекомендации для следующей сессии

**Start with:** Option A (Learner Module)
- Самый естественный next step после Appraisers
- Замыкает loop: Events → Rewards → Learning
- Относительно простая имплементация
- Сразу видны результаты (weight updates)

**Архитектурная картина:**
```
[Perception] → Tokens/Connections → Grid/Graph
       ↓
[Memory] → ExperienceStream (events with rewards)
       ↓
[Evaluation] → Appraisers (reward calculation) ← ADNA weights
       ↓
[Learning] → Learner (weight updates based on rewards) ← NEW
       ↓
[Action] → Policy Executor (execute actions)
       ↓
[Validation] → Guardian (CDNA/ADNA compliance)
```

**Key questions to resolve:**
1. Where to store learned weights? (in Connection structure? separate storage?)
2. Learning rate schedule? (fixed vs adaptive)
3. Batch vs online learning? (update after each event vs batch)
4. Integration point with Graph? (direct weight modification vs delta queue?)

### 11.4 Implementation Template

```rust
// src/core_rust/src/learner/mod.rs

pub struct Learner {
    learning_rate: f32,
    weight_updates: Vec<WeightUpdate>,
}

pub struct WeightUpdate {
    connection_id: EdgeId,
    delta: f32,
    timestamp: u64,
}

impl Learner {
    /// Process experience event and generate weight updates
    pub fn learn(&mut self, event: &ExperienceEvent, adna: &ADNA) -> Vec<WeightUpdate> {
        // 1. Extract state, action, reward from event
        // 2. Calculate weight deltas using Hebbian rule
        // 3. Apply learning rate from ADNA
        // 4. Return updates for Graph to apply
    }

    /// Apply weight updates to graph
    pub fn apply_updates(&self, graph: &mut Graph, updates: &[WeightUpdate]) {
        // Modify connection weights in graph
    }
}
```

---

**Конец спецификации ADNA v1.0 MVP**

*Эта спецификация определяет минимальную жизнеспособную версию ADNA, фокусируясь на структурах данных, валидации и интеграции с Guardian. Она закладывает фундамент для постепенной эволюции до полноценного Policy Engine в будущих версиях.*

**Статус обновлений:**
- v0.23.0 (2025-11-02): Initial ADNA implementation
- v0.24.0 (2025-11-03): Guardian v1.1 integration
- v0.25.0 (2025-11-03): 4 Appraisers + Roadmap
