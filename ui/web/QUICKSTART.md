# ⚡ Quick Start Guide

## 30-Second Setup

```bash
# 1. Install dependencies
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements.txt

# 2. Run API
./run_mvp.sh

# 3. Open browser
# http://localhost:8000/docs
```

## Test Token v2.0

```bash
# Create example tokens
curl -X POST http://localhost:8000/api/v1/tokens/examples/create

# List all tokens
curl http://localhost:8000/api/v1/tokens

# Health check
curl http://localhost:8000/health
```

## Run Dashboard (Optional)

Requires Node.js 18+

```bash
cd ui/web
npm install
npm run dev
# Open http://localhost:3000
```

## What You Get

✅ **Token v2.0** - 64-byte atomic units
✅ **8 Semantic Spaces** - Physical, Emotional, etc.
✅ **REST API** - Full OpenAPI docs
✅ **Cyberpunk Dashboard** - Real-time monitoring

## Next Steps

- Read: [README_MVP.md](README_MVP.md)
- Token Spec: [docs/token_extended_spec.md](docs/token_extended_spec.md)
- API Docs: http://localhost:8000/docs

Happy coding! ⚡
