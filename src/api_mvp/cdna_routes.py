"""
CDNA & Guardian API Routes
Endpoints for managing Cognitive DNA configuration
"""

from typing import Dict, List, Optional
from fastapi import APIRouter, HTTPException
from pydantic import BaseModel, Field

router = APIRouter(prefix="/api/v1/cdna", tags=["CDNA"])

# ═══════════════════════════════════════════════════════
# IN-MEMORY STORAGE
# ═══════════════════════════════════════════════════════

# Current CDNA configuration
CURRENT_CDNA = {
    "version": "2.1.0",
    "profile": "explorer",
    "dimension_scales": [1.0, 1.5, 1.2, 2.0, 3.0, 2.5, 2.0, 10.0],
    "timestamp": None
}

# Available profiles
PROFILES = {
    "explorer": {
        "name": "Explorer",
        "scales": [1.0, 1.5, 1.2, 2.0, 3.0, 2.5, 2.0, 5.0],
        "description": "Свободная структура, высокая пластичность",
        "plasticity": 0.8,
        "evolution_rate": 0.5
    },
    "analyzer": {
        "name": "Analyzer",
        "scales": [1.0, 1.0, 1.0, 1.5, 10.0, 5.0, 3.0, 20.0],
        "description": "Строгие правила, низкая эволюция",
        "plasticity": 0.2,
        "evolution_rate": 0.1
    },
    "creative": {
        "name": "Creative",
        "scales": [1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0],
        "description": "Экспериментальный режим",
        "plasticity": 0.95,
        "evolution_rate": 0.8
    },
    "quarantine": {
        "name": "Quarantine",
        "scales": [1.0, 1.0, 1.0, 1.0, 2.0, 1.5, 1.0, 3.0],
        "description": "Изолированный режим тестирования",
        "plasticity": 0.1,
        "evolution_rate": 0.0,
        "restricted": True,
        "max_change": 0.5
    }
}

# CDNA history
CDNA_HISTORY: List[Dict] = []

# Quarantine state
QUARANTINE_STATE = {
    "active": False,
    "time_left": 300,
    "metrics": {
        "memory_growth": 0,
        "connection_breaks": 0,
        "token_churn": 0
    }
}

# ═══════════════════════════════════════════════════════
# SCHEMAS
# ═══════════════════════════════════════════════════════

class ProfileInfo(BaseModel):
    name: str
    scales: List[float]
    description: str
    plasticity: float
    evolution_rate: float
    restricted: Optional[bool] = False
    max_change: Optional[float] = None


class CDNAConfig(BaseModel):
    version: str
    profile: str
    dimension_scales: List[float] = Field(min_items=8, max_items=8)
    timestamp: Optional[str] = None


class CDNAUpdateRequest(BaseModel):
    profile: Optional[str] = None
    dimension_scales: Optional[List[float]] = Field(None, min_items=8, max_items=8)
    should_validate: bool = True


class QuarantineStatus(BaseModel):
    active: bool
    time_left: int
    metrics: Dict[str, int]


class ValidationResult(BaseModel):
    valid: bool
    warnings: List[str] = []
    errors: List[str] = []


class ValidateRequest(BaseModel):
    scales: List[float] = Field(min_items=8, max_items=8)


# ═══════════════════════════════════════════════════════
# ROUTES
# ═══════════════════════════════════════════════════════

@router.get("/status")
async def get_cdna_status():
    """Get current CDNA status and configuration"""
    return {
        "cdna": CURRENT_CDNA,
        "quarantine": QUARANTINE_STATE,
        "history_count": len(CDNA_HISTORY)
    }


@router.get("/config", response_model=CDNAConfig)
async def get_cdna_config():
    """Get current CDNA configuration"""
    return CURRENT_CDNA


@router.get("/profiles")
async def get_profiles():
    """Get all available CDNA profiles"""
    return {
        "profiles": PROFILES,
        "current": CURRENT_CDNA["profile"]
    }


@router.get("/profiles/{profile_id}", response_model=ProfileInfo)
async def get_profile(profile_id: str):
    """Get specific profile information"""
    if profile_id not in PROFILES:
        raise HTTPException(status_code=404, detail=f"Profile '{profile_id}' not found")

    return PROFILES[profile_id]


@router.post("/profile/{profile_id}")
async def switch_profile(profile_id: str):
    """Switch to a different CDNA profile"""
    if profile_id not in PROFILES:
        raise HTTPException(status_code=404, detail=f"Profile '{profile_id}' not found")

    old_profile = CURRENT_CDNA["profile"]
    profile = PROFILES[profile_id]

    # Add to history
    CDNA_HISTORY.insert(0, {
        "action": "profile_switch",
        "from": old_profile,
        "to": profile_id,
        "timestamp": "now"
    })

    # Update current CDNA
    CURRENT_CDNA["profile"] = profile_id
    CURRENT_CDNA["dimension_scales"] = profile["scales"].copy()

    return {
        "success": True,
        "old_profile": old_profile,
        "new_profile": profile_id,
        "scales": profile["scales"]
    }


