// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! Execution engine: node operations, topological ordering, and the
//! refuse-to-fire evaluator (7C).
//!
//! Pure kernel: no Leptos, no wasm — natively testable.
//!
//! ## The numerics contract
//!
//! The engine **refuses** to carry a value across a connection whose
//! precision verdict is `Lossy` or `Incompatible`: the refusal is recorded
//! as a structured error in the [`ExecutionReport`], the node fails, and
//! every transitively downstream node is skipped. No implicit narrowing
//! ever happens at runtime; explicit conversion nodes are the only
//! sanctioned way to change precision. Unconnected inputs are failures
//! (there are no defaults), and an operation whose emitted values drift
//! from its declared output port types fails with `PortTypeMismatch` —
//! an op lying about its definition is surfaced, not trusted.

use crate::node_graph::connection::Connection;
use crate::node_graph::graph::{NodeGraph, NodeId};
use crate::node_graph::node::NodeDefinition;
use crate::node_graph::precision::{check_connection, ConnectionVerdict, PortType};
use crate::node_graph::value::Value;
use std::collections::{BTreeMap, BTreeSet};
use thiserror::Error;

/// A node operation: the static port truth plus the evaluation.
pub trait NodeOp: Send + Sync {
    /// Port definitions — the engine treats these as the runtime truth for
    /// connection and value coherence checks.
    fn definition(&self) -> &NodeDefinition;

    /// Evaluate with positional inputs (one per declared input port, in
    /// order); returns one value per declared output port, in order.
    fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, ExecError>;
}

/// Engine and operation errors. Structured, never panics.
#[derive(Clone, Debug, Error, PartialEq)]
pub enum ExecError {
    #[error("cycle detected: execution requires an acyclic graph")]
    Cycle,
    #[error("no operation registered for node {}", id.0)]
    MissingOp { id: NodeId },
    #[error("operation registered for unknown node {}", id.0)]
    OrphanOp { id: NodeId },
    #[error("connection references port {port} of node {} ({side:?}) which does not exist", id.0)]
    InvalidPort {
        id: NodeId,
        port: u32,
        side: PortSideTag,
    },
    #[error("input {index} of node {} is not connected", id.0)]
    MissingInput { id: NodeId, index: usize },
    #[error(
        "refused {conn:?}: {} — {}",
        verdict_name(verdict),
        verdict_reason(verdict)
    )]
    RefusedLossyEdge {
        conn: Connection,
        verdict: ConnectionVerdict,
    },
    #[error("output port {port} of node {} produced {got:?}, expected {expected:?}", id.0)]
    PortTypeMismatch {
        id: NodeId,
        port: u32,
        expected: PortType,
        got: crate::node_graph::value::ValueKind,
    },
    #[error("operation of node {} produced {produced} outputs, expected {expected}", id.0)]
    OutputCountMismatch {
        id: NodeId,
        produced: usize,
        expected: usize,
    },
    #[error("integer overflow in {op}")]
    IntegerOverflow { op: &'static str },
    #[error("division by zero")]
    DivideByZero,
    #[error("{0}")]
    NodeError(Cow<'static, str>),
}

/// Which end of a connection a port belongs to (for error reporting).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSideTag {
    Source,
    Target,
}

/// Name a verdict for error display (shared by the `Display` impls above).
fn verdict_name(verdict: &ConnectionVerdict) -> &'static str {
    match verdict {
        ConnectionVerdict::Ok => "ok",
        ConnectionVerdict::Lossy(_) => "lossy transfer",
        ConnectionVerdict::Incompatible(_) => "incompatible transfer",
    }
}

/// Human-readable reason from a verdict (empty for `Ok`).
fn verdict_reason(verdict: &ConnectionVerdict) -> String {
    match verdict {
        ConnectionVerdict::Ok => "exact".to_string(),
        ConnectionVerdict::Lossy(loss) => loss.detail.to_string(),
        ConnectionVerdict::Incompatible(reason) => reason.to_string(),
    }
}

