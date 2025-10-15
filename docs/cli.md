# CLI — NeuroGraph OS

Документация по командной строке (`CLI`) проекта.

Текущее состояние (v0.7):

- CLI находится в стадии разработки (частично реализован).
- Базовый каркас: `src/cli/`.
- Реализована базовая команда `config` в `src/cli/commands/config.py`.

Пример использования команды `config`:

```bash
# Показывает текущую конфигурацию
python -m src.cli.commands.config --show

# Применение конфигурации из файла
python -m src.cli.commands.config --apply path/to/config.yaml
```

Замечание: Точная форма аргументов и API команды `config` зависит от реализации в `src/cli/commands/config.py` — смотрите исходный код для актуальной документации.
