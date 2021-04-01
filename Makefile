run:
	cargo run -- --dev --tmp

keystore-add:
	curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "@keystore.json"

purge:
	cargo run -- purge-chain --dev -y

restart: purge run

check:
	SKIP_WASM_BUILD=1 cargo check

test:
	SKIP_WASM_BUILD=1 cargo test --all

test-lib:
	SKIP_WASM_BUILD=1 cargo test -p pallet-jackblock --lib

build:
	cargo build
