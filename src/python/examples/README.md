# NeuroGraph Python Examples

This directory contains example scripts demonstrating the neurograph Python library.

## Examples

### 1. Basic Usage (`basic_usage.py`)

Introduction to the neurograph API:
- Runtime initialization with configuration
- Bootstrap process overview
- Query execution
- Feedback mechanism

```bash
python examples/basic_usage.py
```

### 2. Semantic Search (`semantic_search.py`)

Full working example with test embeddings:
- Creating and loading embeddings
- Multiple semantic queries
- Result filtering and analysis
- Visual similarity display

```bash
python examples/semantic_search.py
```

**Output Example:**
```
Query: 'cat'
  Top 5 similar words:
    kitten       0.9980 ██████████████████████████████
    dog          0.9950 █████████████████████████████
    puppy        0.9940 ████████████████████████████
    pet          0.9900 ███████████████████████████
    animal       0.9802 ██████████████████████████
```

## Running Examples

Make sure neurograph is installed:

```bash
# From src/python directory
pip install -e .

# Or with maturin
maturin develop --features python-bindings
```

Then run any example:

```bash
python examples/<example_name>.py
```

## Using with Real Embeddings

For production use, download pre-trained embeddings:

### GloVe Embeddings

```bash
# Download GloVe 6B (50d, 100d, 200d, 300d)
wget http://nlp.stanford.edu/data/glove.6B.zip
unzip glove.6B.zip

# Use in your code
runtime.bootstrap("glove.6B.50d.txt", limit=50000)
```

### Word2Vec Embeddings

```bash
# Download Google News Word2Vec
# (requires gensim or manual download)
```

## Next Steps

- See [API Reference](../docs/api.md) for complete documentation
- Check [Configuration](../docs/configuration.md) for tuning options
- Explore [Jupyter Integration](../docs/jupyter.md) for interactive use
