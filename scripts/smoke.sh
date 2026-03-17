#!/usr/bin/env bash

set -euo pipefail

usage() {
  cat <<'EOF'
Usage: scripts/smoke.sh [--url <postgres-url>] [--schema <schema>] [--table <table>] [--limit <n>] [--skip-build]

Runs a local smoke test against the pgq CLI.

Required connection input:
  --url <postgres-url>   PostgreSQL URL
  or set PGQ_URL in the environment

Options:
  --schema <schema>      Schema used for table checks (default: public)
  --table <table>        Optional table to run describe/sample against
  --limit <n>            Sample row limit when --table is provided (default: 5)
  --skip-build           Skip cargo build
  -h, --help             Show this help

Smoke steps:
  1. cargo build
  2. pgq ping
  3. pgq schemas
  4. pgq tables --schema <schema>
  5. pgq describe/sample if --table is provided

Examples:
  PGQ_URL='postgres://user:pass@localhost:5432/app' scripts/smoke.sh
  scripts/smoke.sh --url 'postgres://user:pass@localhost:5432/app' --schema public --table users
EOF
}

log() {
  printf '\n[%s] %s\n' "$(date '+%H:%M:%S')" "$1"
}

redact_url() {
  printf '%s' "$1" | sed -E 's#(postgres(ql)?://[^:/@]+):[^@]*@#\1:***@#'
}

URL="${PGQ_URL:-}"
SCHEMA="public"
TABLE=""
LIMIT="5"
SKIP_BUILD="0"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --url)
      URL="${2:-}"
      shift 2
      ;;
    --schema)
      SCHEMA="${2:-}"
      shift 2
      ;;
    --table)
      TABLE="${2:-}"
      shift 2
      ;;
    --limit)
      LIMIT="${2:-}"
      shift 2
      ;;
    --skip-build)
      SKIP_BUILD="1"
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      printf 'Unknown argument: %s\n\n' "$1" >&2
      usage >&2
      exit 2
      ;;
  esac
done

if [[ -z "$URL" ]]; then
  printf 'Missing PostgreSQL connection string. Use --url or set PGQ_URL.\n\n' >&2
  usage >&2
  exit 2
fi

if ! [[ "$LIMIT" =~ ^[0-9]+$ ]]; then
  printf 'Invalid --limit value: %s\n' "$LIMIT" >&2
  exit 2
fi

if [[ "$SKIP_BUILD" != "1" ]]; then
  log "Building pgq"
  cargo build
fi

PGQ_BIN="target/debug/pgq"

if [[ ! -x "$PGQ_BIN" ]]; then
  printf 'Expected executable not found: %s\n' "$PGQ_BIN" >&2
  exit 1
fi

run_pgq() {
  "$PGQ_BIN" --url "$URL" "$@"
}

log "Connection target: $(redact_url "$URL")"

log "Running pgq ping"
run_pgq ping

log "Running pgq schemas"
run_pgq schemas

log "Running pgq tables --schema ${SCHEMA}"
run_pgq tables --schema "$SCHEMA"

if [[ -n "$TABLE" ]]; then
  log "Running pgq describe ${TABLE} --schema ${SCHEMA}"
  run_pgq describe "$TABLE" --schema "$SCHEMA"

  log "Running pgq sample ${TABLE} --schema ${SCHEMA} --limit ${LIMIT}"
  run_pgq sample "$TABLE" --schema "$SCHEMA" --limit "$LIMIT"
fi

log "Smoke test completed"
