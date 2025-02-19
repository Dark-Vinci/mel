#!/bin/sh

FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app/backend/gateway
RUN apt update && apt install lld clang -y

FROM chef as planner

COPY sdk /app/backend/sdk
COPY gateway /app/backend/gateway

# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/backend/gateway/recipe.json recipe.json
# Build our project dependencies, not our application!

RUN cargo chef cook --release --recipe-path recipe.json

COPY sdk /app/backend/sdk
COPY gateway /app/backend/gateway

# Build our project
RUN cargo build --release --bin main

FROM debian:bookworm-slim AS runtime

WORKDIR /app/backend/gateway

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/backend/gateway/target/release/gateway gateway

ENV ENVIRONMENT production
ENTRYPOINT ["./gateway"]