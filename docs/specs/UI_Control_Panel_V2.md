# 🎮 NeuroGraph OS Control Panel UI Specification


**Version:** v2.0 (Hielo+)
**Status:** In Development
**Framework:** iced (Rust)
**Architecture:** FFI (Direct Core Integration)
## 🔤 Кодировка и локализация

### Стандарты кодировки
```rust
// Вся система работает исключительно в UTF-8
const SYSTEM_ENCODING: &str = "UTF-8";

impl EncodingHandler {
    fn ensure_utf8(input: &[u8]) -> Result<String> {
        // Автоматическая конвертация из других кодировок
        match detect_encoding(input) {
            Encoding::UTF8 => String::from_utf8(input.to_vec()),
            Encoding::Windows1251 => convert_from_cp1251(input),
            Encoding::ASCII => String::from_utf8_lossy(input).into(),
            _ => fallback_to_utf8_lossy(input)
        }
    }
    
    fn normalize_text(text: &str) -> String {
        // Нормализация Unicode (NFC)
        text.nfc().collect::<String>()
    }
}
```

### Обработка специальных символов
- Все ASCII-art должны использовать базовый набор символов (коды 32-126)
- Для псевдографики используем Unicode Box Drawing (U+2500 - U+257F)
- Эмодзи заменяем на моноширинные иконки из Nerd Fonts

## 📊 Обзор интерфейса

### Концепция
Панель управления NeuroGraph OS — это центр мониторинга и взаимодействия с когнитивной системой. Интерфейс сочетает киберпанк-эстетику с функциональным минимализмом, создавая погружение в работу экспериментальной ОС.

### Архитектура компоновки

```
┌─────────────────────────────────────────────────────────────────┐
│  [Logo]  NeuroGraph OS v2.0  │  Status: ACTIVE  │  00:42:17 UTC │
├────────┬────────────────────────────────────────────────────────┤
│        │                                                         │
│  Dock  │                    Main Viewport                       │
│   (L)  │                                                         │
│        │                                                         │
│ ┌────┐ │  ┌──────────────────────────────────────────────────┐ │
│ │ 📊 │ │  │                                                  │ │
│ ├────┤ │  │            System Metrics Visualizations         │ │
│ │ 🧠 │ │  │                                                  │ │
│ ├────┤ │  │                                                  │ │
│ │ 🌐 │ │  └──────────────────────────────────────────────────┘ │
│ ├────┤ │                                                         │
│ │ 💬 │ │  ┌──────────────────────────────────────────────────┐ │
│ ├────┤ │  │                                                  │ │
│ │ ⚙️ │ │  │                 Chat Interface                   │ │
│ └────┘ │  │                                                  │ │
│        │  └──────────────────────────────────────────────────┘ │
└────────┴────────────────────────────────────────────────────────┘
```

---

## 🎨 Визуальная система

### Цветовая схема

#### Основная палитра
```css
/* Фоны */
--bg-primary:     #0a0a0a    /* Основной фон - глубокий черный */
--bg-secondary:   #141414    /* Панели - мягкий темно-серый */
--bg-tertiary:    #1a1a1a    /* Карточки - чуть светлее */
--bg-hover:       #252525    /* Состояние наведения */

/* Текст */
--text-primary:   #e0e0e0    /* Основной текст */
--text-secondary: #a0a0a0    /* Второстепенный */
--text-muted:     #606060    /* Приглушенный */

/* Акценты */
--accent-primary: #00ffcc    /* Неоново-бирюзовый */
--accent-blue:    #3399ff    /* Яркий синий */
--accent-purple:  #9966ff    /* Фиолетовый */

/* Статусы */
--status-ok:      #33ff66    /* Зеленый неон */
--status-warning: #ffaa33    /* Оранжевый */
--status-critical:#ff3366    /* Красный неон */

/* Градиенты */
--gradient-cold:  linear-gradient(135deg, #3399ff, #00ffcc)
--gradient-hot:   linear-gradient(135deg, #ff3366, #ffaa33)
--gradient-neural:linear-gradient(135deg, #9966ff, #3399ff, #00ffcc)
```

### Типографика

