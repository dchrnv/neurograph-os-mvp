import struct
import time
from typing import Optional, Tuple, Dict, Any

class Token:
    BINARY_FORMAT = '<24h I H f H I'
    BINARY_SIZE = struct.calcsize(BINARY_FORMAT)
    ABSENT_VALUE = 127

    def __init__(self, buffer: Optional[bytes] = None):
        if buffer:
            self._unpack(buffer)
        else:
            self.coordinates = [[self.ABSENT_VALUE] * 3 for _ in range(8)]
            self.id = 0
            self.flags = 0
            self.weight = 0.0
            self.reserved = 0
            self.timestamp = int(time.time())
            self._genetic_marker: Optional[int] = None

    def _unpack(self, buffer: bytes) -> None:
        unpacked = struct.unpack(self.BINARY_FORMAT, buffer)
        self.coordinates = [list(unpacked[i*3:(i+1)*3]) for i in range(8)]
        self.id, self.flags, self.weight, self.reserved, self.timestamp = unpacked[24:]

    def pack(self) -> bytes:
        coords_flat = [coord for level in self.coordinates for coord in level]
        return struct.pack(self.BINARY_FORMAT, *coords_flat, self.id, self.flags, 
                         self.weight, self.reserved, self.timestamp)

    def set_coordinates(self, level: int, x: float, y: float, z: float) -> None:
        scaled_x = int(x * 100) if x is not None else self.ABSENT_VALUE
        scaled_y = int(y * 100) if y is not None else self.ABSENT_VALUE  
        scaled_z = int(z * 100) if z is not None else self.ABSENT_VALUE
        self.coordinates[level] = [scaled_x, scaled_y, scaled_z]

    def get_coordinates(self, level: int) -> Optional[Tuple[float, float, float]]:
        x, y, z = self.coordinates[level]
        if x == self.ABSENT_VALUE:
            return None
        return x/100.0, y/100.0, z/100.0

    def to_json(self) -> Dict[str, Any]:
        return {
            **{f"L{i+1}": self.get_coordinates(i) for i in range(8)},
            "meta": {"id": self.id, "weight": self.weight, "flags": self.flags, "timestamp": self.timestamp}
        }
    def set_genetic_marker(self, adna_hash: int) -> None:
        """Установить генетический маркер (в reserved поле)"""
        self.reserved = adna_hash & 0xFFFF
        self._genetic_marker = adna_hash

    def get_genetic_marker(self) -> Optional[int]:
        """Получить генетический маркер токена"""
        return self._genetic_marker
    
    def get_generation_info(self) -> Dict[str, Any]:
        """Получить информацию о поколении токена"""
        return {
        "genetic_marker": self._genetic_marker,
        "birth_timestamp": self.timestamp,
        "current_weight": self.weight,
        "active_flags": bin(self.flags)
    }