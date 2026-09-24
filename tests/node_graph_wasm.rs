// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! In-browser tests for the node-graph editor components.
//!
//! These run only on wasm32 (empty no-op on native builds), so the standard
//! `cargo test` suite is unaffected. Run them locally with a browser:
//!
//! ```text
//! rustup target add wasm32-unknown-unknown
//! cargo install wasm-bindgen-cli --version 0.2.125
//! cargo test --target wasm32-unknown-unknown --features node-graph \
//!     --test node_graph_wasm --no-run
//! wasm-bindgen-test-runner $(ls -t target/wasm32-unknown-unknown/debug/deps/node_graph_wasm-*.wasm | head -1)
//! ```
//!
//! Notes:
//! - `mount_to` returns an `UnmountHandle` whose `Drop` unmounts the view;
//!   tests call `.forget()` (as `mount_to_body` does) so the mount outlives
//!   the assertions, and remove the host element for cleanup.
//! - Synthetic events are `MouseEvent`s dispatched with a pointer event
//!   *type* — listeners fire by type, and `client_x`/`buttons` are readable.

#![cfg(all(feature = "node-graph", target_arch = "wasm32"))]

use std::collections::BTreeMap;

use leptos::mount::mount_to;
use leptos::prelude::*;
use mingot::node_graph::{
    CanvasPoint, Connection, NodeCanvas, NodeDefinition, NodeGraph, NodeId, PortDef, PortType,
    Viewport,
};
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

/// Two chainable nodes (A -> B) with one wire.
fn demo_graph() -> NodeGraph {
    let mut g = NodeGraph::new();
    g.add_node(
        NodeId(0),
        NodeDefinition::new("a", vec![], vec![PortDef::new("out", PortType::Decimal(2))]),
    );
    g.add_node(
        NodeId(1),
        NodeDefinition::new("b", vec![PortDef::new("in", PortType::Decimal(2))], vec![]),
    );
    g.connect(Connection::new(NodeId(0), 0, NodeId(1), 0));
    g
}

/// Everything a test needs to drive and observe a mounted canvas.
struct Harness {
    positions: ReadSignal<BTreeMap<NodeId, CanvasPoint>>,
    selected: ReadSignal<BTreeMap<NodeId, bool>>,
    viewport: ReadSignal<Viewport>,
    moves: StoredValue<Vec<(NodeId, CanvasPoint)>>,
    host: web_sys::Element,
}

/// Mount a canvas with demo-graph state and every echo-back callback wired to
/// the parent-side signals, exactly as a real consumer would.
fn mount_harness() -> Harness {
    let (graph, _set_graph) = signal(demo_graph());
    let mut pos = BTreeMap::new();
    pos.insert(NodeId(0), CanvasPoint::new(0.0, 0.0));
    pos.insert(NodeId(1), CanvasPoint::new(300.0, 100.0));
    let (positions, set_positions) = signal(pos);
    let (selected, set_selected) = signal(BTreeMap::<NodeId, bool>::new());
    let (viewport, set_viewport) = signal(Viewport::default());
    let moves: StoredValue<Vec<(NodeId, CanvasPoint)>> = StoredValue::new(Vec::new());

    let moves_rec = moves;
    let on_node_move = Callback::new(move |(id, pt): (NodeId, CanvasPoint)| {
        moves_rec.update_value(|v| v.push((id, pt)));
        set_positions.update(|p| {
            p.insert(id, pt);
        });
    });
    let on_node_select = Callback::new(move |(id, _): (NodeId, bool)| {
        // Exclusive selection, like the demo page.
        set_selected.update(|s| {
            s.retain(|_, v| {
                *v = false;
                true
            });
            s.insert(id, true);
        });
    });
    let on_viewport_change = Callback::new(move |vp: Viewport| {
        set_viewport.set(vp);
    });

    let doc = document();
    let host = doc.create_element("div").unwrap();
    doc.body().unwrap().append_child(&host).unwrap();

    let host_el: web_sys::HtmlElement = host.clone().unchecked_into();
    // Forget the UnmountHandle so the mount survives the assertions; the
    // host element is removed manually at the end of each test.
    mount_to(host_el, move || {
        view! {
            <NodeCanvas
                graph=graph
                positions=positions
                selected=selected
                viewport=viewport
                on_node_move=on_node_move
                on_node_select=on_node_select
                on_viewport_change=on_viewport_change
            />
        }
    })
    .forget();

    Harness {
        positions,
        selected,
        viewport,
        moves,
        host,
    }
}

