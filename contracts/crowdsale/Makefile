.PHONY: all build clean fmt fmt-check init linter pre-commit test

all: init build full-test

clean:
	@echo ──────────── Clean ────────────────────────────
	@rm -rvf target

build:
	@echo ⚙️ Building a release...
	@cargo +nightly b -r --workspace
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

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

deps:
	@if [ ! -f "./target/ft_main.opt.wasm" ]; then\
	    curl -L\
	        "https://github.com/gear-dapps/sharded-fungible-token/releases/download/0.1.2/ft_main-0.1.2.opt.wasm"\
	        -o "./target/ft_main.opt.wasm";\
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

test: build deps
	@echo ⚙️ Running unit tests...
	@cargo +nightly test --release

node-test: build deps
	@echo ⚙️ Running node tests...
	@wget https://get.gear.rs/gear-nightly-linu\x-x86_64.tar.xz && \
	tar xvf gear-nightly-linux-x86_64.tar.xz && \
	rm gear-nightly-linux-x86_64.tar.xz
	@./gear --dev --tmp > /dev/null 2>&1  & echo "$$!" > gear.pid
	cat gear.pid;
	@cargo +nightly t -Fbinary-vendor -- --include-ignored --test node_tests --test-threads=1 kill `(cat gear.pid)`;
	rm gear; rm gear.pid


full-test: deps
	@echo ⚙️ Running all tests...
	@cargo +nightly t --release -Fbinary-vendor -- --include-ignored --test-threads=1