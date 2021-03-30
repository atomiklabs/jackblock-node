PORT_0?=30333
WS_PORT_0?=9945
RPC_PORT_0?=9933

PORT_1?=30334
WS_PORT_1?=9946
RPC_PORT_1?=9934

BASE_PATH_PREFIX?=./tmp-private-chain
KEYS_PATH_PREFIX?=keys
TELEMETRY_URL?='wss://telemetry.polkadot.io/submit/ 0'
NODE_KEY?=0000000000000000000000000000000000000000000000000000000000000001 # PRIVATE SEED FOR LOCAL NODE IDENTITY
BOOT_NODE_PREFIX?=/ip4/127.0.0.1/tcp/$(PORT_0)/p2p
BOOT_NODES?=$(BOOT_NODE_PREFIX)/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp

PRIVATE_CHAIN_SPEC?=./privateChainSpecRaw.json

start:
	cargo run --release -- --dev --tmp

node-build:
	cargo build --release

local-node-0-start:
	./target/release/node-template -lruntime=debug \
	--base-path $(BASE_PATH_PREFIX)/node_0 \
	--chain local \
	--port $(PORT_0) \
	--ws-port $(WS_PORT_0) \
	--rpc-port $(RPC_PORT_0) \
	--node-key $(NODE_KEY) \
	--validator \
	--rpc-methods Unsafe \
	--telemetry-url $(TELEMETRY_URL) \
	--name jackblock-node-0

local-node-1-start:
	./target/release/node-template -lruntime=debug \
	--base-path $(BASE_PATH_PREFIX)/node_1 \
	--chain local \
	--port $(PORT_1) \
	--ws-port $(WS_PORT_1) \
	--rpc-port $(RPC_PORT_1) \
	--validator \
	--rpc-methods Unsafe \
	--telemetry-url $(TELEMETRY_URL) \
	--bootnodes $(BOOT_NODES) \
	--name jackblock-node-1

local-add-all-keys:
	make local-node-0-add-key-aura && \
	make local-node-0-add-key-grandpa && \
	make local-node-0-add-key-jack && \
	make local-node-1-add-key-aura && \
	make local-node-1-add-key-grandpa && \
	make local-node-1-add-key-jack \

local-node-0-add-key-aura:
	curl http://localhost:$(RPC_PORT_0) -H "Content-Type:application/json;charset=utf-8" -d "@$(KEYS_PATH_PREFIX)/local-node-0-aura.json"

local-node-0-add-key-grandpa:
	curl http://localhost:$(RPC_PORT_0) -H "Content-Type:application/json;charset=utf-8" -d "@$(KEYS_PATH_PREFIX)/local-node-0-grandpa.json"

local-node-0-add-key-jack:
	curl http://localhost:$(RPC_PORT_0) -H "Content-Type:application/json;charset=utf-8" -d "@$(KEYS_PATH_PREFIX)/local-node-0-jack.json"

local-node-1-add-key-aura:
	curl http://localhost:$(RPC_PORT_1) -H "Content-Type:application/json;charset=utf-8" -d "@$(KEYS_PATH_PREFIX)/local-node-1-aura.json"

local-node-1-add-key-grandpa:
	curl http://localhost:$(RPC_PORT_1) -H "Content-Type:application/json;charset=utf-8" -d "@$(KEYS_PATH_PREFIX)/local-node-1-grandpa.json"

local-node-1-add-key-jack:
	curl http://localhost:$(RPC_PORT_1) -H "Content-Type:application/json;charset=utf-8" -d "@$(KEYS_PATH_PREFIX)/local-node-1-jack.json"


private-boot-node-start:
	./target/release/node-template -lruntime=debug \
	--base-path $(BASE_PATH_PREFIX)/private \
	--chain $(PRIVATE_CHAIN_SPEC) \
	--port $(PORT_0) \
	--ws-port $(WS_PORT_0) \
	--rpc-port $(RPC_PORT_0) \
	--validator \
	--rpc-methods Unsafe \
	--name jackblock-private-boot-node

private-node-start:
	./target/release/node-template -lruntime=debug \
	--base-path $(BASE_PATH_PREFIX)/$(NAME) \
	--chain $(PRIVATE_CHAIN_SPEC) \
	--port $(PORT_0) \
	--ws-port $(WS_PORT_0) \
	--rpc-port $(RPC_PORT_0) \
	--validator \
	--rpc-methods Unsafe \
	--bootnodes $(BOOT_NODE_PREFIX)/$(BOOT_NODE_IDENTITY) \
	--name $(NAME)

private-node-add-keys:
	curl http://localhost:$(RPC_PORT_0) -H "Content-Type:application/json;charset=utf-8" -d "@$(KEYS_PATH_PREFIX)/private-key-aura.json" && \
	curl http://localhost:$(RPC_PORT_0) -H "Content-Type:application/json;charset=utf-8" -d "@$(KEYS_PATH_PREFIX)/private-key-grandpa.json"