# Build stage
FROM rust:latest AS builder

WORKDIR /app

# Copy source code
COPY . .

# Build the application in release mode
RUN cargo build --release

# Release stage
FROM debian:stable-slim

# Create a user and group with no privileges
RUN groupadd -r appuser && useradd -r -g appuser appuser

# Set the user
USER appuser

WORKDIR /app

# Copy the binary from the builder stage
COPY --from=builder /app/target/release/account_service .

# Run the application
CMD ["./account_service"]