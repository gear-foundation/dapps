FROM rust:1.82 AS builder

RUN apt-get update
RUN apt-get install -y git jq

RUN wget -c https://github.com/WebAssembly/binaryen/releases/download/version_119/binaryen-version_119-x86_64-linux.tar.gz -O - | tar -xz -C .
RUN cp binaryen-version_119/bin/wasm-opt /usr/bin/
RUN cargo install sails-cli@0.7.3

WORKDIR /app
