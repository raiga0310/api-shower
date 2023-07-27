FROM rust:1.71-bullseye as builder

RUN USER=root cargo new --bin api-shower

WORKDIR /api-shower

ARG DATABASE_URL

ENV DATABASE_URL=$DATABASE_URL

COPY ./Cargo.toml ./Cargo.lock ./

RUN cargo install sqlx-cli --no-default-features --features postgres

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/api_shower*
RUN cargo build --release

FROM debian:bullseye-slim as api

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /api-shower/target/release/api-shower /usr/local/bin/api-shower

CMD ["api-shower"]
