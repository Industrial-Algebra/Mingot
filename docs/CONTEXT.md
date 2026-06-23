# Mingot Project Context

**Last Updated**: June 2026
**Version**: 0.7.0
**Status**: Active Development — Phases 1–6 complete; Phase 7 (Node-Based Network UI) next

> Living document. Update after each release / phase completion.

---

## Project Identity

**Mingot** is the Leptos UI library for applications that demand mathematical precision.

It is **the UI layer of the Industrial Algebra ecosystem**, compiling to WebAssembly for use in scientific, financial, engineering, and mathematical software.

### Unique Value Proposition

Mingot is positioned as the web UI library with first-class support for:

- u64, u128, i64, i128 precision integers
- Arbitrary-precision decimals via **rust_decimal** (128-bit, 28–29 significant digits)
- Zero precision loss in numeric input and display
- Type-safe validation preventing silent precision errors

### Target Audience

1. **Scientific Computing** — researchers, physicists, chemists needing numerical precision
2. **Financial Applications** — trading platforms, cryptocurrency, accounting systems
3. **Engineering & CAD** — manufacturing, aerospace, structural analysis
4. **Mathematical Software** — computer algebra systems, theorem provers, education

### Competitive Differentiation

Standard UI libraries are limited to JavaScript's `Number` (safe integer max: 2^53 − 1) and suffer float errors (`0.1 + 0.2 === 0.30000000000000004`). HTML5 `<input type="number">` inherits these limits. Mingot uses **text-based input with Rust-side validation**, preserving exact user entry and enabling unlimited precision.

---

## Current State (as of v0.7.0)

### Quantitative Snapshot

| Metric | Value |
|---|---|
| Version | 0.7.0 (released 2026-03-08) |
| Components | ~68 component modules (`src/components/`) |
| Tests | **389 passing** (`cargo test --lib`, 0 failures) |
| Source LOC | ~34,000 lines of Rust (`src/`) |
| Feature flags | `csr` (default), `ssr`, `hydrate`, `high-precision`, `theme-tokens` |
| Build | `cargo check --lib` clean; WASM target builds in CI |

### Phase Completion

All planned phases through 1.0 criteria are done except Phase 7:

| Phase | Title | Status | Version |
|---|---|---|---|
| 1 | Foundation (NumberInput + stdlib precision) | ✅ | 0.2.0 |
| 2 | Arbitrary Precision (rust_decimal) | ✅ | 0.3.0 |
| 3A | Demo Site & UI Components | ✅ | 0.4.0 |
| 3B | Gap Analysis & Precision Display | ✅ | 0.5.0 |
| 4 | Scientific Input Components | ✅ | 0.6.0 |
| 5 | Mathematical Expression & Data Entry | ✅ | 0.6.1 |
| 6 | Theme System & Custom Themes | ✅ | 0.7.0 |
| 7 | Node-Based Network UI | 🚧 Planned | 0.8.0 |
| 8 | VFX Extension (WGSL Shaders) | 🅵 Future | 0.9.0 |
| 9 | Visualization & Analysis | 🅵 Future | 0.10.0 |

---

## Technical Architecture

### Stack

- **Framework**: Leptos 0.8.12 (reactive, compiles to WASM)
- **Language**: Rust (edition 2021)
- **Target**: WebAssembly (browser, CSR-first)
- **Precision engine**: **rust_decimal 1.40** (optional, behind `high-precision` feature)
- **Serialization** (optional): serde + serde_json, behind `theme-tokens` feature

### Project Structure

```
Mingot/
├── Cargo.toml                 # Workspace root + library manifest
├── CHANGELOG.md               # Versioned release notes
├── ROADMAP.md                 # 9-phase vision (Phases 1–6 done)
├── README.md                  # Precision-first public docs
├── COMPONENT_GUIDELINES.md    # Component API patterns & pitfalls
├── HIGH_PRECISION_PROPOSAL.md # NumberInput design rationale
├── docs/
│   ├── CONTEXT.md             # This file
│   ├── API_IMPROVEMENTS.md    # Integration feedback (working doc)
│   └── MANTINE_GAP_ANALYSIS.md
├── src/
│   ├── lib.rs                 # Crate root + prelude
│   ├── components/            # ~68 UI components (see inventory below)
│   ├── theme/                 # Theme system (mod, builder, colors, tokens,
│   │   ├── presets/           #   presets, provider, validation, spacing, …)
│   │   └── ...
│   ├── utils/                 # StyleBuilder helper
│   └── validation/            # Form validation framework
├── demo/                      # Trunk-based Leptos demo site (dogfoods Mingot)
│   ├── src/{pages,layout,docs,components}/
│   ├── Trunk.toml, netlify.toml
│   └── dist/                  # Built demo output
└── .github/workflows/         # ci.yml, publish.yml, project.yml
```

