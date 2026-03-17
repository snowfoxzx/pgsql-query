# pgq

`pgq` is a read-only PostgreSQL CLI written in Rust. It is intended for schema inspection and safe data exploration without requiring `psql` or Python.

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
target/debug/pgq --url 'postgres://user:password@host:5432/dbname' schemas
target/debug/pgq --url 'postgres://user:password@host:5432/dbname' describe users --schema public
target/debug/pgq --url 'postgres://user:password@host:5432/dbname' query --sql "select count(*) from public.users"
```

With split fields:

```bash
target/debug/pgq \
  --host 127.0.0.1 \
  --port 5432 \
  --user postgres \
  --password 'secret' \
  --dbname app \
  ping
```

Supported commands:

- `ping`
- `databases`
- `schemas`
- `tables [--schema <schema>]`
- `describe <table> [--schema <schema>]`
- `sample <table> [--schema <schema>] [--limit <n>]`
- `query --sql "<read-only-sql>"`

## Local Validation

Run the local smoke script against a database:

```bash
DATABASE_URL='postgres://user:password@localhost:5432/app' scripts/smoke.sh
```

Run tests:

```bash
cargo test
```

## Skill

This repository also includes the `postgresql-readonly-cli` skill:

- Source: `.codex/skills/postgresql-readonly-cli/SKILL.md`

The skill is designed to guide an agent toward a safe query flow:

1. inspect schemas and tables
2. run `describe`
3. run `sample`
4. use guarded read-only `query` only when needed

## Release Packaging

Create a distributable package with:

```bash
scripts/release.sh
```

The package is written to `dist/pgq/` and includes:

- `bin/pgq`
- `skills/postgresql-readonly-cli/SKILL.md`
- `README.md`
