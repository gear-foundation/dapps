.PHONY: all build clean fmt fmt-check init linter pre-commit test

all: init build test

build:
	@echo ──────────── Build release ────────────────────
	@cargo +nightly build --release
	@ls -l ./target/wasm32-unknown-unknown/release/*.wasm

clean:
	@echo ──────────── Clean ────────────────────────────
	@rm -rvf target

fmt:
	@echo ──────────── Format ───────────────────────────
	@cargo fmt --all

fmt-check:
	@echo ──────────── Check format ─────────────────────
	@cargo fmt --all -- --check

init:
	@echo ──────────── Install toolchains ───────────────
	@rustup toolchain add nightly
	@rustup target add wasm32-unknown-unknown --toolchain nightly

linter:
	@echo ──────────── Run linter ───────────────────────
	@cargo +nightly clippy --all-targets -- --no-deps -D warnings -A "clippy::missing_safety_doc"

pre-commit: fmt linter test

test: build
	@if [ ! -f "./target/nft.wasm" ]; then wget "https://github.com/gear-dapps/non-fungible-token/releases/download/build/nft.wasm" -O "./target/nft.wasm"; fi
	@echo ──────────── Run tests ────────────────────────
	@cargo +nightly test --release
