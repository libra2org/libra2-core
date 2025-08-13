#!/bin/bash
set -euo pipefail

DATA_DIR="$HOME/.libra2/localnet"

# Attempt to stop any running localnet instances
if pgrep -f "libra2.*run-localnet" >/dev/null; then
  echo "Stopping running localnet"
  pkill -f "libra2.*run-localnet"
fi

# Remove localnet data directory
if [ -d "$DATA_DIR" ]; then
  echo "Removing localnet data at $DATA_DIR"
  rm -rf "$DATA_DIR"
fi
