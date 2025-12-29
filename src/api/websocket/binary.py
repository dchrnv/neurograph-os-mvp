
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
WebSocket Binary Message Support

Support for binary WebSocket messages (images, audio, video, etc.)
"""

import struct
import json
from typing import Optional, Dict, Any
from enum import IntEnum

from ..logging_config import get_logger

logger = get_logger(__name__, service="websocket_binary")


class BinaryMessageType(IntEnum):
    """Binary message types."""

    IMAGE = 0x01
    AUDIO = 0x02
    VIDEO = 0x03
    BINARY_DATA = 0x04
    COMPRESSED_JSON = 0x05


class BinaryMessageFormat:
    """
    Binary message format for WebSocket.

    Format:
    [Header: 8 bytes][Metadata: variable][Payload: variable]

    Header structure:
    - Version (1 byte): Protocol version
    - Type (1 byte): Message type (BinaryMessageType)
    - Metadata length (2 bytes): Length of metadata section
    - Payload length (4 bytes): Length of payload section

    Metadata: JSON string with message metadata
    Payload: Raw binary data
    """

    VERSION = 1
    HEADER_SIZE = 8

    @staticmethod
    def pack(
        message_type: BinaryMessageType,
        payload: bytes,
        metadata: Optional[Dict[str, Any]] = None,
    ) -> bytes:
        """
        Pack binary message with header.

        Args:
            message_type: Type of binary message
            payload: Binary payload data
            metadata: Optional metadata dictionary

        Returns:
            Packed binary message
        """
        # Encode metadata as JSON
        metadata_json = json.dumps(metadata or {}).encode('utf-8')
        metadata_len = len(metadata_json)
        payload_len = len(payload)

        # Pack header
        header = struct.pack(
            '>BBHI',  # Big-endian: byte, byte, short, int
            BinaryMessageFormat.VERSION,
            message_type,
            metadata_len,
            payload_len,
        )

        # Combine header + metadata + payload
        message = header + metadata_json + payload

        logger.debug(
            f"Packed binary message: type={message_type.name}, "
            f"metadata={metadata_len}B, payload={payload_len}B",
            extra={
                "event": "binary_message_packed",
                "message_type": message_type.name,
                "metadata_size": metadata_len,
                "payload_size": payload_len,
                "total_size": len(message),
            }
        )

        return message

    @staticmethod
    def unpack(data: bytes) -> tuple[BinaryMessageType, bytes, Dict[str, Any]]:
        """
        Unpack binary message.

        Args:
            data: Binary message data

        Returns:
            Tuple of (message_type, payload, metadata)

        Raises:
            ValueError: If message format is invalid
        """
        if len(data) < BinaryMessageFormat.HEADER_SIZE:
            raise ValueError("Binary message too short")

        # Unpack header
        version, msg_type, metadata_len, payload_len = struct.unpack(
            '>BBHI',
            data[:BinaryMessageFormat.HEADER_SIZE]
        )

        # Validate version
        if version != BinaryMessageFormat.VERSION:
            raise ValueError(f"Unsupported binary message version: {version}")

        # Validate message type
        try:
            message_type = BinaryMessageType(msg_type)
        except ValueError:
            raise ValueError(f"Invalid binary message type: {msg_type}")

        # Extract metadata
        metadata_start = BinaryMessageFormat.HEADER_SIZE
        metadata_end = metadata_start + metadata_len

        if len(data) < metadata_end:
            raise ValueError("Binary message truncated (metadata)")

        metadata_json = data[metadata_start:metadata_end]
        try:
            metadata = json.loads(metadata_json.decode('utf-8'))
        except (json.JSONDecodeError, UnicodeDecodeError) as e:
            raise ValueError(f"Invalid metadata JSON: {e}")

        # Extract payload
        payload_start = metadata_end
        payload_end = payload_start + payload_len

        if len(data) < payload_end:
            raise ValueError("Binary message truncated (payload)")

        payload = data[payload_start:payload_end]

        logger.debug(
            f"Unpacked binary message: type={message_type.name}, "
            f"metadata={metadata_len}B, payload={payload_len}B",
            extra={
                "event": "binary_message_unpacked",
                "message_type": message_type.name,
                "metadata_size": metadata_len,
                "payload_size": payload_len,
            }
        )

        return message_type, payload, metadata


class BinaryMessageHandler:
    """
    Handler for binary WebSocket messages.

    Provides utilities for encoding/decoding different binary message types.
    """

    @staticmethod
    def create_image_message(
        image_data: bytes,
        format: str = "jpeg",
        width: Optional[int] = None,
        height: Optional[int] = None,
        **metadata
    ) -> bytes:
        """
        Create binary message for image data.

        Args:
            image_data: Image bytes
            format: Image format (jpeg, png, etc.)
            width: Image width
            height: Image height
            **metadata: Additional metadata

        Returns:
            Packed binary message
        """
        meta = {
            "format": format,
            "width": width,
            "height": height,
            **metadata,
        }

        return BinaryMessageFormat.pack(
            BinaryMessageType.IMAGE,
            image_data,
            meta
        )

    @staticmethod
    def create_audio_message(
        audio_data: bytes,
        format: str = "wav",
        sample_rate: Optional[int] = None,
        channels: Optional[int] = None,
        **metadata
    ) -> bytes:
        """
        Create binary message for audio data.

        Args:
            audio_data: Audio bytes
            format: Audio format (wav, mp3, etc.)
            sample_rate: Sample rate (Hz)
            channels: Number of audio channels
            **metadata: Additional metadata

        Returns:
            Packed binary message
        """
        meta = {
            "format": format,
            "sample_rate": sample_rate,
            "channels": channels,
            **metadata,
        }

        return BinaryMessageFormat.pack(
            BinaryMessageType.AUDIO,
            audio_data,
            meta
        )

    @staticmethod
    def create_video_message(
        video_data: bytes,
        format: str = "mp4",
        width: Optional[int] = None,
        height: Optional[int] = None,
        fps: Optional[float] = None,
        **metadata
    ) -> bytes:
        """
        Create binary message for video data.

        Args:
            video_data: Video bytes
            format: Video format (mp4, webm, etc.)
            width: Video width
            height: Video height
            fps: Frames per second
            **metadata: Additional metadata

        Returns:
            Packed binary message
        """
        meta = {
            "format": format,
            "width": width,
            "height": height,
            "fps": fps,
            **metadata,
        }

        return BinaryMessageFormat.pack(
            BinaryMessageType.VIDEO,
            video_data,
            meta
        )

    @staticmethod
    def create_binary_data_message(
        data: bytes,
        **metadata
    ) -> bytes:
        """
        Create generic binary data message.

        Args:
            data: Binary data
            **metadata: Metadata

        Returns:
            Packed binary message
        """
        return BinaryMessageFormat.pack(
            BinaryMessageType.BINARY_DATA,
            data,
            metadata
        )

    @staticmethod
    def create_compressed_json_message(
        json_data: Dict[str, Any],
        compression: str = "gzip"
    ) -> bytes:
        """
        Create compressed JSON message.

        Args:
            json_data: JSON data to compress
            compression: Compression algorithm (gzip)

        Returns:
            Packed binary message with compressed JSON
        """
        import gzip

        # Convert to JSON string
        json_str = json.dumps(json_data).encode('utf-8')

        # Compress
        if compression == "gzip":
            compressed = gzip.compress(json_str)
        else:
            raise ValueError(f"Unsupported compression: {compression}")

        metadata = {
            "compression": compression,
            "original_size": len(json_str),
            "compressed_size": len(compressed),
        }

        return BinaryMessageFormat.pack(
            BinaryMessageType.COMPRESSED_JSON,
            compressed,
            metadata
        )

    @staticmethod
    def parse_message(data: bytes) -> Dict[str, Any]:
        """
        Parse binary message and return structured data.

        Args:
            data: Binary message data

        Returns:
            Dictionary with parsed message data
        """
        message_type, payload, metadata = BinaryMessageFormat.unpack(data)

        result = {
            "type": message_type.name,
            "payload": payload,
            "metadata": metadata,
        }

        # Special handling for compressed JSON
        if message_type == BinaryMessageType.COMPRESSED_JSON:
            import gzip
            compression = metadata.get("compression", "gzip")

            if compression == "gzip":
                decompressed = gzip.decompress(payload)
                result["json_data"] = json.loads(decompressed.decode('utf-8'))

        return result


# Global binary message handler
binary_handler = BinaryMessageHandler()
