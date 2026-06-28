// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: AGPL-3.0-only

//! The node editor canvas: an infinite pannable, zoomable SVG surface that
//! renders a [`NodeGraph`] and lets the user move nodes and draw connections.

use super::node_component::Node;
use super::node_connection::{ConnectionStyle, NodeConnection};
use super::node_port::PortSide;
use crate::node_graph::connection::Connection;
use crate::node_graph::graph::{NodeGraph, NodeId};
use crate::node_graph::layout::{CanvasPoint, NodeBox, Viewport};
use crate::utils::StyleBuilder;
use leptos::prelude::*;
use std::collections::BTreeMap;

const NODE_WIDTH: f64 = 180.0;
const NODE_PORT_PITCH: f64 = 26.0;
const NODE_FIRST_PORT_OFFSET: f64 = 40.0;

/// A pending connection being drawn by the user (drag in progress).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PendingConnection {
    pub from_node: NodeId,
    pub from_output: u32,
    pub from_point: CanvasPoint,
    pub to_point: CanvasPoint,
}

fn node_box(origin: CanvasPoint) -> NodeBox {
    NodeBox {
        origin,
        width: NODE_WIDTH,
        port_pitch: NODE_PORT_PITCH,
        first_port_offset: NODE_FIRST_PORT_OFFSET,
    }
}

/// Render the node editor canvas. `graph`, `positions`, `selected`, and
/// `viewport` are reactive signals; mutations surface through the callbacks.
#[component]
pub fn NodeCanvas(
    graph: ReadSignal<NodeGraph>,
    positions: ReadSignal<BTreeMap<NodeId, CanvasPoint>>,
    selected: ReadSignal<BTreeMap<NodeId, bool>>,
    viewport: ReadSignal<Viewport>,
    #[prop(default = (0.1, 4.0))] zoom_range: (f64, f64),
    #[prop(optional)] on_node_move: Option<Callback<(NodeId, CanvasPoint)>>,
    #[prop(optional)] on_connect: Option<Callback<Connection>>,
) -> impl IntoView {
    let _ = (on_connect, zoom_range);
    let (pending, set_pending) = signal::<Option<PendingConnection>>(None);

    let mut canvas_style = StyleBuilder::new();
    canvas_style
        .add("width", "100%")
        .add("height", "100%")
        .add("background-color", "var(--mingot-canvas-bg, #f9fafb)")
        .add("cursor", "grab");
    let canvas_style = canvas_style.build();

    view! {
        <svg class="mingot-node-canvas" style=canvas_style>
            {move || {
                let vp = viewport.get();
                let graph = graph.get();
                let positions = positions.get();
                let selected = selected.get();
                let pending = pending.get();

                let wires: Vec<_> = graph
                    .connections()
                    .iter()
                    .filter_map(|conn| {
                        let from_origin = *positions.get(&conn.from_node)?;
                        let to_origin = *positions.get(&conn.to_node)?;
                        Some((
                            node_box(from_origin).output_port_center(conn.from_output as usize),
                            node_box(to_origin).input_port_center(conn.to_input as usize),
                        ))
                    })
                    .collect();
                let pending_wire = pending.map(|p| (p.from_point, p.to_point));

                let nodes_view = positions
                    .iter()
                    .filter_map(|(id, origin)| {
                        let def = graph.node(*id)?.clone();
                        let is_selected = *selected.get(id).unwrap_or(&false);
                        let node_id = *id;
                        let origin_val = *origin;
                        let on_move_cb = on_node_move.map(move |cb| {
                            let id = node_id;
                            Callback::new(move |pt: CanvasPoint| cb.run((id, pt)))
                        });
                        let on_port_grab_cb = Callback::new(move |(side, idx): (PortSide, u32)| {
                            if side == PortSide::Output {
                                let from_point =
                                    node_box(origin_val).output_port_center(idx as usize);
                                set_pending.set(Some(PendingConnection {
                                    from_node: node_id,
                                    from_output: idx,
                                    from_point,
                                    to_point: from_point,
                                }));
                            }
                        });
                        Some(view! {
                            <Node
                                definition=def
                                origin=origin_val
                                selected=is_selected
                                on_move=on_move_cb.unwrap_or_else(|| Callback::new(move |_| {}))
                                on_port_grab=on_port_grab_cb
                            />
                        })
                    })
                    .collect_view();

                let transform = format!(
                    "translate({} {}) scale({})",
                    -vp.pan.x * vp.zoom,
                    -vp.pan.y * vp.zoom,
                    vp.zoom,
                );

                view! {
                    <g transform=transform>
                        {wires
                            .into_iter()
                            .map(|(from, to)| view! { <NodeConnection from=from to=to style=ConnectionStyle::Bezier /> })
                            .collect_view()}
                        {pending_wire
                            .map(|(from, to)| view! {
                                <NodeConnection from=from to=to style=ConnectionStyle::Bezier highlighted=true />
                            })}
                        {nodes_view}
                    </g>
                }
            }}
        </svg>
    }
}
