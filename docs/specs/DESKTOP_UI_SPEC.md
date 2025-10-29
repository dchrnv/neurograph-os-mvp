# NeuroGraph Desktop UI - Specification

**Version:** v1.0 (Hielo+1)
**Status:** In Development
**Framework:** iced (Rust)
**Architecture:** FFI (Direct Core Integration)

---

## Обзор

NeuroGraph Desktop - это минималистичный desktop-интерфейс в стиле Unity для взаимодействия с NeuroGraph OS. Основная цель - обеспечить безопасный доступ к системе через диалоговое окно и управление конфигурациями модулей.

---

## Архитектура

### Единый процесс с прямой интеграцией:

```
┌─────────────────────────────────────┐
│ Desktop UI (iced framework)         │  ← Native window
│   - Auth screen                     │
│   - Workspace navigation            │
│   - Chat interface                  │
│   - CoreBridge wrapper              │
└─────────────────┬───────────────────┘
                  │ Direct FFI calls
                  ↓
┌─────────────────────────────────────┐
│ neurograph-core (Rust modules)      │  ← Core logic
│   - Token, Grid, Graph              │
│   - Guardian, CDNA                  │
└─────────────────────────────────────┘
```

**Ключевые особенности:**
- Один процесс (desktop + core в одном исполняемом файле)
- Прямые вызовы функций (без HTTP/REST)
- Нулевые накладные расходы на сериализацию
- Нативная производительность
- Общая память через Arc<Mutex<T>>

**Преимущества перед REST:**
- Микросекунды вместо миллисекунд
- Типобезопасность на этапе компиляции
- Проще отладка (один процесс)
- Нет JSON сериализации
- Нативная обработка ошибок Rust

---

## CoreBridge - Обёртка для ядра

### Структура

```rust
// src/desktop/src/core/mod.rs
use neurograph_core::{Token, Grid, Graph, Guardian, CDNA};
use std::sync::{Arc, Mutex};

pub struct CoreBridge {
    grid: Arc<Mutex<Grid>>,
    graph: Arc<Mutex<Graph>>,
    guardian: Arc<Mutex<Guardian>>,
}

impl CoreBridge {
    pub fn new() -> Self {
        let cdna = CDNA::default();
        let guardian = Guardian::new(cdna);

        Self {
            grid: Arc::new(Mutex::new(Grid::new())),
            graph: Arc::new(Mutex::new(Graph::new())),
            guardian: Arc::new(Mutex::new(guardian)),
        }
    }

    pub fn process_message(&self, msg: &str) -> String {
        match msg.to_lowercase().as_str() {
            "статус" | "status" => self.get_status(),
            "создать токен" | "create token" => self.create_token(),
            "help" | "помощь" => self.help(),
            _ => format!("Команда не распознана: '{}'", msg),
        }
    }

    fn get_status(&self) -> String {
        let grid = self.grid.lock().unwrap();
        let graph = self.graph.lock().unwrap();

        format!(
            "System Status:\n\
             - Tokens in grid: {}\n\
             - Graph nodes: {}\n\
             - Graph edges: {}\n\
             - Mode: Running",
            grid.len(),
            graph.node_count(),
            graph.edge_count()
        )
    }

    fn create_token(&self) -> String {
        let mut grid = self.grid.lock().unwrap();
        let token_id = (grid.len() + 1) as u32;
        let token = Token::new(token_id);

        // Валидация через Guardian
        if let Err(e) = self.guardian.lock().unwrap().validate_token(&token) {
            return format!("Ошибка валидации: {:?}", e);
        }

        match grid.add(token) {
            Ok(_) => format!("✓ Токен создан (ID: {})", token_id),
            Err(e) => format!("Ошибка: {:?}", e),
        }
    }
}
```

### Использование в UI

```rust
// src/desktop/src/app.rs
pub struct NeuroGraphApp {
    auth_state: AuthState,
    current_workspace: Workspace,
    core: CoreBridge,  // Прямой доступ к Rust core!
}

impl Application for NeuroGraphApp {
    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                auth_state: AuthState::new(),
                current_workspace: Workspace::Welcome,
                core: CoreBridge::new(),  // Инициализация core
            },
            Command::none(),
        )
    }

    fn view(&self) -> Element<Message> {
        // Передаём core в workspaces
        self.current_workspace.view(&self.auth_state, &self.core)
    }
}
```

