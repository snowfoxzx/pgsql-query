---
name: postgresql-readonly-cli
description: Use when you need to inspect PostgreSQL schemas, tables, columns, or sample data through a local read-only CLI, especially when you want the skill to install the correct binary for the current platform automatically.
---

# PostgreSQL Read-only CLI

Use this skill to run PostgreSQL schema inspection and read-only queries through the bundled `pgq` CLI.

## When To Use

- You need to list PostgreSQL schemas or tables
- You need to inspect a table before querying it
- You need sample rows from a table
- You want a single CLI that works across macOS, Linux, and Windows
- You want a skill-local installer that downloads the correct release binary automatically

## Workflow

1. Ensure the binary is installed:

```bash
sh skills/postgresql-readonly-cli/scripts/install_pgq.sh
```

The install script downloads the platform-specific release archive and verifies it against the release `SHA256SUMS` file before extracting.

2. Run the installed binary:

```bash
skills/postgresql-readonly-cli/bin/pgq --help
```

3. Prefer commands in this order:

```bash
skills/postgresql-readonly-cli/bin/pgq schemas
skills/postgresql-readonly-cli/bin/pgq tables --schema public
skills/postgresql-readonly-cli/bin/pgq describe users --schema public
skills/postgresql-readonly-cli/bin/pgq sample users --schema public --limit 10
skills/postgresql-readonly-cli/bin/pgq query --sql "select count(*) from public.users"
```

## Connection Patterns

Use either a URL:

```bash
skills/postgresql-readonly-cli/bin/pgq \
  --url 'postgres://user:password@host:5432/dbname' \
  schemas
```

Or split fields:

```bash
skills/postgresql-readonly-cli/bin/pgq \
  --host host \
  --port 5432 \
  --user user \
  --password password \
  --dbname dbname \
  tables --schema public
```

Environment variable fallbacks:

- `PGQ_URL`
- `PGQ_HOST`
- `PGQ_PORT`
- `PGQ_USER`
- `PGQ_PASS`
- `PGQ_DB`

## Rules

- Start with `describe` before writing custom SQL against an unfamiliar table
- Use `sample` before broad `query` calls when you need to understand row shape
- Use `--json` when the result needs to be consumed programmatically
- Keep queries read-only; `query` is intended for `SELECT`, `WITH`, and `EXPLAIN`

## Updating

- Re-run the install script to fetch the latest GitHub Release binary
- Set `PGQ_VERSION=vX.Y.Z` before running the install script to pin a specific release
- Set `PGQ_REPO=OWNER/REPO` to override the GitHub repository source