```css
/* Шрифты */
--font-main:      'Inter', -apple-system, sans-serif
--font-mono:      'JetBrains Mono', 'Fira Code', monospace
--font-display:   'Orbitron', 'Inter', sans-serif  /* Для заголовков */

/* Размеры */
--text-xs:        11px
--text-sm:        13px
--text-base:      14px
--text-lg:        16px
--text-xl:        20px
--text-2xl:       24px
--text-display:   32px

/* Весовые коэффициенты */
--weight-light:   300
--weight-regular: 400
--weight-medium:  500
--weight-bold:    700
```

---

## 📊 Визуализации системных метрик

### 1. CPU Load — Волновая форма
```
Компонент: WaveformVisualizer
Размер: 320x120px
```

**Визуальное описание:**
- Анимированная синусоида с переменной амплитудой
- 60 FPS плавная анимация
- Эффект свечения при высокой нагрузке

**Параметры:**
```rust
struct CPUWaveform {
    amplitude: f32,      // 0.0 - 1.0 (загрузка)
    frequency: f32,      // Базовая частота волны
    color_gradient: Gradient,
    glow_intensity: f32, // Интенсивность свечения
    trail_length: u8,    // Длина следа (эффект размытия)
}
```

**Цветовая логика:**
- 0-30%: `#00ffcc` (бирюзовый)
- 31-60%: `#3399ff` (синий)
- 61-85%: `#9966ff` (фиолетовый)
- 86-100%: `#ff3366` (красный + пульсация)

### 2. RAM — Голографический куб
```
Компонент: HoloCube
Размер: 200x200px
```

**Визуальное описание:**
- 3D куб с полупрозрачными гранями
- Заполнение снизу вверх как жидкость
- Мерцающие частицы внутри = активные процессы

**Структура:**
```rust
struct MemoryCube {
    fill_level: f32,        // 0.0 - 1.0 (использование)
    particles: Vec<Particle>,// Активные процессы
    edge_glow: Color,       // Цвет свечения граней
    liquid_color: Gradient, // Градиент "жидкости"
    rotation: Quaternion,   // Медленное вращение
}
```

**Анимация частиц:**
- Размер частицы ∝ объему памяти процесса
- Скорость движения ∝ активности процесса
- Цвет: от белого (idle) до яркого (active)

### 3. Temperature — Термальная сфера
```
Компонент: ThermalSphere
Размер: 180x180px
```

**Визуальное описание:**
- Пульсирующее ядро в центре
- Радиальный градиент температуры
- "Искры" при критических значениях

**Температурные зоны:**
```rust
enum ThermalState {
    Cold {        // < 40°C
        core_color: Color::CYAN,
        pulse_rate: 0.5, // Hz
    },
    Normal {      // 40-65°C
        core_color: Color::BLUE,
        pulse_rate: 1.0,
    },
    Warm {        // 65-80°C
        core_color: Color::PURPLE,
        pulse_rate: 2.0,
    },
    Hot {         // 80-90°C
        core_color: Color::ORANGE,
        pulse_rate: 3.0,
        sparks: true,
    },
    Critical {    // > 90°C
        core_color: Color::RED,
        pulse_rate: 5.0,
        sparks: true,
        warning_ring: true,
    }
}
```

### 4. Disk I/O — Вращающиеся кольца
```
Компонент: DiskRings
Размер: 200x200px
```

**Визуальное описание:**
- Концентрические кольца (3-5 слоев)
- Скорость вращения = активность
- Мерцающие сегменты = операции I/O

**Параметры:**
```rust
struct DiskRings {
    rings: Vec<Ring>,
    read_color: Color::GREEN,
    write_color: Color::ORANGE,
    idle_color: Color::GRAY,
}

struct Ring {
    radius: f32,
    rotation_speed: f32,  // об/сек
    segments: Vec<Segment>,
    opacity: f32,
}
```

### 5. Network — Матричный поток
```
Компонент: DataMatrix
Размер: 280x160px
```

**Визуальное описание:**
- Вертикальные потоки символов (как в Matrix)
- Плотность = нагрузка сети
- Разные цвета для upload/download

**Конфигурация:**
```rust
struct NetworkMatrix {
    columns: Vec<DataColumn>,
    upload_color: Color::GREEN,
    download_color: Color::BLUE,
    symbol_set: &str,  // Набор символов для отображения
}

struct DataColumn {
    position: f32,
    speed: f32,        // Скорость падения
    density: f32,      // Плотность символов
    symbols: Vec<char>,
}
```

