// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! Node definitions: the schema for a node's typed input and output ports.
//!
//! A [`NodeDefinition`] is the blueprint for a node — a title plus its input
//! and output [`PortDef`]s. Concrete nodes in a graph are instances keyed by
//! [`NodeId`] (see [`crate::node_graph::graph::NodeGraph`]). The definition
//! carries enough structure for the 7C execution layer to hang a `NodeOp`
//! trait off later without rework.

use crate::node_graph::precision::PortType;
use std::borrow::Cow;

/// A named, typed port on a node.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PortDef {
    pub name: Cow<'static, str>,
    pub port_type: PortType,
}

impl PortDef {
    /// Create a port from a name and type.
    pub fn new(name: impl Into<Cow<'static, str>>, port_type: PortType) -> Self {
        Self {
            name: name.into(),
            port_type,
        }
    }
}

/// Schema for a node: a title and its input/output ports.
///
/// Port order is significant — connections reference ports by their positional
/// index within `inputs` / `outputs`, so a definition must not be reordered
/// once connections exist against it.
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct NodeDefinition {
    pub title: Cow<'static, str>,
    pub inputs: Vec<PortDef>,
    pub outputs: Vec<PortDef>,
}

impl NodeDefinition {
    /// Create a node definition from a title and its input/output ports.
    pub fn new(
        title: impl Into<Cow<'static, str>>,
        inputs: Vec<PortDef>,
        outputs: Vec<PortDef>,
    ) -> Self {
        Self {
            title: title.into(),
            inputs,
            outputs,
        }
    }

    /// The type of the `idx`-th input port, if present.
    pub fn input_type(&self, idx: u32) -> Option<&PortType> {
        self.inputs.get(idx as usize).map(|p| &p.port_type)
    }

    /// The type of the `idx`-th output port, if present.
    pub fn output_type(&self, idx: u32) -> Option<&PortType> {
        self.outputs.get(idx as usize).map(|p| &p.port_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn port_def_new_accepts_static_and_owned_names() {
        let a = PortDef::new("in", PortType::Text);
        let b = PortDef::new(String::from("out"), PortType::Bool);
        assert_eq!(a.name, "in");
        assert_eq!(b.name, "out");
    }

    #[test]
    fn node_def_looks_up_port_types_by_index() {
        let def = NodeDefinition::new(
            "Add",
            vec![PortDef::new("a", PortType::Decimal(2))],
            vec![
                PortDef::new("sum", PortType::Decimal(2)),
                PortDef::new("carry", PortType::Bool),
            ],
        );
        assert_eq!(def.input_type(0), Some(&PortType::Decimal(2)));
        assert_eq!(def.input_type(1), None);
        assert_eq!(def.output_type(1), Some(&PortType::Bool));
        assert_eq!(def.output_type(2), None);
    }
}
