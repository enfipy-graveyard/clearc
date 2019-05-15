# Build stage
FROM liuchong/rustup:nightly-musl AS builder

ARG PROJECT=dev
WORKDIR /usr/src/${PROJECT}/

ENV RUSTFLAGS=-Clinker=musl-gcc
RUN rustup target install x86_64-unknown-linux-musl

COPY Cargo.toml Cargo.lock ./

RUN mkdir src/ && \
    echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs && \
    cargo build --release --target=x86_64-unknown-linux-musl && \
    rm -rf src && \
    rm -f target/x86_64-unknown-linux-musl/release/deps/${PROJECT}*

COPY ./src ./src

RUN cargo build --release --target=x86_64-unknown-linux-musl && \
    mv ./target/x86_64-unknown-linux-musl/release/${PROJECT} /app

# Final stage
FROM scratch
COPY --from=builder /app .
ENTRYPOINT ["./app"]