---

## 🎭 Режимы визуализации (User vs Root)

### USER режим — упрощенная визуализация

```rust
struct UserModeVisuals {
    style: VisualizationStyle::Simplified,
    update_rate: Duration::from_millis(500),  // Обновление 2 раза в секунду
    details_level: DetailLevel::Basic,
}

// Пример: CPU показывается как простой прогресс-бар
fn render_cpu_user(&self) -> Widget {
    ProgressBar {
        value: self.cpu_usage,
        color: self.gradient_by_value(),
        label: format!("CPU: {}%", self.cpu_usage * 100.0),
        style: ProgressBarStyle::Smooth,
    }
}
```

### ROOT режим — детальная визуализация

```rust
struct RootModeVisuals {
    style: VisualizationStyle::Detailed,
    update_rate: Duration::from_millis(100),  // Обновление 10 раз в секунду
    details_level: DetailLevel::Full,
    show_pids: bool,
    show_threads: bool,
}

// Пример: CPU показывается с деталями по ядрам
fn render_cpu_root(&self) -> Widget {
    MultiCoreView {
        cores: self.cpu_cores.iter().map(|core| {
            CoreWidget {
                id: core.id,
                usage: core.usage,
                frequency: core.frequency,
                temperature: core.temp,
                processes: core.top_processes(5),
            }
        }).collect(),
        style: CoreViewStyle::Matrix,
    }
}
```

### Адаптивное переключение

```rust
impl VisualizationManager {
    fn switch_mode(&mut self, mode: UserMode) {
        match mode {
            UserMode::Regular => {
                self.apply_user_visuals();
                self.hide_system_internals();
                self.simplify_graphs();
            },
            UserMode::Root => {
                self.apply_root_visuals();
                self.show_system_internals();
                self.enable_raw_data_view();
            }
        }
    }
}
```

---

## 💬 Chat Interface — Двухрежимная система

### Режим USER (диалоговый)

```
┌──────────────────────────────────────────────────┐
│ NeuroGraph Assistant               Mode: USER    │ <- Бледно-бирюзовая рамка 1px (#00ffcc33)
├──────────────────────────────────────────────────┤
│                                                   │
│  Привет! Как я могу помочь с NeuroGraph OS?     │
│                                                   │
│  ┌─────────────────────────────────────────────┐ │
│  │ User: Расскажи о текущем состоянии системы  │ │
│  └─────────────────────────────────────────────┘ │
│                                                   │
│  ┌─────────────────────────────────────────────┐ │
│  │ Assistant:                                  │ │
│  │                                             │ │
│  │ Система работает в штатном режиме:         │ │
│  │                                             │ │
│  │ • Загрузка CPU: 42%                        │ │
│  │ • Память: 3.2/8 GB                         │ │
│  │ • Активных токенов: 1,247                  │ │
│  │ • Связей в графе: 8,923                    │ │
│  │                                             │ │
│  │ Все модули функционируют нормально.        │ │
│  └─────────────────────────────────────────────┘ │
│                                                   │
├──────────────────────────────────────────────────┤
│ [Введите вопрос...]                      [Enter] │
└──────────────────────────────────────────────────┘
```

### Режим ROOT (командный)

```
┌──────────────────────────────────────────────────┐
│ NeuroGraph Terminal              Mode: ROOT      │ <- Оранжевая рамка 1px (#ff6600)
├──────────────────────────────────────────────────┤
│ root@neurograph:~$ status --all                  │
│                                                   │
│ MODULE STATUS REPORT                             │
│ =====================================             │
│ TOKEN_MANAGER    : RUNNING  [PID: 1247]         │
│ CONNECTION_POOL  : RUNNING  [PID: 1248]         │
│ GRID_INDEX      : RUNNING  [PID: 1249]         │
│ GRAPH_ENGINE    : PAUSED   [PID: 1250]         │
│ GUARDIAN        : RUNNING  [PID: 1001]         │
│                                                   │
│ root@neurograph:~$ graph start                   │
│ [OK] Graph Engine started successfully           │
│                                                   │
│ root@neurograph:~$ monitor cpu --interval 1s     │
│ CPU: [████████████░░░░░░░░] 62.3%               │
│ CPU: [██████████░░░░░░░░░░] 51.7%               │
│ CPU: [████████████████░░░░] 78.9%               │
│ ^C                                               │
│                                                   │
│ root@neurograph:~$ _                             │
└──────────────────────────────────────────────────┘
```

