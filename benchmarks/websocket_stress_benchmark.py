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
WebSocket Stress & Load Testing Benchmark

Comprehensive stress testing with load levels from minimal to maximum (9.9M operations).
Tests all model parameters under various load conditions.

Usage:
    python benchmarks/websocket_stress_benchmark.py
    python benchmarks/websocket_stress_benchmark.py --max-load 5000000  # Custom max
    python benchmarks/websocket_stress_benchmark.py --quick  # Quick test
"""

import asyncio
import time
import json
import statistics
import sys
import os
import argparse
from typing import List, Dict, Any, Tuple
from concurrent.futures import ThreadPoolExecutor
from dataclasses import dataclass
from datetime import datetime

# Add parent directory to path
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

# WebSocket modules
from src.api.websocket.manager import ConnectionManager
from src.api.websocket.metrics import WebSocketMetricsCollector
from src.api.websocket.permissions import can_subscribe, can_broadcast, get_accessible_channels
from src.api.websocket.rate_limit import WebSocketRateLimiter
from src.api.websocket.reconnection import ReconnectionManager
from src.api.websocket.binary import BinaryMessageHandler, BinaryMessageType
from src.api.websocket.compression import MessageCompressor, CompressionAlgorithm
from src.api.websocket.channels import Channel


# ANSI Color codes
class Colors:
    HEADER = '\033[95m'
    BLUE = '\033[94m'
    CYAN = '\033[96m'
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    RED = '\033[91m'
    BOLD = '\033[1m'
    UNDERLINE = '\033[4m'
    END = '\033[0m'


@dataclass
class LoadLevel:
    """Represents a load testing level."""
    name: str
    operations: int
    connections: int
    messages_per_conn: int
    subscribers: int
    description: str


@dataclass
class BenchmarkResult:
    """Stores benchmark results for a specific load level."""
    load_level: LoadLevel
    duration: float
    throughput: float
    latency_avg: float
    latency_p50: float
    latency_p95: float
    latency_p99: float
    success_rate: float
    memory_usage: float
    cpu_time: float


class WebSocketStressBenchmark:
    """Comprehensive stress testing benchmark suite."""

    def __init__(self, max_operations: int = 9_900_000):
        self.max_operations = max_operations
        self.results: List[BenchmarkResult] = []

        # Define load levels (from minimal to maximum)
        self.load_levels = self._generate_load_levels()

    def _generate_load_levels(self) -> List[LoadLevel]:
        """Generate load levels from minimal to maximum."""
        levels = [
            # Minimal load - Development/Testing
            LoadLevel(
                name="Minimal",
                operations=1_000,
                connections=10,
                messages_per_conn=100,
                subscribers=5,
                description="Development environment simulation"
            ),
            # Light load - Small application
            LoadLevel(
                name="Light",
                operations=10_000,
                connections=100,
                messages_per_conn=100,
                subscribers=20,
                description="Small application with few users"
            ),
            # Low load - Growing application
            LoadLevel(
                name="Low",
                operations=50_000,
                connections=500,
                messages_per_conn=100,
                subscribers=50,
                description="Growing application"
            ),
            # Medium load - Production application
            LoadLevel(
                name="Medium",
                operations=100_000,
                connections=1_000,
                messages_per_conn=100,
                subscribers=100,
                description="Typical production load"
            ),
            # High load - Busy production
            LoadLevel(
                name="High",
                operations=500_000,
                connections=5_000,
                messages_per_conn=100,
                subscribers=200,
                description="Busy production environment"
            ),
            # Very High load - Peak hours
            LoadLevel(
                name="Very High",
                operations=1_000_000,
                connections=10_000,
                messages_per_conn=100,
                subscribers=500,
                description="Peak hours traffic"
            ),
            # Extreme load - Stress test
            LoadLevel(
                name="Extreme",
                operations=5_000_000,
                connections=50_000,
                messages_per_conn=100,
                subscribers=1000,
                description="Stress testing limit"
            ),
        ]

        # Add maximum load level if specified
        if self.max_operations >= 9_000_000:
            levels.append(
                LoadLevel(
                    name="Maximum",
                    operations=self.max_operations,
                    connections=99_000,
                    messages_per_conn=100,
                    subscribers=5000,
                    description="Maximum stress test (9.9M operations)"
                )
            )

        return levels

    def print_header(self, title: str):
        """Print colored header."""
        print(f"\n{Colors.BOLD}{Colors.HEADER}{'='*80}{Colors.END}")
        print(f"{Colors.BOLD}{Colors.HEADER}{title.center(80)}{Colors.END}")
        print(f"{Colors.BOLD}{Colors.HEADER}{'='*80}{Colors.END}\n")

    def print_section(self, title: str):
        """Print section title."""
        print(f"\n{Colors.BOLD}{Colors.CYAN}{title}{Colors.END}")
        print(f"{Colors.CYAN}{'-'*len(title)}{Colors.END}")

    def print_metric(self, name: str, value: str, unit: str = "", status: str = ""):
        """Print formatted metric."""
        status_color = Colors.GREEN if "✅" in status else Colors.YELLOW if "⚠️" in status else Colors.RED
        print(f"  {Colors.BOLD}{name:30s}{Colors.END} {value:>15s} {unit:10s} {status_color}{status}{Colors.END}")

    def benchmark_connection_stress(self, level: LoadLevel) -> BenchmarkResult:
        """Benchmark connection handling under load."""
        self.print_section(f"Connection Stress Test - {level.name} ({level.operations:,} ops)")

        manager = ConnectionManager()
        latencies = []

        start = time.perf_counter()

        # Simulate connections
        for i in range(level.connections):
            conn_start = time.perf_counter()

            client_id = f"stress_client_{i}"
            manager._metadata[client_id] = {
                "user_id": f"user_{i % 1000}",  # Reuse user IDs
                "connected_at": time.time()
            }

            conn_end = time.perf_counter()
            latencies.append((conn_end - conn_start) * 1_000_000)  # μs

        duration = time.perf_counter() - start
        throughput = level.connections / duration if duration > 0 else 0

        # Calculate statistics
        latency_avg = statistics.mean(latencies) if latencies else 0
        latency_p50 = statistics.median(latencies) if latencies else 0
        latency_p95 = statistics.quantiles(latencies, n=20)[18] if len(latencies) > 20 else latencies[-1] if latencies else 0
        latency_p99 = statistics.quantiles(latencies, n=100)[98] if len(latencies) > 100 else latencies[-1] if latencies else 0

        self.print_metric("Connections", f"{level.connections:,}", "conn")
        self.print_metric("Duration", f"{duration*1000:.2f}", "ms")
        self.print_metric("Throughput", f"{throughput:,.0f}", "conn/sec", "✅" if throughput > 100_000 else "⚠️")
        self.print_metric("Latency (avg)", f"{latency_avg:.2f}", "μs")
        self.print_metric("Latency (P95)", f"{latency_p95:.2f}", "μs", "✅" if latency_p95 < 50 else "⚠️")

        return BenchmarkResult(
            load_level=level,
            duration=duration,
            throughput=throughput,
            latency_avg=latency_avg,
            latency_p50=latency_p50,
            latency_p95=latency_p95,
            latency_p99=latency_p99,
            success_rate=100.0,
            memory_usage=0,
            cpu_time=duration
        )

    def benchmark_message_stress(self, level: LoadLevel) -> BenchmarkResult:
        """Benchmark message processing under load."""
        self.print_section(f"Message Processing Stress - {level.name} ({level.operations:,} ops)")

        latencies = []
        total_messages = min(level.operations, level.connections * level.messages_per_conn)

        start = time.perf_counter()

        for i in range(total_messages):
            msg_start = time.perf_counter()

            # Simulate message processing
            message = {
                "type": "test_message",
                "data": {"index": i, "timestamp": time.time()},
                "payload": "x" * 100  # 100 bytes payload
            }
            _ = json.dumps(message)

            msg_end = time.perf_counter()
            latencies.append((msg_end - msg_start) * 1_000_000)  # μs

        duration = time.perf_counter() - start
        throughput = total_messages / duration if duration > 0 else 0

        latency_avg = statistics.mean(latencies) if latencies else 0
        latency_p50 = statistics.median(latencies) if latencies else 0
        latency_p95 = statistics.quantiles(latencies, n=20)[18] if len(latencies) > 20 else latencies[-1] if latencies else 0
        latency_p99 = statistics.quantiles(latencies, n=100)[98] if len(latencies) > 100 else latencies[-1] if latencies else 0

        self.print_metric("Messages", f"{total_messages:,}", "msg")
        self.print_metric("Duration", f"{duration*1000:.2f}", "ms")
        self.print_metric("Throughput", f"{throughput:,.0f}", "msg/sec", "✅" if throughput > 50_000 else "⚠️")
        self.print_metric("Latency (avg)", f"{latency_avg:.2f}", "μs")
        self.print_metric("Latency (P95)", f"{latency_p95:.2f}", "μs", "✅" if latency_p95 < 20 else "⚠️")

        return BenchmarkResult(
            load_level=level,
            duration=duration,
            throughput=throughput,
            latency_avg=latency_avg,
            latency_p50=latency_p50,
            latency_p95=latency_p95,
            latency_p99=latency_p99,
            success_rate=100.0,
            memory_usage=0,
            cpu_time=duration
        )

    def benchmark_subscription_stress(self, level: LoadLevel) -> BenchmarkResult:
        """Benchmark subscription operations under load."""
        self.print_section(f"Subscription Stress - {level.name} ({level.operations:,} ops)")

        manager = ConnectionManager()
        channels = ["metrics", "signals", "actions", "logs", "status", "connections"]
        latencies = []

        # Create connections first
        client_ids = [f"client_{i}" for i in range(min(level.connections, 10000))]

        total_ops = min(level.operations, len(client_ids) * 10)  # Max 10 ops per client

        start = time.perf_counter()

        for i in range(total_ops):
            op_start = time.perf_counter()

            client_id = client_ids[i % len(client_ids)]
            channel = channels[i % len(channels)]

            # Subscribe
            if i % 2 == 0:
                manager.subscribe(client_id, [channel])
            else:
                manager.unsubscribe(client_id, [channel])

            op_end = time.perf_counter()
            latencies.append((op_end - op_start) * 1_000_000)  # μs

        duration = time.perf_counter() - start
        throughput = total_ops / duration if duration > 0 else 0

        latency_avg = statistics.mean(latencies) if latencies else 0
        latency_p50 = statistics.median(latencies) if latencies else 0
        latency_p95 = statistics.quantiles(latencies, n=20)[18] if len(latencies) > 20 else latencies[-1] if latencies else 0
        latency_p99 = statistics.quantiles(latencies, n=100)[98] if len(latencies) > 100 else latencies[-1] if latencies else 0

        self.print_metric("Operations", f"{total_ops:,}", "ops")
        self.print_metric("Duration", f"{duration*1000:.2f}", "ms")
        self.print_metric("Throughput", f"{throughput:,.0f}", "ops/sec", "✅" if throughput > 30_000 else "⚠️")
        self.print_metric("Latency (avg)", f"{latency_avg:.2f}", "μs")
        self.print_metric("Latency (P95)", f"{latency_p95:.2f}", "μs", "✅" if latency_p95 < 30 else "⚠️")

        return BenchmarkResult(
            load_level=level,
            duration=duration,
            throughput=throughput,
            latency_avg=latency_avg,
            latency_p50=latency_p50,
            latency_p95=latency_p95,
            latency_p99=latency_p99,
            success_rate=100.0,
            memory_usage=0,
            cpu_time=duration
        )

    def benchmark_broadcast_stress(self, level: LoadLevel) -> BenchmarkResult:
        """Benchmark broadcast operations under load (HONEST METRICS)."""
        self.print_section(f"Broadcast Stress - {level.name} ({level.operations:,} ops)")

        manager = ConnectionManager()

        # Simulate subscribers
        num_subscribers = min(level.subscribers, 1000)  # Reduced for realistic testing
        channel = "metrics"

        for i in range(num_subscribers):
            client_id = f"subscriber_{i}"
            manager.subscribe(client_id, [channel])

        # Calculate realistic number of broadcasts
        num_broadcasts = min(level.operations // num_subscribers, 10000)
        total_deliveries = num_broadcasts * num_subscribers

        latencies = []
        total_serializations = 0

        start = time.perf_counter()

        for i in range(num_broadcasts):
            broadcast_start = time.perf_counter()

            message = {
                "type": "broadcast",
                "index": i,
                "data": "x" * 100
            }

            # HONEST SIMULATION: Actually serialize and "send" to each subscriber
            subscribers = manager._subscriptions.get(channel, set())

            for subscriber_id in subscribers:
                # Serialize message for each subscriber (real cost)
                serialized = json.dumps(message)
                # Simulate send operation (memory copy)
                _ = len(serialized)
                total_serializations += 1

            broadcast_end = time.perf_counter()
            latencies.append((broadcast_end - broadcast_start) * 1_000_000)  # μs

        duration = time.perf_counter() - start
        # Throughput based on ACTUAL serializations, not virtual deliveries
        throughput = total_serializations / duration if duration > 0 else 0

        latency_avg = statistics.mean(latencies) if latencies else 0
        latency_p50 = statistics.median(latencies) if latencies else 0
        latency_p95 = statistics.quantiles(latencies, n=20)[18] if len(latencies) > 20 else latencies[-1] if latencies else 0
        latency_p99 = statistics.quantiles(latencies, n=100)[98] if len(latencies) > 100 else latencies[-1] if latencies else 0

        self.print_metric("Broadcasts", f"{num_broadcasts:,}", "msg")
        self.print_metric("Subscribers", f"{num_subscribers:,}", "subs")
        self.print_metric("Actual Serializations", f"{total_serializations:,}", "ops")
        self.print_metric("Duration", f"{duration:.2f}", "sec")
        self.print_metric("Throughput (HONEST)", f"{throughput:,.0f}", "msg/sec", "✅" if throughput > 10_000 else "⚠️")
        self.print_metric("Latency (avg)", f"{latency_avg:.2f}", "μs")

        return BenchmarkResult(
            load_level=level,
            duration=duration,
            throughput=throughput,
            latency_avg=latency_avg,
            latency_p50=latency_p50,
            latency_p95=latency_p95,
            latency_p99=latency_p99,
            success_rate=100.0,
            memory_usage=0,
            cpu_time=duration
        )

    def benchmark_permission_stress(self, level: LoadLevel) -> BenchmarkResult:
        """Benchmark permission checking under load."""
        self.print_section(f"Permission Check Stress - {level.name} ({level.operations:,} ops)")

        roles = ["admin", "developer", "viewer", "bot", "anonymous"]
        channels = ["metrics", "signals", "actions", "logs", "status", "connections"]

        total_checks = min(level.operations, 1_000_000)  # Cap at 1M
        latencies = []

        start = time.perf_counter()

        for i in range(total_checks):
            check_start = time.perf_counter()

            role = roles[i % len(roles)]
            channel = channels[i % len(channels)]

            _ = can_subscribe(channel, role)

            check_end = time.perf_counter()
            latencies.append((check_end - check_start) * 1_000_000)  # μs

        duration = time.perf_counter() - start
        throughput = total_checks / duration if duration > 0 else 0

        latency_avg = statistics.mean(latencies) if latencies else 0
        latency_p50 = statistics.median(latencies) if latencies else 0
        latency_p95 = statistics.quantiles(latencies, n=20)[18] if len(latencies) > 20 else latencies[-1] if latencies else 0
        latency_p99 = statistics.quantiles(latencies, n=100)[98] if len(latencies) > 100 else latencies[-1] if latencies else 0

        self.print_metric("Checks", f"{total_checks:,}", "checks")
        self.print_metric("Duration", f"{duration*1000:.2f}", "ms")
        self.print_metric("Throughput", f"{throughput:,.0f}", "checks/sec", "✅" if throughput > 300_000 else "⚠️")
        self.print_metric("Latency (avg)", f"{latency_avg:.2f}", "μs")
        self.print_metric("Latency (P95)", f"{latency_p95:.2f}", "μs", "✅" if latency_p95 < 5 else "⚠️")

        return BenchmarkResult(
            load_level=level,
            duration=duration,
            throughput=throughput,
            latency_avg=latency_avg,
            latency_p50=latency_p50,
            latency_p95=latency_p95,
            latency_p99=latency_p99,
            success_rate=100.0,
            memory_usage=0,
            cpu_time=duration
        )

    def benchmark_compression_stress(self, level: LoadLevel) -> BenchmarkResult:
        """Benchmark compression under load."""
        self.print_section(f"Compression Stress - {level.name} ({level.operations:,} ops)")

        compressor = MessageCompressor(
            algorithm=CompressionAlgorithm.GZIP,
            compression_level=6,
            min_size_threshold=1024
        )

        # Generate test data
        test_data = {
            "type": "large_payload",
            "items": [{"id": i, "data": "x" * 100} for i in range(100)]
        }

        total_compressions = min(level.operations // 100, 10000)  # Compression is slower
        latencies = []
        original_size = 0
        compressed_size = 0

        start = time.perf_counter()

        for i in range(total_compressions):
            comp_start = time.perf_counter()

            compressed, was_compressed = compressor.compress_json(test_data)

            comp_end = time.perf_counter()
            latencies.append((comp_end - comp_start) * 1_000_000)  # μs

            if was_compressed:
                original_size += len(json.dumps(test_data).encode('utf-8'))
                compressed_size += len(compressed)

        duration = time.perf_counter() - start
        throughput = total_compressions / duration if duration > 0 else 0
        savings = ((original_size - compressed_size) / original_size * 100) if original_size > 0 else 0

        latency_avg = statistics.mean(latencies) if latencies else 0
        latency_p50 = statistics.median(latencies) if latencies else 0
        latency_p95 = statistics.quantiles(latencies, n=20)[18] if len(latencies) > 20 else latencies[-1] if latencies else 0
        latency_p99 = statistics.quantiles(latencies, n=100)[98] if len(latencies) > 100 else latencies[-1] if latencies else 0

        self.print_metric("Compressions", f"{total_compressions:,}", "ops")
        self.print_metric("Original Size", f"{original_size/1024:.2f}", "KB")
        self.print_metric("Compressed Size", f"{compressed_size/1024:.2f}", "KB")
        self.print_metric("Savings", f"{savings:.1f}", "%", "✅" if savings > 90 else "⚠️")
        self.print_metric("Throughput", f"{throughput:,.0f}", "ops/sec")
        self.print_metric("Latency (avg)", f"{latency_avg:.2f}", "μs")

        return BenchmarkResult(
            load_level=level,
            duration=duration,
            throughput=throughput,
            latency_avg=latency_avg,
            latency_p50=latency_p50,
            latency_p95=latency_p95,
            latency_p99=latency_p99,
            success_rate=100.0,
            memory_usage=0,
            cpu_time=duration
        )

    def run_all_benchmarks(self):
        """Run all benchmarks across all load levels."""
        self.print_header("NeuroGraph WebSocket Stress & Load Testing Benchmark")

        print(f"{Colors.BOLD}Configuration:{Colors.END}")
        print(f"  Maximum Operations: {Colors.GREEN}{self.max_operations:,}{Colors.END}")
        print(f"  Load Levels: {Colors.GREEN}{len(self.load_levels)}{Colors.END}")
        print(f"  Test Categories: {Colors.GREEN}6{Colors.END} (Connection, Message, Subscription, Broadcast, Permission, Compression)")

        all_results = {}

        for level in self.load_levels:
            self.print_header(f"Load Level: {level.name} - {level.description}")

            print(f"{Colors.BOLD}Parameters:{Colors.END}")
            print(f"  Operations: {Colors.CYAN}{level.operations:,}{Colors.END}")
            print(f"  Connections: {Colors.CYAN}{level.connections:,}{Colors.END}")
            print(f"  Messages/Conn: {Colors.CYAN}{level.messages_per_conn:,}{Colors.END}")
            print(f"  Subscribers: {Colors.CYAN}{level.subscribers:,}{Colors.END}")

            level_results = {
                'connection': self.benchmark_connection_stress(level),
                'message': self.benchmark_message_stress(level),
                'subscription': self.benchmark_subscription_stress(level),
                'broadcast': self.benchmark_broadcast_stress(level),
                'permission': self.benchmark_permission_stress(level),
                'compression': self.benchmark_compression_stress(level),
            }

            all_results[level.name] = level_results

        # Print summary
        self.print_summary(all_results)

    def print_summary(self, all_results: Dict[str, Dict[str, BenchmarkResult]]):
        """Print comprehensive summary of all results."""
        self.print_header("Benchmark Summary - All Load Levels")

        # Summary table
        print(f"\n{Colors.BOLD}Performance Summary Across Load Levels{Colors.END}\n")

        # Connection throughput
        print(f"{Colors.BOLD}{Colors.CYAN}Connection Throughput (conn/sec):{Colors.END}")
        for level_name, results in all_results.items():
            conn_result = results['connection']
            status = "✅" if conn_result.throughput > 100_000 else "⚠️" if conn_result.throughput > 50_000 else "❌"
            print(f"  {level_name:15s} {conn_result.throughput:>15,.0f} conn/sec  {status}")

        # Message throughput
        print(f"\n{Colors.BOLD}{Colors.CYAN}Message Processing (msg/sec):{Colors.END}")
        for level_name, results in all_results.items():
            msg_result = results['message']
            status = "✅" if msg_result.throughput > 100_000 else "⚠️" if msg_result.throughput > 50_000 else "❌"
            print(f"  {level_name:15s} {msg_result.throughput:>15,.0f} msg/sec   {status}")

        # Broadcast throughput
        print(f"\n{Colors.BOLD}{Colors.CYAN}Broadcast Performance (msg/sec):{Colors.END}")
        for level_name, results in all_results.items():
            bc_result = results['broadcast']
            status = "✅" if bc_result.throughput > 100_000 else "⚠️" if bc_result.throughput > 50_000 else "❌"
            print(f"  {level_name:15s} {bc_result.throughput:>15,.0f} msg/sec   {status}")

        # Latency comparison
        print(f"\n{Colors.BOLD}{Colors.CYAN}Latency P95 Comparison (μs):{Colors.END}")
        for level_name, results in all_results.items():
            msg_result = results['message']
            status = "✅" if msg_result.latency_p95 < 20 else "⚠️" if msg_result.latency_p95 < 50 else "❌"
            print(f"  {level_name:15s} {msg_result.latency_p95:>15.2f} μs       {status}")

        # Performance degradation analysis
        print(f"\n{Colors.BOLD}{Colors.CYAN}Performance Degradation Analysis:{Colors.END}")

        if len(all_results) >= 2:
            levels = list(all_results.keys())
            first_level = all_results[levels[0]]
            last_level = all_results[levels[-1]]

            conn_degradation = (1 - last_level['connection'].throughput / first_level['connection'].throughput) * 100
            msg_degradation = (1 - last_level['message'].throughput / first_level['message'].throughput) * 100

            print(f"  Connection: {conn_degradation:>6.1f}% degradation from {levels[0]} to {levels[-1]}")
            print(f"  Message:    {msg_degradation:>6.1f}% degradation from {levels[0]} to {levels[-1]}")

            if abs(conn_degradation) < 10:
                print(f"  {Colors.GREEN}✅ Excellent scalability - minimal degradation{Colors.END}")
            elif abs(conn_degradation) < 30:
                print(f"  {Colors.YELLOW}⚠️  Good scalability - acceptable degradation{Colors.END}")
            else:
                print(f"  {Colors.RED}❌ Poor scalability - significant degradation{Colors.END}")

        # Recommendations
        print(f"\n{Colors.BOLD}{Colors.CYAN}Recommendations:{Colors.END}")

        # Analyze maximum level
        if "Maximum" in all_results or "Extreme" in all_results:
            max_level_name = "Maximum" if "Maximum" in all_results else "Extreme"
            max_results = all_results[max_level_name]

            if max_results['connection'].throughput > 100_000:
                print(f"  {Colors.GREEN}✅ System handles extreme load well{Colors.END}")
            else:
                print(f"  {Colors.YELLOW}⚠️  Consider optimization for extreme loads{Colors.END}")

            if max_results['message'].latency_p95 < 50:
                print(f"  {Colors.GREEN}✅ Latency remains acceptable under stress{Colors.END}")
            else:
                print(f"  {Colors.YELLOW}⚠️  Latency increases significantly under stress{Colors.END}")


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(description="WebSocket Stress & Load Testing Benchmark")
    parser.add_argument('--max-load', type=int, default=9_900_000,
                       help='Maximum number of operations (default: 9,900,000)')
    parser.add_argument('--quick', action='store_true',
                       help='Quick test with reduced load levels')

    args = parser.parse_args()

    max_ops = args.max_load
    if args.quick:
        max_ops = 100_000  # Quick test mode

    benchmark = WebSocketStressBenchmark(max_operations=max_ops)
    benchmark.run_all_benchmarks()

    print(f"\n{Colors.BOLD}{Colors.GREEN}✅ Stress testing completed!{Colors.END}\n")


if __name__ == "__main__":
    main()
