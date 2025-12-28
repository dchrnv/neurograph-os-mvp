"""
CDNA Endpoints

Configuration and management for Cognitive DNA (CDNA) system.
Handles profiles, validation, quarantine mode, and configuration history.
"""

from fastapi import APIRouter, Depends, HTTPException, Query, status
from typing import Dict, Any, List
import logging
from datetime import datetime

from ..models.response import ApiResponse
from ..models.cdna import (
    CDNAConfig,
    CDNAUpdateRequest,
    CDNAStatusResponse,
    CDNAProfilesResponse,
    ProfileInfo,
    ProfileSwitchResponse,
    QuarantineStatus,
    QuarantineStartResponse,
    QuarantineStopResponse,
    ValidationResult,
    ValidateRequest,
    CDNAHistoryResponse,
    CDNAExportResponse,
    CDNAResetResponse,
)
from ..models.auth import User
from ..dependencies import get_cdna_storage
from ..config import settings
from ..auth.dependencies import get_current_active_user
from ..auth.permissions import Permission

logger = logging.getLogger(__name__)
router = APIRouter()


# =============================================================================
# Helper Functions
# =============================================================================

def config_to_response(config: Dict[str, Any]) -> CDNAConfig:
    """Convert storage config to CDNAConfig model."""
    return CDNAConfig(
        version=config.get("version", "2.1.0"),
        profile=config.get("profile", "explorer"),
        dimension_scales=config.get("dimension_scales", [1.0] * 8),
        timestamp=config.get("timestamp")
    )


def profile_to_info(profile: Dict[str, Any]) -> ProfileInfo:
    """Convert storage profile to ProfileInfo model."""
    return ProfileInfo(
        name=profile.get("name", "Unknown"),
        scales=profile.get("scales", [1.0] * 8),
        description=profile.get("description", ""),
        plasticity=profile.get("plasticity", 0.5),
        evolution_rate=profile.get("evolution_rate", 0.5),
        restricted=profile.get("restricted", False),
        max_change=profile.get("max_change")
    )


# =============================================================================
# Endpoints
# =============================================================================

@router.get("/cdna/status", response_model=ApiResponse)
async def get_cdna_status(
    storage=Depends(get_cdna_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Get current CDNA status.

    **Requires:** `cdna:read` permission

    Returns current configuration, quarantine state, and history count.
    """
    # Check permission
    if Permission.READ_CDNA.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.READ_CDNA.value} required"
        )

    if not settings.ENABLE_NEW_CDNA_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="CDNA API not enabled"
        )

    try:
        # Get current config
        config = storage.get_config()
        cdna_config = config_to_response(config)

        # Get quarantine status
        quarantine = storage.get_quarantine_status()

        # Get history count
        history = storage.get_history(limit=1000)  # Get all to count
        history_count = len(history)

        response_data = CDNAStatusResponse(
            cdna=cdna_config,
            quarantine=quarantine,
            history_count=history_count
        )

        logger.debug(f"CDNA status retrieved: profile={cdna_config.profile}")

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except Exception as e:
        logger.error(f"CDNA status retrieval failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to get CDNA status: {str(e)}"
        )


@router.put("/cdna/config", response_model=ApiResponse)
async def update_cdna_config(
    request: CDNAUpdateRequest,
    storage=Depends(get_cdna_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Update CDNA configuration.

    **Requires:** `cdna:write` permission

    Can switch profiles or set custom dimension scales.
    Optionally validates configuration before applying.
    """
    # Check permission
    if Permission.WRITE_CDNA.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.WRITE_CDNA.value} required"
        )

    if not settings.ENABLE_NEW_CDNA_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="CDNA API not enabled"
        )

    try:
        current_config = storage.get_config()

        # Validate if requested
        if request.should_validate:
            scales_to_validate = request.dimension_scales
            if not scales_to_validate and request.profile:
                profile = storage.get_profile(request.profile)
                if not profile:
                    raise HTTPException(
                        status_code=status.HTTP_404_NOT_FOUND,
                        detail=f"Profile '{request.profile}' not found"
                    )
                scales_to_validate = profile.get("scales")

            if scales_to_validate:
                valid, warnings, errors = storage.validate_scales(scales_to_validate)
                if not valid:
                    raise HTTPException(
                        status_code=status.HTTP_400_BAD_REQUEST,
                        detail={
                            "message": "Configuration validation failed",
                            "errors": errors,
                            "warnings": warnings
                        }
                    )

        # Build update dict
        update_data = {}
        if request.profile:
            update_data["profile"] = request.profile
            # Get profile scales
            profile = storage.get_profile(request.profile)
            if profile:
                update_data["dimension_scales"] = profile.get("scales")

        if request.dimension_scales:
            update_data["dimension_scales"] = request.dimension_scales

        # Update timestamp
        update_data["timestamp"] = datetime.utcnow().isoformat() + "Z"

        # Apply update
        success = storage.update_config(update_data)

        if not success:
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                detail="Failed to update configuration"
            )

        # Add to history
        history_entry = {
            "action": "config_update",
            "timestamp": update_data["timestamp"],
            "changes": update_data
        }
        storage.add_history(history_entry)

        # Get updated config
        new_config = storage.get_config()
        response_data = config_to_response(new_config)

        logger.info(f"CDNA config updated: {update_data}")

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"CDNA config update failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to update config: {str(e)}"
        )


