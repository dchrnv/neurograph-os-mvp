# Руководство по работе с Git

Это краткий, но практичный гайд по использованию Git в проекте Neurograph OS. Ориентирован на ежедневные операции: клонирование репозитория, работа с ветками, лучшие практики для коммитов и pull request'ов, а также советы для разрешения конфликтов.
## Коротко о целях
- хранить историю изменений
- работать изолированно в ветках
- поддерживать читаемую и отзывчивую историю коммитов

## Установка
Установите Git через пакетный менеджер вашей ОС.

Debian/Ubuntu:
```bash
sudo apt update
sudo apt install git

```

Windows: установите Git from https://git-scm.com/downloads или используйте Git for Windows.

## Базовая настройка
```bash
git config --global user.name "Ваше Имя"
git config --global user.email "you@example.com"
git config --global core.autocrlf input    # для Linux/macOS
git config --global pull.rebase false      # по умолчанию использовать merge для pull
```

Рекомендуется настроить SSH-ключ для удобной и безопасной аутентификации с GitHub/GitLab.

## Основные команды (шпаргалка)
- Клонирование:
```bash
git clone git@github.com:dchrnv/neurograph-os-dev.git
```
- Создать ветку и переключиться:
```bash
git checkout -b feature/имя-фичи
```
- Состояние и добавление изменений:
```bash
git status
git add <файл>         # или git add .
git commit -m "Краткое сообщение" -m "Подробное описание при необходимости"
```
- Отправка и получение:
```bash
git push -u origin feature/имя-фичи
git fetch
git pull
```
- Слияние и ребейз:
```bash
git checkout main
git pull
git merge feature/имя-фичи   # или
git rebase main               # сделать историю линейной
```
- Откат коммита:
```bash
git revert <sha>   # безопасно, создает новый коммит-откат
git reset --hard <sha>  # опасно, переписывает историю локально
```

## Рекомендуемый workflow
Мы рекомендуем простой feature-branch workflow:
1. Всегда ветка от `main` (или `develop`, если проект использует develop branch).
2. Название ветки: `feature/xyz`, `fix/bug-123`, `chore/upgrade-deps`.
3. Регулярно подтягивайте изменения из `main` в вашу ветку (`git fetch` + `git rebase origin/main` или `git merge origin/main`).
4. Открывайте Pull Request (PR) для обзора кода.
5. После прохождения ревью — мердж через интерфейс (merge commit или squash, в зависимости от политики).

Пример создания PR:
```bash
# локально
git push -u origin feature/имя-фичи
# затем на GitHub/GitLab создайте PR из feature/имя-фичи в main
```

### Политика мерджа
- Если нужна сохранённая история — используйте "Create a merge commit".
- Для упрощения истории и единичных изменений используйте "Squash and merge".
- В командных задачах согласуйте стиль с командой.

## Стиль коммитов
Короткие правила:
- Первая строка — краткое описание (<= 50 символов).
- Пустая строка.
- Подробное описание при необходимости.

Пример:
```
feat: добавить endpoint для health-check

Добавлен новый эндпоинт /api/v1/health, возвращающий статус сервиса;
покрыт тестом в tests/integration/api/health_test.py
```

Можно использовать Conventional Commits, но не обязательно — главное последовательность и информативность.

## Разрешение конфликтов
1. Выполните `git fetch` и `git rebase origin/main` (или `git merge origin/main`).
2. Если возник конфликт, Git покажет файлы с конфликтами. Откройте их, найдите маркеры `<<<<<<<`, `=======`, `>>>>>>>`.
3. Исправьте конфликт, сохраните файл.
4. Добавьте исправленные файлы: `git add <файл>`.
5. Продолжите rebase: `git rebase --continue` или завершите merge: `git commit`.

Если вы запутались, можно прервать rebase: `git rebase --abort`.

## Полезные команды для отладки истории
- Просмотр логов в компактном виде:
```bash
git log --oneline --graph --decorate --all
```
- Просмотр изменений между ветками:
```bash
git diff origin/main...feature/имя-фичи
```
- Временно спрятать локальные изменения:
```bash
git stash save "WIP: описание"
git stash pop
```

## Советы и лучшие практики
- Не пушьте секреты и ключи — используйте `.gitignore` и секретное хранилище.
- Подпишите важные коммиты GPG, если требуется: `git commit -S`.
- Используйте `git rebase` для чистой линейной истории, но не делайте rebase публичных веток.
- Настройте pre-commit hooks (например, через pre-commit) для автоматической проверки форматирования и линтинга.

## Частые проблемы
- "Detached HEAD": обычно значит, что вы находитесь не на ветке — создайте ветку `git checkout -b feature/x`.
- Ошибки при push: возможно, нужны права или ветка защищена — создайте PR.

## Ресурсы
- Официальная документация Git: https://git-scm.com/docs
- Руководство Pro Git (на русском): https://git-scm.com/book/ru/v2

---
Если нужно, могу расширить этот гайд примерами команд для конкретных сценариев CI/CD, GitHooks или шаблоном PR.
````
