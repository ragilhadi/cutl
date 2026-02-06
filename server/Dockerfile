# Stage 1: Build the Rust application
FROM rust:1.85-slim AS builder

# Install build dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    pkg-config \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory to workspace root
WORKDIR /build

# Copy workspace Cargo files
COPY Cargo.toml Cargo.lock* ./

# Copy server and CLI source code
COPY server ./server
COPY cli ./cli

# Build only the server binary in release mode
RUN cargo build --release -p cutl-server

# Stage 2: Create minimal runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    wget \
    gosu \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user to run the application
RUN useradd -m -u 1000 cutl

# Copy the compiled binary from builder
COPY --from=builder /build/target/release/cutl-server /usr/local/bin/cutl-server

# Make binary executable
RUN chmod +x /usr/local/bin/cutl-server

# Copy entrypoint script
COPY server/entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

# Set default environment variables (can be overridden in docker-compose)
ENV DATABASE_URL="sqlite:/data/cutl.db"
ENV BASE_URL="http://localhost:3233"
ENV BIND_ADDRESS="0.0.0.0:3233"

# Expose the default port
EXPOSE 3233

# Set data directory as working directory
WORKDIR /data

# Use entrypoint script to fix permissions and switch user
ENTRYPOINT ["/usr/local/bin/entrypoint.sh"]

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:3233/shorten || exit 1

# Run the application
CMD ["/usr/local/bin/cutl-server"]
