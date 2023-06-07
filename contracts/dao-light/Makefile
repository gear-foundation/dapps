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
	@cargo clippy --workspace --all-targets -- -D warnings

pre-commit: fmt lint full-test

deps:
	@echo ⚙️ Downloading dependencies...
	@path=target/ft.wasm;\
	mkdir -p target && \
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/fungible-token/releases/download/0.1.6/fungible_token.wasm\
			-o $$path;\
	fi

test: deps
	@echo ⚙️ Running unit tests...
	@cargo t

full-test: deps
	@echo ⚙️ Running all tests...
	@cargo t -- --include-ignored
