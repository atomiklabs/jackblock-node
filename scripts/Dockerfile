# ===== PREPARATION STAGE ======
FROM paritytech/ci-linux:production as planner
WORKDIR /substrate
# We only pay the installation cost once, 
# it will be cached from the second build onwards
# To ensure a reproducible build consider pinning 
# the cargo-chef version with `--version X.X.X`
RUN cargo install cargo-chef 
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

# ===== READ-TRHOUGH CACHE STAGE ======

FROM paritytech/ci-linux:production as cacher
WORKDIR /substrate
RUN cargo install cargo-chef
COPY --from=planner /substrate/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

# ===== OPTIMIZED COMPILE STAGE ======

FROM paritytech/ci-linux:production as builder
LABEL description="This is the build stage for Jack Block. Here we create the binary."

ARG PROFILE=release
WORKDIR /substrate

COPY . /substrate

# Copy over the cached dependencies
COPY --from=cacher /substrate/target /substrate/target
COPY --from=cacher /usr/local/cargo /usr/local/cargo

RUN cargo build --$PROFILE

# ===== BUILD STAGE ======

FROM debian:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the Jackblock binary."
ARG PROFILE=release

# install tools and dependencies
RUN apt-get update && \
	DEBIAN_FRONTEND=noninteractive apt-get upgrade -y && \
	DEBIAN_FRONTEND=noninteractive apt-get install -y \
		curl && \
	# apt cleanup
	apt-get autoremove -y && \
	apt-get clean && \
	find /var/lib/apt/lists/ -type f -not -name lock -delete; \
	# add user and create its home directory
	useradd -m -u 1000 -U -s /bin/sh -d /substrate substrate && \
	mkdir -p /substrate/.local/share/substrate && \
	# make sure the user has correct rights
	chown -R substrate:substrate /substrate/.local && \
	# make sure whatever the substrate state is, it's going to be available to export via volume
	ln -s /substrate/.local/share/substrate /data

COPY --from=builder /substrate/target/$PROFILE/node-template /usr/local/bin

# checks
RUN ldd /usr/local/bin/node-template && \
	/usr/local/bin/node-template --version

USER substrate

# 30333 for p2p 
# 9933 for RPC call
# 9944 for Websocket
# 9615 for Prometheus (metrics)
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

CMD ["/usr/local/bin/node-template"]
