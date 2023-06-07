.PHONY: all build fmt init lint pre-commit deps test full-test

all: init build test

build:
	@echo ⚙️ Building a release...
	@cargo b -r
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
	@cargo clippy --all-targets -Fbinary-vendor -- -D warnings

pre-commit: fmt lint full-test

SFT_VERSION = 2.1.2

deps:
	@mkdir -p target
	@echo ⚙️ Downloading dependencies...
	@path=target/ft_main.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/sharded-fungible-token/releases/download/$(SFT_VERSION)/ft_main.opt.wasm\
	        -o $$path;\
	fi
	@path=target/ft_logic.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/sharded-fungible-token/releases/download/$(SFT_VERSION)/ft_logic.opt.wasm\
	        -o $$path;\
	fi
	@path=target/ft_storage.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/sharded-fungible-token/releases/download/$(SFT_VERSION)/ft_storage.opt.wasm\
	        -o $$path;\
	fi

test: deps
	@echo ⚙️ Running tests...
	@cargo t -Fbinary-vendor

full-test: deps
	@echo ⚙️ Running tests...
	@cargo t -Fbinary-vendor -- --include-ignored
