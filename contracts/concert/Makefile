.PHONY: all build fmt init lint pre-commit test deps

all: init build test

build:
	@echo ⚙️ Building a release...
	@cargo +nightly b -r --workspace
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
	@cargo +nightly clippy --workspace --all-targets -- -D warnings

pre-commit: fmt lint test

deps:
	@echo ⚙️ Downloading dependencies...
	@path=target/multi_token.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/multitoken/releases/download/0.3.3/multitoken-0.3.3.opt.wasm\
	        -o $$path;\
	fi

test: deps
	@echo ⚙️ Running tests...
	@cargo +nightly t
