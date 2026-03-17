#!/usr/bin/env bash

set -euo pipefail

usage() {
  cat <<'EOF'
Usage: scripts/release.sh [--skip-tests] [--skip-build] [--output <dir>] [--help]

Builds the pgq release binary and packages distributable artifacts into dist/pgq.

Artifacts:
  dist/pgq/bin/pgq
  dist/pgq/skills/postgresql-readonly-cli/SKILL.md
  dist/pgq/README.md

Options:
  --skip-tests         Skip cargo test before packaging
  --skip-build         Skip cargo build --release before packaging
  --output <dir>       Override output directory (default: dist/pgq)
  -h, --help           Show this help
EOF
}

log() {
  printf '\n[%s] %s\n' "$(date '+%H:%M:%S')" "$1"
}

OUTPUT_DIR="dist/pgq"
SKIP_TESTS="0"
SKIP_BUILD="0"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --skip-tests)
      SKIP_TESTS="1"
      shift
      ;;
    --skip-build)
      SKIP_BUILD="1"
      shift
      ;;
    --output)
      OUTPUT_DIR="${2:-}"
      shift 2
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

if [[ "$SKIP_TESTS" != "1" ]]; then
  log "Running cargo test"
  cargo test
fi

if [[ "$SKIP_BUILD" != "1" ]]; then
  log "Building release binary"
  cargo build --release
fi

BIN_SOURCE="target/release/pgq"
SKILL_SOURCE=".codex/skills/postgresql-readonly-cli/SKILL.md"
README_SOURCE="README.md"

if [[ ! -x "$BIN_SOURCE" ]]; then
  printf 'Missing release binary: %s\n' "$BIN_SOURCE" >&2
  exit 1
fi

if [[ ! -f "$SKILL_SOURCE" ]]; then
  printf 'Missing skill file: %s\n' "$SKILL_SOURCE" >&2
  exit 1
fi

if [[ ! -f "$README_SOURCE" ]]; then
  printf 'Missing README: %s\n' "$README_SOURCE" >&2
  exit 1
fi

log "Preparing ${OUTPUT_DIR}"
rm -rf "$OUTPUT_DIR"
mkdir -p "$OUTPUT_DIR/bin" "$OUTPUT_DIR/skills/postgresql-readonly-cli"

cp "$BIN_SOURCE" "$OUTPUT_DIR/bin/pgq"
cp "$SKILL_SOURCE" "$OUTPUT_DIR/skills/postgresql-readonly-cli/SKILL.md"
cp "$README_SOURCE" "$OUTPUT_DIR/README.md"

chmod +x "$OUTPUT_DIR/bin/pgq"

log "Release package created"
printf 'Binary: %s\n' "$OUTPUT_DIR/bin/pgq"
printf 'Skill:  %s\n' "$OUTPUT_DIR/skills/postgresql-readonly-cli/SKILL.md"
printf 'Readme: %s\n' "$OUTPUT_DIR/README.md"