/// Kahn topological order over the node-level graph. Deterministic
/// (lowest id first among ready nodes). Errors on cycles.
pub fn topo_order(graph: &NodeGraph) -> Result<Vec<NodeId>, ExecError> {
    let nodes: BTreeSet<NodeId> = graph.node_ids().collect();
    // Node-level adjacency, deduplicated across parallel port edges.
    let mut forward: BTreeMap<NodeId, BTreeSet<NodeId>> =
        nodes.iter().map(|&n| (n, BTreeSet::new())).collect();
    let mut indegree: BTreeMap<NodeId, usize> = nodes.iter().map(|&n| (n, 0)).collect();
    for conn in graph.connections() {
        if forward
            .entry(conn.from_node)
            .or_default()
            .insert(conn.to_node)
        {
            *indegree.entry(conn.to_node).or_default() += 1;
        }
    }
    let mut ready: BTreeSet<NodeId> = indegree
        .iter()
        .filter(|(_, &d)| d == 0)
        .map(|(&n, _)| n)
        .collect();
    let mut order = Vec::with_capacity(nodes.len());
    while let Some(&next) = ready.iter().next() {
        ready.remove(&next);
        order.push(next);
        let mut newly_ready = Vec::new();
        if let Some(neighbors) = forward.get(&next) {
            for &n in neighbors {
                let d = indegree
                    .get_mut(&n)
                    .expect("indegree tracked for every node");
                *d -= 1;
                if *d == 0 {
                    newly_ready.push(n);
                }
            }
        }
        ready.extend(newly_ready);
    }
    if order.len() == nodes.len() {
        Ok(order)
    } else {
        Err(ExecError::Cycle)
    }
}

/// Per-node execution outcome.
#[derive(Clone, Debug, PartialEq)]
pub enum NodeOutcome {
    Executed,
    Failed(ExecError),
    Skipped,
}

/// The full result of one engine run.
#[derive(Clone, Debug, Default)]
pub struct ExecutionReport {
    pub outcomes: BTreeMap<NodeId, NodeOutcome>,
    /// Last value emitted per (node, output port).
    pub values: BTreeMap<(NodeId, u32), Value>,
    pub errors: Vec<(NodeId, ExecError)>,
}

impl ExecutionReport {
    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
    }
}

/// A built, runnable graph: topology plus the operation registry.
pub struct Engine {
    graph: NodeGraph,
    ops: BTreeMap<NodeId, Box<dyn NodeOp>>,
}

impl Engine {
    /// Validate the pairing of graph and operations: every node has an op,
    /// no orphans, and every connection references real ports of the *ops'*
    /// definitions.
    pub fn build(
        graph: NodeGraph,
        ops: BTreeMap<NodeId, Box<dyn NodeOp>>,
    ) -> Result<Self, ExecError> {
        // Every node needs an op; every op needs a node.
        for id in graph.node_ids() {
            if !ops.contains_key(&id) {
                return Err(ExecError::MissingOp { id });
            }
        }
        for &id in ops.keys() {
            if graph.node(id).is_none() {
                return Err(ExecError::OrphanOp { id });
            }
        }
        // Every connection must reference real ports of the ops' definitions.
        for conn in graph.connections() {
            let from_def = ops
                .get(&conn.from_node)
                .expect("checked above")
                .definition();
            if from_def.output_type(conn.from_output).is_none() {
                return Err(ExecError::InvalidPort {
                    id: conn.from_node,
                    port: conn.from_output,
                    side: PortSideTag::Source,
                });
            }
            let to_def = ops.get(&conn.to_node).expect("checked above").definition();
            if to_def.input_type(conn.to_input).is_none() {
                return Err(ExecError::InvalidPort {
                    id: conn.to_node,
                    port: conn.to_input,
                    side: PortSideTag::Target,
                });
            }
        }
        Ok(Engine { graph, ops })
    }

    /// Run the whole graph to completion. Infallible: failures land in the
    /// report as structured errors (a cycle fails every node).
    pub fn execute(&self) -> ExecutionReport {
        let mut report = ExecutionReport::default();
        let order = match topo_order(&self.graph) {
            Ok(order) => order,
            Err(err) => {
                // Structurally unrunnable: every node skipped, error carried
                // by the lowest node id (or none, for an empty graph).
                for id in self.graph.node_ids() {
                    report.outcomes.insert(id, NodeOutcome::Skipped);
                }
                if let Some(&carrier) = self.ops.keys().next() {
                    report.errors.push((carrier, err));
                }
                return report;
            }
        };
        for id in order {
            self.run_node(id, &mut report);
        }
        report
    }

