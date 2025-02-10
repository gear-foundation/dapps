FROM rust:1.83 AS builder

RUN apt-get update && apt-get install -y git jq

RUN wget -c https://github.com/WebAssembly/binaryen/releases/download/version_119/binaryen-version_119-x86_64-linux.tar.gz -O - | tar -xz -C .
RUN cp binaryen-version_119/bin/wasm-opt /usr/bin/

RUN cargo install sails-cli@0.7.3

RUN rustup show || curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal

RUN rustup install stable
RUN rustup default stable
RUN rustup component add rust-src --toolchain $(rustup show active-toolchain | cut -d ' ' -f1)

WORKDIR /app
