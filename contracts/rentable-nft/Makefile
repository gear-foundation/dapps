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
ifeq ($(shell uname -s),Linux)
	@echo Linux detected..
	make pin-toolchain-linux
else ifeq ($(shell uname -s),Darwin)
	@echo Macos detected..
	make pin-toolchain-mac-m1
endif

pin-toolchain-mac-m1:
	@rustup toolchain install nightly-2023-03-20 --component llvm-tools-preview
	@rustup target add wasm32-unknown-unknown --toolchain nightly-2023-03-20
	@rm -rf ~/.rustup/toolchains/nightly-aarch64-apple-darwin
	@ln -s ~/.rustup/toolchains/nightly-2023-03-20-aarch64-apple-darwin ~/.rustup/toolchains/nightly-aarch64-apple-darwin

pin-toolchain-linux:
	@rustup toolchain install nightly-2023-03-20 --component llvm-tools-preview
	@rustup target add wasm32-unknown-unknown --toolchain nightly-2023-03-20
	@rm -rf ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu
	@ln -s ~/.rustup/toolchains/nightly-2023-03-20-x86_64-unknown-linux-gnu ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu
	@rustup component add clippy --toolchain nightly-x86_64-unknown-linux-gnu

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
