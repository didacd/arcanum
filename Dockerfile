# Stage 1: Build Tailwind CSS
FROM oven/bun:1 AS css-builder
WORKDIR /app
COPY tailwind/ ./tailwind/
COPY static/ ./static/
COPY templates/ ./templates/
WORKDIR /app/tailwind
RUN bun install
RUN bun run build

# Stage 2: Build Rust binary
FROM rust:latest AS builder
WORKDIR /usr/src/arcanum
COPY . .
RUN cargo install --path .

# Stage 3: Runtime
FROM debian:bookworm-slim
WORKDIR /app
RUN apt-get update && apt-get install -y ca-certificates openssl git libssl-dev && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/local/cargo/bin/arcanum /usr/local/bin/arcanum
COPY --from=css-builder /app/static ./static

CMD ["arcanum"]
