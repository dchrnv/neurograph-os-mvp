# NeuroGraph OS — План реализации v0.35.0 → v1.0.0

**Версия плана:** 2.0 (компактная)  
**Дата:** 2025-01-25  
**Базовая версия системы:** v0.34.0

---

## Критический путь

```
v0.34.0 → Gateway (v0.35.0) → REPL (v0.36.0) → Feedback (v0.37.0) → Curiosity (v0.38.0)
                                                                            ↓
                                                         ┌──────────────────┼──────────────────┐
                                                         ↓                  ↓                  ↓
                                                   REST API (v0.39)   Python (v0.40)    Desktop UI (v0.41)
                                                         └──────────────────┴──────────────────┘
                                                                            ↓
                                                                      v1.0.0 Release
```

---

## v0.35.0 — Gateway v1.0

**Цель:** Единая точка входа + response channels  
**Время:** 5-6 часов  
**Путь:** `src/core_rust/src/gateway/`

### Архитектура

```
InputSignal → Gateway.inject() → ProcessedSignal → Queue → ActionController
                    ↓                                              ↓
            (SignalReceipt, ResultReceiver) ←───────── complete_request(result)
```

### Шаги

**Шаг 1: signals.rs (30 мин)**
- [ ] `InputSignal` enum: Text, SystemTick, DirectToken, DirectState, Command, Feedback
- [ ] `SignalSource` enum: Console, RestApi, WebSocket, InternalTimer, InternalCuriosity
- [ ] `ProcessedSignal` struct: signal_id, state[f32;8], signal_type, source, metadata
- [ ] `SignalType` enum: SemanticQuery, ActionRequest, FeedbackSignal, CuriosityTrigger

**Шаг 2: config.rs (20 мин)**
- [ ] `GatewayConfig`: queue_capacity, processing_timeout_ms, tick_interval_ms
- [ ] `UnknownWordStrategy` enum: Ignore, CreateEmpty, TriggerCuriosity, UseNearest

**Шаг 3: normalizer.rs (40 мин)**
- [ ] `Normalizer` struct с Arc<RwLock<BootstrapLibrary>>
- [ ] `normalize_text(text)` → NormalizationResult { state, matched_tokens, unknown_words }
- [ ] `coords_to_state(coords[3], token_id)` → state[f32;8]
- [ ] `handle_unknown_word(word)` по стратегии из config

**Шаг 4: channels.rs (30 мин)**
- [ ] `SignalReceipt` struct: signal_id, received_at, queue_position
- [ ] `ResultReceiver` = oneshot::Receiver<ActionResult>
- [ ] `PendingRequests` = DashMap<u64, oneshot::Sender<ActionResult>>

**Шаг 5: Gateway struct (30 мин)**
`mod.rs`
- [ ] sender: mpsc::Sender<ProcessedSignal>
- [ ] normalizer: Normalizer
- [ ] pending_requests: PendingRequests
- [ ] stats: Arc<RwLock<GatewayStats>>
- [ ] НЕ создавать свой tokio runtime — использовать текущий

**Шаг 6: Gateway.inject() (40 мин)**
- [ ] `async fn inject(&self, signal) -> Result<(SignalReceipt, ResultReceiver), GatewayError>`
- [ ] Создать oneshot channel
- [ ] Сохранить sender в pending_requests
- [ ] Нормализовать signal → ProcessedSignal
- [ ] Отправить в queue
- [ ] Вернуть (receipt, receiver)

**Шаг 7: Gateway.complete_request() (20 мин)**
- [ ] `fn complete_request(&self, signal_id: u64, result: ActionResult)`
- [ ] Извлечь sender из pending_requests.remove()
- [ ] Отправить result через sender
- [ ] `fn cleanup_stale_requests(&self, max_age_ms: u64)` — периодическая очистка

**Шаг 8: classify_text() (15 мин)**
- [ ] `/` prefix → SystemSignal
- [ ] `?` suffix или what/who/где/что → SemanticQuery
- [ ] create/connect/создай → ActionRequest
- [ ] default → SemanticQuery

**Шаг 9: stats.rs (15 мин)**
- [ ] total_signals, text_signals, tick_signals, unknown_words
- [ ] total_processing_time_us, queue_overflows, timeouts

**Шаг 10: ActionController интеграция (30 мин)**
- [ ] `ActionController::with_gateway(receiver, gateway: Arc<Gateway>)`
- [ ] В run loop после execute: `gateway.complete_request(signal_id, result)`