```rust
// src/desktop/src/workspaces/mod.rs
fn chat_view(core: &CoreBridge) -> Element<'static, Message> {
    // Прямой вызов функции - никакого HTTP!
    let demo_status = core.process_message("status");

    column![
        text("Chat").size(24),
        text("Phase 1 Demo: Direct Core Integration"),
        text("Выполняем: core.process_message(\"status\")"),
        text(demo_status).font(iced::Font::MONOSPACE),
    ]
    .into()
}
```

---

## Безопасность

### Аутентификация

**Два режима доступа:**
- **User Mode** - обычный доступ (диалог, просмотр конфигов)
- **Admin Mode** - расширенный доступ (редактирование CDNA, критичные изменения)

### Хранение паролей

**Технология:** Argon2id (современный стандарт хеширования)

**Расположение:**
```
~/.config/neurograph/auth.json
```

**Формат:**
```json
{
  "user_password_hash": "$argon2id$v=19$m=19456,t=2,p=1$...",
  "root_password_hash": "$argon2id$v=19$m=19456,t=2,p=1$..."
}
```

**Права доступа:** 0600 (только владелец файла)

### Блокировка экрана

- Вручную через кнопку "Lock" в интерфейсе
- При блокировке требуется повторный ввод пароля
- Автоматической блокировки по таймауту пока нет

### Безопасность процесса

- Один процесс (нет сетевых атак)
- Memory-safe Rust
- Нет HTTP поверхности атак
- Только прямые вызовы функций

---

## Интерфейс

### Unity-style Layout

```
┌─┬────────────────────────────────┐
│🏠│                                │
│ │     Active Workspace           │
│💬│                                │
│ │                                │
│⚙️│                                │
│ │                                │
│📊│                                │
│ │                                │
│🔒│  (только в Admin mode)         │
└─┴────────────────────────────────┘
```

**Элементы:**
- **Левый Dock** - навигация между экранами (иконки)
- **Основная область** - активный workspace
- **Нет верхней панели** - минимализм

---

## Workspaces (Экраны)

### 1. Login Screen (Первый экран)

```
┌────────────────────────────────────┐
│                                    │
│    [ASCII Art Logo]                │
│    NEUROGRAPH OS                   │
│                                    │
│   ⚠️ Warning: Experimental System  │
│                                    │
│   ┌──────────────────────┐         │
│   │ Password: ●●●●●●     │         │
│   └──────────────────────┘         │
│                                    │
│    [User Login]  [Root Login]      │
│                                    │
└────────────────────────────────────┘
```

**Функции:**
- Ввод пароля (скрытый текст)
- Выбор режима: User или Root
- Проверка пароля через Argon2id
- Переход в Home после успешного входа

---

### 2. Home/Welcome Screen

```
┌─┬────────────────────────────────┐
│🏠│ Welcome to NeuroGraph OS       │
│💬│                                │
│⚙️│   [ASCII Art Character]        │
│📊│                                │
│🔒│   System Status: Running       │
│ │   Tokens: 1,234                │
│ │   Mode: User/Admin             │
│ │                                │
│ │   → Click Chat to begin        │
└─┴────────────────────────────────┘
```

**Функции:**
- ASCII-art приветствие
- Базовая информация о системе (прямой вызов core.get_status())
- Текущий режим доступа
- Навигационные подсказки

---

### 3. Chat Window (Главный экран)

```
┌─┬────────────────────────────────┐
│🏠│ Chat                           │
│💬│                                │
│⚙️│ User: Покажи статус           │
│📊│                                │
│🔒│ System: Токенов: 1,234        │
│ │         Связей: 5,678          │
│ │                                │
│ │ User: Создай токен             │
│ │                                │
│ │ System: Токен создан (ID: 42)  │
│ │                                │
│ │ ┌────────────────┐ [Send]      │
│ │ │ Message...     │             │
└─┴────────────────────────────────┘
```

**Функции:**
- История сообщений (scrollable)
- Поле ввода сообщения
- Кнопка отправки (или Enter)
- Отправка через `core.process_message(msg)` - прямой вызов!
- Мгновенный ответ (микросекунды)
- Сохранение истории между сеансами (опционально)

