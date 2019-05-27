# Build stage
FROM liuchong/rustup:nightly-musl AS builder

ARG PROJECT=dev
WORKDIR /usr/src/${PROJECT}/

ENV RUSTFLAGS=-Clinker=musl-gcc
RUN apt-get -y remove openssl \
    && apt-get -y install gcc \
    && DEBIAN_FRONTEND=noninteractive apt-get -q update && apt-get -qy install wget make \
    && wget https://www.openssl.org/source/openssl-1.0.2g.tar.gz \
    && tar -xzvf openssl-1.0.2g.tar.gz \
    && cd openssl-1.0.2g \
    && ./config \
    && make install \
    && ln -sf /usr/local/ssl/bin/openssl 'which openssl'
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
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/
COPY --from=builder /app .
ENTRYPOINT ["./app"]
