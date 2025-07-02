FROM rust:1-slim AS builder

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
      pkg-config \
      libssl-dev

COPY . /usr/src/app

WORKDIR /usr/src/app

RUN cargo build --release

FROM debian:bookworm-slim

ENV SCALEWAY_SECRET_KEY=''

RUN apt-get update && \
    apt-get install -y --no-install-recommends ca-certificates libssl3 && \
    rm -rf /var/lib/apt/lists/*

RUN useradd -m rootless

COPY --from=builder \
    /usr/src/app/target/release/scaleway-mcp \
    /usr/local/bin/scaleway-mcp

RUN chown rootless:rootless /usr/local/bin/scaleway-mcp

USER rootless

# Commande d'entrée
ENTRYPOINT ["/usr/local/bin/scaleway-mcp"]
