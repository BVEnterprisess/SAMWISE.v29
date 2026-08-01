FROM rust:1.88-slim AS builder
WORKDIR /app

# Dependency Caching Layer
COPY Cargo.toml Cargo.lock ./
RUN mkdir src migrations && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Application Compilation Layer
COPY src ./src
COPY migrations ./migrations
RUN cargo build --release

# Security-Hardened Distroless/Slim Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates wget && rm -rf /var/lib/apt/lists/*
RUN useradd -r -s /bin/false -u 10001 appuser

WORKDIR /app
COPY --from=builder /app/target/release/samwise-core /app/samwise-core
COPY migrations /app/migrations
RUN mkdir -p /app/data && chown -R appuser:appuser /app

USER appuser
EXPOSE 30000 30001

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget -qO- http://127.0.0.1:30001/health || exit 1

CMD ["./samwise-core"]
