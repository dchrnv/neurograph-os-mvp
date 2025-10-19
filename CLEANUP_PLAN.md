# План очистки проекта

## Что УДАЛЯЕМ (неиспользуемое в MVP):

### 1. Старые/дублирующиеся компоненты
- [ ] src/core/domain/ (дублирует token, grid, graph)
- [ ] src/core/application/ (memory, orchestration, processing)
- [ ] src/core/experience/ (будет в v0.12)
- [ ] src/core/intuition/ (будет позже)
- [ ] src/infrastructure/api/routes/tokens.py (старая версия)
- [ ] src/infrastructure/api/main.py (старая версия с ошибками)
- [ ] src/infrastructure/websocket/ (будет в v0.12)
- [ ] src/infrastructure/messaging/ (не используется)
- [ ] src/infrastructure/config/ (сложная, не нужна для MVP)
- [ ] src/cli/ (будет позже)

### 2. Ненужные requirements файлы
- [ ] requirements-cli.txt
- [ ] requirements-persistence.txt
- [ ] requirements-websocket.txt
- [ ] requirements/all.txt (и другие в папке requirements/)

### 3. UI/Desktop
- [ ] ui/desktop/ (Electron - не нужен для MVP)

### 4. Старые examples
- [ ] examples/ (если есть устаревшие)

### 5. Старая документация
- [ ] docs/token.md (заменен на token_extended_spec.md)

## Что ОСТАВЛЯЕМ:

✅ src/core/token/token_v2.py + tests
✅ src/core/spatial/ (для будущего)
✅ src/core/graph/ (для v0.11)
✅ src/core/events/ (для v0.11)
✅ src/core/dna/ (для v0.11)
✅ src/core/utils/ (если простые)
✅ src/api_mvp/
✅ src/infrastructure/persistence/ (для v0.11)
✅ ui/web/
✅ config/specs/graph_cdna_rules.json
✅ docs/token_extended_spec.md
✅ docs/configuration_structure.md
✅ requirements.txt
✅ setup.py
✅ README_MVP.md, QUICKSTART.md
✅ run_mvp.sh
