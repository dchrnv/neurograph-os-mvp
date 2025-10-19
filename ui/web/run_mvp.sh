#!/bin/bash

echo "═══════════════════════════════════════════════════════════"
echo "🚀 NeuroGraph OS MVP Launcher"
echo "═══════════════════════════════════════════════════════════"

# Activate venv
source .venv/bin/activate

echo "Starting API server on http://localhost:8000..."
python src/api_mvp/main.py &
API_PID=$!

echo "API PID: $API_PID"
echo ""
echo "═══════════════════════════════════════════════════════════"
echo "✅ API is running!"
echo "📖 API Docs: http://localhost:8000/docs"
echo "💚 Health:   http://localhost:8000/health"
echo ""
echo "To start the dashboard (requires Node.js):"
echo "  cd ui/web"
echo "  npm install"
echo "  npm run dev"
echo ""
echo "Press Ctrl+C to stop the API server"
echo "═══════════════════════════════════════════════════════════"

# Wait for Ctrl+C
trap "kill $API_PID 2>/dev/null; echo 'API stopped'; exit" INT
wait $API_PID
