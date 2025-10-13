from typing import List, Optional

from src.core.token.token import Token
from src.core.token.factory import TokenFactory
from src.core.graph.graph_engine import TokenGraph
from src.core.spatial.sparse_grid import SparseGrid
from src.core.spatial.coordinates import MultiCoordinate, Point3D


class TokenService:
    """
    Сервис прикладного уровня для управления жизненным циклом токенов.
    Оркестрирует взаимодействие между фабрикой, графом и пространственной сеткой.
    """

    def __init__(
        self,
        token_factory: TokenFactory,
        token_graph: TokenGraph,
        sparse_grid: SparseGrid,
    ):
        self.token_factory = token_factory
        self.token_graph = token_graph
        self.sparse_grid = sparse_grid

    def create_and_place_token(
        self,
        level: int,
        x: float,
        y: float,
        z: float,
        weight: Optional[float] = None,
        flags: Optional[int] = None,
    ) -> Token:
        """
        Создает, размещает в пространстве и добавляет в граф новый токен.
        """
        # 1. Создаем токен через фабрику
        token = self.token_factory.create_empty_token()
        if weight is not None:
            token.weight = weight
        if flags is not None:
            token.flags = flags

        # 2. Размещаем токен в пространственной сетке
        self.sparse_grid.place_token_simple(token, level, x, y, z)

        # 3. Добавляем токен в граф (который уже знает о сетке)
        self.token_graph.add_token(token)

        return token