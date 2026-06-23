// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: AGPL-3.0-only

//! Graph topology for the node editor: nodes and directed connections, with
//! cycle detection. Execution (7C) requires an acyclic graph, so the model
//! surfaces cycle-creating connections for rejection.

use std::collections::{BTreeMap, BTreeSet};

/// Opaque node identifier. Newtype per IA convention (no raw integers in APIs).
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct NodeId(pub u64);

/// Directed graph of nodes and their output-to-input connections.
///
/// Topology only — ports and port types live in the precision model. A
/// connection from node `a`'s output to node `b`'s input is an edge `a -> b`.
#[derive(Clone, Debug, Default)]
pub struct NodeGraph {
    /// For each node, the set of nodes its outputs feed.
    forward: BTreeMap<NodeId, BTreeSet<NodeId>>,
}

impl NodeGraph {
    /// Create an empty graph.
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a directed edge `from -> to`.
    pub fn add_edge(&mut self, from: NodeId, to: NodeId) {
        self.forward.entry(from).or_default().insert(to);
        // Ensure both endpoints appear in the graph even with no outgoing edges.
        self.forward.entry(to).or_default();
    }

    /// Whether the graph currently contains any cycle.
    pub fn has_cycle(&self) -> bool {
        // A cycle exists iff some edge `u -> v` is a self-loop, or `v` can
        // already reach `u` through a non-empty path.
        self.forward
            .iter()
            .any(|(&u, neighbors)| neighbors.iter().any(|&v| u == v || self.reaches(v, u)))
    }

    /// Whether adding edge `from -> to` would introduce a cycle.
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

    #[test]
    fn edge_into_unrelated_node_does_not_create_cycle() {
        let g = NodeGraph::new();
        // Empty graph: A -> B cannot create a cycle.
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
        g.add_edge(NodeId(0), NodeId(1)); // A -> B
        g.add_edge(NodeId(1), NodeId(2)); // B -> C
        assert!(!g.has_cycle());
        assert!(g.would_create_cycle(NodeId(2), NodeId(0))); // C -> A closes it
    }

    #[test]
    fn chaining_forward_does_not_create_cycle() {
        // A -> B -> C: adding A -> C is a forward edge, no cycle.
        let mut g = NodeGraph::new();
        g.add_edge(NodeId(0), NodeId(1));
        g.add_edge(NodeId(1), NodeId(2));
        assert!(!g.would_create_cycle(NodeId(0), NodeId(2)));
    }

    #[test]
    fn cycle_already_in_graph_is_detected() {
        let mut g = NodeGraph::new();
        g.add_edge(NodeId(0), NodeId(1));
        g.add_edge(NodeId(1), NodeId(2));
        g.add_edge(NodeId(2), NodeId(0)); // closes A -> B -> C -> A
        assert!(g.has_cycle());
    }

    #[test]
    fn dag_has_no_cycle() {
        let mut g = NodeGraph::new();
        // Diamond: A -> {B, C} -> D
        g.add_edge(NodeId(0), NodeId(1));
        g.add_edge(NodeId(0), NodeId(2));
        g.add_edge(NodeId(1), NodeId(3));
        g.add_edge(NodeId(2), NodeId(3));
        assert!(!g.has_cycle());
    }
}
