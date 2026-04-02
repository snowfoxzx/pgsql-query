#!/usr/bin/env sh

set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
SKILL_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
BIN_DIR="$SKILL_DIR/bin"
REPO="${PGQ_REPO:-snowfoxzx/pgsql-query}"
VERSION="${PGQ_VERSION:-latest}"
SHA256SUMS_FILE_OVERRIDE="${PGQ_SHA256SUMS_FILE:-}"

detect_os() {
  if [ -n "${PGQ_OS:-}" ]; then
    printf '%s\n' "$PGQ_OS"
    return 0
  fi

  uname -s
}

detect_arch() {
  if [ -n "${PGQ_ARCH:-}" ]; then
    printf '%s\n' "$PGQ_ARCH"
    return 0
  fi

  uname -m
}

normalized_os() {
  case "$(detect_os)" in
    Linux)
      printf 'linux\n'
      ;;
    Darwin)
      printf 'macos\n'
      ;;
    MINGW*|MSYS*|CYGWIN*|Windows_NT)
      printf 'windows\n'
      ;;
    *)
      printf 'unsupported operating system: %s\n' "$(detect_os)" >&2
      exit 1
      ;;
  esac
}

normalized_arch() {
  case "$(detect_arch)" in
    x86_64|amd64)
      printf 'x86_64\n'
      ;;
    arm64|aarch64)
      printf 'aarch64\n'
      ;;
    *)
      printf 'unsupported architecture: %s\n' "$(detect_arch)" >&2
      exit 1
      ;;
  esac
}

archive_extension() {
  case "$(normalized_os)" in
    windows)
      printf 'zip\n'
      ;;
    *)
      printf 'tar.gz\n'
      ;;
  esac
}

asset_name() {
  printf 'pgsql-query-%s-%s.%s\n' "$(normalized_os)" "$(normalized_arch)" "$(archive_extension)"
}

download_url() {
  asset=$(asset_name)
  if [ "$VERSION" = "latest" ]; then
    printf 'https://github.com/%s/releases/latest/download/%s\n' "$REPO" "$asset"
  else
    printf 'https://github.com/%s/releases/download/%s/%s\n' "$REPO" "$VERSION" "$asset"
  fi
}

checksum_url() {
  if [ "$VERSION" = "latest" ]; then
    printf 'https://github.com/%s/releases/latest/download/SHA256SUMS\n' "$REPO"
  else
    printf 'https://github.com/%s/releases/download/%s/SHA256SUMS\n' "$REPO" "$VERSION"
  fi
}

download_file() {
  url=$1
  output_path=$2

  if command -v curl >/dev/null 2>&1; then
    curl -fsSL "$url" -o "$output_path"
    return 0
  fi

  if command -v wget >/dev/null 2>&1; then
    wget -qO "$output_path" "$url"
    return 0
  fi

  printf 'curl or wget is required to download %s\n' "$url" >&2
  exit 1
}

download_archive() {
  download_file "$(download_url)" "$1"
}

download_checksums() {
  download_file "$(checksum_url)" "$1"
}

checksum_file_path() {
  if [ -n "$SHA256SUMS_FILE_OVERRIDE" ]; then
    printf '%s\n' "$SHA256SUMS_FILE_OVERRIDE"
  else
    printf '%s\n' "$1"
  fi
}

expected_checksum() {
  sums_path=$(checksum_file_path "$1")
  asset=$(asset_name)
  checksum=$(awk -v asset="$asset" '$2 == asset { print $1; exit }' "$sums_path")

  if [ -z "$checksum" ]; then
    printf 'checksum for %s not found in %s\n' "$asset" "$sums_path" >&2
    exit 1
  fi

  printf '%s\n' "$checksum"
}

actual_checksum() {
  archive_path=$1

  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$archive_path" | awk '{print $1}'
    return 0
  fi

  if command -v shasum >/dev/null 2>&1; then
    shasum -a 256 "$archive_path" | awk '{print $1}'
    return 0
  fi

  if command -v openssl >/dev/null 2>&1; then
    openssl dgst -sha256 "$archive_path" | awk '{print $NF}'
    return 0
  fi

  printf 'sha256sum, shasum, or openssl is required to verify downloads\n' >&2
  exit 1
}

verify_archive_checksum() {
  archive_path=$1
  sums_path=$2
  expected=$(expected_checksum "$sums_path")
  actual=$(actual_checksum "$archive_path")

  if [ "$expected" != "$actual" ]; then
    printf 'checksum mismatch for %s\nexpected: %s\nactual:   %s\n' "$(asset_name)" "$expected" "$actual" >&2
    exit 1
  fi
}

extract_archive() {
  archive_path=$1
  temp_dir=$2

  case "$(archive_extension)" in
    tar.gz)
      tar -xzf "$archive_path" -C "$temp_dir"
      ;;
    zip)
      if command -v unzip >/dev/null 2>&1; then
        unzip -oq "$archive_path" -d "$temp_dir"
      elif command -v powershell >/dev/null 2>&1; then
        powershell -NoProfile -Command "Expand-Archive -LiteralPath '$archive_path' -DestinationPath '$temp_dir' -Force"
      else
        printf 'unzip or powershell is required to extract %s\n' "$archive_path" >&2
        exit 1
      fi
      ;;
  esac
}

binary_name() {
  case "$(normalized_os)" in
    windows)
      printf 'pgsql-query.exe\n'
      ;;
    *)
      printf 'pgsql-query\n'
      ;;
  esac
}

install_binary() {
  temp_dir=$(mktemp -d)
  trap 'rm -rf "$temp_dir"' EXIT INT TERM

  archive_path="$temp_dir/$(asset_name)"
  sums_path="$temp_dir/SHA256SUMS"
  download_archive "$archive_path"
  download_checksums "$sums_path"
  verify_archive_checksum "$archive_path" "$sums_path"
  extract_archive "$archive_path" "$temp_dir"

  mkdir -p "$BIN_DIR"
  binary=$(binary_name)
  extracted_binary=$(find "$temp_dir" -name "$binary" -type f | head -n 1)

  if [ -z "$extracted_binary" ]; then
    printf 'extracted binary not found for %s\n' "$binary" >&2
    exit 1
  fi

  cp "$extracted_binary" "$BIN_DIR/$binary"

  case "$(normalized_os)" in
    windows)
      ;;
    *)
      chmod +x "$BIN_DIR/$binary"
      ;;
  esac

  printf 'installed %s\n' "$BIN_DIR/$binary"
}

case "${1:-install}" in
  install)
    install_binary
    ;;
  print-asset-name)
    asset_name
    ;;
  print-download-url)
    download_url
    ;;
  print-checksum-url)
    checksum_url
    ;;
  print-expected-checksum)
    expected_checksum "${SHA256SUMS_FILE_OVERRIDE:-SHA256SUMS}"
    ;;
  *)
    printf 'usage: %s [install|print-asset-name|print-download-url|print-checksum-url|print-expected-checksum]\n' "$0" >&2
    exit 1
    ;;
esac
