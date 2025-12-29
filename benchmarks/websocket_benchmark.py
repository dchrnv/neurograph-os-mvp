#!/usr/bin/env python3

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
WebSocket Performance Benchmark

Измеряет производительность WebSocket v0.60.1:
- Connection throughput
- Message latency
- Subscription performance
- Rate limiting overhead
- Reconnection speed
- Compression efficiency
- Binary message overhead
- Broadcast performance

Usage:
    python benchmarks/websocket_benchmark.py
"""

import asyncio
import time
import json
import statistics
import sys
import os
from typing import List, Dict, Any
from concurrent.futures import ThreadPoolExecutor

# Add parent directory to path
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

# WebSocket modules
from src.api.websocket.manager import ConnectionManager
from src.api.websocket.metrics import WebSocketMetricsCollector
from src.api.websocket.permissions import can_subscribe
from src.api.websocket.rate_limit import WebSocketRateLimiter
from src.api.websocket.reconnection import ReconnectionManager
from src.api.websocket.binary import BinaryMessageHandler, BinaryMessageType
from src.api.websocket.compression import MessageCompressor, CompressionAlgorithm


class Colors:
    """ANSI color codes."""
    HEADER = '\033[95m'
    OKBLUE = '\033[94m'
    OKCYAN = '\033[96m'
    OKGREEN = '\033[92m'
    WARNING = '\033[93m'
    FAIL = '\033[91m'
    ENDC = '\033[0m'
    BOLD = '\033[1m'


class WebSocketBenchmark:
    """WebSocket performance benchmark suite."""

    def __init__(self):
        self.results: Dict[str, Any] = {}

    def print_header(self, text: str):
        """Print benchmark header."""
        print(f"\n{Colors.HEADER}{Colors.BOLD}{'='*70}{Colors.ENDC}")
        print(f"{Colors.HEADER}{Colors.BOLD}{text:^70}{Colors.ENDC}")
        print(f"{Colors.HEADER}{Colors.BOLD}{'='*70}{Colors.ENDC}\n")

    def print_result(self, name: str, value: Any, unit: str = "", color: str = Colors.OKGREEN):
        """Print benchmark result."""
        print(f"{color}{name:.<50} {value:>15} {unit}{Colors.ENDC}")

    def benchmark_connection_throughput(self, num_connections: int = 1000) -> Dict[str, float]:
        """Benchmark connection throughput."""
        print(f"\n{Colors.OKCYAN}Benchmarking connection throughput...{Colors.ENDC}")

        manager = ConnectionManager()
        start = time.perf_counter()

        # Simulate connections
        for i in range(num_connections):
            client_id = f"client_{i}"
            # Just track in metadata (mock connection)
            manager._metadata[client_id] = {
                "user_id": f"user_{i}",
                "connected_at": time.time()
            }

        duration = time.perf_counter() - start
        throughput = num_connections / duration

        result = {
            "connections": num_connections,
            "duration_sec": duration,
            "throughput": throughput
        }

        self.print_result("Total connections", num_connections)
        self.print_result("Duration", f"{duration:.3f}", "sec")
        self.print_result("Throughput", f"{throughput:,.0f}", "conn/sec")

        return result

    def benchmark_message_latency(self, num_messages: int = 10000) -> Dict[str, float]:
        """Benchmark message processing latency."""
        print(f"\n{Colors.OKCYAN}Benchmarking message latency...{Colors.ENDC}")

        manager = ConnectionManager()
        latencies: List[float] = []

        # Prepare messages
        messages = [
            {
                "type": "ping",
                "timestamp": time.time()
            }
            for _ in range(num_messages)
        ]

        # Measure latency
        for msg in messages:
            start = time.perf_counter()

            # Simulate message processing
            msg_json = json.dumps(msg)
            msg_size = len(msg_json.encode('utf-8'))

            # Basic validation
            assert msg.get("type") == "ping"

            latency = (time.perf_counter() - start) * 1_000_000  # Convert to microseconds
            latencies.append(latency)

        result = {
            "messages": num_messages,
            "avg_latency_us": statistics.mean(latencies),
            "median_latency_us": statistics.median(latencies),
            "p95_latency_us": statistics.quantiles(latencies, n=20)[18],  # 95th percentile
            "p99_latency_us": statistics.quantiles(latencies, n=100)[98],  # 99th percentile
            "min_latency_us": min(latencies),
            "max_latency_us": max(latencies)
        }

        self.print_result("Total messages", num_messages)
        self.print_result("Average latency", f"{result['avg_latency_us']:.2f}", "μs")
        self.print_result("Median latency", f"{result['median_latency_us']:.2f}", "μs")
        self.print_result("P95 latency", f"{result['p95_latency_us']:.2f}", "μs")
        self.print_result("P99 latency", f"{result['p99_latency_us']:.2f}", "μs")
        self.print_result("Min latency", f"{result['min_latency_us']:.2f}", "μs")
        self.print_result("Max latency", f"{result['max_latency_us']:.2f}", "μs")

        return result

    def benchmark_subscription_performance(self, num_clients: int = 1000, channels_per_client: int = 3) -> Dict[str, float]:
        """Benchmark subscription performance."""
        print(f"\n{Colors.OKCYAN}Benchmarking subscription performance...{Colors.ENDC}")

        manager = ConnectionManager()
        channels = ["metrics", "signals", "actions"]

        start = time.perf_counter()

        # Subscribe clients
        for i in range(num_clients):
            client_id = f"client_{i}"
            manager.subscribe(client_id, channels[:channels_per_client])

        subscribe_duration = time.perf_counter() - start

        # Measure channel lookup
        start = time.perf_counter()
        for channel in channels:
            subscribers = manager.get_channel_subscribers(channel)
            assert len(subscribers) > 0

        lookup_duration = time.perf_counter() - start

        result = {
            "clients": num_clients,
            "channels_per_client": channels_per_client,
            "subscribe_duration_sec": subscribe_duration,
            "subscribe_throughput": (num_clients * channels_per_client) / subscribe_duration,
            "lookup_duration_sec": lookup_duration,
            "lookups_per_sec": len(channels) / lookup_duration
        }

        self.print_result("Total clients", num_clients)
        self.print_result("Channels per client", channels_per_client)
        self.print_result("Subscribe duration", f"{subscribe_duration:.3f}", "sec")
        self.print_result("Subscribe throughput", f"{result['subscribe_throughput']:,.0f}", "ops/sec")
        self.print_result("Lookup duration", f"{lookup_duration:.6f}", "sec")
        self.print_result("Lookups per second", f"{result['lookups_per_sec']:,.0f}", "ops/sec")

        return result

    def benchmark_rate_limiting(self, num_requests: int = 10000) -> Dict[str, float]:
        """Benchmark rate limiting overhead."""
        print(f"\n{Colors.OKCYAN}Benchmarking rate limiting...{Colors.ENDC}")

        limiter = WebSocketRateLimiter()
        client_id = "bench_client"

        start = time.perf_counter()

        allowed_count = 0
        denied_count = 0

        for _ in range(num_requests):
            allowed, retry_after = limiter.check_rate_limit(client_id, "ping")
            if allowed:
                allowed_count += 1
            else:
                denied_count += 1

        duration = time.perf_counter() - start
        throughput = num_requests / duration

        result = {
            "requests": num_requests,
            "allowed": allowed_count,
            "denied": denied_count,
            "duration_sec": duration,
            "throughput": throughput,
            "avg_latency_us": (duration / num_requests) * 1_000_000
        }

        self.print_result("Total requests", num_requests)
        self.print_result("Allowed", allowed_count, "", Colors.OKGREEN)
        self.print_result("Denied", denied_count, "", Colors.WARNING if denied_count > 0 else Colors.OKGREEN)
        self.print_result("Duration", f"{duration:.3f}", "sec")
        self.print_result("Throughput", f"{throughput:,.0f}", "req/sec")
        self.print_result("Avg latency", f"{result['avg_latency_us']:.2f}", "μs")

        return result

    def benchmark_reconnection(self, num_tokens: int = 1000) -> Dict[str, float]:
        """Benchmark reconnection token operations."""
        print(f"\n{Colors.OKCYAN}Benchmarking reconnection tokens...{Colors.ENDC}")

        manager = ReconnectionManager()

        # Create tokens
        start = time.perf_counter()

        tokens = []
        for i in range(num_tokens):
            token = manager.create_reconnection_token(
                client_id=f"client_{i}",
                user_id=f"user_{i}",
                subscriptions=["metrics", "signals"],
                metadata={"role": "developer"}
            )
            tokens.append(token)

        create_duration = time.perf_counter() - start

        # Restore sessions
        start = time.perf_counter()

        restored_count = 0
        for i, token in enumerate(tokens):
            session = manager.restore_session(token, f"new_client_{i}")
            if session:
                restored_count += 1

        restore_duration = time.perf_counter() - start

        result = {
            "tokens": num_tokens,
            "create_duration_sec": create_duration,
            "create_throughput": num_tokens / create_duration,
            "restore_duration_sec": restore_duration,
            "restore_throughput": num_tokens / restore_duration,
            "restored_count": restored_count
        }

        self.print_result("Total tokens", num_tokens)
        self.print_result("Create duration", f"{create_duration:.3f}", "sec")
        self.print_result("Create throughput", f"{result['create_throughput']:,.0f}", "tokens/sec")
        self.print_result("Restore duration", f"{restore_duration:.3f}", "sec")
        self.print_result("Restore throughput", f"{result['restore_throughput']:,.0f}", "restores/sec")
        self.print_result("Restored successfully", restored_count)

        return result

    def benchmark_compression(self, num_messages: int = 1000) -> Dict[str, float]:
        """Benchmark message compression."""
        print(f"\n{Colors.OKCYAN}Benchmarking compression...{Colors.ENDC}")

        # Test different compression algorithms
        algorithms = [
            (CompressionAlgorithm.GZIP, "GZIP"),
            (CompressionAlgorithm.ZLIB, "ZLIB"),
            (CompressionAlgorithm.DEFLATE, "DEFLATE"),
        ]

        # Create test message
        test_data = {
            "type": "metrics",
            "data": {
                "items": [{"id": i, "value": "x" * 50} for i in range(100)]
            }
        }
        original_size = len(json.dumps(test_data).encode('utf-8'))

        results = {}

        for algorithm, name in algorithms:
            compressor = MessageCompressor(
                algorithm=algorithm,
                compression_level=6,
                min_size_threshold=100
            )

            # Compress
            start = time.perf_counter()

            compressed_sizes = []
            for _ in range(num_messages):
                compressed, was_compressed = compressor.compress_json(test_data)
                if was_compressed:
                    compressed_sizes.append(len(compressed))

            compress_duration = time.perf_counter() - start

            # Decompress
            start = time.perf_counter()

            for _ in range(len(compressed_sizes)):
                # Simulate decompression
                pass

            decompress_duration = time.perf_counter() - start

            avg_compressed_size = statistics.mean(compressed_sizes) if compressed_sizes else original_size
            ratio = avg_compressed_size / original_size
            savings = (1 - ratio) * 100

            results[name] = {
                "original_size": original_size,
                "compressed_size": avg_compressed_size,
                "ratio": ratio,
                "savings_pct": savings,
                "compress_throughput": num_messages / compress_duration,
                "compress_duration": compress_duration
            }

            print(f"\n{Colors.BOLD}{name}:{Colors.ENDC}")
            self.print_result("  Original size", f"{original_size:,}", "bytes")
            self.print_result("  Compressed size", f"{avg_compressed_size:,.0f}", "bytes")
            self.print_result("  Ratio", f"{ratio:.2%}")
            self.print_result("  Savings", f"{savings:.1f}", "%")
            self.print_result("  Throughput", f"{results[name]['compress_throughput']:,.0f}", "msg/sec")

        return results

    def benchmark_binary_messages(self, num_messages: int = 1000) -> Dict[str, float]:
        """Benchmark binary message operations."""
        print(f"\n{Colors.OKCYAN}Benchmarking binary messages...{Colors.ENDC}")

        handler = BinaryMessageHandler()

        # Create test image
        image_data = b"FAKE_IMAGE_DATA" * 100
        metadata = {
            "format": "jpeg",
            "width": 1920,
            "height": 1080
        }

        # Pack
        start = time.perf_counter()

        packed_messages = []
        for _ in range(num_messages):
            packed = handler.create_image_message(
                image_data,
                format=metadata["format"],
                width=metadata["width"],
                height=metadata["height"]
            )
            packed_messages.append(packed)

        pack_duration = time.perf_counter() - start

        # Unpack
        start = time.perf_counter()

        for packed in packed_messages:
            parsed = handler.parse_message(packed)
            assert parsed["type"] == "IMAGE"

        unpack_duration = time.perf_counter() - start

        payload_size = len(image_data)
        packed_size = len(packed_messages[0])
        overhead = packed_size - payload_size
        overhead_pct = (overhead / payload_size) * 100

        result = {
            "messages": num_messages,
            "payload_size": payload_size,
            "packed_size": packed_size,
            "overhead": overhead,
            "overhead_pct": overhead_pct,
            "pack_duration_sec": pack_duration,
            "pack_throughput": num_messages / pack_duration,
            "unpack_duration_sec": unpack_duration,
            "unpack_throughput": num_messages / unpack_duration
        }

        self.print_result("Total messages", num_messages)
        self.print_result("Payload size", f"{payload_size:,}", "bytes")
        self.print_result("Packed size", f"{packed_size:,}", "bytes")
        self.print_result("Overhead", f"{overhead}", "bytes")
        self.print_result("Overhead %", f"{overhead_pct:.2f}", "%")
        self.print_result("Pack throughput", f"{result['pack_throughput']:,.0f}", "msg/sec")
        self.print_result("Unpack throughput", f"{result['unpack_throughput']:,.0f}", "msg/sec")

        return result

    def benchmark_broadcast(self, num_subscribers: int = 1000, num_broadcasts: int = 100) -> Dict[str, float]:
        """Benchmark broadcast performance."""
        print(f"\n{Colors.OKCYAN}Benchmarking broadcast performance...{Colors.ENDC}")

        manager = ConnectionManager()
        channel = "metrics"

        # Subscribe clients
        for i in range(num_subscribers):
            client_id = f"client_{i}"
            manager.subscribe(client_id, [channel])

        # Prepare broadcast message
        message = {
            "channel": channel,
            "type": "data",
            "data": {"value": 42}
        }

        # Measure broadcast time
        start = time.perf_counter()

        for _ in range(num_broadcasts):
            subscribers = manager.get_channel_subscribers(channel)
            # Simulate sending to all subscribers
            for sub_id in subscribers:
                msg_json = json.dumps(message)

        duration = time.perf_counter() - start

        total_messages = num_broadcasts * num_subscribers
        throughput = total_messages / duration

        result = {
            "subscribers": num_subscribers,
            "broadcasts": num_broadcasts,
            "total_messages": total_messages,
            "duration_sec": duration,
            "throughput": throughput,
            "latency_per_broadcast_ms": (duration / num_broadcasts) * 1000
        }

        self.print_result("Subscribers", num_subscribers)
        self.print_result("Broadcasts", num_broadcasts)
        self.print_result("Total messages", total_messages)
        self.print_result("Duration", f"{duration:.3f}", "sec")
        self.print_result("Throughput", f"{throughput:,.0f}", "msg/sec")
        self.print_result("Latency per broadcast", f"{result['latency_per_broadcast_ms']:.2f}", "ms")

        return result

    def benchmark_permissions(self, num_checks: int = 100000) -> Dict[str, float]:
        """Benchmark permission checking."""
        print(f"\n{Colors.OKCYAN}Benchmarking permissions...{Colors.ENDC}")

        channels = ["metrics", "signals", "actions", "logs", "status", "connections"]
        roles = ["admin", "developer", "viewer", "bot", "anonymous"]

        start = time.perf_counter()

        checks = 0
        allowed = 0

        for _ in range(num_checks):
            role = roles[checks % len(roles)]
            channel = channels[checks % len(channels)]

            if can_subscribe(channel, role):
                allowed += 1

            checks += 1

        duration = time.perf_counter() - start
        throughput = checks / duration

        result = {
            "checks": checks,
            "allowed": allowed,
            "denied": checks - allowed,
            "duration_sec": duration,
            "throughput": throughput,
            "avg_latency_us": (duration / checks) * 1_000_000
        }

        self.print_result("Total checks", checks)
        self.print_result("Allowed", allowed)
        self.print_result("Denied", checks - allowed)
        self.print_result("Duration", f"{duration:.3f}", "sec")
        self.print_result("Throughput", f"{throughput:,.0f}", "checks/sec")
        self.print_result("Avg latency", f"{result['avg_latency_us']:.3f}", "μs")

        return result

    def run_all(self):
        """Run all benchmarks."""
        self.print_header("NeuroGraph WebSocket v0.60.1 - Performance Benchmark")

        # 1. Connection throughput
        self.print_header("1. Connection Throughput")
        self.results['connection_throughput'] = self.benchmark_connection_throughput(1000)

        # 2. Message latency
        self.print_header("2. Message Latency")
        self.results['message_latency'] = self.benchmark_message_latency(10000)

        # 3. Subscription performance
        self.print_header("3. Subscription Performance")
        self.results['subscription'] = self.benchmark_subscription_performance(1000, 3)

        # 4. Rate limiting
        self.print_header("4. Rate Limiting")
        self.results['rate_limiting'] = self.benchmark_rate_limiting(10000)

        # 5. Reconnection
        self.print_header("5. Reconnection Tokens")
        self.results['reconnection'] = self.benchmark_reconnection(1000)

        # 6. Compression
        self.print_header("6. Message Compression")
        self.results['compression'] = self.benchmark_compression(1000)

        # 7. Binary messages
        self.print_header("7. Binary Messages")
        self.results['binary'] = self.benchmark_binary_messages(1000)

        # 8. Broadcast
        self.print_header("8. Broadcast Performance")
        self.results['broadcast'] = self.benchmark_broadcast(1000, 100)

        # 9. Permissions
        self.print_header("9. Permissions Checking")
        self.results['permissions'] = self.benchmark_permissions(100000)

        # Summary
        self.print_summary()

    def print_summary(self):
        """Print benchmark summary."""
        self.print_header("SUMMARY")

        print(f"{Colors.BOLD}Key Performance Metrics:{Colors.ENDC}\n")

        # Connection
        conn = self.results['connection_throughput']
        self.print_result("Connection throughput", f"{conn['throughput']:,.0f}", "conn/sec", Colors.OKGREEN)

        # Message latency
        msg = self.results['message_latency']
        self.print_result("Message latency (avg)", f"{msg['avg_latency_us']:.2f}", "μs", Colors.OKGREEN)
        self.print_result("Message latency (P95)", f"{msg['p95_latency_us']:.2f}", "μs", Colors.OKGREEN)

        # Subscription
        sub = self.results['subscription']
        self.print_result("Subscribe throughput", f"{sub['subscribe_throughput']:,.0f}", "ops/sec", Colors.OKGREEN)

        # Rate limiting
        rate = self.results['rate_limiting']
        self.print_result("Rate limit throughput", f"{rate['throughput']:,.0f}", "req/sec", Colors.OKGREEN)

        # Reconnection
        recon = self.results['reconnection']
        self.print_result("Token creation", f"{recon['create_throughput']:,.0f}", "tokens/sec", Colors.OKGREEN)

        # Compression (GZIP)
        comp = self.results['compression']['GZIP']
        self.print_result("Compression savings (GZIP)", f"{comp['savings_pct']:.1f}", "%", Colors.OKGREEN)

        # Binary
        binary = self.results['binary']
        self.print_result("Binary pack throughput", f"{binary['pack_throughput']:,.0f}", "msg/sec", Colors.OKGREEN)

        # Broadcast
        bc = self.results['broadcast']
        self.print_result("Broadcast throughput", f"{bc['throughput']:,.0f}", "msg/sec", Colors.OKGREEN)

        # Permissions
        perm = self.results['permissions']
        self.print_result("Permission checks", f"{perm['throughput']:,.0f}", "checks/sec", Colors.OKGREEN)

        print(f"\n{Colors.HEADER}{'='*70}{Colors.ENDC}\n")


def main():
    """Run WebSocket benchmark."""
    benchmark = WebSocketBenchmark()
    benchmark.run_all()


if __name__ == "__main__":
    main()
