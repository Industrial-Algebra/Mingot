// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: AGPL-3.0-only

//! Precision-aware node graph editor (Phase 7).
//!
//! Enable with the `node-graph` feature. The headless data model lives here;
//! the visual editor (7B) will be added under `components` as that work lands.

pub mod components;
pub mod connection;
pub mod graph;
pub mod layout;
pub mod node;
pub mod precision;
pub mod serialize;
pub mod validate;

pub use components::node_canvas::{NodeCanvas, PendingConnection};
pub use components::node_component::Node;
pub use components::node_connection::{ConnectionStyle, NodeConnection};
pub use components::node_port::{NodePort, PortSide};

pub use connection::Connection;
pub use graph::{NodeGraph, NodeId};
pub use layout::{connection_path_d, CanvasPoint, NodeBox, NodeLayout, Viewport};
pub use node::{NodeDefinition, PortDef};
pub use precision::{check_connection, ConnectionVerdict, IntKind, PortType};
pub use serialize::{from_json, to_json, SCHEMA_VERSION};
pub use validate::{validate, IssueKind, ValidationIssue, ValidationReport};
