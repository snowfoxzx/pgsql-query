# GitHub Actions Release Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a GitHub Actions workflow that builds `pgsql-query` for macOS, Linux, and Windows on x86_64 and arm64, publishes tagged releases to GitHub Releases, and keeps release packaging consistent with the local `dist/pgsql-query` contract.

**Architecture:** Use a matrix GitHub Actions workflow triggered by tags and manual dispatch. A dedicated packaging script will normalize per-target output into archive files named from the tag and Rust target triple, then the workflow will upload artifacts and attach them to a GitHub Release.

**Tech Stack:** GitHub Actions YAML, shell scripts, Cargo, `tar`, `zip`

---

## Chunk 1: Release Contract Tests

### Task 1: Lock artifact naming and workflow triggers

**Files:**
- Create: `tests/github_release_contract.rs`
- Modify: `tests/release_script.rs`

- [ ] Step 1: Write failing tests for release archive naming, workflow trigger text, and package contents
- [ ] Step 2: Run targeted tests and confirm they fail for missing files or missing behavior
- [ ] Step 3: Implement the minimum workflow packaging structure needed for those tests
- [ ] Step 4: Re-run targeted tests until they pass

## Chunk 2: Packaging And Workflow

### Task 2: Add target-aware packaging script and workflow

**Files:**
- Create: `.github/workflows/release.yml`
- Modify: `.github/workflows/release.yml`

- [ ] Step 1: Add a target-aware packaging script that assembles `bin/pgsql-query(.exe)`, `README.md`, and the skill under a versioned package root
- [ ] Step 2: Update the local release script to reuse the packaging logic for the host build
- [ ] Step 3: Add a GitHub Actions matrix for macOS, Linux, and Windows, on x86_64 and arm64, with tag and manual triggers
- [ ] Step 4: Add artifact upload and GitHub Release attachment steps

## Chunk 3: Documentation And Verification

### Task 3: Document the release flow

**Files:**
- Modify: `README.md`
- Modify: `AGENTS.md`

- [ ] Step 1: Update repository docs to describe the GitHub Actions release path and archive naming
- [ ] Step 2: Run local verification for tests and script help output
- [ ] Step 3: Report any remaining gaps, especially workflow behavior that cannot be executed locally
