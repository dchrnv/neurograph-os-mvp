# src/core/token/factory.py
import random
import time
import uuid
import struct
from typing import List, Tuple, Optional

from pydantic import BaseModel, Field
from .token import Token

# Optional experience event model (attach experience stream if available)
try:
    from src.core.experience.event import ExperienceEvent  # absolute import when running from project root
except Exception:
    try:
        from ..experience.event import ExperienceEvent  # relative import as fallback
    except Exception:
        ExperienceEvent = None

# --- DNA integration (необязательно, но при наличии активируется) ---
try:
    from ..dna.guardian import DNAGuardian
    from ..dna.integration import DNAIntegratedComponent
except ImportError:
    DNAGuardian = None
    DNAIntegratedComponent = object

# --- Типы ---
Vector = List[float]
ExperienceData = Tuple[Vector, Vector, float, Vector, bool]


# --- Конфигурационные модели ---
class FactoryDefaults(BaseModel):
    """Значения по умолчанию для создаваемых токенов."""
    weight: float = 0.0
    flags: int = 0
    auto_timestamp: bool = True


class ExperienceFormat(BaseModel):
    """Определяет, как данные опыта упаковываются в уровни токена."""
    state_level: int = Field(0, ge=0, lt=8)
    action_level: int = Field(1, ge=0, lt=8)
    next_state_level: int = Field(2, ge=0, lt=8)
    done_flag_bit: int = Field(0, ge=0, lt=16)


class TokenFactoryConfig(BaseModel):
    """Полная конфигурация TokenFactory."""
    defaults: FactoryDefaults = Field(default_factory=FactoryDefaults)
    experience_format: ExperienceFormat = Field(default_factory=ExperienceFormat)


