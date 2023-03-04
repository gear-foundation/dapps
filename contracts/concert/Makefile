.PHONY: all build fmt init lint pre-commit test deps

all: init build test

build:
	@echo ⚙️ Building a release...
	@cargo +nightly b -r --workspace -Fbinary-vendor
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
	@cargo +nightly clippy --workspace -Fbinary-vendor --all-targets -- -D warnings

pre-commit: fmt lint test

deps:
	@echo ⚙️ Downloading dependencies...
	@mkdir -p target;
	@path=target/multi_token.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/multitoken/releases/download/0.3.4/multitoken.release.opt.wasm\
	        -o $$path;\
	fi

test: deps
	@echo ⚙️ Running tests...
	@cargo +nightly t --workspace -Fbinary-vendor

full-test:
	@echo ⚙️ Running all tests...
	@cargo +nightly t -r --workspace -Fbinary-vendor -- --include-ignored --test-threads=1