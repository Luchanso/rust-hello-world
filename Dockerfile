FROM rust:1.67.1 as builder

ENV CARGO_TERM_COLOR always

WORKDIR /rust-hello-world

# Cache rust index
RUN cargo new --bin build-index \
  && cd build-index \
  && cargo add rand_core \
  && cd .. \
  && rm -rf build-index

RUN cargo install
RUN rustup target add x86_64-unknown-linux-musl
RUN apt-get update && apt-get install musl

COPY Cargo.lock Cargo.toml ./
COPY src/ ./src
ENV TARGET_CC=x86_64-linux-musl-gcc
RUN cargo build --bins --release --target x86_64-unknown-linux-musl

FROM scratch
ARG version=unknown
ARG release=unreleased

COPY --from=builder /rust-hello-world/target/release/binaryName /
CMD ["rust-hello-world"]
