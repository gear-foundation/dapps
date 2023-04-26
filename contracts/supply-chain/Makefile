.PHONY: all build fmt init lint pre-commit test full-test deps

all: init build test

build:
	@echo ⚙️ Building a release...
	@cargo +nightly b -r --workspace --exclude deploy
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

fmt:
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
	@rustup toolchain install nightly-2023-03-14 --component llvm-tools-preview
	@rustup target add wasm32-unknown-unknown --toolchain nightly-2023-03-14
	@rm -rf ~/.rustup/toolchains/nightly-aarch64-apple-darwin
	@ln -s ~/.rustup/toolchains/nightly-2023-03-14-aarch64-apple-darwin ~/.rustup/toolchains/nightly-aarch64-apple-darwin

pin-toolchain-linux:
	@rustup toolchain install nightly-2023-03-14 --component llvm-tools-preview
	@rustup target add wasm32-unknown-unknown --toolchain nightly-2023-03-14
	@rm -rf ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu
	@ln -s ~/.rustup/toolchains/nightly-2023-03-14-x86_64-unknown-linux-gnu ~/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu
	@rustup component add clippy --toolchain nightly-x86_64-unknown-linux-gnu

lint:
	@echo ⚙️ Running the linter...
	@cargo +nightly clippy --workspace --all-targets -- -D warnings

pre-commit: fmt lint full-test

deps:
	@mkdir -p target
	@echo ⚙️ Downloading dependencies...
	@path=target/ft-main.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/sharded-fungible-token/releases/download/2.1.1/ft_main.opt.wasm\
	        -o $$path;\
	fi
	@path=target/ft-logic.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/sharded-fungible-token/releases/download/2.1.1/ft_logic.opt.wasm\
	        -o $$path;\
	fi
	@path=target/ft-storage.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/sharded-fungible-token/releases/download/2.1.1/ft_storage.opt.wasm\
	        -o $$path;\
	fi
	@path=target/nft-state.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/non-fungible-token/releases/download/0.2.10/nft_state-0.2.10.meta.wasm\
	        -o $$path;\
	fi
	@path=target/nft.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/non-fungible-token/releases/download/0.2.10/nft-0.2.10.opt.wasm\
	        -o $$path;\
	fi

test: deps
	@echo ⚙️ Running unit tests...
	@cargo +nightly t

full-test: deps
	@echo ⚙️ Running all tests...
	# TODO: remove the `test-thread` option when multithread tests will be
	# allowed.
	@cargo +nightly t -- --include-ignored --test-threads=1
