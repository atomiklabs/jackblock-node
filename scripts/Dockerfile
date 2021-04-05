FROM paritytech/ci-linux:974ba3ac-20201006 as builder

RUN apt-get update && \
    apt-get install -y \
        clang \
        llvm

WORKDIR /opt

COPY . /opt

RUN cargo fetch

RUN make node-build

FROM debian:buster-slim as runner

LABEL maintainer "tomasz@kopacki.net"
LABEL description="Binary for Jack Block node"

RUN useradd -m -u 1000 -U -s /bin/sh -d /mruser mruser && \
	mkdir -p /mruser/.local/share && \
	mkdir /data && \
	chown -R mruser:mruser /data && \
	ln -s /data /mruser/.local/share/mruser && \
	rm -rf /usr/bin /usr/sbin

USER mruser

COPY --from=builder --chown=mruser /opt/target/release/node-template /opt/jackblock-node
RUN chmod uog+x /opt/jackblock-node

# 30333 for parachain p2p 
# 9933 for RPC call
# 9944 for Websocket
# 9615 for Prometheus (metrics)
EXPOSE 30333 9933 9944 9615 

VOLUME ["/data"]

ENTRYPOINT ["/opt/jackblock-node"]
