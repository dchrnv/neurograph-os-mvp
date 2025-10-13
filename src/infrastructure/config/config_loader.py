import os
import re
from pathlib import Path
from typing import Any, Dict, List, MutableMapping, Optional

import yaml
from dotenv import load_dotenv


class ConfigError(Exception):
    """Базовый класс для ошибок конфигурации."""


class ConfigNotFoundError(ConfigError):
    """Исключение, когда файл конфигурации не найден."""


class ConfigParseError(ConfigError):
    """Исключение при ошибке парсинга файла конфигурации."""


class ConfigLoader:
    """
    Загружает и объединяет конфигурационные файлы YAML в соответствии со средой.
    Поддерживает подстановку переменных окружения.
    """
    ENV_VAR_MATCHER = re.compile(r"\${([A-Z0-9_]+)(?::-(.*?))?}")

    def __init__(self, base_path: str, env: str):
        self.base_path = Path(base_path)
        self.env = env
        load_dotenv()  # Загружаем переменные из .env файла

    def load_and_merge(self) -> Dict[str, Any]:
        """
        Основной метод для загрузки, объединения и обработки конфигураций.
        """
        base_configs = self._load_base_configs()
        env_config = self._load_env_specific_config()

        merged_config = self._deep_merge(base_configs, env_config)
        final_config = self._substitute_env_vars(merged_config)

        return final_config

    def _load_base_configs(self) -> Dict[str, Any]:
        """Загружает и объединяет все базовые конфигурационные файлы."""
        base_config = {}
        config_dirs = ["core", "application", "infrastructure", "interfaces"]
        for directory in config_dirs:
            dir_path = self.base_path / directory
            if not dir_path.is_dir():
                continue
            for file_path in dir_path.glob("*.yaml"):
                config_part = self._load_yaml_file(file_path)
                base_config = self._deep_merge(base_config, config_part)
        return base_config

    def _load_env_specific_config(self) -> Dict[str, Any]:
        """Загружает конфигурацию для конкретной среды."""
        env_file_path = self.base_path / "environments" / f"{self.env}.yaml"
        if not env_file_path.exists():
            # Это не ошибка, окружение может не иметь специфичных настроек
            return {}
        return self._load_yaml_file(env_file_path)

    def _load_yaml_file(self, file_path: Path) -> Dict[str, Any]:
        """Загружает и парсит один YAML файл."""
        try:
            with file_path.open("r", encoding="utf-8") as f:
                data = yaml.safe_load(f)
                return data if data else {}
        except yaml.YAMLError as e:
            raise ConfigParseError(f"Ошибка парсинга YAML файла: {file_path}") from e
        except IOError as e:
            raise ConfigNotFoundError(f"Не удалось прочитать файл: {file_path}") from e

    def _deep_merge(self, source: Dict, destination: Dict) -> Dict:
        """Рекурсивно объединяет два словаря."""
        for key, value in source.items():
            if isinstance(value, dict):
                node = destination.setdefault(key, {})
                self._deep_merge(value, node)
            else:
                destination[key] = value
        return destination

    def _substitute_env_vars(self, config_part: Any) -> Any:
        """Рекурсивно подставляет переменные окружения в конфигурацию."""
        if isinstance(config_part, dict):
            return {k: self._substitute_env_vars(v) for k, v in config_part.items()}
        if isinstance(config_part, list):
            return [self._substitute_env_vars(i) for i in config_part]
        if isinstance(config_part, str):
            return self.ENV_VAR_MATCHER.sub(self._replacer, config_part)
        return config_part

    @staticmethod
    def _replacer(match: re.Match) -> str:
        """Заменяет найденное ${VAR} на значение из окружения."""
        var_name, default_value = match.groups()
        value = os.getenv(var_name)
        if value is not None:
            return value
        if default_value is not None:
            return default_value
        raise ConfigError(
            f"Обязательная переменная окружения '{var_name}' не установлена."
        )