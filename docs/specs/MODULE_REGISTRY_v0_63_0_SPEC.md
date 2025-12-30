# Module Registry — Спецификация v0.63.0

**Версия:** 0.63.0  
**Дата:** 2024-12-30  
**Статус:** Спецификация для реализации  
**Зависимости:** v0.62.0 (Web Dashboard)  

---

## 1. Обзор

### 1.1 Проблема

Модули NeuroGraph — это **компоненты единого Rust ядра**, а не отдельные процессы. Их нельзя "запустить" или "остановить" в традиционном смысле. Однако пользователям нужен способ:

- Видеть какие модули есть в системе
- Включать/выключать функциональность модулей
- Настраивать параметры модулей
- Мониторить метрики каждого модуля

### 1.2 Решение: Feature Flags + Configuration

Вместо start/stop используем **enable/disable**:

```
Start/Stop (невозможно):        Enable/Disable (реализуемо):
┌─────────────┐                 ┌─────────────┐
│   Module    │                 │   Module    │
│  [Process]  │ ← kill/spawn    │  [in Rust]  │ ← enabled: bool
└─────────────┘                 └─────────────┘

Модуль остаётся в памяти,       Модуль проверяет флаг перед
но его функциональность         выполнением операций
отключена
```

### 1.3 Что даёт пользователю

| Действие | Результат |
|----------|-----------|
| Disable IntuitionEngine | Запросы идут напрямую, без интуитивной обработки |
| Disable SignalSystem | События не обрабатываются |
| Disable Gateway | Входные сигналы не принимаются |
| Disable ActionController | Выходные действия не выполняются |
| Configure CDNA scales | Изменение весов измерений на лету |

---

## 2. Архитектура

### 2.1 Общая схема

```
┌─────────────────────────────────────────────────────────────┐
│                      Web Dashboard                           │
│                    /modules page                             │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                        FastAPI                               │
│  GET  /api/v1/modules                                        │
│  GET  /api/v1/modules/{id}                                   │
│  PUT  /api/v1/modules/{id}/enabled                          │
│  GET  /api/v1/modules/{id}/metrics                          │
│  GET  /api/v1/modules/{id}/config                           │
│  PUT  /api/v1/modules/{id}/config                           │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                   Python Layer                               │
│  ModuleService (src/api/services/modules.py)                │
│    - Обёртка над Rust FFI                                   │
│    - Кеширование метрик                                     │
│    - Валидация конфигов                                     │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Rust Core (PyO3 FFI)                      │
│  ModuleRegistry (src/core_rust/src/module_registry.rs)      │
│    - Реестр всех модулей                                    │
│    - Feature flags (enabled/disabled)                       │
│    - Конфигурации модулей                                   │
│    - Сбор метрик                                            │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                    Rust Modules                              │
│  Каждый модуль проверяет registry.is_enabled(self.id)       │
│  перед выполнением операций                                 │
└─────────────────────────────────────────────────────────────┘
```

### 2.2 Файловая структура

```
src/
├── core_rust/src/
│   ├── module_registry.rs      # NEW: Реестр модулей
│   ├── module_id.rs            # NEW: Enum идентификаторов
│   ├── module_config.rs        # NEW: Конфигурации модулей
│   ├── python/
│   │   └── modules.rs          # NEW: FFI bindings для модулей
│   ├── intuition.rs            # UPDATE: добавить проверку enabled
│   ├── signal_system.rs        # UPDATE: добавить проверку enabled
│   ├── gateway.rs              # UPDATE: добавить проверку enabled
│   └── ...
│
├── api/
│   ├── routers/
│   │   └── modules.py          # UPDATE: реальная реализация
│   ├── services/
│   │   └── modules.py          # NEW: бизнес-логика
│   └── models/
│       └── modules.py          # NEW: Pydantic модели
│
└── web/src/
    └── pages/Modules/          # UPDATE: подключить к API
        ├── index.tsx
        └── ModuleCard.tsx
```

---

## 3. Список модулей

### 3.1 Модули для отображения

Показываем **10 ключевых логических модулей** (не все 29 внутренних компонентов):

| ID | Название | Описание | Можно отключить? |
|----|----------|----------|------------------|
| `token_manager` | TokenManager | Хранение и управление токенами | ❌ Нет (core) |
| `connection_manager` | ConnectionManager | Хранение связей между токенами | ❌ Нет (core) |
| `grid` | Grid | Пространственный индекс (8D) | ❌ Нет (core) |
| `intuition_engine` | IntuitionEngine | Интуитивная обработка запросов | ✅ Да |
| `signal_system` | SignalSystem | Обработка и маршрутизация сигналов | ✅ Да |
| `gateway` | Gateway | Входные сенсоры и энкодеры | ✅ Да |
| `action_controller` | ActionController | Выходные действия | ✅ Да |
| `guardian` | Guardian | Валидация и защита (CDNA) | ❌ Нет (критично!) |
| `cdna` | CDNA | Конституция системы | ❌ Нет (core) |
| `bootstrap` | Bootstrap | Загрузка embeddings | ❌ Нет (статус only) |

