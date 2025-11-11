# ExperienceToken v3.0 - Токен Опыта

**Версия:** 3.0.0
**Размер:** 128 байт (выровнен по кэш-линии)
**Формат:** `#[repr(C, packed)]` для бинарной совместимости

---

## Обзор

ExperienceToken — это структура данных для захвата кортежей **состояние-действие-награда** (state-action-reward tuples), которые используются для обучения политик ADNA. Каждый токен опыта представляет собой один шаг в эпизоде взаимодействия агента со средой.

### Ключевые характеристики

- **128 байт** — точный размер для оптимизации кэша (2 кэш-линии по 64 байта)
- **4 блока по 32 байта** — машинно-дружественная структура
- **Приоритезированное воспроизведение** — система флагов для importance sampling
- **Отслеживание версий ADNA** — связь опыта с политикой, которая его сгенерировала
- **Компактное представление** — сжатое next_state для экономии памяти

---

## Философия и принципы дизайна

### 1. Reinforcement Learning First
ExperienceToken следует классической парадигме RL:
```
(s_t, a_t, r_t, s_{t+1})
```
где:
- `s_t` — текущее состояние (8D пространство)
- `a_t` — выбранное действие (8D пространство действий)
- `r_t` — полученная награда
- `s_{t+1}` — следующее состояние (сжато до 6D)

### 2. Эпизодическое обучение
- **Episode ID** — группировка опыта по эпизодам
- **Step Number** — порядок внутри эпизода
- **Terminal/Truncated флаги** — отметки окончания эпизода

### 3. Приоритезированный Replay Buffer
Система флагов позволяет реализовать Prioritized Experience Replay (PER):
- `HIGH_VALUE` / `LOW_VALUE` — важность опыта
- `NOVEL` — новизна состояния
- `SUCCESS` / `FAILURE` — исход действия

### 4. Policy Lineage Tracking
Поле `adna_version_hash` связывает опыт с конкретной версией политики ADNA, что позволяет:
- Отслеживать, какая политика сгенерировала данные
- Фильтровать устаревший опыт
- Анализировать эволюцию политики

---

## Раскладка памяти

```
┌─────────────────────────────────────────────────────────────────┐
│                    ExperienceToken (128 bytes)                   │
├─────────────────────────────────────────────────────────────────┤
│ Block 1: Header (32 bytes)                                       │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │ token_type: u32           (4 bytes)  - Magic 'EXPE'     │   │
│   │ timestamp: u64            (8 bytes)  - Unix epoch       │   │
│   │ episode_id: u64           (8 bytes)  - Episode ID       │   │
│   │ step_number: u32          (4 bytes)  - Step in episode  │   │
│   │ flags: u16                (2 bytes)  - Experience flags │   │
│   │ _reserved1: [u8; 6]       (6 bytes)  - Alignment        │   │
│   └─────────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│ Block 2: State Vector (32 bytes)                                │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │ state: [f32; 8]           (32 bytes) - Current state    │   │
│   │   [L1, L2, L3, L4, L5, L6, L7, L8]                      │   │
│   └─────────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│ Block 3: Action Vector (32 bytes)                               │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │ action: [f32; 8]          (32 bytes) - Taken action     │   │
│   │   [a1, a2, a3, a4, a5, a6, a7, a8]                      │   │
│   └─────────────────────────────────────────────────────────┘   │
├─────────────────────────────────────────────────────────────────┤
│ Block 4: Result (32 bytes)                                      │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │ reward: f32               (4 bytes)  - Reward value     │   │
│   │ next_state: [f32; 6]      (24 bytes) - Compressed s'    │   │
│   │ adna_version_hash: [u8;4] (4 bytes)  - Policy version   │   │
│   └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘

Общий размер: 128 байт (проверено на этапе компиляции)
```

---

## Определения структур

### ExperienceToken

```rust
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct ExperienceToken {
    // Header (32 bytes)
    pub token_type: u32,                // Magic number 0x45585045 'EXPE'
    pub timestamp: u64,                 // Unix epoch seconds
    pub episode_id: u64,                // Episode identifier
    pub step_number: u32,               // Step within episode
    pub flags: u16,                     // Experience + info flags
    pub _reserved1: [u8; 6],            // Alignment padding

    // State vector (32 bytes)
    pub state: [f32; 8],                // Current state (L1-L8)

    // Action vector (32 bytes)
    pub action: [f32; 8],               // Action taken

    // Result (32 bytes)
    pub reward: f32,                    // Reward received
    pub next_state: [f32; 6],           // Compressed next state
    pub adna_version_hash: [u8; 4],     // ADNA version that generated action
}

const _: () = assert!(std::mem::size_of::<ExperienceToken>() == 128);
```

