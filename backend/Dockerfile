# Multi-stage build for Rust backend
FROM rust:1.70 as builder

WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
COPY backend ./backend
COPY programs ./programs

# Build the application
RUN cargo build --release --bin server

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl1.1 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder stage
COPY --from=builder /app/target/release/server ./server

# Create a non-root user
RUN useradd -r -s /bin/false snakeai
RUN chown snakeai:snakeai ./server
USER snakeai

EXPOSE 8000

CMD ["./server"]
