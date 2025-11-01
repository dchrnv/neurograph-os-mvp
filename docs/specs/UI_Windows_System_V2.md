# 🪟 NeuroGraph OS — Window System Specification

**Version:** v2.0 (Hielo+)
**Status:** In Development
**Framework:** iced (Rust)
**Architecture:** 
## 📐 Архитектура оконной системы

### Базовая структура окна

```rust
struct Window {
    id: WindowId,
    window_type: WindowType,
    title: String,
    position: Position,
    size: Size,
    state: WindowState,
    flags: WindowFlags,
    content: Box<dyn WindowContent>,
}

enum WindowType {
    Modal,        // Блокирует взаимодействие с другими окнами
    Dialog,       // Обычный диалог
    Floating,     // Плавающее окно
    Docked,       // Прикрепленное к краю
    Fullscreen,   // Полноэкранное
}

enum WindowState {
    Normal,
    Minimized,
    Maximized,
    Hidden,
    Closing,      // Анимация закрытия
}

struct WindowFlags {
    resizable: bool,
    movable: bool,
    closable: bool,
    always_on_top: bool,
    transparent: bool,
    requires_auth: bool,  // Для критичных настроек
}
```

### Универсальная структура окна

```
┌─────────────────────────────────────────┐
│ [icon] Заголовок окна    [_] [□] [x]   │ <- Заголовок
├─────────────────────────────────────────┤
│ Категория 1 | Категория 2 | Категория 3│ <- Табы (опционально)
├─────────────────────────────────────────┤
│                                         │
│         Основная область контента       │
│                                         │
├─────────────────────────────────────────┤
│ [Отмена]              [Применить] [OK]  │ <- Панель действий
└─────────────────────────────────────────┘
```

---

## ⚙️ Окно настроек (Settings)

### Структура настроек

```
┌──────────────────────────────────────────────────┐
│ ⚙ Настройки                          [x]        │
├────────────┬─────────────────────────────────────┤
│            │                                      │
│ Общие      │  Общие настройки                    │
│ ---------- │  ─────────────────                   │
│            │                                      │
│ ▶ Интерфейс│  Язык:          [Русский     ▼]    │
│            │  Тема:          [Темная      ▼]    │
│ ▶ Произв.  │  Режим:         ○ User ● Root       │
│            │                                      │
│ ▶ Сеть     │  □ Показывать подсказки            │
│            │  □ Звуковые уведомления            │
│ ▶ Безопасн.│  □ Автосохранение каждые [5] мин   │
│            │                                      │
│ ▶ Модули   │  Масштаб интерфейса:               │
│            │  [─────────●─────] 100%             │
│ ▶ Горячие  │                                      │
│   клавиши  │  Частота обновления метрик:        │
│            │  User: [500] мс  Root: [100] мс    │
│ ▶ О системе│                                      │
│            │                                      │
├────────────┴─────────────────────────────────────┤
│ [Сброс]  [Импорт]  [Экспорт]    [Отмена] [OK]   │
└──────────────────────────────────────────────────┘
```

### Категории настроек

```rust
enum SettingsCategory {
    General {
        language: Language,
        theme: Theme,
        user_mode: UserMode,
        auto_save: bool,
        auto_save_interval: Duration,
    },
    
    Interface {
        scale: f32,           // 0.5 - 2.0
        font_size: u8,        // 10 - 20
        animations: bool,
        tooltips: bool,
        compact_mode: bool,
    },
    
    Performance {
        update_rate_user: u32,     // ms
        update_rate_root: u32,     // ms
        cache_size: usize,         // MB
        max_history: usize,        // events
        gpu_acceleration: bool,
    },
    
    Network {
        proxy: Option<ProxyConfig>,
        timeout: Duration,
        max_connections: u32,
        bandwidth_limit: Option<u32>,
    },
    
    Security {
        require_auth_for_root: bool,
        session_timeout: Duration,
        audit_logging: bool,
        encryption_level: EncryptionLevel,
    },
    
    Modules {
        enabled_modules: Vec<ModuleId>,
        module_configs: HashMap<ModuleId, ModuleConfig>,
        auto_start: Vec<ModuleId>,
    },
    
    Hotkeys {
        global: Vec<HotkeyBinding>,
        user_defined: Vec<CustomHotkey>,
        conflicts: Vec<ConflictInfo>,
    }
}
```

### Визуальные элементы настроек

