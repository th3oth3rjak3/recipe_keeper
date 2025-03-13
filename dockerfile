# Stage 1: Build the frontend (npm)
FROM node:22 AS frontend

# Set working directory for the frontend build
WORKDIR /app/client

# Copy the frontend source code (npm project)
COPY ./client/package.json ./client/package-lock.json ./
RUN npm install

# Build the frontend
COPY ./client ./
RUN npm run build

# Stage 2: Build the backend (Rust)
FROM rust:latest AS builder

# Set the working directory for Rust
WORKDIR /app

# Install dependencies for Rust build
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    libclang-dev \
    llvm-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy the Rust source code into the builder
COPY . .

# Build the Rust project in release mode
RUN cargo build --release

# Stage 3: Final minimal runtime image (with newer glibc)
FROM debian:bookworm-slim

# Install necessary runtime libraries for the application
RUN apt-get update && apt-get install -y \
    libssl3 \
    libc6 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set working directory for the runtime
WORKDIR /app

# Copy the compiled Rust binary from the builder stage
COPY --from=builder /app/target/release/recipe_keeper /app/recipe_keeper

# Copy the built static assets from the frontend build stage
COPY --from=frontend /app/static /app/static

# Expose the application port
EXPOSE 8000

# Start the application
CMD ["/app/recipe_keeper"]
