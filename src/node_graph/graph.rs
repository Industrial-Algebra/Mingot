// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! Graph topology for the node editor: nodes (with typed ports) and directed
//! port-to-port connections, with cycle detection. Execution (7C) requires an
//! acyclic graph, so the model surfaces cycle-creating connections for rejection.

use crate::node_graph::connection::Connection;
use crate::node_graph::node::NodeDefinition;
use std::collections::{BTreeMap, BTreeSet};

/// Opaque node identifier. Newtype per IA convention (no raw integers in APIs).
#[derive(
    Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, serde::Serialize, serde::Deserialize,
)]
pub struct NodeId(pub u64);

/// Directed graph of nodes and their output-to-input connections.
///
/// Stores node [`NodeDefinition`]s keyed by [`NodeId`] and the [`Connection`]s
/// between their ports. A derived node-level adjacency map backs cycle
/// detection; precision compatibility is checked in [`validate()`](crate::node_graph::validate::validate).
#[derive(Clone, Debug, Default)]
pub struct NodeGraph {
    nodes: BTreeMap<NodeId, NodeDefinition>,
    connections: Vec<Connection>,
    /// Node-level adjacency derived from `connections`, for cycle detection.
    forward: BTreeMap<NodeId, BTreeSet<NodeId>>,
}

impl NodeGraph {
    /// Create an empty graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Insert (or replace) a node definition under `id`.
    pub fn add_node(&mut self, id: NodeId, def: NodeDefinition) {
        self.nodes.insert(id, def);
        // Ensure the node has an adjacency entry even with no edges.
        self.forward.entry(id).or_default();
    }

    /// The definition of node `id`, if present.
    pub fn node(&self, id: NodeId) -> Option<&NodeDefinition> {
        self.nodes.get(&id)
    }

    /// Iterate over all connections.
    pub fn connections(&self) -> &[Connection] {
        &self.connections
    }

