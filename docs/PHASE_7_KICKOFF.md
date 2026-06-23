# Phase 7 Kickoff — Node-Based Network UI

**Status:** Kickoff (draft) — June 2026
**Target version:** 0.8.0 (7A + 7B), 0.8.1 (7C) — *pending scope decision*
**Canonical spec:** [`ROADMAP.md` § Phase 7](../ROADMAP.md)
**Owning branch (proposed):** `feature/0.8.0-node-graph` ← `develop`

> This document operationalizes the Phase 7 roadmap section into shippable
> work. It is a **kickoff**, not a line-by-line plan. Once the design decisions
> below are ratified, each slice (7A/7B/7C) gets its own TDD plan under
> `docs/plans/` (see the `writing-plans` skill).

---

## 1. Goal & Non-Goals

**Goal.** Deliver a precision-aware node graph editor: build, edit, serialize,
and (eventually) execute directed graphs whose ports carry typed precision
requirements — Mingot's differentiator versus ComfyUI / Blender / Rete.js.

**Non-goals for 0.8.x (deferred):**
- Control-flow nodes (loop / iterate / switch / filter-map) — complex, lower
  demand; propose a later slice.
- WebGPU / heavy VFX rendering — that is Phase 8 territory.
- A node marketplace / plugin loader.
- Real-time collaborative editing.

---

## 2. Why This Slicing (7A → 7B → 7C)

The roadmap Phase 7 is roughly 2–3 releases of work as written. We split it the
same way Phase 3 was split into 3A/3B, ordered so each slice ships something
useful and de-risks the next:

| Slice | What | Ships as | Why first |
|---|---|---|---|
| **7A** | Headless graph **model** + precision-flow analysis + serde | `node-graph` feature, pure Rust | The precision-critical core. Fully unit/property-testable with no DOM. Defines the trait surface custom nodes extend. Hard to change later. |
| **7B** | **Visual** editor over the 7A model (SVG) | v0.8.0 | Renders a model that is already correct. Reuses proven drag/pointer patterns from `PointLocator`, `Slider`, `ParameterSlider`. |
| **7C** | **Execution** engine + built-in precision-preserving nodes | v0.8.1 | Optional at first; a correct-but-static editor is already useful. Execution adds runtime `Value`s and topological eval. |

**Key property:** the model (7A) never depends on the view (7B) or the engine
(7C). A user could consume 7A alone as a precision-graph library.

---

## 3. Recommended Module Layout

A self-contained, feature-gated subsystem. Keeps node graph cohesive rather
than scattering ~10 files across `src/components/`.

```
src/node_graph/
├── mod.rs            # pub re-exports + feature docs
├── id.rs             # NodeId, PortId, ConnectionId (newtypes)
├── port.rs           # PortType, Port, PortDirection
├── value.rs          # Value (runtime data — 7C)
├── precision.rs      # Precision model, ConnectionVerdict, loss analysis  ⭐
├── node.rs           # NodeDefinition, NodeInstance
├── connection.rs     # Connection
├── graph.rs          # NodeGraph: add / remove / connect / disconnect
├── validate.rs       # cycle detection + type checks + validate()
├── serialize.rs      # to_json / from_json  (serde, feature-gated)
├── exec.rs           # NodeOp trait + topological execution  (7C)
├── nodes/            # built-in node library  (7C)
│   ├── mod.rs
│   ├── arithmetic.rs # add/sub/mul/div — precision-preserving
│   └── data.rs       # Constant / Input / Output terminals
└── components/       # visual layer  (7B)
    ├── mod.rs
    ├── node_canvas.rs
    ├── node.rs
    ├── node_port.rs
    └── node_connection.rs
```

`src/lib.rs` exposes it only when opted in:
```rust
#[cfg(feature = "node-graph")]
pub mod node_graph;
```

Public visual components are re-exported into the crate `prelude` under the same
feature gate, matching how `theme-tokens` already gates the tokens module.

---

## 4. Architecture Sketch

### 4.1 Precision model (the differentiator — design first)

Ports declare what they accept/emit. Reuse `NumberInputPrecision` for numerics:

```rust
pub enum PortType {
    Integer(IntKind),          // U64 | U128 | I64 | I128
    Decimal(u32),              // fixed decimal places
    #[cfg(feature = "high-precision")]
    Arbitrary,                 // rust_decimal::Decimal
    Text,
    Bool,
    Color,
    Custom(Cow<'static, str>), // extensibility for node authors
}
```

