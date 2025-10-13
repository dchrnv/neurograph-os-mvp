from typing import Any, Dict, Optional, List
from pydantic import BaseModel, Field


class ExperienceEvent(BaseModel):
    event_id: str
    event_type: str
    timestamp: float
    source_component: str
    source_id: Optional[str] = None
    data: Dict[str, Any] = Field(default_factory=dict)
    state: Optional[Any] = None
    action: Optional[Any] = None
    reward: Optional[float] = None
    next_state: Optional[Any] = None
    done: bool = False
    adna_snapshot_hash: Optional[int] = None
    generation_marker: Optional[int] = None
    priority: float = 0.0
    metadata: Optional[Dict[str, Any]] = None
    caused_by: Optional[str] = None
    leads_to: List[str] = Field(default_factory=list)

    # Pydantic v2 configuration
    model_config = {
        'extra': 'ignore',
        'validate_assignment': True
    }
