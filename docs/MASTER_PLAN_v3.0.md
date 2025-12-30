# NeuroGraph OS - Master Plan v3.1

**Версия:** 3.1
**Дата:** 2024-12-30
**Статус:** Active Development Plan
**Предыдущие версии:**
- [MASTER_PLAN v3.0](archive/MASTER_PLAN_v3.0.md) - Full Platform Vision (2024-12-26)
- [MASTER_PLAN_v2.1.md](archive/MASTER_PLAN_v2.1.md) - Signal Processing Focus
- [IMPLEMENTATION_ROADMAP.md](archive/IMPLEMENTATION_ROADMAP.md) - Full Stack Focus

---

## 🎯 Общая стратегия

Построить полноценную **когнитивную платформу** NeuroGraph OS с тремя ключевыми направлениями:

```
┌─────────────────────────────────────────────────────────────┐
│                    NeuroGraph OS Platform                   │
├─────────────────────────────────────────────────────────────┤
│  ТРЕК A: Core Intelligence (Signal Processing) ✅           │
│  ТРЕК B: Developer Experience (Python Library, Jupyter)     │
│  ТРЕК C: User Interfaces (Web Dashboard, APIs) ✅           │
└─────────────────────────────────────────────────────────────┘
```

---

## 📊 Текущее состояние (2024-12-30)

### ✅ Что работает

#### ТРЕК A: Core Intelligence ✅ ЗАВЕРШЁН (v0.57.0)

**Gateway v2.0 + SignalSystem v1.1 + ActionController**
- ✅ Полный pipeline: Input → Gateway → Core → ActionController → Output
- ✅ Performance: 304,553 events/sec, 0.39μs avg latency
- ✅ Rust core с PyO3 bindings
- ✅ Subscription filters, novelty detection, pattern matching
- ✅ Production infrastructure (REST API, Prometheus, OpenTelemetry, Docker)

#### ТРЕК C: User Interfaces ✅ ЗАВЕРШЁН (v0.62.0)

**Web Dashboard (React SPA)**
- ✅ 7 функциональных страниц (Dashboard, Modules, Config, Bootstrap, Chat, Terminal, Admin)
- ✅ 35+ файлов, 3,512+ строк TypeScript/TSX кода
- ✅ 15+ переиспользуемых компонентов
- ✅ 4 Zustand stores с localStorage persistence
- ✅ Real-time WebSocket communication
- ✅ Полная интернационализация EN/RU (160+ ключей)
- ✅ Dark/Light темы
- ✅ Error boundaries и 404 обработка
- ✅ Connection status monitoring
- ✅ Responsive design для всех экранов
- ✅ Automation scripts (./start-all.sh, ./stop-all.sh)

**Performance Metrics (v0.62.0):**
- Load time: < 2s
- Time to interactive: < 3s
- Lighthouse score: > 90
- Mobile responsive: 100%

---

## ❌ Что нужно реализовать

### ТРЕК B: Developer Experience (4 версии)

**v0.59.0 - Python Library (neurograph package)** - Не начато
**v0.60.0 - WebSocket & Real-time Events** - Не начато
**v0.61.0 - Jupyter Integration** - Не начато

### ТРЕК C: Module Management (1 версия)

**v0.63.0 - Module Registry** - Спецификация готова

### ТРЕК A: Enhanced Capabilities (1 версия)

**v0.64.0 - Enhanced Sensors (Audio & Vision)** - Не начато

---

## 🗺️ Roadmap v3.1 (Next 2 Releases)

---

## v0.63.0 - Module Registry System 🔧

**Цель:** Реализовать систему управления модулями с enable/disable функциональностью

**Приоритет:** 🟡 ВЫСОКИЙ

**Длительность:** 4-5 дней

**Спецификация:** [MODULE_REGISTRY_v0_63_0_SPEC.md](specs/MODULE_REGISTRY_v0_63_0_SPEC.md)

### Обзор концепции

Модули NeuroGraph — это **компоненты единого Rust ядра**, а не отдельные процессы. Вместо start/stop используем **enable/disable**:

