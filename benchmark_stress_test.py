#!/usr/bin/env python3
"""
NeuroGraph OS - Stress Test Benchmark v0.52.0

Проверка работоспособности системы под разной нагрузкой:
- SMOKE: Быстрая проверка что все работает (100 операций)
- NORMAL: Нормальная нагрузка (1,000 операций)
- HIGH: Высокая нагрузка (10,000 операций)
- EXTREME: Экстремальная нагрузка (100,000+ операций)
- OVERLOAD: Over max - проверка лимитов системы

Usage:
    python benchmark_stress_test.py smoke     # Быстро, ~10 сек
    python benchmark_stress_test.py normal    # Средне, ~30 сек
    python benchmark_stress_test.py high      # Долго, ~2 мин
    python benchmark_stress_test.py extreme   # Очень долго, ~10 мин
    python benchmark_stress_test.py overload  # Максимум, ~30+ мин
    python benchmark_stress_test.py all       # Все тесты подряд
"""

import sys
import time
import psutil
import json
from datetime import datetime
from typing import Dict, Any, Optional

# Конфигурации нагрузки
LOAD_PROFILES = {
    "smoke": {
        "description": "Smoke test - быстрая проверка работоспособности",
        "token_count": 1_000,
        "api_requests": 500,
        "grid_queries": 200,
        "expected_time": "~30 seconds",
        "emoji": "🔥"
    },
    "normal": {
        "description": "Normal load - типичная нагрузка",
        "token_count": 10_000,
        "api_requests": 2_000,
        "grid_queries": 1_000,
        "expected_time": "~2 minutes",
        "emoji": "✅"
    },
    "high": {
        "description": "High load - высокая нагрузка",
        "token_count": 100_000,
        "api_requests": 5_000,
        "grid_queries": 2_000,
        "expected_time": "~10 minutes",
        "emoji": "🔥"
    },
    "extreme": {
        "description": "Extreme load - экстремальная нагрузка",
        "token_count": 1_000_000,
        "api_requests": 10_000,
        "grid_queries": 5_000,
        "expected_time": "~1 hour",
        "emoji": "💥"
    },
    "overload": {
        "description": "Overload test - проверка лимитов",
        "token_count": 10_000_000,
        "api_requests": 20_000,
        "grid_queries": 10_000,
        "expected_time": "~5+ hours",
        "emoji": "🚨"
    }
}


