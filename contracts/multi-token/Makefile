.PHONY: all
all: init build test

.PHONY: build
build:
	@ cargo build --workspace --release
	@ ls -l ./target/wasm32-unknown-unknown/release/*.wasm

.PHONY: test
test:
	@ cargo test --workspace --release

.PHONY: init
init:
	@ rustup toolchain add nightly
	@ rustup target add wasm32-unknown-unknown --toolchain nightly
