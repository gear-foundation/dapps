FROM ubuntu:22.04

MAINTAINER GEAR-FOUNDATION

ARG DEFAULT_TOOLCHAIN='1.78.0'
#'stable'

ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=${DEFAULT_TOOLCHAIN}

# Install dependencies
RUN apt update -y && \
    apt install -y clang gcc gzip curl unzip cmake wget; \
    rm -rf /var/cache/apt/archives /var/lib/apt/lists/*

# Install Rust and toolchains
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- -y --default-toolchain $RUST_VERSION; \
    chmod -R a+w $RUSTUP_HOME $CARGO_HOME; \
    rustup update $RUST_VERSION && \
    rustup target add wasm32-unknown-unknown --toolchain $RUST_VERSION; \
    rustup default $RUST_VERSION;

## Download and install wasm-opt
RUN curl -L https://github.com/WebAssembly/binaryen/releases/download/version_117/binaryen-version_117-x86_64-linux.tar.gz | tar xz && \
    mv binaryen-version_117/bin/wasm-opt /usr/local/bin/wasm-opt && \
    rm -rf binaryen-version_117

## Show versions
RUN rustup --version; \
    cargo --version; \
    rustc --version;

## Clean up
RUN apt-get autoremove -y && \
	apt-get clean && \
	rm -rf /var/lib/apt/lists/* && \
	# cargo clean up
	# removes compilation artifacts cargo install creates (>250M)
	rm -rf "${CARGO_HOME}/registry" "${CARGO_HOME}/git" /root/.cache/sccache

WORKDIR /contracts

# Use the shell script as the default command
CMD ["cargo","build"]