# ⚡ NeuroGraph OS MVP - Quick Start

## 30-Second Setup

```bash
# 1. Clone & enter
git clone https://github.com/dchrnv/neurograph-os-dev.git
cd neurograph-os-dev

# 2. Run (автоматически создаст venv и установит зависимости)
./run_mvp.sh

# 3. Open browser
# http://localhost:8000/docs
```

## Manual Setup

```bash
# Create virtual environment
python3 -m venv .venv
source .venv/bin/activate

# Install dependencies
pip install -r requirements.txt

# Run API
python src/api_mvp/main.py
```

## Test API

```bash
# Health check
curl http://localhost:8000/health

# Create example tokens
curl -X POST http://localhost:8000/api/v1/tokens/examples/create

# List all tokens
curl http://localhost:8000/api/v1/tokens
```

## Run Dashboard (Optional)

Requires **Node.js 18+**

```bash
cd ui/web
npm install
npm run dev
```

Dashboard: http://localhost:3000

## What You Get

✅ **Token v2.0** - 64-byte atomic data units  
✅ **8 Semantic Spaces** - Physical, Emotional, Cognitive, etc.  
✅ **REST API** - Full OpenAPI documentation  
✅ **React Dashboard** - Real-time monitoring (optional)

## Next Steps

📖 Read full docs: [README_MVP.md](README_MVP.md)  
🔬 Token specification: [docs/token_extended_spec.md](docs/token_extended_spec.md)  
🌐 API playground: http://localhost:8000/docs

---

**Questions?** Check [README_MVP.md](README_MVP.md) for details.
