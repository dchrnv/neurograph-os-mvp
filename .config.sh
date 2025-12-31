#!/bin/bash
# NeuroGraph Project Configuration
# Single source of truth for all project settings

# Get script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_DIR="$SCRIPT_DIR/config"

# Load configuration files
if [ -f "$CONFIG_DIR/project.env" ]; then
    set -a  # automatically export all variables
    source "$CONFIG_DIR/project.env"
    source "$CONFIG_DIR/python.env"
    source "$CONFIG_DIR/rust.env"
    source "$CONFIG_DIR/versions.env"
    set +a
fi

# Load local overrides if exist
if [ -f "$SCRIPT_DIR/.env.local" ]; then
    set -a
    source "$SCRIPT_DIR/.env.local"
    set +a
fi

# === Auto-detection ===

# Detect Python version and execution prefix based on environment
if [ -f "/.flatpak-info" ] || command -v flatpak-spawn &> /dev/null; then
    # Flatpak environment - use host's Python
    export EXEC_PREFIX="flatpak-spawn --host"
    export IS_FLATPAK=true
    PYTHON_CMD="python3"
else
    # Standard environment
    export EXEC_PREFIX=""
    export IS_FLATPAK=false
    PYTHON_CMD="python3"
fi

export PYTHON_CMD

# === Path Exports ===

export VENV_PATH="$PROJECT_ROOT/$VENV_DIR"

# === Helper Functions ===

# Run command (with or without flatpak-spawn)
run_cmd() {
    if [ -n "$EXEC_PREFIX" ]; then
        $EXEC_PREFIX /bin/bash -c "cd $PROJECT_ROOT && $1"
    else
        cd "$PROJECT_ROOT"
        eval "$1"
    fi
}

# Get Python version
get_python_version() {
    if [ -n "$EXEC_PREFIX" ]; then
        $EXEC_PREFIX $PYTHON_CMD --version
    else
        $PYTHON_CMD --version
    fi
}

# Get project version
get_project_version() {
    echo "$PROJECT_VERSION"
}

# Print configuration summary
print_config() {
    echo "📋 NeuroGraph Configuration"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Project:     $PROJECT_NAME v$PROJECT_VERSION"
    echo "Root:        $PROJECT_ROOT"
    echo "Environment: $ENVIRONMENT"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Python:      $(get_python_version)"
    echo "Flatpak:     $IS_FLATPAK"
    echo "VirtualEnv:  $VENV_PATH"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Backend:     http://localhost:$BACKEND_PORT"
    echo "Frontend:    http://localhost:$FRONTEND_PORT"
    echo "API Docs:    http://localhost:$BACKEND_PORT/docs"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "Features:"
    echo "  Rust Core:  $RUST_CORE_ENABLED"
    echo "  Jupyter:    $JUPYTER_ENABLED"
    echo "  WebSocket:  $WEBSOCKET_ENABLED"
    echo "  Metrics:    $METRICS_ENABLED"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
}

# Check if venv exists
check_venv() {
    if [ ! -d "$VENV_PATH" ]; then
        echo "⚠️  Virtual environment not found at: $VENV_PATH"
        echo "💡 Run ./setup-dependencies.sh to create it"
        return 1
    fi
    return 0
}

# Activate venv helper
activate_venv() {
    if check_venv; then
        source "$VENV_PATH/bin/activate"
        return 0
    fi
    return 1
}
