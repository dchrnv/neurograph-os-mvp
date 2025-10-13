# src/infrastructure/config/config_manager.py
"""
Менеджер конфигураций - центральная точка доступа
"""
import logging
from typing import Dict, Any, Optional, List, TypeVar, Generic
from pathlib import Path
from dataclasses import dataclass, field
import threading
import weakref
import weakref 
from functools import wraps

from .config_loader import ConfigLoader, LoaderSettings
from .config_types import ConfigPurpose, ConfigResult

logger = logging.getLogger(__name__)

T = TypeVar('T')

class ConfigProxy(Generic[T]):
    """Прокси для ленивой загрузки конфигурации"""
    
    def __init__(self, manager: 'ConfigManager', config_name: str, schema_name: Optional[str] = None):
        self._manager = weakref.ref(manager)
        self._config_name = config_name
        self._schema_name = schema_name
        self._data: Optional[T] = None
        self._loaded = False
    
    @property
    def data(self) -> T:
        """Ленивая загрузка данных"""
        if not self._loaded:
            manager = self._manager()
            if manager:
                result = manager.get(self._config_name, schema_name=self._schema_name)
                self._data = result
                self._loaded = True
        return self._data
    
    def reload(self) -> T:
        """Принудительная перезагрузка"""
        self._loaded = False
        return self.data
    
    def __getattr__(self, name):
        """Проксирование атрибутов к данным"""
        return getattr(self.data, name)
    
    def __getitem__(self, key):
        """Проксирование индексации к данным"""
        return self.data[key]

@dataclass
class ConfigGroup:
    """Группа связанных конфигураций"""
    name: str
    configs: Dict[str, str] = field(default_factory=dict)  # name -> path mapping
    specs: Dict[str, str] = field(default_factory=dict)    # name -> spec path mapping
    schemas: Dict[str, str] = field(default_factory=dict)  # name -> schema name mapping

class ConfigManager:
    """
    Синглтон менеджер конфигураций
    Обеспечивает центральный доступ ко всем конфигам
    """
    
    _instance = None
    _lock = threading.Lock()
    
    def __new__(cls):
        if not cls._instance:
            with cls._lock:
                if not cls._instance:
                    cls._instance = super().__new__(cls)
        return cls._instance
    
    def __init__(self):
        if hasattr(self, '_initialized'):
            return
        
        self._initialized = True
        self.loader = ConfigLoader()
        self._configs: Dict[str, Any] = {}
        self._groups: Dict[str, ConfigGroup] = {}
        self._proxies: Dict[str, ConfigProxy] = {}
        
        # Регистрация стандартных групп
        self._register_default_groups()
        
        # Автозагрузка базовых конфигов
        self._autoload_configs()
    
    def update_environment(self, environment: str):
        """Обновляет окружение и пересоздает загрузчик."""
        if self.loader.settings.environment != environment:
            logger.info(f"Switching environment from '{self.loader.settings.environment}' to '{environment}'")
            settings = self.loader.settings
            settings.environment = environment
            self.loader = ConfigLoader(settings)
            self.clear_all()

    def _register_default_groups(self):
        """Регистрация стандартных групп конфигураций"""
        # Core группа
        core_group = ConfigGroup(
            name="core",
            configs={
                "token": "token",
                "grid": "grid", 
                "graph": "graph",
                "dna": "dna"
            },
            specs={
                "token": "token",
                "grid": "grid",
                "graph": "graph"
            },
            schemas={
                "token": "token",
                "grid": "grid",
                "graph": "graph"
            }
        )
        self._groups["core"] = core_group
        
        # Infrastructure группа
        infra_group = ConfigGroup(
            name="infrastructure",
            configs={
                "persistence": "persistence",
                "api": "api",
                "cache": "cache",
                "messaging": "messaging"
            }
        )
        self._groups["infrastructure"] = infra_group
        
        # Processing группа  
        processing_group = ConfigGroup(
            name="processing",
            configs={
                "processor": "processor",
                "pipeline": "pipeline",
                "workers": "workers"
            },
            schemas={
                "processor": "processor"
            }
        )
        self._groups["processing"] = processing_group
    
    def _autoload_configs(self):
        """Автоматическая загрузка критичных конфигов"""
        critical_configs = ["system", "logging"]
        
        for config_name in critical_configs:
            result = self.loader.load_config(config_name)
            if result.success:
                self._configs[config_name] = result.data
                logger.info(f"Autoloaded config: {config_name}")
    
    def get(
        self,
        config_name: str,
        group: Optional[str] = None,
        schema_name: Optional[str] = None,
        default: Any = None,
        force_reload: bool = False
    ) -> Any:
        """
        Получение конфигурации
        
        Args:
            config_name: Имя конфигурации
            group: Имя группы (для группированных конфигов)
            schema_name: Имя схемы для валидации
            default: Значение по умолчанию
            force_reload: Принудительная перезагрузка
        
        Returns:
            Данные конфигурации или default
        """
        cache_key = f"{group}:{config_name}" if group else config_name
        
        # Проверка кэша
        if not force_reload and cache_key in self._configs:
            return self._configs[cache_key]
        
        # Поиск в группе
        if group and group in self._groups:
            group_obj = self._groups[group]
            if config_name in group_obj.configs:
                config_path = group_obj.configs[config_name]
                schema = schema_name or group_obj.schemas.get(config_name)
                result = self.loader.load_config(config_path, schema)
            elif config_name in group_obj.specs:
                spec_path = group_obj.specs[config_name]
                result = self.loader.load_spec(spec_path)
            else: # Если в группе не найдено, ищем как обычный конфиг
                result = self.loader.load_config(config_name, schema_name)
        else:
            # Обычная загрузка
            result = self.loader.load_config(config_name, schema_name)
            
        if result.success:
            self._configs[cache_key] = result.data
            return result.data
        
        logger.warning(f"Failed to load config '{cache_key}': {result.error}")
        return default

    def get_spec(self, spec_name: str, default: Any = None, force_reload: bool = False) -> Any:
        """Получение машинной спецификации (JSON)"""
        cache_key = f"spec:{spec_name}"
        
        if not force_reload and cache_key in self._configs:
            return self._configs[cache_key]
        
        result = self.loader.load_spec(spec_name)
        if result.success:
            self._configs[cache_key] = result.data
            return result.data
        
        logger.warning(f"Failed to load spec '{spec_name}': {result.error}")
        return default

    def reload(self):
        """Перезагрузка всех известных конфигураций"""
        self.loader.reload_all()
        self.clear_all()
        self._autoload_configs()

    def clear_all(self):
        """Очистка всех внутренних кэшей"""
        self._configs.clear()
        self.loader.clear_cache()

    def clear_all(self):
        """Очистка всех внутренних кэшей"""
        self._configs.clear()
        self.loader.clear_cache()