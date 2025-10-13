"""
Модуль для управления зависимостями проекта.

Обеспечивает:
- Загрузку зависимостей из файла requirements.txt
- Управление версиями пакетов
- Разрешение конфликтов версий
- Валидацию зависимостей
"""

from pathlib import Path
from typing import Dict, List, Optional
import re
import logging

# Настройка логирования
logger = logging.getLogger(__name__)

class RequirementsManager:
    """Менеджер зависимостей проекта.
    
    Упрощенная версия, работающая с единым файлом requirements.txt.
    """
    
    def __init__(self, requirements_file: str = "requirements.txt"):
        """Инициализация менеджера зависимостей.
        
        Args:
            requirements_file: Путь к файлу с зависимостями
        """
        self.requirements_file = Path(requirements_file)
        # Словарь для хранения зависимостей: имя_пакета -> версия
        self.requirements: Dict[str, str] = {}
        
    def load_requirements(self) -> None:
        """Загружает зависимости из файла requirements.txt."""
        if not self.requirements_file.exists():
            logger.warning(f"Файл зависимостей не найден: {self.requirements_file}")
            return
            
        with open(self.requirements_file, 'r', encoding='utf-8') as f:
            for line in f:
                line = line.strip()
                # Пропускаем пустые строки и комментарии
                if not line or line.startswith('#'):
                    continue
                    
                # Обрабатываем строку с зависимостью
                self._process_requirement(line)
    
    def _process_requirement(self, requirement: str) -> None:
        """Обрабатывает строку с зависимостью и добавляет её в словарь.
        
        Args:
            requirement: Строка с зависимостью (например, 'package==1.0.0')
        """
        # Удаляем лишние пробелы и комментарии в конце строки
        requirement = re.sub(r'\s*#.*$', '', requirement).strip()
        if not requirement:
            return
            
        # Разбираем строку с зависимостью
        parts = re.split(r'[=<>!~]=?', requirement, maxsplit=1)
        if not parts:
            return
            
        package = parts[0].strip()
        version = parts[1].strip() if len(parts) > 1 else ''
        self.requirements[package] = version
        
    def get_requirements(self) -> Dict[str, str]:
        """Возвращает словарь с зависимостями.
        
        Returns:
            Словарь вида {'имя_пакета': 'версия'}
        """
        return self.requirements.copy()
    
    def _parse_requirements_file(self, file_path: Path) -> Dict[str, str]:
        """Парсит файл требований и возвращает словарь зависимостей.
        
        Args:
            file_path: Путь к файлу с требованиями
            
        Returns:
            Словарь {имя_пакета: версия}
        """
        requirements = {}
        try:
            with open(file_path, 'r', encoding='utf-8') as f:
                for line in f:
                    line = line.strip()
                    # Пропускаем пустые строки и комментарии
                    if not line or line.startswith('#'):
                        continue
                        
                    # Разделяем имя пакета и версию
                    parts = re.split(r'[=<>~!]=?', line, 1)
                    if not parts:
                        continue
                        
                    pkg = parts[0].strip()
                    version = parts[1].strip() if len(parts) > 1 else ""
                    requirements[pkg] = version
                    
        except Exception as e:
            logger.error(f"Ошибка при разборе файла {file_path}: {e}")
            
        return requirements
    
    def get_requirements(self, groups: Optional[List[str]] = None) -> Dict[str, str]:
        """Возвращает объединенные требования для указанных групп.
        
        Args:
            groups: Список групп для объединения. Если None, возвращаются все группы.
            
        Returns:
            Словарь {имя_пакета: версия}
        """
        if groups is None:
            # Если группы не указаны, используем все доступные
            groups = list(self.groups)
            
        result = {}
        for group in groups:
            if group not in self.requirements:
                logger.warning(f"Неизвестная группа требований: {group}")
                continue
                
            # Объединяем требования из всех указанных групп
            for pkg, version in self.requirements[group].items():
                # При конфликте версий выбираем более строгое ограничение
                if pkg in result and version:
                    if not result[pkg] or self._is_stricter_constraint(version, result[pkg]):
                        result[pkg] = version
                else:
                    result[pkg] = version
                    
        return result
    
    def _is_stricter_constraint(self, new: str, old: str) -> bool:
        """Проверяет, является ли новое ограничение версии более строгим, чем старое.
        
        TODO: Реализовать более точное сравнение версий с учетом семантического версионирования
        """
        return True  # Временная реализация
    
    def generate_requirements_file(self, output_file: str, groups: Optional[List[str]] = None) -> None:
        """Генерирует объединенный файл требований.
        
        Args:
            output_file: Путь к выходному файлу
            groups: Список групп для включения. Если None, включаются все группы.
        """
        requirements = self.get_requirements(groups)
        
        with open(output_file, 'w', encoding='utf-8') as f:
            # Заголовок с информацией о генерации
            f.write("# Сгенерированный файл требований\n")
            f.write(f"# Группы: {', '.join(groups) if groups else 'все'}\n\n")
            
            # Записываем зависимости в алфавитном порядке
            for pkg, version in sorted(requirements.items()):
                if version:
                    f.write(f"{pkg}>={version}\n")
                else:
                    f.write(f"{pkg}\n")
                    
    def check_conflicts(self) -> Dict[str, List[str]]:
        """Проверяет конфликты версий между группами.
        
        Returns:
            Словарь {имя_пакета: [конфликтующие_версии]}
        """
        conflicts = {}
        all_packages = {}
        
        # Собираем все версии каждого пакета из всех групп
        for group, packages in self.requirements.items():
            for pkg, version in packages.items():
                if pkg not in all_packages:
                    all_packages[pkg] = set()
                if version:
                    all_packages[pkg].add(f"{group}: {version}")
        
        # Находим пакеты с конфликтующими версиями
        for pkg, versions in all_packages.items():
            if len(versions) > 1:
                conflicts[pkg] = sorted(versions)
                
        return conflicts

# Создаем глобальный экземпляр менеджера для удобного импорта
manager = RequirementsManager()

# Автоматическая загрузка требований при импорте модуля
try:
    manager.load_requirements()
except Exception as e:
    logger.warning(f"Не удалось загрузить требования: {e}")
    logger.debug("Подробная информация об ошибке:", exc_info=True)
