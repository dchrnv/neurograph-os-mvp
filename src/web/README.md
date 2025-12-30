# NeuroGraph Web Dashboard

**Version:** 0.62.0
**Status:** Phase 2 Complete - Dashboard Implementation

## Tech Stack

- **Framework:** React 18.2
- **Language:** TypeScript 5.2
- **UI Kit:** Ant Design Pro 6.x
- **State:** Zustand 4.x
- **Build:** Vite 5.x
- **Charts:** Recharts 2.x
- **Terminal:** xterm.js 5.x
- **i18n:** react-i18next 14.x

## Project Structure

```
src/web/
├── package.json
├── vite.config.ts
├── tsconfig.json
├── index.html
├── public/
├── locales/          # i18n translations
│   ├── ru/
│   └── en/
└── src/
    ├── main.tsx
    ├── App.tsx
    ├── layouts/      # MainLayout with ProLayout
    ├── pages/        # Dashboard, Modules, Config, etc.
    ├── components/   # Reusable components
    ├── stores/       # Zustand stores
    ├── services/     # API & WebSocket
    ├── hooks/        # Custom React hooks
    ├── types/        # TypeScript types
    └── utils/        # Utilities & formatters
```

## Installation

```bash
cd src/web
pnpm install
# or
npm install
```

## Development

### 🚀 Quick Start with Automated Scripts (Recommended)

From project root directory:

```bash
# First time only - install Python dependencies
./setup-dependencies.sh

# Start both frontend and backend together
./start-all.sh

# Or start services individually:
./start-frontend.sh  # Frontend only (port 5173)
./start-backend.sh   # Backend only (port 8000)

# Stop all services
./stop-all.sh
```

**✨ Scripts automatically:**
- Detect Flatpak environment and use `flatpak-spawn --host`
- Create virtual environment if missing
- Kill processes on occupied ports
- Support both tmux and background modes

### Quick Start (Manual - Standard Environment)

```bash
# Frontend only
npm run dev
# or
pnpm dev
```

Opens on http://localhost:5173

API proxy configured to http://localhost:8000/api

### Running in VSCodium Flatpak Environment

If you're running VSCodium in a Flatpak container, Node.js may not be accessible directly. Use `flatpak-spawn --host`:

```bash
# Frontend (from project root)
flatpak-spawn --host /bin/bash -c 'cd /home/chrnv/neurograph-os-mvp/src/web && ./node_modules/.bin/vite'
```

### Full Stack Development (Frontend + Backend)

**1. Start Backend API (port 8000):**

```bash
# From project root
cd /home/chrnv/neurograph-os-mvp

# Standard environment
source .venv/bin/activate
uvicorn src.api.main:app --host 0.0.0.0 --port 8000

# OR in Flatpak environment
flatpak-spawn --host /bin/bash -c 'source .venv/bin/activate && uvicorn src.api.main:app --host 0.0.0.0 --port 8000'
```

**2. Start Frontend (port 5173):**

```bash
# Standard environment
cd src/web
npm run dev

# OR in Flatpak environment
flatpak-spawn --host /bin/bash -c 'cd /home/chrnv/neurograph-os-mvp/src/web && ./node_modules/.bin/vite'
```

**3. Access the application:**
- Frontend: http://localhost:5173
- Backend API: http://localhost:8000
- API Docs: http://localhost:8000/docs
- WebSocket: ws://localhost:8000/ws

### Backend Dependencies Setup (First Time Only)

If running the backend for the first time, install Python dependencies:

```bash
# Standard environment
source .venv/bin/activate
pip install --upgrade pydantic pydantic-settings pydantic-core
pip install fastapi uvicorn[standard] python-multipart python-jose[cryptography] \
    passlib[bcrypt] PyJWT email-validator python-json-logger httpx \
    pytest pytest-asyncio pytest-cov numpy scipy scikit-learn \
    prometheus-client psutil

# OR in Flatpak environment
flatpak-spawn --host /bin/bash -c 'source .venv/bin/activate && \
    pip install --upgrade pydantic pydantic-settings pydantic-core && \
    pip install fastapi uvicorn[standard] python-multipart python-jose[cryptography] \
    passlib[bcrypt] PyJWT email-validator python-json-logger httpx \
    pytest pytest-asyncio pytest-cov numpy scipy scikit-learn \
    prometheus-client psutil'
```

### Troubleshooting

**Port 8000 already in use:**
```bash
# Standard
lsof -ti:8000 | xargs kill -9

# Flatpak
flatpak-spawn --host pkill -f "uvicorn src.api.main"
```

**Node.js not found in Flatpak:**
Use `flatpak-spawn --host` to run commands on the host system instead of inside the container.

**Module not found errors:**
Make sure all dependencies are installed (see Backend Dependencies Setup above).

## Build

```bash
npm run build
# or
pnpm build
```

Output to `dist/` directory

## Implementation Phases

- [x] **Phase 1:** Project Setup (Complete)
  - [x] Vite + React + TypeScript
  - [x] Ant Design Pro
  - [x] Zustand stores (structure)
  - [x] react-i18next setup
  - [x] API & WebSocket services
  - [x] TypeScript types
  - [x] Utils & formatters

- [x] **Phase 2:** Dashboard (Complete)
  - [x] Zustand stores (systemStore, moduleStore, appStore)
  - [x] i18n translations (EN/RU)
  - [x] MainLayout with ProLayout
  - [x] Dashboard page with metrics
  - [x] MetricCard component
  - [x] WebSocket subscriptions
  - [x] Activity table
  - [x] Theme & Language switching

- [ ] **Phase 3:** Modules (1 day)
- [ ] **Phase 4:** Config (1 day)
- [ ] **Phase 5:** Bootstrap (0.5 day)
- [ ] **Phase 6:** Chat (1.5 days)
- [ ] **Phase 7:** Terminal (1 day)
- [ ] **Phase 8:** Admin (0.5 day)
- [ ] **Phase 9:** Polish (1 day)

## Features Implemented

### Phase 2 - Dashboard ✅
- **State Management:** 3 Zustand stores with localStorage persistence
- **Layout:** ProLayout with collapsible sidebar, theme toggle, language selector
- **Dashboard Metrics:**
  - 4 main metric cards (Tokens, Connections, Queries, Events)
  - 6 performance metrics (Latency, Fast Path, Cache Hit, CPU, Memory, Disk)
  - Real-time WebSocket updates
  - Activity log table with event filtering
- **i18n:** Complete EN/RU translations
- **Theme:** Dark/Light mode with CSS variables
- **Routing:** All pages with placeholder components

## Next Steps

Phase 3: Modules page - module management UI

---

**Generated:** 2025-12-30
**Spec:** [WEB_DASHBOARD_v0_62_0_SPEC.md](../../docs/specs/WEB_DASHBOARD_v0_62_0_SPEC.md)
