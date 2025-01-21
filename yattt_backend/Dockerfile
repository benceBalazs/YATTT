# Use Rust official image
FROM rust:1.83 as builder

# Set working directory
WORKDIR /app

# Copy Rust project files
COPY . .

# Build the application in release mode
RUN cargo build --release

# Create a minimal runtime image
FROM debian:bookworm-slim

# Set working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /app/target/release/yattt_backend .

# Expose port
EXPOSE 8080

# Set the startup command
CMD ["./yattt_backend"]