### 3.2 Категории модулей

```
Core (нельзя отключить):
├── TokenManager
├── ConnectionManager
├── Grid
├── CDNA
└── Guardian (критично для безопасности!)

Processing (можно отключить):
├── IntuitionEngine
└── SignalSystem

I/O (можно отключить):
├── Gateway
└── ActionController

Data (только статус):
└── Bootstrap
```

---

## 4. Rust Implementation

### 4.1 ModuleId Enum

**Файл:** `src/core_rust/src/module_id.rs`

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
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

impl ModuleId {
    /// Возвращает человекочитаемое название
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::TokenManager => "TokenManager",
            Self::ConnectionManager => "ConnectionManager",
            Self::Grid => "Grid",
            Self::IntuitionEngine => "IntuitionEngine",
            Self::SignalSystem => "SignalSystem",
            Self::Gateway => "Gateway",
            Self::ActionController => "ActionController",
            Self::Guardian => "Guardian",
            Self::Cdna => "CDNA",
            Self::Bootstrap => "Bootstrap",
        }
    }
    
    /// Возвращает описание модуля
    pub fn description(&self) -> &'static str {
        match self {
            Self::TokenManager => "Хранение и управление токенами",
            Self::ConnectionManager => "Хранение связей между токенами",
            Self::Grid => "Пространственный индекс в 8D пространстве",
            Self::IntuitionEngine => "Интуитивная обработка запросов",
            Self::SignalSystem => "Обработка и маршрутизация сигналов",
            Self::Gateway => "Входные сенсоры и энкодеры",
            Self::ActionController => "Выходные действия и ответы",
            Self::Guardian => "Валидация и защита системы",
            Self::Cdna => "Конституция и правила системы",
            Self::Bootstrap => "Загрузка word embeddings",
        }
    }
    
    /// Возвращает версию модуля
    pub fn version(&self) -> &'static str {
        match self {
            Self::TokenManager => "2.0.0",
            Self::ConnectionManager => "3.0.0",
            Self::Grid => "2.0.0",
            Self::IntuitionEngine => "3.0.0",
            Self::SignalSystem => "1.1.0",
            Self::Gateway => "2.0.0",
            Self::ActionController => "2.0.0",
            Self::Guardian => "1.0.0",
            Self::Cdna => "2.1.0",
            Self::Bootstrap => "1.3.0",
        }
    }
    
    /// Можно ли отключить этот модуль?
    pub fn can_disable(&self) -> bool {
        match self {
            Self::TokenManager => false,
            Self::ConnectionManager => false,
            Self::Grid => false,
            Self::IntuitionEngine => true,
            Self::SignalSystem => true,
            Self::Gateway => true,
            Self::ActionController => true,
            Self::Guardian => false,  // Критично для безопасности!
            Self::Cdna => false,
            Self::Bootstrap => false,
        }
    }
    
    /// Есть ли у модуля конфигурация?
    pub fn is_configurable(&self) -> bool {
        match self {
            Self::IntuitionEngine => true,
            Self::SignalSystem => true,
            Self::Gateway => true,
            Self::Guardian => true,
            Self::Cdna => true,
            _ => false,
        }
    }
    
    /// Требует ли предупреждения при отключении?
    pub fn disable_warning(&self) -> Option<&'static str> {
        match self {
            Self::SignalSystem => Some("Отключение SignalSystem остановит обработку всех событий"),
            Self::Gateway => Some("Отключение Gateway блокирует все входящие сигналы"),
            _ => None,
        }
    }
    
    /// Все модули
    pub fn all() -> &'static [ModuleId] {
        &[
            Self::TokenManager,
            Self::ConnectionManager,
            Self::Grid,
            Self::IntuitionEngine,
            Self::SignalSystem,
            Self::Gateway,
            Self::ActionController,
            Self::Guardian,
            Self::Cdna,
            Self::Bootstrap,
        ]
    }
}
```

### 4.2 ModuleRegistry

**Файл:** `src/core_rust/src/module_registry.rs`

```rust
use std::collections::HashMap;
use std::sync::RwLock;
use serde::{Deserialize, Serialize};

use crate::module_id::ModuleId;

/// Статус модуля
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModuleStatus {
    Active,     // Включен и работает
    Disabled,   // Выключен пользователем
    Error,      // Ошибка в модуле
}

/// Метрики модуля
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ModuleMetrics {
    /// Количество операций
    pub operations: u64,
    /// Операций в секунду
    pub ops_per_sec: f64,
    /// Средняя задержка (микросекунды)
    pub avg_latency_us: f64,
    /// P95 задержка (микросекунды)
    pub p95_latency_us: f64,
    /// Количество ошибок
    pub errors: u64,
    /// Дополнительные метрики (специфичные для модуля)
    pub custom: HashMap<String, f64>,
}

