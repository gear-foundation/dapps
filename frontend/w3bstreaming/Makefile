.PHONY: init build build_contract build_js

init:
	@echo ⚙️ Installing dependencies...
	@npm install

build_contract:
	@echo ⚙️ Building contract...
	@cargo build --release
	@ls -l target/wasm32-unknown-unknown/release/*.wasm

build_js:
	@echo ⚙️ Building js packages...
	@npm run build

build: build_contract build_js

run_server:
	@cd packages/signaling-server && npm run start

run_fe:
	@cd packages/frontend && npm run start

dev_server:
	@cd packages/signaling-server && npm run watch
