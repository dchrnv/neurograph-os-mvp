from typing import Dict, Tuple, Optional, Iterator, Any
from ..token.token import Token  # Импорт нашего токена

class SparseGrid:
    """
    Разреженная 3D сетка для хранения токенов в непрерывном пространстве.
    Координаты: float от -1.00 до +1.00 с фиксированной точностью.
    """
    
    def __init__(self, precision: int = 2):
        """
        Инициализирует пустую сетку.
        
        Args:
            precision: Количество знаков после запятой для округления координат
        """
        self.precision = precision
        self._grid: Dict[Tuple[float, float, float], Token] = {}
    
    def _round_coords(self, x: float, y: float, z: float) -> Tuple[float, float, float]:
        """Округляет координаты до указанной точности."""
        factor = 10 ** self.precision
        return (
            round(x, self.precision),
            round(y, self.precision), 
            round(z, self.precision)
        )
    
    def insert(self, x: float, y: float, z: float, token: Token) -> None:
        """
        Помещает токен в указанные координаты.
        
        Args:
            x, y, z: Координаты от -1.00 до +1.00
            token: Объект токена для сохранения
        """
        if not (-1.0 <= x <= 1.0 and -1.0 <= y <= 1.0 and -1.0 <= z <= 1.0):
            raise ValueError(f"Координаты должны быть в диапазоне [-1.0, 1.0]. Получено: ({x}, {y}, {z})")
        
        rounded_coords = self._round_coords(x, y, z)
        self._grid[rounded_coords] = token
    
    def get(self, x: float, y: float, z: float) -> Optional[Token]:
        """
        Возвращает токен из указанных координат.
        
        Returns:
            Token или None, если ячейка пуста
        """
        rounded_coords = self._round_coords(x, y, z)
        return self._grid.get(rounded_coords)
    
    def remove(self, x: float, y: float, z: float) -> None:
        """Удаляет токен из указанных координат."""
        rounded_coords = self._round_coords(x, y, z)
        if rounded_coords in self._grid:
            del self._grid[rounded_coords]
    
    def contains(self, x: float, y: float, z: float) -> bool:
        """Проверяет, существует ли токен в указанных координатах."""
        rounded_coords = self._round_coords(x, y, z)
        return rounded_coords in self._grid
    
    def occupied_cells(self) -> Iterator[Tuple[Tuple[float, float, float], Token]]:
        """Возвращает итератор по всем занятым ячейкам."""
        for coords, token in self._grid.items():
            yield coords, token
    
    def __len__(self) -> int:
        """Возвращает количество занятых ячеек."""
        return len(self._grid)
    
    def __repr__(self) -> str:
        return f"SparseGrid(precision={self.precision}, occupied_cells={len(self)})"