```
Start/Stop (невозможно):        Enable/Disable (реализуемо):
┌─────────────┐                 ┌─────────────┐
│   Module    │                 │   Module    │
│  [Process]  │ ← kill/spawn    │  [in Rust]  │ ← enabled: bool
└─────────────┘                 └─────────────┘
```

### 10 ключевых модулей для управления

| ID | Название | Описание | Можно отключить? |
|----|----------|----------|------------------|
| `token_manager` | TokenManager | Хранение и управление токенами | ❌ Нет (core) |
| `connection_manager` | ConnectionManager | Хранение связей между токенами | ❌ Нет (core) |
| `grid` | Grid | Пространственный индекс (8D) | ❌ Нет (core) |
| `intuition_engine` | IntuitionEngine | Интуитивная обработка запросов | ✅ Да |
| `signal_system` | SignalSystem | Обработка и маршрутизация сигналов | ✅ Да |
| `gateway` | Gateway | Входные сенсоры и энкодеры | ✅ Да |
| `action_controller` | ActionController | Выходные действия | ✅ Да |
| `guardian` | Guardian | Валидация и защита (CDNA) | ⚠️ Да (опасно) |
| `cdna` | CDNA | Конституция системы | ❌ Нет (core) |
| `bootstrap` | Bootstrap | Загрузка embeddings | ❌ Нет (статус only) |

### Phase 1: Rust Core (2 дня)

**Задачи:**
- [ ] **1.1** Создать `module_id.rs` с enum ModuleId
  ```rust
  pub enum ModuleId {
      TokenManager,
      ConnectionManager,
      Grid,
      IntuitionEngine,
      SignalSystem,
      Gateway,
      ActionController,
      Guardian,
      Cdna,
      Bootstrap,
  }
  ```

- [ ] **1.2** Создать `module_registry.rs` с реестром
  ```rust
  pub struct ModuleRegistry {
      enabled: RwLock<HashMap<ModuleId, bool>>,
      configs: RwLock<HashMap<ModuleId, ModuleConfig>>,
      metrics: RwLock<HashMap<ModuleId, ModuleMetrics>>,
      statuses: RwLock<HashMap<ModuleId, ModuleStatus>>,
  }

  impl ModuleRegistry {
      pub fn is_enabled(&self, module: ModuleId) -> bool
      pub fn set_enabled(&self, module: ModuleId, enabled: bool) -> Result<(), String>
      pub fn get_module_info(&self, module: ModuleId) -> ModuleInfo
      pub fn get_all_modules(&self) -> Vec<ModuleInfo>
  }
  ```

- [ ] **1.3** Создать `python/modules.rs` с FFI bindings
  ```rust
  #[pyfunction]
  pub fn list_modules(py: Python<'_>) -> PyResult<Vec<PyObject>>

  #[pyfunction]
  pub fn get_module(py: Python<'_>, module_id: &str) -> PyResult<PyObject>

  #[pyfunction]
  pub fn set_module_enabled(module_id: &str, enabled: bool) -> PyResult<()>
  ```

- [ ] **1.4** Интегрировать проверку `is_enabled()` в модули:
  - [ ] IntuitionEngine::process()
  - [ ] SignalSystem::emit()
  - [ ] Gateway::push()
  - [ ] ActionController::execute()
  - [ ] Guardian::validate()

- [ ] **1.5** Тесты для ModuleRegistry

**Файлы:**
- `src/core_rust/src/module_id.rs` (NEW)
- `src/core_rust/src/module_registry.rs` (NEW)
- `src/core_rust/src/python/modules.rs` (NEW)
- `src/core_rust/src/lib.rs` (UPDATE)
- `src/core_rust/src/intuition.rs` (UPDATE)
- `src/core_rust/src/signal_system.rs` (UPDATE)
- `src/core_rust/src/gateway.rs` (UPDATE)
- `src/core_rust/src/action_controller.rs` (UPDATE)
- `src/core_rust/src/guardian.rs` (UPDATE)

### Phase 2: Python API Layer (1 день)

