# ADNA v3.0 - Активная ДНК (Движок Политик) - Спецификация Rust

**Версия:** 3.0.0
**Статус:** ✅ РЕАЛИЗОВАНО (v0.22.0)
**Язык:** Rust 2021
**Файл:** `src/core_rust/src/adna.rs`
**Размер:** 256 байт (фиксированный, выровненный по кэшу)
**Последнее обновление:** 2025-01-11

---

## Содержание

1. [Обзор](#обзор)
2. [Философия и принципы дизайна](#философия-и-принципы-дизайна)
3. [Раскладка памяти](#раскладка-памяти)
4. [Определения структур](#определения-структур)
5. [Справочник API](#справочник-api)
6. [Точки интеграции](#точки-интеграции)
7. [Характеристики производительности](#характеристики-производительности)
8. [Примеры использования](#примеры-использования)
9. [Тестирование](#тестирование)
10. [Миграция с v1.0](#миграция-с-v10)
11. [Дорожная карта](#дорожная-карта)

---

## Обзор

ADNA v3.0 представляет собой **фундаментальное переопределение** ADNA из адаптивного кэша параметров в **Движок Политик** - динамическую систему принятия решений, которая отображает состояния среды в действия. Это слой "приобретённых знаний" NeuroGraph OS, непрерывно эволюционирующий в рамках границ, установленных CDNA.

### Ключевые концепции

- **Политика как сущность первого класса**: ADNA фундаментально является функцией `Состояние → Действие`
- **Версионная эволюция**: Каждое состояние ADNA имеет отслеживание происхождения и метрики приспособленности
- **Обновления на основе градиентов**: Изменения управляются Движком Интуиции, анализирующим траектории опыта
- **Удовлетворение ограничений CDNA**: Все мутации проверяются на соответствие конституционным правилам
- **Асинхронное обучение**: Обновления политики происходят в выделенных фазах обучения

### Роль в когнитивной иерархии

```
CDNA (Конституционная) → валидирует → ADNA (Политика) → генерирует → Действия
                                              ↑
                                         Интуиция
                                        (анализирует)
                                              ↑
                                            Опыт
```

---

## Философия и принципы дизайна

### 1. Машинно-ориентированная архитектура

Все структуры используют размеры степеней двойки (32, 64, 128, 256 байт) для оптимальной работы с кэшем CPU:
- **Блоки по 64 байта**: Точно помещаются в одну кэш-линию (типичный размер L1 кэша)
- **256 байт всего**: Помещаются в 4 кэш-линии, минимизируя промахи кэша
- **Выравнивание**: `#[repr(C, align(64))]` обеспечивает правильное выравнивание
- **Упаковка**: `#[repr(C, packed)]` для подструктур для устранения padding

### 2. Разделение ответственности

Ядро ADNA (256 байт) содержит только **метаданные и указатели**. Фактические данные политики (веса, нейронные сети, деревья решений) хранятся отдельно. Это позволяет:
- Быстрая замена ADNA во время эволюции
- Эффективный контроль версий (маленькая структура ядра)
- Политики переменного размера без изменения структуры ядра
- Кэш-дружественный доступ к метаданным

### 3. Отслеживание происхождения

Каждое состояние ADNA отслеживает свою родословную через **SHA256 хэш родителя**, что позволяет:
- Реконструкция эволюционного дерева
- Откат к предыдущим стабильным версиям
- A/B тестирование вариантов политики
- Присвоение заслуг между поколениями

### 4. Поддержка мультимодальных политик

ADNA поддерживает множественные типы политик через enum `PolicyType`:
- **Linear**: Простая матрица весов (быстро, интерпретируемо)
- **Neural**: Политики на нейросетях (выразительно, дифференцируемо)
- **TreeBased**: Деревья решений (интерпретируемо, дискретно)
- **Hybrid**: Комбинация нескольких подходов
- **Programmatic**: Компилируемые правила-политики

---

## Раскладка памяти

### Общая структура (256 байт)

```
┌─────────────────────────────────────────────────────────┐
│ Структура ADNA (256 байт, align(64))                    │
├─────────────────────────────────────────────────────────┤
│ Смещение 0-63:   ADNAHeader        (64 байта)           │
│ Смещение 64-127: EvolutionMetrics  (64 байта, packed)   │
│ Смещение 128-191: PolicyPointer    (64 байта, packed)   │
│ Смещение 192-255: StateMapping     (64 байта)           │
└─────────────────────────────────────────────────────────┘
```

### Блок 1: ADNAHeader (64 байта)

```
┌─────────────────────────────────────────────────────────┐
│ ADNAHeader (64 байта, repr(C))                          │
├──────────────────────┬──────────────────────────────────┤
│ Смещение 0-3    (4Б)  │ magic: u32                       │
│ Смещение 4-5    (2Б)  │ version_major: u16               │
│ Смещение 6-7    (2Б)  │ version_minor: u16               │
│ Смещение 8-9    (2Б)  │ policy_type: u16                 │
│ Смещение 10-31  (22Б) │ _reserved1: [u8; 22]             │
│ Смещение 32-63  (32Б) │ parent_hash: [u8; 32]            │
└──────────────────────┴──────────────────────────────────┘
```

**Поля:**
- `magic`: Магическое число `0x41444E41` ('ADNA' в ASCII) для валидации
- `version_major/minor`: Версия ADNA (текущая 3.0)
- `policy_type`: Дискриминатор типа политики (0=Linear, 1=Neural, и т.д.)
- `_reserved1`: Зарезервировано для будущего использования (например, флаги, время создания)
- `parent_hash`: SHA256 хэш родительской ADNA для отслеживания происхождения

### Блок 2: EvolutionMetrics (64 байта, packed)

```
┌─────────────────────────────────────────────────────────┐
│ EvolutionMetrics (64 байта, repr(C, packed))            │
├──────────────────────┬──────────────────────────────────┤
│ Смещение 0-3    (4Б)  │ generation: u32                  │
│ Смещение 4-7    (4Б)  │ fitness_score: f32               │
│ Смещение 8-11   (4Б)  │ confidence: f32                  │
│ Смещение 12-15  (4Б)  │ exploration_rate: f32            │
│ Смещение 16-19  (4Б)  │ learning_rate: f32               │
│ Смещение 20-23  (4Б)  │ trajectory_count: u32            │
│ Смещение 24-27  (4Б)  │ success_rate: f32                │
│ Смещение 28-35  (8Б)  │ last_update: u64                 │
│ Смещение 36-39  (4Б)  │ update_frequency: u32            │
│ Смещение 40-63  (24Б) │ _reserved: [u8; 24]              │
└──────────────────────┴──────────────────────────────────┘
```

**Поля:**
- `generation`: Инкрементируется при каждом обновлении политики (глубина родословной)
- `fitness_score`: Общая метрика производительности (0.0-1.0, выше = лучше)
- `confidence`: Уверенность системы в текущей политике (0.0-1.0)
- `exploration_rate`: ε-greedy параметр исследования (0.0-1.0)
- `learning_rate`: Размер шага для градиентных обновлений
- `trajectory_count`: Общее количество собранных траекторий опыта
- `success_rate`: Скользящее среднее успешных траекторий (0.0-1.0)
- `last_update`: Unix timestamp последнего обновления политики
- `update_frequency`: Обновлений в час (для мониторинга скорости обновления)

### Блок 3: PolicyPointer (64 байта, packed)

```
┌─────────────────────────────────────────────────────────┐
│ PolicyPointer (64 байта, repr(C, packed))               │
├──────────────────────┬──────────────────────────────────┤
│ Смещение 0-3    (4Б)  │ policy_size: u32                 │
│ Смещение 4-11   (8Б)  │ policy_offset: u64               │
│ Смещение 12     (1Б)  │ compression_type: u8             │
│ Смещение 13     (1Б)  │ encryption_flag: u8              │
│ Смещение 14     (1Б)  │ cache_strategy: u8               │
│ Смещение 15-63  (49Б) │ _reserved: [u8; 49]              │
└──────────────────────┴──────────────────────────────────┘
```

**Поля:**
- `policy_size`: Размер данных политики в байтах
- `policy_offset`: Смещение в памяти/на диске к хранилищу политики
- `compression_type`: 0=нет, 1=LZ4, 2=zstd (для холодного хранения)
- `encryption_flag`: 0=нет, 1=зашифровано (для безопасных политик)
- `cache_strategy`: 0=всегда, 1=ленивая, 2=периодическая (управление кэшем)

### Блок 4: StateMapping (64 байта)

```
┌─────────────────────────────────────────────────────────┐
│ StateMapping (64 байта, repr(C))                        │
├──────────────────────┬──────────────────────────────────┤
│ Смещение 0-1    (2Б)  │ input_dimensions: u16            │
│ Смещение 2-3    (2Б)  │ output_dimensions: u16           │
│ Смещение 4-19   (16Б) │ state_normalization: [f32; 4]    │
│ Смещение 20-35  (16Б) │ action_bounds: [f32; 4]          │
│ Смещение 36-63  (28Б) │ _reserved: [u8; 28]              │
└──────────────────────┴──────────────────────────────────┘
```

**Поля:**
- `input_dimensions`: Размерность пространства состояний (по умолчанию: 8 для L1-L8)
- `output_dimensions`: Размерность пространства действий (по умолчанию: 8)
- `state_normalization`: `[mean, std, min, max]` для предобработки состояний
- `action_bounds`: `[min_x, max_x, min_y, max_y]` для ограничения действий

---

## Определения структур

### Полная структура ADNA

```rust
#[repr(C, align(64))]
#[derive(Debug, Clone, Copy)]
pub struct ADNA {
    pub header: ADNAHeader,             // 64 байта (смещение 0-63)
    pub evolution: EvolutionMetrics,    // 64 байта (смещение 64-127)
    pub policy_ptr: PolicyPointer,      // 64 байта (смещение 128-191)
    pub state_mapping: StateMapping,    // 64 байта (смещение 192-255)
}

// Проверка размера на этапе компиляции
const _: () = assert!(std::mem::size_of::<ADNA>() == 256);
```

### Перечисление PolicyType

```rust
#[repr(u16)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyType {
    Linear = 0,        // Матрица весов: действие = W * состояние + b
    Neural = 1,        // Нейронная сеть (MLP, Transformer, и т.д.)
    TreeBased = 2,     // Дерево решений или случайный лес
    Hybrid = 3,        // Ансамбль нескольких политик
    Programmatic = 4,  // Компилируемые правила-политики
}
```

### Магические числа

```rust
pub const ADNA_MAGIC: u32 = 0x41444E41; // 'ADNA' в ASCII
pub const ADNA_VERSION_MAJOR: u16 = 3;
pub const ADNA_VERSION_MINOR: u16 = 0;
```

---

## Справочник API

### Основные методы

#### `ADNA::new(policy_type: PolicyType) -> Self`

Создаёт новый экземпляр ADNA с параметрами по умолчанию.

**Пример:**
```rust
let adna = ADNA::new(PolicyType::Linear);
assert!(adna.is_valid());
assert_eq!(adna.evolution.generation, 0);
assert_eq!(adna.evolution.exploration_rate, 0.9); // Высокое начальное исследование
```

**Инициализация:**
- Поколение: 0
- Приспособленность: 0.0
- Уверенность: 0.5
- Коэффициент исследования: 0.9 (90% исследования изначально)
- Скорость обучения: 0.01
- Временная метка: текущее Unix время

#### `adna.is_valid() -> bool`

Проверяет целостность структуры ADNA.

**Проверки:**
- Магическое число совпадает с `ADNA_MAGIC`
- Версия совпадает с текущей реализацией

**Пример:**
```rust
if !adna.is_valid() {
    return Err(ADNAError::InvalidStructure);
}
```

#### `adna.policy_type() -> PolicyType`

Возвращает тип политики.

**Пример:**
```rust
match adna.policy_type() {
    PolicyType::Linear => run_linear_policy(&adna),
    PolicyType::Neural => run_neural_policy(&adna),
    _ => unimplemented!(),
}
```

### Отслеживание эволюции

#### `adna.update_fitness(new_fitness: f32)`

Обновляет оценку приспособленности с ограничением в диапазоне [0.0, 1.0].

**Пример:**
```rust
// После оценки производительности политики
let avg_reward = evaluate_policy(&adna, &test_episodes);
adna.update_fitness(avg_reward.clamp(0.0, 1.0));
```

**Побочные эффекты:**
- Обновляет временную метку `last_update`
- Ограничивает приспособленность до допустимого диапазона

#### `adna.increment_generation()`

Инкрементирует счётчик поколений (используйте при создании потомка ADNA).

**Пример:**
```rust
let mut child_adna = parent_adna.clone();
child_adna.increment_generation();
child_adna.header.parent_hash = compute_hash(&parent_adna);
```

#### `adna.record_trajectory(success: bool)`

Записывает результат траектории и обновляет коэффициент успеха через экспоненциальное скользящее среднее.

**Пример:**
```rust
for episode in episodes {
    let success = episode.total_reward > threshold;
    adna.record_trajectory(success);
}

println!("Коэффициент успеха: {:.2}%", adna.evolution.success_rate * 100.0);
```

**Формула:**
```
new_rate = α * (1 если успех иначе 0) + (1 - α) * old_rate
где α = 0.1
```

---

## Точки интеграции

### С CDNA (Конституционная ДНК)

ADNA работает **в рамках ограничений**, определённых CDNA:

```rust
// Перед применением обновления политики
if !cdna.validate_policy_update(&gradient, &adna) {
    return Err(PolicyError::ConstraintViolation);
}

// CDNA определяет границы
if action_magnitude > cdna.max_action_magnitude {
    action = clip_action(action, cdna.action_bounds);
}
```

**Ключевые взаимодействия:**
- CDNA предоставляет `action_bounds` → ADNA использует в `StateMapping`
- CDNA валидирует все мутации политики
- CDNA может запустить экстренный откат политики

### С ExperienceToken

Токены опыта предоставляют обучающие данные для обучения политики:

```rust
// ExperienceToken → вычисление градиента ADNA
pub struct ExperienceToken {
    pub state: [f32; 8],        // Вход в политику
    pub action: [f32; 8],       // Совершённое действие
    pub reward: f32,            // Полученная награда
    pub next_state: [f32; 6],   // Результирующее состояние
    pub adna_version_hash: [u8; 4], // Какая ADNA сгенерировала действие
}

// Политика использует это для вычисления градиентов
let gradient = policy.get_gradient(&experience_token);
```

### С Движком Интуиции

Движок Интуиции анализирует батчи опыта для генерации обновлений политики:

```rust
// Интуиция анализирует батч опыта
let experiences = experience_stream.sample_batch(1000, PrioritizedSampling);

// Генерирует градиент
let gradient = intuition_engine.analyze_trajectories(&experiences, &adna);

// Предлагает обновление
let proposal = Proposal {
    gradient,
    confidence: 0.85,
    expected_improvement: 0.05,
    risk_score: 0.1,
};

// Менеджер Эволюции решает, применять ли
if evolution_manager.should_apply(&proposal, &adna, &cdna) {
    policy.apply_gradient(&gradient, adna.evolution.learning_rate)?;
    adna.increment_generation();
}
```

### С трейтом Policy

Метаданные ADNA направляют выполнение политики:

```rust
pub trait Policy {
    fn map_state(&self, state: &[f32; 8]) -> [f32; 8];
    fn get_gradient(&self, experience: &ExperienceToken) -> Gradient;
    fn apply_gradient(&mut self, gradient: &Gradient, lr: f32) -> Result<()>;
}

// Использование
let state = get_current_state(); // [f32; 8]
let action = policy.map_state(&state);

// Применяем границы действий из ADNA
let bounded_action = clip_action(action, &adna.state_mapping.action_bounds);
```

---

## Характеристики производительности

### Память

| Метрика | Значение | Примечания |
|---------|----------|------------|
| Размер ядра ADNA | 256 байт | Фиксированный, выровненный по кэшу |
| Хранилище политики | 1КБ - 10МБ | Переменный, зависит от типа |
| Используемые кэш-линии | 4 | Оптимально для L1 кэша |
| Выравнивание | 64 байта | Соответствует типичной кэш-линии |

### Временная сложность

| Операция | Сложность | Типичное время |
|----------|-----------|----------------|
| `new()` | O(1) | ~50нс |
| `is_valid()` | O(1) | ~5нс |
| `update_fitness()` | O(1) | ~10нс |
| `increment_generation()` | O(1) | ~5нс |
| `record_trajectory()` | O(1) | ~15нс |

### Результаты бенчмарков (Rust 1.91.1, x86_64)

```
ADNA::new()              ... 48.2нс
ADNA::is_valid()         ... 4.8нс
ADNA::update_fitness()   ... 12.1нс
ADNA::increment_gen()    ... 5.3нс
ADNA::record_trajectory()... 16.7нс
```

---

## Примеры использования

### Пример 1: Базовое создание и валидация ADNA

```rust
use neurograph_core::{ADNA, PolicyType};

fn main() {
    // Создаём новую ADNA с линейной политикой
    let adna = ADNA::new(PolicyType::Linear);

    // Проверяем структуру
    assert!(adna.is_valid());
    assert_eq!(adna.policy_type(), PolicyType::Linear);

    // Проверяем начальное состояние
    assert_eq!(adna.evolution.generation, 0);
    assert_eq!(adna.evolution.fitness_score, 0.0);
    assert_eq!(adna.evolution.confidence, 0.5);
    assert_eq!(adna.evolution.exploration_rate, 0.9);

    println!("ADNA успешно инициализирована");
}
```

### Пример 2: Цикл оценки политики

```rust
use neurograph_core::{ADNA, PolicyType, LinearPolicy, Policy};

fn evaluate_policy(adna: &mut ADNA, policy: &LinearPolicy, episodes: usize) {
    let mut total_reward = 0.0;

    for episode in 0..episodes {
        let mut state = initialize_environment();
        let mut episode_reward = 0.0;

        for step in 0..100 {
            // Используем политику для выбора действия
            let action = policy.map_state(&state);

            // Применяем границы действий из ADNA
            let bounded_action = clip_action(
                action,
                &adna.state_mapping.action_bounds
            );

            // Выполняем действие
            let (next_state, reward, done) = environment_step(bounded_action);

            episode_reward += reward;
            state = next_state;

            if done { break; }
        }

        // Записываем результат траектории
        let success = episode_reward > 0.0;
        adna.record_trajectory(success);
        total_reward += episode_reward;
    }

    // Обновляем приспособленность
    let avg_reward = total_reward / episodes as f32;
    adna.update_fitness(avg_reward.clamp(0.0, 1.0));

    println!("Приспособленность: {:.3}, Коэффициент успеха: {:.2}%",
        adna.evolution.fitness_score,
        adna.evolution.success_rate * 100.0
    );
}
```

### Пример 3: Эволюция политики

```rust
use neurograph_core::{ADNA, PolicyType, LinearPolicy, Gradient};

fn evolve_policy(
    parent_adna: &ADNA,
    policy: &mut LinearPolicy,
    gradient: &Gradient
) -> Result<ADNA, PolicyError> {
    // Создаём потомка ADNA
    let mut child_adna = parent_adna.clone();

    // Применяем градиентное обновление
    policy.apply_gradient(gradient, parent_adna.evolution.learning_rate)?;

    // Обновляем метрики эволюции
    child_adna.increment_generation();
    child_adna.header.parent_hash = compute_hash(parent_adna);

    // Уменьшаем исследование со временем
    child_adna.evolution.exploration_rate *= 0.99;

    Ok(child_adna)
}
```

### Пример 4: A/B тестирование политик

```rust
fn ab_test_policies(
    policy_a: &LinearPolicy,
    policy_b: &LinearPolicy,
    adna_a: &mut ADNA,
    adna_b: &mut ADNA,
    episodes: usize
) -> PolicyType {
    // Оцениваем обе политики
    let score_a = run_evaluation(policy_a, adna_a, episodes);
    let score_b = run_evaluation(policy_b, adna_b, episodes);

    println!("Политика A: {:.3} (поколение {})", score_a, adna_a.evolution.generation);
    println!("Политика B: {:.3} (поколение {})", score_b, adna_b.evolution.generation);

    // Возвращаем победителя
    if score_a > score_b {
        println!("Политика A победила!");
        PolicyType::Linear
    } else {
        println!("Политика B победила!");
        PolicyType::Neural
    }
}
```

---

## Тестирование

### Юнит-тесты

Все тесты находятся в `src/core_rust/src/adna.rs`:

```rust
#[test]
fn test_adna_size() {
    assert_eq!(std::mem::size_of::<ADNA>(), 256);
    assert_eq!(std::mem::size_of::<ADNAHeader>(), 64);
    assert_eq!(std::mem::size_of::<EvolutionMetrics>(), 64);
    assert_eq!(std::mem::size_of::<PolicyPointer>(), 64);
    assert_eq!(std::mem::size_of::<StateMapping>(), 64);
}

#[test]
fn test_adna_creation() {
    let adna = ADNA::new(PolicyType::Linear);
    assert!(adna.is_valid());
    assert_eq!(adna.policy_type(), PolicyType::Linear);
}

#[test]
fn test_adna_fitness_update() {
    let mut adna = ADNA::new(PolicyType::Linear);
    adna.update_fitness(0.75);
    assert_eq!(adna.evolution.fitness_score, 0.75);

    // Проверяем ограничение
    adna.update_fitness(1.5);
    assert_eq!(adna.evolution.fitness_score, 1.0);
}
```

### Запуск тестов

```bash
cd src/core_rust
cargo test adna::tests
```

---

## Миграция с v1.0

ADNA v1.0 → v3.0 это **критическое изменение**. Ключевые отличия:

### ADNA v1.0 (Устаревшая)
- **Размер**: 256 байт
- **Назначение**: Статические параметры политики
- **Структура**: ADNAHeader + ADNAParameters
- **Обновления**: Ручные файлы конфигурации
- **Политика**: Хранится непосредственно в параметрах

### ADNA v3.0 (Текущая)
- **Размер**: 256 байт (тот же)
- **Назначение**: Динамический движок политик
- **Структура**: Header + Metrics + Pointer + Mapping (4x64)
- **Обновления**: Обучение на основе градиентов
- **Политика**: Отдельное хранилище, на основе указателей

### Шаги миграции

1. **Сделайте резервную копию старых конфигов ADNA**
2. **Конвертируйте параметры в LinearPolicy**:
   ```rust
   // Старое: параметры в ADNA
   let old_weights = adna_v1.parameters.weights;

   // Новое: отдельный объект политики
   let mut policy = LinearPolicy::new();
   for i in 0..8 {
       for j in 0..8 {
           policy.set_weight(i, j, old_weights[i][j]);
       }
   }
   ```

3. **Инициализируйте ADNA v3.0**:
   ```rust
   let mut adna = ADNA::new(PolicyType::Linear);
   adna.evolution.fitness_score = old_fitness;
   ```

4. **Настройте хранилище политики**:
   ```rust
   let policy_bytes = policy.serialize();
   adna.policy_ptr.policy_size = policy_bytes.len() as u32;
   adna.policy_ptr.policy_offset = write_to_storage(&policy_bytes);
   ```

---

## Дорожная карта

### v3.1 (Запланировано)
- [ ] Добавить вычисление `current_hash` (SHA256 всей ADNA)
- [ ] Реализовать версионное хранилище политик
- [ ] Добавить поддержку сжатия для больших политик

### v3.2 (Запланировано)
- [ ] Поддержка нейронных политик (интеграция PyTorch/ONNX)
- [ ] Многокритериальная приспособленность (фронт Парето)
- [ ] Автоматическое планирование скорости обучения

### v4.0 (Видение)
- [ ] Самомодифицирующаяся архитектура (ADNA эволюционирует свою собственную структуру)
- [ ] Мета-обучение между средами
- [ ] Квантово-вдохновлённые представления политик

---

## Ссылки

- [Полная спецификация ADNA v3.0](/home/chrnv/neurograph-os-mvp/docs/arch/ADNA v3.0.md)
- [Спецификация ExperienceToken](ExperienceStream_v2.0.md)
- [Реализация трейта Policy](../src/core_rust/src/policy.rs)
- [Примеры интеграции](../examples/)

---

**Версия документа:** 1.0
**Автор:** Команда NeuroGraph OS
**Последний обзор:** 2025-01-11
**Статус:** Активный