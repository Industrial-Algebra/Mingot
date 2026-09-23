// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! In-browser smoke tests for the node-graph editor components.
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
//! Driver note: use Chrome with `chromedriver` on PATH. Snap-packaged Firefox
//! via geckodriver is flaky with the wasm-bindgen-test harness page
//! ("Failed to detect test as having been run").

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

/// Smoke: NodeCanvas mounts and renders its surface, wires, and ports.
#[wasm_bindgen_test]
fn node_canvas_mounts_with_nodes_and_wires() {
    let (graph, _set_graph) = signal(demo_graph());
    let mut positions = BTreeMap::new();
    positions.insert(NodeId(0), CanvasPoint::new(0.0, 0.0));
    positions.insert(NodeId(1), CanvasPoint::new(300.0, 100.0));
    let (positions, _set_positions) = signal(positions);
    let (selected, _set_selected) = signal(BTreeMap::<NodeId, bool>::new());
    let (viewport, _set_viewport) = signal(Viewport::default());

    let doc = document();
    let host = doc.create_element("div").unwrap();
    doc.body().unwrap().append_child(&host).unwrap();

    let host_el: web_sys::HtmlElement = host.clone().unchecked_into();
    mount_to(host_el, move || {
        view! {
            <NodeCanvas
                graph=graph
                positions=positions
                selected=selected
                viewport=viewport
            />
        }
    });

    // The canvas surface itself mounted (queried from our own host element,
    // immune to harness page structure).
    let svg = host
        .query_selector("svg.mingot-node-canvas")
        .expect("query failed");
    assert!(
        svg.is_some(),
        "canvas svg should mount; inner={}",
        host.inner_html()
    );

    // The wire rendered as a path, and ports rendered as circles.
    let svg_el = svg.expect("checked above");
    let wire = svg_el.query_selector("path").expect("query failed");
    assert!(wire.is_some(), "expected a wire path");
    let port = svg_el.query_selector("circle").expect("query failed");
    assert!(port.is_some(), "expected a port circle");

    host.remove();
}
