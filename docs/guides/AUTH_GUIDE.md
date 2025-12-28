# Authentication & Security Guide

> NeuroGraph REST API v0.58.0 - Complete authentication system with JWT, API Keys, RBAC, and Rate Limiting

## Table of Contents
- [Overview](#overview)
- [Quick Start](#quick-start)
- [JWT Authentication](#jwt-authentication)
- [API Keys](#api-keys)
- [RBAC & Permissions](#rbac--permissions)
- [Rate Limiting](#rate-limiting)
- [Security Best Practices](#security-best-practices)

---

## Overview

NeuroGraph API v0.58.0 provides enterprise-grade authentication and authorization:

- **JWT Tokens** - Stateless authentication for users
- **API Keys** - Long-lived credentials for bots/integrations
- **RBAC** - Role-Based Access Control with fine-grained permissions
- **Rate Limiting** - Token bucket algorithm to prevent abuse
- **Dual Auth** - Support both JWT and API keys simultaneously

**Default Users (MVP):**
```
admin / admin123       (Full access)
developer / developer123  (Read + Write)
viewer / viewer123     (Read-only)
```

---

## Quick Start

### 1. Login with JWT

```bash
# Login
curl -X POST http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "admin123"}'

# Response:
{
  "access_token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
  "refresh_token": "eyJ0eXAiOiJKV1QiLCJhbGc...",
  "token_type": "bearer",
  "expires_in": 900,
  "user": {
    "user_id": "admin",
    "username": "admin",
    "role": "admin",
    "scopes": ["tokens:read", "tokens:write", ...]
  }
}
```

### 2. Use Access Token

```bash
# Call protected endpoint
curl http://localhost:8000/api/v1/tokens \
  -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGc..."
```

### 3. Create API Key (Admin only)

```bash
# Create API key
curl -X POST http://localhost:8000/api/v1/api-keys \
  -H "Authorization: Bearer <admin-token>" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Production Bot",
    "scopes": ["tokens:read", "tokens:write"],
    "rate_limit": 500,
    "expires_in_days": 90
  }'

# Response (SAVE THE KEY - shown only once!):
{
  "api_key": "ng_live_FZdGMqO-KGgHjQw...",
  "key_id": "ed3a42f332a9f2e3",
  "warning": "⚠️ Save this key securely! It will not be shown again."
}
```

### 4. Use API Key

```bash
# Call endpoint with API key
curl http://localhost:8000/api/v1/tokens \
  -H "X-API-Key: ng_live_FZdGMqO-KGgHjQw..."
```

---

## JWT Authentication

### Token Lifecycle

**Access Token:**
- Lifetime: 15 minutes
- Type: Short-lived, stateless
- Use: All API requests
- Payload: user_id, role, scopes

**Refresh Token:**
- Lifetime: 7 days
- Type: Long-lived, rotates on use
- Use: Get new access token without re-login

### Endpoints

#### POST /api/v1/auth/login
Login and get tokens.

**Request:**
```json
{
  "username": "developer",
  "password": "developer123"
}
```

**Response:**
```json
{
  "access_token": "eyJ...",
  "refresh_token": "eyJ...",
  "token_type": "bearer",
  "expires_in": 900,
  "user": {
    "user_id": "developer",
    "username": "developer",
    "role": "developer",
    "scopes": ["tokens:read", "tokens:write", "grid:read", ...]
  }
}
```

#### POST /api/v1/auth/refresh
Refresh access token.

**Request:**
```json
{
  "refresh_token": "eyJ..."
}
```

**Response:**
```json
{
  "access_token": "eyJ...",  // New access token
  "refresh_token": "eyJ...",  // New refresh token (rotated)
  "token_type": "bearer",
  "expires_in": 900
}
```

#### POST /api/v1/auth/logout
Logout (revoke tokens).

**Request:**
```json
{
  "token": "eyJ..."  // Optional: specific token to revoke
}
```

#### GET /api/v1/auth/me
Get current user info.

**Headers:**
```
Authorization: Bearer eyJ...
```

**Response:**
```json
{
  "user_id": "developer",
  "username": "developer",
  "role": "developer",
  "scopes": ["tokens:read", "tokens:write", ...],
  "disabled": false
}
```

---

## API Keys

### Key Format

- **Live:** `ng_live_<43-chars-base64>`
- **Test:** `ng_test_<43-chars-base64>`

### Storage

- Keys are **SHA256 hashed** (plaintext never stored)
- Only shown **once** on creation
- File-based storage: `data/api_keys.json`

### Management Endpoints

All require `admin:config` permission.

#### POST /api/v1/api-keys
Create new API key.

**Request:**
```json
{
  "name": "Production Bot",
  "scopes": ["tokens:read", "tokens:write", "health:read"],
  "rate_limit": 500,
  "expires_in_days": 90
}
```

**Response (ONCE ONLY):**
```json
{
  "api_key": "ng_live_FZdGMqO-KGgHjQwE3z...",  // ⚠️ SAVE THIS!
  "key_id": "ed3a42f332a9f2e3",
  "key_prefix": "ng_live_FZdGMqO-",
  "name": "Production Bot",
  "scopes": ["tokens:read", "tokens:write", "health:read"],
  "rate_limit": 500,
  "created_at": "2025-01-15T10:30:00Z",
  "expires_at": "2025-04-15T10:30:00Z",
  "warning": "⚠️ Save this key securely! It will not be shown again."
}
```

#### GET /api/v1/api-keys
List all API keys.

#### GET /api/v1/api-keys/{key_id}
Get specific key details (without full key).

#### POST /api/v1/api-keys/{key_id}/revoke
Revoke (disable) key.

#### DELETE /api/v1/api-keys/{key_id}
Permanently delete key.

---

## RBAC & Permissions

### Roles

| Role | Description | Permissions |
|------|-------------|-------------|
| **admin** | Full system access | ALL |
| **developer** | Read + Write operations | Read/Write tokens, grid, CDNA, metrics |
| **viewer** | Read-only access | Read tokens, grid, CDNA, metrics, status |
| **bot** | Limited integration access | Custom scopes per API key |

### Permissions

**Tokens:**
- `tokens:read` - List, get tokens
- `tokens:write` - Create, update tokens
- `tokens:delete` - Delete tokens

**Grid:**
- `grid:read` - Query grid, find neighbors
- `grid:write` - Create grid, add/remove tokens

**CDNA:**
- `cdna:read` - Get config, profiles, history
- `cdna:write` - Update config, switch profiles, quarantine

**System:**
- `status:read` - System status, stats
- `metrics:read` - Prometheus metrics
- `health:read` - Health checks

**Admin:**
- `admin:config` - Manage API keys, reset config
- `admin:bootstrap` - Bootstrap system
- `admin:logs` - View logs

### Protected Endpoints

All endpoints (except `/health`, `/docs`) require authentication:

```
✓ 7 Token endpoints
✓ 10 Grid endpoints
✓ 12 CDNA endpoints
✓ 2 Status endpoints
✓ 2 Metrics endpoints
✓ 5 API Key endpoints
```

**Total: 38 protected endpoints**

---

## Rate Limiting

### Algorithm

**Token Bucket:**
- Capacity: 100 tokens (default)
- Refill rate: 1 token/second
- Each request consumes 1 token
- Requests blocked when bucket empty

### Per-User Tracking

- **JWT users:** Tracked by `user_id`
- **API keys:** Tracked by key prefix
- **Anonymous:** Tracked by IP address

### Response Headers

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 73
```

### 429 Too Many Requests

```json
{
  "success": false,
  "error": {
    "code": "RATE_LIMIT_EXCEEDED",
    "message": "Rate limit exceeded. Try again later.",
    "details": {
      "rate_limit": 100,
      "remaining": 0,
      "retry_after": 1
    }
  }
}
```

**Headers:**
```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 0
Retry-After: 1
```

### Custom Limits

API keys can have custom rate limits (1-10000 req/min):

```json
{
  "name": "High-Volume Bot",
  "rate_limit": 5000  // 5000 requests/minute
}
```

---

## Security Best Practices

### Production Checklist

✅ **JWT Secrets:**
```bash
# Generate secure secrets
openssl rand -base64 32

# Set in .env
JWT_SECRET_KEY=your-256-bit-secret
JWT_REFRESH_SECRET_KEY=your-256-bit-refresh-secret
```

✅ **HTTPS Only:**
```python
# Force HTTPS in production
if ENVIRONMENT == "production":
    assert request.url.scheme == "https"
```

✅ **Database Storage:**
```python
# Replace in-memory storage with PostgreSQL
USERS_DB → PostgreSQL users table
api_keys.json → PostgreSQL api_keys table
```

✅ **Password Hashing:**
```python
# Use bcrypt instead of SHA256
from passlib.hash import bcrypt
hashed = bcrypt.hash(password)
```

✅ **Token Rotation:**
- Refresh tokens rotate on every use
- Old refresh tokens are blacklisted
- Implement database-backed blacklist for production

✅ **Rate Limiting:**
- Use Redis for distributed rate limiting
- Adjust limits based on user tier
- Monitor and alert on abuse patterns

### Security Headers

```
Authorization: Bearer <token>
X-API-Key: <key>
X-RateLimit-*: <rate limit info>
X-Correlation-ID: <request ID>
```

### Error Handling

- ✅ Generic error messages (don't leak info)
- ✅ Structured logging for debugging
- ✅ Correlation IDs for request tracing
- ✅ 401 Unauthorized vs 403 Forbidden

---

## Examples

### Python Client

```python
import requests

# Login
response = requests.post(
    "http://localhost:8000/api/v1/auth/login",
    json={"username": "developer", "password": "developer123"}
)
data = response.json()
access_token = data["access_token"]

# Use token
headers = {"Authorization": f"Bearer {access_token}"}
tokens = requests.get("http://localhost:8000/api/v1/tokens", headers=headers).json()

# Or use API key
headers = {"X-API-Key": "ng_live_FZdGMqO..."}
tokens = requests.get("http://localhost:8000/api/v1/tokens", headers=headers).json()
```

### cURL Examples

```bash
# Login
TOKEN=$(curl -s -X POST http://localhost:8000/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username":"admin","password":"admin123"}' \
  | jq -r '.access_token')

# Create token
curl -X POST http://localhost:8000/api/v1/tokens \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"data": "Hello World", "context": "test"}'

# Create API key
curl -X POST http://localhost:8000/api/v1/api-keys \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Test Bot",
    "scopes": ["tokens:read"],
    "rate_limit": 100,
    "expires_in_days": 30
  }'
```

---

## Troubleshooting

### "Invalid authentication credentials"
- Check token expiration (15min for access, 7 days for refresh)
- Verify token format: `Authorization: Bearer <token>`
- Ensure JWT secrets match between restarts

### "Permission denied"
- Check user role and permissions
- Verify endpoint requires correct permission
- Admin-only endpoints: `/api-keys`, `/tokens/admin/*`

### "Rate limit exceeded"
- Wait for `retry_after` seconds
- Check `X-RateLimit-Remaining` header
- Request higher limit for API key if needed

### "API key not found"
- Verify key hasn't expired
- Check key hasn't been revoked
- Ensure correct header: `X-API-Key: ng_live_...`

---

**For more details:**
- [API Reference](../API.md)
- [Getting Started](./GETTING_STARTED.md)
- [MASTER_PLAN v3.0](../MASTER_PLAN_v3.0.md)
