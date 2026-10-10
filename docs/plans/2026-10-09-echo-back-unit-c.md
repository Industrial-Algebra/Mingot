# Contract — Echo-Back Unit C: Structured Scientific Components (2026-10-10)

Read `docs/plans/2026-10-09-echo-back.md` and Unit A's §"The conversion
pattern" (identical shapes). Helper `echo_signal` exists — use it, do not
modify. Units A and B are already on this branch.

## Global rules

Same as Unit B: no git; fmt/clippy(-D warnings)/`--lib --all-features` green;
IA standards; anchor mismatch → rule-7 stop for that file; update demo call
sites that pass `RwSignal` to these props (grep `value=` in demo/src).

## The uniform conversion for all ten files

Every file already has the dual-pattern internals (`internal_X` local +
`on_change` firing on commit paths). The conversion is exactly:

1. Value prop `value: Option<RwSignal<T>>` → `value: Option<ReadSignal<T>>`
   (keep any `#[prop(...)]` attributes and doc comments; add one line:
   `/// External controlled value (read-only); echo-back convention.`).
2. The body unwrap line → `echo_signal(value, <THE SAME DEFAULT EXPRESSION>)`.
3. Commit paths already write the internal signal + fire `on_change` —
   unchanged.
4. Any existing `Effect::new` that watches the PROP signal (angle_input,
   tensor_input, unit_input, fraction_input have one) must be adapted to watch
   the internal/echo signal instead (`internal.get()` where it read
   `value.get()`), preserving its body (which updates derived/per-field
   state). The echo effect itself handles external→internal; do not duplicate
   that write in the adapted effect (guard with equality if it wrote the
   internal signal).

## Per-file anchors (prop line → unwrap line)

| # | File | Prop anchor | Unwrap anchor → `echo_signal(value, …)` |
|---|---|---|---|
| 1 | angle_input.rs | ~326 `value: Option<RwSignal<f64>>,` | ~398 `let angle_value = value.unwrap_or_else(\|\| RwSignal::new(0.0));` → `echo_signal(value, 0.0)` |
| 2 | interval_input.rs | ~332 `value: Option<RwSignal<Interval>>,` | ~382 `value.unwrap_or_else(\|\| RwSignal::new(Interval::new(Some(0.0), Some(1.0), bounds)))` → `echo_signal(value, Interval::new(Some(0.0), Some(1.0), bounds))` |
| 3 | vector_input.rs | ~320 `value: Option<RwSignal<Vector>>,` | ~369 `let internal_vector = value.unwrap_or_else(\|\| RwSignal::new(Vector::zeros(dimensions)));` |
| 4 | coordinate_input.rs | ~278 `value: Option<RwSignal<Coordinates>>,` | ~329 `.unwrap_or_else(\|\| RwSignal::new(Coordinates::new(vec![0.0; system.dimensions()], system)))` (note: the line begins with `let internal_coordinates = value` or similar — preserve the binding) |
| 5 | tensor_input.rs | ~268 `value: Option<RwSignal<Tensor>>,` | ~310 `let internal_tensor = value.unwrap_or_else(\|\| RwSignal::new(Tensor::zeros(initial_shape)));` |
| 6 | matrix_input.rs | ~375 `value: Option<RwSignal<Matrix>>,` | ~428 `let internal_matrix = value.unwrap_or_else(\|\| RwSignal::new(Matrix::zeros(rows, cols)));` |
| 7 | point_locator.rs | ~130 `value: Option<RwSignal<Point2D>>,` | ~191 `let internal_point = value.unwrap_or_else(\|\| RwSignal::new(Point2D::new(0.0, 0.0)));` |
| 8 | unit_input.rs | ~379 `value: Option<RwSignal<UnitValue>>,` | ~454 `value.unwrap_or_else(\|\| RwSignal::new(UnitValue::new(0.0, default_unit.clone())))` (preserve binding name) |
| 9 | fraction_input.rs | ~327 `value: Option<RwSignal<Fraction>>,` | ~402 `let fraction_value = value.unwrap_or_else(\|\| RwSignal::new(Fraction::default()));` |
| 10 | equation_editor.rs | ~635 `value: Option<RwSignal<EquationNode>>,` | ~667 `let equation = value.unwrap_or_else(\|\| RwSignal::new(EquationNode::Placeholder));` |

All line numbers are approximate (fmt drift) — the quoted code is the anchor.

## Demo call sites

`rg -n "AngleInput|IntervalInput|VectorInput|CoordinateInput|TensorInput|MatrixInput|PointLocator|UnitInput|FractionInput|EquationEditor" demo/src`
→ for each controlled usage passing `value=`, convert to
`value=read_signal` (the on_change callbacks are already correct).
Uncontrolled usages unchanged. `cargo +nightly check -p <demo package name>`
must pass (check demo/Cargo.toml for the name).

## Wasm tests — APPEND to `tests/echo_back_wasm.rs`

One representative test for the family:
`matrix_input_external_syncs_and_on_change_fires`: mount
`<MatrixInput value=ext on_change=Callback rows=2 cols=2 …>` (match
real prop names from matrix_input.rs). Assert initial cell inputs show the
external matrix values; then simulate typing into a cell (set the input's
value + dispatch "input"/"change" per the harness's synthetic-event notes) →
on_change fired with the updated Matrix; `ext.get_untracked()` unchanged.
Adapt view-detail assumptions to the real file; quoted anchors are binding.

Verify: `cargo +nightly check --target wasm32-unknown-unknown --tests`.

## CHANGELOG

Extend the echo-back subsection under `## [Unreleased]` → `### Changed`:

```markdown
- **Breaking**: Structured scientific inputs (`AngleInput`, `IntervalInput`,
  `VectorInput`, `CoordinateInput`, `TensorInput`, `MatrixInput`,
  `PointLocator`, `UnitInput`, `FractionInput`, `EquationEditor`) take
  optional `ReadSignal` value props instead of `RwSignal`; external value
  changes now sync inward automatically via the shared `echo_signal` helper.
  Commit callbacks (`on_change` family) unchanged.
```

## Report format

Per file: anchors matched / edits / Effect adaptations. Demo call sites.
Gates: fmt, clippy, `--lib --all-features` count, wasm check, demo check.
Rule-7 stops with exact quotes.
