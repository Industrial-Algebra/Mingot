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
fn two_decimal_def(title: &'static str, a: u32, b: u32, out: PortType) -> NodeDefinition {
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
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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
            .ok_or(ExecError::NodeError(
                "decimal multiplication overflow".into(),
            ))
    }
}

/// Decimal division: `Decimal(a) / Decimal(b)` → `Decimal(28)` (or
/// `Arbitrary` under `high-precision`).
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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
#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_graph::connection::Connection;
    use crate::node_graph::exec::{Engine, ExecutionReport, NodeOutcome};
    use crate::node_graph::graph::{NodeGraph, NodeId};
    use crate::node_graph::nodes::constant::Constant;
    use std::collections::BTreeMap;

    fn dec(s: &str) -> Decimal {
        Decimal::from_str_exact(s).unwrap()
    }

    fn run(graph: NodeGraph, ops: BTreeMap<NodeId, Box<dyn NodeOp>>) -> ExecutionReport {
        Engine::build(graph, ops).expect("build").execute()
    }

    /// Build `<lhs const 0> op <rhs const 1>` as node 2 and run it.
    fn run_binary(
        lhs: (Value, PortType),
        rhs: (Value, PortType),
        op: Box<dyn NodeOp>,
    ) -> ExecutionReport {
        let op_def = op.definition().clone();
        let c0 = Constant::with_type(lhs.0, lhs.1).unwrap();
        let c1 = Constant::with_type(rhs.0, rhs.1).unwrap();
        let mut graph = NodeGraph::new();
        graph.add_node(NodeId(0), c0.definition().clone());
        graph.add_node(NodeId(1), c1.definition().clone());
        graph.add_node(NodeId(2), op_def);
        graph.connect(Connection::new(NodeId(0), 0, NodeId(2), 0));
        graph.connect(Connection::new(NodeId(1), 0, NodeId(2), 1));
        let mut ops: BTreeMap<NodeId, Box<dyn NodeOp>> = BTreeMap::new();
        ops.insert(NodeId(0), Box::new(c0));
        ops.insert(NodeId(1), Box::new(c1));
        ops.insert(NodeId(2), op);
        run(graph, ops)
    }

    #[test]
    fn add_ports_tie_to_max_scale() {
        let n = AddDecimal::new(2, 4);
        let def = n.definition();
        assert_eq!(def.inputs[0].port_type, PortType::Decimal(2));
        assert_eq!(def.inputs[1].port_type, PortType::Decimal(4));
        assert_eq!(def.outputs[0].port_type, PortType::Decimal(4));
    }

    #[test]
    fn add_aligns_scales_exactly() {
        let report = run_binary(
            (Value::Decimal(dec("1.00")), PortType::Decimal(2)),
            (Value::Decimal(dec("2.0000")), PortType::Decimal(4)),
            Box::new(AddDecimal::new(2, 4)),
        );
        assert!(report.is_success());
        let v = &report.values[&(NodeId(2), 0)];
        assert_eq!(*v, Value::Decimal(dec("3.0000")));
        match v {
            Value::Decimal(d) => assert_eq!(d.scale(), 4),
            other => panic!("expected decimal, got {other:?}"),
        }
    }

    #[test]
    fn sub_borrows_negatively() {
        let report = run_binary(
            (Value::Decimal(dec("1.00")), PortType::Decimal(2)),
            (Value::Decimal(dec("2.50")), PortType::Decimal(2)),
            Box::new(SubDecimal::new(2, 2)),
        );
        assert!(report.is_success());
        assert_eq!(report.values[&(NodeId(2), 0)], Value::Decimal(dec("-1.50")));
    }

    #[test]
    fn mul_ports_and_exact_scale() {
        let n = MulDecimal::new(2, 4).unwrap();
        assert_eq!(n.definition().outputs[0].port_type, PortType::Decimal(6));

        let report = run_binary(
            (Value::Decimal(dec("1.50")), PortType::Decimal(2)),
            (Value::Decimal(dec("2.0000")), PortType::Decimal(4)),
            Box::new(n),
        );
        assert!(report.is_success());
        let v = &report.values[&(NodeId(2), 0)];
        assert_eq!(*v, Value::Decimal(dec("3.000000")));
        match v {
            Value::Decimal(d) => assert_eq!(d.scale(), 6),
            other => panic!("expected decimal, got {other:?}"),
        }
    }

    #[test]
    fn mul_refuses_scale_sum_over_28() {
        assert!(matches!(
            MulDecimal::new(15, 14),
            Err(ExecError::NodeError(_))
        ));
    }

    #[cfg(not(feature = "high-precision"))]
    #[test]
    fn div_ports_are_widest_decimal() {
        let n = DivDecimal::new(2, 4);
        let def = n.definition();
        assert_eq!(def.inputs[0].port_type, PortType::Decimal(2));
        assert_eq!(def.inputs[1].port_type, PortType::Decimal(4));
        assert_eq!(def.outputs[0].port_type, PortType::Decimal(28));
    }

    #[cfg(not(feature = "high-precision"))]
    #[test]
    fn div_runs_to_scale_28() {
        let report = run_binary(
            (Value::Decimal(dec("1.00")), PortType::Decimal(2)),
            (Value::Decimal(dec("3.00")), PortType::Decimal(2)),
            Box::new(DivDecimal::new(2, 2)),
        );
        assert!(report.is_success());
        let v = &report.values[&(NodeId(2), 0)];
        match v {
            Value::Decimal(d) => {
                assert_eq!(d.scale(), 28);
                assert!(d.to_string().starts_with("0.3333"));
            }
            other => panic!("expected decimal, got {other:?}"),
        }
    }

    #[test]
    fn div_by_zero_is_a_report_error() {
        let report = run_binary(
            (Value::Decimal(dec("1.00")), PortType::Decimal(2)),
            (Value::Decimal(dec("0.00")), PortType::Decimal(2)),
            Box::new(DivDecimal::new(2, 2)),
        );
        assert!(!report.is_success());
        assert_eq!(
            report.outcomes[&NodeId(2)],
            NodeOutcome::Failed(ExecError::DivideByZero)
        );
    }

    #[test]
    fn integer_add_overflow_reports() {
        let big = 2u64.pow(63) as i128;
        let report = run_binary(
            (Value::Integer(big), PortType::Integer(IntKind::U64)),
            (Value::Integer(big), PortType::Integer(IntKind::U64)),
            Box::new(AddInteger::new(IntKind::U64)),
        );
        assert_eq!(
            report.outcomes[&NodeId(2)],
            NodeOutcome::Failed(ExecError::IntegerOverflow { op: "add" })
        );
    }

    #[test]
    fn integer_add_happy_path() {
        let report = run_binary(
            (Value::Integer(5), PortType::Integer(IntKind::I64)),
            (Value::Integer(7), PortType::Integer(IntKind::I64)),
            Box::new(AddInteger::new(IntKind::I64)),
        );
        assert!(report.is_success());
        assert_eq!(report.values[&(NodeId(2), 0)], Value::Integer(12));
    }

    #[test]
    fn integer_div_and_mod() {
        let report = run_binary(
            (Value::Integer(7), PortType::Integer(IntKind::I64)),
            (Value::Integer(2), PortType::Integer(IntKind::I64)),
            Box::new(DivInteger::new(IntKind::I64)),
        );
        assert_eq!(report.values[&(NodeId(2), 0)], Value::Integer(3));

        let report = run_binary(
            (Value::Integer(7), PortType::Integer(IntKind::I64)),
            (Value::Integer(2), PortType::Integer(IntKind::I64)),
            Box::new(ModInteger::new(IntKind::I64)),
        );
        assert_eq!(report.values[&(NodeId(2), 0)], Value::Integer(1));

        let report = run_binary(
            (Value::Integer(7), PortType::Integer(IntKind::I64)),
            (Value::Integer(0), PortType::Integer(IntKind::I64)),
            Box::new(DivInteger::new(IntKind::I64)),
        );
        assert_eq!(
            report.outcomes[&NodeId(2)],
            NodeOutcome::Failed(ExecError::DivideByZero)
        );

        let report = run_binary(
            (Value::Integer(7), PortType::Integer(IntKind::I64)),
            (Value::Integer(0), PortType::Integer(IntKind::I64)),
            Box::new(ModInteger::new(IntKind::I64)),
        );
        assert_eq!(
            report.outcomes[&NodeId(2)],
            NodeOutcome::Failed(ExecError::DivideByZero)
        );
    }

    #[cfg(feature = "high-precision")]
    #[test]
    fn hp_div_outputs_arbitrary() {
        let n = DivDecimal::new(2, 4);
        assert_eq!(n.definition().outputs[0].port_type, PortType::Arbitrary);

        let report = run_binary(
            (Value::Decimal(dec("1.00")), PortType::Decimal(2)),
            (Value::Decimal(dec("3.00")), PortType::Decimal(4)),
            Box::new(n),
        );
        assert!(report.is_success());
        assert!(matches!(
            report.values[&(NodeId(2), 0)],
            Value::Arbitrary(_)
        ));
    }
}
