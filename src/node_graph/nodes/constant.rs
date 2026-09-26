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
#[derive(Debug)]
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
                return Err(ExecError::NodeError(
                    format!(
                        "custom value kind `{}` has no derivable port type; use with_type",
                        c.kind_name()
                    )
                    .into(),
                ))
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
            def: NodeDefinition::new("Constant", vec![], vec![PortDef::new("value", port_type)]),
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_graph::exec::{Engine, ExecutionReport, NodeOutcome};
    use crate::node_graph::graph::{NodeGraph, NodeId};
    use crate::node_graph::value::CustomValue;
    use rust_decimal::Decimal;
    use std::collections::BTreeMap;
    use std::sync::Arc;

    fn dec(s: &str) -> Decimal {
        Decimal::from_str_exact(s).unwrap()
    }

    fn run(graph: NodeGraph, ops: BTreeMap<NodeId, Box<dyn NodeOp>>) -> ExecutionReport {
        Engine::build(graph, ops).expect("build").execute()
    }

    #[test]
    fn derived_decimal_adopts_value_scale() {
        let c = Constant::new(Value::Decimal(dec("1.23"))).unwrap();
        let def = c.definition();
        assert!(def.inputs.is_empty());
        assert_eq!(def.outputs.len(), 1);
        assert_eq!(def.outputs[0].name, "value");
        assert_eq!(def.outputs[0].port_type, PortType::Decimal(2));
    }

    #[test]
    fn derived_integer_adopts_i128() {
        let c = Constant::new(Value::Integer(7)).unwrap();
        assert_eq!(
            c.definition().outputs[0].port_type,
            PortType::Integer(IntKind::I128)
        );
    }

    #[test]
    fn derived_text_and_bool() {
        let t = Constant::new(Value::Text("hi".into())).unwrap();
        assert_eq!(t.definition().outputs[0].port_type, PortType::Text);
        let b = Constant::new(Value::Bool(true)).unwrap();
        assert_eq!(b.definition().outputs[0].port_type, PortType::Bool);
    }

    struct TestCustom;

    impl CustomValue for TestCustom {
        fn kind_name(&self) -> &'static str {
            "test"
        }

        fn summary(&self) -> String {
            "s".into()
        }
    }

    #[test]
    fn custom_has_no_derived_type() {
        assert!(matches!(
            Constant::new(Value::Custom(Arc::new(TestCustom))),
            Err(ExecError::NodeError(_))
        ));
    }

    #[test]
    fn with_type_validates_and_widens() {
        let c = Constant::with_type(Value::Decimal(dec("1.5")), PortType::Decimal(4)).unwrap();
        assert_eq!(c.evaluate(&[]).unwrap(), vec![Value::Decimal(dec("1.5"))]);

        assert!(matches!(
            Constant::with_type(Value::Decimal(dec("1.500")), PortType::Decimal(2)),
            Err(ExecError::NodeError(_))
        ));
    }

    #[test]
    fn constant_runs_in_engine() {
        let c = Constant::new(Value::Decimal(dec("1.23"))).unwrap();
        let mut graph = NodeGraph::new();
        graph.add_node(NodeId(0), c.definition().clone());
        let mut ops: BTreeMap<NodeId, Box<dyn NodeOp>> = BTreeMap::new();
        ops.insert(NodeId(0), Box::new(c));
        let report = run(graph, ops);
        assert!(report.is_success());
        assert_eq!(report.outcomes[&NodeId(0)], NodeOutcome::Executed);
        assert_eq!(report.values[&(NodeId(0), 0)], Value::Decimal(dec("1.23")));
    }
}
