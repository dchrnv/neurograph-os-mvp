# Настройка GitHub репозитория

Это инструкция для настройки описания, тегов и других параметров GitHub репозитория.

---

## 📝 Описание репозитория (About)

### Short description (краткое описание)
```
Экспериментальная когнитивная архитектура на основе token-based пространственных вычислений с 8 семантическими координатными пространствами
```

или короче:

```
Token-based spatial computing с 8 семантическими координатными пространствами для когнитивных архитектур
```

или на английском:

```
Experimental cognitive architecture based on token-based spatial computing with 8 semantic coordinate spaces
```

### Website (если есть)
```
https://github.com/dchrnv/neurograph-os-dev
```

---

## 🏷️ Topics (теги)

Рекомендуемые теги для лучшего поиска:

### Основные
- `cognitive-architecture`
- `spatial-computing`
- `token-based`
- `semantic-spaces`
- `python`
- `fastapi`
- `react`

### Дополнительные
- `artificial-intelligence`
- `machine-learning`
- `graph-database`
- `multi-dimensional`
- `experimental`
- `research`
- `neuromorphic`
- `typescript`
- `mvp`

### Специфичные для проекта
- `neurograph`
- `token-v2`
- `cdna`
- `experience-stream`

**Максимум 20 тегов** - выбери наиболее релевантные.

---

## 🎯 Рекомендуемый набор (Top 15)

```
cognitive-architecture
spatial-computing
token-based
semantic-spaces
python
fastapi
react
typescript
artificial-intelligence
graph-database
multi-dimensional
experimental
neuromorphic
research
mvp
```

---

## ⚙️ Repository Settings

### General

**Features** (что включить):
- ✅ Issues - для багов и фич
- ✅ Projects - для планирования
- ✅ Discussions - для общения
- ✅ Wiki - для расширенной документации (опционально)

**Pull Requests:**
- ✅ Allow merge commits
- ✅ Allow squash merging
- ✅ Allow rebase merging
- ✅ Automatically delete head branches

### Branches

**Branch protection rule for `main`:**
- ✅ Require pull request reviews before merging (если работаете в команде)
- ✅ Require status checks to pass before merging (когда настроите CI/CD)
- ❌ Require branches to be up to date before merging (для начала)
- ✅ Include administrators (для строгости)

---

## 🚀 Social Preview Image

Создайте изображение 1280x640px с:
- Название: **NeuroGraph OS**
- Подзаголовок: **Token-based Spatial Computing**
- Визуал: Киберпанк стиль (cyan/magenta), сетка, токены
- Логотип: ⚡ или свой

Загрузите в **Settings → Options → Social preview**

---

## 📊 GitHub Actions (для будущего)

Рекомендуемые workflows:

### 1. Python Tests (.github/workflows/test.yml)
```yaml
name: Tests
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-python@v4
        with:
          python-version: '3.10'
      - run: pip install -r requirements.txt
      - run: python -m pytest src/core/token/tests/ -v
```

### 2. Linting (.github/workflows/lint.yml)
```yaml
name: Lint
on: [push, pull_request]
jobs:
  lint:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-python@v4
        with:
          python-version: '3.10'
      - run: pip install black flake8 isort
      - run: black --check src/
      - run: isort --check src/
      - run: flake8 src/
```

---

## 📌 GitHub Projects (опционально)

Создайте Project board:

### Колонки:
1. **Backlog** - планируется
2. **Todo** - готово к разработке
3. **In Progress** - в работе
4. **Review** - на ревью
5. **Done** - готово

### Примеры Issues для начала:
- [ ] Добавить GraphEngine (v0.11)
- [ ] Реализовать CDNA валидатор
- [ ] Интегрировать PostgreSQL
- [ ] Добавить WebSocket поддержку
- [ ] Создать визуализацию графа
- [ ] Написать туториалы

---

## 📜 License Badge

Уже есть в README.md:
```markdown
[![License](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)
```

---

## 🌟 Shields.io Badges

Дополнительные бейджи для README.md:

```markdown
![Python](https://img.shields.io/badge/python-3.10+-blue.svg)
![FastAPI](https://img.shields.io/badge/FastAPI-0.104+-green.svg)
![React](https://img.shields.io/badge/React-18+-61dafb.svg)
![TypeScript](https://img.shields.io/badge/TypeScript-5+-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)
![Status](https://img.shields.io/badge/status-MVP-yellow.svg)
```

---

## 🔗 Полезные ссылки

**После публикации добавьте в README:**
- Issues: `https://github.com/dchrnv/neurograph-os-dev/issues`
- Discussions: `https://github.com/dchrnv/neurograph-os-dev/discussions`
- Projects: `https://github.com/dchrnv/neurograph-os-dev/projects`
- Wiki: `https://github.com/dchrnv/neurograph-os-dev/wiki`

---

## ✅ Checklist перед публикацией

- [x] README.md заполнен
- [x] LICENSE создан (MIT)
- [x] CONTRIBUTING.md создан
- [x] .gitignore настроен
- [x] Код работает и протестирован
- [ ] Описание репозитория добавлено
- [ ] Topics/теги добавлены
- [ ] Social preview image создан (опционально)
- [ ] Issues templates созданы (опционально)
- [ ] GitHub Actions настроены (опционально)

---

## 🎬 Шаги публикации

### 1. Локально
```bash
# Убедитесь что всё закоммичено
git status

# Если есть изменения
git add .
git commit -m "docs: prepare for public release"

# Push в удалённый репозиторий
git push origin main
```

### 2. На GitHub.com

1. Перейдите в Settings → General
2. Прокрутите до **Danger Zone**
3. Если репозиторий приватный → **Change visibility** → **Make public**
4. Подтвердите действие

### 3. Настройте About

1. На главной странице репозитория нажмите ⚙️ рядом с About
2. Добавьте краткое описание
3. Добавьте Website (опционально)
4. Добавьте Topics (теги)
5. Сохраните

### 4. Опубликуйте Release (опционально)

```bash
# Создайте тег
git tag -a v0.10.0 -m "Release v0.10.0 - MVP with Token v2.0"
git push origin v0.10.0
```

На GitHub:
1. Releases → Create a new release
2. Choose tag: v0.10.0
3. Release title: **v0.10.0 - Clean MVP Release**
4. Description:
```markdown
## 🎉 NeuroGraph OS v0.10.0 - MVP

Первый публичный релиз проекта с чистой MVP версией.

### ✨ Особенности
- ✅ Token v2.0 (64 bytes, 8 semantic spaces)
- ✅ RESTful API (FastAPI)
- ✅ React Dashboard (Cyberpunk theme)
- ✅ In-memory storage
- ✅ CDNA validation rules

### 📚 Документация
- [README.md](README.md)
- [QUICKSTART.md](QUICKSTART.md)
- [Token Specification](docs/token_extended_spec.md)
- [Architecture Blueprint](architecture_blueprint.json)

### 🚀 Быстрый старт
```bash
git clone https://github.com/dchrnv/neurograph-os-dev.git
cd neurograph-os-dev
./run_mvp.sh
```

### 🎯 Планы на v0.11
- GraphEngine integration
- CDNA validator
- Basic graph visualization

**Full Changelog**: https://github.com/dchrnv/neurograph-os-dev/commits/v0.10.0
```

---

Готово к публикации! 🚀
