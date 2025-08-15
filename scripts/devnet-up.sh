#!/usr/bin/env bash
set -euo pipefail

# --- Config -------------------------------------------------------------------
DATA_DIR="${DATA_DIR:-"$HOME/.libra2/localnet"}"
PIDS_DIR="$DATA_DIR/pids"
LOGS_DIR="$DATA_DIR/logs"

READINESS_URL="http://127.0.0.1:8070/"
REST_URL="http://127.0.0.1:8080"
FAUCET_URL="http://127.0.0.1:8081"

# txnstream gRPC (external server we manage here)
TXN_ADDR="${TXN_ADDR:-127.0.0.1:50052}"
TXN_BIN="txnstream-server" # produced by libra2-txnstream-server crate (see section 2)
TXN_PIDFILE="$PIDS_DIR/txnstream.pid"
TXN_LOG="$LOGS_DIR/txnstream.log"

# node args
FORCE_RESTART="--force-restart"
WITH_FAUCET="--with-faucet"
NO_TXN_STREAM="--no-txn-stream"   # always disable legacy gRPC to avoid port collisions
TEST_DIR_ARG="--test-dir $DATA_DIR"

# --- Helpers ------------------------------------------------------------------
exists_pid() { local p="${1:-}"; [[ -n "$p" ]] && ps -p "$p" >/dev/null 2>&1; }
wait_listen() {
  local hostport="$1" timeout="${2:-30}"
  local start end
  start="$(date +%s)"
  while true; do
    if lsof -n -iTCP:"${hostport#*:}" -sTCP:LISTEN -P 2>/dev/null | grep -q "${hostport%:*}"; then
      return 0
    fi
    end="$(date +%s)"
    if (( end - start >= timeout )); then
      return 1
    fi
    sleep 1
  done
}

clean_data_dir() {
  echo "Cleaning existing localnet data at $DATA_DIR"
  rm -rf "$DATA_DIR"
  mkdir -p "$PIDS_DIR" "$LOGS_DIR"
}

build_txnstream_bin() {
  # Build only once if not present
  if [[ ! -x "target/release/$TXN_BIN" ]]; then
    cargo build -p libra2-txnstream-server --release --bins
  fi
}

start_txnstream() {
  echo "Starting $TXN_BIN at $TXN_ADDR ..."
  build_txnstream_bin
  # Rotate previous log
  if [[ -f "$TXN_LOG" ]]; then mv -f "$TXN_LOG" "$TXN_LOG.$(date +%s).bak"; fi

  # Start in background; write PID
  ( "target/release/$TXN_BIN" --addr "$TXN_ADDR" --rest "$REST_URL" >>"$TXN_LOG" 2>&1 ) &
  echo $! > "$TXN_PIDFILE"

  echo "TxnStream gRPC is starting, please wait..."
  if ! wait_listen "$TXN_ADDR" 45; then
    echo "ERROR: TxnStream gRPC on $TXN_ADDR did not become ready in 45s"
    exit 1
  fi
  echo "TxnStream gRPC is ready. Endpoint: http://$TXN_ADDR/"
}

start_node() {
  # Build node (if needed)
  cargo build -p libra2 --release >/dev/null 2>&1 || true

  echo
  echo "Readiness endpoint: $READINESS_URL"
  echo

  # Start node run-localnet (legacy txn-stream disabled)
  target/release/libra2 node run-localnet \
    $TEST_DIR_ARG \
    $FORCE_RESTART \
    $WITH_FAUCET \
    $NO_TXN_STREAM \
    2>&1 | tee "$DATA_DIR/validator.log"
}

# --- Main ---------------------------------------------------------------------
clean_data_dir

# Start external txnstream (on its own port; legacy disabled on node)
start_txnstream

# Start node (REST + faucet)
start_node

# Post startup banner (best effort; the node prints its own readiness lines)
echo
echo "Applying post startup steps..."
sleep 1
echo
echo "Setup is complete, you can now use the localnet!"
echo