/// Конфигурация модуля (generic)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleConfig {
    pub values: HashMap<String, serde_json::Value>,
}

impl Default for ModuleConfig {
    fn default() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
}

/// Информация о модуле
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    pub id: ModuleId,
    pub name: String,
    pub description: String,
    pub version: String,
    pub status: ModuleStatus,
    pub enabled: bool,
    pub can_disable: bool,
    pub configurable: bool,
    pub disable_warning: Option<String>,
    pub metrics: ModuleMetrics,
}

/// Реестр модулей
pub struct ModuleRegistry {
    /// Флаги включения модулей
    enabled: RwLock<HashMap<ModuleId, bool>>,
    
    /// Конфигурации модулей
    configs: RwLock<HashMap<ModuleId, ModuleConfig>>,
    
    /// Метрики модулей (обновляются периодически)
    metrics: RwLock<HashMap<ModuleId, ModuleMetrics>>,
    
    /// Статусы модулей
    statuses: RwLock<HashMap<ModuleId, ModuleStatus>>,
}

impl ModuleRegistry {
    /// Создать новый реестр с дефолтными значениями
    pub fn new() -> Self {
        let mut enabled = HashMap::new();
        let mut statuses = HashMap::new();
        
        // Все модули включены по умолчанию
        for module in ModuleId::all() {
            enabled.insert(*module, true);
            statuses.insert(*module, ModuleStatus::Active);
        }
        
        Self {
            enabled: RwLock::new(enabled),
            configs: RwLock::new(HashMap::new()),
            metrics: RwLock::new(HashMap::new()),
            statuses: RwLock::new(statuses),
        }
    }
    
    /// Проверить, включен ли модуль
    pub fn is_enabled(&self, module: ModuleId) -> bool {
        self.enabled
            .read()
            .unwrap()
            .get(&module)
            .copied()
            .unwrap_or(true)
    }
    
    /// Включить/выключить модуль
    pub fn set_enabled(&self, module: ModuleId, enabled: bool) -> Result<(), String> {
        // Проверяем, можно ли отключить
        if !enabled && !module.can_disable() {
            return Err(format!(
                "Модуль {} нельзя отключить (core module)",
                module.display_name()
            ));
        }
        
        let mut guard = self.enabled.write().unwrap();
        guard.insert(module, enabled);
        
        // Обновляем статус
        let mut statuses = self.statuses.write().unwrap();
        statuses.insert(
            module,
            if enabled {
                ModuleStatus::Active
            } else {
                ModuleStatus::Disabled
            },
        );
        
        Ok(())
    }
    
    /// Получить информацию о модуле
    pub fn get_module_info(&self, module: ModuleId) -> ModuleInfo {
        let enabled = self.is_enabled(module);
        let status = self.statuses
            .read()
            .unwrap()
            .get(&module)
            .copied()
            .unwrap_or(ModuleStatus::Active);
        let metrics = self.metrics
            .read()
            .unwrap()
            .get(&module)
            .cloned()
            .unwrap_or_default();
        
        ModuleInfo {
            id: module,
            name: module.display_name().to_string(),
            description: module.description().to_string(),
            version: module.version().to_string(),
            status,
            enabled,
            can_disable: module.can_disable(),
            configurable: module.is_configurable(),
            disable_warning: module.disable_warning().map(|s| s.to_string()),
            metrics,
        }
    }
    
    /// Получить информацию о всех модулях
    pub fn get_all_modules(&self) -> Vec<ModuleInfo> {
        ModuleId::all()
            .iter()
            .map(|&id| self.get_module_info(id))
            .collect()
    }
    
    /// Обновить метрики модуля
    pub fn update_metrics(&self, module: ModuleId, metrics: ModuleMetrics) {
        let mut guard = self.metrics.write().unwrap();
        guard.insert(module, metrics);
    }
    
    /// Получить конфигурацию модуля
    pub fn get_config(&self, module: ModuleId) -> Option<ModuleConfig> {
        self.configs.read().unwrap().get(&module).cloned()
    }
    
    /// Обновить конфигурацию модуля
    pub fn set_config(&self, module: ModuleId, config: ModuleConfig) -> Result<(), String> {
        if !module.is_configurable() {
            return Err(format!(
                "Модуль {} не поддерживает конфигурацию",
                module.display_name()
            ));
        }
        
        let mut guard = self.configs.write().unwrap();
        guard.insert(module, config);
        Ok(())
    }
    
    /// Установить статус ошибки для модуля
    pub fn set_error(&self, module: ModuleId, _error: &str) {
        let mut guard = self.statuses.write().unwrap();
        guard.insert(module, ModuleStatus::Error);
    }
    
