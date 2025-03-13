# Stage 1: Build the React application
FROM node:22 AS react-build
WORKDIR /app
COPY client/package.json client/package-lock.json ./client/
RUN cd client && npm install
COPY client ./client
RUN cd client && npm run build

# Stage 2: Build the Rust application
FROM rust:latest AS rust-build
WORKDIR /app
COPY . .
# Copy the React build output from the previous stage
COPY --from=react-build /app/static ./static
RUN cargo build --release

# Stage 3: Create the final minimal runtime image
FROM debian:bookworm-slim
WORKDIR /app
# Install necessary runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=rust-build /app/Rocket.toml .
# Copy the built Rust binary from the previous stage
COPY --from=rust-build /app/target/release/recipe_keeper .
# Copy the static files from the React build
COPY --from=react-build /app/static ./static

# Expose the port your Rocket app runs on
EXPOSE 8000

# Command to run your application
CMD ["/app/recipe_keeper"]