@router.get("/cdna/profiles", response_model=ApiResponse)
async def list_profiles(
    storage=Depends(get_cdna_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    List all available CDNA profiles.

    **Requires:** `cdna:read` permission

    Returns predefined profiles (explorer, analyzer, creative, quarantine).
    """
    # Check permission
    if Permission.READ_CDNA.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.READ_CDNA.value} required"
        )

    if not settings.ENABLE_NEW_CDNA_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="CDNA API not enabled"
        )

    try:
        profiles_dict = storage.list_profiles()
        current_config = storage.get_config()
        current_profile = current_config.get("profile", "explorer")

        # Convert to ProfileInfo models
        profiles_info = {
            profile_id: profile_to_info(profile_data)
            for profile_id, profile_data in profiles_dict.items()
        }

        response_data = CDNAProfilesResponse(
            profiles=profiles_info,
            current=current_profile
        )

        logger.debug(f"Listed {len(profiles_info)} CDNA profiles")

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except Exception as e:
        logger.error(f"Profile listing failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to list profiles: {str(e)}"
        )


@router.get("/cdna/profiles/{profile_id}", response_model=ApiResponse)
async def get_profile(
    profile_id: str,
    storage=Depends(get_cdna_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Get specific CDNA profile.

    **Requires:** `cdna:read` permission

    Returns detailed information about a single profile.
    """
    # Check permission
    if Permission.READ_CDNA.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.READ_CDNA.value} required"
        )

    if not settings.ENABLE_NEW_CDNA_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="CDNA API not enabled"
        )

    try:
        profile = storage.get_profile(profile_id)

        if not profile:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Profile '{profile_id}' not found"
            )

        profile_info = profile_to_info(profile)

        logger.debug(f"Retrieved profile: {profile_id}")

        return ApiResponse.success_response(
            data=profile_info.model_dump(),
            processing_time_ms=0.0
        )

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Profile retrieval failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to get profile: {str(e)}"
        )


@router.post("/cdna/profiles/{profile_id}/switch", response_model=ApiResponse)
async def switch_profile(
    profile_id: str,
    storage=Depends(get_cdna_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Switch to different CDNA profile.

    **Requires:** `cdna:write` permission

    Changes active profile and applies its dimension scales.
    """
    # Check permission
    if Permission.WRITE_CDNA.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.WRITE_CDNA.value} required"
        )

    if not settings.ENABLE_NEW_CDNA_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="CDNA API not enabled"
        )

    try:
        # Check if profile exists
        profile = storage.get_profile(profile_id)
        if not profile:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"Profile '{profile_id}' not found"
            )

        # Get current profile
        current_config = storage.get_config()
        old_profile = current_config.get("profile", "explorer")

        # Switch profile
        success = storage.switch_profile(profile_id)

        if not success:
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                detail="Failed to switch profile"
            )

        # Add to history
        history_entry = {
            "action": "profile_switch",
            "from": old_profile,
            "to": profile_id,
            "timestamp": datetime.utcnow().isoformat() + "Z"
        }
        storage.add_history(history_entry)

        # Get new scales
        new_scales = profile.get("scales", [1.0] * 8)

        response_data = ProfileSwitchResponse(
            success=True,
            old_profile=old_profile,
            new_profile=profile_id,
            scales=new_scales
        )

        logger.info(f"Profile switched: {old_profile} -> {profile_id}")

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Profile switch failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to switch profile: {str(e)}"
        )


@router.post("/cdna/validate", response_model=ApiResponse)
async def validate_scales(
    request: ValidateRequest,
    storage=Depends(get_cdna_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Validate dimension scales.

    **Requires:** `cdna:read` permission

    Checks if scales are within safe operating ranges.
    Returns validation result with warnings and errors.
    """
    # Check permission
    if Permission.READ_CDNA.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.READ_CDNA.value} required"
        )

    if not settings.ENABLE_NEW_CDNA_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="CDNA API not enabled"
        )

    try:
        valid, warnings, errors = storage.validate_scales(request.scales)

        response_data = ValidationResult(
            valid=valid,
            warnings=warnings,
            errors=errors
        )

        logger.debug(f"Scales validation: valid={valid}, warnings={len(warnings)}, errors={len(errors)}")

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except Exception as e:
        logger.error(f"Scales validation failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to validate scales: {str(e)}"
        )


