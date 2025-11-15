# Testing & Benchmarking v0.27.0

> **Baseline performance metrics, integration tests, and benchmark suite**

---

## Цели релиза

v0.27.0 фокусируется на **измерении и документировании производительности** системы после завершения core функциональности (v0.19-v0.26). Это критический этап перед экспериментальными изменениями (v0.28.0 Neural IntuitionEngine).

### Ключевые задачи

1. **Baseline Metrics**: Установить базовые показатели производительности для всех модулей
2. **Integration Tests**: Добавить E2E тесты для критических workflow
3. **Benchmark Suite**: Создать воспроизводимые бенчмарки с Criterion.rs
4. **Coverage Reporting**: Настроить измерение покрытия кода тестами
5. **Performance Profiling**: Идентифицировать узкие места

---

## 1. Benchmark Suite (Criterion.rs)

### 1.1 Зависимости

```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "token_bench"
harness = false

[[bench]]
name = "grid_bench"
harness = false

[[bench]]
name = "graph_bench"
harness = false

[[bench]]
name = "experience_stream_bench"
harness = false

[[bench]]
name = "intuition_bench"
harness = false
```

### 1.2 Token Benchmarks

**Файл**: `benches/token_bench.rs`

Измеряемые операции:
- `token_creation`: Создание 64-byte Token
- `token_similarity`: Вычисление cosine similarity между токенами
- `token_serialization`: Сериализация/десериализация (zero-copy)
- `token_batch_creation`: Создание 10,000 токенов

**Ожидаемые результаты**:
- Creation: <10 ns
- Similarity: <50 ns
- Serialization: <5 ns (zero-copy)
- Batch 10k: <100 μs

### 1.3 Grid Benchmarks

**Файл**: `benches/grid_bench.rs`

Измеряемые операции:
- `grid_insert`: Вставка токена в 8D grid
- `grid_knn_search`: KNN поиск (k=10) в grid с 10,000 токенов
- `grid_range_query`: Range query в 8D пространстве
- `grid_batch_insert`: Вставка 1,000 токенов

**Ожидаемые результаты**:
- Insert: <100 ns (с квантизацией)
- KNN (k=10): <5 μs (O(log n))
- Range query: <10 μs
- Batch 1k: <100 μs

### 1.4 Graph Benchmarks

**Файл**: `benches/graph_bench.rs`

Измеряемые операции:
- `graph_add_node`: Добавление узла
- `graph_add_connection`: Добавление связи (32 bytes)
- `graph_bfs`: BFS обход графа (1,000 узлов)
- `graph_dfs`: DFS обход графа (1,000 узлов)
- `graph_shortest_path`: Поиск кратчайшего пути

**Ожидаемые результаты**:
- Add node: <50 ns
- Add connection: <100 ns
- BFS 1k nodes: <500 μs
- DFS 1k nodes: <500 μs
- Shortest path: <1 ms

### 1.5 ExperienceStream Benchmarks

**Файл**: `benches/experience_stream_bench.rs`

Измеряемые операции:
- `write_event`: Запись 128-byte ExperienceEvent
- `write_event_with_metadata`: Запись события + ActionMetadata
- `read_event`: Чтение события по sequence number
- `sample_batch_uniform`: Uniform sampling (100 событий из 10,000)
- `sample_batch_prioritized`: Prioritized sampling по reward
- `broadcast_latency`: Latency pub-sub broadcast

**Ожидаемые результаты**:
- Write event: <200 ns (lock-free circular buffer)
- Write + metadata: <500 ns (HashMap insert)
- Read: <100 ns (direct array access)
- Sample 100 from 10k: <50 μs
- Broadcast latency: <1 μs

### 1.6 IntuitionEngine Benchmarks

**Файл**: `benches/intuition_bench.rs`

Измеряемые операции:
- `homeostasis_appraisal`: Оценка одного события (L5/L6/L8 проверки)
- `curiosity_appraisal`: Оценка новизны (L2 check)
- `efficiency_appraisal`: Оценка эффективности (L3+L5)
- `goal_directed_appraisal`: Оценка целенаправленности (L7)
- `all_appraisers_parallel`: Все 4 апрейзера параллельно
- `pattern_detection`: Анализ 1,000 событий для обнаружения паттернов

**Ожидаемые результаты**:
- Single appraiser: <100 ns
- All 4 parallel: <500 ns (tokio spawn overhead)
- Pattern detection 1k events: <10 ms (с квантизацией и статистикой)

