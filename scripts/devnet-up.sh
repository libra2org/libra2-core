#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
DATA_DIR="${DATA_DIR:-"$HOME/.libra2/localnet"}"
PIDS_DIR="$DATA_DIR/pids"
LOG_FILE="$DATA_DIR/validator.log"

READINESS_URL="http://127.0.0.1:8070/"
REST_URL="http://127.0.0.1:8080"
FAUCET_URL="http://127.0.0.1:8081"

TXN_ADDR="${TXN_ADDR:-127.0.0.1:50052}"
TXN_BIN="txnstream-server"
TXN_PIDFILE="$PIDS_DIR/txnstream.pid"
NODE_PIDFILE="$PIDS_DIR/node.pid"
USE_EXTERNAL_TXNSTREAM="${USE_EXTERNAL_TXNSTREAM:-0}"

log() {
  echo "[$(date -u +%Y-%m-%dT%H:%M:%SZ)] $*" | tee -a "$LOG_FILE"
}

ensure_port_free() {
  local port="$1"
  if lsof -n -iTCP:"$port" -sTCP:LISTEN -P >/dev/null 2>&1; then
    log "ERROR: port $port is already in use"
    exit 1
  fi
}

wait_port() {
  local hostport="$1" timeout="${2:-30}" start
  start="$(date +%s)"
  while true; do
    if lsof -n -iTCP:"${hostport#*:}" -sTCP:LISTEN -P 2>/dev/null | grep -q "${hostport%:*}"; then
      return 0
    fi
    if (( $(date +%s) - start >= timeout )); then
      return 1
    fi
    sleep 1
  done
}

wait_http() {
  local url="$1" name="$2" timeout="${3:-30}" start
  start="$(date +%s)"
  while true; do
    if curl -fsS "$url" >/dev/null 2>&1; then
      log "$name is ready at $url"
      return 0
    fi
    if (( $(date +%s) - start >= timeout )); then
      log "ERROR: $name failed to respond at $url within ${timeout}s"
      return 1
    fi
    sleep 1
  done
}

cleanup_on_fail() {
  log "Startup failed, cleaning up"
  "$SCRIPT_DIR/devnet-down.sh" >/dev/null 2>&1 || true
}
trap cleanup_on_fail ERR

setup() {
  rm -rf "$DATA_DIR"
  mkdir -p "$PIDS_DIR"
  if [[ -f "$LOG_FILE" ]]; then mv "$LOG_FILE" "$LOG_FILE.$(date +%s).bak"; fi
  touch "$LOG_FILE"
}

start_txnstream() {
  ensure_port_free "${TXN_ADDR#*:}"
  log "Starting $TXN_BIN on $TXN_ADDR"
  cargo build -p libra2-txnstream-server --release >/dev/null
  ( target/release/$TXN_BIN --addr "$TXN_ADDR" --rest "$REST_URL" >>"$LOG_FILE" 2>&1 & )
  echo $! > "$TXN_PIDFILE"
  if ! wait_port "$TXN_ADDR" 45; then
    log "ERROR: $TXN_BIN failed to bind $TXN_ADDR"
    exit 1
  fi
  log "$TXN_BIN is listening on $TXN_ADDR"
}

start_node() {
  ensure_port_free 8070
  ensure_port_free 8080
  ensure_port_free 8081
  log "Starting validator node"
  cargo build -p libra2 --release >/dev/null
  ( target/release/libra2 node run-localnet \
      --test-dir "$DATA_DIR" \
      --force-restart \
      --with-faucet \
      --no-txn-stream \
      >>"$LOG_FILE" 2>&1 & )
  echo $! > "$NODE_PIDFILE"
  wait_http "$READINESS_URL" "validator" 60 || exit 1
  wait_http "$REST_URL" "REST API" 60 || exit 1
  wait_http "$FAUCET_URL" "faucet" 60 || exit 1
  log "Validator node is ready"
}

main() {
  setup
  if [[ "$USE_EXTERNAL_TXNSTREAM" != "1" ]]; then
    start_txnstream
  else
    log "Skipping txnstream-server; using external server at $TXN_ADDR"
  fi
  start_node
  log "Localnet is up. REST endpoint: $REST_URL"
}

main "$@"