    /// Сбросить ошибку модуля
    pub fn clear_error(&self, module: ModuleId) {
        let enabled = self.is_enabled(module);
        let mut guard = self.statuses.write().unwrap();
        guard.insert(
            module,
            if enabled {
                ModuleStatus::Active
            } else {
                ModuleStatus::Disabled
            },
        );
    }
}

impl Default for ModuleRegistry {
    fn default() -> Self {
        Self::new()
    }
}

// Глобальный реестр (singleton)
lazy_static::lazy_static! {
    pub static ref REGISTRY: ModuleRegistry = ModuleRegistry::new();
}

/// Проверка enabled для использования в модулях
#[macro_export]
macro_rules! check_module_enabled {
    ($module:expr) => {
        if !$crate::module_registry::REGISTRY.is_enabled($module) {
            return None;  // или Ok(()) или другое значение по умолчанию
        }
    };
    ($module:expr, $default:expr) => {
        if !$crate::module_registry::REGISTRY.is_enabled($module) {
            return $default;
        }
    };
}
```

### 4.3 PyO3 FFI Bindings

**Файл:** `src/core_rust/src/python/modules.rs`

```rust
use pyo3::prelude::*;
use pyo3::types::PyDict;

use crate::module_id::ModuleId;
use crate::module_registry::{ModuleConfig, ModuleMetrics, REGISTRY};

/// Конвертация ModuleId из строки
fn parse_module_id(id: &str) -> PyResult<ModuleId> {
    match id {
        "token_manager" => Ok(ModuleId::TokenManager),
        "connection_manager" => Ok(ModuleId::ConnectionManager),
        "grid" => Ok(ModuleId::Grid),
        "intuition_engine" => Ok(ModuleId::IntuitionEngine),
        "signal_system" => Ok(ModuleId::SignalSystem),
        "gateway" => Ok(ModuleId::Gateway),
        "action_controller" => Ok(ModuleId::ActionController),
        "guardian" => Ok(ModuleId::Guardian),
        "cdna" => Ok(ModuleId::Cdna),
        "bootstrap" => Ok(ModuleId::Bootstrap),
        _ => Err(PyErr::new::<pyo3::exceptions::PyValueError, _>(
            format!("Unknown module id: {}", id)
        )),
    }
}

/// Получить список всех модулей
#[pyfunction]
pub fn list_modules(py: Python<'_>) -> PyResult<Vec<PyObject>> {
    let modules = REGISTRY.get_all_modules();
    
    modules
        .into_iter()
        .map(|info| {
            let dict = PyDict::new(py);
            dict.set_item("id", format!("{:?}", info.id).to_lowercase())?;
            dict.set_item("name", info.name)?;
            dict.set_item("description", info.description)?;
            dict.set_item("version", info.version)?;
            dict.set_item("status", format!("{:?}", info.status).to_lowercase())?;
            dict.set_item("enabled", info.enabled)?;
            dict.set_item("can_disable", info.can_disable)?;
            dict.set_item("configurable", info.configurable)?;
            dict.set_item("disable_warning", info.disable_warning)?;
            
            // Метрики
            let metrics = PyDict::new(py);
            metrics.set_item("operations", info.metrics.operations)?;
            metrics.set_item("ops_per_sec", info.metrics.ops_per_sec)?;
            metrics.set_item("avg_latency_us", info.metrics.avg_latency_us)?;
            metrics.set_item("p95_latency_us", info.metrics.p95_latency_us)?;
            metrics.set_item("errors", info.metrics.errors)?;
            dict.set_item("metrics", metrics)?;
            
            Ok(dict.into())
        })
        .collect()
}

/// Получить информацию о модуле
#[pyfunction]
pub fn get_module(py: Python<'_>, module_id: &str) -> PyResult<PyObject> {
    let id = parse_module_id(module_id)?;
    let info = REGISTRY.get_module_info(id);
    
    let dict = PyDict::new(py);
    dict.set_item("id", module_id)?;
    dict.set_item("name", info.name)?;
    dict.set_item("description", info.description)?;
    dict.set_item("version", info.version)?;
    dict.set_item("status", format!("{:?}", info.status).to_lowercase())?;
    dict.set_item("enabled", info.enabled)?;
    dict.set_item("can_disable", info.can_disable)?;
    dict.set_item("configurable", info.configurable)?;
    dict.set_item("disable_warning", info.disable_warning)?;
    
    // Метрики
    let metrics = PyDict::new(py);
    metrics.set_item("operations", info.metrics.operations)?;
    metrics.set_item("ops_per_sec", info.metrics.ops_per_sec)?;
    metrics.set_item("avg_latency_us", info.metrics.avg_latency_us)?;
    metrics.set_item("p95_latency_us", info.metrics.p95_latency_us)?;
    metrics.set_item("errors", info.metrics.errors)?;
    dict.set_item("metrics", metrics)?;
    
    Ok(dict.into())
}

