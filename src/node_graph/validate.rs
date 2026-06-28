// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! Whole-graph validation: combines structural checks (do connections reference
//! real nodes and ports?), precision checks (is each connection's source type
//! representable by its target?), and cycle detection into one report.
//!
//! A graph may have several independent problems; [`validate`] returns all of
//! them so the editor can surface every issue at once rather than one at a time.

use crate::node_graph::connection::Connection;
use crate::node_graph::graph::NodeGraph;
use crate::node_graph::precision::{check_connection, ConnectionVerdict};

/// One problem found while validating a graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationIssue {
    pub kind: IssueKind,
    pub message: String,
    /// The connection at fault, when applicable.
    pub connection: Option<Connection>,
}

/// The category of a [`ValidationIssue`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IssueKind {
    /// A connection references a node that is not in the graph.
    DanglingNode,
    /// A connection's source output port index is out of range.
    SourcePortOutOfRange,
    /// A connection's target input port index is out of range.
    TargetPortOutOfRange,
    /// A connection whose source values may lose precision in the target.
    PrecisionLoss,
    /// A connection whose source and target types cannot be connected.
    Incompatible,
    /// The graph contains a cycle.
    Cycle,
}

/// The outcome of validating a graph: every issue found, in encounter order.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ValidationReport {
    pub issues: Vec<ValidationIssue>,
}

impl ValidationReport {
    /// Whether the graph has no issues.
    pub fn is_ok(&self) -> bool {
        self.issues.is_empty()
    }

    /// The list of issues found.
    pub fn issues(&self) -> &[ValidationIssue] {
        &self.issues
    }
}

