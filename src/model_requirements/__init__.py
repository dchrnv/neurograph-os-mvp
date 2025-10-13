"""
Модуль для управления зависимостями проекта.

Обеспечивает загрузку, валидацию и установку зависимостей
из файлов requirements/*.txt в соответствии с конфигурацией.
"""

__version__ = "0.1.0"

from pathlib import Path
from typing import Dict, List, Optional
from dataclasses import dataclass
import json


@dataclass
class Requirement:
    """Класс для представления зависимости."""
    name: str
    version: str = ""
    extras: List[str] = None
    marker: str = ""

    def __str__(self) -> str:
        result = self.name
        if self.extras:
            result += f"[{','.join(self.extras)}]"
        if self.version:
            result += self.version
        if self.marker:
            result += f" ; {self.marker}"
        return result


class RequirementsManager:
    """Менеджер зависимостей проекта."""
    
    def __init__(self, config_path: Optional[str] = None):
        """Инициализация менеджера зависимостей.
        
        Args:
            config_path: Путь к файлу конфигурации (опционально)
        """
        self.config = self._load_config(config_path)
        self.requirements_dir = Path(self.config["file_structure"]["requirements_dir"])
        self.requirements: Dict[str, List[Requirement]] = {}
    
    def _load_config(self, config_path: Optional[str] = None) -> dict:
        """Загрузка конфигурации.
        
        Args:
            config_path: Путь к файлу конфигурации
            
        Returns:
            dict: Загруженная конфигурация
        """
        if config_path is None:
            # Загрузка конфигурации по умолчанию
            default_config = {
                "file_structure": {
                    "requirements_dir": "requirements/",
                    "files": {
                        "all": "all.txt",
                        "api": "api.txt",
                        "core": "core.txt",
                        "dev": "dev.txt",
                        "docs": "docs.txt",
                        "test": "test.txt",
                        "ui": "ui.txt"
                    },
                    "lock_file": "requirements.lock"
                },
                "version_resolution": {
                    "enabled": True,
                    "strategy": "strictest",
                    "constraint_operators": ["==", "<=", ">=", "<", ">", "!=", "~="],
                    "python_version": "3.9",
                    "allow_prereleases": False
                }
            }
            return default_config
            
        # Загрузка конфигурации из файла
        with open(config_path, 'r', encoding='utf-8') as f:
            return json.load(f)
    
    def load_requirements(self, group: str) -> List[Requirement]:
        """Загрузка зависимостей для указанной группы.
        
        Args:
            group: Имя группы зависимостей (all, api, core, и т.д.)
            
        Returns:
            List[Requirement]: Список зависимостей
            
        Raises:
            ValueError: Если указана несуществующая группа
        """
        if group not in self.config["file_structure"]["files"]:
            raise ValueError(f"Unknown requirements group: {group}")
            
        filename = self.config["file_structure"]["files"][group]
        filepath = self.requirements_dir / filename
        
        if not filepath.exists():
            return []
            
        requirements = []
        with open(filepath, 'r', encoding='utf-8') as f:
            for line in f:
                line = line.strip()
                if not line or line.startswith('#'):
                    continue
                    
                # Простая реализация парсинга зависимостей
                req = Requirement(name=line.split('==')[0])
                if '==' in line:
                    req.version = '==' + line.split('==', 1)[1].split(';')[0].strip()
                requirements.append(req)
                
        self.requirements[group] = requirements
        return requirements
    
    def install_requirements(self, group: str, upgrade: bool = False) -> bool:
        """Установка зависимостей для указанной группы.
        
        Args:
            group: Имя группы зависимостей
            upgrade: Обновлять ли существующие пакеты
            
        Returns:
            bool: True, если установка прошла успешно
        """
        import subprocess
        
        requirements = self.load_requirements(group)
        if not requirements:
            print(f"No requirements found for group: {group}")
            return True
            
        cmd = ["pip", "install"]
        if upgrade:
            cmd.append("--upgrade")
            
        cmd.extend([str(req) for req in requirements])
        
        try:
            subprocess.check_call(cmd)
            return True
        except subprocess.CalledProcessError as e:
            print(f"Failed to install requirements for {group}: {e}")
            return False


def main():
    """Точка входа для командной строки."""
    import argparse
    
    parser = argparse.ArgumentParser(description='Управление зависимостями проекта')
    subparsers = parser.add_subparsers(dest='command', help='Команда')
    
    # Команда install
    install_parser = subparsers.add_parser('install', help='Установить зависимости')
    install_parser.add_argument('group', help='Группа зависимостей для установки')
    install_parser.add_argument('--upgrade', action='store_true', help='Обновить пакеты')
    
    # Команда list
    list_parser = subparsers.add_parser('list', help='Показать доступные группы зависимостей')
    
    args = parser.parse_args()
    
    manager = RequirementsManager()
    
    if args.command == 'install':
        success = manager.install_requirements(args.group, args.upgrade)
        return 0 if success else 1
    elif args.command == 'list':
        print("Available requirement groups:")
        for group in manager.config["file_structure"]["files"]:
            print(f"- {group}")
    else:
        parser.print_help()
        return 1
    
    return 0


if __name__ == "__main__":
    import sys
    sys.exit(main())
