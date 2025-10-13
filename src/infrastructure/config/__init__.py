# src/infrastructure/config/__init__.py
"""
Система управления конфигурациями NeurographOS
Поддержка YAML (для людей) и JSON (для машин)
"""
import logging
from typing import Any, Optional, Dict
import signal
import sys
from pathlib import Path

# Настройка логирования для модуля
logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# Импорт основных компонентов
from .config_types import (
    ConfigFormat,
    ConfigPurpose,
    ConfigMetadata,
    ConfigSchema,
    ConfigEntry,
    ConfigResult,
    # Схемы
    TokenConfigSchema,
    GridConfigSchema, 
    GraphConfigSchema,
    ProcessorConfigSchema,
    SCHEMA_REGISTRY
)

from .config_loader import (
    ConfigLoader,
    LoaderSettings
)

from .config_manager import (
    ConfigManager,
    ConfigProxy,
    ConfigGroup
)

# Версия модуля
__version__ = "1.0.0"

# Публичный API
__all__ = [
    # Глобальный экземпляр
    "config_manager",
    
    # Декоратор и хелперы
    "with_config", "get_config", "get_spec", "reload_config",
    
    # Классы
    "ConfigManager",
    "ConfigLoader", 
    "LoaderSettings",
    "ConfigProxy",
    "ConfigGroup",
    
    # Типы
    "ConfigFormat",
    "ConfigPurpose",
    "ConfigMetadata",
    "ConfigSchema",
    "ConfigEntry",
    "ConfigResult",
    
    # Схемы
    "TokenConfigSchema",
    "GridConfigSchema",
    "GraphConfigSchema", 
    "ProcessorConfigSchema",
    "SCHEMA_REGISTRY",

    # Утилиты
    "setup_hot_reload",
    "initialize"
]

config_manager = ConfigManager()

def get_config(config_name: str, group: Optional[str] = None, default: Any = None) -> Any:
    """Хелпер для получения конфигурации"""
    return config_manager.get(config_name, group=group, default=default)

def get_spec(spec_name: str, default: Any = None) -> Any:
    """Хелпер для получения спецификации"""
    return config_manager.get_spec(spec_name, default=default)

def reload_config(config_name: str, group: Optional[str] = None):
    """Хелпер для перезагрузки конкретного конфига"""
    config_manager.get(config_name, group=group, force_reload=True)

def with_config(config_name: str, group: Optional[str] = None, arg_name: Optional[str] = None):
    """
    Декоратор для внедрения конфигурации в функцию как аргумент.
    Пример: @with_config("token")
    """
    from functools import wraps
    def decorator(func):
        @wraps(func)
        def wrapper(*args, **kwargs):
            config_arg_name = arg_name or config_name
            if config_arg_name not in kwargs:
                kwargs[config_arg_name] = get_config(config_name, group=group)
            return func(*args, **kwargs)
        return wrapper
    return decorator

def setup_hot_reload():
    """
    Настройка горячей перезагрузки конфигов по сигналу SIGUSR1 (Unix)
    или через файл-триггер (кроссплатформенно)
    """
    def reload_handler(signum=None, frame=None):
        logger.info("Received reload signal, reloading configs...")
        config_manager.reload()
        logger.info("Configs reloaded successfully")
    
    # Unix-like системы
    if hasattr(signal, 'SIGUSR1'):
        signal.signal(signal.SIGUSR1, reload_handler)
        logger.info("Hot reload enabled via SIGUSR1 signal")
    
    # Кроссплатформенный вариант через файл-триггер
    trigger_file = Path(".reload_configs")
    
    def check_trigger_file():
        """Проверка файла-триггера для перезагрузки"""
        if trigger_file.exists():
            trigger_file.unlink()  # Удаляем файл
            reload_handler()
    
    # Можно запустить в отдельном потоке для периодической проверки
    import threading
    import time
    
    def trigger_watcher():
        while True:
            check_trigger_file()
            time.sleep(5)  # Проверка каждые 5 секунд
    
    watcher = threading.Thread(target=trigger_watcher, daemon=True)
    watcher.start()
    logger.info(f"Hot reload enabled via trigger file: {trigger_file}")

def initialize(
    configs_dir: Optional[str] = None,
    specs_dir: Optional[str] = None,
    environment: Optional[str] = None,
    enable_hot_reload: bool = False
) -> ConfigManager:
    """
    Инициализация системы конфигураций
    
    Args:
        configs_dir: Путь к директории с YAML конфигами
        specs_dir: Путь к директории с JSON спецификациями
        environment: Окружение (development, staging, production)
        enable_hot_reload: Включить горячую перезагрузку
    
    Returns:
        Инициализированный ConfigManager
    """
    # Обновление настроек если указаны
    if configs_dir:
        config_manager.loader.configs_dir = Path(configs_dir)
    
    if specs_dir:
        config_manager.loader.specs_dir = Path(specs_dir)
    
    if environment:
        config_manager.update_environment(environment)
    
    # Включение горячей перезагрузки
    if enable_hot_reload:
        setup_hot_reload()
    
    logger.info(
        f"Config system initialized: "
        f"environment={config_manager.loader.environment}, "
        f"configs_dir={config_manager.loader.configs_dir or config_manager.loader.base_path}, "
        f"specs_dir={config_manager.loader.specs_dir}"
    )
    
    return config_manager

# Автоматическая инициализация при импорте
# (можно отключить если нужна кастомная инициализация)
if not hasattr(config_manager, '_auto_initialized'):
    # Определяем окружение из переменной среды
    import os
    from pathlib import Path
    env = os.getenv('APP_ENV', 'development')
    configs_dir = Path(__file__).parent.parent.parent.parent / 'config'
    specs_dir = configs_dir / 'specs'
    initialize(
        configs_dir=str(configs_dir),
        specs_dir=str(specs_dir),
        environment=env
    )
    config_manager._auto_initialized = True