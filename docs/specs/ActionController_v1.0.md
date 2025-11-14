# ActionController v1.0 - Specification

**Version:** 1.0
**Date:** 2025-01-13
**Status:** Implementation Ready
**Based on:** docs/arch/4/ActionController.md

---

## 1. Обзор

**ActionController** — центральный диспетчер действий, который замыкает цикл "восприятие → обучение → действие".

### Основная задача
Получать от системы намерение (`Intent`), выбирать на основе `ADNA` политик наиболее подходящий исполнительный модуль (`ActionExecutor`), инициировать выполнение и логировать результат в `ExperienceStream`.

### Ключевые принципы
- **Декомпозиция**: ЧТО делать (ActionController + ADNA) vs КАК делать (ActionExecutor)
- **Управление через ADNA**: Изменение ADNA политик меняет поведение системы
- **Полная наблюдаемость**: Каждое действие логируется в ExperienceStream
- **Расширяемость**: Легко добавлять новые executors без изменения ядра

---

## 2. Архитектура

```
Intent → ActionController → ADNA (get policy) → Select Executor → Execute → Log Result
                ↓                                                              ↓
          ExperienceStream ←───────────────────────────────────────────────────┘
```

### Компоненты

1. **ActionController**: Центральный диспетчер
2. **ActionExecutor trait**: Общий интерфейс для всех исполнителей
3. **Concrete Executors**: Специализированные исполнители (TokenMover, MessageSender, etc.)
4. **ADNA Integration**: ADNAReader для получения ActionPolicy
5. **ExperienceStream Integration**: Логирование начала и результата действий

---

## 3. Структуры данных

### 3.1. Intent (уже есть в adna.rs)
```rust
pub struct Intent {
    pub intent_type: String,        // "move_token", "send_message", etc.
    pub context: serde_json::Value, // Вся необходимая информация
    pub state: [i16; 8],            // L1-L8 координаты для контекста
}
```

### 3.2. ActionPolicy (уже есть в adna.rs)
```rust
pub struct ActionPolicy {
    pub action_weights: HashMap<u16, f64>, // action_type → weight
    pub rule_id: String,
    pub last_updated: SystemTime,
    pub metadata: serde_json::Value,
}
```

### 3.3. ActionResult (новая)
```rust
pub struct ActionResult {
    pub success: bool,
    pub output: serde_json::Value,  // Результат в виде JSON
    pub duration_ms: u64,
    pub error: Option<String>,
}
```

### 3.4. ActionError (новая)
```rust
pub enum ActionError {
    ExecutorNotFound(String),
    PolicyNotFound(String),
    ExecutionFailed(String),
    InvalidParameters(String),
}
```

---

## 4. ActionExecutor Trait

Все исполнители должны реализовывать единый async trait:

```rust
#[async_trait]
pub trait ActionExecutor: Send + Sync {
    /// Уникальный ID исполнителя
    fn id(&self) -> &str;

    /// Описание возможностей
    fn description(&self) -> &str;

    /// Выполнить действие с заданными параметрами
    async fn execute(&self, params: serde_json::Value) -> ActionResult;

    /// Валидация параметров перед выполнением (опционально)
    fn validate_params(&self, params: &serde_json::Value) -> Result<(), String> {
        Ok(())
    }
}
```

---

## 5. ActionController API

### 5.1. Структура
```rust
pub struct ActionController {
    adna_reader: Arc<dyn ADNAReader>,
    experience_writer: Arc<dyn ExperienceWriter>,
    executors: RwLock<HashMap<String, Arc<dyn ActionExecutor>>>,
    config: ActionControllerConfig,
}

pub struct ActionControllerConfig {
    pub exploration_rate: f64,      // Epsilon для epsilon-greedy (default: 0.1)
    pub log_all_actions: bool,      // Логировать все действия (default: true)
    pub timeout_ms: u64,            // Таймаут выполнения (default: 30000)
}
```

### 5.2. Основные методы

#### new()
```rust
pub fn new(
    adna_reader: Arc<dyn ADNAReader>,
    experience_writer: Arc<dyn ExperienceWriter>,
    config: ActionControllerConfig,
) -> Self
```

#### register_executor()
```rust
pub fn register_executor(&self, executor: Arc<dyn ActionExecutor>) -> Result<(), ActionError>
```

#### execute_intent()
```rust
pub async fn execute_intent(&self, intent: Intent) -> Result<ActionResult, ActionError>
```

**Алгоритм execute_intent():**
1. Получить `ActionPolicy` из ADNA через `adna_reader.get_action_policy(&intent.state)`
2. Выбрать executor на основе action_weights с exploration/exploitation strategy
3. Логировать `action_started` event в ExperienceStream
4. Валидировать параметры через `executor.validate_params()`
5. Выполнить `executor.execute()` с timeout
6. Логировать `action_finished` event с результатом
7. Вернуть ActionResult

