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

SFT_VERSION = 2.1.2

deps:
	@echo ⚙️ Downloading dependencies...
	@path=target/ft_main.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/sharded-fungible-token/releases/download/$(SFT_VERSION)/ft_main.wasm\
	        -o $$path;\
	fi
	@path=target/ft_logic.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/sharded-fungible-token/releases/download/$(SFT_VERSION)/ft_logic.wasm\
	        -o $$path;\
	fi
	@path=target/ft_storage.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/sharded-fungible-token/releases/download/$(SFT_VERSION)/ft_storage.wasm\
	        -o $$path;\
	fi

test: deps
	@echo ⚙️ Running unit tests...
	@cargo t

full-test: deps
	@echo ⚙️ Running all tests...
	@cargo t -- --include-ignored --test-threads=1