@router.get("/cdna/quarantine/status", response_model=ApiResponse)
async def get_quarantine_status(
    storage=Depends(get_cdna_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Get quarantine mode status.

    **Requires:** `cdna:read` permission

    Returns whether quarantine is active and remaining time.
    """
    # Check permission
    if Permission.READ_CDNA.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.READ_CDNA.value} required"
        )

    if not settings.ENABLE_NEW_CDNA_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="CDNA API not enabled"
        )

    try:
        quarantine_data = storage.get_quarantine_status()

        response_data = QuarantineStatus(
            active=quarantine_data.get("active", False),
            time_left=quarantine_data.get("time_left", 0),
            metrics=quarantine_data.get("metrics", {})
        )

        logger.debug(f"Quarantine status: active={response_data.active}")

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except Exception as e:
        logger.error(f"Quarantine status retrieval failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to get quarantine status: {str(e)}"
        )


@router.post("/cdna/quarantine/start", response_model=ApiResponse)
async def start_quarantine(
    duration: int = Query(300, ge=60, le=3600, description="Quarantine duration in seconds"),
    storage=Depends(get_cdna_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Start quarantine mode.

    **Requires:** `cdna:write` permission

    Switches to quarantine profile with restricted changes.
    Duration is in seconds (default 300, max 3600).
    """
    # Check permission
    if Permission.WRITE_CDNA.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.WRITE_CDNA.value} required"
        )

    if not settings.ENABLE_NEW_CDNA_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="CDNA API not enabled"
        )

    try:
        # Check if already in quarantine
        quarantine_data = storage.get_quarantine_status()
        if quarantine_data.get("active", False):
            raise HTTPException(
                status_code=status.HTTP_400_BAD_REQUEST,
                detail="Quarantine mode already active"
            )

        # Start quarantine
        success = storage.start_quarantine()

        if not success:
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                detail="Failed to start quarantine"
            )

        # Add to history
        history_entry = {
            "action": "quarantine_start",
            "duration": duration,
            "timestamp": datetime.utcnow().isoformat() + "Z"
        }
        storage.add_history(history_entry)

        response_data = QuarantineStartResponse(
            success=True,
            message="Quarantine mode started",
            duration=duration
        )

        logger.warning(f"Quarantine mode started: duration={duration}s")

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Quarantine start failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to start quarantine: {str(e)}"
        )