---

## 2. Integration Tests

### 2.1 E2E Learning Loop Test

**Файл**: `tests/integration/learning_loop_e2e.rs`

**Сценарий**:
1. Инициализация: ExperienceStream + 4 Appraisers + IntuitionEngine + EvolutionManager
2. Генерация опыта: 500 событий с известным паттерном
3. Анализ: IntuitionEngine выявляет паттерн
4. Эволюция: EvolutionManager принимает Proposal
5. Верификация: Проверка, что ADNA политика обновлена корректно

**Assertions**:
- ✅ Обнаружен ровно 1 паттерн
- ✅ Создан ровно 1 Proposal
- ✅ Proposal принят (не отклонен CDNA)
- ✅ ADNA политика существует для обнаруженного state_bin
- ✅ Policy weights соответствуют ожиданиям

### 2.2 E2E Action Controller Test

**Файл**: `tests/integration/action_controller_e2e.rs`

**Сценарий**:
1. Инициализация: ActionController + ADNA + ExperienceStream + 2 executors
2. Создание Intent: Отправка сообщения
3. Policy selection: ADNA выбирает executor
4. Execution: Executor выполняет действие
5. Logging: Проверка, что action_started и action_finished события записаны

**Assertions**:
- ✅ Intent успешно выполнен
- ✅ 2 события в ExperienceStream (started + finished)
- ✅ ActionMetadata записана корректно
- ✅ Epsilon-greedy работает (10% exploration)

### 2.3 E2E Persistence Test

**Файл**: `tests/integration/persistence_e2e.rs` (requires `persistence` feature)

**Сценарий**:
1. Connect to PostgreSQL
2. Write 100 events with metadata
3. Write ADNA policy (2 versions)
4. Write configuration (2 versions)
5. Query events with filtering
6. Verify policy versioning
7. Verify config versioning
8. Archive old events

**Assertions**:
- ✅ Все 100 событий записаны и читаются корректно
- ✅ Metadata integrity (JSONB корректно сериализуется/десериализуется)
- ✅ Policy versioning работает (parent_policy_id корректен)
- ✅ Config versioning работает (parent_config_id корректен)
- ✅ Soft delete (is_active flag) работает
- ✅ Query filtering работает (episode_id, event_type, timestamp_range)

---

## 3. Coverage Reporting

### 3.1 Tools

```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Html --output-dir coverage

# Open report
open coverage/index.html
```

### 3.2 Coverage Targets

| Module | Current Coverage | Target v0.27.0 |
|--------|------------------|----------------|
| Token | ~80% | >90% |
| Connection | ~70% | >85% |
| Grid | ~75% | >85% |
| Graph | ~75% | >85% |
| Guardian + CDNA | ~60% | >80% |
| ExperienceStream | ~65% | >80% |
| IntuitionEngine | ~50% | >75% |
| EvolutionManager | ~40% | >70% |
| ActionController | ~45% | >70% |
| Persistence | ~30% | >60% (requires PostgreSQL) |

**Общая цель**: >75% line coverage для core модулей

---

## 4. Performance Profiling

### 4.1 Tools

```bash
# Install flamegraph
cargo install flamegraph

# Profile learning loop
cargo flamegraph --bin learning-loop-demo

# Profile action controller
cargo flamegraph --bin action-controller-demo
```

### 4.2 Profiling Targets

**Hotspots для анализа**:
1. ExperienceStream write path (должен быть lock-free)
2. ADNA state quantization (4 bins × 8 dimensions)
3. IntuitionEngine pattern detection (O(n log n) sorting)
4. PostgreSQL write transactions (batch optimizations?)

**Цели**:
- Идентифицировать топ-3 узких мест по CPU time
- Документировать allocation patterns (heap vs stack)
- Проверить отсутствие memory leaks (valgrind/miri)

---

## 5. Baseline Metrics Documentation

### 5.1 Performance Report

**Файл**: `docs/PERFORMANCE_BASELINE_v0.27.0.md`

