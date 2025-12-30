# NeuroGraph Automation Scripts

Quick reference for the automated startup scripts in the project root.

## 📋 Available Scripts

### 🔧 `setup-dependencies.sh`
**Purpose:** Install all Python dependencies for the backend API

```bash
./setup-dependencies.sh
```

**What it does:**
- Creates Python virtual environment (`.venv`) if missing
- Upgrades pip to latest version
- Installs all required packages with Python 3.13 compatibility
- Shows installed versions for verification

**Run this:** First time only, or when dependencies change

---

### 🎨 `start-frontend.sh`
**Purpose:** Start the React frontend dev server

```bash
./start-frontend.sh
```

**What it does:**
- Auto-detects Flatpak/standard environment
- Checks if `node_modules` exists, prompts to install if missing
- Starts Vite dev server on port 5173
- Enables hot-reload for development

**Access:** http://localhost:5173

---

### ⚙️ `start-backend.sh`
**Purpose:** Start the FastAPI backend server

```bash
./start-backend.sh
```

**What it does:**
- Auto-detects Flatpak/standard environment
- Kills any existing process on port 8000
- Creates virtual environment if missing
- Starts uvicorn server on port 8000

**Access:**
- API: http://localhost:8000
- Docs: http://localhost:8000/docs
- WebSocket: ws://localhost:8000/ws

---

### 🚀 `start-all.sh`
**Purpose:** Start both frontend and backend together

```bash
./start-all.sh
```

**What it does:**
- Starts backend API (port 8000)
- Starts frontend UI (port 5173)
- Uses tmux for session management (if available)
- Falls back to background processes if tmux not installed

**With tmux (recommended):**
```bash
# Attach to session
tmux attach -t neurograph

# Switch windows
Ctrl+b, n  # Next window
Ctrl+b, p  # Previous window

# Detach (keeps running)
Ctrl+b, d

# Stop all
tmux kill-session -t neurograph
# OR
./stop-all.sh
```

**Without tmux:**
- Processes run in background
- Logs written to `backend.log` and `frontend.log`
- PIDs saved to `.backend.pid` and `.frontend.pid`

---

### 🛑 `stop-all.sh`
**Purpose:** Stop all running NeuroGraph services

```bash
./stop-all.sh
```

**What it does:**
- Kills tmux session if exists
- Stops processes from PID files
- Kills any process on port 8000
- Stops all Vite processes
- Cleans up log files

---

## 🎯 Quick Workflows

### First Time Setup
```bash
# 1. Install frontend dependencies (if not done)
cd src/web && npm install && cd ../..

# 2. Install backend dependencies
./setup-dependencies.sh

# 3. Start everything
./start-all.sh
```

### Daily Development
```bash
# Start everything
./start-all.sh

# Work on your code...
# (tmux: Ctrl+b, d to detach)

# Stop when done
./stop-all.sh
```

### Frontend Only Development
```bash
./start-frontend.sh
```

### Backend Only Development
```bash
./start-backend.sh
```

---

## 🐳 Flatpak Environment

All scripts automatically detect if you're running in a Flatpak container (like VSCodium Flatpak) and use `flatpak-spawn --host` to execute commands on the host system.

**No configuration needed** - it just works!

---

## 🔍 Troubleshooting

### Scripts not executable
```bash
chmod +x *.sh
```

### Port already in use
```bash
# Backend (port 8000)
./stop-all.sh
# OR manually
lsof -ti:8000 | xargs kill -9

# Frontend (port 5173)
pkill -f vite
```

### tmux not found
```bash
# Install tmux for better session management
sudo pacman -S tmux  # Arch Linux
sudo apt install tmux  # Ubuntu/Debian
brew install tmux      # macOS
```

### Dependencies missing
```bash
# Re-run setup
./setup-dependencies.sh

# Or manually
source .venv/bin/activate
pip install -r src/api/requirements.txt
```

---

## 📊 Service Status

Check if services are running:

```bash
# Backend
curl http://localhost:8000/api/v1/health

# Frontend
curl http://localhost:5173

# Processes
ps aux | grep -E "uvicorn|vite"

# Ports
ss -tlnp | grep -E ":8000|:5173"
```

---

**Generated:** 2025-12-30
**Project:** NeuroGraph OS v0.62.0
