FROM ubuntu:22.04

MAINTAINER GEAR-FOUNDATION

ARG DEFAULT_TOOLCHAIN='stable'

# Install dependencies
RUN apt update -y && \
    apt install -y clang gcc gzip curl unzip cmake; \
    rm -rf /var/cache/apt/archives /var/lib/apt/lists/*

# Install Rust and toolchains
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --default-toolchain $DEFAULT_TOOLCHAIN; \
    rustup update $DEFAULT_TOOLCHAIN && \
    rustup target add wasm32-unknown-unknown --toolchain $DEFAULT_TOOLCHAIN; \
    rustup default $DEFAULT_TOOLCHAIN;

## Download and install wasm-opt
RUN curl -L https://github.com/WebAssembly/binaryen/releases/download/version_117/binaryen-version_117-x86_64-linux.tar.gz | tar xz && \
    mv binaryen-version_117/bin/wasm-opt /usr/local/bin/wasm-opt && \
    rm -rf binaryen-version_117

WORKDIR /contracts
COPY . /contracts

# Use the shell script as the default command
CMD ["cargo","build"]