**Шаг 11: Тесты (30 мин)**
- [ ] test_inject_returns_receipt_and_receiver
- [ ] test_complete_request_delivers_result
- [ ] test_normalizer_simple_word
- [ ] test_unknown_word_strategies

**Шаг 12: lib.rs + cargo check (15 мин)**

---

## v0.36.0 — REPL

**Цель:** Первый интерфейс с синхронным request/response  
**Время:** 3-4 часа  
**Путь:** `src/core_rust/src/adapters/`, `bin/repl.rs`

### Шаги

**Шаг 1: adapters/mod.rs (20 мин)**
- [ ] trait OutputAdapter: format_output(), send()
- [ ] FormattedOutput { text, data }
- [ ] OutputContext { signal_id, original_input }

**Шаг 2: adapters/console.rs (40 мин)**
- [ ] ConsoleOutputAdapter: format ActivationResult, errors
- [ ] ConsoleInputAdapter: stdin loop → Gateway

**Шаг 3: Request/Response flow (30 мин)**
- [ ] inject() → (receipt, receiver)
- [ ] tokio::time::timeout(Duration, receiver.await)
- [ ] Handle timeout → "Request timed out"

**Шаг 4: bin/repl.rs (50 мин)**
- [ ] #[tokio::main]
- [ ] Init: Bootstrap, Gateway, ActionController
- [ ] Spawn controller task
- [ ] Input loop: read → inject → await → output
- [ ] Graceful shutdown

**Шаг 5: Команды (30 мин)**
- [ ] `/status`, `/stats`, `/help`, `/quit`

**Шаг 6: Cargo.toml + smoke test (20 мин)**
- [ ] `[[bin]] name = "neurograph-repl"`

---

## v0.37.0 — Feedback Loop

**Цель:** Обучение от пользователя  
**Время:** 3-4 часа  
**Путь:** `src/core_rust/src/feedback/`

### Шаги

**Шаг 1: FeedbackType (15 мин)**
- [ ] Positive { strength }, Negative { strength }
- [ ] Correction { correct_value }, Association { related_word, strength }

**Шаг 2: FeedbackProcessor (30 мин)**
- [ ] Ссылки на ExperienceStream, IntuitionEngine, Bootstrap
- [ ] `async fn process(signal) -> FeedbackResult`

**Шаг 3: Validation (20 мин)**
- [ ] Проверить reference_id существует
- [ ] Проверить feedback не старше MAX_FEEDBACK_AGE (1 час)
- [ ] Limit corrections per signal (max 3)

**Шаг 4: apply_positive/negative (30 мин)**
- [ ] Update reward в ExperienceStream
- [ ] Reinforce/weaken reflex в IntuitionEngine

**Шаг 5: apply_correction (40 мин)**
- [ ] Parse "X это Y" → создать связь
- [ ] Создать новый токен если нужно
- [ ] Создать Connection

**Шаг 6: REPL интеграция (30 мин)**
- [ ] После ответа: `[y/n/c] ` prompt
- [ ] Неизвестные слова → "What is X?"

**Шаг 7: Тесты (25 мин)**

---

## v0.38.0 — Curiosity Drive

**Цель:** Автономное исследование  
**Время:** 6-7 часов  
**Путь:** `src/core_rust/src/curiosity/`

### Шаги

**Шаг 1: config.rs (20 мин)**
- [ ] boredom_threshold, surprise/uncertainty/novelty weights
- [ ] exploration_interval_ms, enable_autonomous
- [ ] exploration_mode: Quiet | Verbose

**Шаг 2: uncertainty.rs (50 мин)**
- [ ] CellKey [i32;8], CellConfidence { confidence, visit_count, last_visit }
- [ ] HashMap<CellKey, CellConfidence>
- [ ] get_uncertainty(), update()
- [ ] **cleanup_old_cells(max_age, min_visits)** — удалять старые неиспользуемые

**Шаг 3: surprise.rs (30 мин)**
- [ ] SurpriseHistory (VecDeque)
- [ ] calculate_surprise(predicted, actual) → f32

**Шаг 4: NoveltyTracker (20 мин)**
- [ ] HashMap<CellKey, u64> last_seen
- [ ] calculate_novelty(state) → f32

**Шаг 5: exploration.rs (30 мин)**
- [ ] ExplorationTarget { state, score, reason, priority }
- [ ] BinaryHeap (priority queue)

**Шаг 6: CuriosityDrive struct (30 мин)**
- [ ] Все компоненты + gateway_sender
- [ ] calculate_curiosity(state, context) → CuriosityScore

