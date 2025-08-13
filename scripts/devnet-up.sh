#!/bin/bash
set -euo pipefail

DATA_DIR="$HOME/.libra2/localnet"

# Remove any previous localnet data
echo "Cleaning existing localnet data at $DATA_DIR"
rm -rf "$DATA_DIR"

# Start a new single validator localnet with faucet
# This uses the Libra2 CLI built from this repository.
# It will automatically generate keys, genesis, and configuration.

cargo run -p libra2 --release -- node run-localnet --test-dir "$DATA_DIR" --force-restart --with-faucet