**Phase 1 Demo:**
```rust
fn chat_view(core: &CoreBridge) -> Element<'static, Message> {
    let demo_status = core.process_message("status");

    column![
        text("Chat").size(24),
        text("Phase 1 Demo: Direct Core Integration"),
        text("Выполняем: core.process_message(\"status\")"),
        text(demo_status).font(iced::Font::MONOSPACE),
        text("Phase 2: Полный UI чата с вводом"),
    ]
    .into()
}
```

---

### 4. Settings (Управление конфигами)

```
┌─┬────────────────────────────────┐
│🏠│ Settings                       │
│💬│                                │
│⚙️│ Module Configurations:         │
│📊│                                │
│🔒│ ☑ Token Config                 │
│ │ ☑ Connection Config            │
│ │ ☑ Grid Config                  │
│ │ ☑ Graph Config                 │
│ │ ☑ Guardian Config              │
│ │                                │
│ │ ⚠️ CDNA locked (need Admin)    │
│ │                                │
│ │ [Select config to view/edit]   │
└─┴────────────────────────────────┘
```

**Функции (User Mode):**
- Список всех конфигов модулей
- Просмотр конфигов (read-only для некритичных)
- Редактирование разрешённых параметров
- Сохранение изменений (прямой вызов core методов)

**Функции (Admin Mode):**
- Всё из User Mode +
- Редактирование CDNA конфигурации
- Критичные параметры системы

---

### 5. Status (Мониторинг системы)

```
┌─┬────────────────────────────────┐
│🏠│ System Status                  │
│💬│                                │
│⚙️│ ┌──────────────────────────┐  │
│📊│ │ CPU: 45%                 │  │
│🔒│ │ Memory: 2.1 GB / 8 GB    │  │
│ │ │ Uptime: 2h 34m           │  │
│ │ └──────────────────────────┘  │
│ │                                │
│ │ ┌──────────────────────────┐  │
│ │ │ Active Tokens: 12,453    │  │
│ │ │ Connections: 45,231      │  │
│ │ │ Grid Cells: 8,912        │  │
│ │ └──────────────────────────┘  │
└─┴────────────────────────────────┘
```

**Функции:**
- Системные метрики (CPU, RAM, Uptime)
- Статистика токенов/связей (прямой вызов grid.len(), graph.node_count())
- Статус модулей (работают/остановлены)
- Обновление в реальном времени

**Реализация:**
```rust
fn status_view(core: &CoreBridge) -> Element<'static, Message> {
    let status = core.process_message("status");

    column![
        text("System Status").size(24),
        text(status).font(iced::Font::MONOSPACE),
        text("Phase 5: Real-time monitoring"),
    ]
    .into()
}
```

---

### 6. Admin Panel (только Root Mode)

```
┌─┬────────────────────────────────┐
│🏠│ Admin Panel                    │
│💬│                                │
│⚙️│ ⚠️ CRITICAL CHANGES            │
│📊│                                │
│🔒│ CDNA Configuration:            │
│ │                                │
│ │ Profile: [Default ▼]           │
│ │                                │
│ │ Physical Max: [100.0 m]        │
│ │ Emotional Range: [±1.0]        │
│ │ ...                            │
│ │                                │
│ │ [Apply Changes] [Rollback]     │
└─┴────────────────────────────────┘
```

**Функции:**
- Редактирование CDNA параметров
- Выбор профилей (Default, Explorer, Analyst, Creative)
- Валидация перед применением (через Guardian)
- Rollback к предыдущей конфигурации
- Логирование всех изменений

---

### 7. Lock Screen

```
┌────────────────────────────────────┐
│                                    │
│    [ASCII Art Logo]                │
│                                    │
│    🔒 SYSTEM LOCKED                │
│                                    │
│   ┌──────────────────────┐         │
│   │ Password: ●●●●●●     │         │
│   └──────────────────────┘         │
│                                    │
│         [Unlock]                   │
│                                    │
└────────────────────────────────────┘
```

**Функции:**
- Блокировка вручную (кнопка Lock)
- Требует повторный ввод пароля
- Сохраняет текущий режим (User/Admin)

---

## Визуальный стиль

### Тема

**Тёмная тема (по умолчанию):**
- Background: `#1a1a1a` (почти чёрный)
- Text: `#e0e0e0` (светло-серый)
- Primary: `#3399cc` (синий акцент)
- Danger: `#cc3333` (красный для admin/warnings)
- Success: `#33cc66` (зелёный для OK статусов)

