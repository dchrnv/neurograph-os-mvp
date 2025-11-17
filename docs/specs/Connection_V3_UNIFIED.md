# Connection v3.0 — Unified Specification

**Version:** 3.0.0
**Status:** 🚧 IN DEVELOPMENT (v0.29.0)
**Date:** 2025-11-17 (originally proposed 2025-01-13)
**Language:** Rust 2021
**Size:** 64 bytes (expanded from v1:32 bytes)
**Compatibility:** NeuroGraph OS Token v2.0
**Implementation Target:** v0.29.0 release  

---

## Архитектурный контекст (2025-11-17)

Эта спецификация была **переоткрыта** после рефлексии о концептуальном пересечении между Connection и IntuitionEngine:

**Проблема:** Connections описывали статические отношения (онтология), но каузальные связи (Cause, Effect, EnabledBy) — это гипотезы, которые должны эволюционировать на основе опыта.

**Решение:** V3 Unified реализует **Hybrid модель** — Connections частично обучаемые:
- **Immutable:** Онтологические истины (IsA, Synonym) — не меняются
- **Learnable:** Каузальные гипотезы (Cause, Effect) — адаптируются через опыт
- **Hypothesis:** Экспериментальные паттерны (быстрое обучение + затухание)

**Биологическая аналогия:** Synaptic plasticity — аксоны/дендриты структурны (immutable), но синаптическая сила меняется (learnable).

**План реализации:** См. [IntuitionEngine_v2.2.md](IntuitionEngine_v2.2.md) — 5 фаз (~1250 LOC).

---

## Оглавление

