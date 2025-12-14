#!/usr/bin/env python
"""
Semantic Search Example

Demonstrates semantic similarity search with real embeddings.
Requires a small test embeddings file.
"""

import neurograph as ng
import tempfile
import os

def create_test_embeddings():
    """Create a small test embeddings file for demonstration."""
    # Create test embeddings with semantic relationships
    embeddings = """cat 0.1 0.2 0.3
dog 0.15 0.25 0.35
kitten 0.12 0.22 0.32
puppy 0.16 0.26 0.36
pet 0.2 0.3 0.4
animal 0.3 0.4 0.5
lion 0.25 0.35 0.45
tiger 0.27 0.37 0.47
bird 0.5 0.6 0.7
eagle 0.52 0.62 0.72
fish 0.6 0.7 0.8
shark 0.62 0.72 0.82
tree 0.9 0.8 0.7
forest 0.92 0.82 0.72
car 0.8 0.9 1.0
vehicle 0.82 0.92 1.02
house 0.7 0.8 0.9
building 0.72 0.82 0.92
computer 1.0 1.1 1.2
laptop 1.02 1.12 1.22"""

    # Write to temporary file
    fd, path = tempfile.mkstemp(suffix='.txt', text=True)
    with os.fdopen(fd, 'w') as f:
        f.write(embeddings)

    return path

def main():
    print("=" * 60)
    print("NeuroGraph - Semantic Search Example")
    print("=" * 60)

    # Create test embeddings
    print("\nCreating test embeddings file...")
    embeddings_path = create_test_embeddings()
    print(f"Created: {embeddings_path}")

    try:
        # Initialize runtime
        print("\nInitializing runtime...")
        runtime = ng.Runtime(ng.Config(dimensions=3))

        # Load embeddings
        print("Loading embeddings...")
        runtime.bootstrap(embeddings_path, limit=20)
        print("Bootstrap complete!")

        # Example queries
        queries = ["cat", "bird", "car", "computer"]

        for query_word in queries:
            print(f"\n{'=' * 60}")
            print(f"Query: '{query_word}'")
            print(f"{'=' * 60}")

            result = runtime.query(query_word, top_k=5)

            if len(result) == 0:
                print("  No results (word not in vocabulary)")
            else:
                print(f"  Top {len(result)} similar words:")
                for word, similarity in result.top(5):
                    # Format with visual bar
                    bar_length = int(similarity * 30)
                    bar = "█" * bar_length
                    print(f"    {word:12} {similarity:.4f} {bar}")

                print(f"\n  Signal ID: {result.signal_id}")

        # Demonstrate result filtering
        print(f"\n{'=' * 60}")
        print("Advanced: Filtering Results")
        print(f"{'=' * 60}")

        result = runtime.query("cat", top_k=10)
        print(f"\nAll results: {len(result)}")

        high_sim = result.filter(min_similarity=0.995)
        print(f"High similarity (>0.995): {len(high_sim)}")
        for word, sim in high_sim:
            print(f"  {word}: {sim:.4f}")

        # Check if specific word is in results
        print(f"\nContains 'dog': {result.contains('dog')}")
        print(f"Contains 'car': {result.contains('car')}")

        # Get similarity for specific word
        if result.contains('dog'):
            sim = result.get_similarity('dog')
            print(f"Similarity to 'dog': {sim:.4f}")

    finally:
        # Cleanup
        print(f"\nCleaning up temporary file...")
        os.unlink(embeddings_path)

    print("\n" + "=" * 60)
    print("Example complete!")
    print("=" * 60)

if __name__ == "__main__":
    main()
