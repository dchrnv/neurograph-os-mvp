# Git Sync Instructions - Linear History

**Цель:** Синхронизировать локальную ветку main с origin/main, сохранив линейную историю

**Текущее состояние:**

```
Local main:
* 1f53a04 v0.13.0 mvp_ConnectionR
* 726f6b6 v0.12.0 mvp_TokenR
* 8c4baf2 major documentation cleanup

Origin main:
* 490b648 Update README.md
* 8c4baf2 major documentation cleanup
```

**Проблема:** Ветки разделились - в origin есть коммит "Update README.md", которого нет локально.

## Решение: Force Push (рекомендуется)

Так как разработка ведётся только тобой, можно безопасно заменить origin/main локальной версией.

### Шаг 1: Force Push с защитой

```bash
git push origin main --force-with-lease
```

**Что это делает:**
- Заменяет origin/main локальной версией
- `--force-with-lease` защищает от случайной перезаписи, если кто-то успел что-то запушить

### Шаг 2: Push тегов

```bash
git push origin v0.12.0
git push origin v0.13.0
```

Или все теги сразу:

```bash
git push origin --tags
```

## Альтернатива: Интерактивный Rebase (если хочешь сохранить коммит из origin)

Если коммит "Update README.md" важен:

```bash
# 1. Fetch последние изменения
git fetch origin

# 2. Rebase локальных коммитов поверх origin/main
git rebase origin/main

# 3. Разрешить конфликты если есть
# (git status покажет файлы с конфликтами)
# Отредактировать конфликтные файлы, затем:
git add <файл>
git rebase --continue

# 4. Push после rebase
git push origin main
git push origin --tags
```

## Рекомендация

**Используй Force Push**, потому что:
- Разработка ведётся только тобой
- Нет риска потерять чужие изменения
- Получится чистая линейная история
- Коммит "Update README.md" скорее всего незначительный

## После успешного Push

Проверь что всё синхронизировано:

```bash
# Обновить информацию о remote
git fetch origin

# Проверить что ветки совпадают
git log origin/main --oneline -3
git log main --oneline -3

# Должны быть одинаковые
```

## Аутентификация GitHub

Если git запросит аутентификацию, есть два варианта:

### Вариант 1: Personal Access Token (рекомендуется)

1. Создай токен на GitHub:
   - Settings → Developer settings → Personal access tokens → Tokens (classic)
   - Generate new token (classic)
   - Выбери срок действия и scope: `repo` (full control)
   - Скопируй токен

2. При push используй токен вместо пароля:
   ```bash
   Username: dchrnv
   Password: <вставь токен>
   ```

### Вариант 2: SSH

Если хочешь использовать SSH вместо HTTPS:

```bash
# Изменить remote на SSH
git remote set-url origin git@github.com:dchrnv/neurograph-os-dev.git

# Проверить
git remote -v
```

Но для этого нужны SSH ключи настроенные на GitHub.

## Команды для копирования

```bash
# Force push (рекомендуется)
git push origin main --force-with-lease
git push origin --tags

# Проверить результат
git fetch origin
git log --oneline --graph --all -5
```

## После Push

Обнови документацию о текущем состоянии:

```bash
# Создать файл с текущим состоянием
cat > GIT_STATUS.md <<'EOF'
# Git Status

**Last sync:** $(date)
**Local branch:** main
**Remote branch:** origin/main
**Status:** ✅ Synchronized

**Latest commits:**
- v0.13.0 mvp_ConnectionR - Connection V1.0
- v0.12.0 mvp_TokenR - Token V2.0

**Tags:**
- v0.13.0
- v0.12.0
- v0.10.0
EOF
```

---

**Готов?** Выполни:
```bash
git push origin main --force-with-lease
git push origin --tags
```
