"""
API Keys Management Endpoints

CRUD operations for API keys.
Requires admin or developer role for key management.
"""

from fastapi import APIRouter, Depends, HTTPException, status
from typing import List

from ..models.response import ApiResponse, SuccessResponse
from ..models.auth import User, APIKey, APIKeyCreate
from ..auth.dependencies import get_current_active_user
from ..auth.permissions import Permission
from ..storage.api_keys import get_api_key_storage
import logging

router = APIRouter()
logger = logging.getLogger(__name__)


@router.post("/api-keys", response_model=ApiResponse, status_code=status.HTTP_201_CREATED)
async def create_api_key(
    request: APIKeyCreate,
    current_user: User = Depends(get_current_active_user)
):
    """
    Create new API key.

    **Requires:** `admin:config` permission

    Returns the full API key **only once**. Store it securely!

    Args:
        request: API key creation parameters

    Returns:
        Full API key and metadata
    """
    # Check permission (only admins and developers can create keys)
    if Permission.ADMIN_CONFIG.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.ADMIN_CONFIG.value} required"
        )

    try:
        api_key_storage = get_api_key_storage()

        # Generate key
        full_key, api_key = api_key_storage.generate_key(
            name=request.name,
            scopes=request.scopes,
            rate_limit=request.rate_limit,
            expires_in_days=request.expires_in_days,
            environment="live"  # TODO: Make configurable
        )

        logger.info(f"API key created: {api_key.key_id} by {current_user.user_id}")

        # Return full key (only shown once!)
        return ApiResponse.success_response(
            data={
                "api_key": full_key,  # ONLY time full key is returned!
                "key_id": api_key.key_id,
                "key_prefix": api_key.key_prefix,
                "name": api_key.name,
                "scopes": api_key.scopes,
                "rate_limit": api_key.rate_limit,
                "created_at": api_key.created_at.isoformat(),
                "expires_at": api_key.expires_at.isoformat() if api_key.expires_at else None,
                "warning": "⚠️ Save this key securely! It will not be shown again."
            }
        )

    except Exception as e:
        logger.error(f"API key creation failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to create API key: {str(e)}"
        )


@router.get("/api-keys", response_model=ApiResponse)
async def list_api_keys(
    current_user: User = Depends(get_current_active_user)
):
    """
    List all API keys.

    **Requires:** `admin:config` permission

    Returns list of API keys (without full key values).
    """
    # Check permission
    if Permission.ADMIN_CONFIG.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.ADMIN_CONFIG.value} required"
        )

    try:
        api_key_storage = get_api_key_storage()
        keys = api_key_storage.list_keys()

        keys_data = [
            {
                "key_id": k.key_id,
                "key_prefix": k.key_prefix,
                "name": k.name,
                "scopes": k.scopes,
                "rate_limit": k.rate_limit,
                "created_at": k.created_at.isoformat(),
                "expires_at": k.expires_at.isoformat() if k.expires_at else None,
                "last_used_at": k.last_used_at.isoformat() if k.last_used_at else None,
                "disabled": k.disabled
            }
            for k in keys
        ]

        logger.debug(f"Listed {len(keys)} API keys")

        return ApiResponse.success_response(
            data={"keys": keys_data, "count": len(keys)}
        )

    except Exception as e:
        logger.error(f"API key listing failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to list API keys: {str(e)}"
        )


@router.get("/api-keys/{key_id}", response_model=ApiResponse)
async def get_api_key(
    key_id: str,
    current_user: User = Depends(get_current_active_user)
):
    """
    Get specific API key details.

    **Requires:** `admin:config` permission

    Returns key metadata (without full key value).
    """
    # Check permission
    if Permission.ADMIN_CONFIG.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.ADMIN_CONFIG.value} required"
        )

    try:
        api_key_storage = get_api_key_storage()
        api_key = api_key_storage.get_key(key_id)

        if not api_key:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"API key {key_id} not found"
            )

        return ApiResponse.success_response(
            data={
                "key_id": api_key.key_id,
                "key_prefix": api_key.key_prefix,
                "name": api_key.name,
                "scopes": api_key.scopes,
                "rate_limit": api_key.rate_limit,
                "created_at": api_key.created_at.isoformat(),
                "expires_at": api_key.expires_at.isoformat() if api_key.expires_at else None,
                "last_used_at": api_key.last_used_at.isoformat() if api_key.last_used_at else None,
                "disabled": api_key.disabled
            }
        )

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"API key retrieval failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to get API key: {str(e)}"
        )


@router.post("/api-keys/{key_id}/revoke", response_model=ApiResponse)
async def revoke_api_key(
    key_id: str,
    current_user: User = Depends(get_current_active_user)
):
    """
    Revoke (disable) API key.

    **Requires:** `admin:config` permission

    The key remains in storage but is disabled.
    """
    # Check permission
    if Permission.ADMIN_CONFIG.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.ADMIN_CONFIG.value} required"
        )

    try:
        api_key_storage = get_api_key_storage()
        success = api_key_storage.revoke_key(key_id)

        if not success:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"API key {key_id} not found"
            )

        logger.info(f"API key revoked: {key_id} by {current_user.user_id}")

        return ApiResponse.success_response(
            data={"key_id": key_id, "status": "revoked"}
        )

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"API key revocation failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to revoke API key: {str(e)}"
        )


@router.delete("/api-keys/{key_id}", status_code=status.HTTP_204_NO_CONTENT)
async def delete_api_key(
    key_id: str,
    current_user: User = Depends(get_current_active_user)
):
    """
    Permanently delete API key.

    **Requires:** `admin:config` permission

    The key is completely removed from storage.
    """
    # Check permission
    if Permission.ADMIN_CONFIG.value not in current_user.scopes:
        raise HTTPException(
            status_code=status.HTTP_403_FORBIDDEN,
            detail=f"Permission denied: {Permission.ADMIN_CONFIG.value} required"
        )

    try:
        api_key_storage = get_api_key_storage()
        success = api_key_storage.delete_key(key_id)

        if not success:
            raise HTTPException(
                status_code=status.HTTP_404_NOT_FOUND,
                detail=f"API key {key_id} not found"
            )

        logger.info(f"API key deleted: {key_id} by {current_user.user_id}")

    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"API key deletion failed: {e}", exc_info=True)
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"Failed to delete API key: {str(e)}"
        )
