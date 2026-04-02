# Skill Installer Release Implementation Plan

> **For agentic workers:** REQUIRED: Use superpowers:subagent-driven-development (if subagents available) or superpowers:executing-plans to implement this plan. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Change `pgsql-query` distribution to publish multi-platform binaries via GitHub Releases and make the bundled skill install the correct binary for the current platform on demand.

**Architecture:** Move the distributable skill to a top-level `skills/` directory with its own installer script and local `bin/` target. Update release packaging so GitHub Actions publishes per-platform archives plus a `SHA256SUMS` manifest, while the skill installer resolves the correct asset from GitHub Releases and verifies the checksum before extracting.

**Tech Stack:** Rust, shell scripts, GitHub Actions, `tar`, `zip`, `curl`/`wget`, checksum tools

---

## Chunk 1: Distribution Contract Tests

### Task 1: Lock the new skill and release layout

**Files:**
- Create: `tests/skill_distribution_contract.rs`
- Modify: `tests/github_release_contract.rs`

- [ ] Step 1: Write failing tests for the new `skills/pgsql-query/` layout and installer entrypoint
- [ ] Step 2: Write failing tests for release archives and `SHA256SUMS`
- [ ] Step 3: Run targeted tests and confirm they fail for missing files or missing behavior
- [ ] Step 4: Implement the minimum structure to make those tests pass

## Chunk 2: Packaging And Installer

### Task 2: Implement skill-local installation flow

**Files:**
- Create: `skills/pgsql-query/SKILL.md`
- Create: `skills/pgsql-query/scripts/install_pgsql_query.sh`
- Modify: `.github/workflows/release.yml`

- [ ] Step 1: Add the installer script with platform detection, release URL resolution, and checksum verification
- [ ] Step 2: Update local packaging to emit archives and a `SHA256SUMS` file
- [ ] Step 3: Update the local release script to keep `dist/pgsql-query` for host packaging while sharing release archive logic
- [ ] Step 4: Remove the old project-local skill discovery location from the source tree

## Chunk 3: Workflow And Docs

### Task 3: Align GitHub Actions and documentation

**Files:**
- Modify: `.github/workflows/release.yml`
- Modify: `README.md`
- Modify: `AGENTS.md`

- [ ] Step 1: Update the workflow to publish installer-compatible archive names and `SHA256SUMS`
- [ ] Step 2: Document the new skill installation flow and release behavior
- [ ] Step 3: Run local verification for tests and packaging help output