/// Проверить, включен ли модуль
#[pyfunction]
pub fn is_module_enabled(module_id: &str) -> PyResult<bool> {
    let id = parse_module_id(module_id)?;
    Ok(REGISTRY.is_enabled(id))
}

/// Включить/выключить модуль
#[pyfunction]
pub fn set_module_enabled(module_id: &str, enabled: bool) -> PyResult<()> {
    let id = parse_module_id(module_id)?;
    REGISTRY
        .set_enabled(id, enabled)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))
}

/// Получить конфигурацию модуля
#[pyfunction]
pub fn get_module_config(py: Python<'_>, module_id: &str) -> PyResult<Option<PyObject>> {
    let id = parse_module_id(module_id)?;
    
    match REGISTRY.get_config(id) {
        Some(config) => {
            let dict = PyDict::new(py);
            for (key, value) in config.values {
                // Конвертируем serde_json::Value в Python объект
                let py_value = json_to_py(py, &value)?;
                dict.set_item(key, py_value)?;
            }
            Ok(Some(dict.into()))
        }
        None => Ok(None),
    }
}

/// Обновить конфигурацию модуля
#[pyfunction]
pub fn set_module_config(module_id: &str, config: &PyDict) -> PyResult<()> {
    let id = parse_module_id(module_id)?;
    
    let mut values = std::collections::HashMap::new();
    for (key, value) in config.iter() {
        let key_str: String = key.extract()?;
        let json_value = py_to_json(value)?;
        values.insert(key_str, json_value);
    }
    
    let module_config = ModuleConfig { values };
    
    REGISTRY
        .set_config(id, module_config)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e))
}

// Вспомогательные функции для конвертации JSON <-> Python
fn json_to_py(py: Python<'_>, value: &serde_json::Value) -> PyResult<PyObject> {
    match value {
        serde_json::Value::Null => Ok(py.None()),
        serde_json::Value::Bool(b) => Ok(b.into_py(py)),
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(i.into_py(py))
            } else if let Some(f) = n.as_f64() {
                Ok(f.into_py(py))
            } else {
                Ok(py.None())
            }
        }
        serde_json::Value::String(s) => Ok(s.into_py(py)),
        serde_json::Value::Array(arr) => {
            let list: Vec<PyObject> = arr
                .iter()
                .map(|v| json_to_py(py, v))
                .collect::<PyResult<_>>()?;
            Ok(list.into_py(py))
        }
        serde_json::Value::Object(obj) => {
            let dict = PyDict::new(py);
            for (k, v) in obj {
                dict.set_item(k, json_to_py(py, v)?)?;
            }
            Ok(dict.into())
        }
    }
}

fn py_to_json(obj: &PyAny) -> PyResult<serde_json::Value> {
    if obj.is_none() {
        Ok(serde_json::Value::Null)
    } else if let Ok(b) = obj.extract::<bool>() {
        Ok(serde_json::Value::Bool(b))
    } else if let Ok(i) = obj.extract::<i64>() {
        Ok(serde_json::Value::Number(i.into()))
    } else if let Ok(f) = obj.extract::<f64>() {
        Ok(serde_json::json!(f))
    } else if let Ok(s) = obj.extract::<String>() {
        Ok(serde_json::Value::String(s))
    } else if let Ok(list) = obj.extract::<Vec<&PyAny>>() {
        let arr: Result<Vec<_>, _> = list.iter().map(|v| py_to_json(v)).collect();
        Ok(serde_json::Value::Array(arr?))
    } else if let Ok(dict) = obj.downcast::<PyDict>() {
        let mut map = serde_json::Map::new();
        for (k, v) in dict.iter() {
            let key: String = k.extract()?;
            map.insert(key, py_to_json(v)?);
        }
        Ok(serde_json::Value::Object(map))
    } else {
        Err(PyErr::new::<pyo3::exceptions::PyTypeError, _>(
            "Unsupported type for JSON conversion"
        ))
    }
}

/// Регистрация модуля в PyO3
pub fn register_module(py: Python<'_>, parent: &PyModule) -> PyResult<()> {
    let module = PyModule::new(py, "modules")?;
    module.add_function(wrap_pyfunction!(list_modules, module)?)?;
    module.add_function(wrap_pyfunction!(get_module, module)?)?;
    module.add_function(wrap_pyfunction!(is_module_enabled, module)?)?;
    module.add_function(wrap_pyfunction!(set_module_enabled, module)?)?;
    module.add_function(wrap_pyfunction!(get_module_config, module)?)?;
    module.add_function(wrap_pyfunction!(set_module_config, module)?)?;
    parent.add_submodule(module)?;
    Ok(())
}
```

### 4.4 Интеграция в существующие модули

Пример интеграции в IntuitionEngine:

**Файл:** `src/core_rust/src/intuition.rs` (изменения)

```rust
use crate::module_id::ModuleId;
use crate::module_registry::REGISTRY;

