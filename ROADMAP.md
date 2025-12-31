# NeuroGraph OS Roadmap

**Актуальный план развития проекта**

> 📘 **Детальный план:** См. [docs/DEVELOPMENT_PLAN.md](docs/DEVELOPMENT_PLAN.md)

---

## 🎯 Текущий фокус: Стабилизация

### v0.63.2 (In Progress) - Stabilization Release
**ETA:** 3-5 дней | **Приоритет:** 🔴 КРИТИЧЕСКИЙ

- [ ] Завершить Module Registry
- [ ] Синхронизировать документацию с кодом
- [ ] Базовые тесты (40% coverage)
- [ ] CI/CD setup (GitHub Actions)

---

## 📅 Ближайшие релизы

### v0.64.0 - Python Package & Jupyter
**ETA:** 7-10 дней | **Приоритет:** 🟡 ВЫСОКИЙ

- Python package для pip install
- Jupyter magic commands
- Tutorial notebooks
- CLI tool improvements

### v0.65.0 - Production Readiness
**ETA:** 5-7 дней | **Приоритет:** 🟢 СРЕДНИЙ

- 70%+ test coverage
- Monitoring dashboards (Grafana)
- Kubernetes deployment
- Production guide

---

## 📊 Прогресс по трекам

**ТРЕК A: Core Intelligence** ✅ COMPLETE (v0.57.0)
- SignalSystem, Gateway, ActionController
- Performance: 304K events/sec

**ТРЕК B: Developer Experience** ⚠️ IN PROGRESS
- ✅ WebSocket (v0.60.0) - DONE
- 🚧 Python Package (v0.64.0) - Next
- 🚧 Jupyter Integration (v0.64.0) - Next

**ТРЕК C: User Interfaces** ✅ COMPLETE (v0.62.0)
- Web Dashboard (React SPA)
- Real-time updates, i18n, themes

**ТРЕК D: Module Management** ⚠️ IN PROGRESS (v0.63.0-v0.63.2)
- Module Registry API
- Enable/disable functionality

---

## 🗓️ Timeline

```
┌─────────────┬─────────────┬─────────────┬─────────────┐
│   Неделя 1  │  Неделя 2   │  Неделя 3   │  Неделя 4   │
├─────────────┼─────────────┼─────────────┼─────────────┤
│   v0.63.2   │   v0.64.0   │   v0.64.0   │   v0.65.0   │
│Stabilization│  Python Pkg │   Jupyter   │ Production  │
│             │             │             │   Ready     │
└─────────────┴─────────────┴─────────────┴─────────────┘
```

---

## 🚀 После v0.65.0 (опционально)

- **v0.66.0** - Enhanced Sensors (Audio & Vision)
- **v0.67.0** - Advanced Analytics & ML
- **v0.68.0** - Multi-node clustering
- **v0.69.0** - GraphQL API

---

**Последнее обновление:** 2024-12-31
**Детали:** [docs/DEVELOPMENT_PLAN.md](docs/DEVELOPMENT_PLAN.md)