@router.post("/update")
async def update_cdna(request: CDNAUpdateRequest):
    """Update CDNA configuration"""

    # Validate request
    if request.profile and request.profile not in PROFILES:
        raise HTTPException(status_code=400, detail=f"Invalid profile: {request.profile}")

    if request.dimension_scales:
        # Validate scales
        for scale in request.dimension_scales:
            if scale < 0 or scale > 50:
                raise HTTPException(
                    status_code=400,
                    detail=f"Scale value {scale} out of range [0, 50]"
                )

    # Update configuration
    if request.profile:
        CURRENT_CDNA["profile"] = request.profile
        CURRENT_CDNA["dimension_scales"] = PROFILES[request.profile]["scales"].copy()

    if request.dimension_scales:
        CURRENT_CDNA["dimension_scales"] = request.dimension_scales

    # Add to history
    CDNA_HISTORY.insert(0, {
        "action": "manual_update",
        "profile": CURRENT_CDNA["profile"],
        "scales": CURRENT_CDNA["dimension_scales"],
        "timestamp": "now"
    })

    return {
        "success": True,
        "cdna": CURRENT_CDNA
    }


@router.post("/validate")
async def validate_cdna(request: ValidateRequest):
    """Validate CDNA configuration"""

    warnings = []
    errors = []

    # Dimension limits
    dimension_limits = [
        (0, 20), (0, 20), (0, 20), (0, 20),  # Physical, Sensory, Motor, Emotional
        (0, 30), (0, 20), (0, 20), (0, 50)   # Cognitive, Social, Temporal, Abstract
    ]

    for i, (scale, (min_val, max_val)) in enumerate(zip(request.scales, dimension_limits)):
        if scale < min_val or scale > max_val:
            errors.append(f"Dimension {i} value {scale} out of range [{min_val}, {max_val}]")
        elif scale > max_val * 0.75:
            warnings.append(f"Dimension {i} value {scale} in danger zone (>{max_val * 0.75})")
        elif scale > max_val * 0.5:
            warnings.append(f"Dimension {i} value {scale} in caution zone (>{max_val * 0.5})")

    return ValidationResult(
        valid=len(errors) == 0,
        warnings=warnings,
        errors=errors
    )


@router.get("/history")
async def get_cdna_history(limit: int = 10):
    """Get CDNA configuration history"""
    return {
        "history": CDNA_HISTORY[:limit],
        "total": len(CDNA_HISTORY)
    }


@router.post("/quarantine/start")
async def start_quarantine():
    """Start quarantine mode for testing configuration changes"""

    if QUARANTINE_STATE["active"]:
        raise HTTPException(status_code=400, detail="Quarantine already active")

    QUARANTINE_STATE["active"] = True
    QUARANTINE_STATE["time_left"] = 300
    QUARANTINE_STATE["metrics"] = {
        "memory_growth": 0,
        "connection_breaks": 0,
        "token_churn": 0
    }

    return {
        "success": True,
        "message": "Quarantine mode activated",
        "duration": 300
    }


@router.post("/quarantine/stop")
async def stop_quarantine(apply: bool = False):
    """Stop quarantine mode"""

    if not QUARANTINE_STATE["active"]:
        raise HTTPException(status_code=400, detail="No active quarantine")

    QUARANTINE_STATE["active"] = False

    if apply:
        # Add to history
        CDNA_HISTORY.insert(0, {
            "action": "quarantine_applied",
            "metrics": QUARANTINE_STATE["metrics"],
            "timestamp": "now"
        })
        message = "Quarantine changes applied"
    else:
        message = "Quarantine changes discarded"

    return {
        "success": True,
        "message": message,
        "applied": apply
    }


@router.get("/quarantine/status", response_model=QuarantineStatus)
async def get_quarantine_status():
    """Get quarantine status"""
    return QUARANTINE_STATE


@router.post("/export")
async def export_cdna():
    """Export current CDNA configuration"""
    import json
    from datetime import datetime

    export_data = {
        **CURRENT_CDNA,
        "timestamp": datetime.now().isoformat(),
        "profile_info": PROFILES[CURRENT_CDNA["profile"]]
    }

    return {
        "success": True,
        "data": export_data,
        "filename": f"cdna_{CURRENT_CDNA['profile']}_{int(datetime.now().timestamp())}.json"
    }


@router.post("/reset")
async def reset_cdna():
    """Reset CDNA to default (Explorer) profile"""

    CURRENT_CDNA["profile"] = "explorer"
    CURRENT_CDNA["dimension_scales"] = PROFILES["explorer"]["scales"].copy()

    CDNA_HISTORY.insert(0, {
        "action": "reset_to_default",
        "profile": "explorer",
        "timestamp": "now"
    })

    return {
        "success": True,
        "message": "CDNA reset to Explorer profile",
        "cdna": CURRENT_CDNA
    }
