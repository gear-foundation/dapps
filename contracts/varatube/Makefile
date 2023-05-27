.PHONY: all build fmt init lint pre-commit test deps full-test

all: init build test

build:
	@echo ⚙️ Building a release...
	@cargo b -r --workspace -Fbinary-vendor
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
	@cargo clippy --workspace -Fbinary-vendor --all-targets -- -D warnings

pre-commit: fmt lint full-test

deps:
	@echo ⚙️ Downloading dependencies...
	@path=target/fungible_token.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/fungible-token/releases/download/0.1.5/fungible_token.wasm\
	        -o $$path;\
	fi
	
test: deps
	@echo ⚙️ Running tests...
	@cargo t -Fbinary-vendor

full-test: deps
	@echo ⚙️ Running all tests...
	@cargo t -Fbinary-vendor -- --include-ignored

		