# --- Основной класс TokenFactory ---
class TokenFactory(DNAIntegratedComponent):
    """
    TokenFactory с поддержкой RL-опыта и интеграцией DNA Guardian (CDNA/ADNA).
    """

    def __init__(self,
                 config: TokenFactoryConfig = TokenFactoryConfig(),
                 dna_guardian: Optional[DNAGuardian] = None):
        # DNA Integration
        if dna_guardian:
            DNAIntegratedComponent.__init__(self, "token", dna_guardian)
        else:
            self.dna_guardian = None
        # optional experience stream (attach externally)
        self.experience = None
        self.config = config
        self._counter = 0

        # CDNA ограничения по умолчанию
        self._weight_min = -1.0
        self._weight_max = 1.0
        self._base_flags_allowed = 0xFFFF
        self._max_coordinate_levels = 8

        # если Guardian есть — загрузим ограничения из CDNA
        if self.dna_guardian:
            self._load_token_constraints_from_cdna()

    # --- CDNA Integration ---
    def _load_token_constraints_from_cdna(self) -> None:
        """Загрузить ограничения токенов из блока CDNA (TokenProperties: 64–95 байты)."""
        cdna_data = self.get_cdna_slice("token")
        if not cdna_data or len(cdna_data) < 32:
            return

        try:
            unpacked = struct.unpack("<2f4I4H8B", cdna_data[:32])
            self._weight_min = unpacked[0]
            self._weight_max = unpacked[1]
            self._base_flags_allowed = unpacked[2]
            self._max_coordinate_levels = unpacked[6] if len(unpacked) > 6 else 8
            print(f"[DNA] TokenFactory loaded CDNA constraints: "
                  f"weight={self._weight_min}-{self._weight_max}, levels={self._max_coordinate_levels}")
        except struct.error:
            print("[DNA] TokenFactory: invalid CDNA token block format")

    def on_cdna_updated(self, event) -> None:
        """Обработка обновления CDNA."""
        if "token" in event.metadata.get("targets", {}):
            print("[DNA] TokenFactory: CDNA updated → reloading constraints")
            self._load_token_constraints_from_cdna()

    def on_adna_updated(self, event) -> None:
        """Обработка изменений ADNA."""
        key = event.metadata.get("key", "")
        if "token" in key.lower():
            print(f"[DNA] TokenFactory: ADNA parameter changed: {key}")

    # --- Utility ---
    def _generate_id(self) -> int:
        return uuid.uuid4().int & (2**32 - 1)

    # --- Основные методы создания токенов ---
    def create_empty_token(self) -> Token:
        """Создает пустой токен с учётом CDNA-ограничений."""
        token = Token()
        token.id = self._generate_id()

        defaults = self.config.defaults
        token.weight = max(self._weight_min, min(self._weight_max, defaults.weight))
        token.flags = defaults.flags

        if defaults.auto_timestamp:
            token.timestamp = int(time.time())

        # генетический маркер из ADNA (если Guardian доступен)
        if self.dna_guardian:
            adna_hash = hash(str(self.dna_guardian._adna))
            token.set_genetic_marker(adna_hash)

        # Записываем опыт (если подключена подсистема Experience и модель события доступна)
        try:
            if self.experience and ExperienceEvent is not None:
                event = ExperienceEvent(
                    event_id=f"token_create_{token.id}",
                    event_type="token_created",
                    timestamp=time.time(),
                    source_component="token_factory",
                    data={
                        "token_id": token.id,
                        "weight": token.weight,
                        "generation_marker": token.get_genetic_marker()
                    },
                    # RL компоненты (если есть контекст)
                    state=self._get_current_state() if hasattr(self, '_get_current_state') else None,
                    reward=self._calculate_creation_reward(token) if hasattr(self, '_calculate_creation_reward') else None
                )
                # best-effort send
                try:
                    self.experience.write_event(event)
                except Exception:
                    pass
        except Exception:
            # don't break token creation if ExperienceEvent isn't available
            pass
        
        return token

    def create_experience_token(self, s_t: Vector, a_t: Vector, r_t: float,
                                s_t_plus_1: Vector, done: bool = False) -> Token:
        """
        Создает токен опыта для буфера воспроизведения.
        """
        token = self.create_empty_token()
        exp_format = self.config.experience_format

        self._write_vector_to_level(token, exp_format.state_level, s_t)
        self._write_vector_to_level(token, exp_format.action_level, a_t)
        self._write_vector_to_level(token, exp_format.next_state_level, s_t_plus_1)

        token.weight = max(self._weight_min, min(self._weight_max, r_t))

        bit = exp_format.done_flag_bit
        if done:
            token.flags |= (1 << bit)
        else:
            token.flags &= ~(1 << bit)
        return token

    def create_state_token(self, observation: Vector) -> Token:
        token = self.create_empty_token()
        self._write_vector_to_level(token, self.config.experience_format.state_level, observation)
        return token

    def create_action_token(self, action: Vector) -> Token:
        token = self.create_empty_token()
        self._write_vector_to_level(token, self.config.experience_format.action_level, action)
        return token

    def create_random_experience(self, state_dim: int, action_dim: int) -> Token:
        """Создает случайный токен опыта для тестирования."""
        s_t = [random.uniform(-1.0, 1.0) for _ in range(state_dim)]
        a_t = [random.uniform(-1.0, 1.0) for _ in range(action_dim)]
        s_t_plus_1 = [random.uniform(-1.0, 1.0) for _ in range(state_dim)]
        r_t = random.uniform(-1.0, 1.0)
        done = random.random() > 0.9
        return self.create_experience_token(s_t, a_t, r_t, s_t_plus_1, done)

    def parse_experience_token(self, token: Token) -> ExperienceData:
        """Извлекает данные опыта из токена."""
        exp_format = self.config.experience_format
        s_t = self._read_vector_from_level(token, exp_format.state_level)
        a_t = self._read_vector_from_level(token, exp_format.action_level)
        s_t_plus_1 = self._read_vector_from_level(token, exp_format.next_state_level)
        done = bool(token.flags & (1 << exp_format.done_flag_bit))
        return s_t, a_t, token.weight, s_t_plus_1, done

    def batch_create_experience(self, experiences: List[ExperienceData]) -> List[Token]:
        """Создает несколько токенов опыта пачкой."""
        return [self.create_experience_token(*exp) for exp in experiences]

    # --- Новый метод: генерация поколений ---
    def create_batch_with_generation_tracking(self, count: int, strategy, params) -> List[Token]:
        """
        Создает пакет токенов с маркировкой поколения (ADNA hash).
        """
        tokens = []
        current_generation_hash = None
        if self.dna_guardian:
            current_generation_hash = hash(str(self.dna_guardian._adna))

        for _ in range(count):
            token = self.create_empty_token()

            # стратегия может быть расширена в будущем (random, adaptive и т.д.)
            if strategy == "random":
                token.weight = random.uniform(self._weight_min, self._weight_max)

            if current_generation_hash:
                token.set_genetic_marker(current_generation_hash)

            tokens.append(token)

        return tokens

    # --- Internal vector I/O ---
    def _write_vector_to_level(self, token: Token, level: int, vector: Vector) -> None:
        """Записывает вектор в указанный уровень токена."""
        if not vector:
            return

        x = vector[0] if len(vector) > 0 else None
        y = vector[1] if len(vector) > 1 else None
        z = vector[2] if len(vector) > 2 else None
        token.set_coordinates(level, x, y, z)

        for i, value in enumerate(vector[3:], start=1):
            if level + i < self._max_coordinate_levels:
                token.set_coordinates(level + i, value, None, None)

    def _read_vector_from_level(self, token: Token, start_level: int) -> Vector:
        """Читает вектор из последовательности уровней токена."""
        vector = []
        for level in range(start_level, self._max_coordinate_levels):
            coords = token.get_coordinates(level)
            if coords is not None:
                if level == start_level:
                    vector.extend(c for c in coords if c is not None)
                elif coords[0] is not None:
                    vector.append(coords[0])
                else:
                    break
            else:
                break
        return vector
