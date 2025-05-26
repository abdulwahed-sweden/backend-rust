FROM rust:1.82-slim

# Install system dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Pre-cache dependencies
COPY Cargo.toml .
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Copy project files
COPY . .

# Create necessary directories
RUN mkdir -p templates static/css static/js static/images

# Build release version
RUN cargo build --release

# Expose port
EXPOSE 8080

CMD ["./target/release/backend"]