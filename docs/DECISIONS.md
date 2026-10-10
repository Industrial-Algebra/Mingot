# Mingot Decision Records

Durable, one-time decisions with their rationale. Pointer-style by design: each
entry names its authoritative home instead of duplicating state. Living state
(test counts, versions, dependency lists) is deliberately **not** kept here —
derive it from `Cargo.toml`, `CHANGELOG.md`, and CI output.

---

## D1. rust_decimal, not Amari — decimal backend (Phase 2, v0.3.0)

**Decision**: Implement decimal precision with `rust_decimal`, not Amari.

**Rationale**: The originally-assumed `amari::Number` type does not exist.
Amari's type system targets specialized mathematical structures (dual numbers
for AD, tropical semiring, geometric-algebra scalars/multivectors) — none
provide decimal arithmetic. `rust_decimal` provides 128-bit fixed-point
decimals (96-bit mantissa, 28–29 significant digits), exact checked
arithmetic, `FromStr` parsing, and full WASM compatibility.

**Amended (v0.8.0 era)**: Described honestly as *bounded precision*, not
"arbitrary precision" — see D8.

**Detail**: `HIGH_PRECISION_PROPOSAL.md`; backend lives behind the
`high-precision` feature flag.

## D2. Text-based input with Rust-side validation (Phase 1)

**Decision**: `<input type="text">` + Rust validation, never
`<input type="number">`.

**Rationale**: HTML5 number input coerces to JavaScript `Number`
(safe integers ≤ 2^53 − 1) and loses precision before Rust ever sees the
value. Text input preserves exact user entry; Rust validation produces
detailed, type-specific `ParseError`s.

**Detail**: `HIGH_PRECISION_PROPOSAL.md`, `COMPONENT_GUIDELINES.md`.

## D3. Concrete `Callback<T>` props, not generic closures (Phase 1)

**Decision**: Optional callbacks are `Option<Callback<T>>`, not
`Option<F> where F: Fn(...)`.

**Rationale**: Generic optional callbacks fail type inference when passed as
`None`.

**Detail**: `COMPONENT_GUIDELINES.md`.

## D4. `Cow<'static, str>` for string props (Phase 6)

**Decision**: All string props take `Cow<'static, str>`.

**Rationale**: Zero-copy for `&'static str` literals, still accepts owned
`String`.

## D5. Theme system via CSS custom properties (Phase 6)

**Decision**: Themes inject `--mingot-*` CSS custom properties (colors,
surfaces, spacing, radius, shadows, typography) with WCAG 2.1 AA contrast
validation. `ThemeBuilder` fluent API, five presets, scoped `ThemeOverride`,
and optional design-token JSON export (`theme-tokens` feature).

**Rationale**: No utility-class framework (Tailwind et al.); styling is
deliberately CSS-variable + `StyleBuilder` based. This convention has been
borrowed by sibling IA projects.

## D6. Opt-in features are zero-cost

**Decision**: Every capability (`high-precision`, `theme-tokens`, node-graph)
sits behind an additive feature flag; standard CSR builds stay small. WASM
release profile uses `opt-level = 'z'`, LTO, single codegen unit.

## D7. Node-graph architectural doctrine (Phase 7, operator decision 2026-09-25)

**Decision**: Three layered laws, all central:
1. **Pure kernel / thin shell** — `value.rs`, `exec.rs`, `nodes/` carry zero
   Leptos/wasm-bindgen dependencies; the engine is consumable without WASM.
2. **Precision lattice** — typed ports with a static lossiness order
   (Ok / Lossy / Incompatible verdicts for *drawing* connections).
3. **Refuse-to-fire numerics** — executing a `Lossy` or `Incompatible` edge is
   a structured runtime refusal in `ExecutionReport`; explicit conversion
   nodes are the only sanctioned precision change. Never a silent narrowing.

**Detail**: `docs/plans/2026-09-25-node-graph-7c.md` (the doctrine record),
`src/node_graph/exec.rs`.

## D8. Bounded-precision honesty (v0.8.0 audit remediation)

**Decision**: All public docs state rust_decimal's reality — 96-bit mantissa,
28–29 significant digits, bounded precision — never "arbitrary precision" or
"unlimited".

**Rationale**: Mathematical correctness is paramount; downstream consumers
must know exactly what precision they are getting (README remediation, PR #70).

## D9. Per-domain error homes (v0.8.0 audit remediation)

**Decision**: Typed errors live in their owning module
(`ValidationError`/validation, `ExecError`/node-graph, `FractionError`,
`IntervalError`, `FormulaParseError`), documented in `src/lib.rs` as a
deliberate convention — no single `error.rs` (it would need cfg-riddled
feature gating).

---

*Historical note: this file salvaged the durable content of `docs/CONTEXT.md`,
decommissioned 2026-10-09 — state snapshots committed to a repo rot by
construction. Working state lives in the IA memory service; truth lives in
the artifacts above.*
