# WebSocket Stress Testing Guide

**Version:** v0.60.1
**Tool:** `websocket_stress_benchmark.py`
**Date:** 2025-12-29

---

## Overview

Comprehensive stress and load testing tool designed to evaluate WebSocket performance across **8 load levels** from minimal (1K operations) to maximum (9.9M operations).

### Test Coverage

The benchmark tests **6 critical performance dimensions**:
1. **Connection Management** - Connection establishment throughput
2. **Message Processing** - Message handling latency and throughput
3. **Subscription Operations** - Subscribe/unsubscribe performance
4. **Broadcast Performance** - Multi-subscriber message delivery
5. **Permission Checking** - RBAC overhead
6. **Compression** - GZIP compression efficiency

---

## Load Levels

### 1. Minimal (1,000 ops)
```
Operations:     1,000
Connections:    10
Messages/Conn:  100
Subscribers:    5
```
**Purpose:** Development and unit testing environment
**Use Case:** Local development, debugging

### 2. Light (10,000 ops)
```
Operations:     10,000
Connections:    100
Messages/Conn:  100
Subscribers:    20
```
**Purpose:** Small application with few concurrent users
**Use Case:** MVP, beta testing, small teams

### 3. Low (50,000 ops)
```
Operations:     50,000
Connections:    500
Messages/Conn:  100
Subscribers:    50
```
**Purpose:** Growing application
**Use Case:** Early production, increasing user base

### 4. Medium (100,000 ops)
```
Operations:     100,000
Connections:    1,000
Messages/Conn:  100
Subscribers:    100
```
**Purpose:** Typical production load
**Use Case:** Stable production with steady traffic

### 5. High (500,000 ops)
```
Operations:     500,000
Connections:    5,000
Messages/Conn:  100
Subscribers:    200
```
**Purpose:** Busy production environment
**Use Case:** High-traffic application, peak hours

### 6. Very High (1,000,000 ops)
```
Operations:     1,000,000
Connections:    10,000
Messages/Conn:  100
Subscribers:    500
```
**Purpose:** Peak hour traffic surge
**Use Case:** Major events, viral content, marketing campaigns

### 7. Extreme (5,000,000 ops)
```
Operations:     5,000,000
Connections:    50,000
Messages/Conn:  100
Subscribers:    1,000
```
**Purpose:** Stress testing limit
**Use Case:** Capacity planning, identifying bottlenecks

### 8. Maximum (9,900,000 ops)
```
Operations:     9,900,000
Connections:    99,000
Messages/Conn:  100
Subscribers:    5,000
```
**Purpose:** Ultimate stress test
**Use Case:** Breaking point analysis, extreme capacity validation

---

## Usage

### Basic Usage

```bash
# Run full benchmark (9.9M operations)
python benchmarks/websocket_stress_benchmark.py

# Quick test (100K operations)
python benchmarks/websocket_stress_benchmark.py --quick

# Custom maximum load
python benchmarks/websocket_stress_benchmark.py --max-load 5000000
```

### Command-Line Options

```
--max-load INT    Maximum operations (default: 9,900,000)
--quick           Quick test mode (max 100,000 ops)
```

### Output

The benchmark outputs:
- Real-time progress for each load level
- Color-coded results (✅ pass, ⚠️ warning, ❌ fail)
- Detailed metrics (throughput, latency, P95, P99)
- Summary comparison across all load levels
- Performance degradation analysis
- Recommendations

---

## Performance Metrics

### Connection Throughput

**Measurement:** Connections established per second

**Thresholds:**
- ✅ Excellent: > 500,000 conn/sec
- ⚠️ Good: > 100,000 conn/sec
- ❌ Poor: < 100,000 conn/sec

**Example Results:**
```
Minimal     436,053 conn/sec   ✅
Light       734,835 conn/sec   ✅
Medium      681,099 conn/sec   ✅
Extreme     299,501 conn/sec   ✅
```

### Message Processing

**Measurement:** Messages processed per second

**Thresholds:**
- ✅ Excellent: > 100,000 msg/sec
- ⚠️ Good: > 50,000 msg/sec
- ❌ Poor: < 50,000 msg/sec

**Example Results:**
```
Minimal     115,830 msg/sec    ✅
Light       126,429 msg/sec    ✅
Medium      127,894 msg/sec    ✅
Extreme     121,587 msg/sec    ✅
```

### Broadcast Performance

**Measurement:** Total message deliveries per second

**Thresholds:**
- ✅ Excellent: > 100,000 msg/sec
- ⚠️ Good: > 50,000 msg/sec
- ❌ Poor: < 50,000 msg/sec

**Example Results:**
```
Minimal       5,424,906 msg/sec   ✅
Medium      103,493,206 msg/sec   ✅
Extreme     823,921,269 msg/sec   ✅
```

**Note:** Broadcast throughput increases with subscriber count (expected behavior)

### Latency (P95)

