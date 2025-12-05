# NeuroGraph OS - Multi-stage Docker Build
# Target: API Server (neurograph-api binary)
# Final image: <50MB (Alpine-based)

# =============================================================================
# Stage 1: Builder - Compile Rust code
# =============================================================================
FROM rust:1.83-alpine AS builder

# Install build dependencies
RUN apk add --no-cache \
    musl-dev \
    pkgconfig \
    openssl-dev \
    openssl-libs-static

# Set working directory
WORKDIR /build

# Copy only Cargo files first for better caching
COPY src/core_rust/Cargo.toml src/core_rust/Cargo.lock* ./src/core_rust/

# Create dummy source to cache dependencies
RUN mkdir -p src/core_rust/src && \
    echo "fn main() {}" > src/core_rust/src/lib.rs

# Build dependencies only (cached layer)
WORKDIR /build/src/core_rust
RUN cargo build --release --bin neurograph-api

# Remove dummy source
RUN rm -rf src/

# Copy actual source code
WORKDIR /build
COPY src/core_rust/src ./src/core_rust/src
COPY src/core_rust/benches ./src/core_rust/benches
COPY src/core_rust/examples ./src/core_rust/examples

# Build the actual binary
WORKDIR /build/src/core_rust
RUN cargo build --release --bin neurograph-api && \
    strip target/release/neurograph-api

# =============================================================================
# Stage 2: Runtime - Minimal Alpine image
# =============================================================================
FROM alpine:3.19 AS runtime

# Install runtime dependencies only
RUN apk add --no-cache \
    ca-certificates \
    libgcc

# Create non-root user
RUN addgroup -g 1000 neurograph && \
    adduser -D -u 1000 -G neurograph neurograph

# Create necessary directories
RUN mkdir -p /app/data /app/logs && \
    chown -R neurograph:neurograph /app

# Copy binary from builder
COPY --from=builder /build/src/core_rust/target/release/neurograph-api /app/neurograph-api

# Set ownership
RUN chown neurograph:neurograph /app/neurograph-api

# Switch to non-root user
USER neurograph
WORKDIR /app

# Environment variables (can be overridden)
ENV RUST_LOG=info
ENV NEUROGRAPH_HOST=0.0.0.0
ENV NEUROGRAPH_PORT=8080
ENV NEUROGRAPH_MAX_TOKENS=10000000
ENV NEUROGRAPH_MAX_MEMORY_BYTES=1073741824

# Expose API port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=5s --start-period=10s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

# Run the API server
CMD ["/app/neurograph-api"]
