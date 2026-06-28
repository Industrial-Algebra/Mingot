> **SUPERSEDED (June 2026):** The AGPL-3.0 relicensing described below was
> reverted before any AGPL version was published. IA has since standardized on
> Apache-2.0 + CLA for all projects; see the `chore/relicense-apache` branch
> and `CONTRIBUTING.md`. This document is retained for history only.

# IA Spec Compliance — Spec Essentials Pass

> **Scope decision (user, 2026-06-20):** Option 2 — licensing + headers + toolchain + thiserror + CI hardening + feature docs now. **Defer** central `error.rs` consolidation (do for Phase 7 new code) and doc-test audit.
>
> **Toolchain:** nightly per `ia-coding-standards` skill (user did not veto). Trivially revertible to stable.

**Goal:** Bring Mingot to IA coding-standards compliance as a prerequisite to Phase 7.

**Base branch:** `main` @ 823b98c (verified: 389 tests pass, `--all-features` compiles clean).
**Working branch:** `chore/ia-spec-essentials`

> **Note on gitflow:** `develop` has diverged from `main` and is *missing the v0.7.0 release*. Branching from `develop` would lose the theme system. We branch from the verified `main` instead. **Action item for maintainer:** reconcile `develop` ↔ `main` separately.

**Legal:** Sole author = Justin Elliott Cobb / Industrial Algebra (no external contributors, no CLA). AGPL relicensing is clean. Prior published versions (crates.io v0.1–0.7, git tags) remain MIT/Apache permanently; v0.8.0+ source is AGPL-3.0-only.

---

## Tasks

### Task 0 — Planning docs (carry-over)
Commit the Phase 7 planning artifacts produced this session: `ROADMAP.md` (1.0 criteria), `docs/CONTEXT.md` (refreshed), `docs/PHASE_7_KICKOFF.md`, and this plan.
- **Verify:** `git status` clean after commit.

### Task 1 — AGPL relicensing (atomic)
- `Cargo.toml`: `license = "MIT OR Apache-2.0"` → `"AGPL-3.0-only"`
- Add `LICENSE-AGPL` (canonical GNU AGPL-3.0 text, 661 lines)
- Keep `LICENSE-MIT` / `LICENSE-APACHE` (reference for ≤0.7.0 releases)
- Rewrite README "License" section: AGPL-3.0-only for 0.8.0+, note prior versions dual MIT/Apache
- **Verify:** `cargo check --lib` still passes (license field is metadata-only). crates.io license badge auto-updates from Cargo.toml — no manual badge change.

### Task 2 — SPDX headers on all 112 `.rs` files
Prepend to every `src/**/*.rs` and `demo/src/**/*.rs` (idempotent):
```rust
// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: AGPL-3.0-only
```
- **Verify:** `cargo fmt -- --check` clean, `cargo check --lib` clean, header count = 112, no file lost content (line counts increased by 2).

### Task 3 — Nightly toolchain
- Add `rust-toolchain.toml`: `channel = "nightly"`, components `rustfmt`, `clippy`
- `ci.yml`: `dtolnay/rust-toolchain@stable` → `@nightly` (lines 21, 69)
- **Verify:** `cargo +nightly check --lib` clean. ⚠️ Highest-risk change (nightly drift, wasm32+nightly). Revert to `@stable` + stable toolchain if it breaks.

### Task 4 — thiserror migration (3 error types)
- Add `thiserror` dep
- `number_input.rs::ParseError`, `formula_input.rs::FormulaParseError`, `validation/mod.rs::ValidationError` → `#[derive(Error, Debug, ...)]` with `#[error("...")]`, preserving **exact** message text
- Leave the 7 domain-type Display impls (Fraction, ComplexNumber, etc.) untouched — not errors
- **Verify:** `cargo test --lib` = 389 pass (messages unchanged → no test regressions), `cargo clippy --all-features -- -D warnings` clean.

### Task 5 — Feature docs in `lib.rs`
Add `//! ## Features` section documenting all 5 flags (csr, ssr, hydrate, high-precision, theme-tokens).
- **Verify:** `cargo doc --no-deps --lib` builds.

### Task 6 — CI hardening
- `ci.yml`: `cargo test --lib` → also `cargo test --all-features`; add `cargo doc --no-deps --all-features`
- **Verify:** all gates pass locally: `cargo fmt --check`, `cargo clippy --all-features -- -D warnings`, `cargo test --all-features`, `cargo doc --no-deps --all-features`, `cargo build --target wasm32-unknown-unknown --lib`.

---

## Deferred (explicitly out of scope)
- Central `src/error.rs` consolidation (do for Phase 7 new code only)
- Full doc-test audit on all public items (enforce going forward)
- `phantom.rs` module (N/A until ecosystem phantom types are used)
- `develop` ↔ `main` reconciliation