### Визуальные различия режимов

```css
/* USER режим - дружелюбный интерфейс */
.chat-mode-user {
    border: 1px solid rgba(0, 255, 204, 0.2);  /* Бледно-бирюзовая */
    background: linear-gradient(180deg, #0a0a0a, #0f0f0f);
    font-family: 'Inter', sans-serif;
}

.chat-mode-user .message {
    border-radius: 12px;
    padding: 12px 16px;
    margin: 8px 0;
    animation: fadeIn 0.3s ease;
}

/* ROOT режим - терминальный интерфейс */
.chat-mode-root {
    border: 1px solid #ff6600;  /* Оранжевая предупреждающая */
    background: #000000;  /* Чистый черный */
    font-family: 'JetBrains Mono', monospace;
}

.chat-mode-root .output {
    color: #00ff00;  /* Классический терминальный зеленый */
    white-space: pre-wrap;
    line-height: 1.4;
}

.chat-mode-root .error {
    color: #ff3333;
}

.chat-mode-root .warning {
    color: #ffaa00;
}
```

### Переключение режимов

```rust
impl ChatMode {
    fn switch_mode(&mut self, password: Option<&str>) -> Result<()> {
        match self.current {
            Mode::User => {
                // Требуется пароль для входа в root
                if authenticate_root(password)? {
                    self.current = Mode::Root;
                    self.apply_root_theme();
                    Ok(())
                } else {
                    Err("Authentication failed")
                }
            },
            Mode::Root => {
                // Выход из root без пароля
                self.current = Mode::User;
                self.apply_user_theme();
                Ok(())
            }
        }
    }
}
```

---

## 📚 Библиотека команд ROOT

### Базовые команды

```bash
# Управление модулями
status [--all | --module NAME]     # Статус модулей
start MODULE_NAME                   # Запуск модуля
stop MODULE_NAME                    # Остановка модуля
restart MODULE_NAME                 # Перезапуск
pause MODULE_NAME                   # Пауза модуля
resume MODULE_NAME                  # Возобновление

# Мониторинг
monitor METRIC [--interval TIME]   # Мониторинг в реальном времени
  cpu                              # Загрузка процессора
  memory                           # Использование памяти
  disk                             # Дисковый I/O
  network                          # Сетевая активность
  tokens                           # Активность токенов

# Работа с графом
graph info                         # Информация о графе
graph stats                        # Статистика
graph query "QUERY"                # Запрос к графу
graph export --format [dot|json]   # Экспорт графа
graph import FILE                  # Импорт графа

# Управление токенами
token create --space LEVEL         # Создать токен
token delete ID                    # Удалить токен
token list [--filter EXPR]         # Список токенов
token inspect ID                   # Детали токена

# Системные команды
clear                              # Очистить экран
history [--last N]                 # История команд
config get KEY                     # Получить конфигурацию
config set KEY VALUE               # Установить параметр
log [--level LEVEL] [--tail N]    # Просмотр логов

# DNA операции
dna status                         # Статус DNA Guardian
dna validate                       # Валидация CDNA
dna cache --clear                  # Очистка кэша
dna stats                          # Статистика обращений
```

### Расширенные команды

```bash
# Отладка
debug enable MODULE                # Включить отладку
debug disable MODULE               # Выключить отладку
trace TOKEN_ID                     # Трассировка токена
profile [--duration SECS]          # Профилирование

# Пакетные операции
batch < script.ngs                 # Выполнить скрипт
pipe COMMAND | COMMAND             # Конвейер команд
parallel [COMMANDS]                # Параллельное выполнение

# Экспериментальные
evolve --iterations N              # Запустить эволюцию
intuition analyze                  # Анализ интуиции
consensus vote                     # Голосование консенсуса
```

### Автодополнение и подсказки

```rust
struct CommandCompleter {
    commands: HashMap<String, CommandInfo>,
    history: Vec<String>,
    
    fn suggest(&self, partial: &str) -> Vec<String> {
        self.commands
            .keys()
            .filter(|cmd| cmd.starts_with(partial))
            .cloned()
            .collect()
    }
    
    fn show_help(&self, command: &str) -> String {
        self.commands
            .get(command)
            .map(|info| info.help_text.clone())
            .unwrap_or("Unknown command".to_string())
    }
}
```

