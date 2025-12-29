
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
WebSocket Message Compression

Utilities for compressing/decompressing WebSocket messages to save bandwidth.
"""

import gzip
import zlib
import json
from typing import Any, Dict, Optional
from enum import Enum

from ..logging_config import get_logger

logger = get_logger(__name__, service="websocket_compression")


class CompressionAlgorithm(str, Enum):
    """Supported compression algorithms."""

    NONE = "none"
    GZIP = "gzip"
    ZLIB = "zlib"
    DEFLATE = "deflate"


class MessageCompressor:
    """
    Compressor for WebSocket messages.

    Automatically compresses/decompresses JSON messages based on size threshold.
    """

    def __init__(
        self,
        algorithm: CompressionAlgorithm = CompressionAlgorithm.GZIP,
        compression_level: int = 6,
        min_size_threshold: int = 1024,  # Compress messages > 1KB
    ):
        """
        Initialize message compressor.

        Args:
            algorithm: Compression algorithm to use
            compression_level: Compression level (1-9, 9 is max)
            min_size_threshold: Minimum message size to compress (bytes)
        """
        self.algorithm = algorithm
        self.compression_level = compression_level
        self.min_size_threshold = min_size_threshold

        logger.info(
            "Message compressor initialized",
            extra={
                "event": "compressor_init",
                "algorithm": algorithm,
                "level": compression_level,
                "threshold": min_size_threshold,
            }
        )

    def compress(self, data: str | bytes) -> bytes:
        """
        Compress data.

        Args:
            data: Data to compress (string or bytes)

        Returns:
            Compressed bytes
        """
        # Convert string to bytes
        if isinstance(data, str):
            data = data.encode('utf-8')

        # Choose compression algorithm
        if self.algorithm == CompressionAlgorithm.GZIP:
            compressed = gzip.compress(data, compresslevel=self.compression_level)
        elif self.algorithm == CompressionAlgorithm.ZLIB:
            compressed = zlib.compress(data, level=self.compression_level)
        elif self.algorithm == CompressionAlgorithm.DEFLATE:
            # Deflate is zlib without header/checksum
            compressor = zlib.compressobj(self.compression_level, zlib.DEFLATED, -zlib.MAX_WBITS)
            compressed = compressor.compress(data) + compressor.flush()
        else:
            # No compression
            compressed = data

        original_size = len(data)
        compressed_size = len(compressed)
        ratio = compressed_size / original_size if original_size > 0 else 1.0

        logger.debug(
            f"Compressed {original_size}B → {compressed_size}B ({ratio:.2%})",
            extra={
                "event": "message_compressed",
                "algorithm": self.algorithm,
                "original_size": original_size,
                "compressed_size": compressed_size,
                "ratio": ratio,
            }
        )

        return compressed

    def decompress(self, data: bytes) -> bytes:
        """
        Decompress data.

        Args:
            data: Compressed data

        Returns:
            Decompressed bytes
        """
        # Choose decompression algorithm
        if self.algorithm == CompressionAlgorithm.GZIP:
            decompressed = gzip.decompress(data)
        elif self.algorithm == CompressionAlgorithm.ZLIB:
            decompressed = zlib.decompress(data)
        elif self.algorithm == CompressionAlgorithm.DEFLATE:
            decompressor = zlib.decompressobj(-zlib.MAX_WBITS)
            decompressed = decompressor.decompress(data) + decompressor.flush()
        else:
            # No decompression
            decompressed = data

        logger.debug(
            f"Decompressed {len(data)}B → {len(decompressed)}B",
            extra={
                "event": "message_decompressed",
                "algorithm": self.algorithm,
                "compressed_size": len(data),
                "decompressed_size": len(decompressed),
            }
        )

        return decompressed

    def should_compress(self, data: str | bytes) -> bool:
        """
        Check if data should be compressed.

        Args:
            data: Data to check

        Returns:
            True if data should be compressed
        """
        if self.algorithm == CompressionAlgorithm.NONE:
            return False

        # Calculate size
        size = len(data.encode('utf-8') if isinstance(data, str) else data)

        # Only compress if above threshold
        return size >= self.min_size_threshold

    def compress_json(self, data: Dict[str, Any]) -> tuple[bytes, bool]:
        """
        Compress JSON data if beneficial.

        Args:
            data: JSON dictionary

        Returns:
            Tuple of (compressed_data: bytes, was_compressed: bool)
        """
        # Convert to JSON string
        json_str = json.dumps(data)

        # Check if should compress
        if not self.should_compress(json_str):
            return json_str.encode('utf-8'), False

        # Compress
        compressed = self.compress(json_str)

        # Check if compression actually helped
        original_size = len(json_str.encode('utf-8'))
        if len(compressed) >= original_size * 0.9:  # Less than 10% saving
            logger.debug(
                "Compression not beneficial, using uncompressed",
                extra={
                    "event": "compression_skipped",
                    "original_size": original_size,
                    "compressed_size": len(compressed),
                }
            )
            return json_str.encode('utf-8'), False

        return compressed, True

    def decompress_json(self, data: bytes) -> Dict[str, Any]:
        """
        Decompress and parse JSON data.

        Args:
            data: Compressed JSON bytes

        Returns:
            Parsed JSON dictionary
        """
        decompressed = self.decompress(data)
        return json.loads(decompressed.decode('utf-8'))


class AdaptiveCompressor:
    """
    Adaptive compressor that chooses best algorithm based on message type.

    Different message types compress better with different algorithms.
    """

    def __init__(self):
        """Initialize adaptive compressor."""
        # Different compressors for different data types
        self.compressors = {
            "text": MessageCompressor(CompressionAlgorithm.GZIP, 6, 512),
            "json": MessageCompressor(CompressionAlgorithm.ZLIB, 5, 1024),
            "binary": MessageCompressor(CompressionAlgorithm.ZLIB, 4, 2048),
        }

        logger.info(
            "Adaptive compressor initialized",
            extra={"event": "adaptive_compressor_init"}
        )

    def compress(
        self,
        data: str | bytes,
        data_type: str = "json"
    ) -> tuple[bytes, CompressionAlgorithm]:
        """
        Compress data using appropriate algorithm.

        Args:
            data: Data to compress
            data_type: Type of data (text, json, binary)

        Returns:
            Tuple of (compressed_data, algorithm_used)
        """
        compressor = self.compressors.get(data_type, self.compressors["json"])

        if not compressor.should_compress(data):
            # Convert to bytes if string
            result = data.encode('utf-8') if isinstance(data, str) else data
            return result, CompressionAlgorithm.NONE

        compressed = compressor.compress(data)
        return compressed, compressor.algorithm

    def decompress(
        self,
        data: bytes,
        algorithm: CompressionAlgorithm
    ) -> bytes:
        """
        Decompress data using specified algorithm.

        Args:
            data: Compressed data
            algorithm: Algorithm used for compression

        Returns:
            Decompressed bytes
        """
        if algorithm == CompressionAlgorithm.NONE:
            return data

        # Find compressor with this algorithm
        for compressor in self.compressors.values():
            if compressor.algorithm == algorithm:
                return compressor.decompress(data)

        # Fallback: use first compressor
        return list(self.compressors.values())[0].decompress(data)


# Global compressor instances
default_compressor = MessageCompressor(
    algorithm=CompressionAlgorithm.GZIP,
    compression_level=6,
    min_size_threshold=1024,
)

adaptive_compressor = AdaptiveCompressor()
