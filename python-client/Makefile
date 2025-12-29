# Makefile for NeuroGraph Python Client development

.PHONY: help install install-dev test test-cov lint format type-check clean build publish

help:  ## Show this help message
	@echo "NeuroGraph Python Client - Development Commands"
	@echo ""
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

install:  ## Install package
	pip install -e .

install-dev:  ## Install package with dev dependencies
	pip install -e ".[dev]"
	pre-commit install

test:  ## Run tests
	pytest -v

test-cov:  ## Run tests with coverage
	pytest --cov=neurograph --cov-report=html --cov-report=term

test-watch:  ## Run tests in watch mode
	pytest-watch

lint:  ## Run linters
	ruff check neurograph/ tests/ examples/
	black --check neurograph/ tests/ examples/

format:  ## Format code
	black neurograph/ tests/ examples/
	isort neurograph/ tests/ examples/
	ruff check --fix neurograph/ tests/ examples/

type-check:  ## Run type checking
	mypy neurograph/

clean:  ## Clean build artifacts
	rm -rf build/
	rm -rf dist/
	rm -rf *.egg-info
	rm -rf .pytest_cache
	rm -rf .mypy_cache
	rm -rf .ruff_cache
	rm -rf htmlcov/
	rm -rf .coverage
	find . -type d -name __pycache__ -exec rm -rf {} +
	find . -type f -name '*.pyc' -delete

build:  ## Build distribution packages
	python -m build

publish-test:  ## Publish to TestPyPI
	python -m twine upload --repository testpypi dist/*

publish:  ## Publish to PyPI
	python -m twine upload dist/*

check:  ## Run all checks (lint + type-check + test)
	@echo "Running linters..."
	@make lint
	@echo "\nRunning type checks..."
	@make type-check
	@echo "\nRunning tests..."
	@make test

pre-commit:  ## Run pre-commit on all files
	pre-commit run --all-files

dev-server:  ## Start development API server (requires API code)
	cd .. && uvicorn src.api.main:app --reload

example-sync:  ## Run synchronous example
	python examples/basic_usage.py

example-async:  ## Run asynchronous example
	python examples/async_usage.py

docs-serve:  ## Serve documentation locally (if using mkdocs)
	mkdocs serve

.DEFAULT_GOAL := help
