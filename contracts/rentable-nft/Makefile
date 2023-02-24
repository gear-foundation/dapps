.PHONY: all build clean fmt fmt-check init lint pre-commit test

all: init build full-test

build:
	@echo ⚙️ Building a release...
	@cargo +nightly b -r --workspace
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

clean:
	@echo ──────────── Clean ────────────────────────────
	@rm -rvf target

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

pre-commit: fmt lint test

deps:
	@if [ ! -f "./target/fungible_token-0.1.0.wasm" ]; then \
		curl -L "https://github.com/gear-dapps/fungible-token/releases/download/0.1.0/fungible_token-0.1.0.wasm" \
		-o "./target/fungible_token-0.1.0.wasm"; \
	fi

test: deps
	@echo ⚙️ Running unit tests...
	@cargo +nightly t

full-test:
	@echo ⚙️ Running all tests...
	@cargo +nightly t -Fbinary-vendor -- --include-ignored --test-threads=1
