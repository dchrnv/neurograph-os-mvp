# NeuroGraph Desktop UI - Specification v2.0

**Version:** v2.0 (Cyberpunk Edition)
**Status:** ✅ Implemented
**Framework:** iced 0.12 (Rust)
**Architecture:** FFI (Direct Core Integration)
**Design:** Киберпанк эстетика с двухрежимной системой

---

## 🎯 Что реализовано в v2.0

Версия 2.0 включает полный редизайн UI в киберпанк-стиле согласно спецификациям:
- [UI_Control_Panel_V2.md](UI_Control_Panel_V2.md)
- [UI_Windows_System_V2.md](UI_Windows_System_V2.md)

---

## 🎨 Визуальная система (Киберпанк)

### Цветовая палитра

Полная палитра реализована в [src/desktop/src/theme.rs](../../src/desktop/src/theme.rs):

```rust
// Фоны - темные градации
BG_PRIMARY:   #0a0a0a  // Глубокий черный
BG_SECONDARY: #141414  // Темно-серый
BG_TERTIARY:  #1a1a1a  // Чуть светлее
BG_HOVER:     #252525  // Состояние наведения

// Текст - от яркого к приглушенному
TEXT_PRIMARY:   #e0e0e0  // Основной
TEXT_SECONDARY: #a0a0a0  // Второстепенный
TEXT_MUTED:     #606060  // Приглушенный

// Акценты - неоновые цвета
ACCENT_PRIMARY: #00ffcc  // Неоново-бирюзовый
ACCENT_BLUE:    #3399ff  // Яркий синий
ACCENT_PURPLE:  #9966ff  // Фиолетовый

// Статусы
STATUS_OK:       #33ff66  // Зеленый неон
STATUS_WARNING:  #ffaa33  // Оранжевый
STATUS_CRITICAL: #ff3366  // Красный неон

// Режимы доступа
MODE_USER: #00ffcc33  // Прозрачный бирюзовый
MODE_ROOT: #ff6600    // Оранжевый (предупреждающий)
```

### Типографика

```rust
// Размеры текста
XS:      11px
SM:      13px
BASE:    14px
LG:      16px
XL:      20px
XXL:     24px
DISPLAY: 32px

// Отступы
XS:  4px
SM:  8px
BASE: 12px
LG:  16px
XL:  24px
XXL: 32px
```

---

## 🏗️ Архитектура UI

### Структура модулей

```
src/desktop/src/
├── main.rs           # Entry point
├── app.rs            # Main application state
├── auth.rs           # Authentication logic
├── theme.rs          # 🆕 Cyberpunk color system
├── metrics.rs        # 🆕 System metrics visualizations
├── core/
│   └── mod.rs        # CoreBridge (FFI wrapper)
└── workspaces/
    └── mod.rs        # All UI screens
```

### Прямая интеграция с core

```rust
// Нет HTTP! Прямые вызовы функций:
let response = core.process_message("status");  // Микросекунды
```

**Преимущества:**
- ⚡ Производительность: микросекунды вместо миллисекунд
- 🔒 Безопасность: нет HTTP поверхности атак
- 🎯 Типобезопасность: проверка на этапе компиляции
- 🐛 Отладка: один процесс

---

## 📱 Workspaces (Экраны)

### Левая панель (Dock) - ASCII иконки

```
┌─┐
│≈│  Home    - Приветствие
│◐│  Chat    - Диалог с системой
│◉│  Metrics - Системные метрики
│⬡│  Modules - Управление модулями
│⚙│  Settings - Настройки
│!│  Admin   - Только для Root
└─┘
```

**Реализация:** [src/desktop/src/workspaces/mod.rs:18-40](../../src/desktop/src/workspaces/mod.rs)

### 1. Home (Welcome Screen)

