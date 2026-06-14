# Multi-stage Dockerfile for SolNetP2P Node
# Produces a small runtime image (~50-80MB)

# === Build Stage ===
FROM rust:1-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy manifests first for better caching
COPY Cargo.toml Cargo.lock* ./

# Create dummy main.rs to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release --bin solnetp2p-node

# Now copy real source
COPY src ./src

# Build the actual binary
RUN touch src/main.rs && cargo build --release --bin solnetp2p-node

# === Runtime Stage ===
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from builder
COPY --from=builder /app/target/release/solnetp2p-node /usr/local/bin/solnetp2p-node

# Create directory for identity and data
RUN mkdir -p /data

# Expose default UDP port (can be overridden)
EXPOSE 9000/udp

# Default command (can be overridden in docker-compose)
ENTRYPOINT ["solnetp2p-node"]
CMD ["--listen", "0.0.0.0:9000", "--bootstrap"]