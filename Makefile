start:
	cargo run -- --dev --tmp

keystore-add:
	curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "@keystore.json"