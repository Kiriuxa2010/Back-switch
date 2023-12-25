# Build stage
FROM rust:latest AS builder

WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files to cache dependencies
COPY . .

# Build the actual release executable
RUN make build

# Final stage
FROM debian:latest

WORKDIR /app

# Copy the built binary from the builder stage to the final stage
COPY --from=builder /app/target/release/FrontswitchWebServer .
COPY --from=builder /app/hello.html .
COPY --from=builder /app/404.html .
COPY --from=builder /app/help.html .
COPY --from=builder /app/atlas.html .
COPY --from=builder /app/pepeh.jpeg .

# Expose the port your server listens on
EXPOSE 8000

# Run your executable when the container starts
CMD ["./FrontswitchWebServer"]
