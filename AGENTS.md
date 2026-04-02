# AGENTS

## Purpose

This repository contains `pgsql-query`, a Rust command-line tool for read-only PostgreSQL inspection, plus a companion skill that teaches agents how to use the CLI safely.

The primary deliverables in this repository are:

- the `pgsql-query` binary
- the `pgsql-query` skill and its self-installing distribution assets
- release workflow and validation coverage for distribution builds

## Working Boundary

`pgsql-query` is the product. It should remain the single command-line entry point for end-user functionality.

Agents working in this repository should preserve these constraints:

- Do not introduce runtime dependencies on `psql`, Python, or other database client tools
- Do not move product behavior into shell scripts or auxiliary tools
- New user-facing capabilities should be added to `pgsql-query` subcommands, flags, or output modes
- Repository automation may support validation and packaging, but it is not part of the product surface
- Keep the PostgreSQL workflow read-only unless the repository requirements explicitly change
- Use only the `PGQ_*` environment variable names for runtime configuration: `PGQ_URL`, `PGQ_HOST`, `PGQ_PORT`, `PGQ_USER`, `PGQ_PASS`, and `PGQ_DB`

## Repository Layout

- `src/`: Rust implementation of the CLI
- `tests/`: CLI parsing, query guard, packaging, and documentation checks
- `skills/pgsql-query/`: distributable skill, installer script, and local skill binary directory
- `.github/workflows/`: GitHub Actions release automation
- `docs/`: implementation planning artifacts
- `dist/`: generated release output; do not treat as source

## Core Commands

Use these commands when changing the project:

```bash
cargo test
cargo build
cargo build --release
```

## Change Rules

- Read `README.md` before making broad changes to behavior or packaging
- Add or update tests when changing CLI behavior, connection handling, packaging, or documentation contracts
- Keep `README.md`, release packaging, and skill instructions aligned
- If release contents change, update `.github/workflows/release.yml` and any tests that assert package layout
- If GitHub release targets or archive names change, update `.github/workflows/release.yml`, `README.md`, and the release contract tests together
- Preserve the installer contract: the skill installer resolves GitHub Release assets by platform name and verifies them with `SHA256SUMS`
- Do not commit `dist/` or build artifacts
- Prefer extending the existing CLI flow over creating new entry-point scripts

## Release Contract

The current release package is expected to contain:

- `pgsql-query` or `pgsql-query.exe`
- `README.md`

Agents should preserve this contract unless the release design is intentionally changed everywhere that depends on it.

GitHub Actions release archives currently follow these names:

- `pgsql-query-<os>-<arch>.tar.gz`
- `pgsql-query-<os>-<arch>.zip` for Windows targets
- `SHA256SUMS`

## Agent Workflow

Before making changes:

1. Read this file and `README.md`
2. Inspect the affected code and tests
3. Update or add tests first when behavior changes
4. Verify with fresh command output before claiming completion

When working on PostgreSQL behavior, prefer improving `pgsql-query` itself rather than routing work through external tools.
