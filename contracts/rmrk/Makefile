.PHONY: all build fmt init lint pre-commit test full-test deps

all: init build test

build:
	@echo ⚙️ Building a release...
	@cargo b -r --workspace
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

fmt:
	@echo ⚙️ Checking a format...
	@cargo fmt --all --check

init:
	@echo ⚙️ Installing a toolchain \& a target...
	@rustup show

lint:
	@echo ⚙️ Running the linter...
	@cargo clippy -F binary-vendor --workspace --all-targets -- -D warnings 

pre-commit: fmt lint full-test

test:
	@echo ⚙️ Running unit tests...
	@cargo t -Fbinary-vendor -r

full-test:
	@echo ⚙️ Running all tests...
	@cargo t -Fbinary-vendor -r -- --include-ignored --test-threads=1
