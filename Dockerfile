# Multi-stage build for WaspsWithBazookas
FROM rust:1.75-slim as builder

WORKDIR /app
COPY . .

# Build all binaries
RUN cargo build --release --bin hive --bin wasp --bin test-dummy

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -m -u 1000 wasp

# Copy binaries from builder
COPY --from=builder /app/target/release/hive /usr/local/bin/
COPY --from=builder /app/target/release/wasp /usr/local/bin/
COPY --from=builder /app/target/release/test-dummy /usr/local/bin/

# Set ownership
RUN chown wasp:wasp /usr/local/bin/hive /usr/local/bin/wasp /usr/local/bin/test-dummy

# Switch to non-root user
USER wasp

# Expose common ports
EXPOSE 4269 3000 8080

# Default command
CMD ["hive", "--help"] 