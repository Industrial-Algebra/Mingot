// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: AGPL-3.0-only

//! Precision-aware node graph editor (Phase 7).
//!
//! Enable with the `node-graph` feature. The headless data model lives here;
//! the visual editor (7B) will be added under `components` as that work lands.

pub mod connection;
pub mod graph;
pub mod node;
pub mod precision;
pub mod serialize;
pub mod validate;

pub use connection::Connection;
pub use graph::{NodeGraph, NodeId};
pub use node::{NodeDefinition, PortDef};
pub use precision::{check_connection, ConnectionVerdict, IntKind, PortType};
pub use serialize::{from_json, to_json, SCHEMA_VERSION};
pub use validate::{validate, IssueKind, ValidationIssue, ValidationReport};
