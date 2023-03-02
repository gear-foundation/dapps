.PHONY: all build clean fmt fmt-check init linter pre-commit test full-test

all: init build full-test

build:
	@echo ⚙️ Building a release...
	@cargo +nightly b -r --workspace
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

fmt:
	@echo ⚙️ Checking a format...
	@cargo fmt --all --check

init:
	@echo ⚙️ Installing a toolchain \& a target...
	@rustup toolchain add nightly
	@rustup target add wasm32-unknown-unknown --toolchain nightly

lint:
	@echo ⚙️ Running the linter...
	@cargo +nightly clippy --workspace --all-targets -- -D warnings

pre-commit: fmt linter test

deps:
	@echo ⚙️ Downloading dependencies...
	@path=target/ft_main.wasm;\
	mkdir -p target/ && \
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/sharded-fungible-token/releases/download/0.1.4/ft_main-0.1.4.opt.wasm\
	        -o $$path;\
	fi
	@path=target/ft_logic.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/sharded-fungible-token/releases/download/0.1.4/ft_logic-0.1.4.opt.wasm\
	        -o $$path;\
	fi
	@path=target/ft_storage.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/sharded-fungible-token/releases/download/0.1.4/ft_storage-0.1.4.opt.wasm\
	        -o $$path;\
	fi

test: deps
	@echo ⚙️ Running unit tests...
	@cargo +nightly t

full-test: deps
	@echo ⚙️ Running all tests...
	@cargo +nightly t -- --include-ignored --test-threads=1
