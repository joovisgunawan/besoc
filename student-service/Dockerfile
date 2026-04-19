# ============================================
# BASE (shared dependencies)
# ============================================
FROM rust:1.94-slim-bookworm AS base

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    ca-certificates \
    libpq-dev \
    libzstd-dev \
    libsasl2-dev \
    librdkafka-dev \
    libcurl4-openssl-dev \
    build-essential \
    cmake \
    clang \
    libclang-dev \
    git \
    wget \
    && rm -rf /var/lib/apt/lists/*

ENV CARGO_HOME=/usr/local/cargo


# ============================================
# STAGE 1: Planner (cargo-chef)
# ============================================
FROM lukemathwalker/cargo-chef:latest-rust-1.94-slim-bookworm AS planner
WORKDIR /app

# Copy full project (important for workspaces & proper caching)
COPY . .
RUN cargo chef prepare --recipe-path recipe.json


# ============================================
# STAGE 2: Cacher
# ============================================
FROM base AS cacher
WORKDIR /app

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json


# ============================================
# STAGE 3: Builder
# ============================================
FROM base AS builder
WORKDIR /app

# SQLx offline mode
COPY .sqlx .sqlx
ENV SQLX_OFFLINE=true

COPY . .

ENV RUSTFLAGS="-C link-arg=-lzstd"
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV CARGO_PROFILE_RELEASE_STRIP=true

RUN cargo build --release --locked


# ============================================
# STAGE 4: Runtime
# ============================================
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    libpq5 \
    libzstd1 \
    wget \
    && rm -rf /var/lib/apt/lists/*

RUN useradd -m appuser && mkdir /app && chown appuser:appuser /app

WORKDIR /app

COPY --from=builder /app/target/release/student-service /app/student-service

ENV RUST_LOG=info

USER appuser

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=3s --retries=3 \
  CMD wget -qO- http://localhost:8080/health || exit 1

CMD ["./student-service"]