---

## 🎛️ Левая панель (Dock)

### Монохромные иконки

```rust
// Используем символы из Nerd Fonts для консистентного вида
enum DockIcon {
    Metrics {
        icon: "", // nf-oct-graph (график)
        label: "Метрики",
        badge: Option<u32>,
    },
    Neural {
        icon: "", // nf-md-brain (мозг)
        label: "Нейросеть",
        pulse: bool,
    },
    Network {
        icon: "", // nf-md-graphql (граф)
        label: "Граф",
        connections: u32,
    },
    Chat {
        icon: "", // nf-oct-comment (чат)
        label: "Чат",
        unread: u32,
    },
    Settings {
        icon: "", // nf-oct-gear (настройки)
        label: "Настройки",
    },
    Wiki {
        icon: "", // nf-oct-book (документация)
        label: "Wiki",
    },
}

// Альтернатива - ASCII-art иконки
const ASCII_ICONS: &[(&str, &str)] = &[
    ("metrics",  "[≈]"),  // Волны
    ("neural",   "[◉]"),  // Нейрон
    ("network",  "[⬡]"),  // Граф (гексагон)
    ("chat",     "[◐]"),  // Диалог
    ("settings", "[⚙]"),  // Шестеренка
    ("wiki",     "[?]"),  // Справка
];
```

### Визуальные состояния

#### Обычное состояние
```css
.dock-icon {
    width: 48px;
    height: 48px;
    background: #141414;
    border: 1px solid #252525;
    border-radius: 12px;
    transition: all 0.2s ease;
}
```

#### Активное состояние
```css
.dock-icon.active {
    background: linear-gradient(135deg, #1a1a1a, #252525);
    border-color: #00ffcc;
    box-shadow: 0 0 20px rgba(0, 255, 204, 0.3);
}
```

#### Состояние с алертом
```css
.dock-icon.alert::after {
    content: attr(data-badge);
    position: absolute;
    top: -4px;
    right: -4px;
    background: #ff3366;
    color: white;
    border-radius: 10px;
    padding: 2px 6px;
    font-size: 10px;
    animation: pulse 1s infinite;
}
```

---

## ⚡ Анимации и переходы

### Базовые анимации

```css
/* Пульсация для критических состояний */
@keyframes pulse {
    0%, 100% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.7; transform: scale(1.05); }
}

/* Свечение для активных элементов */
@keyframes glow {
    0%, 100% { box-shadow: 0 0 5px currentColor; }
    50% { box-shadow: 0 0 20px currentColor, 0 0 30px currentColor; }
}

/* Вращение для загрузки */
@keyframes rotate {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
}

/* Появление элементов */
@keyframes fadeIn {
    from { opacity: 0; transform: translateY(10px); }
    to { opacity: 1; transform: translateY(0); }
}
```

### Переходы между состояниями

```rust
struct Transition {
    duration: Duration::from_millis(200),
    easing: EasingFunction::EaseOutCubic,
    properties: vec!["opacity", "transform", "color"],
}
```

---

## 🔔 Уведомления и алерты

### Типы уведомлений

```rust
enum NotificationType {
    Info {
        icon: "[i]",  // ASCII символ
        color: Color::CYAN,
        duration: Duration::from_secs(3),
    },
    Success {
        icon: "[✓]",  // или "[OK]"
        color: Color::GREEN,
        duration: Duration::from_secs(2),
    },
    Warning {
        icon: "[!]",  // Восклицательный знак
        color: Color::ORANGE,
        duration: Duration::from_secs(5),
    },
    Critical {
        icon: "[X]",  // или "[!!]"
        color: Color::RED,
        duration: Duration::from_secs(10),
        pulse: true,
    },
}
```

### Визуальное представление

```
┌──────────────────────────────────────┐
│ [!] Заголовок уведомления        [x] │
├──────────────────────────────────────┤
│ Описание проблемы или события       │
│                                      │
│ [Action]  [Dismiss]                  │
└──────────────────────────────────────┘
```

### Различие для режимов

