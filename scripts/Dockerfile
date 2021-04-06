FROM paritytech/ci-linux:production as builder
LABEL description="This is the build stage for Jack Block. Here we create the binary."

ARG PROFILE=release
WORKDIR /tmp

COPY . /tmp

RUN cargo build --$PROFILE

# ===== SECOND STAGE ======

FROM debian:buster-slim
LABEL description="This is the 2nd stage: a very small image where we copy the Jackblock binary."
ARG PROFILE=release

RUN useradd -m -u 1000 -U -s /bin/sh -d /jackblock jackblock && \
	mkdir -p /jackblock/.local/share && \
	mkdir -p /data && \
	chown -R jackblock:jackblock /data && \
	ln -s /data /jackblock/.local/share/jackblock && \
	rm -rf /usr/bin /usr/sbin

USER jackblock

COPY --from=builder --chown=jackblock /tmp/target/$PROFILE/node-template /usr/local/bin
RUN chmod uog+x /usr/local/bin/node-template

# 30333 for p2p 
# 9933 for RPC call
# 9944 for Websocket
# 9615 for Prometheus (metrics)
EXPOSE 30333 9933 9944 9615
VOLUME ["/data"]

CMD ["/usr/local/bin/node-template"]