### Система флагов

#### ExperienceFlags (младший байт + старший байт)

```rust
pub struct ExperienceFlags;

impl ExperienceFlags {
    // Флаги опыта (младший байт 0x00XX)
    pub const TERMINAL: u16      = 0x0001;  // Эпизод завершен
    pub const TRUNCATED: u16     = 0x0002;  // Эпизод прерван (timeout/error)
    pub const HIGH_VALUE: u16    = 0x0004;  // Высокая ценность для обучения
    pub const LOW_VALUE: u16     = 0x0008;  // Низкая ценность
    pub const EXPLORATION: u16   = 0x0010;  // Exploration action (vs exploit)
    pub const USER_FEEDBACK: u16 = 0x0020;  // Пользователь дал обратную связь

    // Информационные флаги (старший байт 0xXX00)
    pub const SUCCESS: u16       = 0x0100;  // Успешное выполнение
    pub const FAILURE: u16       = 0x0200;  // Неудача
    pub const TIMEOUT: u16       = 0x0400;  // Тайм-аут
    pub const ERROR: u16         = 0x0800;  // Ошибка
    pub const NOVEL: u16         = 0x1000;  // Новое состояние
}
```

**Комбинирование флагов:**
```rust
let mut token = ExperienceToken::new(1, 0);
token.set_flag(ExperienceFlags::TERMINAL | ExperienceFlags::SUCCESS);
```

---

## Справочник API

### Создание токенов

#### `ExperienceToken::new(episode_id: u64, step_number: u32) -> Self`
Создает новый токен опыта с минимальными параметрами.

```rust
let token = ExperienceToken::new(1, 0);
```

#### `ExperienceToken::with_data(...) -> Self`
Создает токен со всеми данными сразу.

```rust
let token = ExperienceToken::with_data(
    episode_id: 1,
    step_number: 5,
    state: [0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8],
    action: [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
    reward: 10.5,
    next_state: [0.2, 0.3, 0.4, 0.5, 0.6, 0.7],
    adna_hash: [0xAA, 0xBB, 0xCC, 0xDD],
);
```

### Проверки состояния

#### `is_valid(&self) -> bool`
Проверяет валидность токена по magic number.

```rust
if token.is_valid() {
    // Токен корректен
}
```

#### `is_done(&self) -> bool`
Проверяет, завершен ли эпизод (TERMINAL или TRUNCATED).

```rust
if token.is_done() {
    println!("Episode finished!");
}
```

#### `is_truncated(&self) -> bool`
Проверяет, был ли эпизод прерван досрочно.

#### `is_exploration(&self) -> bool`
Проверяет, было ли действие exploration (а не exploitation).

### Работа с флагами

#### `set_flag(&mut self, flag: u16)`
Устанавливает флаг.

```rust
token.set_flag(ExperienceFlags::HIGH_VALUE);
```

#### `clear_flag(&mut self, flag: u16)`
Снимает флаг.

```rust
token.clear_flag(ExperienceFlags::HIGH_VALUE);
```

#### `has_flag(&self, flag: u16) -> bool`
Проверяет наличие флага.

```rust
if token.has_flag(ExperienceFlags::SUCCESS) {
    println!("Action succeeded!");
}
```

### Удобные методы

#### `mark_terminal(&mut self)`
Отмечает эпизод как завершенный естественным образом.

```rust
token.mark_terminal();
```

#### `mark_truncated(&mut self)`
Отмечает эпизод как прерванный.

```rust
token.mark_truncated();
```

#### `mark_high_value(&mut self)`
Отмечает опыт как высокоценный для приоритезированного воспроизведения.

```rust
token.mark_high_value();
```

### Приоритезация

#### `priority(&self) -> f32`
Вычисляет приоритет для replay buffer на основе награды и флагов.

**Формула:**
```
priority = |reward| × multipliers

где multipliers:
- HIGH_VALUE: ×2.0
- LOW_VALUE: ×0.5
- NOVEL: ×1.5
```

**Пример:**
```rust
let priority = token.priority();
// Если reward = 5.0 и HIGH_VALUE установлен:
// priority = 5.0 × 2.0 = 10.0
```

---

## Точки интеграции

### 1. ADNA Policy Executor
ADNA использует ExperienceToken для записи результатов выполнения действий:

