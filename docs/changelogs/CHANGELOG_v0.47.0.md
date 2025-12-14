# Changelog

All notable changes to NeuroGraph OS will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased][Unreleased]

### Added - Python Library (Phase 1)

#### [0.47.5] - 2025-01-14

**Phase 1.5: Testing & Documentation - Final Release**

Added:

- `src/python/examples/` - Complete example suite
  - `basic_usage.py` - Introduction to neurograph API
  - `semantic_search.py` - Full working semantic search example with test embeddings
  - `README.md` - Examples documentation with usage instructions
- Example features:
  - Test embeddings creation for quick prototyping
  - Visual similarity display with progress bars
  - Result filtering demonstrations
  - Multiple query examples (cat, bird, car, computer)
  - Cleanup and resource management

Changed:

- Python package fully functional with real semantic search
- All core features tested and working:
  - Bootstrap loading (GloVe format)
  - PCA projection to 3D
  - Grid KNN search
  - Similarity scoring
  - Result filtering and manipulation

Fixed:

- Test coverage for real bootstrap and query operations
- Example scripts tested with synthetic embeddings

#### [0.47.4] - 2025-01-14

**Phase 1.4: Query Engine & Semantic Search**

Added:

- Full semantic query implementation using Grid KNN search
  - Word-to-ID lookup for query terms
  - Grid-based nearest neighbor search in 3D semantic space
  - Distance-to-similarity conversion (exponential decay)
  - Automatic result sorting by similarity (descending)
  - Empty result handling for unknown words
- `src/core_rust/src/python/runtime.rs`:
  - `PyRuntime::query()` now performs real semantic search
  - Grid KNN search using `CoordinateSpace::L1Physical`
  - Similarity scoring: `exp(-distance/scale)`
  - Top-k filtering and self-exclusion

Changed:

- `PyRuntime::query()` no longer returns mock data
- Query results now reflect actual semantic similarity based on embeddings

#### [0.47.3] - 2025-01-14

**Phase 1.3: Python Runtime Manager - Bootstrap Integration**

Added:

- Full BootstrapLibrary integration in PyRuntime
  - Load embeddings from GloVe/Word2Vec text files
  - Automatic PCA projection to 3D coordinates
  - Graph and Grid population from embeddings
  - Word-to-ID bidirectional mappings for fast lookup
- `src/core_rust/src/bootstrap.rs`:
  - `BootstrapLibrary::concepts_iter()` - Public iterator over all concepts
- `src/core_rust/src/python/runtime.rs`:
  - Extended PyRuntime with `bootstrap: Option<BootstrapLibrary>`
  - Extended PyRuntime with `word_to_id` and `id_to_word` HashMaps
  - Complete bootstrap pipeline implementation:
    1. Load embeddings from file (with limit support)
    2. Run PCA pipeline (project to 3D)
    3. Populate graph with nodes
    4. Populate grid for spatial queries
    5. Build word/ID lookup tables

Changed:

- `PyRuntime::bootstrap()` now performs full embedding loading and processing
- Bootstrap validation includes: file existence, embedding count, PCA projection, graph/grid population

#### [0.47.2] - 2025-01-14

**Phase 1.2: PyO3 FFI Bindings**

Added:

- `src/core_rust/src/python/runtime.rs` - PyRuntime FFI wrapper for Python integration
  - `PyRuntime::new(config)` - Initialize runtime with configuration dict
  - `PyRuntime::bootstrap(path, format, limit, progress)` - Load embeddings (stub mode)
  - `PyRuntime::query(text, top_k, context)` - Semantic query (stub mode with mock results)
  - `PyRuntime::feedback(signal_id, feedback_type)` - User feedback (stub mode)
  - `PyRuntime::export_metrics()` - Export Prometheus metrics
  - `PyRuntime::is_initialized()` - Check initialization status
  - `PyRuntime::get_dimensions()` - Get embedding dimensions

Changed:

- `src/core_rust/src/python/mod.rs`:
  - Updated from v0.40.0 to v0.45.0
  - Renamed Python module from `neurograph` to `_core` (low-level FFI)
  - Added PyRuntime to module exports
  - Updated feature flag from `python` to `python-bindings`
  - Updated metadata: author "NeuroGraph Team", license "AGPL-3.0-or-later"
- `src/core_rust/src/lib.rs:83`:
  - Changed feature gate from `#[cfg(feature = "python")]` to `#[cfg(feature = "python-bindings")]`
- `src/core_rust/Cargo.toml:96`:
  - Renamed feature `python` to `python-bindings` for consistency with pyproject.toml
- `src/python/neurograph/runtime.py`:
  - Added FFI integration via `neurograph._core.PyRuntime`
  - Added graceful fallback to stub mode when FFI not available
  - Connected `bootstrap()`, `query()`, `feedback()`, `export_metrics()` to FFI
- `src/python/neurograph/query.py`:
  - Added `QueryContext.to_dict()` method for FFI serialization

Fixed:

- Maturin build compatibility with PyO3 0.22
- Graph::new() call (removed grid_size parameter)
- metrics::export_metrics() error handling (returns Result)
- Unused variable warnings in PyRuntime

**Phase 1.1: Python Project Setup**

Added:

- `src/python/pyproject.toml` - Python package configuration
  - Maturin build system for PyO3 integration
  - Package metadata: neurograph v0.1.0, AGPL-3.0-or-later license
  - Dependencies: numpy>=1.20.0, tqdm>=4.60.0
  - Dev dependencies: pytest, pytest-cov, black, mypy, ruff
  - Jupyter integration dependencies: ipython, matplotlib, pandas, plotly
  - Maturin config: manifest-path, python-source, module-name, features
