# ADNA v1.0 MVP — Adaptive DNA (Static Policy Edition)

**Версия:** 1.0.0 (MVP)
**Дата:** 2025-11-02
**Статус:** Спецификация для реализации
**Зависимости:** CDNA v2.1, Guardian v1.0, Token v2.0, Connection v1.0
**Размер:** 256 байт (фиксированный)
**Цель:** Базовая инфраструктура для статических политик и параметров системы

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
#[repr(C, align(64))]
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

    /// SHA256 hash of parent ADNA version
    pub parent_hash: [u8; 32],

    /// SHA256 hash of current state
    pub current_hash: [u8; 32],

    /// Creation timestamp (Unix epoch)
    pub created_at: u64,

    /// Last modification timestamp
    pub modified_at: u64,
}
```

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

    /// Reserved for future use
    pub _reserved: [u8; 28],
}
```

**MVP поля:**
- `fitness_score` - устанавливается вручную администратором
- `confidence` - может отражать стабильность системы
- Остальные поля зарезервированы для ADNA v2.0+

### 2.4 Policy Pointer Block (64 bytes)

```rust
pub struct PolicyPointer {
    /// Size of external policy file (bytes)
    pub policy_size: u32,

    /// File path hash (для идентификации)
    pub policy_path_hash: u64,

    /// Compression type (0 = none, 1 = LZ4, 2 = Zstd)
    pub compression_type: u8,

    /// Encryption flag (0 = none, 1 = AES-256)
    pub encryption_flag: u8,

    /// Cache strategy (0 = always, 1 = on-demand)
    pub cache_strategy: u8,

    /// Reserved
    pub _reserved1: u8,

    /// Checksum of policy file (FNV-1a)
    pub policy_checksum: u64,

    /// Reserved for future
    pub _reserved2: [u8; 36],
}
```

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

/// Complete ADNA structure (256 bytes)
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct ADNA {
    pub header: ADNAHeader,
    pub metrics: EvolutionMetrics,
    pub pointer: PolicyPointer,
    pub parameters: ADNAParameters,
}

// Compile-time size check
const _: () = assert!(std::mem::size_of::<ADNA>() == 256);

impl ADNA {
    /// Create new ADNA with default parameters
    pub fn new() -> Self {
        let mut adna = Self {
            header: ADNAHeader::default(),
            metrics: EvolutionMetrics::default(),
            pointer: PolicyPointer::default(),
            parameters: ADNAParameters::default(),
        };
        adna.header.current_hash = adna.compute_hash();
        adna
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

    /// Compute SHA256 hash of ADNA (excluding current_hash field)
    pub fn compute_hash(&self) -> [u8; 32] {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();

        // Hash all fields except current_hash
        let bytes = unsafe {
            std::slice::from_raw_parts(self as *const ADNA as *const u8, 256)
        };

        // Skip current_hash field (offset 12, size 32)
        hasher.update(&bytes[0..12]);
        hasher.update(&bytes[44..256]);

        let result = hasher.finalize();
        let mut hash = [0u8; 32];
        hash.copy_from_slice(&result);
        hash
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

        // Check hash
        let computed = self.compute_hash();
        if computed != self.header.current_hash {
            return Err(ADNAError::HashMismatch);
        }

        // Validate parameters
        self.parameters.validate()?;

        Ok(())
    }

    /// Create new version based on current
    pub fn evolve(&self) -> Self {
        let mut new_adna = *self;
        new_adna.header.parent_hash = self.header.current_hash;
        new_adna.metrics.generation += 1;
        new_adna.header.modified_at = current_timestamp();
        new_adna.header.current_hash = new_adna.compute_hash();
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
        assert_eq!(hash1, adna.header.current_hash);

        let mut adna2 = adna;
        adna2.parameters.exploration_rate = 0.5;
        adna2.header.current_hash = adna2.compute_hash();
        let hash2 = adna2.header.current_hash;

        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_parameter_validation() {
        let mut adna = ADNA::new();
        adna.parameters.homeostasis_weight = 1.5; // Invalid
        adna.header.current_hash = adna.compute_hash();
        assert!(adna.validate().is_err());
    }

    #[test]
    fn test_evolution() {
        let adna1 = ADNA::new();
        let adna2 = adna1.evolve();

        assert_eq!(adna2.header.parent_hash, adna1.header.current_hash);
        assert_eq!(adna2.metrics.generation, adna1.metrics.generation + 1);
        assert_ne!(adna2.header.current_hash, adna1.header.current_hash);
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
- Phase 2: Reward System (Appraisers)
- Phase 3: IntuitionEngine integration
- Phase 4: ActionController integration
- Phase 5: Full learning loop (v2.0+)

---

**Конец спецификации ADNA v1.0 MVP**

*Эта спецификация определяет минимальную жизнеспособную версию ADNA, фокусируясь на структурах данных, валидации и интеграции с Guardian. Она закладывает фундамент для постепенной эволюции до полноценного Policy Engine в будущих версиях.*
