#!/usr/bin/env python3
"""
Скрипт для генерации файла requirements.txt

Использование:
    python scripts/generate_requirements.py [--check]

Опции:
    --check  Проверить, актуален ли requirements.txt
"""

import sys
from pathlib import Path
from typing import Set, Optional

def read_requirements(path: Path) -> Set[str]:
    """Читает зависимости из файла."""
    requirements = set()
    if not path.exists():
        return requirements
        
    with open(path, 'r', encoding='utf-8') as f:
        for line in f:
            line = line.strip()
            # Пропускаем пустые строки и комментарии
            if line and not line.startswith('#'):
                requirements.add(line)
    return requirements

def generate_requirements() -> bool:
    """Генерирует файл requirements.txt.
    
    Returns:
        bool: True если файл был обновлен, иначе False
    """
    base_dir = Path(__file__).parent.parent
    requirements_file = base_dir / "requirements.txt"
    
    print("Чтение зависимостей...")
    current_requirements = read_requirements(requirements_file)
    
    # Здесь можно добавить автоматическое определение зависимостей
    # из pyproject.toml или setup.py, если нужно
    
    # Сортируем для читаемости и стабильности
    sorted_requirements = sorted(current_requirements)
    
    # Формируем новое содержимое
    new_content = (
        "# Этот файл сгенерирован автоматически.\n"
        "# Для обновления зависимостей используйте команду:\n"
        "#   python scripts/generate_requirements.py\n\n"
    )
    
    # Группируем зависимости по категориям
    categories = {
        "Основные зависимости": [],
        "API": [],
        "Асинхронность": [],
        "Логирование": [],
        "Тестирование": [],
        "Документация": [],
        "Разработка": [],
        "Безопасность": [],
        "Утилиты": []
    }
    
    # Распределяем зависимости по категориям
    for req in sorted_requirements:
        req_lower = req.lower()
        if any(pkg in req_lower for pkg in ["fastapi", "uvicorn", "pydantic"]):
            categories["API"].append(req)
        elif any(pkg in req_lower for pkg in ["pytest", "pytest-"]):
            categories["Тестирование"].append(req)
        elif any(pkg in req_lower for pkg in ["mkdocs", "mkdoc"]):
            categories["Документация"].append(req)
        elif any(pkg in req_lower for pkg in ["black", "isort", "flake8", "mypy", "pre-commit"]):
            categories["Разработка"].append(req)
        elif any(pkg in req_lower for pkg in ["bandit", "safety"]):
            categories["Безопасность"].append(req)
        elif any(pkg in req_lower for pkg in ["numpy", "pandas", "requests"]):
            categories["Основные зависимости"].append(req)
        elif any(pkg in req_lower for pkg in ["loguru"]):
            categories["Логирование"].append(req)
        elif any(pkg in req_lower for pkg in ["anyio", "asyncio"]):
            categories["Асинхронность"].append(req)
        else:
            categories["Утилиты"].append(req)
    
    # Формируем содержимое с категориями
    for category, deps in categories.items():
        if deps:
            new_content += f"\n# {category}\n"
            new_content += "\n".join(sorted(deps)) + "\n"
    
    # Проверяем, изменилось ли содержимое
    current_content = ""
    if requirements_file.exists():
        with open(requirements_file, 'r', encoding='utf-8') as f:
            current_content = f.read()
    
    if current_content.strip() == new_content.strip():
        print("Файл requirements.txt актуален.")
        return False
    
    # Записываем новый файл
    with open(requirements_file, 'w', encoding='utf-8') as f:
        f.write(new_content)
    
    print(f"Файл {requirements_file} успешно обновлен.")
    return True

def main():
    check_mode = "--check" in sys.argv
    
    if generate_requirements() and check_mode:
        print("Ошибка: файл requirements.txt не актуален.")
        print("Запустите 'python scripts/generate_requirements.py' для обновления.")
        sys.exit(1)

if __name__ == "__main__":
    main()