1. [Философия и эволюция](#философия-и-эволюция)
2. [Что нового в v3.0](#что-нового-в-v30)
3. [Бинарная структура](#бинарная-структура)
4. [Система обучения](#система-обучения)
5. [Поля данных](#поля-данных)
6. [Типы связей](#типы-связей)
7. [API и интеграция](#api-и-интеграция)
8. [Миграция с предыдущих версий](#миграция-с-предыдущих-версий)

---

## Философия и эволюция

### Проблема статичности v1.0

В версии 1.0 (32 байта) Connection были **чисто структурными** — они описывали отношения между токенами, но не могли эволюционировать на основе опыта:

```
Connection v1.0: "Action X CAUSES State Y" — статично
Реальность: Иногда срабатывает, иногда нет
Вопрос: Как адаптировать связь к реальному опыту?
```

### Решение: Гибридная модель обучения

Version 3.0 вводит **64-байтную структуру** с тремя уровнями изменяемости:

1. **Immutable (Онтологические факты)** — Синонимы, гипернимы, структурные отношения
2. **Learnable (Причинные гипотезы)** — Причины, эффекты, временные паттерны
3. **Hypothesis (Экспериментальные)** — Временные связи для тестирования

### Биологическая аналогия

```
Аксоны/дендриты (структура) = Immutable Connections
Синаптическая сила = Learnable Connection confidence  
Временная потенциация = Hypothesis Connections
```

---

## Что нового в v3.0

### Сравнение версий

| Аспект | v1.0 (32B) | v2.0 (spec) | v3.0 (64B) |
|--------|------------|-------------|------------|
| **Размер** | 32 байта | 32 байта | 64 байта |
| **Обучение** | ❌ Нет | ✅ Концепция | ✅ Полная реализация |
| **Mutability** | Static | — | 3 уровня |
| **Confidence** | — | — | 0-255 (0.0-1.0) |
| **Evidence** | — | — | Счетчик наблюдений |
| **Source tracking** | — | — | ID создателя |
| **Learning rate** | — | — | Настраиваемая |
| **Decay** | — | — | Автоматический |

### Ключевые улучшения

- 🆕 **Learning Extension (32 bytes)** — вторая половина структуры для обучения
- 🆕 **Three-tier mutability** — Immutable, Learnable, Hypothesis
- 🆕 **Evidence-based confidence** — растет с подтверждениями
- 🆕 **IntuitionEngine integration** — автоматические предложения
- 🆕 **Guardian validation** — проверка изменений
- ✅ **Backward compatible** — первые 32 байта совместимы с v1.0

---

## Бинарная структура

### Полная структура (64 байта)

```
Offset  Size   Type      Field                  Description
------  -----  --------  --------------------   ---------------------------
                        [CORE STRUCTURE - 32 bytes, v1.0 compatible]
0       4      uint32    token_a_id             ID первого токена
4       4      uint32    token_b_id             ID второго токена  
8       1      uint8     connection_type        Тип связи (0-255)
9       1      uint8     rigidity               Жёсткость (0-255 = 0.0-1.0)
10      1      uint8     active_levels          Битмаска активных уровней
11      1      uint8     flags                  Флаги состояния
12      4      uint32    activation_count       Счётчик активаций
16      4      float32   pull_strength          Сила притяжения/отталкивания
20      4      float32   preferred_distance     Желаемое расстояние
24      4      uint32    created_at             Unix timestamp создания
28      4      uint32    last_activation        Unix timestamp последней активации

                        [LEARNING EXTENSION - 32 bytes, NEW in v3.0]
32      1      uint8     mutability             0=Immutable, 1=Learnable, 2=Hypothesis
33      1      uint8     confidence             0-255 (0.0-1.0 confidence score)
34      2      uint16    evidence_count         Количество наблюдений
36      4      uint32    last_update            Unix timestamp последнего обновления
40      1      uint8     learning_rate          0-255 (0.0-1.0 скорость обучения)
41      1      uint8     decay_rate             0-255 (0.0-1.0 скорость затухания)
42      2      uint16    padding1               Выравнивание (reserved)
44      4      uint32    source_id              ID создателя (0=manual, >0=IntuitionEngine)
48      16     uint8[16] reserved               Зарезервировано для будущего
------  -----  --------  --------------------   ---------------------------
TOTAL: 64 bytes
```

### Выравнивание и упаковка

```rust
#[repr(C, align(64))]  // Выровнено по кэш-линии
pub struct ConnectionV3 {
    // Core fields (32 bytes)
    pub token_a_id: u32,
    pub token_b_id: u32,
    pub connection_type: u8,
    pub rigidity: u8,
    pub active_levels: u8,
    pub flags: u8,
    pub activation_count: u32,
    pub pull_strength: f32,
    pub preferred_distance: f32,
    pub created_at: u32,
    pub last_activation: u32,
    
    // Learning fields (32 bytes)
    pub mutability: ConnectionMutability,  // enum as u8
    pub confidence: u8,
    pub evidence_count: u16,
    pub last_update: u32,
    pub learning_rate: u8,
    pub decay_rate: u8,
    pub _padding1: u16,
    pub source_id: u32,
    pub reserved: [u8; 16],
}
```

---

## Система обучения

### 1. Уровни изменяемости (Mutability)

```rust
#[repr(u8)]
pub enum ConnectionMutability {
    Immutable = 0,   // Онтологические факты (не меняются)
    Learnable = 1,   // Обучаемые гипотезы (меняются медленно)
    Hypothesis = 2,  // Экспериментальные (меняются быстро)
}
```

#### Immutable Connections
- **Типы:** Synonym, Antonym, Hypernym, PartOf, HasPart
- **Философия:** Определительные истины, не зависящие от опыта
- **Создание:** Вручную, импорт WordNet, онтологии
- **Изменение:** ЗАПРЕЩЕНО IntuitionEngine
- **Confidence:** Всегда 255 (абсолютная уверенность)

#### Learnable Connections
- **Типы:** Cause, Effect, EnabledBy, UsedFor, Before, After
- **Философия:** Рабочие гипотезы, уточняемые наблюдением
- **Создание:** Вручную или IntuitionEngine
- **Изменение:** Через proposals от IntuitionEngine + Guardian
- **Confidence:** Растет с подтверждениями, падает с опровержениями

#### Hypothesis Connections
- **Типы:** Любые временные паттерны
- **Философия:** Экспериментальные идеи для тестирования
- **Создание:** IntuitionEngine при обнаружении паттернов
- **Жизненный цикл:** 
  - Повышение до Learnable при накоплении evidence
  - Удаление при опровержении или timeout

### 2. Механизм confidence

```rust
// Обновление confidence на основе результата
pub fn update_confidence(&mut self, success: bool) {
    let delta = self.learning_rate as f32 / 255.0;
    
    if success {
        // Увеличиваем confidence (с насыщением)
        let new_conf = (self.confidence as f32 / 255.0) + delta;
        self.confidence = (new_conf.min(1.0) * 255.0) as u8;
        self.evidence_count = self.evidence_count.saturating_add(1);
    } else {
        // Уменьшаем confidence
        let new_conf = (self.confidence as f32 / 255.0) - delta * 0.5;
        self.confidence = (new_conf.max(0.0) * 255.0) as u8;
    }
    
    self.last_update = current_timestamp();
}
```

### 3. Evidence и decay

```rust
// Автоматическое затухание hypothesis connections
pub fn apply_decay(&mut self) {
    if self.mutability == ConnectionMutability::Hypothesis {
        let time_since_update = current_timestamp() - self.last_update;
        
        // Затухание если нет новых evidence > 1 час
        if time_since_update > 3600 {
            let decay_factor = self.decay_rate as f32 / 255.0;
            let new_conf = (self.confidence as f32 / 255.0) * (1.0 - decay_factor);
            self.confidence = (new_conf * 255.0) as u8;
            
            // Удаляем если confidence < threshold
            if self.confidence < 25 {  // < 10%
                self.mark_for_deletion();
            }
        }
    }
}
```

### 4. Интеграция с IntuitionEngine

```rust
// IntuitionEngine генерирует proposals
pub struct ConnectionProposal {
    pub target_connection: u64,      // ID существующей связи или 0 для новой
    pub proposed_change: ProposalType,
    pub justification: String,
    pub evidence: Vec<ExperienceEventId>,
    pub expected_impact: f32,        // Ожидаемое улучшение reward
}

pub enum ProposalType {
    Create {
        token_a: u32,
        token_b: u32,
        connection_type: u8,
        initial_confidence: u8,
    },
    Modify {
        field: ConnectionField,
        new_value: Value,
    },
    Delete,
    PromoteToLearnable,  // Hypothesis → Learnable
}
```

---

## Поля данных

### Core поля (байты 0-31)

#### token_a_id, token_b_id (0-7)
- **Тип:** u32 каждый
- **Правило:** В канонической форме `token_a_id < token_b_id`
- **Валидация:** Оба ID должны существовать в системе

#### connection_type (8)
- **Тип:** u8 (256 типов)
- **Категории:** 11 основных категорий (см. раздел Типы связей)

#### rigidity (9)
- **Тип:** u8
- **Диапазон:** 0-255 maps to 0.0-1.0
- **Семантика:** Сопротивление изменению расстояния

#### active_levels (10)
- **Тип:** u8 bitfield
```
Bit 0: L1_PHYSICAL   (0x01)
Bit 1: L2_SENSORY    (0x02)
Bit 2: L3_MOTOR      (0x04)
Bit 3: L4_EMOTIONAL  (0x08)
Bit 4: L5_COGNITIVE  (0x10)
Bit 5: L6_SOCIAL     (0x20)
Bit 6: L7_TEMPORAL   (0x40)
Bit 7: L8_ABSTRACT   (0x80)
```

#### flags (11)
```rust
pub mod connection_flags {
    pub const ACTIVE: u8      = 0x01;
    pub const PERSISTENT: u8  = 0x02;
    pub const BIDIRECTIONAL: u8 = 0x04;
    pub const INHIBITORY: u8  = 0x08;
    pub const MODIFIED: u8    = 0x10;
    pub const REINFORCED: u8  = 0x20;
    pub const DECAYING: u8    = 0x40;
    pub const USER_FLAG: u8   = 0x80;
}
```

#### activation_count (12-15)
- **Тип:** u32
- **Семантика:** Инкрементируется при каждой активации

#### pull_strength (16-19)
- **Тип:** f32
- **Диапазон:** -10.0 to +10.0
- **Семантика:** 
  - Positive = притяжение
  - Negative = отталкивание

#### preferred_distance (20-23)
- **Тип:** f32
- **Диапазон:** 0.01 to 100.0
- **Семантика:** Целевое расстояние между токенами

#### created_at, last_activation (24-31)
- **Тип:** u32 (Unix timestamp)

### Learning поля (байты 32-63)

#### mutability (32)
- **Тип:** u8 (enum)
- **Значения:** 0=Immutable, 1=Learnable, 2=Hypothesis

#### confidence (33)
- **Тип:** u8
- **Диапазон:** 0-255 представляет 0.0-1.0
- **Семантика:** Уверенность в истинности связи

#### evidence_count (34-35)
- **Тип:** u16
- **Семантика:** Количество наблюдений, поддерживающих связь

#### last_update (36-39)
- **Тип:** u32 (Unix timestamp)
- **Семантика:** Последнее изменение learning полей

#### learning_rate (40)
- **Тип:** u8
- **Диапазон:** 0-255 представляет 0.0-1.0
- **Семантика:** Скорость адаптации (большая = быстрее учится)

#### decay_rate (41)
- **Тип:** u8
- **Диапазон:** 0-255 представляет 0.0-1.0
- **Семантика:** Скорость затухания без подтверждений

#### source_id (44-47)
- **Тип:** u32
- **Семантика:** 
  - 0 = создано вручную
  - >0 = ID proposal от IntuitionEngine

#### reserved (48-63)
- **Тип:** [u8; 16]
- **Назначение:** Будущие расширения

---

## Типы связей

### Полная таблица (256 типов)

```rust
#[repr(u8)]
pub enum ConnectionType {
    // Semantic (0x00-0x0F)
    Synonym = 0x00,
    Antonym = 0x01,
    Hypernym = 0x02,        // IsA
    Hyponym = 0x03,         // TypeOf
    Meronym = 0x04,         // PartOf
    Holonym = 0x05,         // HasPart
    Troponym = 0x06,        // MannerOf
    Entailment = 0x07,
    Similar = 0x08,
    Also = 0x09,
    Attribute = 0x0A,
    Derivation = 0x0B,
    Domain = 0x0C,
    Topic = 0x0D,
    Usage = 0x0E,
    Region = 0x0F,
    
    // Causal (0x10-0x1F)
    Cause = 0x10,
    Effect = 0x11,
    Precondition = 0x12,
    Postcondition = 0x13,
    EnabledBy = 0x14,
    DisabledBy = 0x15,
    PreventedBy = 0x16,
    Triggered = 0x17,
    Influences = 0x18,
    Correlates = 0x19,
    Depends = 0x1A,
    Produces = 0x1B,
    Consumes = 0x1C,
    Modifies = 0x1D,
    Maintains = 0x1E,
    Destroys = 0x1F,
    
    // Temporal (0x20-0x2F)
    Before = 0x20,
    After = 0x21,
    During = 0x22,
    Overlaps = 0x23,
    Starts = 0x24,
    Finishes = 0x25,
    Meets = 0x26,
    Equals = 0x27,
    Simultaneous = 0x28,
    Sequential = 0x29,
    Parallel = 0x2A,
    Periodic = 0x2B,
    Continuous = 0x2C,
    Discrete = 0x2D,
    Instant = 0x2E,
    Extended = 0x2F,
    
    // Spatial (0x30-0x3F)
    Near = 0x30,
    Far = 0x31,
    Above = 0x32,
    Below = 0x33,
    Left = 0x34,
    Right = 0x35,
    Inside = 0x36,
    Outside = 0x37,
    Adjacent = 0x38,
    Overlapping = 0x39,
    Touching = 0x3A,
    Containing = 0x3B,
    Crossing = 0x3C,
    Behind = 0x3D,
    Front = 0x3E,
    Between = 0x3F,
    
    // Logical (0x40-0x4F)
    And = 0x40,
    Or = 0x41,
    Not = 0x42,
    Xor = 0x43,
    Implies = 0x44,
    Equivalent = 0x45,
    Contradicts = 0x46,
    Consistent = 0x47,
    Proves = 0x48,
    Disproves = 0x49,
    Assumes = 0x4A,
    Concludes = 0x4B,
    Necessary = 0x4C,
    Sufficient = 0x4D,
    Possible = 0x4E,
    Impossible = 0x4F,
    
    // Associative (0x50-0x5F)
    AssociatedWith = 0x50,
    RelatedTo = 0x51,
    SimilarTo = 0x52,
    ContrastedWith = 0x53,
    ComparedTo = 0x54,
    DistinguishedFrom = 0x55,
    AlternativeTo = 0x56,
    SubstituteFor = 0x57,
    ComplementOf = 0x58,
    VariantOf = 0x59,
    VersionOf = 0x5A,
    ExampleOf = 0x5B,
    InstanceOf = 0x5C,
    KindOf = 0x5D,
    FormOf = 0x5E,
    ManifestationOf = 0x5F,
    
    // Structural (0x60-0x6F)
    PartOf = 0x60,
    HasPart = 0x61,
    MemberOf = 0x62,
    HasMember = 0x63,
    SubclassOf = 0x64,
    SuperclassOf = 0x65,
    Contains = 0x66,
    ContainedBy = 0x67,
    Comprises = 0x68,
    ComposedOf = 0x69,
    ElementOf = 0x6A,
    HasElement = 0x6B,
    CollectionOf = 0x6C,
    ItemIn = 0x6D,
    SegmentOf = 0x6E,
    Whole = 0x6F,
    
    // Functional (0x70-0x7F)
    UsedFor = 0x70,
    UsedBy = 0x71,
    ToolFor = 0x72,
    MethodFor = 0x73,
    InputTo = 0x74,
    OutputFrom = 0x75,
    ResourceFor = 0x76,
    RequiredBy = 0x77,
    ProvidedBy = 0x78,
    CapableOf = 0x79,
    SupportsFunction = 0x7A,
    ImplementsFunction = 0x7B,
    InterfaceFor = 0x7C,
    ProtocolFor = 0x7D,
    StandardFor = 0x7E,
    OptimizedFor = 0x7F,
    
    // Emotional (0x80-0x8F)  
    Likes = 0x80,
    Dislikes = 0x81,
    Loves = 0x82,
    Hates = 0x83,
    Fears = 0x84,
    Trusts = 0x85,
    Distrusts = 0x86,
    Respects = 0x87,
    Admires = 0x88,
    Envies = 0x89,
    Sympathizes = 0x8A,
    Empathizes = 0x8B,
    Resents = 0x8C,
    Forgives = 0x8D,
    Blames = 0x8E,
    Grateful = 0x8F,
    
    // Rule/Metaphor (0x90-0x9F)
    Rule = 0x90,
    Exception = 0x91,
    Constraint = 0x92,
    Permission = 0x93,
    Prohibition = 0x94,
    Obligation = 0x95,
    Metaphor = 0x96,
    Analogy = 0x97,
    Symbol = 0x98,
    Represents = 0x99,
    Signifies = 0x9A,
    Indicates = 0x9B,
    Suggests = 0x9C,
    Connotes = 0x9D,
    Denotes = 0x9E,
    References = 0x9F,
    
    // Dynamic (0xA0-0xAF)
    Becomes = 0xA0,
    Transforms = 0xA1,
    Evolves = 0xA2,
    Develops = 0xA3,
    Grows = 0xA4,
    Decays = 0xA5,
    Improves = 0xA6,
    Degrades = 0xA7,
    Strengthens = 0xA8,
    Weakens = 0xA9,
    Accelerates = 0xAA,
    Decelerates = 0xAB,
    Stabilizes = 0xAC,
    Destabilizes = 0xAD,
    Cycles = 0xAE,
    Alternates = 0xAF,
    
    // Reserved (0xB0-0xFF) for extensions
}
```

---

## API и интеграция

### Основные операции

```rust
use neurograph_os::{Connection, ConnectionType, ConnectionMutability};

impl Connection {
    // Создание новой связи
    pub fn new(token_a: u32, token_b: u32) -> Self {
        let (a, b) = if token_a < token_b {
            (token_a, token_b)
        } else {
            (token_b, token_a)  // Canonical order
        };
        
        Self {
            token_a_id: a,
            token_b_id: b,
            connection_type: ConnectionType::AssociatedWith as u8,
            rigidity: 128,  // 0.5
            active_levels: 0,
            flags: 0,
            activation_count: 0,
            pull_strength: 0.0,
            preferred_distance: 1.0,
            created_at: current_timestamp(),
            last_activation: 0,
            
            // Learning defaults
            mutability: ConnectionMutability::Learnable,
            confidence: 128,  // 0.5
            evidence_count: 0,
            last_update: current_timestamp(),
            learning_rate: 32,  // 0.125
            decay_rate: 16,     // 0.0625
            _padding1: 0,
            source_id: 0,
            reserved: [0; 16],
        }
    }
    
    // Активация связи
    pub fn activate(&mut self) {
        self.activation_count = self.activation_count.saturating_add(1);
        self.last_activation = current_timestamp();
        self.flags |= connection_flags::ACTIVE;
        
        // Reinforcement для learnable
        if self.mutability == ConnectionMutability::Learnable {
            self.flags |= connection_flags::REINFORCED;
            self.rigidity = self.rigidity.saturating_add(1);
        }
    }
    
    // Проверка возможности модификации
    pub fn can_modify(&self) -> bool {
        self.mutability != ConnectionMutability::Immutable
    }
    
    // Применение предложения от IntuitionEngine
    pub fn apply_proposal(&mut self, proposal: &ConnectionProposal) -> Result<(), Error> {
        // Валидация Guardian
        if !Guardian::validate_proposal(self, proposal)? {
            return Err(Error::ProposalRejected);
        }
        
        // Применение изменений
        match &proposal.proposed_change {
            ProposalType::Modify { field, new_value } => {
                match field {
                    ConnectionField::Confidence => {
                        self.confidence = new_value.as_u8()?;
                        self.evidence_count += proposal.evidence.len() as u16;
                    },
                    ConnectionField::PullStrength => {
                        self.pull_strength = new_value.as_f32()?;
                    },
                    // ... другие поля
                }
            },
            ProposalType::PromoteToLearnable => {
                if self.mutability == ConnectionMutability::Hypothesis {
                    self.mutability = ConnectionMutability::Learnable;
                    self.learning_rate = 32;  // Slower learning
                    self.decay_rate = 8;      // Slower decay
                }
            },
            _ => {}
        }
        
        self.last_update = current_timestamp();
        self.flags |= connection_flags::MODIFIED;
        Ok(())
    }
}
```

### Интеграция с IntuitionEngine

```rust
// IntuitionEngine анализирует ExperienceStream и генерирует proposals
impl IntuitionEngine {
    pub fn analyze_pattern(&self, events: &[ExperienceEvent]) -> Vec<ConnectionProposal> {
        let mut proposals = Vec::new();
        
        // Пример: обнаружен паттерн "A часто следует за B"  
        if let Some(pattern) = self.detect_temporal_pattern(events) {
            if pattern.confidence > 0.7 && pattern.occurrences > 10 {
                proposals.push(ConnectionProposal {
                    target_connection: 0,  // New connection
                    proposed_change: ProposalType::Create {
                        token_a: pattern.token_a,
                        token_b: pattern.token_b,
                        connection_type: ConnectionType::After as u8,
                        initial_confidence: (pattern.confidence * 255.0) as u8,
                    },
                    justification: format!(
                        "Pattern detected: {} follows {} in {:.1}% cases (n={})",
                        pattern.token_b, pattern.token_a,
                        pattern.confidence * 100.0, pattern.occurrences
                    ),
                    evidence: pattern.supporting_events,
                    expected_impact: pattern.expected_reward_improvement,
                });
            }
        }
        
        proposals
    }
}
```

### Интеграция с Guardian

```rust
impl Guardian {
    pub fn validate_proposal(
        connection: &Connection, 
        proposal: &ConnectionProposal
    ) -> Result<bool, Error> {
        // Проверка CDNA constraints
        if !self.cdna.check_connection_allowed(connection, proposal)? {
            return Ok(false);
        }
        
        // Проверка mutability
        if connection.mutability == ConnectionMutability::Immutable {
            return Ok(false);  // Нельзя менять immutable
        }
        
        // Проверка evidence significance
        if proposal.evidence.len() < 5 {
            return Ok(false);  // Недостаточно доказательств
        }
        
        // Проверка разумности изменения
        if let ProposalType::Modify { field: ConnectionField::Confidence, new_value } = &proposal.proposed_change {
            let old = connection.confidence as f32 / 255.0;
            let new = new_value.as_u8()? as f32 / 255.0;
            
            // Не позволяем резкие скачки confidence
            if (new - old).abs() > 0.3 {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
}
```

### Физическая модель

```rust
// Расчет силы взаимодействия
pub fn calculate_force(&self, current_distance: f32) -> f32 {
    let delta = self.preferred_distance - current_distance;
    let rigidity_factor = self.rigidity as f32 / 255.0;
    let confidence_factor = if self.can_modify() {
        self.confidence as f32 / 255.0
    } else {
        1.0  // Immutable connections всегда активны
    };
    
    delta * rigidity_factor * self.pull_strength * confidence_factor
}
```

---

## Миграция с предыдущих версий

### Миграция v1.0 → v3.0

```rust
pub fn migrate_v1_to_v3(v1_data: &[u8; 32]) -> ConnectionV3 {
    let mut v3 = ConnectionV3::default();
    
    // Копируем первые 32 байта as-is
    unsafe {
        std::ptr::copy_nonoverlapping(
            v1_data.as_ptr(),
            &mut v3 as *mut _ as *mut u8,
            32
        );
    }
    
    // Устанавливаем defaults для learning полей
    v3.mutability = guess_mutability(v3.connection_type);
    v3.confidence = 200;  // ~0.78 для старых связей
    v3.evidence_count = v3.activation_count.min(u16::MAX as u32) as u16;
    v3.last_update = v3.last_activation;
    v3.learning_rate = 16;  // Conservative
    v3.decay_rate = 8;      // Slow decay
    v3.source_id = 0;       // Manual/imported
    
    v3
}

fn guess_mutability(conn_type: u8) -> ConnectionMutability {
    match conn_type {
        0x00..=0x0F => ConnectionMutability::Immutable,  // Semantic
        0x10..=0x1F => ConnectionMutability::Learnable,  // Causal
        0x20..=0x2F => ConnectionMutability::Learnable,  // Temporal
        0x60..=0x6F => ConnectionMutability::Immutable,  // Structural
        _ => ConnectionMutability::Learnable,
    }
}
```

### Обратная совместимость

Первые 32 байта Connection v3.0 полностью совместимы с v1.0, что позволяет:

1. **Чтение v1.0 данных:** Системы v3.0 могут читать v1.0 connections
2. **Graceful degradation:** v1.0 системы видят только core поля
3. **Постепенная миграция:** Можно обновлять connections по мере необходимости

---

## Примеры использования

### 1. Создание Immutable семантической связи

```rust
let mut synonym = Connection::new(word1_id, word2_id);
synonym.set_connection_type(ConnectionType::Synonym);
synonym.mutability = ConnectionMutability::Immutable;
synonym.confidence = 255;  // Absolute
synonym.pull_strength = 0.9;
synonym.preferred_distance = 0.05;
synonym.active_levels = active_levels::L8_ABSTRACT;
// Эта связь НИКОГДА не будет изменена IntuitionEngine
```

### 2. Создание Learnable причинной связи

```rust
let mut causal = Connection::new(action_id, effect_id);
causal.set_connection_type(ConnectionType::Cause);
causal.mutability = ConnectionMutability::Learnable;
causal.confidence = 128;  // 0.5 initial belief
causal.learning_rate = 32;  // 0.125 learning rate
causal.pull_strength = 0.7;
causal.preferred_distance = 1.0;
// Будет адаптироваться на основе опыта
```

### 3. Hypothesis connection от IntuitionEngine

```rust
// IntuitionEngine обнаружил паттерн
let mut hypothesis = Connection::new(state_a_id, state_b_id);
hypothesis.set_connection_type(ConnectionType::After);
hypothesis.mutability = ConnectionMutability::Hypothesis;
hypothesis.confidence = 64;  // 0.25 initial
hypothesis.evidence_count = 5;
hypothesis.learning_rate = 128;  // Fast learning
hypothesis.decay_rate = 32;  // Moderate decay
hypothesis.source_id = 12345;  // IntuitionEngine proposal ID
// Будет либо подтвержден и повышен, либо удален
```

### 4. Обработка предложения от IntuitionEngine

```rust
// IntuitionEngine анализирует опыт и предлагает усилить связь
let proposal = ConnectionProposal {
    target_connection: conn_id,
    proposed_change: ProposalType::Modify {
        field: ConnectionField::Confidence,
        new_value: Value::U8(192),  // Повысить до 0.75
    },
    justification: "Success rate 78% in last 100 trials (p<0.01)".to_string(),
    evidence: vec![event1, event2, /* ... */],
    expected_impact: 0.15,
};

// Guardian проверяет и применяет
if Guardian::validate_proposal(&connection, &proposal)? {
    connection.apply_proposal(&proposal)?;
}
```

---

## Тестирование

### Unit тесты

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_size() {
        assert_eq!(std::mem::size_of::<ConnectionV3>(), 64);
    }
    
    #[test]
    fn test_mutability_semantics() {
        let mut conn = Connection::new(1, 2);
        conn.mutability = ConnectionMutability::Immutable;
        assert!(!conn.can_modify());
        
        conn.mutability = ConnectionMutability::Learnable;
        assert!(conn.can_modify());
    }
    
    #[test]
    fn test_confidence_update() {
        let mut conn = Connection::new(1, 2);
        conn.confidence = 128;
        conn.learning_rate = 25;  // ~0.1
        
        conn.update_confidence(true);  // Success
        assert!(conn.confidence > 128);
        
        let old_conf = conn.confidence;
        conn.update_confidence(false);  // Failure
        assert!(conn.confidence < old_conf);
    }
    
    #[test]
    fn test_decay() {
        let mut conn = Connection::new(1, 2);
        conn.mutability = ConnectionMutability::Hypothesis;
        conn.confidence = 100;
        conn.decay_rate = 25;
        conn.last_update = current_timestamp() - 7200;  // 2 hours ago
        
        conn.apply_decay();
        assert!(conn.confidence < 100);
    }
}
```

---

## Производительность

### Характеристики

- **Размер:** Ровно 64 байта (1 кэш-линия)
- **Выравнивание:** 64-byte aligned для оптимального доступа
- **Операции:** O(1) для всех базовых операций
- **Сериализация:** Zero-copy через transmute
- **Пропускная способность:** ~10M connections/sec на современном CPU

### Оптимизации

1. **Cache-friendly:** Структура помещается в 1 кэш-линию
2. **SIMD-ready:** Можно обрабатывать батчами
3. **Lock-free updates:** Для confidence через atomic operations
4. **Sparse storage:** Только активные connections материализованы

---

## Будущие расширения (reserved space)

16 байт зарезервированного пространства позволяют добавить:

- **Векторное представление** (4 x f32) для neural embeddings
- **Дополнительные метрики** качества связи
- **Версионирование** для истории изменений
- **Криптографическую подпись** для верификации

---

## Заключение

Connection v3.0 представляет собой **полноценную learning-capable** структуру связей для NeuroGraph OS:

✅ **64 байта** — оптимальный размер для современных CPU  
✅ **Три уровня mutability** — от фактов до гипотез  
✅ **Evidence-based learning** — адаптация на основе опыта  
✅ **IntuitionEngine integration** — автоматическое обучение  
✅ **Guardian validation** — безопасные изменения  
✅ **Backward compatible** — плавная миграция с v1.0  

Эта версия обеспечивает баланс между:
- **Стабильностью** онтологических фактов
- **Адаптивностью** причинных гипотез
- **Экспериментированием** с новыми паттернами

---

**Version:** 3.0.0
**Date:** 2025-11-17 (originally proposed 2025-01-13)
**Authors:** NeuroGraph OS Team
**Status:** 🚧 IN DEVELOPMENT (v0.29.0)
**Implementation:** Targeted for v0.29.0 release