    /// Run one node: skip-if-upstream-failed, input gathering with transfer
    /// verdicts (refuse-to-fire), evaluation, and output coherence checks.
    fn run_node(&self, id: NodeId, report: &mut ExecutionReport) {
        let incoming: Vec<&Connection> = self
            .graph
            .connections()
            .iter()
            .filter(|c| c.to_node == id)
            .collect();

        // Upstream failure or refusal skips this node entirely.
        if incoming
            .iter()
            .any(|c| report.outcomes.get(&c.from_node) != Some(&NodeOutcome::Executed))
        {
            report.outcomes.insert(id, NodeOutcome::Skipped);
            return;
        }

        let def = self
            .ops
            .get(&id)
            .expect("build checked coverage")
            .definition();

        // Every declared input must be connected — there are no defaults.
        for idx in 0..def.inputs.len() {
            if !incoming.iter().any(|c| c.to_input as usize == idx) {
                let err = ExecError::MissingInput { id, index: idx };
                report.outcomes.insert(id, NodeOutcome::Failed(err.clone()));
                report.errors.push((id, err));
                return;
            }
        }

        // Gather inputs positionally; refuse any lossy/incompatible transfer.
        let mut inputs: Vec<Value> = Vec::with_capacity(def.inputs.len());
        for _ in 0..def.inputs.len() {
            inputs.push(Value::Bool(false)); // placeholder, replaced below
        }
        for conn in &incoming {
            let from_def = self
                .ops
                .get(&conn.from_node)
                .expect("build checked coverage")
                .definition();
            let src_ty = from_def
                .output_type(conn.from_output)
                .expect("build checked ports");
            let dst_ty = def.input_type(conn.to_input).expect("build checked ports");
            match check_connection(src_ty, dst_ty) {
                ConnectionVerdict::Ok => {}
                verdict => {
                    let err = ExecError::RefusedLossyEdge {
                        conn: **conn,
                        verdict,
                    };
                    report.outcomes.insert(id, NodeOutcome::Failed(err.clone()));
                    report.errors.push((id, err));
                    return;
                }
            }
            let value = report
                .values
                .get(&(conn.from_node, conn.from_output))
                .expect("upstream executed: value recorded")
                .clone();
            inputs[conn.to_input as usize] = value;
        }

        // Evaluate, then hold the op to its declared outputs.
        match self
            .ops
            .get(&id)
            .expect("build checked coverage")
            .evaluate(&inputs)
        {
            Err(err) => {
                report.outcomes.insert(id, NodeOutcome::Failed(err.clone()));
                report.errors.push((id, err));
            }
            Ok(outputs) => {
                if outputs.len() != def.outputs.len() {
                    let err = ExecError::OutputCountMismatch {
                        id,
                        produced: outputs.len(),
                        expected: def.outputs.len(),
                    };
                    report.outcomes.insert(id, NodeOutcome::Failed(err.clone()));
                    report.errors.push((id, err));
                    return;
                }
                for (port, (value, port_def)) in
                    outputs.into_iter().zip(def.outputs.iter()).enumerate()
                {
                    if value.matches(&port_def.port_type) {
                        report.values.insert((id, port as u32), value);
                    } else {
                        let err = ExecError::PortTypeMismatch {
                            id,
                            port: port as u32,
                            expected: port_def.port_type.clone(),
                            got: value.kind(),
                        };
                        report.outcomes.insert(id, NodeOutcome::Failed(err.clone()));
                        report.errors.push((id, err));
                        return;
                    }
                }
                report.outcomes.insert(id, NodeOutcome::Executed);
            }
        }
    }
}

