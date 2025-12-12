# Changelog

All notable changes to NeuroGraph OS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.46.0] - 2025-12-12

### 🎨 Major UI Redesign - Terminal Modern Aesthetic

Complete redesign of Desktop UI following the new V3 specification with Terminal Modern aesthetic inspired by Linear, Raycast, GitHub Dark, and Warp Terminal.

### Added

#### 🔐 Authentication
- **PIN-based authentication system** replacing password authentication
  - Visual PIN dots indicator (● ● ● ○ ○ ○)
  - 4-6 digit PIN validation
  - Mock PINs: User (1234), Root (0000)
  - Failed attempt tracking (3 attempts)
  - Redesigned login card with modern styling

#### 📊 Dashboard Screen
- **New Dashboard workspace** as main overview screen
- **6 metric cards**: TOKENS, CONNECTIONS, MEMORY, THROUGHPUT, LATENCY, UPTIME
- **Modules section** with status indicators (● Running/Stopped)
  - Core Engine, API Gateway, WebSocket, Database
- **Quick Actions panel**:
  - 💬 New Chat
  - 📋 View Logs
  - 🔄 Restart All
  - ⚙ Export Config

#### 💬 Chat/Terminal Dual-Mode Interface
- **Tab-based Chat screen** with two modes:
  - **Chat mode**: Traditional message history view
  - **Terminal mode**: Command-line style with prompt
- **Terminal prompt** styling:
  - `user@neurograph $` (blue accent)
  - `root@neurograph #` (yellow warning)
- **Mode badge** showing USER/ROOT status
- Last 10 commands history in Terminal view

#### 📋 Logs Screen
- **System Logs viewer** with filtering capabilities
- **Level filters**: All, Error, Warn, Info, Debug
- **Log entry display**:
  - Timestamp in monospace font
  - Color-coded level badges (semi-transparent)
  - Full message text
- **Footer statistics**: Total, Errors, Warnings count

#### 🎨 Theme System
- **Terminal Modern color palette** (GitHub Dark inspired):
  - Background: `#0d1117`, `#161b22`
  - Text: `#c9d1d9`, `#8b949e`
  - Accent: `#58a6ff` (soft blue)
  - Status colors: Green (#3fb950), Yellow (#d29922), Red (#f85149)
- **Text size constants**: XS (11px) → DISPLAY (32px)
- **Spacing constants**: XS (4px) → XXL (32px)

#### 🧭 Navigation
- **Sidebar navigation** replacing bottom dock
- **New workspaces added**:
  - Dashboard (□)
  - Logs (☰)
  - Integrations (⊕)
- Unicode icons for all workspaces

#### 📚 Documentation
- **QUICKSTART.md** - Comprehensive quick start guide
  - Installation instructions
  - All available commands
  - Development commands (build, test, run)
  - Project structure
  - UI design specifications
  - Debugging tips
  - Development workflow

### Changed

#### 🎨 Visual Updates
- **Cyberpunk neon theme** → **Terminal Modern** (GitHub Dark)
- **Bottom dock** → **Left sidebar** (80px width)
- **Password input** → **PIN input** with visual feedback
- **Bright neon colors** → **Muted professional palette**
- **Card radius**: Consistent 8px for primary cards, 6px for buttons
- **Border styling**: Subtle borders with accent highlights on active states

#### 🏗️ Architecture
- **Layout module** (layout.rs) - New Header and Status Bar components
- **CoreBridge.get_stats()** - Added method for Dashboard metrics
- **ChatMode enum** - Chat/Terminal mode tracking
- **SystemStats struct** - Stats data structure for UI

#### 🔧 Code Organization
- Modular workspace system with clear separation
- Custom style structs for all UI components
- Consistent naming: `*Style`, `*View`, `*_view()`

### Fixed
- Rust lifetime issues in status_bar function
- Format string issues (removed `{:,}` separator)
- Build cache issues with palette crate
- Duplicate type exports causing compilation errors
- Unused variable warnings

### Deprecated
- **Status workspace** - Being replaced by Dashboard
- **CyberColors** - Aliased to TerminalColors for backward compatibility

### Infrastructure
- ✅ All unit tests passing (9 fixes applied)
- ✅ 9.9M tokens stress test successful
- ✅ Clean cargo build with zero errors
- ✅ Desktop UI compiles successfully

---

## [0.45.0] - Previous Version

### Core Functionality
- Token-based spatial computation system
- Grid and Graph data structures
- Guardian validation with CDNA
- Direct Rust core integration (FFI)
- WebSocket support
- Basic desktop UI with iced framework

---

## Roadmap

### Planned for 0.47.0
- [ ] Real-time metrics updates
- [ ] Working Quick Actions buttons
- [ ] Log filtering implementation
- [ ] Integrations screen implementation
- [ ] Settings screen with preferences
- [ ] Module control (start/stop/restart)

### Planned for 0.48.0
- [ ] WebSocket real-time updates
- [ ] API Gateway integration
- [ ] Database persistence
- [ ] User authentication backend
- [ ] System metrics collection

### Planned for 1.0.0
- [ ] Complete CDNA system
- [ ] Production-ready authentication
- [ ] Multi-user support
- [ ] Backup and restore
- [ ] Comprehensive documentation
- [ ] Performance optimizations

---

**Legend:**
- 🎨 UI/UX changes
- 🔐 Security features
- 📊 Data visualization
- 💬 Communication features
- 📋 System monitoring
- 🧭 Navigation improvements
- 🏗️ Architecture changes
- 🔧 Technical improvements
- 📚 Documentation

**Links:**
- [GitHub Repository](https://github.com/your-org/neurograph-os-mvp)
- [Documentation](docs/)
- [Quick Start Guide](QUICKSTART.md)
