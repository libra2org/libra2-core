#!/bin/bash
set -euo pipefail

DATA_DIR="$HOME/.libra2/localnet"

# Remove any previous localnet data
echo "Cleaning existing localnet data at $DATA_DIR"
rm -rf "$DATA_DIR"

# Start a new single validator localnet with faucet
# This uses the Libra2 CLI built from this repository.
# It will automatically generate keys, genesis, and configuration.

# Disable the transaction stream for local development. The gRPC transaction stream
# service is not yet implemented in Libra2, so enabling it causes a startup failure.
# See the README for details. The --no-txn-stream flag skips starting the gRPC
# transaction stream and prevents the "Unimplemented" error seen when polling
# the transaction stream endpoint.

cargo run -p libra2 --release -- node run-localnet \
  --test-dir "$DATA_DIR" \
  --force-restart \
  --with-faucet # --no-txn-stream
