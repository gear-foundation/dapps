.PHONY: all build clean fmt fmt-check init linter pre-commit test

all: init build test

build:
	@echo ──────────── Build release ────────────────────
	@cargo build --release

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
	@echo ⚙️ Installing a toolchain \& a target...
	@rustup show

linter:
	@echo ──────────── Run linter ───────────────────────
	@cargo clippy --all-targets -- --no-deps -D warnings

pre-commit: fmt linter test

test: build
	@echo ──────────── Run tests ────────────────────────
	@cargo test --release
