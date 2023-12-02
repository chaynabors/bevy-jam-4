# Signaling server as a docker image
#
# to build, run `docker build -f crates/server/Dockerfile .` from root of the
# repository

ARG RUST_VERSION=1.74
ARG DEBIAN_VERSION=bullseye

FROM rust:${RUST_VERSION}-slim-${DEBIAN_VERSION} as builder

WORKDIR /usr/src/

COPY . .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/src/target \
    cargo build --release --package server && \
    mv target/release/server .

FROM debian:${DEBIAN_VERSION}-slim
RUN apt-get update && apt-get install -y libssl1.1 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/server /usr/local/bin/server
CMD ["server"]
