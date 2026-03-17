#!/usr/bin/env bash

set -euo pipefail

usage() {
  cat <<'EOF'
Usage: scripts/package-release.sh --version <version> --target <target> --binary <path> [--output-dir <dir>] [--package-root <dir>] [--no-archive] [--help]

Packages a built pgq binary together with the skill and README.

Default archive names:
  pgq-${VERSION}-${TARGET}.tar.gz
  pgq-${VERSION}-${TARGET}.zip

Contents:
  README.md
  bin/pgq or bin/pgq.exe
  skills/postgresql-readonly-cli/SKILL.md

Options:
  --version <version>     Version or tag used in archive naming
  --target <target>       Rust target triple, for example x86_64-apple-darwin
  --binary <path>         Path to the compiled pgq binary
  --output-dir <dir>      Output directory for archives and package folders (default: dist/releases)
  --package-root <dir>    Override the package directory name/path
  --no-archive            Only assemble the package directory, skip tar.gz/zip creation
  -h, --help              Show this help
EOF
}

VERSION=""
TARGET=""
BINARY=""
OUTPUT_DIR="dist/releases"
PACKAGE_ROOT=""
NO_ARCHIVE="0"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --version)
      VERSION="${2:-}"
      shift 2
      ;;
    --target)
      TARGET="${2:-}"
      shift 2
      ;;
    --binary)
      BINARY="${2:-}"
      shift 2
      ;;
    --output-dir)
      OUTPUT_DIR="${2:-}"
      shift 2
      ;;
    --package-root)
      PACKAGE_ROOT="${2:-}"
      shift 2
      ;;
    --no-archive)
      NO_ARCHIVE="1"
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

if [[ -z "$VERSION" || -z "$TARGET" || -z "$BINARY" ]]; then
  printf 'Missing required arguments. --version, --target, and --binary are required.\n\n' >&2
  usage >&2
  exit 2
fi

if [[ ! -f "$BINARY" ]]; then
  printf 'Binary not found: %s\n' "$BINARY" >&2
  exit 1
fi

README_SOURCE="README.md"
SKILL_SOURCE=".codex/skills/postgresql-readonly-cli/SKILL.md"

if [[ ! -f "$README_SOURCE" ]]; then
  printf 'Missing README: %s\n' "$README_SOURCE" >&2
  exit 1
fi

if [[ ! -f "$SKILL_SOURCE" ]]; then
  printf 'Missing skill file: %s\n' "$SKILL_SOURCE" >&2
  exit 1
fi

PACKAGE_NAME="pgq-${VERSION}-${TARGET}"
PACKAGE_DIR="${PACKAGE_ROOT:-${OUTPUT_DIR}/${PACKAGE_NAME}}"
BIN_NAME="pgq"

case "$BINARY" in
  *.exe)
    BIN_NAME="pgq.exe"
    ;;
esac

rm -rf "$PACKAGE_DIR"
mkdir -p "$PACKAGE_DIR/bin" "$PACKAGE_DIR/skills/postgresql-readonly-cli"

cp "$BINARY" "$PACKAGE_DIR/bin/${BIN_NAME}"
cp "$README_SOURCE" "$PACKAGE_DIR/README.md"
cp "$SKILL_SOURCE" "$PACKAGE_DIR/skills/postgresql-readonly-cli/SKILL.md"

if [[ "$BIN_NAME" != "pgq.exe" ]]; then
  chmod +x "$PACKAGE_DIR/bin/${BIN_NAME}"
fi

printf 'Package directory: %s\n' "$PACKAGE_DIR"

if [[ "$NO_ARCHIVE" == "1" ]]; then
  exit 0
fi

mkdir -p "$OUTPUT_DIR"

if [[ "$TARGET" == *windows* ]]; then
  ARCHIVE_PATH="${OUTPUT_DIR}/${PACKAGE_NAME}.zip"
  rm -f "$ARCHIVE_PATH"
  powershell.exe -NoLogo -NoProfile -Command \
    "Compress-Archive -Path '${PACKAGE_DIR//\//\\}\*' -DestinationPath '${ARCHIVE_PATH//\//\\}'"
else
  ARCHIVE_PATH="${OUTPUT_DIR}/${PACKAGE_NAME}.tar.gz"
  rm -f "$ARCHIVE_PATH"
  tar -C "$OUTPUT_DIR" -czf "$ARCHIVE_PATH" "$(basename "$PACKAGE_DIR")"
fi

printf 'Archive: %s\n' "$ARCHIVE_PATH"
