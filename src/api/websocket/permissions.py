
    # NeuroGraph - Высокопроизводительная система пространственных вычислений на основе токенов.
    # Copyright (C) 2024-2025 Chernov Denys

    # This program is free software: you can redistribute it and/or modify
    # it under the terms of the GNU Affero General Public License as published by
    # the Free Software Foundation, either version 3 of the License, or
    # (at your option) any later version.

    # This program is distributed in the hope that it will be useful,
    # but WITHOUT ANY WARRANTY; without even the implied warranty of
    # MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
    # GNU Affero General Public License for more details.

    # You should have received a copy of the GNU Affero General Public License
    # along with this program. If not, see <https://www.gnu.org/licenses/>.

"""
WebSocket Channel Permissions

RBAC for WebSocket channels - which roles can subscribe to which channels.
"""

from typing import Set, Optional, List
from enum import Enum

from .channels import Channel
from ..logging_config import get_logger

logger = get_logger(__name__, service="websocket_permissions")


class ChannelPermission(str, Enum):
    """Channel access permissions."""

    SUBSCRIBE = "subscribe"  # Can subscribe to channel
    BROADCAST = "broadcast"  # Can broadcast to channel (admin only)


# Channel permissions by role
CHANNEL_PERMISSIONS = {
    # Metrics channel - all authenticated users
    Channel.METRICS: {
        "admin": [ChannelPermission.SUBSCRIBE, ChannelPermission.BROADCAST],
        "developer": [ChannelPermission.SUBSCRIBE],
        "viewer": [ChannelPermission.SUBSCRIBE],
        "bot": [ChannelPermission.SUBSCRIBE],
        "anonymous": [ChannelPermission.SUBSCRIBE],  # Public channel
    },

    # Signals channel - developers and admins only
    Channel.SIGNALS: {
        "admin": [ChannelPermission.SUBSCRIBE, ChannelPermission.BROADCAST],
        "developer": [ChannelPermission.SUBSCRIBE],
        "bot": [ChannelPermission.SUBSCRIBE],  # Bots can monitor signals
    },

    # Actions channel - developers and admins only
    Channel.ACTIONS: {
        "admin": [ChannelPermission.SUBSCRIBE, ChannelPermission.BROADCAST],
        "developer": [ChannelPermission.SUBSCRIBE],
    },

    # Logs channel - developers and admins only
    Channel.LOGS: {
        "admin": [ChannelPermission.SUBSCRIBE, ChannelPermission.BROADCAST],
        "developer": [ChannelPermission.SUBSCRIBE],
    },

    # Status channel - all authenticated users
    Channel.STATUS: {
        "admin": [ChannelPermission.SUBSCRIBE, ChannelPermission.BROADCAST],
        "developer": [ChannelPermission.SUBSCRIBE],
        "viewer": [ChannelPermission.SUBSCRIBE],
        "bot": [ChannelPermission.SUBSCRIBE],
        "anonymous": [ChannelPermission.SUBSCRIBE],  # Public channel
    },

    # Connections channel - admins only
    Channel.CONNECTIONS: {
        "admin": [ChannelPermission.SUBSCRIBE, ChannelPermission.BROADCAST],
    },
}


def get_accessible_channels(role: str) -> List[str]:
    """
    Get list of channels accessible to a role.

    Args:
        role: User role (admin, developer, viewer, bot, anonymous)

    Returns:
        List of channel names the role can access
    """
    accessible = []

    for channel, permissions in CHANNEL_PERMISSIONS.items():
        if role in permissions and ChannelPermission.SUBSCRIBE in permissions[role]:
            accessible.append(channel)

    return accessible


def can_subscribe(channel: str, role: Optional[str] = None) -> bool:
    """
    Check if a role can subscribe to a channel.

    Args:
        channel: Channel name
        role: User role (None = anonymous)

    Returns:
        True if role can subscribe, False otherwise
    """
    role = role or "anonymous"

    # Check if channel exists
    if channel not in CHANNEL_PERMISSIONS:
        logger.warning(
            f"Unknown channel: {channel}",
            extra={"event": "channel_permission_check", "channel": channel, "role": role}
        )
        return False

    channel_perms = CHANNEL_PERMISSIONS[channel]

    # Check if role has permission
    if role not in channel_perms:
        logger.info(
            f"Role {role} denied access to channel {channel}",
            extra={
                "event": "channel_access_denied",
                "channel": channel,
                "role": role,
                "reason": "role_not_in_permissions"
            }
        )
        return False

    # Check if subscribe permission exists
    has_permission = ChannelPermission.SUBSCRIBE in channel_perms[role]

    if not has_permission:
        logger.info(
            f"Role {role} denied subscribe to channel {channel}",
            extra={
                "event": "channel_subscribe_denied",
                "channel": channel,
                "role": role,
                "reason": "no_subscribe_permission"
            }
        )

    return has_permission


def can_broadcast(channel: str, role: Optional[str] = None) -> bool:
    """
    Check if a role can broadcast to a channel.

    Args:
        channel: Channel name
        role: User role (None = anonymous)

    Returns:
        True if role can broadcast, False otherwise
    """
    role = role or "anonymous"

    # Check if channel exists
    if channel not in CHANNEL_PERMISSIONS:
        return False

    channel_perms = CHANNEL_PERMISSIONS[channel]

    # Check if role has permission
    if role not in channel_perms:
        return False

    # Check if broadcast permission exists
    return ChannelPermission.BROADCAST in channel_perms[role]


def filter_channels_by_permission(
    channels: List[str],
    role: Optional[str] = None,
    permission: ChannelPermission = ChannelPermission.SUBSCRIBE
) -> List[str]:
    """
    Filter channels by permission.

    Args:
        channels: List of channel names to filter
        role: User role
        permission: Required permission

    Returns:
        Filtered list of channels the role has permission for
    """
    role = role or "anonymous"
    filtered = []

    for channel in channels:
        if channel not in CHANNEL_PERMISSIONS:
            continue

        channel_perms = CHANNEL_PERMISSIONS[channel]
        if role in channel_perms and permission in channel_perms[role]:
            filtered.append(channel)

    return filtered


def get_denied_channels(
    channels: List[str],
    role: Optional[str] = None
) -> List[str]:
    """
    Get list of channels from the input that are denied for the role.

    Args:
        channels: List of channel names to check
        role: User role

    Returns:
        List of denied channels
    """
    role = role or "anonymous"
    denied = []

    for channel in channels:
        if not can_subscribe(channel, role):
            denied.append(channel)

    return denied


def get_channel_info_with_permissions(role: Optional[str] = None) -> dict:
    """
    Get channel information with permission indicators for a role.

    Args:
        role: User role

    Returns:
        Dictionary with channel names as keys and permission info as values
    """
    role = role or "anonymous"
    info = {}

    for channel in CHANNEL_PERMISSIONS.keys():
        permissions = {
            "can_subscribe": can_subscribe(channel, role),
            "can_broadcast": can_broadcast(channel, role),
        }
        info[channel] = permissions

    return info