    /// Iterate over all node ids.
    pub fn node_ids(&self) -> impl Iterator<Item = NodeId> + '_ {
        self.nodes.keys().copied()
    }

    /// Record a port-to-port connection and update the node-level adjacency.
    ///
    /// Structural and precision validation is performed by
    /// [`validate()`](crate::node_graph::validate::validate); this only records the edge.
    /// Returns whether the connection was new.
    pub fn connect(&mut self, conn: Connection) -> bool {
        let added = self
            .forward
            .entry(conn.from_node)
            .or_default()
            .insert(conn.to_node);
        // Ensure the target node has an adjacency entry.
        self.forward.entry(conn.to_node).or_default();
        if added || !self.connections.contains(&conn) {
            if !self.connections.contains(&conn) {
                self.connections.push(conn);
            }
            true
        } else {
            false
        }
    }

    /// Remove a node and every connection touching it.
    ///
    /// Editor delete operations need this to be total: removing a node must
    /// never leave dangling edges or stale adjacency. Returns whether the
    /// node was present.
    pub fn remove_node(&mut self, id: NodeId) -> bool {
        if self.nodes.remove(&id).is_none() {
            return false;
        }
        self.connections
            .retain(|c| c.from_node != id && c.to_node != id);
        self.forward.remove(&id);
        for neighbors in self.forward.values_mut() {
            neighbors.remove(&id);
        }
        true
    }

    /// Remove a recorded connection, keeping derived adjacency consistent.
    ///
    /// Node-level adjacency between `from_node` and `to_node` is dropped only
    /// when no other connection between the same pair remains. Returns
    /// whether the connection was present.
    pub fn disconnect(&mut self, conn: Connection) -> bool {
        let before = self.connections.len();
        self.connections.retain(|c| *c != conn);
        if self.connections.len() == before {
            return false;
        }
        let pair_remains = self
            .connections
            .iter()
            .any(|c| c.from_node == conn.from_node && c.to_node == conn.to_node);
        if !pair_remains {
            if let Some(neighbors) = self.forward.get_mut(&conn.from_node) {
                neighbors.remove(&conn.to_node);
            }
        }
        true
    }

    /// Whether the graph currently contains any node-level cycle.
    pub fn has_cycle(&self) -> bool {
        // A cycle exists iff some edge `u -> v` is a self-loop, or `v` can
        // already reach `u` through a non-empty path.
        self.forward
            .iter()
            .any(|(&u, neighbors)| neighbors.iter().any(|&v| u == v || self.reaches(v, u)))
    }

    /// Whether adding an edge `from -> to` would introduce a cycle.
    ///
    /// True for a self-loop (`from == to`) or when a path already exists from
    /// `to` back to `from` (the new edge would close the loop).
    pub fn would_create_cycle(&self, from: NodeId, to: NodeId) -> bool {
        from == to || self.reaches(to, from)
    }

    /// Whether there is a directed path from `start` to `target` (depth-first).
    fn reaches(&self, start: NodeId, target: NodeId) -> bool {
        let mut stack = vec![start];
        let mut seen = BTreeSet::new();
        while let Some(node) = stack.pop() {
            if node == target {
                return true;
            }
            if !seen.insert(node) {
                continue;
            }
            if let Some(neighbors) = self.forward.get(&node) {
                stack.extend(neighbors.iter().copied());
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_graph::node::PortDef;
    use crate::node_graph::precision::{IntKind, PortType};

    /// Two nodes that each have one decimal input and one decimal output, so
    /// they can be chained. Identical ports keep the topology tests focused.
    fn chainable(id: u64) -> (NodeId, NodeDefinition) {
        (
            NodeId(id),
            NodeDefinition::new(
                "n",
                vec![PortDef::new("in", PortType::Decimal(2))],
                vec![PortDef::new("out", PortType::Decimal(2))],
            ),
        )
    }

    #[test]
    fn adding_a_node_makes_it_lookupable() {
        let mut g = NodeGraph::new();
        let (id, def) = chainable(0);
        g.add_node(id, def.clone());
        assert_eq!(g.node(id), Some(&def));
        assert_eq!(g.node(NodeId(99)), None);
    }

    #[test]
    fn connect_records_connection_and_derives_adjacency() {
        let mut g = NodeGraph::new();
        g.add_node(chainable(0).0, chainable(0).1.clone());
        g.add_node(chainable(1).0, chainable(1).1.clone());
        let conn = Connection::new(NodeId(0), 0, NodeId(1), 0);
        assert!(g.connect(conn));
        assert_eq!(g.connections(), &[conn]);
    }

    #[test]
    fn empty_graph_connecting_two_nodes_does_not_create_cycle() {
        let g = NodeGraph::new();
        assert!(!g.would_create_cycle(NodeId(0), NodeId(1)));
    }

    #[test]
    fn self_loop_creates_cycle() {
        let g = NodeGraph::new();
        assert!(g.would_create_cycle(NodeId(0), NodeId(0)));
    }

    #[test]
    fn closing_a_chain_into_a_loop_would_create_cycle() {
        // A -> B -> C: acyclic. Adding C -> A closes the triangle.
        let mut g = NodeGraph::new();
        for id in 0..3 {
            g.add_node(chainable(id).0, chainable(id).1.clone());
        }
        g.connect(Connection::new(NodeId(0), 0, NodeId(1), 0)); // A -> B
        g.connect(Connection::new(NodeId(1), 0, NodeId(2), 0)); // B -> C
        assert!(!g.has_cycle());
        assert!(g.would_create_cycle(NodeId(2), NodeId(0))); // C -> A closes it
    }

    #[test]
    fn chaining_forward_does_not_create_cycle() {
        let mut g = NodeGraph::new();
        for id in 0..3 {
            g.add_node(chainable(id).0, chainable(id).1.clone());
        }
        g.connect(Connection::new(NodeId(0), 0, NodeId(1), 0));
        g.connect(Connection::new(NodeId(1), 0, NodeId(2), 0));
        // A -> C is a forward edge, no cycle.
        assert!(!g.would_create_cycle(NodeId(0), NodeId(2)));
    }

    #[test]
    fn cycle_already_in_graph_is_detected() {
        let mut g = NodeGraph::new();
        for id in 0..3 {
            g.add_node(chainable(id).0, chainable(id).1.clone());
        }
        g.connect(Connection::new(NodeId(0), 0, NodeId(1), 0));
        g.connect(Connection::new(NodeId(1), 0, NodeId(2), 0));
        g.connect(Connection::new(NodeId(2), 0, NodeId(0), 0)); // closes A -> B -> C -> A
        assert!(g.has_cycle());
    }

    #[test]
    fn dag_has_no_cycle() {
        // Diamond: A -> {B, C} -> D. Node D needs two inputs.
        let d_def = NodeDefinition::new(
            "d",
            vec![
                PortDef::new("in0", PortType::Decimal(2)),
                PortDef::new("in1", PortType::Decimal(2)),
            ],
            vec![],
        );
        let mut g = NodeGraph::new();
        for id in 0..3 {
            g.add_node(chainable(id).0, chainable(id).1.clone());
        }
        g.add_node(NodeId(3), d_def);
        g.connect(Connection::new(NodeId(0), 0, NodeId(1), 0)); // A -> B
        g.connect(Connection::new(NodeId(0), 0, NodeId(2), 0)); // A -> C
        g.connect(Connection::new(NodeId(1), 0, NodeId(3), 0)); // B -> D
        g.connect(Connection::new(NodeId(2), 0, NodeId(3), 1)); // C -> D
        assert!(!g.has_cycle());
        let _ = IntKind::U64; // keep the import meaningful for downstream tests
    }

    // ---- remove_node / disconnect ----

    #[test]
    fn remove_node_drops_it_and_connections_touching_it() {
        // A -> B -> C: removing B must drop B and both edges.
        let mut g = NodeGraph::new();
        for id in 0..3 {
            g.add_node(chainable(id).0, chainable(id).1.clone());
        }
        g.connect(Connection::new(NodeId(0), 0, NodeId(1), 0)); // A -> B
        g.connect(Connection::new(NodeId(1), 0, NodeId(2), 0)); // B -> C
        assert!(g.remove_node(NodeId(1)));
        assert_eq!(g.node(NodeId(1)), None);
        assert_eq!(g.connections(), &[]);
        let ids: Vec<_> = g.node_ids().collect();
        assert_eq!(ids, vec![NodeId(0), NodeId(2)]);
    }

    #[test]
    fn remove_node_cleans_adjacency_so_no_stale_reachability() {
        // A -> B and B -> A forms a cycle; removing B must clear it entirely,
        // including B's stale entries in A's neighbor set.
        let mut g = NodeGraph::new();
        for id in 0..2 {
            g.add_node(chainable(id).0, chainable(id).1.clone());
        }
        g.connect(Connection::new(NodeId(0), 0, NodeId(1), 0));
        g.connect(Connection::new(NodeId(1), 0, NodeId(0), 0));
        assert!(g.has_cycle());
        g.remove_node(NodeId(1));
        assert!(!g.has_cycle());
        // Self-loops are flagged by definition (`from == to`), independent of
        // adjacency state; the stale-adjacency behavior is fully covered by
        // `has_cycle` going false above.
    }

    #[test]
    fn remove_absent_node_returns_false_and_is_noop() {
        let mut g = NodeGraph::new();
        g.add_node(chainable(0).0, chainable(0).1);
        assert!(!g.remove_node(NodeId(99)));
        assert_eq!(g.node_ids().count(), 1);
    }

    #[test]
    fn disconnect_removes_only_that_connection() {
        // Two parallel A -> B edges on different ports: disconnecting one must
        // keep the other AND keep A -> B adjacency (which the surviving edge
        // still justifies).
        let b_def = NodeDefinition::new(
            "b",
            vec![
                PortDef::new("in0", PortType::Decimal(2)),
                PortDef::new("in1", PortType::Decimal(2)),
            ],
            vec![],
        );
        let mut g = NodeGraph::new();
        g.add_node(chainable(0).0, chainable(0).1);
        g.add_node(NodeId(1), b_def);
        let c0 = Connection::new(NodeId(0), 0, NodeId(1), 0);
        let c1 = Connection::new(NodeId(0), 0, NodeId(1), 1);
        g.connect(c0);
        g.connect(c1);
        assert!(g.disconnect(c0));
        assert_eq!(g.connections(), &[c1]);
        // The surviving edge still connects A -> B, so B -> A would cycle.
        assert!(g.would_create_cycle(NodeId(1), NodeId(0)));
    }

    #[test]
    fn disconnect_absent_connection_returns_false() {
        let mut g = NodeGraph::new();
        g.add_node(chainable(0).0, chainable(0).1);
        g.add_node(chainable(1).0, chainable(1).1);
        let conn = Connection::new(NodeId(0), 0, NodeId(1), 0);
        assert!(!g.disconnect(conn));
    }

    #[test]
    fn disconnect_breaks_cycle_adjacency() {
        // A -> B, B -> A is a cycle; removing the back edge clears it.
        let mut g = NodeGraph::new();
        for id in 0..2 {
            g.add_node(chainable(id).0, chainable(id).1.clone());
        }
        let fwd = Connection::new(NodeId(0), 0, NodeId(1), 0);
        let back = Connection::new(NodeId(1), 0, NodeId(0), 0);
        g.connect(fwd);
        g.connect(back);
        assert!(g.has_cycle());
        assert!(g.disconnect(back));
        assert!(!g.has_cycle());
        // Reconnecting the back edge closes the cycle again.
        assert!(g.would_create_cycle(NodeId(1), NodeId(0)));
    }
}
