#!/usr/bin/env sh

set -eu

ROOT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
SKILL_DIR="$ROOT_DIR/skills/pgsql-query"
SKILL_FILE="$SKILL_DIR/SKILL.md"
INSTALL_SCRIPT="$SKILL_DIR/scripts/install_pgsql_query.sh"
PLUGIN_MANIFEST="$ROOT_DIR/.codex-plugin/plugin.json"

test -d "$SKILL_DIR"
test -f "$SKILL_FILE"
test -f "$INSTALL_SCRIPT"
test -f "$PLUGIN_MANIFEST"

grep -F 'name: pgsql-query' "$SKILL_FILE" >/dev/null
grep -F 'sh "$SKILL_DIR/scripts/install_pgsql_query.sh"' "$SKILL_FILE" >/dev/null
grep -F '"$SKILL_DIR/bin/pgsql-query"' "$SKILL_FILE" >/dev/null

if grep -F 'skills/pgsql-query/' "$SKILL_FILE" >/dev/null; then
  printf 'SKILL.md should not hardcode repository-relative skill paths.\n' >&2
  exit 1
fi

grep -F '"skills": "./skills/"' "$PLUGIN_MANIFEST" >/dev/null

printf 'install-script contract checks passed\n'
