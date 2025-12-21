#!/usr/bin/env python3
"""
Test for Encoders - Phase 4
"""

from src.gateway.encoders import (
    PassthroughEncoder,
    NumericDirectEncoder,
    TextTfidfEncoder,
    SentimentSimpleEncoder,
)


def test_passthrough_encoder():
    """Test PASSTHROUGH encoder"""
    print("=" * 60)
    print("Test 1: PassthroughEncoder")
    print("=" * 60)
    print()

    encoder = PassthroughEncoder()

    # Test with list
    vector = encoder.encode([0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8])
    print(f"Input: [0.1, 0.2, ..., 0.8]")
    print(f"Output: {vector}")
    assert len(vector) == 8
    print()

    # Test with dict
    vector2 = encoder.encode({'vector': [0.5] * 8})
    print(f"Input: dict with 'vector' key")
    print(f"Output: {vector2}")
    assert len(vector2) == 8
    print()

    print("✓ PASSTHROUGH encoder working")
    print()


def test_numeric_encoder():
    """Test NUMERIC_DIRECT encoder"""
    print("=" * 60)
    print("Test 2: NumericDirectEncoder")
    print("=" * 60)
    print()

    encoder = NumericDirectEncoder(scale_factor=100.0)

    # Test with single number
    vector = encoder.encode(45.7)
    print(f"Input: 45.7")
    print(f"Output: {vector}")
    assert len(vector) == 8
    assert vector[0] > 0  # First dimension should be non-zero
    print()

    # Test with list of numbers
    vector2 = encoder.encode([10.0, 20.0, 30.0])
    print(f"Input: [10.0, 20.0, 30.0]")
    print(f"Output: {vector2}")
    assert len(vector2) == 8
    assert vector2[0] == 0.1
    assert vector2[1] == 0.2
    assert vector2[2] == 0.3
    print()

    # Test with dict
    vector3 = encoder.encode({'cpu': 45.7, 'mem': 67.3})
    print(f"Input: dict {{'cpu': 45.7, 'mem': 67.3}}")
    print(f"Output: {vector3}")
    assert len(vector3) == 8
    print()

    print("✓ NUMERIC_DIRECT encoder working")
    print()


def test_text_tfidf_encoder():
    """Test TEXT_TFIDF encoder"""
    print("=" * 60)
    print("Test 3: TextTfidfEncoder")
    print("=" * 60)
    print()

    encoder = TextTfidfEncoder()

    # Test with short text
    text1 = "Hello, NeuroGraph!"
    vector1 = encoder.encode(text1)
    print(f"Input: \"{text1}\"")
    print(f"Output: {vector1}")
    assert len(vector1) == 8
    assert sum(vector1) > 0  # Should have some non-zero values
    print()

    # Test with longer text
    text2 = "Machine learning and artificial intelligence are transforming technology"
    vector2 = encoder.encode(text2)
    print(f"Input: \"{text2}\"")
    print(f"Output: {vector2}")
    assert len(vector2) == 8
    print()

    # Verify different texts produce different vectors
    assert vector1 != vector2
    print("✓ Different texts produce different vectors")
    print()

    print("✓ TEXT_TFIDF encoder working")
    print()


def test_sentiment_encoder():
    """Test SENTIMENT_SIMPLE encoder"""
    print("=" * 60)
    print("Test 4: SentimentSimpleEncoder")
    print("=" * 60)
    print()

    encoder = SentimentSimpleEncoder()

    # Test with positive text
    text1 = "I am very happy and excited today!"
    vector1 = encoder.encode(text1)
    print(f"Input: \"{text1}\"")
    print(f"Output: {vector1}")
    print(f"  Polarity (dim 0): {vector1[0]:.2f}")
    print(f"  Subjectivity (dim 1): {vector1[1]:.2f}")
    print(f"  Intensity (dim 2): {vector1[2]:.2f}")
    print(f"  Joy (dim 3): {vector1[3]:.2f}")
    assert len(vector1) == 8
    assert vector1[0] > 0.5  # Should be positive
    assert vector1[3] > 0  # Joy dimension should be activated
    print()

    # Test with negative text
    text2 = "I am very sad and disappointed"
    vector2 = encoder.encode(text2)
    print(f"Input: \"{text2}\"")
    print(f"Output: {vector2}")
    print(f"  Polarity (dim 0): {vector2[0]:.2f}")
    print(f"  Subjectivity (dim 1): {vector2[1]:.2f}")
    print(f"  Intensity (dim 2): {vector2[2]:.2f}")
    print(f"  Sadness (dim 4): {vector2[4]:.2f}")
    assert len(vector2) == 8
    assert vector2[0] < 0.5  # Should be negative
    assert vector2[4] > 0  # Sadness dimension should be activated
    print()

    # Test with neutral text
    text3 = "The system is running"
    vector3 = encoder.encode(text3)
    print(f"Input: \"{text3}\"")
    print(f"Output: {vector3}")
    print(f"  Polarity (dim 0): {vector3[0]:.2f} (neutral)")
    assert len(vector3) == 8
    print()

    print("✓ SENTIMENT_SIMPLE encoder working")
    print()


def test_encoder_normalization():
    """Test that all encoders produce [0, 1] normalized vectors"""
    print("=" * 60)
    print("Test 5: Vector Normalization")
    print("=" * 60)
    print()

    encoders = [
        ("PASSTHROUGH", PassthroughEncoder(), [0.5] * 8),
        ("NUMERIC", NumericDirectEncoder(), 50.0),
        ("TEXT_TFIDF", TextTfidfEncoder(), "test text"),
        ("SENTIMENT", SentimentSimpleEncoder(), "happy text"),
    ]

    for name, encoder, data in encoders:
        vector = encoder.encode(data)
        assert all(0.0 <= v <= 1.0 for v in vector), f"{name} produced out-of-range values"
        print(f"✓ {name}: all values in [0, 1]")

    print()
    print("✓ All encoders produce normalized vectors")
    print()


def main():
    print("\n" + "=" * 60)
    print("Encoder Test Suite")
    print("=" * 60)
    print()

    test_passthrough_encoder()
    test_numeric_encoder()
    test_text_tfidf_encoder()
    test_sentiment_encoder()
    test_encoder_normalization()

    print("=" * 60)
    print("✓ All encoder tests passed!")
    print("=" * 60)
    print()


if __name__ == "__main__":
    main()
