.PHONY: all build fmt init lint pre-commit test full-test deps

NIGHTLY_TOOLCHAIN_VERSION = 2023-03-14
TARGET = `rustc -Vv | grep 'host: ' | sed 's/^host: \(.*\)/\1/'`

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
	@rustup toolchain install nightly-$(NIGHTLY_TOOLCHAIN_VERSION) --component llvm-tools-preview --component clippy
	@rustup target add wasm32-unknown-unknown --toolchain nightly-$(NIGHTLY_TOOLCHAIN_VERSION)
	@rm -rf ~/.rustup/toolchains/nightly-$(TARGET)
	@ln -s ~/.rustup/toolchains/nightly-$(NIGHTLY_TOOLCHAIN_VERSION)-$(TARGET) ~/.rustup/toolchains/nightly-$(TARGET)

lint:
	@echo ⚙️ Running the linter...
	@cargo +nightly clippy --workspace --all-targets -- -D warnings

pre-commit: fmt lint full-test

test:
	@echo ⚙️ Running unit tests...
	@cargo +nightly t

full-test:
	@echo ⚙️ Running all tests...
# TODO: remove the `test-thread` option when multithread tests will be allowed.
	@cargo +nightly t -- --include-ignored --test-threads=1
