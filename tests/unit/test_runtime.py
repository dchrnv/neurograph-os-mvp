"""Tests for neurograph.runtime module."""

import pytest
from neurograph import Runtime, Config
from neurograph.exceptions import RuntimeError, BootstrapError, ConfigError


class TestConfig:
    """Tests for Config class."""

    def test_default_config(self):
        """Test default configuration values."""
        config = Config()
        assert config.grid_size == 1000
        assert config.dimensions == 50
        assert config.learning_rate == 0.01
        assert config.max_connections == 100
        assert config.enable_metrics is False
        assert config.metrics_port == 9090

    def test_custom_config(self):
        """Test custom configuration values."""
        config = Config(
            grid_size=2000,
            dimensions=100,
            learning_rate=0.05,
            max_connections=200,
            enable_metrics=True,
            metrics_port=8080,
        )
        assert config.grid_size == 2000
        assert config.dimensions == 100
        assert config.learning_rate == 0.05
        assert config.max_connections == 200
        assert config.enable_metrics is True
        assert config.metrics_port == 8080

    def test_invalid_grid_size(self):
        """Test that invalid grid_size raises ConfigError."""
        with pytest.raises(ConfigError, match="grid_size must be positive"):
            Config(grid_size=-1)

    def test_invalid_dimensions(self):
        """Test that invalid dimensions raises ConfigError."""
        with pytest.raises(ConfigError, match="dimensions must be positive"):
            Config(dimensions=0)

    def test_invalid_learning_rate(self):
        """Test that invalid learning_rate raises ConfigError."""
        with pytest.raises(ConfigError, match="learning_rate must be in"):
            Config(learning_rate=1.5)

    def test_config_to_dict(self):
        """Test config to dictionary conversion."""
        config = Config(grid_size=500)
        config_dict = config.to_dict()
        assert config_dict["grid_size"] == 500
        assert "dimensions" in config_dict
        assert "learning_rate" in config_dict


class TestRuntime:
    """Tests for Runtime class."""

    def test_runtime_init_default(self):
        """Test runtime initialization with default config."""
        runtime = Runtime()
        assert runtime._config is not None
        assert runtime._initialized is False

    def test_runtime_init_custom_config(self):
        """Test runtime initialization with custom config."""
        config = Config(grid_size=2000)
        runtime = Runtime(config=config)
        assert runtime._config.grid_size == 2000

    def test_bootstrap_file_not_found(self):
        """Test that bootstrap raises error for missing file."""
        runtime = Runtime()
        with pytest.raises(BootstrapError, match="Embeddings file not found"):
            runtime.bootstrap("/nonexistent/path.txt")

    @pytest.mark.skip(reason="Requires FFI implementation")
    def test_query_before_bootstrap(self):
        """Test that query before bootstrap raises error."""
        runtime = Runtime()
        with pytest.raises(RuntimeError, match="Runtime not initialized"):
            runtime.query("test")

    @pytest.mark.skip(reason="Requires FFI implementation")
    def test_feedback_before_bootstrap(self):
        """Test that feedback before bootstrap raises error."""
        runtime = Runtime()
        with pytest.raises(RuntimeError, match="Runtime not initialized"):
            runtime.feedback("signal-123", "positive")

    def test_context_manager(self):
        """Test runtime as context manager."""
        with Runtime() as runtime:
            assert runtime is not None
