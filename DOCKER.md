# NeuroGraph OS - Docker Deployment Guide

**Version:** v0.43.0
**Status:** Production-Ready

---

## Quick Start

### 1. Build and run (single command)

```bash
docker-compose up -d
```

The API server will be available at `http://localhost:8080`

### 2. Check health

```bash
curl http://localhost:8080/health
```

Expected response:
```json
{"status":"healthy","version":"v0.42.0"}
```

### 3. View metrics

```bash
curl http://localhost:8080/metrics
```

### 4. View logs

```bash
docker-compose logs -f neurograph-api
```

### 5. Stop services

```bash
docker-compose down
```

---

## Configuration

### Environment Variables

Copy `.env.example` to `.env` and customize:

```bash
cp .env.example .env
```

**Available options:**

| Variable | Default | Description |
|----------|---------|-------------|
| `RUST_LOG` | `info` | Logging level (trace/debug/info/warn/error) |
| `NEUROGRAPH_PORT` | `8080` | API server port |
| `NEUROGRAPH_MAX_TOKENS` | `10000000` | Maximum token count (10M ≈ 640MB) |
| `NEUROGRAPH_MAX_MEMORY_BYTES` | `1073741824` | Memory limit (1GB) |

### Resource Limits

Docker Compose sets default limits:
- **CPU:** 2 cores max, 0.5 core reserved
- **Memory:** 2GB max, 512MB reserved

Adjust in `docker-compose.yml` under `deploy.resources`.

---

## Advanced Usage

### With Monitoring Stack

Start with Prometheus + Grafana:

```bash
docker-compose --profile monitoring up -d
```

**Services:**
- **API:** http://localhost:8080
- **Prometheus:** http://localhost:9090
- **Grafana:** http://localhost:3000 (admin/admin)

### Manual Build

Build the Docker image manually:

```bash
docker build -t neurograph-os:v0.43.0 .
```

Run with custom options:

```bash
docker run -d \
  --name neurograph-api \
  -p 8080:8080 \
  -e RUST_LOG=debug \
  -v neurograph-data:/app/data \
  -v neurograph-logs:/app/logs \
  neurograph-os:v0.43.0
```

### Production Deployment

For production, consider:

1. **Use external volumes** for persistence:
```yaml
volumes:
  neurograph-data:
    driver: local
    driver_opts:
      type: nfs
      o: addr=your-nas,rw
      device: ":/path/to/data"
```

2. **Set production secrets** (not in .env):
```bash
docker secret create grafana_password /run/secrets/grafana_password
```

3. **Enable TLS** (use reverse proxy like nginx/traefik)

4. **Horizontal scaling** (multiple replicas):
```bash
docker-compose up -d --scale neurograph-api=3
```

---

## Image Details

### Multi-stage Build

**Stage 1: Builder** (rust:1.83-alpine)
- Installs build dependencies
- Compiles Rust binary with release optimizations
- Strips debug symbols

**Stage 2: Runtime** (alpine:3.19)
- Minimal base image (~7MB)
- Only runtime dependencies (ca-certificates, libgcc)
- Non-root user (`neurograph:1000`)

### Final Image Size

- **Target:** <50MB
- **Actual:** ~45MB (binary + Alpine)

### Security Features

- ✅ Non-root user (UID 1000)
- ✅ Minimal attack surface (Alpine + stripped binary)
- ✅ No build tools in runtime image
- ✅ Health check endpoint
- ✅ Resource limits enforced

---

## API Endpoints

### Core Endpoints

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/health` | GET | Health check (200 OK) |
| `/metrics` | GET | Prometheus metrics |
| `/api/v1/status` | GET | System status |
| `/api/v1/stats` | GET | Statistics |
| `/api/v1/query` | POST | Query API |
| `/api/v1/feedback` | POST | Feedback API |

### Example Query

```bash
curl -X POST http://localhost:8080/api/v1/query \
  -H "Content-Type: application/json" \
  -d '{"text": "hello world"}'