```rust
// ADNA выполняет действие
let action = adna.execute_policy(&state);

// Создаем токен опыта
let mut experience = ExperienceToken::with_data(
    episode_id,
    step_number,
    state,
    action,
    reward,
    next_state,
    adna.get_version_hash(),
);

// Отмечаем характеристики
if is_exploration {
    experience.set_flag(ExperienceFlags::EXPLORATION);
}
```

### 2. Experience Stream / Replay Buffer
ExperienceToken собираются в поток для последующего обучения:

```rust
struct ExperienceBuffer {
    buffer: Vec<ExperienceToken>,
    capacity: usize,
}

impl ExperienceBuffer {
    fn add(&mut self, experience: ExperienceToken) {
        self.buffer.push(experience);
        if self.buffer.len() > self.capacity {
            self.buffer.remove(0);
        }
    }

    fn sample_prioritized(&self, batch_size: usize) -> Vec<ExperienceToken> {
        // Сэмплирование с учетом приоритетов
        let priorities: Vec<f32> = self.buffer.iter()
            .map(|e| e.priority())
            .collect();

        // Importance sampling...
    }
}
```

### 3. Intuition Engine (Gradient Computation)
Intuition Engine анализирует ExperienceToken для вычисления градиентов:

```rust
impl IntuitionEngine {
    fn compute_gradient(&self, experience: &ExperienceToken) -> Gradient {
        // TD-error для critic
        let td_error = experience.reward
            + self.gamma * self.value(&experience.next_state)
            - self.value(&experience.state);

        // Policy gradient
        let policy_gradient = self.compute_policy_gradient(
            &experience.state,
            &experience.action,
            td_error,
        );

        Gradient::new(policy_gradient, GradientSource::TDLearning)
    }
}
```

### 4. Appraisers (Reward Calculation)
Апprейзеры вычисляют награды для ExperienceToken:

```rust
// Aesthetic Appraiser
let aesthetic_reward = aesthetic_appraiser.evaluate(&state, &action);

// Pragmatic Appraiser
let pragmatic_reward = pragmatic_appraiser.evaluate(&state, &action);

// Комбинированная награда
experience.reward = aesthetic_reward * 0.3 + pragmatic_reward * 0.7;
```

---

## Характеристики производительности

### Размер и выравнивание
- **128 байт** — 2 кэш-линии L1 (64 байта каждая)
- `#[repr(C, packed)]` — без паддинга, бинарная совместимость
- Compile-time assertion гарантирует размер

### Операции копирования
```rust
// Копирование структуры (128 байт)
let copy = token;  // memcpy 128 bytes

// Сериализация в бинарный формат
let bytes: [u8; 128] = unsafe {
    std::mem::transmute(token)
};
```

### Память для Replay Buffer
```
Replay buffer на 1 млн опытов:
1_000_000 × 128 bytes = 128 MB

Replay buffer на 10 млн опытов:
10_000_000 × 128 bytes = 1.28 GB
```

### Пропускная способность
На современных CPU (с AVX):
- **Копирование:** ~100M tokens/sec (~12.8 GB/s)
- **Обработка приоритетов:** ~50M tokens/sec
- **Сжатие в replay buffer:** зависит от алгоритма

---

## Примеры использования

### Пример 1: Базовый цикл обучения

```rust
use neurograph_core::{ExperienceToken, ExperienceFlags};

fn training_loop() {
    let mut episode_id = 0;

    loop {
        let mut state = env.reset();
        let mut step = 0;

        while !done {
            // ADNA выбирает действие
            let action = adna.select_action(&state);

            // Выполняем в среде
            let (next_state, reward, done) = env.step(&action);

            // Создаем токен опыта
            let mut experience = ExperienceToken::with_data(
                episode_id,
                step,
                state,
                action,
                reward,
                compress_state(&next_state),
                adna.version_hash(),
            );

            // Добавляем флаги
            if done {
                experience.mark_terminal();
                if reward > 0.0 {
                    experience.set_flag(ExperienceFlags::SUCCESS);
                }
            }

            // Сохраняем в replay buffer
            replay_buffer.add(experience);

            state = next_state;
            step += 1;
        }

        // Обучаем политику на батче
        let batch = replay_buffer.sample_prioritized(32);
        intuition_engine.train(&batch);

        episode_id += 1;
    }
}
```

### Пример 2: Фильтрация по версии ADNA