```css
/* Левая панель навигации */
.settings-nav {
    width: 180px;
    background: #0a0a0a;
    border-right: 1px solid #252525;
}

.settings-nav-item {
    padding: 12px 16px;
    cursor: pointer;
    transition: all 0.2s;
}

.settings-nav-item:hover {
    background: #141414;
    border-left: 2px solid #00ffcc;
}

.settings-nav-item.active {
    background: #1a1a1a;
    border-left: 3px solid #00ffcc;
    color: #00ffcc;
}

/* Контролы настроек */
.setting-control {
    margin: 16px 0;
    display: grid;
    grid-template-columns: 200px 1fr;
    align-items: center;
    gap: 16px;
}

.setting-label {
    color: #a0a0a0;
    font-size: 14px;
}

.setting-input {
    background: #141414;
    border: 1px solid #252525;
    padding: 6px 12px;
    border-radius: 4px;
}

.setting-slider {
    appearance: none;
    height: 4px;
    background: #252525;
    outline: none;
}

.setting-slider::-webkit-slider-thumb {
    appearance: none;
    width: 16px;
    height: 16px;
    background: #00ffcc;
    border-radius: 50%;
    cursor: pointer;
}
```

---

## 🧩 Окно управления модулями

### Менеджер модулей

```
┌──────────────────────────────────────────────────┐
│ 🧩 Менеджер модулей                    [x]      │
├──────────────────────────────────────────────────┤
│ [Все] [Активные] [Остановленные] [Ошибки]  🔍[_]│
├──────────────────────────────────────────────────┤
│                                                   │
│ ┌────────────────────────────────────────────┐  │
│ │ ▶ Token Manager                    [RUNNING]│  │
│ │   PID: 1247 | CPU: 2.3% | RAM: 124MB       │  │
│ │   Uptime: 2h 34m | Events: 45,823          │  │
│ │   [Pause] [Restart] [Config] [Logs]         │  │
│ └────────────────────────────────────────────┘  │
│                                                   │
│ ┌────────────────────────────────────────────┐  │
│ │ ▶ Connection Pool                  [RUNNING]│  │
│ │   PID: 1248 | CPU: 0.8% | RAM: 89MB        │  │
│ │   Active: 523 | Queue: 12 | Errors: 0      │  │
│ │   [Pause] [Restart] [Config] [Logs]         │  │
│ └────────────────────────────────────────────┘  │
│                                                   │
│ ┌────────────────────────────────────────────┐  │
│ │ ▼ Graph Engine                      [PAUSED]│  │
│ │   Status: Paused by user                    │  │
│ │   Last active: 15 minutes ago               │  │
│ │   [Resume] [Config] [Clear Cache]           │  │
│ └────────────────────────────────────────────┘  │
│                                                   │
│ ┌────────────────────────────────────────────┐  │
│ │ ▶ Guardian                          [ERROR] │  │
│ │   Error: Connection timeout                 │  │
│ │   Retry in: 23s | Attempts: 3/5            │  │
│ │   [Restart] [Debug] [View Error] [Disable] │  │
│ └────────────────────────────────────────────┘  │
│                                                   │
├──────────────────────────────────────────────────┤
│ [Start All] [Stop All] [Refresh]     [Close]    │
└──────────────────────────────────────────────────┘
```

### Структура модуля

```rust
struct ModuleView {
    id: ModuleId,
    name: String,
    status: ModuleStatus,
    metrics: ModuleMetrics,
    controls: Vec<ModuleControl>,
    expanded: bool,
}

struct ModuleMetrics {
    pid: Option<u32>,
    cpu_usage: f32,
    memory_usage: usize,
    uptime: Duration,
    event_count: u64,
    error_count: u32,
    custom_metrics: HashMap<String, MetricValue>,
}

enum ModuleControl {
    Start,
    Stop,
    Pause,
    Resume,
    Restart,
    Configure,
    ViewLogs,
    Debug,
    ClearCache,
}

enum ModuleStatus {
    Running { health: HealthScore },
    Paused { reason: String },
    Stopped,
    Starting { progress: f32 },
    Stopping,
    Error { message: String, retry_in: Option<Duration> },
    Crashed { dump_available: bool },
}
```

---

## 🔧 Окно конфигурации модуля

### Детальная конфигурация

