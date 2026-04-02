# pgsql-query

`pgsql-query` is a read-only PostgreSQL CLI written in Rust. It is intended for schema inspection and safe data exploration without requiring `psql`.

## Features

- Connect with `--url` or split PostgreSQL connection fields
- List databases, schemas, and tables
- Describe table columns and primary key information
- Read sample rows from a table
- Run guarded read-only SQL with `SELECT`, `WITH`, and `EXPLAIN`
- Render results as tables by default or as JSON with `--json`

## Build

Debug build:

```bash
cargo build
```

Release build:

```bash
cargo build --release
```

## Basic Usage

With a connection URL:

```bash
pgsql-query --url 'postgres://user:password@host:5432/dbname' schemas
pgsql-query --url 'postgres://user:password@host:5432/dbname' describe users --schema public
pgsql-query --url 'postgres://user:password@host:5432/dbname' query --sql "select count(*) from public.users"
```

With split fields:

```bash
pgsql-query \
  --host 127.0.0.1 \
  --port 5432 \
  --user postgres \
  --password 'secret' \
  --dbname app \
  ping
```

Environment variable equivalents:

```bash
export PGQ_URL='postgres://user:password@host:5432/dbname'
export PGQ_HOST='127.0.0.1'
export PGQ_PORT='5432'
export PGQ_USER='postgres'
export PGQ_PASS='secret'
export PGQ_DB='app'
```

Supported commands:

- `ping`
- `databases`
- `schemas`
- `tables [--schema <schema>]`
- `describe <table> [--schema <schema>]`
- `sample <table> [--schema <schema>] [--limit <n>]`
- `query --sql "<read-only-sql>"`

## Validation

Run tests:

```bash
cargo test
```

## Skill

This repository also includes the `pgsql-query` skill:

- Source: `skills/pgsql-query/SKILL.md`

Install the correct binary for the current platform with:

```bash
sh skills/pgsql-query/scripts/install_pgsql_query.sh
```

The skill installer downloads the correct GitHub Release archive for the local platform, verifies it against `SHA256SUMS`, and extracts the binary to `skills/pgsql-query/bin/`.

## Release Packaging

GitHub Actions publishes per-platform release archives directly from the workflow.

Release archives include:

- `pgsql-query` or `pgsql-query.exe`
- `README.md`

## GitHub Releases

This repository is designed to publish cross-platform release archives through GitHub Actions.

Supported targets:

- `x86_64-apple-darwin`
- `aarch64-apple-darwin`
- `x86_64-unknown-linux-gnu`
- `aarch64-unknown-linux-gnu`
- `x86_64-pc-windows-msvc`
- `aarch64-pc-windows-msvc`

Workflow behavior:

- tag pushes matching `v*` create release builds automatically
- `workflow_dispatch` can be used for manual releases by providing a tag
- workflow artifacts are uploaded for each target
- GitHub Release assets are named as `pgsql-query-<os>-<arch>.tar.gz`
- Windows assets are named as `pgsql-query-<os>-<arch>.zip`
- each release publishes a `SHA256SUMS` file for installer verification

Release rule:

- Before any release, update the crate version and ensure the `Cargo.toml` version matches the tag being published
