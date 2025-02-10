ARG RUST_VERSION=1.83.0
FROM rust:${RUST_VERSION} AS builder

RUN apt-get update && apt-get install -y git jq

RUN wget -c https://github.com/WebAssembly/binaryen/releases/download/version_119/binaryen-version_119-x86_64-linux.tar.gz -O - | tar -xz -C .
RUN cp binaryen-version_119/bin/wasm-opt /usr/bin/

RUN cargo install sails-cli@0.7.3

ARG RUST_VERSION=1.83.0
RUN rustup install ${RUST_VERSION} && rustup default ${RUST_VERSION}

RUN rustup component add rust-src --toolchain ${RUST_VERSION}

WORKDIR /app