```rust
fn filter_recent_experience(
    buffer: &[ExperienceToken],
    current_adna_hash: [u8; 4],
) -> Vec<ExperienceToken> {
    buffer.iter()
        .filter(|e| e.adna_version_hash == current_adna_hash)
        .copied()
        .collect()
}
```

### Пример 3: Анализ эпизода

```rust
fn analyze_episode(buffer: &[ExperienceToken], episode_id: u64) {
    let episode: Vec<_> = buffer.iter()
        .filter(|e| e.episode_id == episode_id)
        .collect();

    let total_reward: f32 = episode.iter()
        .map(|e| e.reward)
        .sum();

    let success = episode.last()
        .map(|e| e.has_flag(ExperienceFlags::SUCCESS))
        .unwrap_or(false);

    println!("Episode {}: reward={}, success={}",
             episode_id, total_reward, success);
}
```

### Пример 4: Приоритезированная выборка

```rust
fn sample_with_priority(
    buffer: &[ExperienceToken],
    batch_size: usize,
) -> Vec<ExperienceToken> {
    use rand::prelude::*;

    // Вычисляем приоритеты
    let priorities: Vec<f32> = buffer.iter()
        .map(|e| e.priority())
        .collect();

    let total: f32 = priorities.iter().sum();

    // Нормализуем в вероятности
    let probs: Vec<f32> = priorities.iter()
        .map(|p| p / total)
        .collect();

    // Сэмплируем
    let mut rng = thread_rng();
    let mut batch = Vec::new();

    for _ in 0..batch_size {
        let mut cumsum = 0.0;
        let sample = rng.gen::<f32>();

        for (i, &prob) in probs.iter().enumerate() {
            cumsum += prob;
            if sample <= cumsum {
                batch.push(buffer[i]);
                break;
            }
        }
    }

    batch
}
```

---

## Тестирование

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_experience_token_size() {
        assert_eq!(std::mem::size_of::<ExperienceToken>(), 128);
    }

    #[test]
    fn test_flags() {
        let mut token = ExperienceToken::new(1, 0);

        assert!(!token.is_done());
        token.mark_terminal();
        assert!(token.is_done());

        token.set_flag(ExperienceFlags::SUCCESS);
        assert!(token.has_flag(ExperienceFlags::SUCCESS));
    }

    #[test]
    fn test_priority() {
        let mut token = ExperienceToken::new(1, 0);
        token.reward = 5.0;

        assert_eq!(token.priority(), 5.0);

        token.mark_high_value();
        assert_eq!(token.priority(), 10.0);
    }
}
```

### Integration Tests

```rust
#[test]
fn test_experience_collection() {
    let mut buffer = Vec::new();

    // Симулируем эпизод
    for step in 0..10 {
        let mut token = ExperienceToken::new(1, step);
        token.state = [step as f32; 8];
        token.action = [1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        token.reward = step as f32;

        if step == 9 {
            token.mark_terminal();
            token.set_flag(ExperienceFlags::SUCCESS);
        }

        buffer.push(token);
    }

    assert_eq!(buffer.len(), 10);
    assert!(buffer.last().unwrap().is_done());

    let total_reward: f32 = buffer.iter().map(|e| e.reward).sum();
    assert_eq!(total_reward, 45.0); // 0+1+2+...+9
}
```

---

## Дорожная карта

### v3.1 (Планируется)
- [ ] Добавить `trajectory_id` для multi-agent scenarios
- [ ] Расширить `next_state` до полных 8D (если позволяет память)
- [ ] Добавить `discount_factor` per-experience для variable gamma

### v3.2 (Будущее)
- [ ] Поддержка multi-step returns (n-step TD)
- [ ] Compression hooks для больших replay buffers
- [ ] Интеграция с distributed replay (Ray, MPI)

### v4.0 (Концепция)
- [ ] Hierarchical experience (meta-learning)
- [ ] Intrinsic motivation rewards
- [ ] Automatic curriculum learning signals

---

## Заключение

ExperienceToken v3.0 — это фундаментальный строительный блок для системы обучения NeuroGraph OS. Его дизайн оптимизирован для:

1. **Производительности** — машинно-дружественные 128 байт
2. **Гибкости** — система флагов для различных сценариев обучения
3. **Прослеживаемости** — связь с версиями ADNA через hash
4. **Простоты** — минималистичный API с мощными возможностями

Совместно с ADNA v3.0, ExperienceToken образует полный цикл обучения с подкреплением:

```
ADNA → Action → Environment → ExperienceToken → Intuition Engine → ADNA*
```

где `ADNA*` — улучшенная версия политики.