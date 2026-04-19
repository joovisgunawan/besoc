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

COPY .sqlx .sqlx
ENV SQLX_OFFLINE=true

COPY . .

ENV RUSTFLAGS="-C link-arg=-lzstd"
ENV PKG_CONFIG_ALLOW_CROSS=1
ENV CARGO_PROFILE_RELEASE_STRIP=true

RUN cargo build --release --locked


# ============================================
# STAGE 4: Runtime (DISTROLESS)
# ============================================
FROM gcr.io/distroless/cc-debian12 AS runtime

WORKDIR /app

# Copy binary
COPY --from=builder /app/target/release/student-service /app/student-service

# 🔑 Copy required shared libraries (IMPORTANT)
COPY --from=builder /usr/lib/x86_64-linux-gnu/libssl.so.3 /usr/lib/x86_64-linux-gnu/
COPY --from=builder /usr/lib/x86_64-linux-gnu/libcrypto.so.3 /usr/lib/x86_64-linux-gnu/
COPY --from=builder /usr/lib/x86_64-linux-gnu/libpq.so.5 /usr/lib/x86_64-linux-gnu/
COPY --from=builder /usr/lib/x86_64-linux-gnu/libzstd.so.1 /usr/lib/x86_64-linux-gnu/
COPY --from=builder /usr/lib/x86_64-linux-gnu/libcurl.so.4 /usr/lib/x86_64-linux-gnu/

# Certificates (for HTTPS, DB, etc.)
COPY --from=builder /etc/ssl/certs /etc/ssl/certs

USER nonroot

EXPOSE 8080

CMD ["/app/student-service"]