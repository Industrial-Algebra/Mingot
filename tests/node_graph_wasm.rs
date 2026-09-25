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
    /// Emulates the parent echoing a wheel-driven viewport change.
    set_viewport: WriteSignal<Viewport>,
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
        set_viewport,
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

    // Press on the node body, then two moves on the (never-replaced) svg
    // root: the first move's echo-back (selection + position update)
    // re-renders and recreates the Node components — the drag must survive
    // and accumulate the full delta.
    let svg = svg_of(&h);
    pointer_event(&rect, "pointerdown", 100, 100);
    pointer_event(&svg, "pointermove", 120, 105);
    pointer_event(&svg, "pointermove", 140, 110);
    pointer_event(&svg, "pointerup", 140, 110);

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

    let svg = svg_of(&h);
    pointer_event(&title, "pointerdown", 100, 100);
    pointer_event(&svg, "pointermove", 140, 110);
    pointer_event(&svg, "pointerup", 140, 110);

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

/// P2 regression: a drag must keep tracking when the pointer outruns the
/// node and a move lands on the canvas background — the drag is handled by
/// the never-replaced svg root, not the node's own hit area.
#[wasm_bindgen_test]
fn drag_continues_when_pointer_moves_onto_background() {
    let h = mount_harness();
    let rect = node_body_rect(&h);
    let svg = svg_of(&h);

    // Press on the node body, one small move while still over it, then a
    // large move that lands well outside the node (on the background).
    pointer_event(&rect, "pointerdown", 100, 100);
    pointer_event(&rect, "pointermove", 110, 105);
    pointer_event(&svg, "pointermove", 160, 165);
    pointer_event(&svg, "pointerup", 160, 165);

    assert_eq!(
        h.positions.get().get(&NodeId(0)).copied(),
        Some(CanvasPoint::new(60.0, 65.0)),
        "drag must accumulate the full (60, 65) delta including the \
         background segment (zoom 1)"
    );

    h.host.remove();
}

/// P2 regression: a mid-drag viewport zoom must only scale *new* movement —
/// previously applied movement is not recomputed at the new zoom (the zoom
/// keeps the world point under the cursor fixed, so the node stays put).
#[wasm_bindgen_test]
fn drag_rebases_across_mid_drag_zoom() {
    let h = mount_harness();
    let rect = node_body_rect(&h);
    let svg = svg_of(&h);

    pointer_event(&rect, "pointerdown", 100, 100);
    pointer_event(&svg, "pointermove", 250, 100); // +150 at zoom 1
    assert_eq!(
        h.positions.get().get(&NodeId(0)).copied(),
        Some(CanvasPoint::new(150.0, 0.0)),
        "pre-zoom displacement is 150 at zoom 1"
    );

    // Wheel-zoom to 1.1 (emulated by the parent's viewport echo-back).
    let mut vp = h.viewport.get();
    vp.zoom = 1.1;
    h.set_viewport.set(vp);

    // One screen pixel at zoom 1.1 is 1/1.1 world units; the node must
    // advance by that — not jump backward by recomputing its whole
    // displacement at the new zoom (which would give 151/1.1 = 137.27).
    pointer_event(&svg, "pointermove", 251, 100);
    let moved = h.positions.get().get(&NodeId(0)).copied();
    let expected = 150.0 + 1.0 / 1.1;
    let got = moved.map(|p| p.x).unwrap_or(f64::NAN);
    assert!(
        (got - expected).abs() < 1e-6,
        "expected x ≈ {expected}, got {got}"
    );

    pointer_event(&svg, "pointerup", 251, 100);
    h.host.remove();
}
