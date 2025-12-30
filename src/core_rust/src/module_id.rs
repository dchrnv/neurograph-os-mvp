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