```
┌────────────────────────────────┐
│                                │
│    [ASCII Art Logo]            │
│    NEUROGRAPH OS               │
│                                │
│    Mode: User/Admin            │
│    System: Running             │
│    Tokens: 1,234               │
│                                │
│    → Click Chat to begin       │
│                                │
│    [Lock]                      │
└────────────────────────────────┘
```

### 2. Chat - Двухрежимная система

#### USER режим (диалоговый)

```
┌──────────────────────────────────────┐
│ NeuroGraph Chat    [Mode: USER]     │ ← Бирюзовая рамка
├──────────────────────────────────────┤
│                                      │
│  User: status                        │
│                                      │
│  System: Tokens: 1,247               │
│          Connections: 523            │
│                                      │
│ ┌──────────────────────┐ [Send]     │
│ │ Message...           │             │
└──────────────────────────────────────┘
```

**Визуальные особенности:**
- Фон: `#0a0a0a` (BG_PRIMARY)
- Рамка: `#00ffcc33` (полупрозрачный бирюзовый)
- Закругленные углы: 8px
- Мягкий, дружелюбный стиль

#### ROOT режим (терминальный)

```
┌──────────────────────────────────────┐
│ NeuroGraph Chat    [Mode: ROOT]     │ ← Оранжевая рамка
├──────────────────────────────────────┤
│ root@neurograph:~$ status --all     │
│                                      │
│ TOKEN_MANAGER:   RUNNING [PID:1247] │
│ CONNECTION_POOL: RUNNING [PID:1248] │
│ GRID_INDEX:      RUNNING [PID:1249] │
│                                      │
│ root@neurograph:~$ _                │
└──────────────────────────────────────┘
```

**Визуальные особенности:**
- Фон: `#000000` (чистый черный)
- Рамка: `#ff6600` (оранжевый предупреждающий)
- Острые углы: 0px
- Терминальный, строгий стиль

**Реализация:** [src/desktop/src/workspaces/mod.rs:108-200](../../src/desktop/src/workspaces/mod.rs)

### 3. Metrics (Status) - Системные метрики

```
┌──────────────────────────────────────┐
│ System Metrics                       │
├──────────────────────────────────────┤
│                                      │
│  CPU Load      Memory      Disk I/O  │
│  [████░░] 42%  [██████] 65%  [██] 25%│
│                                      │
│  Network       Temperature           │
│  [███░░░] 35%  58°C                  │
│                                      │
└──────────────────────────────────────┘
```

**Реализация:** [src/desktop/src/metrics.rs](../../src/desktop/src/metrics.rs)

**Цветовая логика:**
- CPU: 0-30% бирюзовый, 31-60% синий, 61-85% фиолетовый, 86-100% красный
- Temperature: <40°C cyan, 40-65°C синий, 65-80°C фиолетовый, 80-90°C оранжевый, >90°C красный

### 4. Modules - Управление модулями

```
┌──────────────────────────────────────┐
│ Module Manager                       │
├──────────────────────────────────────┤
│                                      │
│ Системные модули:                    │
│                                      │
│ ▶ Token Manager      [RUNNING]      │
│ ▶ Connection Pool    [RUNNING]      │
│ ▶ Grid Index         [RUNNING]      │
│ ▶ Graph Engine       [RUNNING]      │
│ ▶ Guardian           [RUNNING]      │
│                                      │
└──────────────────────────────────────┘
```

**Реализация:** [src/desktop/src/workspaces/mod.rs:209-271](../../src/desktop/src/workspaces/mod.rs)

### 5. Settings - Конфигурация

```
┌──────────────────────────────────────┐
│ Settings                             │
├──────────────────────────────────────┤
│                                      │
│ Module Configurations:               │
│ • Token Config                       │
│ • Connection Config                  │
│ • Grid Config                        │
│ • Graph Config                       │
│ • Guardian Config                    │
│                                      │
│ Phase 3: Config management           │
└──────────────────────────────────────┘
```

### 6. Admin Panel (только Root)

