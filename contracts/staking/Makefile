.PHONY: all build clean fmt fmt-check init lint pre-commit test full-test

all: init build test

build:
	@echo ⚙️ Building a release...
	@cargo +nightly build --release
	@ls -l ./target/wasm32-unknown-unknown/release/*.wasm

clean:
	@echo ──────────── Clean ────────────────────────────
	@rm -rvf target

fmt:
	@echo ⚙️ Formatting...
	@cargo fmt --all

fmt-check:
	@echo ⚙️ Checking a format...
	@cargo fmt --all --check

init:
	@echo ──────────── Install toolchains ───────────────
	@rustup toolchain add nightly
	@rustup target add wasm32-unknown-unknown --toolchain nightly

lint:
	@echo ⚙️ Running the linter...
	@cargo +nightly clippy --all-targets -- --no-deps -D warnings

pre-commit: fmt lint test

test: build
	@if [ ! -f "./target/fungible_token-0.1.3.wasm" ]; then\
		wget "https://github.com/gear-dapps/fungible-token/releases/download/0.1.3/fungible_token-0.1.3.wasm"\
			-O "./target/fungible_token-0.1.3.wasm";\
	fi
	@echo ⚙️ Building a release...
	@cargo +nightly --release

node-test:
	@if [ ! -f "./target/fungible_token-0.1.3.wasm" ]; then\
		wget "https://github.com/gear-dapps/fungible-token/releases/download/0.1.3/fungible_token-0.1.3.wasm"\
			-O "./target/fungible_token-0.1.3.wasm";\
	fi
	wget https://get.gear.rs/gear-nightly-linu\x-x86_64.tar.xz && \
	tar xvf gear-nightly-linux-x86_64.tar.xz && \
	rm gear-nightly-linux-x86_64.tar.xz
	@./gear --dev --tmp > /dev/null 2>&1  & echo "$$!" > gear.pid
	cat gear.pid;
	@cargo test --package staking --test node_test -- --include-ignored --test-threads=1; 	kill `(cat gear.pid)`; rm gear; rm gear.pid

full-test:
	@if [ ! -f "./target/fungible_token-0.1.3.wasm" ]; then\
		wget "https://github.com/gear-dapps/fungible-token/releases/download/0.1.3/fungible_token-0.1.3.wasm"\
			-O "./target/fungible_token-0.1.3.wasm";\
	fi
	@echo ⚙️ Running all tests...
	@cargo +nightly t -- --include-ignored --test-threads=1
