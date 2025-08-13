#!/bin/bash
set -euo pipefail

DATA_DIR="$HOME/.libra2/localnet"

NO_TXN_STREAM=false
TXN_ADDR="127.0.0.1"
TXN_PORT="50051"
ARGS=()
while [[ $# -gt 0 ]]; do
  case $1 in
    --no-txn-stream)
      NO_TXN_STREAM=true
      ARGS+=("$1")
      shift
      ;;
    --txn-stream-addr)
      TXN_ADDR="$2"
      ARGS+=("$1" "$2")
      shift 2
      ;;
    --txn-stream-port)
      TXN_PORT="$2"
      ARGS+=("$1" "$2")
      shift 2
      ;;
    *)
      ARGS+=("$1")
      shift
      ;;
  esac
done

echo "Cleaning existing localnet data at $DATA_DIR"
rm -rf "$DATA_DIR"

if [ "$NO_TXN_STREAM" = false ]; then
  echo "gRPC txn-stream endpoint: ${TXN_ADDR}:${TXN_PORT}"
fi

cargo run -p libra2 --release -- node run-localnet \
  --test-dir "$DATA_DIR" \
  --force-restart \
  --with-faucet \
  "${ARGS[@]}"