impl IntuitionEngine {
    pub fn process(&self, input: &Input) -> Option<Output> {
        // Проверка: модуль включен?
        if !REGISTRY.is_enabled(ModuleId::IntuitionEngine) {
            // Модуль выключен — пропускаем обработку
            return None;
        }

        // Обычная логика обработки
        // ...
    }
}
```

Аналогично для других модулей:
- `SignalSystem::emit()` — проверка перед обработкой
- `Gateway::push()` — проверка перед приёмом сигнала
- `ActionController::execute()` — проверка перед выполнением
- ~~`Guardian::validate()`~~ — **НЕ проверяем**, Guardian всегда активен!

---

## 5. Python API Layer

### 5.1 Pydantic Models

**Файл:** `src/api/models/modules.py`

```python
from enum import Enum
from typing import Optional, Dict, Any
from pydantic import BaseModel, Field


class ModuleStatus(str, Enum):
    ACTIVE = "active"
    DISABLED = "disabled"
    ERROR = "error"


class ModuleMetrics(BaseModel):
    """Метрики модуля"""
    operations: int = Field(default=0, description="Количество операций")
    ops_per_sec: float = Field(default=0.0, description="Операций в секунду")
    avg_latency_us: float = Field(default=0.0, description="Средняя задержка (мкс)")
    p95_latency_us: float = Field(default=0.0, description="P95 задержка (мкс)")
    errors: int = Field(default=0, description="Количество ошибок")
    custom: Dict[str, float] = Field(default_factory=dict, description="Дополнительные метрики")


class ModuleInfo(BaseModel):
    """Информация о модуле"""
    id: str = Field(..., description="Идентификатор модуля")
    name: str = Field(..., description="Название модуля")
    description: str = Field(..., description="Описание модуля")
    version: str = Field(..., description="Версия модуля")
    status: ModuleStatus = Field(..., description="Текущий статус")
    enabled: bool = Field(..., description="Включен ли модуль")
    can_disable: bool = Field(..., description="Можно ли отключить")
    configurable: bool = Field(..., description="Есть ли конфигурация")
    disable_warning: Optional[str] = Field(None, description="Предупреждение при отключении")
    metrics: ModuleMetrics = Field(default_factory=ModuleMetrics)

    class Config:
        json_schema_extra = {
            "example": {
                "id": "intuition_engine",
                "name": "IntuitionEngine",
                "description": "Интуитивная обработка запросов",
                "version": "3.0.0",
                "status": "active",
                "enabled": True,
                "can_disable": True,
                "configurable": True,
                "disable_warning": None,
                "metrics": {
                    "operations": 12847,
                    "ops_per_sec": 1284.7,
                    "avg_latency_us": 69.5,
                    "p95_latency_us": 120.0,
                    "errors": 0
                }
            }
        }


class ModuleConfig(BaseModel):
    """Конфигурация модуля"""
    values: Dict[str, Any] = Field(default_factory=dict)


class SetEnabledRequest(BaseModel):
    """Запрос на включение/выключение модуля"""
    enabled: bool = Field(..., description="Включить (true) или выключить (false)")


class SetConfigRequest(BaseModel):
    """Запрос на обновление конфигурации"""
    config: Dict[str, Any] = Field(..., description="Новая конфигурация")


class ModuleListResponse(BaseModel):
    """Ответ со списком модулей"""
    modules: list[ModuleInfo]
    total: int


class ModuleResponse(BaseModel):
    """Ответ с информацией о модуле"""
    module: ModuleInfo


class SuccessResponse(BaseModel):
    """Успешный ответ"""
    success: bool = True
    message: str = ""
```

### 5.2 Module Service

**Файл:** `src/api/services/modules.py`

```python
from typing import Optional, Dict, Any, List
from neurograph import _core  # PyO3 bindings

from ..models.modules import (
    ModuleInfo,
    ModuleMetrics,
    ModuleStatus,
    ModuleConfig,
)


