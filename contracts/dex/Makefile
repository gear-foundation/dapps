.PHONY: all build fmt init lint pre-commit deps test full-test

NIGHTLY_TOOLCHAIN_VERSION ?= 2023-03-21
TARGET = `rustc -Vv | grep 'host: ' | sed 's/^host: \(.*\)/\1/'`

all: init build test

build:
	@echo ⚙️ Building a release...
	@cargo +nightly b -r
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

fmt:
	@echo ⚙️ Checking a format...
	@cargo fmt --all --check

init:
	@echo ⚙️ Installing a toolchain \& a target...
	@rustup toolchain install nightly-$(NIGHTLY_TOOLCHAIN_VERSION) -cclippy -twasm32-unknown-unknown
	@rm -rf ~/.rustup/toolchains/nightly-$(TARGET)
	@ln -s ~/.rustup/toolchains/nightly-$(NIGHTLY_TOOLCHAIN_VERSION)-$(TARGET) ~/.rustup/toolchains/nightly-$(TARGET)

lint:
	@echo ⚙️ Running the linter...
	@cargo +nightly clippy -- -D warnings
	@cargo +nightly clippy --all-targets -Fbinary-vendor -- -D warnings

pre-commit: fmt lint full-test

deps:
	@mkdir -p target
	@echo ⚙️ Downloading dependencies...
	@path=target/ft_main.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/sharded-fungible-token/releases/download/2.1.1/ft_main.opt.wasm\
	        -o $$path;\
	fi
	@path=target/ft_logic.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/sharded-fungible-token/releases/download/2.1.1/ft_logic.opt.wasm\
	        -o $$path;\
	fi
	@path=target/ft_storage.wasm;\
	if [ ! -f $$path ]; then\
	    curl -L\
	        https://github.com/gear-dapps/sharded-fungible-token/releases/download/2.1.1/ft_storage.opt.wasm\
	        -o $$path;\
	fi

test: deps
	@echo ⚙️ Running tests...
	@cargo +nightly t -Fbinary-vendor

full-test: deps
	@echo ⚙️ Running tests...
	@cargo +nightly t -Fbinary-vendor -- --include-ignored