Connection checking returns a verdict, not a bool, so lossy-but-allowed
connections can be *warned about* rather than rejected:

```rust
pub enum ConnectionVerdict {
    Ok,
    Lossy(PrecisionLoss),   // e.g. Arbitrary -> Decimal(2): digits will be dropped
    Incompatible(String),   // e.g. Text -> Integer: hard reject
}
pub fn check_connection(source: &PortType, target: &PortType) -> ConnectionVerdict;
```

`Precision` is modelled as an ordered notion so `check_connection` is a simple
comparison with property tests for transitivity/antisymmetry. **This module is
the hardest to change after release — spike + review it before broad build-out.**

### 4.2 Graph operations (7A)

```rust
let mut g = NodeGraph::new();
g.add_node(def)?;
g.connect(from, to)?;   // Err on Incompatible or cycle; Ok-with-warning exposed via validate()
g.validate()?;           // cycles + type compatibility + precision-loss report
let json = g.to_json()?; // feature-gated serde, round-trip tested
g = NodeGraph::from_json(&json)?;
```

`Result`, never panic (IA standard). Errors via the chosen error type (see
decision D4).

### 4.3 Rendering (7B) — SVG, not Canvas/WebGPU

- **Recommendation: SVG for v0.8.x.** Declarative in `view!`, accessible via the
  DOM (ARIA + keyboard), trivial hit-testing for ports/connections, themes via
  the existing `--mingot-*` CSS variables, crisp at any zoom, animatable with CSS.
- Canvas/WebGPU are faster at very high node counts but cost us accessibility,
  CSS theming, and hit-testing — and WebGPU belongs to Phase 8.
- **Design the model/renderer boundary so the renderer is swappable.** Revisit at
  the 7B perf milestone if we need to support ≥ ~500 nodes smoothly.

### 4.4 Execution (7C)

```rust
pub trait NodeOp {
    fn evaluate(&self, inputs: &[(PortId, Value)]) -> Result<OutputMap, NodeError>;
}
```

Topological order comes from 7A's `validate()` (cycle-free DAG). `Value` carries
its precision; narrowing conversions go through `Result` and surface
`PrecisionLoss`. Built-ins start with arithmetic (precision-preserving
add/sub/mul/div) and data terminals (Constant wired to `NumberInput`).

---

## 5. Reuse What We Already Have

Do **not** rebuild these — wire node graph into existing, tested components:

| Need | Existing component |
|---|---|
| Drag-on-canvas positioning | `PointLocator` (drag-and-drop + grid snap) |
| Pointer drag patterns / wheel zoom | `Slider`, `RangeSlider`, `ParameterSlider` |
| Inline precision value editing in a node | `NumberInput` (all `NumberInputPrecision`) |
| Port hover value preview / Precision Inspector | `Tooltip`, `Popover` |
| Node/port coloring by type | Theme `--mingot-*` CSS vars + `StyleBuilder` |
| Collapsible node body | `Accordion` |

This both shrinks scope and keeps visual consistency with the rest of the library.

---

## 6. Testing Strategy

**TDD is non-negotiable** (IA standard): failing test → minimal code → refactor.

- **7A is the sweet spot** — pure Rust, no DOM. Use unit + property tests:
  - Precision ordering: transitivity, antisymmetry.
  - `NodeGraph` invariants: connect/disconnect symmetry, idempotent removes.
  - Cycle detection on random DAGs (accept) and injected cycles (reject).
  - Serde round-trip equality across random graphs.
- **7B** needs interactive verification. The repo currently has `wasm-bindgen-test`
  as a dev-dep but **zero** `#[wasm_bindgen_test]` tests in `src/`. As part of 7A
  we should **stand up a minimal wasm-bindgen-test harness** so 7B can test
  component behavior in-browser, supplemented by the demo site.
- CI gates unchanged: `fmt`, `clippy --all-features -- -D warnings`, `cargo test --lib`,
  `wasm32-unknown-unknown` build. New feature combo `--features node-graph` must
  pass all of these.

---

## 7. Workflow

- **IA gitflow** (as Mingot practices it): `feature/0.8.0-node-graph` → `develop`
  → release PR → `main`. Dependabot already targets `develop`.
- One feature PR per slice at most; keep PRs reviewable.
- `feat:` / `fix:` / `docs:` / `chore:` commit prefixes.
- Human review only, never auto-merge; all CI green before merge.
- Document as you go — every public item gets a doc comment; non-trivial fns get
  doc tests.

