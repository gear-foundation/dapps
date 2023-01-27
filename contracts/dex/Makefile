.PHONY: all build fmt init lint pre-commit test deps

all: init build test

build:
	@echo ⚙️ Building a release...
	@cargo +nightly b -r
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

fmt:
	@echo ⚙️ Checking a format...
	@cargo fmt --all --check

init:
	@echo ⚙️ Installing a toolchain \& a target...
	@rustup toolchain add nightly
	@rustup target add wasm32-unknown-unknown --toolchain nightly

lint:
	@echo ⚙️ Running the linter...
	@cargo +nightly clippy -- -D warnings
	@cargo +nightly clippy --all-targets -Fbinary-vendor -- -D warnings

pre-commit: fmt lint test

deps:
	@echo ⚙️ Downloading dependencies...
	@path=target/fungible_token.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/fungible-token/releases/download/0.1.3/fungible_token-0.1.3.wasm\
	        -o $$path;\
	fi

test: deps
	@echo ⚙️ Running tests...
	@cargo +nightly t -Fbinary-vendor
