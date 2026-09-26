# Contract A — 7C built-in node library (`nodes/`)

Unit A of the Phase 7C execution engine. Depends on: `value.rs` + `exec.rs`
(already landed, commit 5add0b8 + 120176b). Followed by: Contract B (demo).

**You implement this file exactly.** When prose and a Contract disagree, the
Contract wins. Run no git commands. Pipe long tool outputs through
grep/tail; tool timeouts are SECONDS (cargo ≤ 600).

## Files

- CREATE `src/node_graph/nodes/mod.rs`
- CREATE `src/node_graph/nodes/constant.rs`
- CREATE `src/node_graph/nodes/arithmetic.rs`
- CREATE `src/node_graph/nodes/convert.rs`
- MODIFY `src/node_graph/mod.rs` — two anchored inserts (below). Change
  nothing else in it.

No other file may be created, modified, or deleted. Build artifacts
(`Cargo.lock`, `target/`) excepted.

## Context you need (existing, landed API — do not re-implement)

`src/node_graph/value.rs`: `Value` enum with `Integer(i128)`,
`Decimal(rust_decimal::Decimal)`, `#[cfg(feature = "high-precision")]
Arbitrary(rust_decimal::Decimal)`, `Text(Cow<'static, str>)`, `Bool(bool)`,
`Custom(CustomValueBox)`; `Value::matches(&self, &PortType) -> bool`
(strict: integer must fit the kind's range, decimal scale must be ≤ n);
`Value::kind() -> ValueKind`.

`src/node_graph/exec.rs`: `NodeOp` trait — `fn definition(&self) ->
&NodeDefinition;` and `fn evaluate(&self, inputs: &[Value]) ->
Result<Vec<Value>, ExecError>;`. `ExecError` variants you will construct:
`NodeError(Cow<'static, str>)`, `IntegerOverflow { op: &'static str }`,
`DivideByZero`. `Engine::build(NodeGraph, BTreeMap<NodeId, Box<dyn NodeOp>>)
-> Result<Engine, ExecError>`, `Engine::execute(&self) -> ExecutionReport`
with `pub outcomes: BTreeMap<NodeId, NodeOutcome>`, `pub values:
BTreeMap<(NodeId, u32), Value>`, `pub errors: Vec<(NodeId, ExecError)>`;
`NodeOutcome::{Executed, Failed(ExecError), Skipped}`.

`src/node_graph/node.rs`: `NodeDefinition::new(title: impl
Into<Cow<'static, str>>, inputs: Vec<PortDef>, outputs: Vec<PortDef>)`,
`PortDef::new(name: impl Into<Cow<'static, str>>, port_type: PortType)`.
`src/node_graph/precision.rs`: `PortType::{Integer(IntKind),
Decimal(u32), Text, Bool}` and (only with the `high-precision` feature)
`PortType::Arbitrary`; `IntKind::{U64, U128, I64, I128}`.
`src/node_graph/graph.rs`: `NodeGraph::new()`, `add_node(NodeId,
NodeDefinition)`, `connect(Connection)`; `NodeId(pub u64)`.
`src/node_graph/connection.rs`: `Connection::new(from: NodeId,
from_output: u32, to: NodeId, to_input: u32)`.

The whole `node_graph` module is behind `#[cfg(feature = "node-graph")]` at
`src/lib.rs`, so files inside it need NO cfg of their own — EXCEPT sites
referencing `PortType::Arbitrary` / `Value::Arbitrary`, which exist only
under `high-precision` and must carry `#[cfg(feature = "high-precision")]`
at EVERY referencing site.

Pinned rust_decimal 1.42 semantics (empirically verified — trust these):
`a + b` result scale = max of operand scales; `a * b` result scale = sum of
operand scales; `a / b` result scale = 28; `round_dp(u32)` uses banker's
rounding (half-to-even) and yields exactly the requested scale;
`rescale(u32)` pads zeros when increasing scale; `checked_div` returns
`None` on division by zero; `Decimal::from_i128_with_scale(v, 0)` treats
`v` as an unscaled mantissa at scale 0 (i.e. exactly `v`) and errors when
`|v|` exceeds the decimal range (~7.9e28); `i128::try_from(Decimal)`
exists.

## Contracts

### `src/node_graph/nodes/mod.rs` — verbatim

```rust
// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! Built-in pure node library: terminal constants, precision-preserving
//! arithmetic, and the sanctioned conversion nodes.
//!
//! Pure kernel — no Leptos, no wasm. Construction ties each node *instance*
//! to fixed port types; the engine's transfer verdicts and output-coherence
//! checks then guarantee no implicit precision change at runtime. The only
//! nodes that ever change precision are the conversion nodes in [`convert`].

pub mod arithmetic;
pub mod constant;
pub mod convert;

pub use arithmetic::{
    AddDecimal, AddInteger, DivDecimal, DivInteger, ModInteger, MulDecimal, MulInteger,
    SubDecimal, SubInteger,
};
pub use constant::Constant;
pub use convert::{DecimalToInt, IntegerCast, IntToDecimal, RescaleDecimal};
```

### `src/node_graph/nodes/constant.rs` — verbatim

```rust
// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! Terminal source node emitting one fixed [`Value`].

use crate::node_graph::exec::{ExecError, NodeOp};
use crate::node_graph::node::{NodeDefinition, PortDef};
use crate::node_graph::precision::{IntKind, PortType};
use crate::node_graph::value::Value;

/// A source node with no inputs and one output port, emitting a fixed
/// value.
///
/// The port type is *derived* from the value by [`Constant::new`]:
/// decimals adopt the value's scale, integers adopt `Integer(I128)`,
/// text/bool adopt their category. Use [`Constant::with_type`] to declare
/// a wider port than the value's own precision (e.g. a `1.5` on a
/// `Decimal(4)` port).
pub struct Constant {
    def: NodeDefinition,
    value: Value,
}

impl Constant {
    /// Derive the output port type from the value.
    ///
    /// Errors on `Custom` values (no derivable builtin port type).
    pub fn new(value: Value) -> Result<Self, ExecError> {
        let port_type = match &value {
            Value::Integer(_) => PortType::Integer(IntKind::I128),
            Value::Decimal(d) => PortType::Decimal(d.scale()),
            #[cfg(feature = "high-precision")]
            Value::Arbitrary(_) => PortType::Arbitrary,
            Value::Text(_) => PortType::Text,
            Value::Bool(_) => PortType::Bool,
            Value::Custom(c) => {
                return Err(ExecError::NodeError(format!(
                    "custom value kind `{}` has no derivable port type; use with_type",
                    c.kind_name()
                )
                .into()))
            }
        };
        Self::with_type(value, port_type)
    }

    /// Declare the output port type explicitly; the value must sit on it
    /// exactly (same category, in-range, scale not exceeding the port's).
    pub fn with_type(value: Value, port_type: PortType) -> Result<Self, ExecError> {
        if !value.matches(&port_type) {
            return Err(ExecError::NodeError(
                "constant value does not match its declared port type".into(),
            ));
        }
        Ok(Constant {
            def: NodeDefinition::new(
                "Constant",
                vec![],
                vec![PortDef::new("value", port_type)],
            ),
            value,
        })
    }
}

impl NodeOp for Constant {
    fn definition(&self) -> &NodeDefinition {
        &self.def
    }

    fn evaluate(&self, _inputs: &[Value]) -> Result<Vec<Value>, ExecError> {
        Ok(vec![self.value.clone()])
    }
}
```


### `src/node_graph/nodes/arithmetic.rs` — verbatim, complete file

```rust
// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! Precision-preserving arithmetic nodes.
//!
//! Decimal rules (pinned to rust_decimal semantics):
//! - Add/Sub output `Decimal(max(a, b))` — decimal align is exact.
//! - Mul output `Decimal(a + b)` — the exact-product scale law; the
//!   constructor refuses scale sums above 28 (rust_decimal's ceiling).
//! - Div outputs `Decimal(28)` (or `Arbitrary` under `high-precision`) —
//!   division is where precision is born, not lost.
//!
//! Integer rules: checked i128 arithmetic with the result held to the
//! declared kind's range (overflow → [`ExecError::IntegerOverflow`]);
//! `/` and `%` by zero → [`ExecError::DivideByZero`]. Cross-kind integer
//! flows go through [`crate::node_graph::nodes::convert::IntegerCast`].

use crate::node_graph::exec::{ExecError, NodeOp};
use crate::node_graph::node::{NodeDefinition, PortDef};
use crate::node_graph::precision::{IntKind, PortType};
use crate::node_graph::value::Value;
use rust_decimal::Decimal;

/// Extract two decimal operands, else fail with `NodeError`.
fn decimals(inputs: &[Value], op: &str) -> Result<[Decimal; 2], ExecError> {
    match inputs {
        [Value::Decimal(a), Value::Decimal(b)] => Ok([*a, *b]),
        _ => Err(ExecError::NodeError(
            format!("{op} expects two decimal inputs").into(),
        )),
    }
}

/// Extract two integer operands, else fail with `NodeError`.
fn integers(inputs: &[Value], op: &str) -> Result<[i128; 2], ExecError> {
    match inputs {
        [Value::Integer(a), Value::Integer(b)] => Ok([*a, *b]),
        _ => Err(ExecError::NodeError(
            format!("{op} expects two integer inputs").into(),
        )),
    }
}

/// Wrap an i128 result as a `Value`, held to the declared kind's range.
fn hold_kind(r: i128, kind: IntKind, op: &'static str) -> Result<Value, ExecError> {
    if Value::Integer(r).matches(&PortType::Integer(kind)) {
        Ok(Value::Integer(r))
    } else {
        Err(ExecError::IntegerOverflow { op })
    }
}

/// Two decimal inputs at fixed scales, one output at `out`.
fn two_decimal_def(title: &str, a: u32, b: u32, out: PortType) -> NodeDefinition {
    NodeDefinition::new(
        title,
        vec![
            PortDef::new("lhs", PortType::Decimal(a)),
            PortDef::new("rhs", PortType::Decimal(b)),
        ],
        vec![PortDef::new("out", out)],
    )
}

/// Division's output type: widest available decimal — `Decimal(28)` on the
/// default stack, `Arbitrary` under the `high-precision` feature.
fn out_of_division() -> PortType {
    #[cfg(feature = "high-precision")]
    {
        PortType::Arbitrary
    }
    #[cfg(not(feature = "high-precision"))]
    {
        PortType::Decimal(28)
    }
}

/// Wrap a division quotient as the value matching the division output
/// port: `Value::Arbitrary` under `high-precision`, `Value::Decimal`
/// otherwise.
fn quotient_value(q: Decimal) -> Value {
    #[cfg(feature = "high-precision")]
    {
        Value::Arbitrary(q)
    }
    #[cfg(not(feature = "high-precision"))]
    {
        Value::Decimal(q)
    }
}

/// Exact decimal addition: `Decimal(a) + Decimal(b)` → `Decimal(max(a, b))`.
pub struct AddDecimal {
    def: NodeDefinition,
}

impl AddDecimal {
    pub fn new(a: u32, b: u32) -> Self {
        Self {
            def: two_decimal_def("Add", a, b, PortType::Decimal(a.max(b))),
        }
    }
}

impl NodeOp for AddDecimal {
    fn definition(&self) -> &NodeDefinition {
        &self.def
    }
    fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, ExecError> {
        let [a, b] = decimals(inputs, "add")?;
        Ok(vec![Value::Decimal(a + b)])
    }
}

/// Exact decimal subtraction: `Decimal(a) − Decimal(b)` → `Decimal(max(a, b))`.
pub struct SubDecimal {
    def: NodeDefinition,
}

impl SubDecimal {
    pub fn new(a: u32, b: u32) -> Self {
        Self {
            def: two_decimal_def("Subtract", a, b, PortType::Decimal(a.max(b))),
        }
    }
}

impl NodeOp for SubDecimal {
    fn definition(&self) -> &NodeDefinition {
        &self.def
    }
    fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, ExecError> {
        let [a, b] = decimals(inputs, "sub")?;
        Ok(vec![Value::Decimal(a - b)])
    }
}

/// Exact decimal multiplication: `Decimal(a) × Decimal(b)` → `Decimal(a + b)`.
///
/// Errors at construction when `a + b > 28` — an exact product cannot be
/// held on a port past rust_decimal's 28-fractional-digit ceiling, and
/// silently rounding it would violate the numerics contract.
pub struct MulDecimal {
    def: NodeDefinition,
}

impl MulDecimal {
    pub fn new(a: u32, b: u32) -> Result<Self, ExecError> {
        if a + b > 28 {
            return Err(ExecError::NodeError(
                format!("mul scale sum {a}+{b} exceeds the exact-product ceiling of 28").into(),
            ));
        }
        Ok(Self {
            def: two_decimal_def("Multiply", a, b, PortType::Decimal(a + b)),
        })
    }
}

impl NodeOp for MulDecimal {
    fn definition(&self) -> &NodeDefinition {
        &self.def
    }
    fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, ExecError> {
        let [a, b] = decimals(inputs, "mul")?;
        a.checked_mul(b)
            .map(|p| vec![Value::Decimal(p)])
            .ok_or(ExecError::NodeError("decimal multiplication overflow".into()))
    }
}

/// Decimal division: `Decimal(a) / Decimal(b)` → `Decimal(28)` (or
/// `Arbitrary` under `high-precision`).
pub struct DivDecimal {
    def: NodeDefinition,
}

impl DivDecimal {
    pub fn new(a: u32, b: u32) -> Self {
        Self {
            def: two_decimal_def("Divide", a, b, out_of_division()),
        }
    }
}

impl NodeOp for DivDecimal {
    fn definition(&self) -> &NodeDefinition {
        &self.def
    }
    fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, ExecError> {
        let [a, b] = decimals(inputs, "div")?;
        match a.checked_div(b) {
            Some(q) => Ok(vec![quotient_value(q)]),
            None => Err(ExecError::DivideByZero),
        }
    }
}

// -- checked integer ops: two inputs and one output, all one IntKind ------

/// Shared definition for a two-input integer op of one kind.
fn integer_def(title: &'static str, kind: IntKind) -> NodeDefinition {
    NodeDefinition::new(
        title,
        vec![
            PortDef::new("lhs", PortType::Integer(kind)),
            PortDef::new("rhs", PortType::Integer(kind)),
        ],
        vec![PortDef::new("out", PortType::Integer(kind))],
    )
}

/// Checked integer addition: inputs and output share one `IntKind`.
pub struct AddInteger {
    def: NodeDefinition,
    kind: IntKind,
}

impl AddInteger {
    pub fn new(kind: IntKind) -> Self {
        Self {
            def: integer_def("Add", kind),
            kind,
        }
    }
}

impl NodeOp for AddInteger {
    fn definition(&self) -> &NodeDefinition {
        &self.def
    }
    fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, ExecError> {
        let [a, b] = integers(inputs, "add")?;
        let r = a
            .checked_add(b)
            .ok_or(ExecError::IntegerOverflow { op: "add" })?;
        hold_kind(r, self.kind, "add").map(|v| vec![v])
    }
}

/// Checked integer subtraction: inputs and output share one `IntKind`.
pub struct SubInteger {
    def: NodeDefinition,
    kind: IntKind,
}

impl SubInteger {
    pub fn new(kind: IntKind) -> Self {
        Self {
            def: integer_def("Subtract", kind),
            kind,
        }
    }
}

impl NodeOp for SubInteger {
    fn definition(&self) -> &NodeDefinition {
        &self.def
    }
    fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, ExecError> {
        let [a, b] = integers(inputs, "sub")?;
        let r = a
            .checked_sub(b)
            .ok_or(ExecError::IntegerOverflow { op: "sub" })?;
        hold_kind(r, self.kind, "sub").map(|v| vec![v])
    }
}

/// Checked integer multiplication: inputs and output share one `IntKind`.
pub struct MulInteger {
    def: NodeDefinition,
    kind: IntKind,
}

impl MulInteger {
    pub fn new(kind: IntKind) -> Self {
        Self {
            def: integer_def("Multiply", kind),
            kind,
        }
    }
}

impl NodeOp for MulInteger {
    fn definition(&self) -> &NodeDefinition {
        &self.def
    }
    fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, ExecError> {
        let [a, b] = integers(inputs, "mul")?;
        let r = a
            .checked_mul(b)
            .ok_or(ExecError::IntegerOverflow { op: "mul" })?;
        hold_kind(r, self.kind, "mul").map(|v| vec![v])
    }
}

/// Checked integer division (`/`): zero divisor → `DivideByZero`;
/// `i128::MIN / -1` and kind-range overflow → `IntegerOverflow`.
pub struct DivInteger {
    def: NodeDefinition,
    kind: IntKind,
}

impl DivInteger {
    pub fn new(kind: IntKind) -> Self {
        Self {
            def: integer_def("Divide", kind),
            kind,
        }
    }
}

impl NodeOp for DivInteger {
    fn definition(&self) -> &NodeDefinition {
        &self.def
    }
    fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, ExecError> {
        let [a, b] = integers(inputs, "div")?;
        if b == 0 {
            return Err(ExecError::DivideByZero);
        }
        let r = a
            .checked_div(b)
            .ok_or(ExecError::IntegerOverflow { op: "div" })?;
        hold_kind(r, self.kind, "div").map(|v| vec![v])
    }
}

/// Checked integer remainder (`%`): zero divisor → `DivideByZero`;
/// kind-range overflow → `IntegerOverflow`.
pub struct ModInteger {
    def: NodeDefinition,
    kind: IntKind,
}

impl ModInteger {
    pub fn new(kind: IntKind) -> Self {
        Self {
            def: integer_def("Modulo", kind),
            kind,
        }
    }
}

impl NodeOp for ModInteger {
    fn definition(&self) -> &NodeDefinition {
        &self.def
    }
    fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, ExecError> {
        let [a, b] = integers(inputs, "mod")?;
        if b == 0 {
            return Err(ExecError::DivideByZero);
        }
        let r = a
            .checked_rem(b)
            .ok_or(ExecError::IntegerOverflow { op: "mod" })?;
        hold_kind(r, self.kind, "mod").map(|v| vec![v])
    }
}
```

### `src/node_graph/nodes/convert.rs` — verbatim

```rust
// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! Sanctioned conversion nodes — the only place precision changes.
//!
//! Every other edge in the engine refuses lossy transfers at runtime;
//! these nodes make precision change *explicit and inspectable*.

use crate::node_graph::exec::{ExecError, NodeOp};
use crate::node_graph::node::{NodeDefinition, PortDef};
use crate::node_graph::precision::{IntKind, PortType};
use crate::node_graph::value::Value;
use rust_decimal::Decimal;

/// `Decimal(n)` → `Decimal(m)`: widening pads zeros (exact); narrowing
/// rounds half-to-even (banker's, rust_decimal `round_dp`).
pub struct RescaleDecimal {
    def: NodeDefinition,
    to: u32,
}

impl RescaleDecimal {
    pub fn new(from: u32, to: u32) -> Self {
        Self {
            def: NodeDefinition::new(
                "Rescale Decimal",
                vec![PortDef::new("in", PortType::Decimal(from))],
                vec![PortDef::new("out", PortType::Decimal(to))],
            ),
            to,
        }
    }
}

impl NodeOp for RescaleDecimal {
    fn definition(&self) -> &NodeDefinition {
        &self.def
    }
    fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, ExecError> {
        let Value::Decimal(d) = &inputs[0] else {
            return Err(ExecError::NodeError("rescale expects a decimal input".into()));
        };
        let mut d = *d;
        if self.to >= d.scale() {
            d.rescale(self.to);
            Ok(vec![Value::Decimal(d)])
        } else {
            Ok(vec![Value::Decimal(d.round_dp(self.to))])
        }
    }
}

/// `Integer(from)` → `Integer(to)`: widening is exact; narrowing is
/// range-checked and errors on out-of-range values (never truncates).
pub struct IntegerCast {
    def: NodeDefinition,
    to: IntKind,
}

impl IntegerCast {
    pub fn new(from: IntKind, to: IntKind) -> Self {
        Self {
            def: NodeDefinition::new(
                "Cast Integer",
                vec![PortDef::new("in", PortType::Integer(from))],
                vec![PortDef::new("out", PortType::Integer(to))],
            ),
            to,
        }
    }
}

impl NodeOp for IntegerCast {
    fn definition(&self) -> &NodeDefinition {
        &self.def
    }
    fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, ExecError> {
        let Value::Integer(v) = &inputs[0] else {
            return Err(ExecError::NodeError("cast expects an integer input".into()));
        };
        if Value::Integer(*v).matches(&PortType::Integer(self.to)) {
            Ok(vec![Value::Integer(*v)])
        } else {
            Err(ExecError::NodeError("integer out of range".into()))
        }
    }
}

/// `Integer(kind)` → `Decimal(scale)`: exact conversion, zero-padded to
/// the declared scale.
pub struct IntToDecimal {
    def: NodeDefinition,
    scale: u32,
}

impl IntToDecimal {
    pub fn new(kind: IntKind, scale: u32) -> Self {
        Self {
            def: NodeDefinition::new(
                "Int → Decimal",
                vec![PortDef::new("in", PortType::Integer(kind))],
                vec![PortDef::new("out", PortType::Decimal(scale))],
            ),
            scale,
        }
    }
}

impl NodeOp for IntToDecimal {
    fn definition(&self) -> &NodeDefinition {
        &self.def
    }
    fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, ExecError> {
        let Value::Integer(v) = &inputs[0] else {
            return Err(ExecError::NodeError("int→decimal expects an integer input".into()));
        };
        let mut d = Decimal::from_i128_with_scale(*v, 0)
            .map_err(|_| ExecError::NodeError("integer not representable as decimal".into()))?;
        d.rescale(self.scale);
        Ok(vec![Value::Decimal(d)])
    }
}

/// `Decimal(scale)` → `Integer(kind)`: rounds half-to-even to zero
/// fractional digits, then range-checks.
pub struct DecimalToInt {
    def: NodeDefinition,
    kind: IntKind,
}

impl DecimalToInt {
    pub fn new(scale: u32, kind: IntKind) -> Self {
        Self {
            def: NodeDefinition::new(
                "Decimal → Int",
                vec![PortDef::new("in", PortType::Decimal(scale))],
                vec![PortDef::new("out", PortType::Integer(kind))],
            ),
            kind,
        }
    }
}

impl NodeOp for DecimalToInt {
    fn definition(&self) -> &NodeDefinition {
        &self.def
    }
    fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, ExecError> {
        let Value::Decimal(d) = &inputs[0] else {
            return Err(ExecError::NodeError("decimal→int expects a decimal input".into()));
        };
        let rounded = d.round_dp(0);
        let v = i128::try_from(rounded)
            .map_err(|_| ExecError::NodeError("decimal out of integer range".into()))?;
        if Value::Integer(v).matches(&PortType::Integer(self.kind)) {
            Ok(vec![Value::Integer(v)])
        } else {
            Err(ExecError::NodeError("integer out of range".into()))
        }
    }
}
```

### `src/node_graph/mod.rs` — anchored inserts (change NOTHING else)

Insert A — after the line `pub mod node;`, before `pub mod precision;`,
add (alphabetical position):

```rust
pub mod nodes;
```

Insert B — after the line `pub use node::{NodeDefinition, PortDef};`, add:

```rust
pub use nodes::{
    AddDecimal, AddInteger, Constant, DecimalToInt, DivDecimal, DivInteger, IntToDecimal,
    IntegerCast, ModInteger, MulDecimal, MulInteger, RescaleDecimal, SubDecimal, SubInteger,
};
```

## Tests

Write every test below, exactly as enumerated. Each test module lives in
the file it tests (`#[cfg(test)] mod tests { ... }` at the bottom of
`constant.rs`, `arithmetic.rs`, `convert.rs`). Engine-integration tests
build a real `NodeGraph`, register ops, and run `Engine::execute()` —
shared helper (put a copy in each test module that needs it):

```rust
fn run(graph: NodeGraph, ops: BTreeMap<NodeId, Box<dyn NodeOp>>) -> ExecutionReport {
    Engine::build(graph, ops).expect("build").execute()
}
```

### `constant.rs` tests (6)

1. `derived_decimal_adopts_value_scale` — `Constant::new(Value::Decimal(Decimal::from_str_exact("1.23")?))`:
   assert `def.inputs.is_empty()`; `def.outputs.len() == 1`; output port
   name == "value"; output port type == `PortType::Decimal(2)`.
2. `derived_integer_adopts_i128` — `Constant::new(Value::Integer(7))` →
   output type `PortType::Integer(IntKind::I128)`.
3. `derived_text_and_bool` — Text "hi" → `PortType::Text`; Bool true →
   `PortType::Bool`.
4. `custom_has_no_derived_type` — `Constant::new` with a `Value::Custom`
   (use a minimal test struct implementing `CustomValue`: `kind_name() ->
   "test"`, `summary() -> "s"`) returns `Err(ExecError::NodeError(_))`.
5. `with_type_validates_and_widens` —
   `Constant::with_type(Decimal("1.5"), PortType::Decimal(4))` is Ok and
   `evaluate(&[])` returns `Ok(vec![Value::Decimal("1.5")])`;
   `Constant::with_type(Decimal("1.500"), PortType::Decimal(2))` is
   `Err(NodeError(_))` (scale 3 exceeds 2).
6. `constant_runs_in_engine` — one-node graph (id `NodeId(0)`, the
   constant's def) with the constant op; `run` → `report.is_success()`;
   `report.outcomes[&NodeId(0)] == NodeOutcome::Executed`;
   `report.values[&(NodeId(0), 0)] == Value::Decimal("1.23")` (scale-2
   constant from test 1).

### `arithmetic.rs` tests (12)

7. `add_ports_tie_to_max_scale` — `AddDecimal::new(2, 4)`: inputs
   `[Decimal(2), Decimal(4)]`, output `Decimal(4)` (assert all three port
   types via `definition()`).
8. `add_aligns_scales_exactly` — engine: constant "1.00" @Decimal(2) +
   constant "2.0000" @Decimal(4) → `AddDecimal::new(2, 4)`; assert
   success; result value `Decimal("3.0000")`; assert result
   `.scale() == 4`.
9. `sub_borrows_negatively` — engine: "1.00" − "2.50" @Decimal(2) via
   `SubDecimal::new(2, 2)` → value `Decimal("-1.50")`.
10. `mul_ports_and_exact_scale` — `MulDecimal::new(2, 4)` ports:
    output `Decimal(6)`; engine "1.50" @2 × "2.0000" @4 → value
    `Decimal("3.000000")` with `.scale() == 6`.
11. `mul_refuses_scale_sum_over_28` — `MulDecimal::new(15, 14)` is
    `Err(ExecError::NodeError(_))`.
12. `div_ports_are_widest_decimal` — `DivDecimal::new(2, 4)` output
    `PortType::Decimal(28)`; inputs `[Decimal(2), Decimal(4)]`.
13. `div_runs_to_scale_28` — engine "1.00" / "3.00" @Decimal(2) →
    success; value scale `== 28`; the value's string form starts with
    "0.3333" (assert `starts_with`).
14. `div_by_zero_is_a_report_error` — engine "1.00" / "0.00" @Decimal(2)
    → `!report.is_success()`; node outcome is
    `Failed(ExecError::DivideByZero)`.
15. `integer_add_overflow_reports` — `AddInteger::new(IntKind::U64)`;
    engine with constants `Integer(2u64.pow(63) as i128)` and
    `Integer(2u64.pow(63) as i128)`: outcome
    `Failed(ExecError::IntegerOverflow { op: "add" })`.
16. `integer_add_happy_path` — `AddInteger::new(IntKind::I64)`, 5 + 7 →
    `report.values[&(sink_id, 0)] == Value::Integer(12)`. (Two constant
    nodes feed the add; read the add node's own output value.)
17. `integer_div_and_mod` — `DivInteger::new(IntKind::I64)`: 7 / 2 →
    `Integer(3)`. `ModInteger::new(IntKind::I64)`: 7 % 2 → `Integer(1)`.
    `DivInteger` with rhs 0 → `Failed(DivideByZero)`; `ModInteger` with
    rhs 0 → `Failed(DivideByZero)`.
18. `hp_div_outputs_arbitrary` — `#[cfg(feature = "high-precision")]`:
    `DivDecimal::new(2, 4)` output `PortType::Arbitrary`; engine "1.00" /
    "3.00" → output value `Value::Arbitrary(_)` (assert via `matches!`)
    and report success (proving the arbitrary output passes the engine's
    coherence check).

Engine-wiring detail for tests 8–10, 13–18: constants are
`Constant::with_type(value, declared_port_type)` nodes; connect
`Connection::new(c0, 0, op_node, 0)` etc. Give node ids in ascending
order (constants 0..n, then the op node), assert the op node's outcome
and its output value at port 0.

### `convert.rs` tests (8)

19. `rescale_widens_with_zero_padding` — `RescaleDecimal::new(2, 4)`:
    ports in `Decimal(2)` out `Decimal(4)`; engine: constant "1.50" @2 →
    rescale → value `Decimal("1.5000")`, `.scale() == 4`.
20. `rescale_narrows_with_bankers_rounding` — engine via
    `RescaleDecimal::new(3, 2)`: input "0.005" → `Decimal("0.00")`;
    input "0.015" → `Decimal("0.02")` (ties to even; two separate runs).
21. `rescale_identity_when_scales_equal` — `RescaleDecimal::new(3, 3)`:
    "1.234" → `Decimal("1.234")`.
22. `rescale_pads_lower_scale_input` — declared in-port `Decimal(4)` but
    the upstream value arrives at scale 1 (constant
    `with_type(Decimal("1.5"), PortType::Decimal(4))`):
    `RescaleDecimal::new(4, 6)` → `Decimal("1.500000")`.
23. `integer_cast_widens_exactly` — `IntegerCast::new(IntKind::I64,
    IntKind::I128)` with 5 → `Integer(5)`.
24. `integer_cast_narrows_with_range_check` — `IntegerCast::new(
    IntKind::I128, IntKind::I64)` with `i64::MAX as i128 + 1` →
    `Failed(ExecError::NodeError(ref m))` where `m.contains("out of
    range")`; `IntegerCast::new(IntKind::I128, IntKind::U64)` with −1 →
    same error kind.
25. `int_to_decimal_zero_pads` — `IntToDecimal::new(IntKind::I64, 2)`
    with 5 → `Decimal("5.00")` (`.scale() == 2`); with
    `i128::MAX` → `Failed(NodeError)` containing "not representable".
26. `decimal_to_int_rounds_bankers_then_range_checks` —
    `DecimalToInt::new(2, IntKind::I64)`: "2.5" → `Integer(2)`; "−2.5"
    → `Integer(-2)`; "3.99" → `Integer(3)`; "99999999999999999999" (20
    nines: fits i128, exceeds i64) → `Failed(NodeError)` containing "out
    of range".

## Constraints

- License headers exactly as shown. No new crates; only
  `rust_decimal`, `serde`, `serde_json`, `thiserror`, `crate::` paths.
- Pure kernel: no `leptos`, `wasm_bindgen`, `js_sys`, or `web_sys`
  references anywhere in `nodes/`.
- No `#[allow(...)]` attributes. No panics in library code paths
  (`unreachable!` permitted only inside `IntegerCast`-style structurally
  impossible matches if you chose that route; prefer the field-storage
  route).
- Feature discipline: every `Arbitrary` reference site carries
  `#[cfg(feature = "high-precision")]`; test 18 is cfg-gated the same way;
  tests 12–13 (DivDecimal ports) must NOT reference `Arbitrary` — under
  no-`high-precision` the output is `Decimal(28)` and that is what those
  tests assert. Mentally compile the no-`high-precision` configuration:
  nothing in `constant.rs`/`convert.rs` or tests 1–17, 19–26 may mention
  `Arbitrary`.
- `Value::Decimal("x")` in test assertions means
  `Value::Decimal(Decimal::from_str_exact("x").unwrap())`.

## Completion

Run, in the repo root, fixing until all green:

```bash
cargo +nightly test --features node-graph 2>&1 | grep -E 'test result|FAILED|error' | tail -12
cargo +nightly test --all-features 2>&1 | grep -E 'test result|FAILED|error' | tail -6
cargo +nightly test --features node-graph,high-precision 2>&1 | grep -E 'test result|FAILED' | tail -3
cargo +nightly clippy --all-targets --features node-graph 2>&1 | grep -E '^(error|warning)' | tail -8
cargo +nightly clippy --all-targets --all-features -- -D warnings 2>&1 | tail -4
cargo +nightly fmt
cargo +nightly fmt -- --check && echo FMT-OK
grep -rn 'leptos\|wasm_bindgen' src/node_graph/nodes/ && echo PURITY-FAIL || echo PURITY-OK
```

Expected: node-graph config 500+ tests pass (26 new), all-features 524+
pass, zero clippy errors, FMT-OK, PURITY-OK.

## Out of scope

- `src/node_graph/exec.rs`, `value.rs`, `graph.rs`, `precision.rs`, and
  every other existing file except the two anchored inserts in
  `node_graph/mod.rs`.
- Deserialize for `Value`. Demo/UI work (Contract B). Any new ops beyond
  the ones named here. Benchmarks. Doc-tests.

## Finish

Report: files touched, tests passed (counts per config), and any ambiguity
you resolved and how (rule 7).
