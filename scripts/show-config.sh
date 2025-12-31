#!/bin/bash
# Show NeuroGraph Configuration
# Displays current project configuration

# Get project root (parent of scripts dir)
PROJECT_ROOT="$(cd "$(dirname "$0")/.." && pwd)"

source "$PROJECT_ROOT/.config.sh"

print_config
