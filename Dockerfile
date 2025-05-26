FROM rust:1.74-slim

# Install system dependencies
RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential

# Create app directory
WORKDIR /app

# Pre-cache dependencies
COPY Cargo.toml .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy project files
COPY . .

# Build release version
RUN cargo build --release

CMD ["./target/release/backend"]
