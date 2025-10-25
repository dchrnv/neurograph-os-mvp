# Installation and Testing Guide

## Prerequisites

### Install Rust

If you don't have Rust installed, install it using rustup:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the prompts and restart your terminal after installation.

Verify installation:

```bash
rustc --version
cargo --version
```

## Build Instructions

### 1. Navigate to the Rust core directory

```bash
cd src/core_rust
```

### 2. Build the project

Development build:
```bash
cargo build
```

Release build (optimized):
```bash
cargo build --release
```

### 3. Run tests

Run all tests:
```bash
cargo test
```

Run tests with output:
```bash
cargo test -- --nocapture
```

Run a specific test:
```bash
cargo test test_token_size -- --nocapture
```

### 4. Run the demo

```bash
cargo run --bin token-demo
```

Or with release optimizations:
```bash
cargo run --release --bin token-demo
```

## Expected Output

### Test Output

```
running 12 tests
test token::tests::test_coordinate_encoding ... ok
test token::tests::test_create_id ... ok
test token::tests::test_entity_type ... ok
test token::tests::test_field_radius ... ok
test token::tests::test_field_strength ... ok
test token::tests::test_flags ... ok
test token::tests::test_serialization ... ok
test token::tests::test_set_get_coordinates ... ok
test token::tests::test_token_new ... ok
test token::tests::test_token_size ... ok
test token::tests::test_validation ... ok
test tests::test_version ... ok

test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### Demo Output

```
=== NeuroGraph OS Token V2.0 Demo ===

1. Creating new token...
   Token ID: 37752345
   Local ID: 12345
   Domain: 1
   Size: 64 bytes

2. Setting coordinates in semantic spaces...
   L1 Physical: [10.5, 20.3, 5.2]
   L4 Emotional (VAD): [0.8, 0.6, 0.5]
   L8 Abstract: [0.9, 0.3, 0.7]

3. Setting entity type and flags...
   Entity type: Concept
   Active: true
   Persistent: true
   Mutable: true

4. Setting weight and field properties...
   Weight: 0.75
   Field radius: 1.5
   Field strength: 0.85

5. Validating token...
   ✓ Token is valid

6. Testing serialization...
   Serialized to 64 bytes
   Deserialized token ID: 37752345
   Weight preserved: 0.75
   Coordinates preserved: [10.5, 20.3, 5.2]

7. Debug representation:
Token { id: 37752345, local_id: 12345, entity_type: Concept, domain: 1, flags: 0x0503, weight: 0.75, field_radius: 1.5, field_strength: 0.85, timestamp: 1730123456, coordinates: [8 spaces × 3 axes] }

8. Creating multiple tokens with different types...
   Token 1: Object - Physical object
   Token 2: Event - Temporal event
   Token 3: Process - Running process
   Token 4: Memory - Stored memory

=== Demo Complete ===
```

## Troubleshooting

### Cargo not found

If you get "cargo: command not found", you need to install Rust:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Build errors

Make sure you're using Rust 1.70+ (2021 edition required):

```bash
rustup update
```

### Test failures

If tests fail, check that the system time is correct (affects timestamp validation).

## Development Workflow

### Format code

```bash
cargo fmt
```

### Lint code

```bash
cargo clippy
```

### Clean build artifacts

```bash
cargo clean
```

### Generate documentation

```bash
cargo doc --open
```

## Next Steps

After successful build and testing:

1. ✅ Token V2.0 Rust implementation is ready
2. 📋 Next: Implement Connection V2 in Rust
3. 📋 Then: Create FFI bindings for Python interop
4. 📋 Then: Implement Grid V2.0

## Questions?

See README.md for API documentation and usage examples.