**Задачи:**
- [ ] **2.1** Создать Pydantic models (`src/api/models/modules.py`)
  ```python
  class ModuleStatus(str, Enum):
      ACTIVE = "active"
      DISABLED = "disabled"
      ERROR = "error"

  class ModuleMetrics(BaseModel):
      operations: int
      ops_per_sec: float
      avg_latency_us: float
      p95_latency_us: float
      errors: int

  class ModuleInfo(BaseModel):
      id: str
      name: str
      description: str
      version: str
      status: ModuleStatus
      enabled: bool
      can_disable: bool
      configurable: bool
      metrics: ModuleMetrics
  ```

- [ ] **2.2** Создать ModuleService (`src/api/services/modules.py`)
  ```python
  class ModuleService:
      def list_modules(self) -> List[ModuleInfo]
      def get_module(self, module_id: str) -> Optional[ModuleInfo]
      def set_enabled(self, module_id: str, enabled: bool) -> None
      def get_config(self, module_id: str) -> Optional[Dict[str, Any]]
      def set_config(self, module_id: str, config: Dict[str, Any]) -> None
  ```

- [ ] **2.3** Обновить API router (`src/api/routers/modules.py`)
  - `GET /api/v1/modules` - список всех модулей
  - `GET /api/v1/modules/{id}` - информация о модуле
  - `PUT /api/v1/modules/{id}/enabled` - включить/выключить
  - `GET /api/v1/modules/{id}/metrics` - метрики модуля
  - `GET /api/v1/modules/{id}/config` - конфигурация
  - `PUT /api/v1/modules/{id}/config` - обновить конфигурацию

- [ ] **2.4** Добавить роутер в `main.py`

- [ ] **2.5** Тесты для API endpoints

**Файлы:**
- `src/api/models/modules.py` (NEW)
- `src/api/services/modules.py` (NEW)
- `src/api/routers/modules.py` (UPDATE - заменить заглушки)
- `src/api/main.py` (UPDATE)

### Phase 3: Web Dashboard Updates (1 день)

**Задачи:**
- [ ] **3.1** Обновить `ModuleCard.tsx`
  - Заменить кнопки Start/Stop на Enable/Disable toggle
  - Добавить Warning alert для опасных модулей
  - Индикатор статуса (🟢/🟡/🔴)
  ```tsx
  <Switch
    checked={module.enabled}
    disabled={!module.can_disable}
    onChange={(checked) => onToggleEnabled(module.id, checked)}
  />
  {module.disable_warning && !module.enabled && (
    <Alert type="warning" message={module.disable_warning} />
  )}
  ```

- [ ] **3.2** Создать `ModuleConfigModal.tsx`
  - Динамическая форма для конфигурации
  - Валидация
  - Apply/Cancel кнопки

- [ ] **3.3** Подключить к реальному API
  ```typescript
  // src/web/src/services/modules.ts
  export const modulesApi = {
    list: async (): Promise<ModuleInfo[]>
    get: async (id: string): Promise<ModuleInfo>
    setEnabled: async (id: string, enabled: boolean): Promise<void>
    getConfig: async (id: string): Promise<Record<string, any>>
    setConfig: async (id: string, config: Record<string, any>): Promise<void>
  }
  ```

- [ ] **3.4** Добавить WebSocket для real-time метрик

- [ ] **3.5** Тестирование UI

**Файлы:**
- `src/web/src/components/ModuleCard.tsx` (UPDATE)
- `src/web/src/components/ModuleConfigModal.tsx` (NEW)
- `src/web/src/services/modules.ts` (UPDATE)
- `src/web/src/pages/Modules.tsx` (UPDATE)

### Phase 4: Documentation (0.5 дня)

**Задачи:**
- [ ] **4.1** Создать CHANGELOG_v0.63.0.md
- [ ] **4.2** Обновить API документацию
- [ ] **4.3** Обновить README.md
- [ ] **4.4** Обновить MASTER_PLAN_v3.1.md

