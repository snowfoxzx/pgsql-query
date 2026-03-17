---
name: postgresql-readonly-cli
description: Use when you need to inspect PostgreSQL schemas, tables, columns, or sample data from this workspace without psql, especially when a safe read-only query workflow is preferred.
---

# PostgreSQL Read-only CLI

Use the local `pgq` Rust CLI in this workspace instead of `psql` or Python.

## When to Use

- You need to list PostgreSQL databases, schemas, or tables
- You need to inspect a table definition before querying it
- You need sample rows from a table
- You need a one-off read-only SQL query and want a safe default workflow

## Workflow

1. Build the tool if `target/debug/pgq` does not exist: `cargo build`
2. Prefer commands in this order:
   - `pgq schemas`
   - `pgq tables --schema <schema>`
   - `pgq describe <table> --schema <schema>`
   - `pgq sample <table> --schema <schema> --limit 20`
   - `pgq query --sql "select ..."`
3. Keep queries read-only. `pgq query` is intended for `SELECT`, `WITH`, and `EXPLAIN`.

## Connection Patterns

Use either a URL:

```bash
target/debug/pgq --url 'postgres://user:password@host:5432/dbname' schemas
```

Or split fields:

```bash
target/debug/pgq --host host --port 5432 --user user --password password --dbname dbname tables --schema public
```

The CLI also accepts PostgreSQL environment variables such as `DATABASE_URL`, `PGHOST`, `PGPORT`, `PGUSER`, `PGPASSWORD`, and `PGDATABASE`.

## Common Commands

```bash
target/debug/pgq ping
target/debug/pgq tables --schema public
target/debug/pgq describe users --schema public
target/debug/pgq sample users --schema public --limit 10
target/debug/pgq query --sql "select count(*) from public.users"
target/debug/pgq --json describe users --schema public
```

## Rules

- Start with `describe` before writing custom SQL against an unfamiliar table
- Use `sample` before broad `query` calls when you need to understand row shape
- Use `--json` when the result needs to be consumed programmatically
- If a query is rejected, rewrite it as a single read-only statement
