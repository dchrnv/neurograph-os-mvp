# NeuroGraph Web Dashboard

**Version:** 0.62.0
**Status:** Phase 1 Complete - Project Setup

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
```

## Development

```bash
pnpm dev
```

Opens on http://localhost:5173

API proxy configured to http://localhost:8000/api

## Build

```bash
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

- [ ] **Phase 2:** Dashboard (1.5 days)
- [ ] **Phase 3:** Modules (1 day)
- [ ] **Phase 4:** Config (1 day)
- [ ] **Phase 5:** Bootstrap (0.5 day)
- [ ] **Phase 6:** Chat (1.5 days)
- [ ] **Phase 7:** Terminal (1 day)
- [ ] **Phase 8:** Admin (0.5 day)
- [ ] **Phase 9:** Polish (1 day)

## Next Steps

1. Implement Dashboard page with metrics
2. Create MainLayout with ProLayout
3. Add routing for all pages
4. Implement WebSocket subscriptions
5. Add i18n translations

---

**Generated:** 2025-12-30
**Spec:** [WEB_DASHBOARD_v0_62_0_SPEC.md](../../docs/specs/WEB_DASHBOARD_v0_62_0_SPEC.md)
