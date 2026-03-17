# AGENTS

## Purpose

This repository contains `pgq`, a Rust command-line tool for read-only PostgreSQL inspection, plus a companion skill that teaches agents how to use the CLI safely.

The primary deliverables in this repository are:

- the `pgq` binary
- the `postgresql-readonly-cli` skill
- packaging and validation scripts for local development and release builds

## Working Boundary

`pgq` is the product. It should remain the single command-line entry point for end-user functionality.

Agents working in this repository should preserve these constraints:

- Do not introduce runtime dependencies on `psql`, Python, or other database client tools
- Do not move product behavior into shell scripts or auxiliary tools
- New user-facing capabilities should be added to `pgq` subcommands, flags, or output modes
- Repository scripts may support development, smoke testing, and packaging, but they are not part of the product surface
- Keep the PostgreSQL workflow read-only unless the repository requirements explicitly change

## Repository Layout

- `src/`: Rust implementation of the CLI
- `tests/`: CLI parsing, query guard, packaging, and documentation checks
- `scripts/`: development and packaging helpers such as `smoke.sh` and `release.sh`
- `.codex/skills/postgresql-readonly-cli/`: skill definition for agent usage
- `docs/`: implementation planning artifacts
- `dist/`: generated release output; do not treat as source

## Core Commands

Use these commands when changing the project:

```bash
cargo test
cargo build
cargo build --release
scripts/smoke.sh --help
scripts/release.sh --help
```

For real database validation, provide a PostgreSQL connection string to `scripts/smoke.sh`.

## Change Rules

- Read `README.md` before making broad changes to behavior or packaging
- Add or update tests when changing CLI behavior, connection handling, packaging, or documentation contracts
- Keep `README.md`, release packaging, and skill instructions aligned
- If release contents change, update `scripts/release.sh` and any tests that assert package layout
- Do not commit `dist/` or build artifacts
- Prefer extending the existing CLI flow over creating new entry-point scripts

## Release Contract

The current release package is expected to contain:

- `bin/pgq`
- `skills/postgresql-readonly-cli/SKILL.md`
- `README.md`

Agents should preserve this contract unless the release design is intentionally changed everywhere that depends on it.

## Agent Workflow

Before making changes:

1. Read this file and `README.md`
2. Inspect the affected code and tests
3. Update or add tests first when behavior changes
4. Verify with fresh command output before claiming completion

When working on PostgreSQL behavior, prefer improving `pgq` itself rather than routing work through external tools.