```css
/* USER режим - мягкие уведомления */
.notification-user {
    background: rgba(20, 20, 20, 0.95);
    border: 1px solid rgba(0, 255, 204, 0.3);
    backdrop-filter: blur(10px);
}

/* ROOT режим - системные алерты */
.notification-root {
    background: #000000;
    border: 1px solid #ff6600;
    font-family: 'JetBrains Mono';
    text-transform: uppercase;
}
```

---

## 📱 Адаптивность и масштабирование

### Брейкпоинты

```css
/* Компактный режим (< 1280px) */
@media (max-width: 1279px) {
    .dock { width: 60px; }
    .dock-icon { width: 40px; height: 40px; }
    .main-viewport { padding: 16px; }
}

/* Полный режим (>= 1280px) */
@media (min-width: 1280px) {
    .dock { width: 80px; }
    .dock-icon { width: 48px; height: 48px; }
    .main-viewport { padding: 24px; }
}

/* Широкий экран (>= 1920px) */
@media (min-width: 1920px) {
    .metrics-grid { grid-template-columns: repeat(3, 1fr); }
}
```

### DPI Scaling

```rust
impl UIScaling {
    fn calculate_scale(dpi: f32) -> f32 {
        match dpi {
            0.0..=96.0 => 1.0,
            96.1..=120.0 => 1.25,
            120.1..=144.0 => 1.5,
            144.1..=192.0 => 1.75,
            _ => 2.0,
        }
    }
}
```

---

## 🎮 Интерактивность

### Горячие клавиши

```rust
// Базовые хоткеи (работают в обоих режимах)
const GLOBAL_HOTKEYS: &[Hotkey] = &[
    Hotkey { key: "Ctrl+M", action: Action::ToggleMetrics },
    Hotkey { key: "Ctrl+N", action: Action::OpenNeural },
    Hotkey { key: "Ctrl+G", action: Action::ShowGraph },
    Hotkey { key: "Ctrl+/", action: Action::FocusChat },
    Hotkey { key: "Ctrl+,", action: Action::OpenSettings },
    Hotkey { key: "Ctrl+W", action: Action::OpenWiki },
    Hotkey { key: "Escape", action: Action::CloseModal },
    Hotkey { key: "F11", action: Action::ToggleFullscreen },
    Hotkey { key: "F1", action: Action::ShowHelp },
];

// Дополнительные хоткеи для ROOT режима
const ROOT_HOTKEYS: &[Hotkey] = &[
    Hotkey { key: "Ctrl+R", action: Action::RestartModule },
    Hotkey { key: "Ctrl+K", action: Action::KillProcess },
    Hotkey { key: "Ctrl+L", action: Action::ClearLog },
    Hotkey { key: "Ctrl+D", action: Action::DebugMode },
    Hotkey { key: "Ctrl+Shift+C", action: Action::EmergencyStop },
];

// Пользовательские хоткеи (настраиваемые)
struct CustomHotkey {
    key_combination: String,
    command: String,
    description: String,
    created_by: UserId,
    created_at: Timestamp,
}

impl HotkeyManager {
    fn add_custom(&mut self, hotkey: CustomHotkey) -> Result<()> {
        // Проверка на конфликты
        if self.is_conflict(&hotkey.key_combination) {
            return Err("Hotkey conflict");
        }
        self.custom_hotkeys.push(hotkey);
        self.save_to_config()?;
        Ok(())
    }
}
```

---

## 📖 Wiki Interface

### Структура Wiki

```
┌──────────────────────────────────────────────────┐
│ NeuroGraph Wiki                    [Search: ___] │
├────────┬─────────────────────────────────────────┤
│        │                                          │
│ Навиг. │           Содержание                    │
│        │                                          │
│ ▼ Быст.│  # Горячие клавиши                     │
│   Старт│                                          │
│   • Хот│  ## Глобальные                          │
│     кеи│  - `Ctrl+M` - Метрики                   │
│   • Ком│  - `Ctrl+N` - Нейросеть                │
│     анды│  - `Ctrl+G` - Граф                     │
│        │  - `F1` - Помощь                        │
│ ▼ ROOT │                                          │
│   • Мод│  ## ROOT режим                          │
│     ули│  - `Ctrl+R` - Перезапуск модуля        │
│   • Граф│  - `Ctrl+K` - Убить процесс            │
│   • DNA│  - `Ctrl+Shift+C` - Аварийная остановка│
│        │                                          │
│ ▼ API  │  ## Пользовательские                   │
│   • Тип│  [+ Добавить хоткей]                    │
│     ы  │                                          │
│   • Мет│  ### Мои хоткеи:                        │
│     оды│  - `Alt+1` → "monitor cpu"              │
│        │  - `Alt+2` → "graph stats"              │
│ ▼ Прим.│                                          │
│   • Скр│  [Редактировать] [Удалить]             │
│     ипты│                                          │
└────────┴─────────────────────────────────────────┘
```