class ModuleService:
    """Сервис для работы с модулями"""
    
    def __init__(self):
        pass
    
    def list_modules(self) -> List[ModuleInfo]:
        """Получить список всех модулей"""
        raw_modules = _core.modules.list_modules()
        return [self._convert_module_info(m) for m in raw_modules]
    
    def get_module(self, module_id: str) -> Optional[ModuleInfo]:
        """Получить информацию о модуле"""
        try:
            raw = _core.modules.get_module(module_id)
            return self._convert_module_info(raw)
        except ValueError:
            return None
    
    def is_enabled(self, module_id: str) -> bool:
        """Проверить, включен ли модуль"""
        return _core.modules.is_module_enabled(module_id)
    
    def set_enabled(self, module_id: str, enabled: bool) -> None:
        """Включить/выключить модуль"""
        _core.modules.set_module_enabled(module_id, enabled)
    
    def get_config(self, module_id: str) -> Optional[Dict[str, Any]]:
        """Получить конфигурацию модуля"""
        return _core.modules.get_module_config(module_id)
    
    def set_config(self, module_id: str, config: Dict[str, Any]) -> None:
        """Обновить конфигурацию модуля"""
        _core.modules.set_module_config(module_id, config)
    
    def _convert_module_info(self, raw: dict) -> ModuleInfo:
        """Конвертация из dict в Pydantic модель"""
        metrics_raw = raw.get("metrics", {})
        metrics = ModuleMetrics(
            operations=metrics_raw.get("operations", 0),
            ops_per_sec=metrics_raw.get("ops_per_sec", 0.0),
            avg_latency_us=metrics_raw.get("avg_latency_us", 0.0),
            p95_latency_us=metrics_raw.get("p95_latency_us", 0.0),
            errors=metrics_raw.get("errors", 0),
            custom=metrics_raw.get("custom", {}),
        )
        
        return ModuleInfo(
            id=raw["id"],
            name=raw["name"],
            description=raw["description"],
            version=raw["version"],
            status=ModuleStatus(raw["status"]),
            enabled=raw["enabled"],
            can_disable=raw["can_disable"],
            configurable=raw["configurable"],
            disable_warning=raw.get("disable_warning"),
            metrics=metrics,
        )


# Singleton instance
module_service = ModuleService()
```

### 5.3 API Router

**Файл:** `src/api/routers/modules.py`

```python
from typing import Optional
from fastapi import APIRouter, HTTPException, status

from ..models.modules import (
    ModuleInfo,
    ModuleListResponse,
    ModuleResponse,
    SetEnabledRequest,
    SetConfigRequest,
    SuccessResponse,
)
from ..services.modules import module_service


router = APIRouter(prefix="/modules", tags=["modules"])


@router.get(
    "",
    response_model=ModuleListResponse,
    summary="Список модулей",
    description="Получить список всех модулей системы с их статусами и метриками",
)
async def list_modules():
    """Получить список всех модулей"""
    modules = module_service.list_modules()
    return ModuleListResponse(modules=modules, total=len(modules))


@router.get(
    "/{module_id}",
    response_model=ModuleResponse,
    summary="Информация о модуле",
    description="Получить детальную информацию о конкретном модуле",
)
async def get_module(module_id: str):
    """Получить информацию о модуле"""
    module = module_service.get_module(module_id)
    if module is None:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail=f"Модуль '{module_id}' не найден",
        )
    return ModuleResponse(module=module)


@router.put(
    "/{module_id}/enabled",
    response_model=SuccessResponse,
    summary="Включить/выключить модуль",
    description="Включить или выключить функциональность модуля",
)
async def set_module_enabled(module_id: str, request: SetEnabledRequest):
    """Включить/выключить модуль"""
    module = module_service.get_module(module_id)
    if module is None:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail=f"Модуль '{module_id}' не найден",
        )
    
    if not request.enabled and not module.can_disable:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=f"Модуль '{module_id}' нельзя отключить (core module)",
        )
    
    try:
        module_service.set_enabled(module_id, request.enabled)
        action = "включен" if request.enabled else "выключен"
        return SuccessResponse(
            success=True,
            message=f"Модуль '{module.name}' {action}",
        )
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e),
        )


@router.get(
    "/{module_id}/metrics",
    summary="Метрики модуля",
    description="Получить текущие метрики модуля",
)
async def get_module_metrics(module_id: str):
    """Получить метрики модуля"""
    module = module_service.get_module(module_id)
    if module is None:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail=f"Модуль '{module_id}' не найден",
        )
    return {"metrics": module.metrics}


@router.get(
    "/{module_id}/config",
    summary="Конфигурация модуля",
    description="Получить текущую конфигурацию модуля",
)
async def get_module_config(module_id: str):
    """Получить конфигурацию модуля"""
    module = module_service.get_module(module_id)
    if module is None:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail=f"Модуль '{module_id}' не найден",
        )
    
    if not module.configurable:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=f"Модуль '{module_id}' не поддерживает конфигурацию",
        )
    
    config = module_service.get_config(module_id)
    return {"config": config or {}}


@router.put(
    "/{module_id}/config",
    response_model=SuccessResponse,
    summary="Обновить конфигурацию",
    description="Обновить конфигурацию модуля",
)
async def set_module_config(module_id: str, request: SetConfigRequest):
    """Обновить конфигурацию модуля"""
    module = module_service.get_module(module_id)
    if module is None:
        raise HTTPException(
            status_code=status.HTTP_404_NOT_FOUND,
            detail=f"Модуль '{module_id}' не найден",
        )
    
    if not module.configurable:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=f"Модуль '{module_id}' не поддерживает конфигурацию",
        )
    
    try:
        module_service.set_config(module_id, request.config)
        return SuccessResponse(
            success=True,
            message=f"Конфигурация модуля '{module.name}' обновлена",
        )
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=str(e),
        )