/// Dispatch a synthetic (bubbling, primary-button) pointer event of `kind`
/// ("pointerdown" | "pointermove" | "pointerup") at `target`.
fn pointer_event(target: &web_sys::Element, kind: &str, x: i32, y: i32) {
    let init = web_sys::MouseEventInit::new();
    init.set_bubbles(true);
    init.set_client_x(x);
    init.set_client_y(y);
    init.set_buttons(1);
    let ev = web_sys::MouseEvent::new_with_mouse_event_init_dict(kind, &init)
        .expect("synthetic pointer event");
    let ev: web_sys::Event = ev.into();
    let _ = target.dispatch_event(&ev);
}

fn node_body_rect(h: &Harness) -> web_sys::Element {
    h.host
        .query_selector("svg.mingot-node-canvas rect")
        .expect("query failed")
        .expect("node body rect rendered")
}

fn node_title_text(h: &Harness) -> web_sys::Element {
    h.host
        .query_selector("svg.mingot-node-canvas text")
        .expect("query failed")
        .expect("node title rendered")
}

fn svg_of(h: &Harness) -> web_sys::Element {
    h.host
        .query_selector("svg.mingot-node-canvas")
        .expect("query failed")
        .expect("canvas svg mounted")
}

/// Smoke: the canvas mounts and renders surface, wire, and ports.
#[wasm_bindgen_test]
fn node_canvas_mounts_with_nodes_and_wires() {
    let h = mount_harness();

    let svg = svg_of(&h);
    assert!(
        svg.query_selector("path").expect("q").is_some(),
        "expected a wire path; inner={}",
        h.host.inner_html()
    );
    assert!(
        svg.query_selector("circle").expect("q").is_some(),
        "expected a port circle"
    );

    h.host.remove();
}

/// P1 regression: selecting a node and continuously dragging it must move it,
/// even though the selection and each echo-back move re-run the canvas render
/// and recreate the Node components (drag state lives on the canvas).
#[wasm_bindgen_test]
fn select_then_drag_moves_the_node_across_rerenders() {
    let h = mount_harness();
    let rect = node_body_rect(&h);

    // Press on the node body, then two moves: the first move's echo-back
    // (selection + position update) re-renders and recreates the Node
    // components — the drag must survive and accumulate the full delta.
    pointer_event(&rect, "pointerdown", 100, 100);
    pointer_event(&rect, "pointermove", 120, 105);
    pointer_event(&rect, "pointermove", 140, 110);
    pointer_event(&rect, "pointerup", 140, 110);

    assert_eq!(
        h.selected.get().get(&NodeId(0)),
        Some(&true),
        "press should select the node"
    );
    assert_eq!(
        h.positions.get().get(&NodeId(0)).copied(),
        Some(CanvasPoint::new(40.0, 10.0)),
        "node A should move by the full (40, 10) screen delta (zoom 1)"
    );
    let all_moves = h.moves.get_value();
    assert!(
        all_moves
            .iter()
            .any(|(id, p)| *id == NodeId(0) && *p == CanvasPoint::new(40.0, 10.0)),
        "on_node_move should report the cumulative delta; got {all_moves:?}"
    );

    h.host.remove();
}

/// P2 regression: pressing the node title is a node drag (whole-group hit
/// area), not a background pan.
#[wasm_bindgen_test]
fn dragging_the_title_moves_the_node_and_does_not_pan() {
    let h = mount_harness();
    let title = node_title_text(&h);

    pointer_event(&title, "pointerdown", 100, 100);
    pointer_event(&title, "pointermove", 140, 110);
    pointer_event(&title, "pointerup", 140, 110);

    let vp = h.viewport.get();
    assert_eq!(
        (vp.pan.x, vp.pan.y),
        (0.0, 0.0),
        "title drag must not pan the background"
    );
    assert_eq!(
        h.positions.get().get(&NodeId(0)).copied(),
        Some(CanvasPoint::new(40.0, 10.0)),
        "title drag should move the node"
    );

    h.host.remove();
}

/// P2 regression: pointer-pressing a node focuses the canvas so keyboard
/// commands (delete, nudge) work without an extra Tab.
#[wasm_bindgen_test]
fn selecting_a_node_focuses_the_canvas() {
    let h = mount_harness();
    let rect = node_body_rect(&h);

    pointer_event(&rect, "pointerdown", 100, 100);

    let svg = svg_of(&h);
    let active_tag = document()
        .active_element()
        .map(|a| a.tag_name())
        .unwrap_or_else(|| "<none>".to_string());
    let focused = document()
        .active_element()
        .as_ref()
        .map(|a| a.is_same_node(Some(svg.unchecked_ref())))
        .unwrap_or(false);
    assert!(
        focused,
        "canvas svg should be focused after node press; active={active_tag}, \
         active={active_tag}, svg_class={}",
        svg.get_attribute("class").unwrap_or_default()
    );

    h.host.remove();
}