```
┌──────────────────────────────────────────────────┐
│ ⚙ Конфигурация: Token Manager          [x]      │
├──────────────────────────────────────────────────┤
│ Основные | Производительность | Логирование | API│
├──────────────────────────────────────────────────┤
│                                                   │
│ Основные параметры                               │
│ ──────────────────                               │
│                                                   │
│ Имя модуля:        [Token Manager_______]        │
│ Автозапуск:        ☑ При старте системы          │
│ Приоритет:         [Normal         ▼]            │
│                                                   │
│ Лимиты ресурсов                                  │
│ ───────────────                                  │
│                                                   │
│ Max CPU:           [────────●────] 80%           │
│ Max Memory:        [256] MB                      │
│ Max Threads:       [16]                          │
│                                                   │
│ Специфичные настройки                            │
│ ─────────────────────                           │
│                                                   │
│ Token TTL:         [3600] секунд                 │
│ Max Tokens:        [10000]                       │
│ Cleanup Interval:  [300] секунд                  │
│                                                   │
│ □ Включить сжатие токенов                        │
│ □ Кэшировать частые запросы                      │
│ ☑ Валидировать при создании                      │
│                                                   │
│ Пространства координат:                          │
│ ☑ L1 Physical  ☑ L5 Cognitive                   │
│ ☑ L2 Sensory   ☑ L6 Social                      │
│ ☑ L3 Motor     ☑ L7 Temporal                    │
│ ☑ L4 Emotional ☑ L8 Abstract                    │
│                                                   │
├──────────────────────────────────────────────────┤
│ [Сброс]  [Загрузить]  [Сохранить как...]         │
│                        [Отмена] [Применить]      │
└──────────────────────────────────────────────────┘
```

### Система конфигурации

```rust
struct ModuleConfig {
    // Базовые параметры
    base: BaseConfig {
        name: String,
        auto_start: bool,
        priority: Priority,
        dependencies: Vec<ModuleId>,
    },
    
    // Лимиты ресурсов
    resources: ResourceLimits {
        max_cpu_percent: f32,
        max_memory_mb: usize,
        max_threads: u32,
        max_file_handles: u32,
    },
    
    // Специфичные для модуля
    specific: serde_json::Value,  // JSON для гибкости
    
    // Версионирование конфигурации
    version: Version,
    last_modified: Timestamp,
    modified_by: UserId,
}

impl ConfigValidator {
    fn validate(&self, config: &ModuleConfig) -> Result<(), ValidationError> {
        // Проверка базовых параметров
        self.validate_base(&config.base)?;
        
        // Проверка лимитов
        self.validate_resources(&config.resources)?;
        
        // Модуль-специфичная валидация
        self.validate_specific(&config.specific)?;
        
        Ok(())
    }
}
```

---

## 📊 Окно мониторинга модуля

### Детальный мониторинг

```
┌──────────────────────────────────────────────────┐
│ 📊 Мониторинг: Connection Pool          [x]      │
├──────────────────────────────────────────────────┤
│ График | Метрики | События | Профайлер           │
├──────────────────────────────────────────────────┤
│                                                   │
│ Активные соединения (последние 5 минут)          │
│ 600 ┤                                            │
│ 500 ┤           ╭─╮                              │
│ 400 ┤       ╭──╯  ╰─╮                           │
│ 300 ┤    ╭──╯       ╰──╮                        │
│ 200 ┤ ───╯             ╰───                     │
│ 100 ┤                                            │
│   0 └────────────────────────────────────        │
│     -5m    -4m    -3m    -2m    -1m    now      │
│                                                   │
│ ┌────────────────────────────────────────────┐  │
│ │ Текущие метрики             | Среднее за 1ч│  │
│ ├────────────────────────────────────────────┤  │
│ │ Активные соединения:    523 | 487          │  │
│ │ В очереди:               12 | 8            │  │
│ │ Обработано/сек:        1247 | 1189         │  │
│ │ Средняя задержка:     12ms | 15ms          │  │
│ │ Ошибки/мин:              0 | 0.3           │  │
│ │ CPU:                   0.8% | 1.2%          │  │
│ │ RAM:                  89MB | 92MB           │  │
│ └────────────────────────────────────────────┘  │
│                                                   │
│ Горячие пути:                                    │
│ [████████████████░░] /api/token/create    (42%) │
│ [██████████░░░░░░░░] /api/graph/query     (28%) │
│ [██████░░░░░░░░░░░░] /api/connection/new  (15%) │
│                                                   │
├──────────────────────────────────────────────────┤
│ [Экспорт CSV] [Сброс счетчиков]     [Закрыть]   │
└──────────────────────────────────────────────────┘
```

---

## 🎯 Окно быстрых действий (Command Palette)

### Универсальный поиск и команды

