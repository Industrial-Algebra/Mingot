// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: AGPL-3.0-only

//! Directed connections between node ports.
//!
//! A [`Connection`] always flows from one node's output port into another
//! node's input port — direction is fixed by construction, so an
//! output-to-output or input-to-input link is impossible at the type level.
//! Ports are referenced positionally (index into the node's `outputs` / `inputs`).

use crate::node_graph::graph::NodeId;

/// A directed wire from `(from_node, from_output)` to `(to_node, to_input)`.
///
/// Port indices are positional within the source/target [`NodeDefinition`]'s
/// `outputs` / `inputs` vectors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Connection {
    pub from_node: NodeId,
    pub from_output: u32,
    pub to_node: NodeId,
    pub to_input: u32,
}

impl Connection {
    /// Create a new output-to-input connection.
    pub fn new(from_node: NodeId, from_output: u32, to_node: NodeId, to_input: u32) -> Self {
        Self {
            from_node,
            from_output,
            to_node,
            to_input,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connection_new_records_endpoints() {
        let c = Connection::new(NodeId(1), 0, NodeId(2), 1);
        assert_eq!(c.from_node, NodeId(1));
        assert_eq!(c.from_output, 0);
        assert_eq!(c.to_node, NodeId(2));
        assert_eq!(c.to_input, 1);
    }
}