```

---

## 6. Web Dashboard Updates

### 6.1 Изменения в ModuleCard

Заменить кнопки Start/Stop на Enable/Disable toggle:

**Было:**
```tsx
<Button onClick={onStart}>Запустить</Button>
<Button onClick={onStop}>Остановить</Button>
```

**Стало:**
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

### 6.2 API Integration

Подключить реальные API вызовы вместо моков:

```typescript
// src/web/src/services/modules.ts

const API_BASE = '/api/v1';

export const modulesApi = {
  list: async (): Promise<ModuleInfo[]> => {
    const response = await fetch(`${API_BASE}/modules`);
    const data = await response.json();
    return data.modules;
  },
  
  get: async (id: string): Promise<ModuleInfo> => {
    const response = await fetch(`${API_BASE}/modules/${id}`);
    const data = await response.json();
    return data.module;
  },
  
  setEnabled: async (id: string, enabled: boolean): Promise<void> => {
    await fetch(`${API_BASE}/modules/${id}/enabled`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ enabled }),
    });
  },
  
  getConfig: async (id: string): Promise<Record<string, any>> => {
    const response = await fetch(`${API_BASE}/modules/${id}/config`);
    const data = await response.json();
    return data.config;
  },
  
  setConfig: async (id: string, config: Record<string, any>): Promise<void> => {
    await fetch(`${API_BASE}/modules/${id}/config`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ config }),
    });
  },
};
```

### 6.3 UI Компоненты

**ModuleCard обновления:**

1. Toggle switch для enabled/disabled
2. Индикатор статуса (🟢/🟡/🔴)
3. Warning alert при отключении опасных модулей
4. Кнопка "Configure" для модулей с конфигурацией
5. Метрики в реальном времени

**ModuleConfigModal:**

Модальное окно для редактирования конфигурации модуля:
- Динамическая форма на основе schema модуля
- Валидация
- Apply/Cancel кнопки

---

## 7. План реализации

### Phase 1: Rust Core (2 дня)

- [ ] Создать `module_id.rs` с enum ModuleId
- [ ] Создать `module_registry.rs` с реестром
- [ ] Создать `python/modules.rs` с FFI bindings
- [ ] Добавить в `lib.rs` экспорт модулей
- [ ] Интегрировать проверку `is_enabled()` в:
  - [ ] IntuitionEngine
  - [ ] SignalSystem
  - [ ] Gateway
  - [ ] ActionController
- [ ] Тесты для ModuleRegistry

### Phase 2: Python Layer (1 день)

- [ ] Создать `models/modules.py`
- [ ] Создать `services/modules.py`
- [ ] Обновить `routers/modules.py`
- [ ] Добавить роутер в `main.py`
- [ ] Тесты для API endpoints

### Phase 3: Web Dashboard (1 день)

- [ ] Обновить `ModuleCard.tsx` (toggle вместо buttons)
- [ ] Создать `ModuleConfigModal.tsx`
- [ ] Подключить к реальному API
- [ ] Добавить WebSocket для real-time метрик
- [ ] Тестирование UI

### Phase 4: Documentation (0.5 дня)

- [ ] CHANGELOG_v0.63.0.md
- [ ] Обновить API документацию
- [ ] README обновления

---

## 8. API Reference

### Endpoints

| Method | Path | Description |
|--------|------|-------------|
| GET | `/api/v1/modules` | Список всех модулей |
| GET | `/api/v1/modules/{id}` | Информация о модуле |
| PUT | `/api/v1/modules/{id}/enabled` | Включить/выключить модуль |
| GET | `/api/v1/modules/{id}/metrics` | Метрики модуля |
| GET | `/api/v1/modules/{id}/config` | Конфигурация модуля |
| PUT | `/api/v1/modules/{id}/config` | Обновить конфигурацию |

### Module IDs

- `token_manager`
- `connection_manager`
- `grid`
- `intuition_engine`
- `signal_system`
- `gateway`
- `action_controller`
- `guardian`
- `cdna`
- `bootstrap`

---

## 9. Важные замечания

### Для разработчика

1. **Singleton Registry** — используем `lazy_static!` для глобального реестра
2. **Thread Safety** — все поля под `RwLock`
3. **Graceful Degradation** — если модуль выключен, операции просто пропускаются
4. **No Panics** — все ошибки возвращаются как Result

### Предупреждения

1. **SignalSystem** — отключение останавливает обработку событий
2. **Gateway** — отключение блокирует все входящие сигналы
3. **Core modules** — TokenManager, ConnectionManager, Grid, CDNA, Guardian нельзя отключить

### Будущие улучшения

- Bootstrap Library (управление embeddings)
- Динамические модули (плагины)
- Module dependencies (граф зависимостей)
- Hot reload конфигурации

---

**Конец спецификации.**

*Готово к реализации. Удачи, Sonnet!* 🚀
