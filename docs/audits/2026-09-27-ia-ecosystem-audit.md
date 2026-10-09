# IA Ecosystem Audit: Mingot v0.7.0 (develop @ bd105dc)

**Date:** 2026-09-27
**Audited by:** pi agent (ia-ecosystem-audit)
**Scope:** post-Phase-7 (7A+7B+7C merged), pre-0.8.0 release

## Summary

| Category | Status |
|---|---|
| 1. TDD Compliance | ⚠ PARTIAL |
| 2. Phantom Types & Patterns | ⚠ MIXED |
| 3. Error Types | ✗ FAIL |
| 4. Feature Gates | ✓ PASS |
| 5. Documentation | ✓ PASS (gap noted) |
| 6. License Headers | ✓ PASS |
| 7. Workspace Structure | ⚠ MIXED |
| 8. Ecosystem Composability | ✓ PASS |
| 9. CI Readiness | ✓ PASS |

**Overall: 5/9 clean, 4 with findings** — kernel (node_graph model + engine)
is clean; the findings concentrate in the pre-0.8 presentational layer.

## Findings

### 1. TDD Compliance — ⚠ PARTIAL

- ✓ All kernel modules fully tested inline (precision, graph, validate,
  serialize, layout, value, exec, nodes/: 470 → 533 tests over Phase 7)
- ✓ 7B visual components covered by the browser harness
  (`tests/node_graph_wasm.rs`, 6 tests) instead of unit tests — sanctioned
  for DOM-behavior
- ✗ ~55 presentational components + 5 theme presets + 2 theme providers
  have no test modules (pre-existing baseline, not regression)
- ✗ `src/validation/mod.rs`: `ValidationError` constructors
  (`required`, `invalid_email`, `min_value`, …) untested

### 2. Phantom Types & Patterns — ⚠ MIXED

- ✓ `NodeId`, `CanvasPoint` newtypes; engine Result-not-panic discipline
- ✓ `exec.rs`/`serialize.rs` `expect`s are provable post-validation
  invariants with rationale strings (e.g. `"checked above"` after build's
  coverage checks) — accepted with documentation
- ✗ **CRITICAL** `src/components/fraction_input.rs:46,65`:
  `Fraction::new` / `from_mixed` **panic** on zero denominator —
  user-controllable input reaching a library panic
- ⚠ ~25 UI-layer `ev.target().unwrap()` / `dyn_ref().unwrap()` /
  `.expect("Expected HtmlInputElement")` sites (Leptos idiom, event
  attachment guarantees the type) — pre-0.8 baseline; batch remediation
  candidate, not release-blocking
- ⚠ `use_context().expect("must be used within MingotProvider")`
  (provider.rs:97, override_provider.rs:42) — provider-invariant pattern;
  acceptable with message
- Low: `divider.rs:165` (`label.unwrap()` behind `has_label`),
  `complex_number_input.rs:202` (`chars.next().unwrap()` behind `peek`) —
  provable; prefer `if let` / `unwrap_or` polish
- Note: `formula_input.rs:547+` `self.expect(&Token…)` is a parser method
  returning `Result` — not std `expect`; not a finding

### 3. Error Types — ✗ FAIL

- ✗ No `src/error.rs`; four error enums scattered:
  `ValidationError` (validation/mod.rs), `ExecError` (exec.rs),
  plus internal enums in number_input.rs and formula_input.rs
- ✗ Public `Result<_, String>` APIs (anti-pattern):
  - `formula_input.rs:237` `evaluate() -> Result<f64, String>`
  - `interval_input.rs:240,278` `parse_interval` / `parse_bound`
- ✓ `ExecError` itself is exemplary (thiserror, structured variants,
  documented refusals) — the model to consolidate toward

### 4. Feature Gates — ✓ PASS

- Additive only; no feature removes API
- All 6 `#[cfg(not(feature = "high-precision"))]` sites are internal
  dispatch between feature configurations (division output type, quotient
  value wrapping, cfg-gated tests) — verified in all 7 clippy configs
- `//! ## Features` present in lib.rs

### 5. Documentation — ✓ PASS (gap noted)

- rustdoc fully clean: 0 warnings, 0 undocumented items (fixed during 7C)
- Gap: `# Examples` sections on ~1/554 public fns; doc-test audit remains
  the recorded pre-release follow-up

### 6. License Headers — ✓ PASS

All `src/**/*.rs` carry the Apache-2.0 SPDX header.

### 7. Workspace Structure — ⚠ MIXED

- ✗ `src/error.rs` and `src/phantom.rs` absent (phantom.rs arguably N/A
  for a standalone UI leaf; error.rs tied to finding 3)
- ⚠ Files > 500 lines: `number_input.rs` **2928**, `equation_editor.rs`
  1395, `parameter_tree.rs` 1211, `symbol_palette.rs` 1086,
  `formula_input.rs` 1050, `complex_number_input.rs` 955,
  `exec.rs` 923 (~500 of which are inline tests — justified by the
  same-file TDD convention), `uncertainty_input.rs` 922.
  Refactor candidates for post-0.8.0, not blockers.

### 8. Ecosystem Composability — ✓ PASS

- Standalone UI leaf: no ecosystem deps to version-vs-path; zero
  `path =` deps in the workspace manifest (demo's intra-workspace dep is
  sanctioned)
