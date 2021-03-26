start:
	cargo run --release -- --dev --tmp

add-private-keys:
	curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "@keystores/private-key-aura.json" && \
	curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "@keystores/private-key-grandpa.json"

node-build:
	cargo build --release

node-0-start:
	./target/release/node-template -lruntime=debug \
	--base-path ./tmp-private-chain/node_0 \
	--chain private \
	--port 30333 \
	--ws-port 9945 \
	--rpc-port 9933 \
	--node-key-file ./keystores/.local-node-identity-secret-key \
	--validator \
	--rpc-methods Unsafe \
	--name jackblock-node-0

node-1-start:
	./target/release/node-template -lruntime=debug \
	--base-path ./tmp-private-chain/node_1 \
	--chain private \
	--port 30334 \
	--ws-port 9946 \
	--rpc-port 9934 \
	--validator \
	--rpc-methods Unsafe \
	--bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
	--name jackblock-node-1

add-all-keys:
	make node-0-add-key-aura && \
	make node-0-add-key-grandpa && \
	make node-0-add-key-jack && \
	make node-1-add-key-aura && \
	make node-1-add-key-grandpa && \
	make node-1-add-key-jack \

node-0-add-key-aura:
	curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "@keystores/node-0-aura.json"

node-0-add-key-grandpa:
	curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "@keystores/node-0-grandpa.json"

node-0-add-key-jack:
	curl http://localhost:9933 -H "Content-Type:application/json;charset=utf-8" -d "@keystores/node-0-jack.json"

node-1-add-key-aura:
	curl http://localhost:9934 -H "Content-Type:application/json;charset=utf-8" -d "@keystores/node-1-aura.json"

node-1-add-key-grandpa:
	curl http://localhost:9934 -H "Content-Type:application/json;charset=utf-8" -d "@keystores/node-1-grandpa.json"

node-1-add-key-jack:
	curl http://localhost:9934 -H "Content-Type:application/json;charset=utf-8" -d "@keystores/node-1-jack.json"