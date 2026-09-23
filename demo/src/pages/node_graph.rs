// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! Node graph editor demo: a small precision pipeline (Constant → Add →
//! Result) wired through every NodeCanvas callback. Pan with background drag,
//! zoom with the wheel, drag from an output port and drop on an input port to
//! connect, click a node to select, `Delete` removes it, arrow keys nudge.

use leptos::prelude::*;
use mingot::prelude::*;
use std::collections::BTreeMap;

#[component]
pub fn NodeGraphPage() -> impl IntoView {
    // -- Model state (all owned here; NodeCanvas reports intent back) ------
    let (graph, set_graph) = signal(NodeGraph::new());
    let (positions, set_positions) = signal(BTreeMap::<NodeId, CanvasPoint>::new());
    let (selected, set_selected) = signal(BTreeMap::<NodeId, bool>::new());
    let (viewport, set_viewport) = signal(Viewport::default());
    let (last_report, set_last_report) = signal("No edits yet.".to_string());

    // Seed the demo graph: c0 (out) -> a0 (in0), c1 (out) -> a0 (in1),
    // a0 (out) -> r0 (in0). Decimal(2) everywhere, all connections Ok.
    {
        let mut g = NodeGraph::new();
        g.add_node(
            NodeId(0),
            NodeDefinition::new(
                "Constant 10.00",
                vec![],
                vec![PortDef::new("value", PortType::Decimal(2))],
            ),
        );
        g.add_node(
            NodeId(1),
            NodeDefinition::new(
                "Constant 2.50",
                vec![],
                vec![PortDef::new("value", PortType::Decimal(2))],
            ),
        );
        g.add_node(
            NodeId(2),
            NodeDefinition::new(
                "Add",
                vec![
                    PortDef::new("lhs", PortType::Decimal(2)),
                    PortDef::new("rhs", PortType::Decimal(2)),
                ],
                vec![PortDef::new("sum", PortType::Decimal(2))],
            ),
        );
        g.add_node(
            NodeId(3),
            NodeDefinition::new(
                "Result",
                vec![PortDef::new("in", PortType::Decimal(2))],
                vec![],
            ),
        );
        g.connect(Connection::new(NodeId(0), 0, NodeId(2), 0));
        g.connect(Connection::new(NodeId(1), 0, NodeId(2), 1));
        g.connect(Connection::new(NodeId(2), 0, NodeId(3), 0));
        set_graph.set(g);

        let mut pos = BTreeMap::new();
        pos.insert(NodeId(0), CanvasPoint::new(40.0, 60.0));
        pos.insert(NodeId(1), CanvasPoint::new(40.0, 220.0));
        pos.insert(NodeId(2), CanvasPoint::new(320.0, 130.0));
        pos.insert(NodeId(3), CanvasPoint::new(600.0, 130.0));
        set_positions.set(pos);
    }

    // -- Echo-back handlers ------------------------------------------------
    let on_node_move = Callback::new(move |(id, pt): (NodeId, CanvasPoint)| {
        set_positions.update(|p| {
            p.insert(id, pt);
        });
    });
    let on_connect = Callback::new(move |conn: Connection| {
        set_last_report.set(format!(
            "Connect {}:{} -> {}:{}",
            conn.from_node.0, conn.from_output, conn.to_node.0, conn.to_input
        ));
        set_graph.update(|g| {
            g.connect(conn);
        });
    });
    let on_node_select = Callback::new(move |(id, _): (NodeId, bool)| {
        // Exclusive selection: clicking a node clears all others.
        set_selected.update(|s| {
            s.retain(|_, v| {
                *v = false;
                true
            });
            s.insert(id, true);
        });
    });
    let on_node_delete = Callback::new(move |id: NodeId| {
        set_last_report.set(format!("Delete node {}", id.0));
        set_graph.update(|g| {
            g.remove_node(id);
        });
        set_positions.update(|p| {
            p.remove(&id);
        });
        set_selected.update(|s| {
            s.remove(&id);
        });
    });
    let on_viewport_change = Callback::new(move |vp: Viewport| {
        set_viewport.set(vp);
    });

    let report = move || {
        let g = graph.get();
        format!(
            "{} | {} nodes, {} connections | zoom {:.2}",
            last_report.get(),
            g.node_ids().count(),
            g.connections().len(),
            viewport.get().zoom
        )
    };

    view! {
        <div class="demo-section">
            <h2>"Node Graph Editor"</h2>
            <p>
                "Pan with background drag, zoom with the wheel, drag output → input to connect, "
                "click a node to select, Delete to remove, arrow keys to nudge (Shift = fine)."
            </p>
            <p class="demo-hint">{report}</p>
            <div
                class="node-graph-frame"
                style="height: 420px; border: 1px solid #d1d5db; border-radius: 8px; overflow: hidden;"
            >
                <NodeCanvas
                    graph=graph
                    positions=positions
                    selected=selected
                    viewport=viewport
                    on_node_move=on_node_move
                    on_connect=on_connect
                    on_node_select=on_node_select
                    on_node_delete=on_node_delete
                    on_viewport_change=on_viewport_change
                />
            </div>
        </div>
    }
}