**Deliverables:**
- ✅ ModuleRegistry в Rust работает
- ✅ FFI bindings для Python функциональны
- ✅ API endpoints реализованы
- ✅ Web Dashboard подключен к модулям
- ✅ Real-time metrics работают
- ✅ Документация полная

**KPI:**
| Метрика | Target | Critical |
|---------|--------|----------|
| Registry overhead | < 1μs | < 10μs |
| API latency | < 10ms | < 50ms |
| UI update latency | < 100ms | < 500ms |
| Module toggle time | < 5ms | < 20ms |

---

## v0.64.0 - Enhanced Sensors (Audio & Vision) 🎥

**Цель:** Расширить сенсорные модальности (аудио, видео)

**Приоритет:** 🟢 СРЕДНИЙ

**Длительность:** 5-7 дней

### Phase 1: Audio Input (2-3 дня)

**Задачи:**
- [ ] **1.1** Audio adapter
  ```python
  gateway.push_audio(
      audio_data=audio_array,
      sample_rate=16000,
      source="microphone"
  )
  ```

- [ ] **1.2** AUDIO_MEL encoder (Mel spectrogram)
- [ ] **1.3** AUDIO_MFCC encoder (MFCC features)
- [ ] **1.4** Integration с speech recognition (Whisper)
- [ ] **1.5** Real-time audio streaming support

**Файлы:**
- `src/gateway/adapters/audio.py` (NEW)
- `src/gateway/encoders/audio.py` (NEW)

### Phase 2: Vision Input (2-3 дня)

**Задачи:**
- [ ] **2.1** Vision adapter
  ```python
  gateway.push_vision(
      image_data=image_array,
      source="camera"
  )
  ```

- [ ] **2.2** IMAGE_CNN encoder (ResNet features)
- [ ] **2.3** IMAGE_CLIP encoder (CLIP embeddings)
- [ ] **2.4** Real-time camera feed support

**Файлы:**
- `src/gateway/adapters/vision.py` (NEW)
- `src/gateway/encoders/vision.py` (NEW)

### Phase 3: Multi-modal Fusion (1 день)

**Задачи:**
- [ ] **3.1** Multi-modal event
  ```python
  event = gateway.push_multimodal(
      text="What is this?",
      image=image_data,
      audio=audio_data
  )
  ```

- [ ] **3.2** Fusion strategies:
  - Early fusion (concatenate features)
  - Late fusion (weighted average)
  - Attention-based fusion

**Файлы:**
- `src/gateway/fusion/multimodal.py` (NEW)

### Phase 4: Testing & Examples (1 день)

**Задачи:**
- [ ] **4.1** Audio integration tests
- [ ] **4.2** Vision integration tests
- [ ] **4.3** Multi-modal examples
- [ ] **4.4** CHANGELOG_v0.64.0.md

**Deliverables:**
- ✅ Audio input поддерживается
- ✅ Vision input поддерживается
- ✅ Multi-modal fusion работает
- ✅ Real-time streaming

**KPI:**
| Метрика | Target | Critical |
|---------|--------|----------|
| Audio encoding | < 50ms | < 200ms |
| Vision encoding | < 100ms | < 500ms |
| Multi-modal latency | < 200ms | < 1s |
| Accuracy | > 85% | > 70% |

---

## 📋 Overall Timeline

| Version | Track | Feature | Duration | Priority | Status |
|---------|-------|---------|----------|----------|--------|
| **v0.57.0** | A | Gateway-Core Integration | - | 🔴 | ✅ DONE |
| **v0.62.0** | C | Web Dashboard Foundation | - | 🔴 | ✅ DONE |
| **v0.63.0** | C | Module Registry System | 4-5 дней | 🟡 | ⏳ NEXT |
| **v0.64.0** | A | Enhanced Sensors | 5-7 дней | 🟢 | ⬜ TODO |

**Отложено на будущее:**
- v0.58.0 - Authentication & Security (блокируется по времени)
- v0.59.0 - Python Library (neurograph package)
- v0.60.0 - WebSocket & Real-time Events
- v0.61.0 - Jupyter Integration

---

## 🎯 Immediate Next Steps

