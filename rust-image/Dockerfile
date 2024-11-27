FROM ubuntu:22.04

MAINTAINER GEAR-FOUNDATION

# Set default Rust toolchain version
ARG DEFAULT_TOOLCHAIN='1.78.0'

# Set environment variables for Rust toolchain and Cargo
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=${DEFAULT_TOOLCHAIN}

# Install dependencies, Rust and set up environment
RUN apt update -y && \
    apt install -y clang gcc gzip curl unzip cmake && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain $RUST_VERSION && \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME && \
    rustup update $RUST_VERSION && \
    rustup target add wasm32-unknown-unknown --toolchain $RUST_VERSION && \
    rustup default $RUST_VERSION && \
    rm -rf /var/lib/apt/lists/* /root/.cargo/registry /root/.cargo/git

# Display installed versions of Rust tools
RUN rustup --version && cargo --version && rustc --version

# Set the working directory for the application
WORKDIR /contracts

# Set default command to build the project using Cargo
CMD ["cargo", "build"]
