PHONY: all build clean fmt fmt-check init lint pre-commit test full-test

all: init build test

build:
	@echo ⚙️ Building a release...
	@cargo +nightly b -r --workspace
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

fmt:
	@echo ⚙️ Formatting...
	@cargo fmt --all

fmt-check:
	@echo ⚙️ Checking a format...
	@cargo fmt --all --check

init:
	@echo ⚙️ Installing a toolchain \& a target...
	@rustup toolchain add nightly
	@rustup target add wasm32-unknown-unknown --toolchain nightly

lint:
	@echo ⚙️ Running the linter...
	@cargo +nightly clippy --workspace -- -D warnings
	@cargo +nightly clippy \
	--all-targets \
	--workspace \
	-Fbinary-vendor \
	-- -D warnings

pre-commit: fmt lint full-test

test: build
	@echo ──────────── Run tests ────────────────────────
	@cargo +nightly test --release --workspace -Fbinary-vendor \

full-test:
	@echo ⚙️ Running all tests...
	@cargo +nightly t --release --workspace -Fbinary-vendor -- --include-ignored