```
┌──────────────────────────────────────┐
│ Admin Panel                          │
│ !!! CRITICAL CHANGES !!!             │
├──────────────────────────────────────┤
│                                      │
│ Phase 4: CDNA configuration          │
│ Direct access to Guardian & CDNA     │
│                                      │
└──────────────────────────────────────┘
```

---

## 🎭 Двухрежимная система (User vs Root)

### Визуальная дифференциация

| Аспект | USER режим | ROOT режим |
|--------|------------|------------|
| **Рамки** | Бирюзовые (#00ffcc33) | Оранжевые (#ff6600) |
| **Фон** | Темно-серый (#0a0a0a) | Черный (#000000) |
| **Углы** | Закругленные (8px) | Острые (0px) |
| **Стиль** | Дружелюбный, мягкий | Терминальный, строгий |
| **Шрифт** | Inter (variable) | JetBrains Mono |
| **Доступ** | Базовый | Расширенный + CDNA |

### Переключение режимов

```rust
// При логине выбирается режим
Message::LoginAttempt(is_root: bool)

// Root требует пароль
if auth.is_admin() {
    // Показываем Admin Panel
    // Разрешаем критичные изменения
}
```

---

## 🔐 Безопасность

### Аутентификация

**Технология:** Argon2id (современный стандарт)

**Хранение:**
```
~/.config/neurograph/auth.json
{
  "user_password_hash": "$argon2id$v=19$m=19456,t=2,p=1$...",
  "root_password_hash": "$argon2id$v=19$m=19456,t=2,p=1$..."
}
```

**Права:** 0600 (только владелец)

### По умолчанию

**User mode:**
- Username: `user`
- Password: `user123`
- Доступ: Welcome, Chat, Metrics, Modules, Settings

**Root mode:**
- Username: `root`
- Password: `root123`
- Доступ: Все + Admin Panel

---

## 🚀 Сборка и запуск

### Разработка

```bash
cd src/desktop
cargo run
```

### Production

```bash
cd src/desktop
cargo build --release
# Binary: target/release/neurograph-desktop
```

### Результат сборки v2.0

```
✅ Finished `dev` profile in 0.24s
⚠️  20 warnings (косметические, не критичные)
```

---

## 📊 Реализованные компоненты

### ✅ Полностью реализовано

1. **Киберпанк цветовая система** - theme.rs
2. **ASCII иконки в Dock** - [≈], [◐], [⚙], [◉], [⬡], [!]
3. **Двухрежимный Chat** - USER (бирюзовый) / ROOT (оранжевый)
4. **Метрики с прогресс-барами** - CPU, RAM, Temperature, Disk, Network
5. **Module Manager** - список модулей со статусами
6. **Кастомные стили** - DockStyle, ChatContainer, ModeBadge

### 🔄 Готово к расширению

1. **Settings** - структура готова, нужны формы редактирования
2. **Admin Panel** - базовый UI, нужна интеграция с Guardian
3. **Command Palette** - не реализовано (Phase 7)
4. **Real-time metrics** - нужна интеграция с `sysinfo` crate
5. **Notifications** - не реализовано (Phase 8)

---

## 🎨 Кастомные стили (iced)

### Dock стили

```rust
// Активная кнопка
ActiveDockButton:
  - background: #1a1a1a
  - border: 2px solid #00ffcc
  - radius: 8px

// Неактивная кнопка
InactiveDockButton:
  - background: #141414
  - border: 1px solid #252525
  - radius: 8px
```

### Chat стили

```rust
// USER контейнер
ChatContainer { is_root: false }:
  - background: #0a0a0a
  - border: 1px solid #00ffcc33
  - radius: 8px

// ROOT контейнер
ChatContainer { is_root: true }:
  - background: #000000
  - border: 1px solid #ff6600
  - radius: 0px
```

### Badge стили

```rust
// USER badge
ModeBadge { is_root: false }:
  - background: #1a1a1a
  - border: 1px solid #00ffcc
  - radius: 12px

// ROOT badge
ModeBadge { is_root: true }:
  - background: rgba(26, 10, 0)
  - border: 1px solid #ff6600
  - radius: 12px
```

---

## 🔧 Технические детали

### Ограничения iced 0.12

**Нет встроенного Canvas API** - поэтому визуализации упрощены:
- ❌ Волновые формы (WaveformVisualizer)
- ❌ 3D кубы (HoloCube)
- ❌ Пульсирующие сферы (ThermalSphere)
- ✅ Progress bars с цветовой индикацией

**Альтернатива:** Можно добавить `iced_graphics` или использовать внешний crate для Canvas, но это добавит сложности.

### Метрики - упрощенная версия

```rust
// Вместо сложных Canvas-визуализаций:
progress_bar(0.0..=1.0, cpu_usage)
    .height(8.0)
    .width(200.0)

// + цветовая логика
match cpu_usage {
    x if x < 0.3 => ACCENT_PRIMARY,  // Бирюзовый
    x if x < 0.6 => ACCENT_BLUE,     // Синий
    x if x < 0.85 => ACCENT_PURPLE,  // Фиолетовый
    _ => STATUS_CRITICAL,            // Красный
}
```

---

## 📋 Roadmap (будущие версии)

### Phase 3: Enhanced Settings ⏳
- Формы редактирования конфигов
- Валидация полей
- Применение изменений через CoreBridge

### Phase 4: Admin CDNA Editor ⏳
- Редактор CDNA параметров
- Выбор профилей (Default, Explorer, Analyst)
- Rollback механизм

### Phase 5: Real-time Monitoring ⏳
- Интеграция `sysinfo` crate
- Живые метрики CPU/RAM
- Авто-обновление каждые 500ms (User) / 100ms (Root)

### Phase 6: Module Control ⏳
- Start/Stop/Restart модулей
- Просмотр логов
- Детальная статистика

### Phase 7: Command Palette 📝
- Быстрые команды (Ctrl+P)
- Fuzzy search
- История команд

### Phase 8: Notifications 📝
- Система алертов
- Типы: Info, Success, Warning, Critical
- Авто-закрытие с таймером

---

## 🔍 Сравнение v1.0 → v2.0

| Аспект | v1.0 | v2.0 |
|--------|------|------|
| **Цветовая схема** | Стандартная темная | Киберпанк (неон) |
| **Иконки** | Текст `[H]`, `[C]` | ASCII `[≈]`, `[◐]` |
| **Режимы** | Единый стиль | User/Root визуальная дифференциация |
| **Метрики** | Только текст | Progress bars + цвета |
| **Модули** | Нет | Module Manager |
| **Стили** | Встроенные | Кастомные (18 struct) |
| **Workspaces** | 5 | 6 (добавлен Modules) |

---

## 📚 Ссылки на код

- **Theme система:** [src/desktop/src/theme.rs](../../src/desktop/src/theme.rs)
- **Метрики:** [src/desktop/src/metrics.rs](../../src/desktop/src/metrics.rs)
- **Workspaces:** [src/desktop/src/workspaces/mod.rs](../../src/desktop/src/workspaces/mod.rs)
- **Главное приложение:** [src/desktop/src/app.rs](../../src/desktop/src/app.rs)
- **CoreBridge:** [src/desktop/src/core/mod.rs](../../src/desktop/src/core/mod.rs)

---

## 📄 Связанные спецификации

- [UI_Control_Panel_V2.md](UI_Control_Panel_V2.md) - детальный дизайн панели управления
- [UI_Windows_System_V2.md](UI_Windows_System_V2.md) - система окон и диалогов
- [DESKTOP_UI_SPEC.md](arch/DESKTOP_UI_SPEC.md) - оригинальная спецификация v1.0

---

**NeuroGraph Desktop UI v2.0** - киберпанк-интерфейс для когнитивной архитектуры

Версия: 2.0 (Cyberpunk Edition)
Дата: 2025-01-XX
Статус: ✅ Implemented & Ready
