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
COPY --from=builder /tmp/target/$PROFILE/node-template /usr/local/bin

RUN useradd -m -u 1000 -U -s /bin/sh -d /jackblock jackblock && \
	mkdir -p /jackblock/.local/share && \
	mkdir /data && \
	chown -R jackblock:jackblock /data && \
	ln -s /data /jackblock/.local/share/jackblock && \
	rm -rf /usr/bin /usr/sbin

USER jackblock
EXPOSE 30333 9933 9944
VOLUME ["/data"]

CMD ["/usr/local/bin/node-template"]
