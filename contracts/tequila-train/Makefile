.PHONY: all contracts deploy fmt fmt-check frontend full-test init-contracts init-frontend lint nginx node pre-commit restart test test-contracts serve

NIGHTLY_TOOLCHAIN_VERSION ?= 2023-03-14
TARGET = `rustc -Vv | grep 'host: ' | sed 's/^host: \(.*\)/\1/'`

all: contracts

contracts:
	@echo 🚂 Building contracts...
	@cd contracts && cargo b -r --workspace
	@ls -l contracts/target/wasm32-unknown-unknown/release/*.wasm

deploy: frontend
	@echo 🚂 Deploy frontend...
	@ansible-playbook ansible/deploy.yml -i tequila.rs, -u ec2-user

fmt:
	@echo 🚂 Formatting...
	@cd contracts && cargo fmt

fmt-check:
	@echo 🚂 Checking a format...
	@cd contracts && cargo fmt --all --check

frontend:
	@echo 🚂 Building frontend...
	@cp frontend/.env.example.local frontend/.env
	@cd frontend && yarn build

full-test:
	@echo 🚂 Running all tests...
	@cd contracts && cargo t -Fbinary-vendor -- --include-ignored --test-threads=1

init-contracts:
	@echo ⚙️ Installing a toolchain \& a target...
	@rustup toolchain install nightly-$(NIGHTLY_TOOLCHAIN_VERSION) --component llvm-tools-preview --component clippy
	@rustup target add wasm32-unknown-unknown --toolchain nightly-$(NIGHTLY_TOOLCHAIN_VERSION)
	@rm -rf ~/.rustup/toolchains/nightly-$(TARGET)
	@ln -s ~/.rustup/toolchains/nightly-$(NIGHTLY_TOOLCHAIN_VERSION)-$(TARGET) ~/.rustup/toolchains/nightly-$(TARGET)

init-frontend:
	@echo 🚂 Installing frontent deps...
	@cd frontend && yarn

lint:
	@echo 🚂 Running the linter...
	@cd contracts && cargo clippy --workspace -- -D warnings

nginx:
	@echo 🚂 Configuring Nginx...
	@ansible-playbook ansible/configure-nginx.yml -i tequila.rs, -u ec2-user

node:
	@echo 🚂 Configuring Gear node...
	@ansible-playbook ansible/configure-node.yml -i node.tequila-train.com, -u ec2-user

pre-commit: fmt lint test

restart:
	@echo 🚂 Restarting Gear node...
	@ansible-playbook ansible/restart-node.yml -i node.tequila-train.com, -u ec2-user

test: test-contracts

test-contracts:
	@echo 🚂 Running unit tests...
	@cd contracts && cargo t && cargo test -p tequila-io

serve: frontend
	@echo 🚂 Running server...
	@cd frontend && yarn start