### 5.3. Exploration/Exploitation

**Epsilon-greedy strategy:**
- С вероятностью `exploration_rate`: выбрать случайный executor (exploration)
- С вероятностью `1 - exploration_rate`: выбрать executor с максимальным весом (exploitation)

---

## 6. Event Types для ExperienceStream

### 6.1. action_started (event_type: 1000)
```json
{
  "intent_type": "move_token",
  "executor_id": "token_mover",
  "state": [0.5, 0.2, ...],
  "timestamp": 1234567890
}
```

### 6.2. action_finished (event_type: 1001)
```json
{
  "intent_type": "move_token",
  "executor_id": "token_mover",
  "success": true,
  "duration_ms": 123,
  "output_summary": "moved token 42 to [1.0, 2.0, 3.0]"
}
```

---

## 7. Базовые Executors (для демо)

### 7.1. TokenMoverExecutor
**ID:** `token_mover`
**Описание:** Перемещает Token в Grid
**Параметры:**
```json
{
  "token_id": 42,
  "target_position": [1.0, 2.0, 3.0]
}
```

### 7.2. MessageSenderExecutor
**ID:** `message_sender`
**Описание:** Отправляет сообщение в лог
**Параметры:**
```json
{
  "message": "Hello from ActionController!",
  "priority": "info"
}
```

### 7.3. NoOpExecutor
**ID:** `noop`
**Описание:** Пустое действие (для тестирования)
**Параметры:** `{}`

---

## 8. Интеграция с ADNA

### 8.1. Расширение ADNAReader trait

Добавить метод:
```rust
async fn get_action_policy(&self, state: &[i16; 8]) -> Result<ActionPolicy, String>;
```

### 8.2. Реализация в InMemoryADNAReader

- Квантизовать state в state_bin_id (используя ту же логику, что в IntuitionEngine)
- Искать ActionPolicy для этого bin_id
- Если не найдена — вернуть default policy (uniform weights)

---

## 9. Замыкание Learning Loop

**Полный цикл:**
```
1. ActionController.execute_intent(Intent)
2. → ADNA.get_action_policy() → выбор executor
3. → Executor.execute() → реальное действие
4. → ExperienceStream.push() → логирование
5. → Appraisers → оценка вознаграждений
6. → IntuitionEngine → анализ паттернов
7. → EvolutionManager → обновление ADNA
8. → Новый execute_intent() использует обновлённую политику ✓
```

---

## 10. Тестирование

### Unit tests
- `test_register_executor()` — регистрация executors
- `test_execute_intent_success()` — успешное выполнение
- `test_execute_intent_executor_not_found()` — ошибка
- `test_exploration_exploitation()` — epsilon-greedy работает

### Integration tests
- E2E demo с полным циклом (см. раздел 11)

---

## 11. E2E Demo Scenario

**Сценарий:** Система учится выбирать оптимальный способ перемещения токена.

**Setup:**
1. Два executors: `fast_mover` (быстрый, но неточный) и `precise_mover` (медленный, но точный)
2. ADNA с начальной политикой (uniform weights)
3. Appraisers оценивают точность и скорость

**Процесс:**
1. Генерировать 100 Intents для перемещения токенов
2. ActionController выполняет действия, выбирая executors
3. Appraisers награждают за точность, штрафуют за время
4. IntuitionEngine обнаруживает: `precise_mover` → higher reward
5. EvolutionManager обновляет ADNA: увеличивает вес `precise_mover`
6. Последующие действия чаще используют `precise_mover`

**Expected result:** После ~50 действий система начинает предпочитать `precise_mover` в 80%+ случаев.

---

## 12. Файловая структура

```
src/core_rust/src/
  action_controller.rs       # ActionController struct
  action_executor.rs         # ActionExecutor trait + ActionResult + ActionError
  executors/
    mod.rs                   # Re-exports
    token_mover.rs           # TokenMoverExecutor
    message_sender.rs        # MessageSenderExecutor
    noop.rs                  # NoOpExecutor
  bin/
    action-controller-demo.rs  # E2E demo
```

---

## 13. Зависимости

Новые:
- Нет (используем уже добавленные: serde_json, async-trait, tokio)

---

## 14. Метрики успеха

- ✅ ActionController успешно выполняет Intents
- ✅ Executors регистрируются и вызываются
- ✅ События логируются в ExperienceStream
- ✅ ADNA политики влияют на выбор executors
- ✅ E2E demo показывает обучение (изменение предпочтений)
- ✅ Exploration/exploitation работает корректно

---

## 15. Следующие шаги (после v1.0)

**v1.1 (будущее):**
- Softmax selection вместо epsilon-greedy
- Multi-step action sequences (планирование)
- Action rollback capability
- Priority queue для Intents
- Parallel execution нескольких Intents

---

*Спецификация готова к реализации.*