Структура:
```markdown
# NeuroGraph OS - Performance Baseline v0.27.0

## Environment
- CPU: [model]
- RAM: [amount]
- OS: [version]
- Rust: [version]

## Benchmark Results

### Token Module
- Creation: X ns
- Similarity: X ns
- Serialization: X ns

### Grid Module
- Insert: X ns
- KNN (k=10): X μs
...

## Integration Test Results
- Learning Loop E2E: PASS (X ms)
- Action Controller E2E: PASS (X ms)
- Persistence E2E: PASS (X ms)

## Coverage Report
- Overall: X%
- Per module: [table]

## Identified Bottlenecks
1. [Description]
2. [Description]

## Recommendations for v0.28.0+
- [Optimization opportunity 1]
- [Optimization opportunity 2]
```

---

## 6. Implementation Plan

### Phase 1: Benchmark Suite (Day 1)
1. ✅ Create spec (this document)
2. Add criterion dependency
3. Implement Token benchmarks
4. Implement Grid benchmarks
5. Implement Graph benchmarks
6. Implement ExperienceStream benchmarks
7. Implement IntuitionEngine benchmarks
8. Run all benchmarks and collect baseline

### Phase 2: Integration Tests (Day 2)
1. Create `tests/integration/` directory
2. Implement Learning Loop E2E test
3. Implement Action Controller E2E test
4. Implement Persistence E2E test (optional feature)
5. Run all tests and verify PASS

### Phase 3: Coverage & Profiling (Day 3)
1. Install tarpaulin
2. Run coverage and generate report
3. Identify gaps in test coverage
4. Add missing unit tests to reach targets
5. Install flamegraph
6. Profile learning-loop-demo
7. Profile action-controller-demo
8. Analyze hotspots

### Phase 4: Documentation (Day 4)
1. Create PERFORMANCE_BASELINE_v0.27.0.md
2. Document benchmark results
3. Document integration test results
4. Document coverage metrics
5. Document identified bottlenecks
6. Update ROADMAP.md
7. Update README.md with v0.27.0 completion

---

## 7. Success Criteria

v0.27.0 считается завершенным, когда:

✅ **Benchmark Suite**:
- Все 6 benchmark файлов созданы и работают
- Criterion HTML reports генерируются
- Baseline results задокументированы

✅ **Integration Tests**:
- Минимум 3 E2E теста (Learning Loop, Action Controller, Persistence)
- Все тесты PASS
- CI/CD готов (опционально)

✅ **Coverage**:
- Tarpaulin установлен и работает
- Coverage report сгенерирован
- Общее покрытие >75% или gaps задокументированы

✅ **Profiling**:
- Flamegraph отчеты созданы для 2+ demo
- Топ-3 hotspots идентифицированы
- Recommendations задокументированы

✅ **Documentation**:
- PERFORMANCE_BASELINE_v0.27.0.md создан
- ROADMAP и README обновлены
- PROJECT_HISTORY обновлен

---

## 8. Non-Goals (Out of Scope)

**Что НЕ входит в v0.27.0**:
- ❌ Оптимизация производительности (это v0.28.0+)
- ❌ Исправление найденных узких мест (только документирование)
- ❌ CI/CD setup (GitHub Actions) - опционально
- ❌ Stress testing (1M+ events) - только baseline
- ❌ Multi-threaded benchmarks - только single-threaded baseline

---

## 9. Dependencies

**Новые зависимости**:
```toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }
```

**Tools (cargo install)**:
```bash
cargo install cargo-tarpaulin
cargo install flamegraph
```

---

## 10. Files to Create

```
neurograph-os-mvp/
├── benches/
│   ├── token_bench.rs
│   ├── grid_bench.rs
│   ├── graph_bench.rs
│   ├── experience_stream_bench.rs
│   └── intuition_bench.rs
├── tests/
│   └── integration/
│       ├── learning_loop_e2e.rs
│       ├── action_controller_e2e.rs
│       └── persistence_e2e.rs
├── docs/
│   ├── specs/
│   │   └── Testing_Benchmarking_v0.27.0.md (this file)
│   └── PERFORMANCE_BASELINE_v0.27.0.md (to be created)
└── src/core_rust/Cargo.toml (update with criterion)
```

---

## Заключение

v0.27.0 - это **фундамент для будущих оптимизаций**. Без baseline metrics невозможно объективно оценить улучшения от Neural IntuitionEngine (v0.28.0) или других экспериментов.

**Ключевой принцип**: Measure → Document → Optimize. v0.27.0 фокусируется на первых двух шагах.

---

*Версия: 1.0*
*Дата: 2025-01-14*
*Статус: DRAFT → READY FOR IMPLEMENTATION*