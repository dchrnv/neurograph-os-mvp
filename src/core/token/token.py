import struct
import time
import uuid
from dataclasses import dataclass, field
from typing import Optional, Tuple, Dict, Any, List, Set, ClassVar

@dataclass
class Token:
    """
    Узел графа, представляющий токен с поддержкой пространственных координат.
    
    Атрибуты:
        content: Содержимое токена (текст, данные)
        token_id: Уникальный идентификатор токена
        token_type: Тип токена (например, 'word', 'concept', 'entity')
        weight: Вес/важность токена
        metadata: Дополнительные метаданные
        created_at: Временная метка создания
        last_updated: Временная метка последнего обновления
        coordinates: 8 уровней 3D координат
        flags: Флаги токена
        _genetic_marker: Генетический маркер для эволюционных алгоритмов
    """
    
    # Бинарный формат для сериализации
    BINARY_FORMAT: ClassVar[str] = '<24h I H f H I'
    BINARY_SIZE: ClassVar[int] = struct.calcsize(BINARY_FORMAT)
    ABSENT_VALUE: ClassVar[int] = 127
    
    # Основные поля
    content: str = ""
    token_id: int = field(default_factory=lambda: int(uuid.uuid4().int & (1 << 64)-1))
    token_type: str = "default"
    weight: float = 1.0
    metadata: Dict[str, Any] = field(default_factory=dict)
    created_at: float = field(default_factory=time.time)
    last_updated: float = field(default_factory=time.time)
    
    # Унаследованные поля для совместимости
    coordinates: List[List[int]] = field(default_factory=lambda: [[127] * 3 for _ in range(8)])
    flags: int = 0
    reserved: int = 0
    _genetic_marker: Optional[int] = None
    
    def __post_init__(self):
        """Инициализация после создания объекта."""
        if not hasattr(self, 'coordinates'):
            self.coordinates = [[self.ABSENT_VALUE] * 3 for _ in range(8)]
        if not hasattr(self, 'created_at'):
            self.created_at = time.time()
        if not hasattr(self, 'last_updated'):
            self.last_updated = self.created_at
    
    # Методы для работы с бинарным форматом
    def _unpack(self, buffer: bytes) -> None:
        """Распаковать данные из бинарного формата."""
        unpacked = struct.unpack(self.BINARY_FORMAT, buffer)
        self.coordinates = [list(unpacked[i*3:(i+1)*3]) for i in range(8)]
        self.token_id, self.flags, self.weight, self.reserved, timestamp = unpacked[24:]
        self.created_at = timestamp
        self.last_updated = timestamp
    
    def pack(self) -> bytes:
        """Упаковать данные в бинарный формат."""
        coords_flat = [coord for level in self.coordinates for coord in level]
        return struct.pack(self.BINARY_FORMAT, 
                         *coords_flat, 
                         self.token_id, 
                         self.flags, 
                         self.weight, 
                         self.reserved, 
                         int(self.last_updated))
    
    # Методы для работы с координатами
    def set_coordinates(self, level: int, x: float, y: float, z: float) -> None:
        """Установить координаты для указанного уровня."""
        if not 0 <= level < 8:
            raise ValueError("Уровень должен быть от 0 до 7")
            
        scaled_x = int(x * 100) if x is not None else self.ABSENT_VALUE
        scaled_y = int(y * 100) if y is not None else self.ABSENT_VALUE  
        scaled_z = int(z * 100) if z is not None else self.ABSENT_VALUE
        
        self.coordinates[level] = [scaled_x, scaled_y, scaled_z]
        self.last_updated = time.time()
    
    def get_coordinates(self, level: int) -> Optional[Tuple[float, float, float]]:
        """Получить координаты для указанного уровня."""
        if not 0 <= level < 8:
            raise ValueError("Уровень должен быть от 0 до 7")
            
        x, y, z = self.coordinates[level]
        if x == self.ABSENT_VALUE:
            return None
        return x/100.0, y/100.0, z/100.0
    
    # Методы для работы с генетическими маркерами
    @property
    def genetic_marker(self) -> Optional[int]:
        """Получить генетический маркер."""
        return self._genetic_marker
    
    @genetic_marker.setter
    def genetic_marker(self, value: Optional[int]) -> None:
        """Установить генетический маркер."""
        self._genetic_marker = value
        self.last_updated = time.time()
    
    # Сериализация
    def to_json(self) -> Dict[str, Any]:
        """Преобразовать токен в словарь JSON."""
        result = {
            'token_id': self.token_id,
            'content': self.content,
            'token_type': self.token_type,
            'weight': self.weight,
            'flags': self.flags,
            'reserved': self.reserved,
            'created_at': self.created_at,
            'last_updated': self.last_updated,
            'coordinates': {f"L{i+1}": self.get_coordinates(i) for i in range(8)}
        }
        
        # Добавляем метаданные, если они есть
        if hasattr(self, 'metadata') and self.metadata:
            result['metadata'] = self.metadata
            
        return result
    
    @classmethod
    def from_json(cls, data: Dict[str, Any]) -> 'Token':
        """Создать токен из словаря JSON."""
        token = cls()
        token.token_id = data.get('token_id', int(uuid.uuid4().int & (1 << 64)-1))
        token.content = data.get('content', '')
        token.token_type = data.get('token_type', 'default')
        token.weight = data.get('weight', 1.0)
        token.flags = data.get('flags', 0)
        token.reserved = data.get('reserved', 0)
        token.created_at = data.get('created_at', time.time())
        token.last_updated = data.get('last_updated', token.created_at)
        token.metadata = data.get('metadata', {})
        
        # Восстанавливаем координаты
        coords_data = data.get('coordinates', {})
        for i in range(8):
            level_key = f"L{i+1}"
            if level_key in coords_data and coords_data[level_key] is not None:
                token.set_coordinates(i, *coords_data[level_key])
        
        return token
    
    def update(self, **kwargs) -> None:
        """Обновить атрибуты токена."""
        for key, value in kwargs.items():
            if hasattr(self, key):
                setattr(self, key, value)
        self.last_updated = time.time()
    
    def __hash__(self) -> int:
        return hash((self.token_id, self.content, self.token_type))
    
    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Token):
            return NotImplemented
        return self.token_id == other.token_id
    
    def set_genetic_marker(self, adna_hash: int) -> None:
        """Установить генетический маркер (в reserved поле)"""
        self.reserved = adna_hash & 0xFFFF
        self._genetic_marker = adna_hash
        self.last_updated = time.time()
    
    def get_genetic_marker(self) -> Optional[int]:
        """Получить генетический маркер токена"""
        return self._genetic_marker
    
    def to_dict(self) -> Dict[str, Any]:
        """Альтернативный метод сериализации в словарь"""
        return {
            'token_id': self.token_id,
            'content': self.content,
            'token_type': self.token_type,
            'weight': self.weight,
            'flags': self.flags,
            'reserved': self.reserved,
            'created_at': self.created_at,
            'last_updated': self.last_updated,
            'genetic_marker': self._genetic_marker,
            'coordinates': {f"L{i+1}": self.get_coordinates(i) for i in range(8)},
            'metadata': self.metadata
        }
    
    def get_generation_info(self) -> Dict[str, Any]:
        """Получить информацию о поколении токена"""
        return {
        "genetic_marker": self._genetic_marker,
        "birth_timestamp": self.timestamp,
        "current_weight": self.weight,
        "active_flags": bin(self.flags)
    }