### Шрифты

- **Основной:** System UI шрифт (SF Pro/Segoe UI/Ubuntu)
- **Моноширинный:** для ASCII-art, логов, кода
- **Размеры:**
  - Обычный текст: 14px
  - Заголовки: 18-24px
  - ASCII-art: 12px

### ASCII Art Logo

```
+---------------------------------------+
|                                       |
|    *   * ***** *   * ****   ***       |
|    **  * *     *   * *   * *   *      |
|    * * * ***   *   * ****  *   *      |
|    *  ** *     *   * *  *  *   *      |
|    *   * *****  ***  *   *  ***       |
|                                       |
|    ***  ****   ***  ****  *   *       |
|   *     *   * *   * *   * *   *       |
|   * *** ****  ***** ****  *****       |
|   *   * *  *  *   * *     *   *       |
|    ***  *   * *   * *     *   *       |
|                                       |
|               O S                     |
|                                       |
|    ! EXPERIMENTAL COGNITIVE SYSTEM !  |
|                                       |
+---------------------------------------+
```

---

## Технический стек

### Desktop приложение

```toml
[package]
name = "neurograph-desktop"
version = "0.1.0"
edition = "2021"

[dependencies]
# UI Framework
iced = { version = "0.12", features = ["tokio", "advanced"] }

# Direct link to Rust core (NO HTTP!)
neurograph-core = { path = "../core_rust" }

# Password hashing
argon2 = "0.5"
rand = "0.8"

# Config management
serde = { version = "1", features = ["derive"] }
serde_json = "1"
dirs = "5.0"

# Async runtime
tokio = { version = "1", features = ["full"] }
```

**Ключевое отличие:**
- `neurograph-core = { path = "../core_rust" }` - прямая зависимость!
- НЕТ `reqwest` или других HTTP клиентов
- НЕТ `axum` или серверов

---

## Структура проекта

```
neurograph-os/
├── src/
│   ├── core_rust/              # Существующее Rust ядро
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── token/
│   │       ├── connection/
│   │       ├── grid/
│   │       ├── graph/
│   │       ├── guardian/
│   │       └── cdna/
│   │
│   └── desktop/                # Desktop UI
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs         # Entry point
│           ├── app.rs          # Main app state
│           ├── auth.rs         # Auth logic
│           ├── theme.rs        # Visual theme
│           ├── core/
│           │   └── mod.rs      # CoreBridge wrapper
│           └── workspaces/
│               └── mod.rs      # All workspaces
│
└── docs/specs/
    └── DESKTOP_UI_SPEC.md      # Этот документ
```

**НЕТ src/backend/ директории!** - не нужен отдельный HTTP сервер

---

## Roadmap

### Phase 1: Foundation (текущая) - "Auth + Navigation"

**Цель:** Базовая оболочка с аутентификацией

- ✅ Структура проекта (desktop только)
- ✅ iced приложение с базовым окном
- ✅ Login screen с password input
- ✅ Password manager (Argon2id)
- ✅ Первоначальная настройка паролей
- ✅ Dock навигация (иконки)
- ✅ Welcome screen
- ✅ Lock screen
- ✅ Переключение между workspaces
- ✅ CoreBridge integration
- ✅ Demo: прямой вызов core.process_message("status")

**Результат:** Можно войти, переключаться между экранами, видеть статус через прямые вызовы core

---

### Phase 2: Interactive Chat (следующая) - "Dialog"

**Цель:** Работающий диалог с core

- ☐ Chat UI (история + ввод)
- ☐ Message history scrolling
- ☐ Расширение CoreBridge commands
- ☐ Обработка команд (status, create token, help)
- ☐ Real-time updates
- ☐ Error handling

**Результат:** Можно общаться с системой через UI (прямые вызовы, без HTTP)

---

### Phase 3: Configuration (следующая) - "Settings"

**Цель:** Управление конфигами модулей

- ☐ Settings workspace UI
- ☐ Список конфигов (через CoreBridge)
- ☐ Просмотр YAML/JSON
- ☐ Редактирование (некритичные)
- ☐ Сохранение изменений (прямой вызов core)
- ☐ Валидация