use std::borrow::Cow;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_graph::node::PortDef;
    use crate::node_graph::value::ValueKind;
    use rust_decimal::Decimal;
    use std::sync::Arc;

    // -- test scaffolding ---------------------------------------------------

    /// An ad-hoc operation for tests: fixed definition, canned outputs.
    struct FnNode {
        def: NodeDefinition,
        outputs: Vec<Value>,
    }
    impl FnNode {
        fn boxed(def: NodeDefinition, outputs: Vec<Value>) -> Box<dyn NodeOp> {
            Box::new(Self { def, outputs })
        }
    }
    impl NodeOp for FnNode {
        fn definition(&self) -> &NodeDefinition {
            &self.def
        }
        fn evaluate(&self, _inputs: &[Value]) -> Result<Vec<Value>, ExecError> {
            Ok(self.outputs.clone())
        }
    }

    fn constant(id: u64, value: &str, scale: u32) -> (NodeId, Box<dyn NodeOp>) {
        (
            NodeId(id),
            FnNode::boxed(
                NodeDefinition::new(
                    "constant",
                    vec![],
                    vec![PortDef::new("out", PortType::Decimal(scale))],
                ),
                vec![Value::Decimal(Decimal::from_str_exact(value).unwrap())],
            ),
        )
    }

    /// Pass-through: one decimal input, one decimal output, emits 1.00.
    fn sink(id: u64, scale: u32) -> (NodeId, Box<dyn NodeOp>) {
        (
            NodeId(id),
            FnNode::boxed(
                NodeDefinition::new(
                    "pass",
                    vec![PortDef::new("in", PortType::Decimal(scale))],
                    vec![PortDef::new("out", PortType::Decimal(scale))],
                ),
                vec![Value::Decimal(Decimal::from_str_exact("1.00").unwrap())],
            ),
        )
    }

    fn two_node_graph(
        from_scale: u32,
        to_scale: u32,
    ) -> (NodeGraph, BTreeMap<NodeId, Box<dyn NodeOp>>, Connection) {
        let mut g = NodeGraph::new();
        g.add_node(
            NodeId(0),
            constant(0, "1.00", from_scale).1.definition().clone(),
        );
        g.add_node(NodeId(1), sink(1, to_scale).1.definition().clone());
        let conn = Connection::new(NodeId(0), 0, NodeId(1), 0);
        g.connect(conn);
        let mut ops = BTreeMap::new();
        ops.insert(
            constant(0, "1.00", from_scale).0,
            constant(0, "1.00", from_scale).1,
        );
        ops.insert(sink(1, to_scale).0, sink(1, to_scale).1);
        (g, ops, conn)
    }

    // -- topo_order ----------------------------------------------------------

    #[test]
    fn topo_linear_chain_orders_earlier_first() {
        let mut g = NodeGraph::new();
        for id in 0..3 {
            g.add_node(NodeId(id), sink(id, 2).1.definition().clone());
        }
        g.connect(Connection::new(NodeId(0), 0, NodeId(1), 0));
        g.connect(Connection::new(NodeId(1), 0, NodeId(2), 0));
        assert_eq!(topo_order(&g), Ok(vec![NodeId(0), NodeId(1), NodeId(2)]));
    }

    #[test]
    fn topo_diamond_orders_source_before_sinks_before_join() {
        let mut g = NodeGraph::new();
        for id in 0..4 {
            g.add_node(NodeId(id), sink(id, 2).1.definition().clone());
        }
        g.connect(Connection::new(NodeId(0), 0, NodeId(1), 0));
        g.connect(Connection::new(NodeId(0), 0, NodeId(2), 0));
        g.connect(Connection::new(NodeId(1), 0, NodeId(3), 0));
        g.connect(Connection::new(NodeId(2), 0, NodeId(3), 0));
        // Deterministic but shape-free assertions: 0 first, 3 last.
        let order = topo_order(&g).unwrap();
        assert_eq!(order.first(), Some(&NodeId(0)));
        assert_eq!(order.last(), Some(&NodeId(3)));
        assert_eq!(order.len(), 4);
    }

    #[test]
    fn topo_cycle_is_an_error() {
        let mut g = NodeGraph::new();
        for id in 0..2 {
            g.add_node(NodeId(id), sink(id, 2).1.definition().clone());
        }
        g.connect(Connection::new(NodeId(0), 0, NodeId(1), 0));
        g.connect(Connection::new(NodeId(1), 0, NodeId(0), 0));
        assert_eq!(topo_order(&g), Err(ExecError::Cycle));
    }

    #[test]
    fn topo_empty_graph_is_empty_order() {
        assert_eq!(topo_order(&NodeGraph::new()), Ok(vec![]));
    }

    // -- Engine::build -------------------------------------------------------

    #[test]
    fn build_rejects_missing_op() {
        let mut g = NodeGraph::new();
        g.add_node(NodeId(0), sink(0, 2).1.definition().clone());
        assert_eq!(
            Engine::build(g, BTreeMap::new()).err(),
            Some(ExecError::MissingOp { id: NodeId(0) })
        );
    }

    #[test]
    fn build_rejects_orphan_op() {
        let mut ops = BTreeMap::new();
        ops.insert(sink(9, 2).0, sink(9, 2).1);
        assert_eq!(
            Engine::build(NodeGraph::new(), ops).err(),
            Some(ExecError::OrphanOp { id: NodeId(9) })
        );
    }

    #[test]
    fn build_rejects_connection_with_out_of_range_port() {
        let mut g = NodeGraph::new();
        g.add_node(NodeId(0), constant(0, "1.00", 2).1.definition().clone());
        g.add_node(NodeId(1), sink(1, 2).1.definition().clone());
        // Output port 5 does not exist on node 0 (it has one output: 0).
        g.connect(Connection::new(NodeId(0), 5, NodeId(1), 0));
        let mut ops = BTreeMap::new();
        ops.insert(constant(0, "1.00", 2).0, constant(0, "1.00", 2).1);
        ops.insert(sink(1, 2).0, sink(1, 2).1);
        assert_eq!(
            Engine::build(g, ops).err(),
            Some(ExecError::InvalidPort {
                id: NodeId(0),
                port: 5,
                side: PortSideTag::Source,
            })
        );
    }

    // -- Engine::execute -----------------------------------------------------

    #[test]
    fn execute_happy_path_records_values() {
        let (g, ops, conn) = two_node_graph(2, 2);
        let engine = Engine::build(g, ops).unwrap();
        let report = engine.execute();
        assert!(report.is_success(), "{report:?}");
        assert_eq!(
            report.outcomes.get(&NodeId(0)),
            Some(&NodeOutcome::Executed)
        );
        assert_eq!(
            report.outcomes.get(&NodeId(1)),
            Some(&NodeOutcome::Executed)
        );
        assert_eq!(
            report.values.get(&(NodeId(0), 0)),
            Some(&Value::Decimal(Decimal::from_str_exact("1.00").unwrap()))
        );
        let _ = conn;
    }

    #[test]
    fn execute_widening_decimal_edge_passes() {
        // Decimal(2) -> Decimal(4) is exact (Ok verdict) — must pass.
        let (g, ops, _) = two_node_graph(2, 4);
        let report = Engine::build(g, ops).unwrap().execute();
        assert!(report.is_success(), "{report:?}");
    }

    #[test]
    fn execute_refuses_lossy_decimal_edge() {
        // Decimal(4) -> Decimal(2) is Lossy: refuse-to-fire.
        let (g, ops, conn) = two_node_graph(4, 2);
        let report = Engine::build(g, ops).unwrap().execute();
        assert!(!report.is_success());
        let (id, err) = report.errors.first().expect("an error").clone();
        assert_eq!(id, NodeId(1), "the receiving node fails: {err}");
        assert!(matches!(err, ExecError::RefusedLossyEdge { .. }), "{err}");
        if let ExecError::RefusedLossyEdge { conn: c, .. } = err {
            assert_eq!(c, conn);
        }
        assert_eq!(
            report.outcomes.get(&NodeId(1)),
            Some(&NodeOutcome::Failed(ExecError::RefusedLossyEdge {
                conn,
                verdict: check_connection(&PortType::Decimal(4), &PortType::Decimal(2)),
            }))
        );
    }

    #[test]
    fn execute_refuses_incompatible_edge() {
        // Text -> Decimal is Incompatible.
        let mut g = NodeGraph::new();
        let text_def =
            NodeDefinition::new("text", vec![], vec![PortDef::new("out", PortType::Text)]);
        g.add_node(NodeId(0), text_def.clone());
        g.add_node(NodeId(1), sink(1, 2).1.definition().clone());
        g.connect(Connection::new(NodeId(0), 0, NodeId(1), 0));
        let mut ops: BTreeMap<NodeId, Box<dyn NodeOp>> = BTreeMap::new();
        ops.insert(
            NodeId(0),
            FnNode::boxed(text_def, vec![Value::Text("1.00".into())]),
        );
        ops.insert(sink(1, 2).0, sink(1, 2).1);
        let report = Engine::build(g, ops).unwrap().execute();
        assert!(!report.is_success());
        assert!(matches!(
            report.errors.first().map(|(_, e)| e),
            Some(ExecError::RefusedLossyEdge { .. })
        ));
    }

    #[test]
    fn execute_unconnected_input_fails_and_skips_downstream() {
        let mut g = NodeGraph::new();
        g.add_node(NodeId(0), sink(0, 2).1.definition().clone());
        g.add_node(NodeId(1), sink(1, 2).1.definition().clone());
        g.connect(Connection::new(NodeId(0), 0, NodeId(1), 0));
        // Node 0's input is unconnected: 0 fails, 1 skips.
        let mut ops = BTreeMap::new();
        ops.insert(sink(0, 2).0, sink(0, 2).1);
        ops.insert(sink(1, 2).0, sink(1, 2).1);
        let report = Engine::build(g, ops).unwrap().execute();
        assert!(!report.is_success());
        assert!(matches!(
            report.outcomes.get(&NodeId(0)),
            Some(NodeOutcome::Failed(ExecError::MissingInput { .. }))
        ));
        assert_eq!(report.outcomes.get(&NodeId(1)), Some(&NodeOutcome::Skipped));
    }

    #[test]
    fn execute_skip_propagates_transitively() {
        let mut g = NodeGraph::new();
        for id in 0..3 {
            g.add_node(NodeId(id), sink(id, 2).1.definition().clone());
        }
        g.connect(Connection::new(NodeId(0), 0, NodeId(1), 0));
        g.connect(Connection::new(NodeId(1), 0, NodeId(2), 0));
        let mut ops = BTreeMap::new();
        for id in 0..3 {
            ops.insert(sink(id, 2).0, sink(id, 2).1);
        }
        let report = Engine::build(g, ops).unwrap().execute();
        assert!(matches!(
            report.outcomes.get(&NodeId(0)),
            Some(NodeOutcome::Failed(_))
        ));
        assert_eq!(report.outcomes.get(&NodeId(1)), Some(&NodeOutcome::Skipped));
        assert_eq!(report.outcomes.get(&NodeId(2)), Some(&NodeOutcome::Skipped));
    }

    #[test]
    fn execute_flags_output_type_drift() {
        // Op declares a Bool output but emits a Decimal: surfaced, not trusted.
        let mut g = NodeGraph::new();
        let def = NodeDefinition::new("liar", vec![], vec![PortDef::new("flag", PortType::Bool)]);
        g.add_node(NodeId(0), def.clone());
        let mut ops: BTreeMap<NodeId, Box<dyn NodeOp>> = BTreeMap::new();
        ops.insert(
            NodeId(0),
            FnNode::boxed(def, vec![Value::Decimal(Decimal::ONE)]),
        );
        let report = Engine::build(g, ops).unwrap().execute();
        assert!(matches!(
            report.outcomes.get(&NodeId(0)),
            Some(NodeOutcome::Failed(ExecError::PortTypeMismatch {
                port: 0,
                ..
            }))
        ));
    }

    #[test]
    fn execute_flags_output_scale_drift() {
        // Declares Decimal(2) output but emits scale-3 value.
        let mut g = NodeGraph::new();
        let def = NodeDefinition::new(
            "scale-liar",
            vec![],
            vec![PortDef::new("out", PortType::Decimal(2))],
        );
        g.add_node(NodeId(0), def.clone());
        let mut ops: BTreeMap<NodeId, Box<dyn NodeOp>> = BTreeMap::new();
        ops.insert(
            NodeId(0),
            FnNode::boxed(
                def,
                vec![Value::Decimal(Decimal::from_str_exact("1.000").unwrap())],
            ),
        );
        let report = Engine::build(g, ops).unwrap().execute();
        assert!(matches!(
            report.outcomes.get(&NodeId(0)),
            Some(NodeOutcome::Failed(ExecError::PortTypeMismatch {
                port: 0,
                ..
            }))
        ));
    }

    #[test]
    fn execute_flags_output_count_drift() {
        let mut g = NodeGraph::new();
        let def = NodeDefinition::new(
            "two-outputs",
            vec![],
            vec![
                PortDef::new("a", PortType::Bool),
                PortDef::new("b", PortType::Bool),
            ],
        );
        g.add_node(NodeId(0), def.clone());
        let mut ops: BTreeMap<NodeId, Box<dyn NodeOp>> = BTreeMap::new();
        ops.insert(NodeId(0), FnNode::boxed(def, vec![Value::Bool(true)]));
        let report = Engine::build(g, ops).unwrap().execute();
        assert!(matches!(
            report.outcomes.get(&NodeId(0)),
            Some(NodeOutcome::Failed(ExecError::OutputCountMismatch { .. }))
        ));
    }

    #[test]
    fn execute_cycle_fails_every_node_structurally() {
        let mut g = NodeGraph::new();
        for id in 0..2 {
            g.add_node(NodeId(id), sink(id, 2).1.definition().clone());
        }
        g.connect(Connection::new(NodeId(0), 0, NodeId(1), 0));
        g.connect(Connection::new(NodeId(1), 0, NodeId(0), 0));
        let mut ops = BTreeMap::new();
        ops.insert(sink(0, 2).0, sink(0, 2).1);
        ops.insert(sink(1, 2).0, sink(1, 2).1);
        let report = Engine::build(g, ops).unwrap().execute();
        assert!(!report.is_success());
        assert!(report
            .errors
            .iter()
            .any(|(_, e)| matches!(e, ExecError::Cycle)));
    }

    #[test]
    fn execute_passes_values_positionally() {
        // Two inputs, both fed: sink sees both values in port order.
        use std::sync::Mutex;
        static SEEN: Mutex<Vec<Value>> = Mutex::new(Vec::new());
        struct Recorder(NodeDefinition);
        impl NodeOp for Recorder {
            fn definition(&self) -> &NodeDefinition {
                &self.0
            }
            fn evaluate(&self, inputs: &[Value]) -> Result<Vec<Value>, ExecError> {
                SEEN.lock().unwrap().extend(inputs.iter().cloned());
                Ok(vec![])
            }
        }
        let mut g = NodeGraph::new();
        let rec_def = NodeDefinition::new(
            "recorder",
            vec![
                PortDef::new("a", PortType::Decimal(2)),
                PortDef::new("b", PortType::Decimal(2)),
            ],
            vec![],
        );
        g.add_node(NodeId(0), constant(0, "1.00", 2).1.definition().clone());
        g.add_node(NodeId(1), constant(1, "2.00", 2).1.definition().clone());
        g.add_node(NodeId(2), rec_def.clone());
        g.connect(Connection::new(NodeId(0), 0, NodeId(2), 0));
        g.connect(Connection::new(NodeId(1), 0, NodeId(2), 1));
        let mut ops: BTreeMap<NodeId, Box<dyn NodeOp>> = BTreeMap::new();
        ops.insert(constant(0, "1.00", 2).0, constant(0, "1.00", 2).1);
        ops.insert(constant(1, "2.00", 2).0, constant(1, "2.00", 2).1);
        ops.insert(NodeId(2), Box::new(Recorder(rec_def)));
        let report = Engine::build(g, ops).unwrap().execute();
        assert!(report.is_success(), "{report:?}");
        let seen = SEEN.lock().unwrap().clone();
        assert_eq!(
            seen,
            vec![
                Value::Decimal(Decimal::from_str_exact("1.00").unwrap()),
                Value::Decimal(Decimal::from_str_exact("2.00").unwrap()),
            ]
        );
        let _ = ValueKind::Custom; // keep import exercised
        let _ = Arc::new(0u8); // keep Arc import exercised
    }
}
