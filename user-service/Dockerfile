# ---------- BUILD STAGE ----------
FROM rust:1.76 as builder

WORKDIR /app

# cache dependencies
COPY Cargo.toml Cargo.lock ./

RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

RUN cargo build --release
RUN rm -rf src

# copy real source
COPY src ./src

RUN cargo build --release


# ---------- RUNTIME STAGE ----------
FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# copy compiled binary
COPY --from=builder /app/target/release/user-service /usr/local/bin/user-service

EXPOSE 8080

CMD ["user-service"]