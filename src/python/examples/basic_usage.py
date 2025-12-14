#!/usr/bin/env python
"""
Basic Usage Example for neurograph Python Library

This example demonstrates:
- Runtime initialization
- Loading embeddings
- Semantic query
- Working with results
"""

import neurograph as ng

def main():
    print("=" * 60)
    print("NeuroGraph Python Library - Basic Usage Example")
    print("=" * 60)

    # Step 1: Create runtime with custom configuration
    print("\n1. Creating runtime...")
    config = ng.Config(
        dimensions=50,      # Embedding dimensions
        grid_size=1000,     # Spatial grid size
        learning_rate=0.01, # Learning rate for updates
    )
    runtime = ng.Runtime(config)
    print(f"   Runtime created: {runtime._core}")

    # Step 2: Bootstrap with embeddings
    print("\n2. Loading embeddings...")
    print("   Note: Using GloVe embeddings (download from https://nlp.stanford.edu/projects/glove/)")

    # Example with a small test file
    # For real usage, download GloVe embeddings:
    # glove_path = "glove.6B.50d.txt"
    # runtime.bootstrap(glove_path, limit=50000)

    print("   Skipping bootstrap in this example (no embeddings file)")
    print("   To run with real embeddings:")
    print("   - Download GloVe: wget http://nlp.stanford.edu/data/glove.6B.zip")
    print("   - Extract: unzip glove.6B.zip")
    print("   - Use: runtime.bootstrap('glove.6B.50d.txt', limit=50000)")

    # Step 3: Query (would work after bootstrap)
    print("\n3. Semantic Query Example:")
    print("   After bootstrap, you can query:")
    print("   result = runtime.query('cat', top_k=5)")
    print("   for word, similarity in result.top(5):")
    print("       print(f'{word}: {similarity:.4f}')")

    # Step 4: Feedback
    print("\n4. Feedback Example:")
    print("   result.feedback('positive')  # Improve these results")
    print("   result.feedback('negative')  # Avoid these results")

    print("\n" + "=" * 60)
    print("Example complete!")
    print("=" * 60)

if __name__ == "__main__":
    main()
