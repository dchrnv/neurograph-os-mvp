# NeuroGraph Desktop UI v0.46.0

Native desktop interface for NeuroGraph OS cognitive architecture.

## Features

- **iced 0.13** - Modern Rust GUI framework
- **Direct FFI** - Microsecond latency to core (no HTTP)
- **Dark + Light themes** - Terminal Modern style
- **Real-time monitoring** - Live metrics, logs, module status
- **Keyboard-first** - Power user hotkeys (Ctrl+1-6, Ctrl+K, etc.)
- **Command Palette** - Quick actions (Ctrl+K/Ctrl+P)

## Screens

1. **Dashboard** - System overview, metrics, quick actions
2. **Chat/Terminal** - Dual-mode interface (conversational + CLI)
3. **Modules** - Manage all system modules
4. **Logs** - Real-time log viewer with filtering
5. **Settings** - Full system configuration
6. **Integrations** - Connect external LLMs, embeddings, tools

## Build & Run

```bash
cd src/desktop_ui
cargo build --release
cargo run
```

## Architecture

```
src/
├── main.rs           - App entry point
├── theme/            - Color palettes, styles
├── screens/          - Main UI screens
│   ├── auth.rs       - PIN authentication
│   ├── dashboard.rs
│   ├── chat.rs
│   └── ...
├── components/       - Reusable UI components
├── core_bridge/      - FFI to neurograph-core
└── utils/            - Helper functions
```

## Development Status

- [x] Phase 1: Foundation (theme, auth, structure)
- [ ] Phase 2: Dashboard
- [ ] Phase 3: Chat/Terminal
- [ ] Phase 4: Modules
- [ ] Phase 5: Logs
- [ ] Phase 6: Settings
- [ ] Phase 7: Integrations
- [ ] Phase 8: Polish

See [DESKTOP_UI_SPEC_V3.md](../../docs/specs/DESKTOP_UI_SPEC_V3.md) for full specification.

## License

Same as NeuroGraph OS - Dual licensing (AGPLv3 + Commercial)