**Результат:** Можно настроить модули через UI

---

### Phase 4: Admin Mode (следующая) - "Critical Access"

**Цель:** Безопасное редактирование CDNA

- ☐ Root mode проверка
- ☐ Admin Panel UI
- ☐ CDNA редактор
- ☐ Валидация критичных изменений (через Guardian)
- ☐ Rollback механизм
- ☐ Audit log

**Результат:** Полный контроль над системой через UI

---

### Phase 5: Monitoring (следующая) - "Status"

**Цель:** Мониторинг системы

- ☐ Status workspace UI
- ☐ Системные метрики (через sysinfo)
- ☐ Статистика токенов/связей (прямой доступ к Grid/Graph)
- ☐ Real-time updates
- ☐ Performance metrics

**Результат:** Видимость состояния системы

---

## Примеры интеграции

### Получить статус системы
```rust
// UI code
let status = core.process_message("status");
// Возвращает форматированную строку мгновенно (микросекунды)
```

### Создать токен
```rust
// UI code
let result = core.process_message("создать токен");
// Токен создан и валидирован синхронно
```

### Прямой доступ (будущее)
```rust
// Advanced usage
let grid = core.grid.lock().unwrap();
let token = grid.get(token_id)?;
// Прямой доступ к структурам данных core
```

**Преимущества перед REST:**
- Нет HTTP overhead (микросекунды vs миллисекунды)
- Типобезопасность на этапе компиляции
- Один процесс (проще отладка)
- Общая память (нет сериализации)
- Нативная обработка ошибок Rust

---

## Тестирование

### Build and Run
```bash
cd src/desktop
cargo run --release
# Window opens with lock screen
```

### Default Credentials
**User mode:**
- Username: `user`
- Password: `user123`
- Access: Welcome, Chat, Settings, Status

**Root mode:**
- Username: `root`
- Password: `root123`
- Access: All workspaces including Admin

### Unit Tests
```bash
# Password manager
cargo test -p neurograph-desktop -- auth

# CoreBridge
cargo test -p neurograph-desktop -- core
```

### Integration Tests
```bash
# Full flow test
cargo test --workspace -- --test-threads=1
```

---

## Deployment

### Development
```bash
cd src/desktop
cargo run
```

### Production
```bash
cd src/desktop
cargo build --release
# Binary в target/release/neurograph-desktop
```

### Auto-start (Arch + i3)
```bash
# ~/.config/i3/config
exec --no-startup-id /path/to/neurograph-desktop
```

---

## FAQ

**Q: Почему FFI а не REST API?**
A: FFI быстрее (микросекунды vs миллисекунды), проще (один процесс), типобезопасен, нет сериализации, нет HTTP уязвимостей.

**Q: Почему iced а не egui?**
A: Elm architecture (чистое состояние), декларативный UI, проще делать сложные layouts.

**Q: Сохраняется ли история чата?**
A: В Phase 2 - нет, в памяти только. В Phase 6+ можно добавить persistence.

**Q: Можно ли использовать удалённо?**
A: Нет, это десктоп приложение (один процесс). Для remote access нужен отдельный проект.

**Q: Что если забыл пароль?**
A: Удалить `~/.config/neurograph/auth.json` и пройти first-run setup снова.

**Q: Почему не было REST API с самого начала?**
A: Была ошибка в первоначальном дизайне. FFI_INTEGRATION.md уже показывал, что core_rust готов для прямого использования.

---

## Changelog

### v1.1 (2025-10-28) - FFI Architecture Correction
- ИСПРАВЛЕНО: Архитектура изменена с REST на FFI
- Удалён backend/ директория (не нужен)
- Добавлен CoreBridge wrapper
- Обновлён Cargo.toml (прямая зависимость на core_rust)
- Обновлены примеры кода
- Обновлён roadmap

### v1.0 (2025-10-28) - Initial Specification
- ~~Определена архитектура (REST API)~~ [УСТАРЕЛО]
- Описаны все workspaces
- Определён roadmap (5 фаз)
- Выбран tech stack (iced)
- Определена система безопасности (Argon2id)

---

**NeuroGraph Desktop UI** - минималистичный интерфейс для когнитивной архитектуры

Версия спецификации: 1.1 (FFI)
Дата: 2025-10-28
Статус: В разработке (Phase 1)
