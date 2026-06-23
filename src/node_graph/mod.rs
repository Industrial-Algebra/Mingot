// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: AGPL-3.0-only

//! Precision-aware node graph editor (Phase 7).
//!
//! Enable with the `node-graph` feature. The headless data model lives here;
//! the visual editor (7B) will be added under `components` as that work lands.

pub mod graph;
pub mod precision;

pub use graph::{NodeGraph, NodeId};
pub use precision::{check_connection, ConnectionVerdict, IntKind, PortType};