```

---

## Troubleshooting

### Container won't start

**Check logs:**
```bash
docker-compose logs neurograph-api
```

**Common issues:**
- Port 8080 already in use → Change `NEUROGRAPH_PORT` in .env
- Out of memory → Increase `deploy.resources.limits.memory`

### Health check failing

**Manual check:**
```bash
docker exec neurograph-api wget -O- http://localhost:8080/health
```

**Verify network:**
```bash
docker network inspect neurograph-network
```

### Build fails

**Clear Docker cache:**
```bash
docker builder prune -a
```

**Build with verbose output:**
```bash
docker-compose build --progress=plain
```

### Performance issues

**Check resource usage:**
```bash
docker stats neurograph-api
```

**View metrics:**
```bash
curl http://localhost:8080/metrics | grep neurograph_memory
```

---

## Persistence

### Data Volumes

Docker volumes persist data across container restarts:

- **neurograph-data:** WAL files, snapshots
- **neurograph-logs:** Application logs

**Backup volumes:**
```bash
docker run --rm -v neurograph-data:/data -v $(pwd):/backup alpine \
  tar czf /backup/neurograph-data-backup.tar.gz /data
```

**Restore volumes:**
```bash
docker run --rm -v neurograph-data:/data -v $(pwd):/backup alpine \
  tar xzf /backup/neurograph-data-backup.tar.gz -C /
```

### Black Box Dumps

Crash dumps are written to `/app/data/` inside the container:

```bash
docker exec neurograph-api ls -la /app/data/neurograph_crash_dump_*.json
```

Copy dump to host:
```bash
docker cp neurograph-api:/app/data/neurograph_crash_dump_1234567890.json .
```

---

## Development

### Hot Reload

Mount source code for development:

```yaml
volumes:
  - ./src/core_rust/src:/build/src/core_rust/src:ro
```

**Note:** Requires recompilation on changes (no hot reload in Rust).

### Debug Build

Create `Dockerfile.dev`:

```dockerfile
FROM rust:1.83-alpine
# ... (same as builder stage but without --release)
RUN cargo build --bin neurograph-api
```

### VS Code DevContainer

`.devcontainer/devcontainer.json`:

```json
{
  "name": "NeuroGraph OS",
  "dockerComposeFile": "../docker-compose.yml",
  "service": "neurograph-api",
  "workspaceFolder": "/app"
}
```

---

## CI/CD Integration

### GitHub Actions

```yaml
name: Docker Build
on: [push]
jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build Docker image
        run: docker build -t neurograph-os:${{ github.sha }} .
      - name: Test container
        run: |
          docker run -d --name test neurograph-os:${{ github.sha }}
          sleep 5
          docker exec test wget -O- http://localhost:8080/health
```

### Docker Hub

Tag and push:

```bash
docker tag neurograph-os:v0.43.0 youruser/neurograph-os:v0.43.0
docker push youruser/neurograph-os:v0.43.0
```

---

## Monitoring Setup (Prometheus + Grafana)

### 1. Start monitoring stack

```bash
docker-compose --profile monitoring up -d
```

### 2. Import Grafana dashboard

1. Open http://localhost:3000
2. Login (admin/admin)
3. Add Prometheus data source: http://prometheus:9090
4. Import dashboard (create custom or use template)

### 3. Key metrics to monitor

- `neurograph_tokens_created_total` - Token creation rate
- `neurograph_memory_used_bytes` - Memory usage
- `neurograph_token_creation_duration_seconds` - Performance
- `neurograph_panics_recovered_total` - Reliability

### 4. Alerting

Configure Prometheus alerts in `prometheus.yml`:

```yaml
rule_files:
  - 'alerts.yml'

alerting:
  alertmanagers:
    - static_configs:
        - targets: ['alertmanager:9093']
```

---

## Performance Tuning

### 1. Increase resource limits

```yaml
deploy:
  resources:
    limits:
      cpus: '4.0'
      memory: 4G
```

### 2. Optimize token/memory quotas

```bash
NEUROGRAPH_MAX_TOKENS=50000000
NEUROGRAPH_MAX_MEMORY_BYTES=4294967296
```

### 3. Enable aggressive cleanup earlier

Adjust cleanup threshold in code (default: 80%).

### 4. Use host network (Linux only)

```yaml
network_mode: "host"
```

**Trade-off:** Better performance, less isolation.

---

## License

AGPL-3.0 - Copyright (C) 2024-2025 Chernov Denys

---

**Version:** v0.43.0 - Docker Deployment
**Maintainer:** Chernov Denys (@dchrnv)