**Шаг 7: Autonomous loop (50 мин)**
- [ ] `async fn run_autonomous_exploration(controller)`
- [ ] Check exploration_mode: Quiet → без REPL output
- [ ] Периодический cleanup_old_cells()
- [ ] Pop queue или find uncertain regions

**Шаг 8: ActionController integration (30 мин)**
- [ ] act_with_curiosity()
- [ ] explore()

**Шаг 9: REPL команды (20 мин)**
- [ ] `/curiosity`, `/explore`

**Шаг 10: Тесты (40 мин)**

---

## v0.39.0 — REST API

**Цель:** HTTP доступ  
**Время:** 4-5 часов  
**Путь:** `src/core_rust/src/api/`  
**Framework:** axum

### Шаги

**Шаг 1: Dependencies (10 мин)**
- [ ] axum, tower, tower-http

**Шаг 2: models.rs (30 мин)**
- [ ] QueryRequest/Response, FeedbackRequest/Response, StatusResponse

**Шаг 3: state.rs (20 мин)**
- [ ] ApiState { gateway, controller, config }

**Шаг 4: handlers.rs (60 мин)**
- [ ] POST /api/v1/query, /feedback
- [ ] GET /api/v1/status, /stats, /health

**Шаг 5: router.rs (30 мин)**
- [ ] Routes + CORS + logging

**Шаг 6: Optional auth (20 мин)**
- [ ] X-API-Key header check
- [ ] Rate limiting

**Шаг 7: websocket.rs (40 мин)**
- [ ] WS upgrade, bidirectional

**Шаг 8: bin/api.rs + тесты (40 мин)**

---

## v0.40.0 — Python Bindings

**Цель:** Python API  
**Время:** 4-5 часов  
**Путь:** `src/python/`

### Шаги

**Шаг 1: Setup (20 мин)**
- [ ] Cargo.toml (pyo3, cdylib), pyproject.toml (maturin)

**Шаг 2: Runtime class (60 мин)**
- [ ] #[pyclass] Runtime
- [ ] Внутренний tokio::Runtime (создаётся один раз)
- [ ] `self.runtime.block_on()` для async→sync
- [ ] `py.allow_threads()` для GIL release

**Шаг 3: Methods (40 мин)**
- [ ] bootstrap(), query(), activate(), feedback(), status(), stats()

**Шаг 4: Result classes (30 мин)**
- [ ] QueryResult, ActivationResult с to_dict(), top(n)

**Шаг 5: Type stubs (20 мин)**
- [ ] neurograph.pyi

**Шаг 6: Tests + example (40 мин)**

---

## v0.41.0 — Desktop UI

**Цель:** Визуализация  
**Время:** 8-10 часов  
**Путь:** `src/desktop/`  
**Framework:** iced (уже используется)

### Шаги

**Шаг 0: Framework check (15 мин)**
- [ ] Подтвердить iced версию
- [ ] GPU acceleration если нужно

**Шаг 1: Gateway bridge (40 мин)**
**Шаг 2: Query input (30 мин)**
**Шаг 3: Results view (60 мин)**
**Шаг 4: Curiosity dashboard (60 мин)**
**Шаг 5: Metrics panel (40 мин)**
**Шаг 6: Integration (60 мин)**

---

## v1.0.0 — Release

**Время:** 4-6 часов

- [ ] Все тесты проходят
- [ ] cargo clippy clean
- [ ] Документация актуальна
- [ ] README, CHANGELOG
- [ ] Examples работают
- [ ] Benchmarks

---

## Сводка времени

| Версия | Время | Зависимости |
|--------|-------|-------------|
| v0.35.0 Gateway | 5-6 ч | v0.34.0 |
| v0.36.0 REPL | 3-4 ч | Gateway |
| v0.37.0 Feedback | 3-4 ч | REPL |
| v0.38.0 Curiosity | 6-7 ч | Feedback |
| v0.39.0 REST API | 4-5 ч | Gateway |
| v0.40.0 Python | 4-5 ч | Gateway |
| v0.41.0 Desktop UI | 8-10 ч | Gateway + Curiosity |
| v1.0.0 Release | 4-6 ч | Все |

**Критический путь:** ~18-21 ч  
**Общее:** ~40-50 ч

---

## Ключевые изменения (v2.0)

1. **Gateway**: Response channel pattern (oneshot)
2. **REPL**: Request tracking с timeout
3. **Feedback**: Validation (age, limit)
4. **Curiosity**: Memory cleanup, Quiet/Verbose mode
5. **REST API**: Optional auth
6. **Python**: Explicit block_on + GIL release
7. **Desktop UI**: Confirmed iced, increased estimate
