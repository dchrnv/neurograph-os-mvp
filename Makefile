
## 🛠️ `Makefile` (полная версия)

```makefile
# NeuroGraph OS Makefile

.PHONY: help run-dev run-prod run-watch test test-unit test-integration lint format docs clean docker-build docker-run

# Variables
PYTHON = python
PIP = pip
MODULE = src.main
PORT ?= 8000
ENV ?= development

help:
	@echo "NeuroGraph OS - Available commands:"
	@echo "  make run-dev       - Run in development mode"
	@echo "  make run-prod      - Run in production mode"
	@echo "  make run-watch     - Run with auto-reload"
	@echo "  make test          - Run all tests"
	@echo "  make test-unit     - Run unit tests"
	@echo "  make test-integration - Run integration tests"
	@echo "  make lint          - Run code linting"
	@echo "  make format        - Format code"
	@echo "  make docs          - Generate documentation"
	@echo "  make clean         - Clean temporary files"
	@echo "  make docker-build  - Build Docker image"
	@echo "  make docker-run    - Run in Docker"

# Installation
install:
	$(PIP) install -r requirements/core.txt

install-dev:
	$(PIP) install -r requirements/dev.txt

install-all: install install-dev

# Running
run-dev:
	ENVIRONMENT=development $(PYTHON) -m $(MODULE)

run-prod:
	ENVIRONMENT=production $(PYTHON) -m $(MODULE)

run-watch:
	$(PIP) install watchdog
	ENVIRONMENT=development $(PYTHON) -m src.main --reload

run-debug:
	ENVIRONMENT=development $(PYTHON) -m debugpy --listen 0.0.0.0:5678 -m $(MODULE)

# Testing
test:
	$(PYTHON) -m pytest tests/ -v

test-unit:
	$(PYTHON) -m pytest tests/unit/ -v

test-integration:
	$(PYTHON) -m pytest tests/integration/ -v

test-coverage:
	$(PYTHON) -m pytest tests/ --cov=src --cov-report=html

# Code quality
lint:
	$(PYTHON) -m flake8 src/ tests/
	$(PYTHON) -m mypy src/
	$(PYTHON) -m bandit -r src/

format:
	$(PYTHON) -m black src/ tests/
	$(PYTHON) -m isort src/ tests/

# Documentation
docs:
	$(PYTHON) -m pdoc src/ -o docs/api/ --html

docs-serve:
	$(PYTHON) -m pdoc src/ -o docs/api/ --http :8080

# Cleanup
clean:
	find . -name "*.pyc" -delete
	find . -name "__pycache__" -delete
	find . -name ".pytest_cache" -exec rm -rf {} +
	find . -name ".mypy_cache" -exec rm -rf {} +
	rm -rf .coverage htmlcov/
	rm -rf docs/api/
	rm -rf build/ dist/ *.egg-info/

# Docker
docker-build:
	docker build -t neurograph-os:latest .

docker-run:
	docker run -p $(PORT):8000 -e ENVIRONMENT=$(ENV) neurograph-os:latest

docker-compose-up:
	docker-compose up --build

docker-compose-down:
	docker-compose down

# Database
db-migrate:
	$(PYTHON) -m alembic upgrade head

db-rollback:
	$(PYTHON) -m alembic downgrade -1

db-reset: clean
	rm -rf data/*.db
	$(PYTHON) -m alembic upgrade head

# Development utilities
requirements-update:
	$(PIP) freeze > requirements.txt

venv-create:
	$(PYTHON) -m venv venv

venv-activate:
	@echo "Run: source venv/bin/activate (Linux/Mac) or venv\Scripts\activate (Windows)"

# Default target
.DEFAULT_GOAL := help

# Docker commands
docker-build:
	docker build -t neurograph-os:latest .

docker-build-dev:
	docker build -f Dockerfile.dev -t neurograph-os:dev .

docker-run:
	docker run -p 8000:8000 -v ./data:/app/data neurograph-os:latest

docker-compose-up:
	docker-compose up --build

docker-compose-down:
	docker-compose down

docker-compose-logs:
	docker-compose logs -f

docker-shell:
	docker-compose exec neurograph bash

# Requirements management
requirements-update:
	python scripts/generate_requirements.py

requirements-install:
	pip install -r requirements.txt

requirements-install-dev:
	pip install -r requirements/dev.txt

requirements-install-core:
	pip install -r requirements/core.txt

requirements-install-api:
	pip install -r requirements/core.txt -r requirements/api.txt