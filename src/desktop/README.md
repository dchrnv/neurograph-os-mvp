# NeuroGraph Desktop UI

**Phase 1: Auth + Navigation + Core Integration** - Минимальный прототип

## Архитектура

**FFI (Foreign Function Interface)** - прямая интеграция с Rust core:

```bash
Desktop UI (iced)
       ↓
  CoreBridge
       ↓
neurograph-core (прямые вызовы функций)
```

**НЕТ HTTP/REST API** - один процесс, нативная производительность.

## Quick Start

```bash
# Из корня проекта
cd src/desktop

# Запуск
cargo run
```

## Demo Credentials

- **User Mode**: password = `demo`
- **Admin Mode**: password = `root`

## Что работает (Phase 1)

- ✅ Login screen с ASCII-art логотипом
- ✅ User/Root mode выбор
- ✅ Mock authentication (demo/root)
- ✅ Unity-style dock слева
- ✅ Переключение между workspaces
- ✅ Welcome/Chat/Settings/Status/Admin экраны
- ✅ Lock button
- ✅ **CoreBridge - прямая интеграция с neurograph-core**
- ✅ **Demo: core.process_message("status") в Chat workspace**

## Что НЕ работает (ждёт Phase 2+)

- ❌ Real password hashing (Argon2id)
- ❌ Full chat UI с историей и вводом
- ❌ Settings editing
- ❌ Admin CDNA editing
- ❌ Real-time status updates

## Структура

```rust
desktop/
├── src/
│   ├── main.rs         # Entry point
│   ├── app.rs          # Main app state + CoreBridge
│   ├── auth.rs         # Auth module
│   ├── core/
│   │   └── mod.rs      # CoreBridge wrapper
│   ├── workspaces/
│   │   └── mod.rs      # All UI screens
│   ├── theme.rs        # Theme (Dark)
│   └── assets/
│       └── logo.txt    # ASCII art
└── Cargo.toml          # Dependencies (iced + neurograph-core)
```

## CoreBridge Example

```rust
// src/desktop/src/core/mod.rs
use neurograph_core::{Token, Grid, Graph, Guardian, CDNA};

pub struct CoreBridge {
    grid: Arc<Mutex<Grid>>,
    graph: Arc<Mutex<Graph>>,
    guardian: Arc<Mutex<Guardian>>,
}

impl CoreBridge {
    pub fn process_message(&self, msg: &str) -> String {
        match msg.to_lowercase().as_str() {
            "статус" | "status" => self.get_status(),
            "создать токен" | "create token" => self.create_token(),
            _ => format!("Команда не распознана"),
        }
    }
}
```

Использование в UI:

```rust
// src/desktop/src/workspaces/mod.rs
fn chat_view(core: &CoreBridge) -> Element<'static, Message> {
    let demo_status = core.process_message("status");
    // Прямой вызов - микросекунды, не миллисекунды!
}
```

## Next Steps

- [ ] Phase 2: Full chat UI (история + ввод)
- [ ] Phase 2: Расширение CoreBridge commands
- [ ] Phase 3: Config management через CoreBridge
- [ ] Phase 4: Admin panel (CDNA editing через Guardian)
- [ ] Phase 5: Real-time status monitoring

## Технологии

- **iced 0.12** - Rust GUI framework (Elm architecture)
- **neurograph-core** - Direct dependency на core_rust
- **Argon2id** - Password hashing (Phase 2+)
- **Arc<Mutex<T>>>** - Thread-safe shared state

## Преимущества FFI

- Микросекунды вместо миллисекунд (vs REST)
- Типобезопасность на этапе компиляции
- Один процесс (проще отладка)
- Нет JSON сериализации
- Нет HTTP уязвимостей

---

See [DESKTOP_UI_SPEC.md](../../docs/specs/DESKTOP_UI_SPEC.md) for full specification.
