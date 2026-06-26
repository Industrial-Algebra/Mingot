// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: AGPL-3.0-only

//! Serialization of node graphs to/from JSON, with a schema version for
//! forward-compatible evolution.
//!
//! Graphs are persisted as a single JSON object carrying a [`SCHEMA_VERSION`].
//! Loaders should reject or migrate versions they do not understand.

use crate::node_graph::connection::Connection;
use crate::node_graph::graph::{NodeGraph, NodeId};
use crate::node_graph::node::NodeDefinition;
use serde::de::Error as _;

/// The serialization schema version produced by this version of Mingot.
///
/// Increment when the on-disk format changes incompatibly. Loaders compare
/// against this to decide whether a document is understood.
pub const SCHEMA_VERSION: u32 = 1;

/// Versioned, serializable representation of a [`NodeGraph`]. The derived
/// `forward` adjacency map is excluded — it is reconstructed on load.
#[derive(serde::Serialize, serde::Deserialize)]
struct GraphDocument {
    schema_version: u32,
    nodes: Vec<SerializedNode>,
    connections: Vec<Connection>,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct SerializedNode {
    id: u64,
    definition: NodeDefinition,
}

/// Serialize `graph` to a JSON string.
pub fn to_json(graph: &NodeGraph) -> Result<String, serde_json::Error> {
    let nodes = graph
        .node_ids()
        .map(|id| SerializedNode {
            id: id.0,
            definition: graph.node(id).expect("id came from the graph").clone(),
        })
        .collect();
    let doc = GraphDocument {
        schema_version: SCHEMA_VERSION,
        nodes,
        connections: graph.connections().to_vec(),
    };
    serde_json::to_string_pretty(&doc)
}

/// Deserialize a graph from a JSON string.
///
/// Returns an error if the schema version is not [`SCHEMA_VERSION`] or if the
/// document is malformed.
pub fn from_json(json: &str) -> Result<NodeGraph, serde_json::Error> {
    let doc: GraphDocument = serde_json::from_str(json)?;
    if doc.schema_version != SCHEMA_VERSION {
        return Err(serde_json::Error::custom(format!(
            "unsupported node-graph schema version {} (expected {})",
            doc.schema_version, SCHEMA_VERSION
        )));
    }
    let mut graph = NodeGraph::new();
    for node in doc.nodes {
        graph.add_node(NodeId(node.id), node.definition);
    }
    for conn in doc.connections {
        graph.connect(conn);
    }
    Ok(graph)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::node_graph::connection::Connection;
    use crate::node_graph::graph::{NodeGraph, NodeId};
    use crate::node_graph::node::{NodeDefinition, PortDef};
    use crate::node_graph::precision::{IntKind, PortType};

    /// A small but non-trivial graph exercising nodes, ports, connections.
    fn sample_graph() -> NodeGraph {
        let mut g = NodeGraph::new();
        g.add_node(
            NodeId(0),
            NodeDefinition::new(
                "Source",
                vec![],
                vec![PortDef::new("value", PortType::Integer(IntKind::U64))],
            ),
        );
        g.add_node(
            NodeId(1),
            NodeDefinition::new(
                "Sink",
                vec![PortDef::new("in", PortType::Integer(IntKind::U64))],
                vec![],
            ),
        );
        g.connect(Connection::new(NodeId(0), 0, NodeId(1), 0));
        g
    }

    #[test]
    fn round_trip_preserves_graph_structure() {
        let original = sample_graph();
        let json = to_json(&original).expect("serialize");
        let restored = from_json(&json).expect("deserialize");
        assert_eq_graph(&restored, &original);
    }

    /// Compare two graphs on the properties that round-trip must preserve.
    fn assert_eq_graph(a: &NodeGraph, b: &NodeGraph) {
        // Same node set with identical definitions.
        let a_nodes: Vec<_> = a.node_ids().collect();
        let b_nodes: Vec<_> = b.node_ids().collect();
        assert_eq!(a_nodes, b_nodes, "node id sets differ");
        for id in &a_nodes {
            assert_eq!(a.node(*id), b.node(*id), "node {id:?} definition differs");
        }
        // Same connections, in order.
        assert_eq!(a.connections(), b.connections(), "connections differ");
    }

    #[test]
    fn serialized_document_carries_schema_version() {
        let json = to_json(&sample_graph()).expect("serialize");
        let doc: serde_json::Value =
            serde_json::from_str(&json).expect("document must be valid JSON");
        assert_eq!(doc["schema_version"], SCHEMA_VERSION);
    }

    #[test]
    fn future_schema_version_is_rejected() {
        // Hand-craft a document claiming a version we do not understand.
        let doc = r#"{"schema_version": 9999, "nodes": [], "connections": []}"#;
        let result = from_json(doc);
        assert!(
            result.is_err(),
            "an unknown schema version must not deserialize"
        );
    }
}
