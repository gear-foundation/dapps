.PHONY: all build clean fmt fmt-check init linter pre-commit test

all: init build full-test

clean:
	@echo ──────────── Clean ────────────────────────────
	@rm -rvf target

build:
	@echo ⚙️ Building a release...
	@cargo +nightly b -r --workspace -Fbinary-vendor
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

debug-build:
	@echo ──────────── Build debug ────────────────────
	@cargo +nightly build
	@ls -l ./target/wasm32-unknown-unknown/debug/*.wasm

fmt:
	@echo ⚙️ Formatting...
	@cargo fmt --all

fmt-check:
	@echo ⚙️ Checking a format...
	@cargo fmt --all --check

init:
	@echo ⚙️ Installing a toolchain \& a target...
	@rustup toolchain add nightly
	@rustup target add wasm32-unknown-unknown --toolchain nightly

lint:
	@echo ⚙️ Running the linter...
	@cargo +nightly clippy -- -D warnings
	@cargo +nightly clippy \
	--all-targets \
	--workspace \
	-Fbinary-vendor \
	-- -D warnings

pre-commit: fmt lint full-test

test:
	@echo ⚙️ Running unit tests...
	@cargo +nightly t --release -Fbinary-vendor

node-test: deps
	@echo ⚙️ Running node tests...
	@wget https://get.gear.rs/gear-nightly-linu\x-x86_64.tar.xz && \
	tar xvf gear-nightly-linux-x86_64.tar.xz && \
	rm gear-nightly-linux-x86_64.tar.xz
	@./gear --dev --tmp > /dev/null 2>&1  & echo "$$!" > gear.pid
	cat gear.pid;
	@cargo +nightly t -r --workspace -Fbinary-vendor -- --include-ignored --test node_tests --test-threads=1 kill `(cat gear.pid)`;
	rm gear; rm gear.pid


full-test:
	@echo ⚙️ Running all tests...
	@cargo +nightly t -r --workspace -Fbinary-vendor -- --include-ignored --test-threads=1