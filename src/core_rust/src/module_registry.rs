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
