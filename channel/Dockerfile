#!/bin/sh

FROM lukemathwalker/cargo-chef:latest as chef
WORKDIR /app/backend/channel
RUN apt update && apt install lld clang -y

FROM chef as planner

COPY sdk /app/backend/sdk
COPY channel /app/backend/channel

# Compute a lock-like file for our project
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/backend/channel/recipe.json recipe.json
# Build our project dependencies, not our application!

RUN cargo chef cook --release --recipe-path recipe.json

COPY sdk /app/backend/sdk
COPY channel /app/backend/channel

# Build our project
RUN cargo build --release --bin main

FROM debian:bookworm-slim AS runtime

WORKDIR /app/backend/channel

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/backend/channel/target/release/channel channel

ENV ENVIRONMENT production
ENTRYPOINT ["./channel"]