@router.post("/cdna/quarantine/stop", response_model=ApiResponse)
async def stop_quarantine(
    apply: bool = Query(False, description="Apply quarantine changes"),
    storage=Depends(get_cdna_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Stop quarantine mode.

    **Requires:** `cdna:write` permission

    Optionally applies changes made during quarantine.
    If not applied, reverts to pre-quarantine state.
    """
    # Check permission
    if Permission.WRITE_CDNA.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.WRITE_CDNA.value} required"
        )

    if not settings.ENABLE_NEW_CDNA_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="CDNA API not enabled"
        )

    try:
        # Check if quarantine is active
        quarantine_data = storage.get_quarantine_status()
        if not quarantine_data.get("active", False):
            raise HTTPException(
                status_code=status.HTTP_400_BAD_REQUEST,
                detail="Quarantine mode not active"
            )

        # Stop quarantine
        success = storage.stop_quarantine(apply=apply)

        if not success:
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                detail="Failed to stop quarantine"
            )

        # Add to history
        history_entry = {
            "action": "quarantine_stop",
            "applied": apply,
            "timestamp": datetime.utcnow().isoformat() + "Z"
        }
        storage.add_history(history_entry)

        response_data = QuarantineStopResponse(
            success=True,
            message="Quarantine mode stopped" + (" and changes applied" if apply else " (changes discarded)"),
            applied=apply
        )

        logger.info(f"Quarantine mode stopped: apply={apply}")

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Quarantine stop failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to stop quarantine: {str(e)}"
        )


@router.get("/cdna/history", response_model=ApiResponse)
async def get_history(
    limit: int = Query(10, ge=1, le=100, description="Maximum number of entries"),
    storage=Depends(get_cdna_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Get CDNA configuration history.

    **Requires:** `cdna:read` permission

    Returns recent configuration changes and profile switches.
    """
    # Check permission
    if Permission.READ_CDNA.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.READ_CDNA.value} required"
        )

    if not settings.ENABLE_NEW_CDNA_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="CDNA API not enabled"
        )

    try:
        history = storage.get_history(limit=limit)
        total = len(storage.get_history(limit=1000))  # Get all to count

        response_data = CDNAHistoryResponse(
            history=history,
            total=total
        )

        logger.debug(f"Retrieved {len(history)} history entries")

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except Exception as e:
        logger.error(f"History retrieval failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to get history: {str(e)}"
        )


@router.post("/cdna/export", response_model=ApiResponse)
async def export_config(
    storage=Depends(get_cdna_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Export current CDNA configuration.

    **Requires:** `cdna:read` permission

    Returns full configuration in exportable format.
    """
    # Check permission
    if Permission.READ_CDNA.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.READ_CDNA.value} required"
        )

    if not settings.ENABLE_NEW_CDNA_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="CDNA API not enabled"
        )

    try:
        config = storage.get_config()
        history = storage.get_history(limit=100)

        export_data = {
            "cdna": config,
            "history": history,
            "exported_at": datetime.utcnow().isoformat() + "Z"
        }

        filename = f"cdna_export_{datetime.utcnow().strftime('%Y%m%d_%H%M%S')}.json"

        response_data = CDNAExportResponse(
            success=True,
            data=export_data,
            filename=filename
        )

        logger.info(f"CDNA configuration exported: {filename}")

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except Exception as e:
        logger.error(f"Config export failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to export config: {str(e)}"
        )


@router.post("/cdna/reset", response_model=ApiResponse)
async def reset_config(
    storage=Depends(get_cdna_storage),
    current_user: User = Depends(get_current_active_user)
):
    """
    Reset CDNA to default configuration.

    **Requires:** `admin:config` permission

    Resets to explorer profile and clears history.
    Use with caution!
    """
    # Check permission
    if Permission.ADMIN_CONFIG.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.ADMIN_CONFIG.value} required"
        )

    if not settings.ENABLE_NEW_CDNA_API:
        raise HTTPException(
            status_code=status.HTTP_503_SERVICE_UNAVAILABLE,
            detail="CDNA API not enabled"
        )

    try:
        # TODO: Add admin authentication check
        # For now, anyone can reset (development mode)

        # Reset to default (explorer profile)
        default_config = {
            "version": "2.1.0",
            "profile": "explorer",
            "dimension_scales": [1.0, 1.5, 1.2, 2.0, 3.0, 2.5, 2.0, 5.0],
            "timestamp": datetime.utcnow().isoformat() + "Z"
        }

        success = storage.update_config(default_config)

        if not success:
            raise HTTPException(
                status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
                detail="Failed to reset configuration"
            )

        # Add to history
        history_entry = {
            "action": "reset",
            "timestamp": default_config["timestamp"]
        }
        storage.add_history(history_entry)

        new_config = storage.get_config()
        config_response = config_to_response(new_config)

        response_data = CDNAResetResponse(
            success=True,
            message="CDNA configuration reset to defaults",
            cdna=config_response
        )

        logger.warning("CDNA configuration reset to defaults")

        return ApiResponse.success_response(
            data=response_data.model_dump(),
            processing_time_ms=0.0
        )

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Config reset failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to reset config: {str(e)}"
        )
