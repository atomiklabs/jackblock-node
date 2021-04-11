#!/usr/bin/env bash

set -e

ENTRYPOINT="${NODE_BIN_PATH:-'/usr/local/bin/node-template'}"

echo "Launching ${ENTRYPOINT} $@"

exec $ENTRYPOINT $@

