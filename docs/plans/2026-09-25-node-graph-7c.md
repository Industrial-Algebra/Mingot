# Phase 7C — Execution Engine & Built-in Nodes (TDD Plan)

> **Status: IMPLEMENTED** (2026-09-26, commits 6b9f98e…+): value/exec
> hand-written TDD; `nodes/` + demo via Mercury dispatch (deepseek-flash)
> from contracts `2026-09-25-7c-unit-a-nodes.md` / `-unit-b-demo.md`,
> verified in-session. Demo DoD met. This doc remains the design record.

**Date:** 2026-09-25
**Branch:** `feature/0.8.0-node-graph-7c` (off `develop` @ cec15ac)
**Target:** v0.8.0 (supersedes PHASE_7_KICKOFF.md's provisional 0.8.1 — ratified via D1: consumers build apps on running graphs)
**Doctrine (operator, 2026-09-25, from RABBIT_HOLE_2026-09-23_Mingot.md):**
1. **Pure kernel / thin shell** — the engine is pure Rust: zero Leptos/wasm_bindgen
   deps in `exec.rs`/`value.rs`/`nodes/`; natively testable. The Leptos shell
   (demo/UI) consumes it.
2. **Precision lattice first** — runtime semantics are the lattice's dynamic arm.
3. **Refuse-to-fire numerics contract (operator decision, 2026-09-25):** executing
   a `Lossy` (or `Incompatible`) edge is a **runtime error**, surfaced in a
   structured `ExecutionReport` — never a panic, never a silent narrowing.
   The static verdict stays three-valued for *drawing* (validation flags
   PrecisionLoss as a warning); the *engine* refuses to cross. Explicit
   conversion nodes are the only sanctioned precision change.

## Module layout (all `#[cfg(feature = "node-graph")]`, inside `src/node_graph/`)

```
src/node_graph/
├── value.rs        # Value: runtime data + its kind (NEW)
├── exec.rs         # NodeOp trait, topo_order, Engine, ExecutionReport (NEW)
├── nodes/          # built-in pure node library (NEW)
│   ├── mod.rs
│   ├── constant.rs     # terminal source
│   ├── arithmetic.rs   # checked/precision-preserving add/sub/mul/div
│   └── convert.rs      # explicit conversion ("cast") nodes
```

No changes to 7A model files (graph/precision/connection/validate/serialize)
except additive re-exports in `node_graph/mod.rs`.

## 1. `value.rs` — runtime data

```rust
pub enum Value {
    Integer(i128),          // carries IntKind expectation via port type, not value
    Decimal(rust_decimal::Decimal),
    #[cfg(feature = "high-precision")]
    Arbitrary(rust_decimal::Decimal),
    Text(Cow<'static, str>),
    Bool(bool),
    Custom(CustomValueBox), // open hatch for media-like values (video phase)
}

pub trait CustomValue: Send + Sync {
    fn kind_name(&self) -> &'static str;      // e.g. "frame", "timestamp"
    fn summary(&self) -> String;              // human-readable for inspector
}
pub type CustomValueBox = std::sync::Arc<dyn CustomValue>;
```

- `Value::kind(&self) -> ValueKind` mirrors `PortType` categories for
  engine-side coherence checks (Integer/Decimal/Arbitrary/Text/Bool/Custom).
- serde: `Value` implements `Serialize/Deserialize` for *result export only*
  (graphs serialize structure, not runtime values). `Custom` serializes as
  `{"custom": kind_name, "summary": ...}` — round-trip is lossy by design
  and documented.
- TDD: kind classification; Custom arc sharing; text Cow static/borrowed.

## 2. `exec.rs` — engine kernel

```rust
pub trait NodeOp: Send + Sync {
    fn definition(&self) -> &NodeDefinition;          // port types (static truth)
    fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, ExecError>;
}

pub enum ExecError {
    MissingInput { index: usize },                    // unconnected input port
    PortTypeMismatch { port: u32, expected: PortType, got: ValueKind },
    RefusedLossyEdge { conn: Connection, verdict: ConnectionVerdict },
    IntegerOverflow { op: Cow<'static, str> },
    DivideByZero,
    NodeError(Cow<'static, str>),                     // op-specific, via thiserror
}

pub fn topo_order(graph: &NodeGraph) -> Result<Vec<NodeId>, ExecError>; // Kahn; Cycle error

pub struct Engine { ops: BTreeMap<NodeId, Box<dyn NodeOp>> }

pub enum NodeOutcome {
    Executed { outputs: Vec<Value> },
    Failed { error: ExecError },
    Skipped,                                          // upstream failed/refused
}

pub struct ExecutionReport {
    pub outcomes: BTreeMap<NodeId, NodeOutcome>,
    pub values: BTreeMap<(NodeId, u32), Value>,       // last output per port
    pub errors: Vec<(NodeId, ExecError)>,
}
```

Semantics (the contract, in order):
1. `Engine::build(graph, ops)` — validates: every node id has an op; every op's
   def port types are compatible with... (no — op defs ARE the types; the
   engine cross-checks connection endpoints against *op defs*, mirroring
   `validate()` against the graph's own defs; mismatch = build error).
2. `execute(&self) -> ExecutionReport`:
   a. topo order (refuse cycles via `ExecError` variant).
   b. per node, in order: gather inputs by port index. Unconnected input →
      `Failed(MissingInput)` (no defaults — strictness is the product).
   c. **Edge transfer check:** for each incoming connection, the *op def*
      output type → input type runs `check_connection`. `Ok` passes;
      `Lossy`/`Incompatible` → `Failed(RefusedLossyEdge)` with the verdict's
      reason. **No implicit narrowing ever happens.**
   d. `evaluate(&[Value])`; outputs length/type-checked against def outputs
      (`PortTypeMismatch` on drift — an op lying about its def is a bug we
      surface, not trust).
   e. A `Failed` node marks all transitively downstream nodes `Skipped`
      (no partial cascades, no panics).
3. Result-not-panic everywhere (IA standard): overflow → `IntegerOverflow`,
   div by zero → `DivideByZero`.

TDD list: topo order (linear/diamond/cycle/empty); missing input; refuse on
lossy decimal edge; refuse on incompatible edge; type-check outputs; skip
propagation; full happy path with values recorded; `Ok`-verdict passing
(e.g. Decimal(2)→Decimal(4) widen, Decimal→Arbitrary).

## 3. `nodes/` — built-in library (pure)

- `constant.rs` — `Constant { def with 0 inputs/1 output, value }`; def's
  port type derived from the Value at construction.
- `arithmetic.rs` — for `Decimal(n)`:
  - Add/Sub: output `Decimal(max(a, b))` (exact — decimal align is exact).
  - Mul: output `Decimal(min(a + b, 28))` (rust_decimal mul scale law).
  - Div: output `Arbitrary` under high-precision, else `Decimal(28)` —
    division is where precision is born, not lost; document.
  - Integer: checked_add/sub/mul over i128/u128 kinds → `IntegerOverflow`.
    `DivideByZero` for / and %.
  - Construction ties the *node instance* to fixed input port types (e.g.
    `AddDecimal::new(2, 4)` ⇒ inputs Decimal(2), Decimal(4), output Decimal(4)).
- `convert.rs` — the sanctioned casts (the only place precision changes):
  - `RescaleDecimal { from: n, to: m }` — m < n narrows via `round_dp` (banker's,
    rust_decimal); m > n widens (exact, pad zeros).
  - `IntegerCast { from: IntKind, to: IntKind }` — narrowing via TryFrom range
    check → `ExecError::NodeError("integer out of range")`; widening exact.
  - `IntToDecimal { kind, scale }` / `DecimalToInt { scale, kind }` — the
    cross-category bridge (round_dp then range-check).

TDD per node: construction port types; happy paths; overflow/refusal paths;
the conversion nodes' rounding/edge semantics (0.005 → Decimal(2) half-even →
0.00? — banker's on 0.005→2dp: 0.00 (ties to even last digit); pin in tests).

## 4. Demo integration (thin shell)

- New demo section/page `/node-graph` extension or `/node-graph/exec`: seed a
  runnable graph (Constant → Add → Result), a **Run** button, `ExecutionReport`
  rendered (per-node outcome, error reasons, values), and JSON result export.
- This is the 7C DoD from the kickoff: "a working compute graph in the demo
  (exact decimal arithmetic), with serialized results."

## 5. Out of scope for 0.8.0 (recorded, not built)

- Precision Inspector UI (port-value tooltips) — post-7C polish with the audit.
- Video/media node library — its own phase, for an as-yet-unnamed project in
  the **Studio Orinth** org (separate from IA, not started; will consume
  Mingot externally). `CustomValue` is the seam for its media values.
- Lazy/incremental re-execution, caching, subscription — engine runs whole
  graphs to completion in 0.8.0.

## Verification gates

Per repo standard, on nightly: `fmt --check`, `clippy --all-targets
--all-features -D warnings`, `test --all-features` (expect +~45 engine/node
tests), `doc`, wasm32 lib + demo builds. Engine files must compile without
leptos in the dep graph (kernel purity is checkable by grep: no
`leptos::`/`wasm_bindgen` in value.rs/exec.rs/nodes/).
