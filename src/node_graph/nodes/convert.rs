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
#[derive(Debug)]
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
            return Err(ExecError::NodeError(
                "rescale expects a decimal input".into(),
            ));
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
#[derive(Debug)]
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
#[derive(Debug)]
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
            return Err(ExecError::NodeError(
                "int→decimal expects an integer input".into(),
            ));
        };
        let mut d = Decimal::try_from_i128_with_scale(*v, 0)
            .map_err(|_| ExecError::NodeError("integer not representable as decimal".into()))?;
        d.rescale(self.scale);
        Ok(vec![Value::Decimal(d)])
    }
}

/// `Decimal(scale)` → `Integer(kind)`: rounds half-to-even to zero
/// fractional digits, then range-checks.
#[derive(Debug)]
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
            return Err(ExecError::NodeError(
                "decimal→int expects a decimal input".into(),
            ));
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

    /// Build `<input const 0> op` as node 1 and run it.
    fn run_unary(input: (Value, PortType), op: Box<dyn NodeOp>) -> ExecutionReport {
        let op_def = op.definition().clone();
        let c0 = Constant::with_type(input.0, input.1).unwrap();
        let mut graph = NodeGraph::new();
        graph.add_node(NodeId(0), c0.definition().clone());
        graph.add_node(NodeId(1), op_def);
        graph.connect(Connection::new(NodeId(0), 0, NodeId(1), 0));
        let mut ops: BTreeMap<NodeId, Box<dyn NodeOp>> = BTreeMap::new();
        ops.insert(NodeId(0), Box::new(c0));
        ops.insert(NodeId(1), op);
        run(graph, ops)
    }

    fn expect_node_error_containing(report: &ExecutionReport, needle: &str) {
        match &report.outcomes[&NodeId(1)] {
            NodeOutcome::Failed(ExecError::NodeError(m)) => {
                assert!(m.contains(needle), "error `{m}` lacks `{needle}`")
            }
            other => panic!("expected NodeError, got {other:?}"),
        }
    }

    #[test]
    fn rescale_widens_with_zero_padding() {
        let n = RescaleDecimal::new(2, 4);
        let def = n.definition();
        assert_eq!(def.inputs[0].port_type, PortType::Decimal(2));
        assert_eq!(def.outputs[0].port_type, PortType::Decimal(4));

        let report = run_unary(
            (Value::Decimal(dec("1.50")), PortType::Decimal(2)),
            Box::new(n),
        );
        assert!(report.is_success());
        let v = &report.values[&(NodeId(1), 0)];
        assert_eq!(*v, Value::Decimal(dec("1.5000")));
        match v {
            Value::Decimal(d) => assert_eq!(d.scale(), 4),
            other => panic!("expected decimal, got {other:?}"),
        }
    }

    #[test]
    fn rescale_narrows_with_bankers_rounding() {
        let report = run_unary(
            (Value::Decimal(dec("0.005")), PortType::Decimal(3)),
            Box::new(RescaleDecimal::new(3, 2)),
        );
        assert!(report.is_success());
        assert_eq!(report.values[&(NodeId(1), 0)], Value::Decimal(dec("0.00")));

        let report = run_unary(
            (Value::Decimal(dec("0.015")), PortType::Decimal(3)),
            Box::new(RescaleDecimal::new(3, 2)),
        );
        assert!(report.is_success());
        assert_eq!(report.values[&(NodeId(1), 0)], Value::Decimal(dec("0.02")));
    }

    #[test]
    fn rescale_identity_when_scales_equal() {
        let report = run_unary(
            (Value::Decimal(dec("1.234")), PortType::Decimal(3)),
            Box::new(RescaleDecimal::new(3, 3)),
        );
        assert!(report.is_success());
        assert_eq!(report.values[&(NodeId(1), 0)], Value::Decimal(dec("1.234")));
    }

    #[test]
    fn rescale_pads_lower_scale_input() {
        let report = run_unary(
            (Value::Decimal(dec("1.5")), PortType::Decimal(4)),
            Box::new(RescaleDecimal::new(4, 6)),
        );
        assert!(report.is_success());
        assert_eq!(
            report.values[&(NodeId(1), 0)],
            Value::Decimal(dec("1.500000"))
        );
    }

    #[test]
    fn integer_cast_widens_exactly() {
        let report = run_unary(
            (Value::Integer(5), PortType::Integer(IntKind::I64)),
            Box::new(IntegerCast::new(IntKind::I64, IntKind::I128)),
        );
        assert!(report.is_success());
        assert_eq!(report.values[&(NodeId(1), 0)], Value::Integer(5));
    }

    #[test]
    fn integer_cast_narrows_with_range_check() {
        let report = run_unary(
            (
                Value::Integer(i64::MAX as i128 + 1),
                PortType::Integer(IntKind::I128),
            ),
            Box::new(IntegerCast::new(IntKind::I128, IntKind::I64)),
        );
        expect_node_error_containing(&report, "out of range");

        let report = run_unary(
            (Value::Integer(-1), PortType::Integer(IntKind::I128)),
            Box::new(IntegerCast::new(IntKind::I128, IntKind::U64)),
        );
        expect_node_error_containing(&report, "out of range");
    }

    #[test]
    fn int_to_decimal_zero_pads() {
        let report = run_unary(
            (Value::Integer(5), PortType::Integer(IntKind::I64)),
            Box::new(IntToDecimal::new(IntKind::I64, 2)),
        );
        assert!(report.is_success());
        let v = &report.values[&(NodeId(1), 0)];
        assert_eq!(*v, Value::Decimal(dec("5.00")));
        match v {
            Value::Decimal(d) => assert_eq!(d.scale(), 2),
            other => panic!("expected decimal, got {other:?}"),
        }

        // The declared port kind is I64, so an i128::MAX operand cannot sit
        // on the input port; exercise the op directly to reach the decimal
        // range check.
        let err = IntToDecimal::new(IntKind::I64, 2)
            .evaluate(&[Value::Integer(i128::MAX)])
            .unwrap_err();
        match err {
            ExecError::NodeError(m) => assert!(m.contains("not representable")),
            other => panic!("expected NodeError, got {other:?}"),
        }
    }

    #[test]
    fn decimal_to_int_rounds_bankers_then_range_checks() {
        let report = run_unary(
            (Value::Decimal(dec("2.5")), PortType::Decimal(2)),
            Box::new(DecimalToInt::new(2, IntKind::I64)),
        );
        assert!(report.is_success());
        assert_eq!(report.values[&(NodeId(1), 0)], Value::Integer(2));

        let report = run_unary(
            (Value::Decimal(dec("-2.5")), PortType::Decimal(2)),
            Box::new(DecimalToInt::new(2, IntKind::I64)),
        );
        assert!(report.is_success());
        assert_eq!(report.values[&(NodeId(1), 0)], Value::Integer(-2));

        let report = run_unary(
            (Value::Decimal(dec("3.99")), PortType::Decimal(2)),
            Box::new(DecimalToInt::new(2, IntKind::I64)),
        );
        assert!(report.is_success());
        assert_eq!(report.values[&(NodeId(1), 0)], Value::Integer(4));

        let report = run_unary(
            (
                Value::Decimal(dec("99999999999999999999")),
                PortType::Decimal(2),
            ),
            Box::new(DecimalToInt::new(2, IntKind::I64)),
        );
        expect_node_error_containing(&report, "out of range");
    }
}
