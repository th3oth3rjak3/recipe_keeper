# Assuming your Rocket app is in /app
FROM rust:latest AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Now use an appropriate base image to run your app
FROM ubuntu:22.04
WORKDIR /app

# Install dependencies for the app to run
RUN apt-get update && apt-get install -y \
    libc6 \
    libssl-dev \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy compiled binary and static assets into the container
COPY --from=builder /app/target/release/recipe_keeper /app/recipe_keeper
COPY ./static /app/static

EXPOSE 8000

CMD ["/app/recipe_keeper"]
