FROM paritytech/ci-linux:production as builder
LABEL description="This is the build stage for Jack Block. Here we create the binary."

ARG PROFILE=release
WORKDIR /substrate

COPY . /substrate

RUN cargo build --$PROFILE

# ===== SECOND STAGE ======

FROM debian:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the Jackblock binary."
ARG PROFILE=release

RUN useradd -m -u 1000 -U -s /bin/sh -d /substrate substrate && \
	mkdir -p /substrate/.local/share/substrate && \
	chown -R substrate:substrate /substrate/.local && \
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