**Measurement:** 95th percentile message processing latency

**Thresholds:**
- ✅ Excellent: < 20 μs
- ⚠️ Good: < 50 μs
- ❌ Poor: > 50 μs

**Example Results:**
```
Minimal      9.86 μs    ✅
Medium       9.18 μs    ✅
Extreme     10.80 μs    ✅
```

### Permission Checks

**Measurement:** RBAC permission checks per second

**Thresholds:**
- ✅ Excellent: > 300,000 checks/sec
- ⚠️ Good: > 100,000 checks/sec
- ❌ Poor: < 100,000 checks/sec

**Example Results:**
```
Minimal     511,245 checks/sec   ✅
Medium      541,006 checks/sec   ✅
Extreme     526,415 checks/sec   ✅
```

### Compression

**Measurement:** Compression savings percentage and throughput

**Thresholds:**
- ✅ Excellent: > 90% savings
- ⚠️ Good: > 70% savings
- ❌ Poor: < 70% savings

**Example Results:**
```
All Levels:  97.1% savings    ✅
Throughput:  3,000-3,400 ops/sec
```

---

## Performance Analysis

### Scalability Assessment

The benchmark calculates **performance degradation** from minimal to maximum load:

```
Degradation = (1 - Max_Throughput / Min_Throughput) × 100%
```

**Interpretation:**
- **< 10%** - ✅ Excellent scalability (near-linear)
- **10-30%** - ⚠️ Good scalability (acceptable degradation)
- **> 30%** - ❌ Poor scalability (significant degradation)

**Example:**
```
Connection: 31.3% degradation from Minimal to Extreme
Message:    -5.0% degradation from Minimal to Extreme (improved!)
```

**Note:** Negative degradation indicates performance improvement at scale (e.g., batch processing benefits)

### Bottleneck Identification

Monitor which metric degrades most:
1. **Connection degradation** → Network/socket limits
2. **Message degradation** → CPU/JSON processing
3. **Subscription degradation** → Memory/data structures
4. **Broadcast degradation** → Network/bandwidth
5. **Permission degradation** → Dictionary lookups
6. **Compression degradation** → CPU/algorithm efficiency

---

## Interpreting Results

### Green Checkmarks ✅

**Meaning:** Performance exceeds production thresholds
**Action:** No action needed, system performing well

### Yellow Warnings ⚠️

**Meaning:** Performance acceptable but below optimal
**Action:**
- Monitor in production
- Consider optimization if traffic increases
- Review system resources

### Red Crosses ❌

**Meaning:** Performance below production requirements
**Action:**
- Immediate investigation required
- Optimize code or infrastructure
- Consider horizontal scaling

---

## Production Recommendations

### Based on Load Level

**Light to Medium (100K ops):**
- ✅ Single server sufficient
- ✅ Vertical scaling if needed
- ✅ Monitor CPU/memory

**High to Very High (1M ops):**
- ⚠️ Consider horizontal scaling
- ⚠️ Load balancer with sticky sessions
- ⚠️ Redis for shared state
- ⚠️ Monitor network bandwidth

**Extreme to Maximum (5-10M ops):**
- ❌ Kubernetes cluster required
- ❌ Auto-scaling essential
- ❌ Distributed architecture
- ❌ CDN for static assets
- ❌ Database read replicas

### Infrastructure Sizing

**Single Server Capacity (AWS c6i.2xlarge):**
```
Concurrent Connections:  10,000
Messages/sec:            100,000
Subscribers:             1,000
CPU:                     8 vCPU
RAM:                     16 GB
Cost:                    ~$245/month
```

**10-Server Cluster:**
```
Concurrent Connections:  100,000
Messages/sec:            1,000,000
Subscribers:             10,000
Cost:                    ~$2,450/month
```

**100-Server Cluster (Maximum Load):**
```
Concurrent Connections:  1,000,000
Messages/sec:            10,000,000
Subscribers:             100,000
Cost:                    ~$24,500/month
```

---

## Optimization Tips

### Connection Optimization

1. **Enable connection pooling**
   ```python
   manager = ConnectionManager(pool_size=1000)
   ```

2. **Tune kernel parameters**
   ```bash
   sysctl -w net.core.somaxconn=4096
   sysctl -w net.ipv4.tcp_max_syn_backlog=4096
   sysctl -w net.ipv4.ip_local_port_range="10000 65535"
   ```

3. **Use uvloop for asyncio**
   ```bash
   pip install uvloop
   ```

### Message Optimization

1. **Enable compression for large messages**
   ```python
   compressor = MessageCompressor(min_size_threshold=1024)
   ```

2. **Use binary protocol for media**
   ```python
   handler = BinaryMessageHandler()
   message = handler.pack_image(image_data, {"format": "jpeg"})
   ```

3. **Batch message processing**
   ```python
   async def process_batch(messages):
       await asyncio.gather(*[process(msg) for msg in messages])
   ```

