FROM rust:1-slim AS builder

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      pkg-config \
      libssl-dev

COPY . /usr/src/app

WORKDIR /usr/src/app

RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder \
    /usr/src/app/target/release/scaleway-mcp \
    /usr/local/bin/scaleway-mcp

# Commande d'entrée
ENTRYPOINT ["/usr/local/bin/scaleway-mcp"]