### Component Inventory (~68 modules)

**Precision / Numeric** (flagship): `number_input`, `angle_input`, `fraction_input`,
`unit_input`, `complex_number_input`, `uncertainty_input`, `interval_input`,
`coordinate_input`, `point_locator`, `parameter_slider`, `parameter_grid`,
`parameter_tree`.

**Mathematical expression**: `equation_editor`, `formula_input`, `symbol_palette`,
`matrix_input`, `vector_input`, `tensor_input`.

**Forms**: `input`, `textarea`, `select`, `checkbox`, `radio`, `switch`,
`password_input`, `file_input`, `pin_input`.

**Range / selection**: `slider`, `range_slider`, `segmented_control`.

**Layout**: `container`, `stack`, `group`, `grid`, `appshell`, `card`, `paper`,
`divider`, `simple_grid`/`grid`.

**Navigation**: `navbar`, `menu`, `breadcrumbs`, `tabs`, `pagination`.

**Feedback / overlay**: `alert`, `banner`, `modal`, `drawer`, `notification`,
`tooltip`, `popover`, `progress`, `ring_progress`, `loader`, `skeleton`,
`loading_overlay`, `error_page`.

**Typography / display**: `text`, `badge`, `avatar`, `stats`, `accordion`,
`hero`, `header`, `footer`.

**Actions / misc**: `button`, `action_icon`, `burger`, `table`.

---

## Key Architectural Decisions

### 1. rust_decimal instead of Amari (major pivot, Phase 2)

**Decision**: Implement arbitrary precision with `rust_decimal`, **not** Amari.

**Rationale**: The originally-assumed `amari::Number` type does **not** exist.
Amari's type system focuses on specialized mathematical structures —
`DualNumber` (automatic differentiation), `TropicalNumber` (max-plus semiring),
and `Scalar`/`Multivector` (geometric/Clifford algebra) — none of which provide
arbitrary-precision *decimal* arithmetic. `rust_decimal` gives 128-bit fixed-point
decimals (28–29 significant digits), exact arithmetic, `FromStr` parsing, and full
WASM compatibility.

**Feature flag**:
```toml
[features]
high-precision = ["rust_decimal"]
[dependencies]
rust_decimal = { version = "1.40", optional = true, default-features = false }
```

Future Amari integration remains possible for *specialized* math (DualNumber inputs,
geometric-algebra scalars) as additional precision variants — not as a decimal backend.

### 2. Text-based NumberInput with Rust-side validation

**Decision**: Use `<input type="text">` + Rust validation rather than native number input.

**Rationale**: HTML5 number input coerces to JS `Number` and loses precision.
Text input preserves exact user entry; Rust validation produces detailed, type-specific
`ParseError`s. See `HIGH_PRECISION_PROPOSAL.md`.

### 3. Concrete `Callback<T>` props (not generics)

**Decision**: Optional callbacks use `Option<Callback<T>>`, not `Option<F> where F: Fn(...)`.

**Rationale**: Generic optional callbacks fail type inference when `None`. Documented in
`COMPONENT_GUIDELINES.md`.

### 4. `Cow<'static, str>` for string props (Phase 6)

**Decision**: Migrated all string props from `String` to `Cow<'static, str>`.

**Rationale**: Zero-copy efficiency; accepts both `&'static str` and owned `String`.

### 5. Comprehensive theme system via CSS variables (Phase 6)

**Decision**: Inject `--mingot-*` CSS custom properties (colors, surfaces, spacing,
radius, shadows, borders, layout, typography) and validate contrast (WCAG 2.1 AA).

Deliverables: `ThemeBuilder` fluent API, 5 presets (Default, Dark, Industrial,
Scientific, Financial), 8 additional color palettes, `ThemeOverride` for scoped
subtrees, design-token JSON export/import (`theme-tokens` feature).

### 6. Optional features are zero-cost

Every opt-in capability (`high-precision`, `theme-tokens`, `ssr`, `hydrate`) is behind a
feature flag so standard CSR builds stay small. WASM release profile uses
`opt-level = 'z'`, LTO, and single codegen unit.

---

## Build, Test & CI/CD

### Local

```bash
cargo test --lib                          # 389 tests, all passing
cargo check --lib                         # clean
cargo clippy --all-targets --all-features -- -D warnings
cargo build --target wasm32-unknown-unknown --lib
```

### CI (`.github/workflows/ci.yml`)