### Wiki в режиме ROOT

```rust
struct WikiPage {
    title: String,
    content: Markdown,
    category: Category,
    tags: Vec<String>,
    editable: bool,  // true для ROOT
}

impl WikiEditor {
    fn edit_page(&mut self, page_id: &str, new_content: &str) -> Result<()> {
        if !self.user.is_root() {
            return Err("Permission denied");
        }
        
        let page = self.get_page_mut(page_id)?;
        page.content = parse_markdown(new_content)?;
        page.updated_at = Timestamp::now();
        page.updated_by = self.user.id;
        
        self.save_to_storage()?;
        self.index_for_search(page)?;
        Ok(())
    }
    
    fn add_command(&mut self, cmd: Command) -> Result<()> {
        // Добавление новой команды в Wiki
        let doc = self.generate_command_doc(&cmd)?;
        self.add_page(doc)?;
        self.update_command_index(&cmd)?;
        Ok(())
    }
}
```

### Интерактивные элементы Wiki

```css
/* Редактор команд в Wiki */
.wiki-command-editor {
    background: #0a0a0a;
    border: 1px solid #ff6600;  /* Оранжевый для ROOT */
    border-radius: 8px;
    padding: 16px;
}

.wiki-command-editor input {
    background: #141414;
    border: 1px solid #252525;
    color: #e0e0e0;
    font-family: 'JetBrains Mono';
    padding: 8px;
    margin: 4px 0;
}

.wiki-hotkey-builder {
    display: grid;
    grid-template-columns: 150px 1fr 100px;
    gap: 8px;
    align-items: center;
}

.wiki-hotkey-builder .key-input {
    background: #1a1a1a;
    border: 1px solid #3399ff;
    border-radius: 4px;
    padding: 6px;
}

.wiki-hotkey-builder .command-input {
    font-family: 'JetBrains Mono';
    background: #0a0a0a;
}

.wiki-hotkey-builder .test-btn {
    background: linear-gradient(135deg, #1a1a1a, #252525);
    border: 1px solid #00ffcc;
    color: #00ffcc;
    cursor: pointer;
    transition: all 0.2s;
}

.wiki-hotkey-builder .test-btn:hover {
    box-shadow: 0 0 10px rgba(0, 255, 204, 0.5);
}
```

---

## ♿ Доступность

### Навигация с клавиатуры

```rust
impl KeyboardNavigation {
    fn handle_key(&mut self, key: KeyEvent) -> Action {
        match (key.code, key.modifiers) {
            // Tab навигация
            (KeyCode::Tab, KeyModifiers::NONE) => {
                self.focus_next_element()
            },
            (KeyCode::Tab, KeyModifiers::SHIFT) => {
                self.focus_previous_element()
            },
            
            // Стрелки для навигации по спискам
            (KeyCode::Up, _) => self.select_previous(),
            (KeyCode::Down, _) => self.select_next(),
            (KeyCode::Left, _) => self.collapse_section(),
            (KeyCode::Right, _) => self.expand_section(),
            
            // Enter для активации
            (KeyCode::Enter, _) => self.activate_focused(),
            
            // Space для чекбоксов и переключателей
            (KeyCode::Space, _) => self.toggle_focused(),
            
            // Escape для отмены/закрытия
            (KeyCode::Escape, _) => self.close_or_cancel(),
            
            _ => Action::None
        }
    }
}

// Фокус индикация
struct FocusIndicator {
    style: FocusStyle::Outline,  // или FocusStyle::Glow
    color: Color::from_hex("#00ffcc"),
    width: 2.0,
}
```

### Поддержка мыши

