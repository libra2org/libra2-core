#!/usr/bin/env bash
set -euo pipefail

DATA_DIR="${DATA_DIR:-"$HOME/.libra2/localnet"}"
PIDS_DIR="$DATA_DIR/pids"

exists_pid() {
  # Return 0 if the given pid is alive; 1 otherwise
  local _pid="${1:-}"
  [[ -n "${_pid}" ]] && ps -p "${_pid}" >/dev/null 2>&1
}

usage() {
  cat <<EOF
Usage: $0 [--aggressive]
  --aggressive  In addition to PID-file shutdown, sweep and kill any process listening on:
                8070, 8080, 8081, 9101, and 50051-50060 (use with caution).
EOF
}

AGGRESSIVE=false
while [[ $# -gt 0 ]]; do
  case "$1" in
    --aggressive) AGGRESSIVE=true; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "Unknown arg: $1"; usage; exit 1 ;;
  esac
done

kill_pidfile() {
  local name="${1:-}"
  if [[ -z "$name" ]]; then
    # Nothing to do if the caller forgot the name
    return 0
  fi

  # If no PIDs dir, nothing to stop
  if [[ ! -d "$PIDS_DIR" ]]; then
    return 0
  fi

  local file="$PIDS_DIR/${name}.pid"
  if [[ -f "$file" ]]; then
    local pid
    pid="$(cat "$file" 2>/dev/null || true)"
    if exists_pid "$pid"; then
      echo "[devnet-down] Stopping $name (pid $pid)"
      kill "$pid" 2>/dev/null || true
      sleep 1
      if exists_pid "$pid"; then
        echo "[devnet-down] Force killing $name (pid $pid)"
        kill -9 "$pid" 2>/dev/null || true
      fi
    fi
    rm -f "$file"
  fi
}

echo "[devnet-down] Stopping managed processes via PID files (if any)"
kill_pidfile "node"
kill_pidfile "txnstream"

# Best-effort: remove pids dir if empty
if [[ -d "$PIDS_DIR" ]] && [[ -z "$(ls -A "$PIDS_DIR" 2>/dev/null)" ]]; then
  rmdir "$PIDS_DIR" 2>/dev/null || true
fi

if $AGGRESSIVE; then
  echo "[devnet-down] Aggressive mode: sweeping common devnet ports"
  declare -a PORTS=(8070 8080 8081 9101)
  for p in $(seq 50051 50060); do PORTS+=("$p"); done
  for port in "${PORTS[@]}"; do
    if lsof -n -iTCP:"$port" -sTCP:LISTEN -P >/dev/null 2>&1; then
      echo "[devnet-down] Killing listeners on :$port"
      # shellcheck disable=SC2046
      kill -9 $(lsof -t -n -iTCP:"$port" -sTCP:LISTEN -P) 2>/dev/null || true
    fi
  done
fi

echo "[devnet-down] Removing localnet data at $DATA_DIR"
rm -rf "$DATA_DIR"
echo "[devnet-down] Done."
