.PHONY: all contracts fmt fmt-check frontend  full-test init-contracts init-frontend lint pre-commit test test-contracts serve

all: contracts

contracts:
	@echo 🚂 Building contracts...
	@cd contracts && cargo +nightly b -r --workspace
	@ls -l contracts/target/wasm32-unknown-unknown/release/*.wasm

fmt:
	@echo 🚂 Formatting...
	@cd contracts && cargo fmt

fmt-check:
	@echo 🚂 Checking a format...
	@cd contracts && cargo fmt --all --check

frontend:
	@echo 🚂 Building frontend...
	@cd frontend && yarn build

init-contracts:
	@echo 🚂 Installing a toolchain and a target...
	@rustup toolchain add nightly
	@rustup target add wasm32-unknown-unknown --toolchain nightly

init-frontend:
	@echo 🚂 Installing frontent deps...
	@cd frontend && yarn

lint:
	@echo 🚂 Running the linter...
	@cd contracts && cargo +nightly clippy --workspace -- -D warnings

pre-commit: fmt lint test

test: test-contracts

test-contracts:
	@echo 🚂 Running unit tests...
	@cd contracts && cargo +nightly t -Fbinary-vendor

full-test:
	@echo 🚂 Running all tests...
	@cd contracts && cargo +nightly t -Fbinary-vendor -- --include-ignored --test-threads=1

serve:
	@echo 🚂 Running server...
	@cd frontend && yarn start