/// Validate `graph`, returning a report of every structural, precision, and
/// cycle issue found.
pub fn validate(graph: &NodeGraph) -> ValidationReport {
    let mut issues = Vec::new();

    for conn in graph.connections() {
        // Structural: both endpoints must exist.
        let from_def = match graph.node(conn.from_node) {
            Some(def) => def,
            None => {
                issues.push(ValidationIssue {
                    kind: IssueKind::DanglingNode,
                    message: format!(
                        "connection source node {} is not in the graph",
                        conn.from_node.0
                    ),
                    connection: Some(*conn),
                });
                continue;
            }
        };
        let to_def = match graph.node(conn.to_node) {
            Some(def) => def,
            None => {
                issues.push(ValidationIssue {
                    kind: IssueKind::DanglingNode,
                    message: format!(
                        "connection target node {} is not in the graph",
                        conn.to_node.0
                    ),
                    connection: Some(*conn),
                });
                continue;
            }
        };

        // Structural: port indices must be in range.
        let src_type = match from_def.output_type(conn.from_output) {
            Some(t) => t,
            None => {
                issues.push(ValidationIssue {
                    kind: IssueKind::SourcePortOutOfRange,
                    message: format!(
                        "node {} has no output port index {}",
                        conn.from_node.0, conn.from_output
                    ),
                    connection: Some(*conn),
                });
                continue;
            }
        };
        let dst_type = match to_def.input_type(conn.to_input) {
            Some(t) => t,
            None => {
                issues.push(ValidationIssue {
                    kind: IssueKind::TargetPortOutOfRange,
                    message: format!(
                        "node {} has no input port index {}",
                        conn.to_node.0, conn.to_input
                    ),
                    connection: Some(*conn),
                });
                continue;
            }
        };

        // Precision: compare source output type against target input type.
        match check_connection(src_type, dst_type) {
            ConnectionVerdict::Ok => {}
            ConnectionVerdict::Lossy(loss) => issues.push(ValidationIssue {
                kind: IssueKind::PrecisionLoss,
                message: loss.detail.into_owned(),
                connection: Some(*conn),
            }),
            ConnectionVerdict::Incompatible(reason) => issues.push(ValidationIssue {
                kind: IssueKind::Incompatible,
                message: reason.into_owned(),
                connection: Some(*conn),
            }),
        }
    }

    // Topology: cycles block execution (7C).
    if graph.has_cycle() {
        issues.push(ValidationIssue {
            kind: IssueKind::Cycle,
            message: "graph contains a cycle".to_string(),
            connection: None,
        });
    }

    ValidationReport { issues }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_graph::graph::{NodeGraph, NodeId};
    use crate::node_graph::node::{NodeDefinition, PortDef};
    use crate::node_graph::precision::{IntKind, PortType};

    fn decimal_node(id: u64, places: u32) -> (NodeId, NodeDefinition) {
        (
            NodeId(id),
            NodeDefinition::new(
                "n",
                vec![PortDef::new("in", PortType::Decimal(places))],
                vec![PortDef::new("out", PortType::Decimal(places))],
            ),
        )
    }

    #[test]
    fn compatible_acyclic_graph_validates_clean() {
        let mut g = NodeGraph::new();
        g.add_node(decimal_node(0, 2).0, decimal_node(0, 2).1);
        g.add_node(decimal_node(1, 2).0, decimal_node(1, 2).1);
        g.connect(Connection::new(NodeId(0), 0, NodeId(1), 0));
        let report = validate(&g);
        assert!(
            report.is_ok(),
            "expected no issues, got {:?}",
            report.issues
        );
    }

    #[test]
    fn incompatible_connection_is_reported() {
        // Integer output -> decimal input is a cross-category mismatch.
        let mut g = NodeGraph::new();
        g.add_node(
            NodeId(0),
            NodeDefinition::new(
                "src",
                vec![],
                vec![PortDef::new("out", PortType::Integer(IntKind::U64))],
            ),
        );
        g.add_node(decimal_node(1, 2).0, decimal_node(1, 2).1);
        g.connect(Connection::new(NodeId(0), 0, NodeId(1), 0));
        let report = validate(&g);
        assert!(
            report
                .issues()
                .iter()
                .any(|i| i.kind == IssueKind::Incompatible),
            "expected an Incompatible issue, got {:?}",
            report.issues
        );
    }

    #[test]
    fn lossy_connection_is_reported_separately_from_incompatible() {
        // decimal(8) -> decimal(2) loses precision but is connectable.
        let mut g = NodeGraph::new();
        g.add_node(decimal_node(0, 8).0, decimal_node(0, 8).1);
        g.add_node(decimal_node(1, 2).0, decimal_node(1, 2).1);
        g.connect(Connection::new(NodeId(0), 0, NodeId(1), 0));
        let report = validate(&g);
        assert!(
            report
                .issues()
                .iter()
                .any(|i| i.kind == IssueKind::PrecisionLoss),
            "expected a PrecisionLoss issue, got {:?}",
            report.issues
        );
        assert!(
            !report
                .issues()
                .iter()
                .any(|i| i.kind == IssueKind::Incompatible),
            "decimal(8)->decimal(2) must not be Incompatible"
        );
    }

    #[test]
    fn out_of_range_target_port_is_reported() {
        let mut g = NodeGraph::new();
        g.add_node(decimal_node(0, 2).0, decimal_node(0, 2).1);
        g.add_node(decimal_node(1, 2).0, decimal_node(1, 2).1);
        // Port index 5 does not exist on node 1.
        g.connect(Connection::new(NodeId(0), 0, NodeId(1), 5));
        let report = validate(&g);
        assert!(
            report
                .issues()
                .iter()
                .any(|i| i.kind == IssueKind::TargetPortOutOfRange),
            "expected TargetPortOutOfRange, got {:?}",
            report.issues
        );
    }

    #[test]
    fn cycle_is_reported() {
        let mut g = NodeGraph::new();
        for id in 0..2 {
            g.add_node(decimal_node(id, 2).0, decimal_node(id, 2).1);
        }
        g.connect(Connection::new(NodeId(0), 0, NodeId(1), 0)); // A -> B
        g.connect(Connection::new(NodeId(1), 0, NodeId(0), 0)); // B -> A (cycle)
        let report = validate(&g);
        assert!(
            report.issues().iter().any(|i| i.kind == IssueKind::Cycle),
            "expected a Cycle issue, got {:?}",
            report.issues
        );
    }
}
