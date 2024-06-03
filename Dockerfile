# Use Rust 1.70 as the builder image
FROM rust:1.70 as builder

# Install protobuf-compiler
RUN apt-get update && DEBIAN_FRONTEND=noninteractive apt-get install --no-install-recommends --assume-yes protobuf-compiler libprotobuf-dev

# Set the working directory
WORKDIR /usr/src/artie-engine-rs

# Copy the configuration files and dependencies
COPY Cargo.toml .
COPY Cargo.lock .

# Copy the project files
COPY . .

# Build the project
RUN cargo build --release

# Final stage
FROM debian:buster-slim

# Copy the built binary from the build stage
COPY --from=builder /usr/src/artie-engine-rs/target/release/artie-engine-rs /usr/local/bin/artie-engine-rs

# Set the entry point
ENTRYPOINT ["artie-engine-rs"]

# Expose the gRPC port
EXPOSE 50051
