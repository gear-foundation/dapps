.PHONY: all build clean fmt fmt-check init linter pre-commit test full-test

all: init build full-test

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
	@cargo clippy -- -D warnings
	@cargo clippy -Fbinary-vendor --workspace --all-targets -- -D warnings

pre-commit: fmt linter test

test:
	@echo ⚙️ Running unit tests...
	@cargo t -Fbinary-vendor

full-test:
	@echo ⚙️ Running all tests...
	@cargo t -- --include-ignored