### Broadcast Optimization

1. **Use channel-based routing**
   ```python
   manager.subscribe(client_id, ["metrics"])  # Only relevant channels
   ```

2. **Implement fanout queues**
   ```python
   # Use Redis Pub/Sub for distributed broadcasting
   ```

3. **Limit subscriber count per channel**
   ```python
   MAX_SUBSCRIBERS_PER_CHANNEL = 10000
   ```

### Memory Optimization

1. **Enable message buffering**
   ```python
   manager.buffer_events(client_id, max_size=100)
   ```

2. **Periodic cleanup**
   ```python
   await reconnection_manager.cleanup_expired_tokens()
   ```

3. **Connection metadata pruning**
   ```python
   # Remove old connections periodically
   ```

---

## Continuous Testing

### Integration with CI/CD

Add to GitHub Actions:
```yaml
- name: Run WebSocket Stress Test
  run: |
    python benchmarks/websocket_stress_benchmark.py --quick
```

### Performance Regression Detection

Track metrics over time:
```bash
# Run and save results
python benchmarks/websocket_stress_benchmark.py > results_$(date +%Y%m%d).txt

# Compare with baseline
diff baseline.txt results_$(date +%Y%m%d).txt
```

### Monitoring Alerts

Set up alerts based on benchmark thresholds:
```yaml
- alert: WebSocketLatencyHigh
  expr: websocket_message_latency_p95 > 50
  for: 5m
  annotations:
    summary: "WebSocket P95 latency exceeds 50μs"
```

---

## Troubleshooting

### High Latency

**Symptom:** P95 > 50μs

**Possible Causes:**
- CPU bottleneck
- Memory pressure
- Network congestion
- Database slow queries

**Solutions:**
- Scale vertically (more CPU/RAM)
- Optimize hot paths
- Add caching layer
- Use async I/O

### Low Throughput

**Symptom:** < 100K msg/sec

**Possible Causes:**
- Single-threaded bottleneck
- Synchronous blocking I/O
- Serialization overhead
- Network bandwidth limit

**Solutions:**
- Use asyncio/uvloop
- Enable compression
- Batch operations
- Upgrade network

### Performance Degradation

**Symptom:** > 30% degradation at scale

**Possible Causes:**
- Memory leaks
- Connection pool exhaustion
- Lock contention
- GC pressure

**Solutions:**
- Profile with py-spy
- Enable connection pooling
- Use lock-free data structures
- Tune GC parameters

### Out of Memory

**Symptom:** Process killed at high load

**Possible Causes:**
- Unbounded buffers
- Connection metadata leak
- Large message payloads

**Solutions:**
- Implement buffer limits
- Cleanup expired connections
- Use binary protocol
- Stream large payloads

---

## Benchmark Limitations

### What This Benchmark Tests

✅ Connection establishment performance
✅ Message processing throughput
✅ Subscription management overhead
✅ Broadcast scalability
✅ Permission checking performance
✅ Compression efficiency

### What This Benchmark Does NOT Test

❌ Network latency (local testing only)
❌ Actual WebSocket protocol overhead
❌ Database query performance
❌ External API calls
❌ Client-side performance
❌ Security vulnerabilities

### For Production Testing

Use additional tools:
- **Load testing:** Artillery, k6, Locust
- **Real WebSocket:** wscat, websocat
- **Monitoring:** Prometheus, Grafana
- **Profiling:** py-spy, cProfile
- **Security:** OWASP ZAP, Burp Suite

---

## Expected Results

### Quick Test (100K ops)

**Duration:** ~1-2 minutes
**Memory:** < 1 GB
**CPU:** < 50%

**Expected Metrics:**
```
Connection:    500K+ conn/sec
Message:       100K+ msg/sec
Latency P95:   < 20 μs
Compression:   > 95% savings
```

### Full Test (9.9M ops)

**Duration:** ~10-15 minutes
**Memory:** < 4 GB
**CPU:** < 80%

**Expected Metrics:**
```
Connection:    300K+ conn/sec (at maximum load)
Message:       100K+ msg/sec (consistent)
Latency P95:   < 50 μs
Broadcast:     500M+ msg/sec (with 5K subscribers)
```

---

## Conclusion

The WebSocket stress benchmark provides comprehensive performance validation across realistic load scenarios. Use it to:

1. **Validate** system performance before production
2. **Identify** bottlenecks and optimization opportunities
3. **Plan** infrastructure capacity
4. **Regression test** performance changes
5. **Document** system capabilities

**Recommended Testing Schedule:**
- **Development:** Quick test before each commit
- **CI/CD:** Quick test on every PR
- **Pre-release:** Full test before deployment
- **Production:** Monthly capacity validation

---

**Tool Location:** `benchmarks/websocket_stress_benchmark.py`
**Documentation:** This file
**Support:** See project README for contact information

**Version:** v0.60.1
**Last Updated:** 2025-12-29