```
┌──────────────────────────────────────────────────┐
│ 🔍 Быстрые действия                              │
├──────────────────────────────────────────────────┤
│                                                   │
│ > restart token                                  │
│                                                   │
│ Предложения:                                     │
│ ┌────────────────────────────────────────────┐  │
│ │ [⚡] restart token manager      Ctrl+R,T    │  │
│ │ [⚡] restart all tokens                     │  │
│ │ [⚙] token configuration                    │  │
│ │ [📊] show token metrics                    │  │
│ └────────────────────────────────────────────┘  │
│                                                   │
│ Последние команды:                               │
│ • monitor cpu --interval 1s                      │
│ • graph stats                                    │
│ • status --all                                   │
│                                                   │
└──────────────────────────────────────────────────┘
```

```rust
struct CommandPalette {
    input: String,
    suggestions: Vec<CommandSuggestion>,
    history: Vec<HistoryItem>,
    fuzzy_matcher: FuzzyMatcher,
}

struct CommandSuggestion {
    icon: &'static str,
    label: String,
    command: String,
    hotkey: Option<String>,
    score: f32,  // Релевантность
}

impl CommandPalette {
    fn search(&mut self, query: &str) -> Vec<CommandSuggestion> {
        let mut results = vec![];
        
        // Поиск по командам
        results.extend(self.search_commands(query));
        
        // Поиск по настройкам
        results.extend(self.search_settings(query));
        
        // Поиск по модулям
        results.extend(self.search_modules(query));
        
        // Сортировка по релевантности
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        
        results.truncate(10);  // Топ 10
        results
    }
}
```

---

## 🔐 Диалоги подтверждения

### Критичные действия

```
┌──────────────────────────────────────────────────┐
│ ⚠ Подтверждение                       [x]       │
├──────────────────────────────────────────────────┤
│                                                   │
│        Остановить все модули?                    │
│                                                   │
│   Это действие остановит работу всех активных    │
│   модулей системы. Несохраненные данные могут    │
│   быть потеряны.                                 │
│                                                   │
│   Активные модули: 7                             │
│   Активные токены: 1,247                         │
│   Открытые соединения: 523                       │
│                                                   │
│   □ Сохранить состояние перед остановкой         │
│                                                   │
├──────────────────────────────────────────────────┤
│              [Отмена] [Остановить]               │
└──────────────────────────────────────────────────┘
```

### ROOT-режим подтверждение

```
┌──────────────────────────────────────────────────┐
│ 🔐 Требуется авторизация              [x]       │
├──────────────────────────────────────────────────┤
│                                                   │
│   Для выполнения этого действия требуется        │
│   режим администратора.                          │
│                                                   │
│   Пароль: [●●●●●●●●●●●●●●●●●●●●●]               │
│                                                   │
│   □ Запомнить на 5 минут                         │
│                                                   │
├──────────────────────────────────────────────────┤
│                  [Отмена] [Войти]                │
└──────────────────────────────────────────────────┘
```

---

## 🎨 Визуальные состояния окон

### Темы окон по режимам

```css
/* USER режим - мягкие окна */
.window-user {
    background: linear-gradient(180deg, #141414, #0a0a0a);
    border: 1px solid rgba(0, 255, 204, 0.1);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
    border-radius: 8px;
}

.window-user .title-bar {
    background: #1a1a1a;
    border-bottom: 1px solid #252525;
    height: 32px;
}

/* ROOT режим - системные окна */
.window-root {
    background: #000000;
    border: 1px solid #ff6600;
    box-shadow: 0 0 20px rgba(255, 102, 0, 0.2);
    border-radius: 0;  /* Острые углы */
}

.window-root .title-bar {
    background: linear-gradient(90deg, #1a0a00, #0a0a0a);
    border-bottom: 1px solid #ff6600;
    height: 28px;
    font-family: 'JetBrains Mono';
}

/* Критичные диалоги */
.window-critical {
    border: 2px solid #ff3366;
    animation: pulse-border 1s infinite;
}

@keyframes pulse-border {
    0%, 100% { border-color: #ff3366; }
    50% { border-color: #ff6600; }
}
```

### Анимации окон

```rust
enum WindowAnimation {
    // Появление
    FadeIn { duration: Duration },
    SlideIn { from: Direction, duration: Duration },
    ScaleIn { from: f32, duration: Duration },
    
    // Исчезание
    FadeOut { duration: Duration },
    SlideOut { to: Direction, duration: Duration },
    ScaleOut { to: f32, duration: Duration },
    
    // Переходы
    Morph { from: Rect, to: Rect, duration: Duration },
    Shake { intensity: f32, duration: Duration },
}

impl WindowManager {
    fn show_window(&mut self, window: Window, animation: WindowAnimation) {
        match animation {
            WindowAnimation::FadeIn { duration } => {
                self.animate_opacity(0.0, 1.0, duration);
            },
            WindowAnimation::SlideIn { from, duration } => {
                let start_pos = self.calculate_slide_start(from);
                self.animate_position(start_pos, window.position, duration);
            },
            // ...
        }
    }
}
```