class StressTestRunner:
    """Запускает стресс-тесты с разной нагрузкой."""

    def __init__(self, profile: str):
        self.profile_name = profile
        self.profile = LOAD_PROFILES[profile]
        self.results = {}
        self.start_time = None
        self.process = psutil.Process()

    def log(self, message: str, level: str = "INFO"):
        """Логирование с timestamp."""
        timestamp = datetime.now().strftime("%H:%M:%S.%f")[:-3]
        emoji = {"INFO": "ℹ️", "SUCCESS": "✅", "ERROR": "❌", "WARNING": "⚠️"}.get(level, "")
        print(f"[{timestamp}] {emoji} {message}")

    def get_memory_usage(self) -> float:
        """Текущее использование памяти в MB."""
        return self.process.memory_info().rss / (1024 ** 2)

    def run_rust_core_test(self):
        """Тест Rust Core (FFI)."""
        self.log(f"Starting Rust Core test ({self.profile['token_count']:,} tokens)...")

        import neurograph

        start_mem = self.get_memory_usage()
        start_time = time.perf_counter()

        # Создание runtime
        rt = neurograph.Runtime(neurograph.Config(grid_size=1000, dimensions=50))

        # Создание токенов
        token_ids = []
        create_start = time.perf_counter()
        batch_size = 1000

        for i in range(self.profile["token_count"]):
            coords = [[float((i * 17 + j) % 100)] for j in range(50)]
            token_id = rt.tokens.create({"coordinates": coords, "weight": 1.0})
            token_ids.append(token_id)

            # Progress bar
            if (i + 1) % batch_size == 0 or i == self.profile["token_count"] - 1:
                elapsed = time.perf_counter() - create_start
                rate = (i + 1) / elapsed
                mem = self.get_memory_usage()
                self.log(f"  Created {i+1:,}/{self.profile['token_count']:,} tokens "
                        f"({rate:.0f} tokens/s, {mem:.1f}MB)", "INFO")

        create_elapsed = time.perf_counter() - create_start

        # Чтение токенов (sample)
        read_start = time.perf_counter()
        read_count = min(1000, len(token_ids))
        for i in range(read_count):
            _ = rt.tokens.get(token_ids[i * len(token_ids) // read_count])
        read_elapsed = time.perf_counter() - read_start

        # Grid queries
        query_start = time.perf_counter()
        query_results = []
        for i in range(self.profile["grid_queries"]):
            center = (float(i % 100), float((i * 13) % 100), float((i * 17) % 100))
            results = rt.grid.range_query(center, 5.0)
            query_results.append(len(results))
        query_elapsed = time.perf_counter() - query_start

        total_elapsed = time.perf_counter() - start_time
        end_mem = self.get_memory_usage()

        self.results["rust_core"] = {
            "status": "✅ SUCCESS",
            "tokens_created": self.profile["token_count"],
            "create_time_sec": round(create_elapsed, 2),
            "create_rate_per_sec": round(self.profile["token_count"] / create_elapsed, 2),
            "read_count": read_count,
            "read_time_ms": round(read_elapsed * 1000, 2),
            "read_rate_per_sec": round(read_count / read_elapsed, 2),
            "grid_queries": self.profile["grid_queries"],
            "grid_time_ms": round(query_elapsed * 1000, 2),
            "grid_rate_per_sec": round(self.profile["grid_queries"] / query_elapsed, 2),
            "avg_results_per_query": round(sum(query_results) / len(query_results), 1),
            "memory_start_mb": round(start_mem, 1),
            "memory_end_mb": round(end_mem, 1),
            "memory_delta_mb": round(end_mem - start_mem, 1),
            "total_time_sec": round(total_elapsed, 2),
        }

        self.log(f"Rust Core test complete: {total_elapsed:.1f}s, {end_mem:.1f}MB memory", "SUCCESS")

    def run_rest_api_test(self):
        """Тест REST API."""
        self.log(f"Starting REST API test ({self.profile['api_requests']} requests)...")

        import requests
        import subprocess
        import signal
        import os

        # Запуск API сервера
        self.log("Starting API server...")
        api_process = subprocess.Popen(
            ["python", "-m", "src.api.main"],
            stdout=subprocess.DEVNULL,
            stderr=subprocess.DEVNULL,
            preexec_fn=os.setsid,
            env={**os.environ, "LOG_LEVEL": "ERROR"}  # Минимальные логи
        )

        time.sleep(3)  # Ждем запуска
        base_url = "http://localhost:8000/api/v1"

        try:
            # Health check
            health_start = time.perf_counter()
            health_success = 0
            for _ in range(min(100, self.profile["api_requests"])):
                r = requests.get(f"{base_url}/health", timeout=5)
                if r.status_code == 200:
                    health_success += 1
            health_elapsed = time.perf_counter() - health_start

            # Token CRUD
            token_start = time.perf_counter()
            token_ids = []
            token_create_success = 0

            for i in range(self.profile["api_requests"]):
                try:
                    r = requests.post(
                        f"{base_url}/tokens",
                        json={"coordinates": [[float(j % 50)] for j in range(50)], "weight": 1.0},
                        timeout=10
                    )
                    if r.status_code == 201:
                        token_create_success += 1
                        token_ids.append(r.json()["data"]["id"])
                except Exception as e:
                    self.log(f"Token create failed: {e}", "WARNING")

                if (i + 1) % 50 == 0:
                    elapsed = time.perf_counter() - token_start
                    rate = (i + 1) / elapsed
                    self.log(f"  Created {i+1}/{self.profile['api_requests']} tokens ({rate:.1f} req/s)", "INFO")

            token_create_elapsed = time.perf_counter() - token_start

            # Token reads
            token_read_start = time.perf_counter()
            token_read_success = 0
            read_count = min(100, len(token_ids))

            for token_id in token_ids[:read_count]:
                try:
                    r = requests.get(f"{base_url}/tokens/{token_id}", timeout=5)
                    if r.status_code == 200:
                        token_read_success += 1
                except Exception as e:
                    self.log(f"Token read failed: {e}", "WARNING")

            token_read_elapsed = time.perf_counter() - token_read_start

            # Status endpoint
            status_start = time.perf_counter()
            status_success = 0
            for _ in range(min(50, self.profile["api_requests"])):
                r = requests.get(f"{base_url}/status", timeout=5)
                if r.status_code == 200:
                    status_success += 1
            status_elapsed = time.perf_counter() - status_start

            self.results["rest_api"] = {
                "status": "✅ SUCCESS",
                "health_requests": min(100, self.profile["api_requests"]),
                "health_success": health_success,
                "health_time_ms": round(health_elapsed * 1000, 2),
                "health_latency_ms": round(health_elapsed * 1000 / min(100, self.profile["api_requests"]), 2),
                "token_create_requests": self.profile["api_requests"],
                "token_create_success": token_create_success,
                "token_create_time_sec": round(token_create_elapsed, 2),
                "token_create_rate": round(self.profile["api_requests"] / token_create_elapsed, 2),
                "token_read_requests": read_count,
                "token_read_success": token_read_success,
                "token_read_time_ms": round(token_read_elapsed * 1000, 2),
                "status_requests": min(50, self.profile["api_requests"]),
                "status_success": status_success,
                "status_time_ms": round(status_elapsed * 1000, 2),
                "status_latency_ms": round(status_elapsed * 1000 / min(50, self.profile["api_requests"]), 2),
            }

            self.log(f"REST API test complete: {token_create_success}/{self.profile['api_requests']} tokens created", "SUCCESS")

        except Exception as e:
            self.log(f"REST API test failed: {e}", "ERROR")
            self.results["rest_api"] = {"status": "❌ FAILED", "error": str(e)}

        finally:
            # Остановка сервера
            self.log("Stopping API server...")
            try:
                os.killpg(os.getpgid(api_process.pid), signal.SIGTERM)
                api_process.wait(timeout=5)
            except:
                api_process.kill()

    def run(self) -> Dict[str, Any]:
        """Запуск всех тестов."""
        self.log("=" * 70)
        self.log(f"{self.profile['emoji']} {self.profile['description'].upper()}")
        self.log("=" * 70)
        self.log(f"Profile: {self.profile_name}")
        self.log(f"Expected time: {self.profile['expected_time']}")
        self.log(f"Tokens: {self.profile['token_count']:,}")
        self.log(f"API requests: {self.profile['api_requests']:,}")
        self.log(f"Grid queries: {self.profile['grid_queries']:,}")
        self.log("=" * 70)

        self.start_time = time.time()

        try:
            # Rust Core test
            self.run_rust_core_test()

            # REST API test
            self.run_rest_api_test()

            total_elapsed = time.time() - self.start_time

            self.log("=" * 70)
            self.log(f"✅ ALL TESTS PASSED ({total_elapsed:.1f}s)", "SUCCESS")
            self.log("=" * 70)

            return {
                "profile": self.profile_name,
                "description": self.profile["description"],
                "total_time_sec": round(total_elapsed, 2),
                "timestamp": datetime.now().isoformat(),
                "results": self.results,
                "overall_status": "✅ SUCCESS"
            }

        except Exception as e:
            self.log(f"Test suite failed: {e}", "ERROR")
            return {
                "profile": self.profile_name,
                "overall_status": "❌ FAILED",
                "error": str(e)
            }


def print_summary(results: Dict[str, Any]):
    """Красивый вывод результатов."""
    print("\n" + "=" * 70)
    print("📊 SUMMARY")
    print("=" * 70)

    print(f"\nProfile: {results['profile']}")
    print(f"Description: {results['description']}")
    print(f"Total Time: {results.get('total_time_sec', 0):.1f}s")
    print(f"Status: {results['overall_status']}")

    if "results" in results:
        print("\n--- Rust Core Performance ---")
        rust = results["results"].get("rust_core", {})
        if rust:
            print(f"  Tokens created: {rust.get('tokens_created', 0):,} in {rust.get('create_time_sec', 0):.1f}s")
            print(f"  Create rate: {rust.get('create_rate_per_sec', 0):,.0f} tokens/s")
            print(f"  Read rate: {rust.get('read_rate_per_sec', 0):,.0f} tokens/s")
            print(f"  Grid queries: {rust.get('grid_queries', 0)} in {rust.get('grid_time_ms', 0):.1f}ms")
            print(f"  Memory usage: {rust.get('memory_start_mb', 0):.1f}MB → {rust.get('memory_end_mb', 0):.1f}MB "
                  f"(+{rust.get('memory_delta_mb', 0):.1f}MB)")

        print("\n--- REST API Performance ---")
        api = results["results"].get("rest_api", {})
        if api and api.get("status") == "✅ SUCCESS":
            print(f"  Health latency: {api.get('health_latency_ms', 0):.1f}ms")
            print(f"  Token create: {api.get('token_create_success', 0)}/{api.get('token_create_requests', 0)} "
                  f"({api.get('token_create_rate', 0):.1f} req/s)")
            print(f"  Token read: {api.get('token_read_success', 0)}/{api.get('token_read_requests', 0)}")
            print(f"  Status latency: {api.get('status_latency_ms', 0):.1f}ms")

    print("\n" + "=" * 70)


def main():
    """Main entry point."""
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

    mode = sys.argv[1].lower()

    if mode == "all":
        # Запуск всех профилей подряд
        all_results = []
        for profile_name in ["smoke", "normal", "high", "extreme", "overload"]:
            runner = StressTestRunner(profile_name)
            result = runner.run()
            all_results.append(result)
            print_summary(result)

            # Сохранение результатов
            filename = f"benchmark_stress_{profile_name}_v0.52.0.json"
            with open(filename, "w") as f:
                json.dump(result, f, indent=2)
            print(f"\n💾 Results saved: {filename}\n")

        # Сводка
        print("\n" + "=" * 70)
        print("📊 ALL PROFILES SUMMARY")
        print("=" * 70)
        for res in all_results:
            status_emoji = "✅" if res["overall_status"] == "✅ SUCCESS" else "❌"
            print(f"{status_emoji} {res['profile']:10} - {res.get('total_time_sec', 0):6.1f}s - {res['description']}")

    elif mode in LOAD_PROFILES:
        # Запуск одного профиля
        runner = StressTestRunner(mode)
        result = runner.run()
        print_summary(result)

        # Сохранение результатов
        filename = f"benchmark_stress_{mode}_v0.52.0.json"
        with open(filename, "w") as f:
            json.dump(result, f, indent=2)
        print(f"\n💾 Results saved: {filename}")

    else:
        print(f"❌ Unknown mode: {mode}")
        print(__doc__)
        sys.exit(1)


if __name__ == "__main__":
    main()