---

## 8. Milestones & Definition of Done

### 7A — Precision Graph Model (pure Rust)
- [ ] `id.rs`, `port.rs`, `node.rs`, `connection.rs`, `graph.rs`
- [ ] `precision.rs` — `ConnectionVerdict` + `check_connection` (reviewed design)
- [ ] `validate.rs` — cycle detection + type compatibility + precision-loss report
- [ ] `serialize.rs` — `to_json`/`from_json` with a `schema_version` field
- [ ] `node-graph` feature in `Cargo.toml`; `pub mod node_graph` gated in `lib.rs`
- [ ] Unit + property tests green; `clippy --features node-graph -- -D warnings` clean
- [ ] **DoD:** a usable, serialization-safe precision-graph model with zero DOM code.

### 7B — Visual Node Editor (v0.8.0)
- [ ] `NodePort`, `Node`, `NodeConnection`, `NodeCanvas` (SVG)
- [ ] Pan / zoom, grid snap, drag-to-connect, multi-select
- [ ] Keyboard navigation (add / move / connect / delete) — a11y gate
- [ ] Theme integration; inline `NumberInput` editing in nodes
- [ ] Demo page: build & persist a non-trivial graph
- [ ] wasm-bindgen-test harness in place for component behavior
- [ ] **DoD:** build, edit, save, and reload a graph in the demo, fully keyboard-operable.

### 7C — Execution & Built-in Nodes (v0.8.1)
- [ ] `value.rs` + `NodeOp` trait + topological executor
- [ ] Arithmetic nodes (precision-preserving) + data terminals (Constant via `NumberInput`)
- [ ] Runtime precision propagation; Precision Inspector on connections
- [ ] Execution + precision-loss tests
- [ ] **DoD:** a working compute graph in the demo (e.g. exact decimal arithmetic),
  with serialized results.

---

## 9. Decisions to Ratify Before Coding

These shape the whole design. Recommendations are given; please confirm or redirect.

| # | Decision | Recommendation |
|---|---|---|
| **D1** | **0.8.0 scope** — visual editor only (7A+7B), defer execution to 0.8.1? | **Yes.** Ship a correct, editable, serializable graph first. Add execution when there's a concrete consumer. *(biggest fork — affects timeline ~2×)* |
| **D2** | **Rendering tech** | **SVG** for v0.8.x; revisit Canvas at the 7B perf milestone. |
| **D3** | **Feature gating** | New additive `node-graph` feature, **off by default**; serialization via `node-graph` + existing `serde`/`serde_json` deps. |
| **D4** | **Error types** | Introduce **`thiserror`** for the new `node_graph` errors (cleaner for many variants); leave `ParseError`'s manual `Display` as-is to avoid churn. |
| **D5** | **Module home** | Self-contained `src/node_graph/` (above) over scattering visuals into `src/components/`. |
| **D6** | **Control-flow nodes** | **Defer** beyond 0.8.x. |

---

## 10. Risks

| Risk | Mitigation |
|---|---|
| SVG perf at scale | Cap v1, virtualize/recycle nodes, revisit renderer at 7B milestone. |
| Precision model is hard to change post-release | Spike + review `precision.rs` before broad build-out; property tests. |
| No in-browser component test harness today | Stand one up in 7A so 7B is testable. |
| Scope creep (every node type, control flow) | Hard 0.8.0 = 7A+7B boundary; D6 defers control flow. |
| Leptos 0.8 drag/pan reactivity | Low — already proven in `PointLocator`/`Slider`; reuse those patterns. |

---

## 11. Immediate Next Actions

1. **Ratify decisions D1–D6** (D1 and D4 carry the most weight).
2. Branch `feature/0.8.0-node-graph` from `develop`.
3. **Half-day spike:** implement `precision.rs` (`ConnectionVerdict` +
   `check_connection`) + cycle detection with tests — validate the riskiest
   design before committing to the full module layout.
4. Add the `node-graph` feature skeleton: `Cargo.toml` entry + gated
   `pub mod node_graph;` in `lib.rs` (compiles empty, tests pass).
5. On ratification, turn 7A into a `docs/plans/<date>-node-graph-model.md` plan
   (`writing-plans` skill) and execute with TDD.

---

**Mingot: Precision without compromise — now with wires.**