---

## 🔄 Система состояний

### Сохранение и восстановление

```rust
struct WindowState {
    layout: WindowLayout,
    configs: HashMap<WindowId, WindowConfig>,
    user_preferences: UserPreferences,
}

struct WindowLayout {
    positions: HashMap<WindowId, Position>,
    sizes: HashMap<WindowId, Size>,
    docked: Vec<DockedWindow>,
    z_order: Vec<WindowId>,
}

impl WindowStateManager {
    fn save_state(&self) -> Result<()> {
        let state = self.collect_current_state();
        let json = serde_json::to_string_pretty(&state)?;
        fs::write(self.state_file_path(), json)?;
        Ok(())
    }
    
    fn restore_state(&mut self) -> Result<()> {
        let json = fs::read_to_string(self.state_file_path())?;
        let state: WindowState = serde_json::from_str(&json)?;
        self.apply_state(state)?;
        Ok(())
    }
    
    fn auto_save(&mut self) {
        // Автосохранение каждые N секунд
        if self.last_save.elapsed() > self.auto_save_interval {
            let _ = self.save_state();
            self.last_save = Instant::now();
        }
    }
}
```

---

## 📱 Адаптивность окон

### Responsive поведение

```rust
impl ResponsiveWindow {
    fn adapt_to_screen(&mut self, screen_size: Size) {
        match screen_size.width {
            0..=1280 => {
                // Компактный режим
                self.use_compact_layout();
                self.hide_optional_elements();
            },
            1281..=1920 => {
                // Стандартный режим
                self.use_standard_layout();
            },
            _ => {
                // Широкий экран
                self.use_expanded_layout();
                self.show_additional_panels();
            }
        }
    }
    
    fn handle_resize(&mut self, new_size: Size) {
        if new_size.width < self.min_size.width {
            // Переход в компактный режим
            self.switch_to_compact();
        }
        
        // Перераспределение пространства
        self.reflow_content(new_size);
    }
}
```

---

## 🏗️ Фабрика окон

### Создание окон

```rust
struct WindowFactory {
    templates: HashMap<WindowType, WindowTemplate>,
    theme: Theme,
}

impl WindowFactory {
    fn create_settings_window(&self) -> Window {
        Window {
            id: WindowId::new(),
            window_type: WindowType::Dialog,
            title: "Настройки".to_string(),
            size: Size::new(800, 600),
            position: Position::center(),
            content: Box::new(SettingsContent::new()),
            ..self.default_window()
        }
    }
    
    fn create_module_manager(&self) -> Window {
        Window {
            id: WindowId::new(),
            window_type: WindowType::Floating,
            title: "Менеджер модулей".to_string(),
            size: Size::new(900, 650),
            flags: WindowFlags {
                resizable: true,
                always_on_top: false,
                ..Default::default()
            },
            content: Box::new(ModuleManagerContent::new()),
            ..self.default_window()
        }
    }
    
    fn create_confirmation(&self, message: &str, critical: bool) -> Window {
        let window_type = if critical {
            WindowType::Modal
        } else {
            WindowType::Dialog
        };
        
        Window {
            id: WindowId::new(),
            window_type,
            title: if critical { "⚠ Внимание" } else { "Подтверждение" }.to_string(),
            size: Size::new(400, 200),
            position: Position::center(),
            flags: WindowFlags {
                resizable: false,
                movable: true,
                closable: !critical,
                ..Default::default()
            },
            content: Box::new(ConfirmationContent::new(message)),
            ..self.default_window()
        }
    }
}
```

---

## 📝 Итоговые принципы

### Консистентность
- Все окна следуют единой структуре
- Одинаковые отступы и размеры элементов
- Единая цветовая схема

### Расширяемость
- Легко добавить новые типы окон
- Модульная система контента
- Гибкая конфигурация

### Производительность
- Ленивая загрузка контента
- Виртуализация списков
- Кэширование состояний

### Безопасность
- Разделение прав для User/Root
- Подтверждение критичных действий
- Аудит изменений конфигурации

---

**Версия:** 1.0  
**Дата:** Октябрь 2025  
**Статус:** Базовая архитектура готова к расширению
