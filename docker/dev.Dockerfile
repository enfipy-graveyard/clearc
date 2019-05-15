FROM liuchong/rustup:nightly-musl AS builder

ARG PROJECT=dev
WORKDIR /usr/src/${PROJECT}/

ENV RUSTFLAGS=-Clinker=musl-gcc
RUN apt-get -y update && \
    rustup target install x86_64-unknown-linux-musl && \
    cargo install cargo-watch

COPY Cargo.toml Cargo.lock ./

CMD cargo watch -q -x run