- Echo-back convention: 41 components expose intent-callback
  (`Callback<...>`) props; 6 components show writable-signal usage to
  review in the deep echo-back pass (notification, select, popover,
  equation_editor, angle_input, matrix_input)

### 9. CI Readiness — ✓ PASS

`ci.yml` (fmt, clippy, test, wasm matrix) + `rust-toolchain.toml`
(nightly) present; CI green on develop.

## Supplementary (operator deferred items)

- ✗ **README Amari residue** — 5 mentions of a nonexistent "Amari
  integration feature" (lines 225, 370, 389, 456, 494). Fix before
  0.8.0: Mingot's arbitrary precision is rust_decimal, not Amari.
- ✓ **equation_editor GA-notation** — confirmed as Hestenes geometric
  algebra notation (wedge/geometric products, ⟨M⟩ₖ grade projection,
  e/γ basis conventions, rotor sandwich), render-only by design
  (`to_latex`/`to_unicode`). One real defect found and fixed:
  `GradeProjection::symbol()` (the palette/insert surface) emitted empty
  brackets `⟨⟩ₖ` — invalid Hestenes notation — and a test had pinned the
  broken form while the full renderers were correct. Now `⟨M⟩ₖ` with a
  shared `grade_subscript` helper; grades ≥ 10 render `ₙ`.
- ✗ `validate()` lacks a duplicate-producer issue (noted in PR #69);
  the engine refuses at build — add the static arm for parity.

## Remediation status (2026-09-27, branch feature/0.8.0-audit-remediation)

1. ✓ **Critical — Fraction**: `new`/`from_mixed` return
   `Result<Self, FractionError>` (`ZeroDenominator`). Correction
   (review): the UI parse path was already guarded — both
   `parse_simple_fraction` and `parse_mixed_number` returned `None` on a
   zero denominator before this change, and the demo reverts invalid
   text on blur, so no UI behavior changed. The fix converts the public
   constructors themselves from panics to structured errors, for
   programmatic callers. Breaking change, changelog'd; demo registry
   examples updated.
   **Tracked separately (pre-existing, reproduced in Chrome via the
   demo)**: `Fraction::from_mixed(i64::MAX, 1, 2)` panics on
   multiplication overflow and `from_mixed(i64::MIN, 1, 2)` panics in
   `abs()` — mixed-number arithmetic needs checked conversion; also
   present on the base commit, out of scope for this remediation.
2. ✓ **High — String errors**: `formula_input::evaluate` returns
   `Result<f64, FormulaParseError>` (new variants `UnknownOperator`,
   `UnknownUnaryOperator`, `FunctionArity`, `UndefinedVariable`);
   `parse_interval`/`parse_bound` return `Result<_, IntervalError>`
   (`BracketMismatch`, `InvalidFormat`, `InvalidBound`). Display strings
   preserved. Zero `Result<_, String>` remains in `src/`.
3. ✓ **High — README**: all five Amari-integration sites rewritten to
   rust_decimal reality; ecosystem link corrected to the
   Industrial-Algebra org URL.
4. ✓ **Medium — error homes**: documented as the deliberate per-domain
   convention in lib.rs ("Error handling" section) rather than forcing a
   cfg-riddled `error.rs`; audit category 3 re-judged as conforming to
   the documented convention.
5. ✓ **Medium — validate()**: `IssueKind::DuplicateProducer` added
   (reports every connection beyond the first on a positional input),
   mirroring the engine's build refusal. TDD'd.
6. ⚠ **Medium — echo-back deep pass**: investigated, then re-scoped
   after review (the original five-component list was non-exhaustive).
   `notification` is clean (context-internal state). **Twelve components
   take writable-signal props** (16 prop sites, all `RwSignal<T>`):
   `select.rs:51` (value), `popover.rs:16` (opened),
   `accordion.rs:78` (opened), `tabs.rs:22` (active),
   `switch.rs:30` (checked), `banner.rs:51` (opened),
   `table.rs:96-97,515-522` (sort_column/sort_direction/current_page),
   `textarea.rs:29` (value), `checkbox.rs:19` (checked),
   `radio.rs:20,237` (checked, _value),
   `equation_editor.rs:635` (value), `angle_input.rs:326` (value),
   `matrix_input.rs:375` (value), plus `number_input.rs:983` (optional
   `value: Option<RwSignal<String>>` alongside its callbacks — dual
   pattern, not purely two-way). Converting to read-signal-in /
   intent-callback-out changes state ownership during editing — a
   per-component design pass, scoped as its own unit before 0.8.0, not
   batched here.

## Recommendations (pre-0.8.0 fix pass, by severity)

1. **Critical** — `Fraction::new`/`from_mixed`: return `Result`
   (breaking change is acceptable pre-1.0; changelog it) or add
   `try_*` constructors and deprecate the panicking ones
2. **High** — replace the 3 public `Result<_, String>` APIs with
   thiserror enums (`FormulaError`, reuse/extend `ValidationError`)
3. **High** — README Amari-integration rewrite (5 sites)
4. **Medium** — consolidate a `src/error.rs` home for the public error
   types (ExecError + ValidationError + the new FormulaError), or
   document per-module error homes as the deliberate convention
5. **Medium** — `validate()`: add `IssueKind::DuplicateProducer`
6. **Medium** — deep echo-back pass over the 6 flagged components
7. **Low** — batch-polish UI-layer event-target unwraps; `# Examples`
   doc-test audit; file-size refactors (post-release)