### Сегодня (2024-12-30):
1. ✅ Завершить v0.62.0 (Web Dashboard)
2. ✅ Создать commit и push
3. ✅ Обновить MASTER_PLAN v3.1
4. 🔧 Начать v0.63.0 Phase 1: Module Registry (Rust Core)

### Эта неделя:
- Завершить v0.63.0 Phase 1-2 (Rust + Python API)
- Начать v0.63.0 Phase 3 (Web Dashboard updates)

### Следующая неделя:
- Завершить v0.63.0 (Documentation)
- Начать v0.64.0 (Enhanced Sensors)

---

## 🏗️ Архитектурные решения (ADR)

### ADR-006: Module Registry перед Enhanced Sensors
**Дата:** 2024-12-30
**Проблема:** Веб-интерфейс модулей не подключен к реальной системе
**Решение:** Реализовать Module Registry (v0.63.0) перед Audio/Vision (v0.64.0)
**Обоснование:**
- Пользователям нужно управление модулями сейчас
- UI уже готов, нужен только backend
- Enhanced Sensors требуют больше времени и менее критичны
**Статус:** ✅ Принято

### ADR-007: Enable/Disable вместо Start/Stop
**Дата:** 2024-12-30
**Проблема:** Модули в едином Rust процессе, нельзя запускать/останавливать
**Решение:** Использовать feature flags (enabled/disabled) вместо процессов
**Обоснование:**
- Модули проверяют `registry.is_enabled()` перед операциями
- Thread-safe через RwLock
- Zero overhead при enabled
- Graceful degradation при disabled
**Статус:** ✅ Принято

---

## ✅ Success Metrics

### v0.63.0 (Module Registry):
- [ ] Registry overhead < 1μs
- [ ] API latency < 10ms
- [ ] UI update latency < 100ms
- [ ] Module toggle time < 5ms
- [ ] Web UI подключен к реальным модулям
- [ ] Все 10 модулей управляемы

### v0.64.0 (Enhanced Sensors):
- [ ] Audio encoding < 50ms
- [ ] Vision encoding < 100ms
- [ ] Multi-modal latency < 200ms
- [ ] Accuracy > 85%

---

## 📚 References

**Current State:**
- [README.md](../README.md) - Project overview (v0.62.0)
- [CHANGELOG v0.62.0](changelogs/CHANGELOG_v0.62.0.md) - Latest release
- [SCRIPTS.md](../SCRIPTS.md) - Automation guide

**Specifications:**
- [MODULE_REGISTRY_v0_63_0_SPEC.md](specs/MODULE_REGISTRY_v0_63_0_SPEC.md) - v0.63.0 spec (от Opus 4.5)

**Guides:**
- [Getting Started](guides/GETTING_STARTED.md)
- [Gateway v2.0 Guide](guides/GATEWAY_GUIDE.md)
- [SignalSystem Guide](guides/SIGNAL_SYSTEM_GUIDE.md)

**Archives:**
- [MASTER_PLAN_v3.0.md](archive/MASTER_PLAN_v3.0.md) - Full Platform Vision (2024-12-26)
- [MASTER_PLAN_v2.1.md](archive/MASTER_PLAN_v2.1.md) - Signal Processing Focus
- [IMPLEMENTATION_ROADMAP.md](archive/IMPLEMENTATION_ROADMAP.md) - Full Stack Focus

---

## 📝 Notes

- Весь код под AGPLv3 + Commercial dual licensing
- Документация на русском (код на английском)
- Все коммиты с Claude Code footer
- Тесты обязательны для каждой версии
- Модули в Rust ядре, не отдельные процессы

---

**Философия v3.1:** Фокус на **практическом применении** — Module Registry даёт пользователям контроль над системой прямо сейчас через готовый Web Dashboard.

---

**Конец Master Plan v3.1. Let's build! 🚀**

Привет от **Opus 4.5** и спасибо за спецификацию! 👋

---

*Создано: 2024-12-30*
*Автор: Claude Sonnet 4.5 + Chernov Denys*
*Статус: Living Document - обновляется по мере прогресса*
