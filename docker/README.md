# NeuroGraph Docker Setup

Complete Docker setup for NeuroGraph with all services and examples.

## Services

- **neurograph-api**: Main NeuroGraph API server
- **postgres**: PostgreSQL database
- **redis**: Redis cache
- **express-example**: Example Express.js API integration
- **fastapi-example**: Example FastAPI service integration

## Quick Start

### Start All Services

```bash
cd docker
docker-compose up -d
```

### Start Specific Services

```bash
# Only API and dependencies
docker-compose up -d neurograph-api postgres redis

# Only examples
docker-compose up -d express-example fastapi-example
```

### Check Logs

```bash
# All services
docker-compose logs -f

# Specific service
docker-compose logs -f neurograph-api
```

### Stop Services

```bash
docker-compose down

# Remove volumes too
docker-compose down -v
```

## Service URLs

- **NeuroGraph API**: http://localhost:8000
- **API Docs**: http://localhost:8000/docs
- **Express Example**: http://localhost:3001
- **FastAPI Example**: http://localhost:8001
  - **Docs**: http://localhost:8001/docs
- **PostgreSQL**: localhost:5432
- **Redis**: localhost:6379

## Environment Variables

Create `.env` file in docker directory:

```bash
# API Configuration
JWT_SECRET_KEY=your-secret-key-change-in-production
ENVIRONMENT=development

# Database
POSTGRES_USER=neurograph
POSTGRES_PASSWORD=password
POSTGRES_DB=neurograph

# Redis
REDIS_URL=redis://redis:6379

# Client credentials
NEUROGRAPH_USERNAME=developer
NEUROGRAPH_PASSWORD=developer123
```

## Development Workflow

### Live Reload

API service has live reload enabled. Changes to `/src` are reflected automatically.

### Rebuild Services

```bash
# Rebuild all
docker-compose build

# Rebuild specific service
docker-compose build neurograph-api
```

### Run Commands in Containers

```bash
# Execute command in API container
docker-compose exec neurograph-api python -c "print('hello')"

# Open shell in API container
docker-compose exec neurograph-api sh

# Run tests
docker-compose exec neurograph-api pytest
```

## Database Management

### Initialize Database

Database is automatically initialized on first start.

### Run Migrations

```bash
docker-compose exec neurograph-api alembic upgrade head
```

### Reset Database

```bash
docker-compose down -v
docker-compose up -d postgres
# Wait for postgres to start
docker-compose up -d neurograph-api
```

### Access Database

```bash
docker-compose exec postgres psql -U neurograph -d neurograph
```

## Production Deployment

### Security Checklist

- [ ] Change `JWT_SECRET_KEY` to strong random value
- [ ] Change database password
- [ ] Use environment-specific `.env` file
- [ ] Enable HTTPS/TLS
- [ ] Configure firewall rules
- [ ] Set up backup strategy
- [ ] Configure log rotation
- [ ] Set resource limits

### Production Compose File

Create `docker-compose.prod.yml`:

```yaml
version: '3.8'

services:
  neurograph-api:
    restart: always
    environment:
      - ENVIRONMENT=production
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 4G

  postgres:
    restart: always
    deploy:
      resources:
        limits:
          memory: 2G

  redis:
    restart: always
    deploy:
      resources:
        limits:
          memory: 512M
```

Run with:

```bash
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

## Monitoring

### Check Service Health

```bash
# API health
curl http://localhost:8000/health

# Container stats
docker stats

# Container health
docker-compose ps
```

### View Resource Usage

```bash
docker-compose top
```

## Troubleshooting

### Services Won't Start

```bash
# Check logs
docker-compose logs

# Verify network
docker network ls
docker network inspect docker_neurograph-network

# Check ports
netstat -tulpn | grep -E '8000|3001|8001|5432|6379'
```

### Database Connection Issues

```bash
# Verify postgres is running
docker-compose ps postgres

# Check postgres logs
docker-compose logs postgres

# Test connection
docker-compose exec neurograph-api nc -zv postgres 5432
```

### Permission Issues

```bash
# Fix volume permissions
docker-compose down -v
sudo chown -R $USER:$USER ../
docker-compose up -d
```

## Cleanup

### Remove Everything

```bash
# Stop and remove containers, networks, volumes
docker-compose down -v

# Remove images
docker-compose down --rmi all

# Remove all unused Docker resources
docker system prune -a --volumes
```

## Advanced Configuration

### Custom Network

```yaml
networks:
  neurograph-network:
    driver: bridge
    ipam:
      config:
        - subnet: 172.25.0.0/16
```

### Health Checks

```yaml
services:
  neurograph-api:
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8000/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
```

### Volume Mounts for Development

```yaml
services:
  neurograph-api:
    volumes:
      - ../src:/app/src:ro
      - ../tests:/app/tests:ro
```

## Docker Hub

Build and push to Docker Hub:

```bash
# Build
docker build -t neurograph/api:latest .

# Tag
docker tag neurograph/api:latest neurograph/api:0.59.3

# Push
docker push neurograph/api:latest
docker push neurograph/api:0.59.3
```

## Kubernetes

For Kubernetes deployment, see `k8s/` directory.