```rust
impl MouseSupport {
    fn handle_mouse(&mut self, event: MouseEvent) -> Action {
        match event {
            MouseEvent::Click(button, pos) => {
                match button {
                    MouseButton::Left => self.primary_action(pos),
                    MouseButton::Right => self.show_context_menu(pos),
                    MouseButton::Middle => self.pan_view(pos),
                    _ => Action::None
                }
            },
            
            MouseEvent::DoubleClick(pos) => {
                self.activate_element_at(pos)
            },
            
            MouseEvent::Scroll(delta) => {
                if self.modifiers.ctrl {
                    self.zoom(delta)
                } else {
                    self.scroll(delta)
                }
            },
            
            MouseEvent::Hover(pos) => {
                self.show_tooltip_at(pos)
            },
            
            MouseEvent::Drag(start, current) => {
                self.drag_element(start, current)
            }
        }
    }
}

// Курсоры для разных состояний
enum CursorStyle {
    Default,      // Стрелка
    Pointer,      // Рука для кликабельных элементов
    Text,         // I-beam для текста
    Move,         // Перемещение
    Resize,       // Изменение размера
    Wait,         // Загрузка
    NotAllowed,   // Запрещено
}
```

### Screen Reader поддержка

```rust
impl Accessibility {
    fn aria_labels() -> HashMap<&'static str, &'static str> {
        hashmap! {
            "metrics_panel" => "Панель системных метрик",
            "cpu_viz" => "График загрузки процессора",
            "memory_viz" => "Визуализация использования памяти",
            "chat_input" => "Поле ввода сообщения",
            "root_terminal" => "Терминал администратора",
            "wiki_search" => "Поиск по документации",
        }
    }
    
    fn announce(&self, message: &str) {
        // Объявление для screen reader
        self.screen_reader.speak(message);
    }
}
```

---

## 🔊 Звуковая обратная связь (опционально)

### Звуковые события

```rust
enum SoundEvent {
    ModuleStart => "sounds/module_start.ogg",
    ModuleStop => "sounds/module_stop.ogg",
    Alert => "sounds/alert.ogg",
    Success => "sounds/success.ogg",
    MessageReceived => "sounds/message.ogg",
    Error => "sounds/error.ogg",
}
```

### Параметры звука

```rust
struct SoundSettings {
    enabled: bool,
    volume: f32,  // 0.0 - 1.0
    ambient_enabled: bool,  // Фоновые звуки системы
}
```

---

## 🏁 Финальные рекомендации

### Принципы дизайна

1. **Монохромная элегантность** — минимум цветов, максимум контраста
2. **Режимная адаптивность** — User для комфорта, Root для контроля
3. **Информативность без перегрузки** — показываем только важное
4. **Плавность и отзывчивость** — 60 FPS для всех анимаций
5. **Доступность** — полная поддержка клавиатуры и мыши

### Визуальная иерархия по режимам

```rust
enum VisualPriority {
    User {
        focus: "Понятность и дружелюбность",
        colors: vec!["#00ffcc33", "#e0e0e0", "#1a1a1a"],
        complexity: Complexity::Low,
    },
    Root {
        focus: "Детализация и контроль",
        colors: vec!["#ff6600", "#00ff00", "#000000"],
        complexity: Complexity::High,
    }
}
```

### Производительность

- GPU-ускорение для визуализаций
- Виртуализация списков для больших объемов данных
- Lazy loading для тяжелых компонентов
- Кэширование отрендеренных элементов
- Разная частота обновления для User (2Hz) и Root (10Hz)

### Расширяемость

- Модульная архитектура компонентов
- Настраиваемые команды и хоткеи через Wiki
- API для внешних интеграций
- Плагины для дополнительных визуализаций

### Безопасность UI

```rust
impl SecurityFeatures {
    fn validate_mode_switch(&self) -> bool {
        // Требуем подтверждение при переходе в Root
        self.require_password && self.show_warning
    }
    
    fn audit_root_actions(&self) {
        // Логируем все действия в Root режиме
        self.log_command_history();
        self.track_system_changes();
    }
}

---

## 📝 Примечания

Этот документ определяет визуальную и функциональную структуру панели управления NeuroGraph OS. Все цвета, размеры и анимации оптимизированы для длительной работы с минимальной нагрузкой на глаза при сохранении футуристичной эстетики системы.

**Версия:** 1.0  
**Дата:** Октябрь 2025  
**Статус:** Готов к реализации
