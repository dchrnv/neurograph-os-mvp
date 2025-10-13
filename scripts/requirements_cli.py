#!/usr/bin/env python3
"""
Командный интерфейс для управления зависимостями проекта.

Использование:
  python -m scripts.requirements_cli check
  python -m scripts.requirements_cli generate --output requirements.txt
  python -m scripts.requirements_cli generate --group core --group api --output requirements-prod.txt
"""

import argparse
import sys
from pathlib import Path

# Добавляем корень проекта в PYTHONPATH
sys.path.insert(0, str(Path(__file__).parent.parent))

from src.infrastructure.requirements import manager as req_manager

def check_conflicts():
    """Проверяет конфликты версий между зависимостями."""
    conflicts = req_manager.check_conflicts()
    
    if not conflicts:
        print("\033[92m✓ Конфликты версий не найдены\033[0m")
        return True
    
    print("\033[91m✗ Обнаружены конфликты версий:\033[0m")
    for pkg, versions in conflicts.items():
        print(f"\n\033[93m{pkg}:\033[0m")
        for version in versions:
            print(f"  - {version}")
    
    return False

def generate_requirements(groups, output):
    """Генерирует файл требований из указанных групп."""
    try:
        req_manager.generate_requirements_file(output, groups)
        print(f"\033[92m✓ Файл требований создан: {output}\033[0m")
        return True
    except Exception as e:
        print(f"\033[91m✗ Ошибка при генерации файла требований: {e}\033[0m")
        return False

def list_groups():
    """Выводит список доступных групп зависимостей."""
    print("\n\033[94mДоступные группы зависимостей:\033[0m")
    for group in sorted(req_manager.groups):
        print(f"  - {group}")
    print()

if __name__ == "__main__":
    # Настройка парсера аргументов командной строки
    parser = argparse.ArgumentParser(description="Управление зависимостями проекта")
    subparsers = parser.add_subparsers(dest="command", required=True)
    
    # Команда проверки конфликтов
    check_parser = subparsers.add_parser("check", help="Проверить конфликты версий")
    
    # Команда генерации файла требований
    generate_parser = subparsers.add_parser("generate", help="Сгенерировать файл требований")
    generate_parser.add_argument(
        "-g", "--group", 
        action="append",
        help="Группы зависимостей для включения (можно указать несколько раз)"
    )
    generate_parser.add_argument(
        "-o", "--output",
        default="requirements.txt",
        help="Путь к выходному файлу (по умолчанию: requirements.txt)"
    )
    
    # Команда вывода списка групп
    list_parser = subparsers.add_parser("list", help="Показать доступные группы зависимостей")
    
    # Разбор аргументов командной строки
    args = parser.parse_args()
    
    try:
        if args.command == "check":
            success = check_conflicts()
            sys.exit(0 if success else 1)
            
        elif args.command == "generate":
            success = generate_requirements(args.group, args.output)
            sys.exit(0 if success else 1)
            
        elif args.command == "list":
            list_groups()
            
    except KeyboardInterrupt:
        print("\n\033[93mОперация отменена\033[0m")
        sys.exit(1)
    except Exception as e:
        print(f"\033[91mОшибка: {e}\033[0m")
        sys.exit(1)
