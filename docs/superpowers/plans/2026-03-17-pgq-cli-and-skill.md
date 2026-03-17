# pgq CLI And Skill Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a Rust CLI that connects to PostgreSQL without `psql`, supports read-only schema and data inspection, and package a reusable skill that drives the CLI safely.

**Architecture:** Implement a single-binary Rust application with `clap` for command parsing, `tokio-postgres` for database access, and a small output layer that renders either human-readable tables or JSON. Keep PostgreSQL metadata access in focused query helpers so the later skill can rely on stable commands instead of embedding SQL repeatedly.

**Tech Stack:** Rust, Cargo, `clap`, `tokio`, `tokio-postgres`, `comfy-table`, `serde`, `serde_json`, `anyhow`

---

## Chunk 1: Project Scaffold

### Task 1: Create the CLI crate

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`
- Create: `src/cli.rs`
- Create: `src/config.rs`
- Create: `src/db.rs`
- Create: `src/commands.rs`
- Create: `src/output.rs`

- [ ] Step 1: Initialize a Cargo binary crate in the current workspace
- [ ] Step 2: Add dependencies for CLI parsing, async runtime, PostgreSQL, JSON, and table rendering
- [ ] Step 3: Add a minimal smoke test for CLI argument parsing
- [ ] Step 4: Run the parsing test and confirm the initial failure or missing implementation
- [ ] Step 5: Implement the minimal CLI structure so the parsing test passes

## Chunk 2: Read-only PostgreSQL Commands

### Task 2: Implement connection config and validation

**Files:**
- Modify: `src/cli.rs`
- Modify: `src/config.rs`
- Modify: `src/db.rs`
- Test: `tests/cli_parse.rs`

- [ ] Step 1: Write failing tests that cover URL-based and field-based connection parsing
- [ ] Step 2: Run only those tests to verify they fail for the expected reason
- [ ] Step 3: Implement connection option resolution with CLI-over-env precedence
- [ ] Step 4: Re-run the targeted tests until they pass

### Task 3: Implement read-only commands

**Files:**
- Modify: `src/commands.rs`
- Modify: `src/db.rs`
- Modify: `src/output.rs`
- Test: `tests/query_guard.rs`

- [ ] Step 1: Write failing tests for the read-only SQL guard and command-level JSON/table shaping helpers
- [ ] Step 2: Run the targeted tests and verify they fail correctly
- [ ] Step 3: Implement `ping`, `databases`, `schemas`, `tables`, `describe`, `sample`, and guarded `query`
- [ ] Step 4: Re-run the targeted tests until they pass

## Chunk 3: Packaging And Verification

### Task 4: Create the skill

**Files:**
- Create: `.codex/skills/postgresql-readonly-cli/SKILL.md`

- [ ] Step 1: Write the skill with concise trigger conditions and stable command examples
- [ ] Step 2: Check that the skill stays aligned with the final CLI behavior

### Task 5: Verify the workspace

**Files:**
- Verify: `Cargo.toml`
- Verify: `src/*.rs`
- Verify: `tests/*.rs`
- Verify: `.codex/skills/postgresql-readonly-cli/SKILL.md`

- [ ] Step 1: Run `cargo test`
- [ ] Step 2: Run `cargo build`
- [ ] Step 3: If available, run a smoke command like `cargo run -- --help`
- [ ] Step 4: Report actual verification results and any remaining gaps
