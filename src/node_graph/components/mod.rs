// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: AGPL-3.0-only

//! SVG components for the node editor: port, node, connection, and canvas.
//!
//! These render the headless [`crate::node_graph`] model. They are pure
//! presentation over signals — all graph mutations flow through the model's
//! callbacks, keeping the view stateless and testable via the model.

pub mod node_canvas;
pub mod node_component;
pub mod node_connection;
pub mod node_port;

pub use node_canvas::NodeCanvas;
pub use node_component::Node;
pub use node_connection::NodeConnection;
pub use node_port::NodePort;