- `src/python/README.md` - Python package documentation
  - Quick start guide with example usage
  - Installation instructions (PyPI and from source)
  - Feature list and architecture diagram
  - Development setup and contributing guidelines
- `src/python/neurograph/__init__.py` - Public API module
  - Version, author, license metadata
  - Exports: Runtime, Config, QueryResult, QueryContext
  - Exports: NeurographError, RuntimeError, QueryError, BootstrapError, ConfigError
  - Exports: FeedbackType, EmbeddingFormat
- `src/python/neurograph/runtime.py` - Runtime and Config classes
  - `Config` dataclass with validation (grid_size, dimensions, learning_rate, etc.)
  - `Config.to_dict()` for FFI serialization
  - `Runtime.__init__(config)` - Initialize with optional config
  - `Runtime.bootstrap(path, format, limit, progress)` - Load embeddings
  - `Runtime.query(text, top_k, context)` - Execute semantic query
  - `Runtime.feedback(signal_id, feedback_type)` - Provide feedback
  - `Runtime.export_metrics()` - Export Prometheus metrics
  - Context manager support (`__enter__`, `__exit__`)
- `src/python/neurograph/query.py` - Query execution and results
  - `QueryContext` dataclass (filters, boost_terms, exclude_terms)
  - `QueryResult` class with methods:
    - `top(k)` - Get top-k results
    - `all()` - Get all results
    - `filter(min_similarity)` - Filter by threshold
    - `contains(term)` - Check if term in results
    - `get_similarity(term)` - Get similarity score
    - `feedback(feedback_type)` - Provide feedback
  - Iterator protocol support
- `src/python/neurograph/exceptions.py` - Exception hierarchy
  - `NeurographError` - Base exception
  - `RuntimeError` - Runtime initialization/operation errors
  - `QueryError` - Query execution errors
  - `BootstrapError` - Bootstrap loading errors
  - `ConfigError` - Configuration validation errors
  - `FFIError` - FFI call errors
  - `EmbeddingError` - Embedding operation errors
- `src/python/neurograph/types.py` - Type definitions
  - `FeedbackType` enum (POSITIVE, NEGATIVE, NEUTRAL)
  - `EmbeddingFormat` enum (GLOVE, WORD2VEC, FASTTEXT)
  - `TokenInfo` dataclass (token_id, text, embedding, connection_count, metadata)
  - `ConnectionInfo` dataclass (source_id, target_id, weight, metadata)
  - Type aliases: `SimilarityPair`, `SimilarityList`

#### [0.47.1] - 2025-01-14

**Phase 1.1: Python Project Setup**

Added:

- `src/python/neurograph/tests/__init__.py` - Test package
- `src/python/neurograph/tests/test_runtime.py` - Runtime tests (12 tests)
  - Config validation tests (default, custom, invalid parameters)
  - Runtime initialization tests
  - Bootstrap error handling tests
  - Context manager tests
- `src/python/neurograph/tests/test_query.py` - Query tests (10 tests)
  - QueryContext tests
  - QueryResult manipulation tests (top, all, filter, contains, get_similarity)
  - Iterator and repr tests
- `src/python/neurograph/tests/test_types.py` - Type tests (6 tests)
  - Enum value tests
  - Dataclass tests (TokenInfo, ConnectionInfo)

## [0.45.0][0.45.0] - Previous Release

### Added

- Rust Core v0.45.0
  - Token System v2.0
  - Connection v3.0
  - Grid, Graph, Guardian (CDNA)
  - IntuitionEngine v3.0 (Hybrid Reflex System)
  - ExperienceStream v2.1
  - Bootstrap Library v1.2
  - Prometheus Metrics v1.0
  - OpenTelemetry Distributed Tracing v1.0
  - Adaptive Tracing Sampling v1.0

### Changed

- Desktop UI removed (both old and new implementations)
- Benchmarks temporarily disabled due to API incompatibilities

## Version History

- **0.47.x** - Python Library Phase 1 ✅ COMPLETE
  - 0.47.1 - Project Setup
  - 0.47.2 - PyO3 FFI Bindings
  - 0.47.3 - Runtime Manager (Bootstrap Integration)
  - 0.47.4 - Query Engine & Semantic Search
  - 0.47.5 - Testing & Documentation (Final Release)
- **0.45.0** - Core Rust implementation with all subsystems
- **0.44.3** - Adaptive Tracing Sampling
- **0.44.2** - Async WAL Writer
- **0.44.0** - OpenTelemetry Distributed Tracing
- **0.42.0** - Prometheus Metrics + Black Box Recorder
- **0.41.0** - WAL + Panic Recovery
- **0.40.0** - Python Bindings v1.0 (PyToken, PyIntuitionEngine)
- **0.39.0** - REST API v1.0
- **0.38.0** - Curiosity Drive v1.0
- **0.37.0** - Feedback System v1.0
- **0.36.0** - Output/Input Adapters v1.0
- **0.35.0** - Gateway v1.0
- **0.33.0** - Bootstrap Library v1.2
- **0.31.0** - Reflex System v3.0
- **0.30.2** - Hybrid Learning v2.2

[Unreleased]: https://github.com/dchrnv/neurograph-os/compare/v0.47.5...HEAD
[0.47.5]: https://github.com/dchrnv/neurograph-os/compare/v0.47.4...v0.47.5
[0.47.4]: https://github.com/dchrnv/neurograph-os/compare/v0.47.3...v0.47.4
[0.47.3]: https://github.com/dchrnv/neurograph-os/compare/v0.47.2...v0.47.3
[0.47.2]: https://github.com/dchrnv/neurograph-os/compare/v0.47.1...v0.47.2
[0.47.1]: https://github.com/dchrnv/neurograph-os/compare/v0.45.0...v0.47.1
[0.45.0]: https://github.com/dchrnv/neurograph-os/releases/tag/v0.45.0
