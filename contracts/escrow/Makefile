.PHONY: all build clean fmt fmt-check init linter pre-commit test

all: init build test

build:
	@echo ──────────── Build release ────────────────────
	@cargo +nightly build --release
	@ls -l ./target/wasm32-unknown-unknown/release/*.wasm

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
	@echo ──────────── Install toolchains ───────────────
	@rustup toolchain add nightly
	@rustup target add wasm32-unknown-unknown --toolchain nightly

linter:
	@echo ──────────── Run linter ───────────────────────
	@cargo +nightly clippy --all-targets -- --no-deps -D warnings

pre-commit: fmt linter test

test: build
	@if [ ! -f "./target/ft_main.wasm" ]; then\
	    curl -L\
	        "https://github.com/gear-dapps/sharded-fungible-token/releases/download/0.1.2/ft_main-0.1.2.opt.wasm"\
	        -o "./target/ft_main.wasm";\
	fi
	@if [ ! -f "./target/ft_logic.opt.wasm" ]; then\
	    curl -L\
	        "https://github.com/gear-dapps/sharded-fungible-token/releases/download/0.1.2/ft_logic-0.1.2.opt.wasm"\
	        -o "./target/ft_logic.opt.wasm";\
	fi
	@if [ ! -f "./target/ft_storage.opt.wasm" ]; then\
	    curl -L\
	        "https://github.com/gear-dapps/sharded-fungible-token/releases/download/0.1.2/ft_storage-0.1.2.opt.wasm"\
	        -o "./target/ft_storage.opt.wasm";\
	fi
	@echo ──────────── Run tests ────────────────────────
	@cargo +nightly test --package escrow --test cancel --test confirm --test create --test deposit --test other --test refund
	# @wget https://get.gear.rs/gear-nightly-linu\x-x86_64.tar.xz && \
	# tar xvf gear-nightly-linux-x86_64.tar.xz && \
	# rm gear-nightly-linux-x86_64.tar.xz
	# @./gear --dev --tmp > /dev/null 2>&1  & echo "$$!" > gear.pid
	# cat gear.pid;
	# @cargo +nightly test --package escrow --test node_test -- --test-threads=1; 	kill `(cat gear.pid)`; rm gear; rm gear.pid