- **test job**: fmt check, clippy with `-D warnings` across all features,
  `cargo test --lib`, library build.
- **build-wasm job**: builds `wasm32-unknown-unknown` target.

### Release / publish (`.github/workflows/publish.yml`)

Triggered on GitHub release. Verifies `Cargo.toml` version matches the git tag,
runs tests, then `cargo publish` to crates.io.

### Project automation (`.github/workflows/project.yml`)

Auto-routes issues/PRs to the cluster project board.

### Demo site

Trunk-based Leptos app in `demo/`, deployed to Netlify (`netlify.toml`).
Dogfoods Mingot (path dependency). Includes theming playground, preset switcher,
and per-component documentation pages with live code examples.

### Pre-commit hooks (`.githooks/`)

Run `cargo fmt`, `cargo clippy`, `cargo test` locally.

---

## Public API Surface (`src/lib.rs`)

Re-exports `components::*` and the theme API:
`MingotProvider`, `Theme`, `ThemeBuilder`, `ThemeOverride`, `ThemeContext`,
`ColorSchemeMode`, `ActiveColorScheme`, `use_theme`, `use_color_scheme`,
`use_color_scheme_toggle`, `use_set_color_scheme`. Validation API:
`Validator`, `ValidationError`, `ValidationResult`. `leptos` is re-exported
for convenience. A `prelude` module aggregates the common imports.

### Feature flags

| Flag | Effect |
|---|---|
| `csr` (default) | Leptos CSR rendering |
| `ssr` | Leptos SSR rendering |
| `hydrate` | Leptos hydration |
| `high-precision` | Enables `NumberInputPrecision::Arbitrary` via `rust_decimal` |
| `theme-tokens` | Enables serde JSON design-token export/import |

---

## Versioning & Release Strategy

- **Pre-1.0** (`0.x.y`): minor bumps per phase, patches for fixes/docs. Breaking
  changes are allowed while API evolves.
- **1.0.0 criteria** (per ROADMAP): Phases 1–6 ✅, Phase 7 ❌, comprehensive docs,
  real-world usage in 3+ production apps, perf benchmarks met, WCAG 2.1 AA audit,
  API stability commitment.

Current: **0.7.0**. Next milestone: **0.8.0** (Phase 7 — Node-Based Network UI).

---

## Documentation Inventory

| Document | Purpose |
|---|---|
| `README.md` | Public-facing, precision-first overview + examples |
| `ROADMAP.md` | 9-phase plan, success metrics, versioning strategy |
| `CHANGELOG.md` | Versioned release notes (Keep a Changelog format) |
| `COMPONENT_GUIDELINES.md` | Callback patterns, HTML5 attributes, pitfalls |
| `HIGH_PRECISION_PROPOSAL.md` | NumberInput design + rust_decimal decision rationale |
| `docs/API_IMPROVEMENTS.md` | Integration feedback working doc |
| `docs/MANTINE_GAP_ANALYSIS.md` | Mantine parity tracking |
| `src/lib.rs` | Crate-level docs with examples |
| docs.rs | Auto-generated API docs (on publish) |

---

## Industrial Algebra Ecosystem

Mingot is the **UI layer** of the ecosystem:

- **Amari** — mathematical computing engine (geometric/tropical algebra, AD). *Not*
  used by Mingot today; specialized Amari-type inputs are a possible future addition.
- **Mingot** — this library (UI components).
- **Ultramarine-Red** — real-world application used for integration testing / feedback
  (tracked in `API_IMPROVEMENTS.md`).

Other IA projects (e.g. the `possum` exploration) borrow Mingot's styling approach
(CSS custom properties + `StyleBuilder`, no utility-class frameworks like Tailwind)
as a deliberate convention.

---

## Next Steps

1. **Phase 7 — Node-Based Network UI (v0.8.0)**: `NodeCanvas`, `Node`, `NodePort`,
   `NodeConnection` with precision-aware, type-safe ports. See ROADMAP §Phase 7.
2. Keep `CHANGELOG.md` and this file updated on each release.
3. Continued integration feedback via Ultramarine-Red (`API_IMPROVEMENTS.md`).
4. Roadmap "Next Review: June 2026" — re-evaluate Phase 7 scope and 1.0 criteria.

---

## Environment & Tools

- **OS**: Linux (Ubuntu/Debian)
- **Rust**: stable (1.75+)
- **WASM target**: `wasm32-unknown-unknown`
- **Demo build**: Trunk
- **Deploy**: Netlify (demo), crates.io (library), docs.rs (API docs)
- **Quality**: rustfmt, clippy (`-D warnings`), 389 unit tests

---

**Mingot: Precision without compromise.**
