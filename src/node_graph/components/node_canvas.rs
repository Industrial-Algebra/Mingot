// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! The node editor canvas: an infinite pannable, zoomable SVG surface that
//! renders a [`NodeGraph`] and lets the user move nodes and draw connections.
//!
//! Interaction model (echo-back callbacks, same pattern as `on_node_move`):
//! the canvas never owns graph/position/viewport state — it reports intent
//! through callbacks and the parent writes back into the signals it passed
//! in. Pointer events: drag empty background to pan, wheel to zoom about the
//! cursor, drag from an output port and release over an input port to
//! connect. Keyboard: focus the canvas, `Delete`/`Backspace` removes
//! selected nodes, arrow keys nudge selected nodes (10px, `Shift` for 1px).

use super::node_component::Node;
use super::node_connection::{ConnectionStyle, NodeConnection};
use super::node_port::PortSide;
use crate::node_graph::connection::Connection;
use crate::node_graph::graph::{NodeGraph, NodeId};
use crate::node_graph::layout::{
    hit_test_input_port, CanvasPoint, NodeBox, Viewport, DEFAULT_PORT_HIT_RADIUS,
};
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;
use std::collections::BTreeMap;
use wasm_bindgen::JsCast;

const NODE_WIDTH: f64 = 180.0;
const NODE_PORT_PITCH: f64 = 26.0;
const NODE_FIRST_PORT_OFFSET: f64 = 40.0;

/// Screen-space distance per keydown nudge (canvas units).
const KEYBOARD_NUDGE: f64 = 10.0;
/// Nudge step while `Shift` is held.
const KEYBOARD_NUDGE_FINE: f64 = 1.0;
/// Multiplicative zoom step per wheel tick.
const WHEEL_ZOOM_STEP: f64 = 1.1;

/// A pending connection being drawn by the user (drag in progress).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PendingConnection {
    pub from_node: NodeId,
    pub from_output: u32,
    pub from_point: CanvasPoint,
    pub to_point: CanvasPoint,
}

/// Canvas-tracked node drag: survives echo-back re-renders (which recreate
/// `Node` components and would destroy any state stored inside them).
///
/// Accumulated incrementally (`origin += (cur - last_screen) / zoom`) rather
/// than recomputing from the press point, so a mid-drag viewport zoom only
/// scales *new* movement — the world point under the cursor is fixed by the
/// zoom, and already-applied node movement must not be rescaled.
#[derive(Clone, Copy, Debug, PartialEq)]
struct NodeDrag {
    id: NodeId,
    /// Current node origin, in canvas coordinates (updated every move).
    origin: CanvasPoint,
    /// Previous move's raw screen (client) position.
    last_screen: CanvasPoint,
}

fn node_box(origin: CanvasPoint) -> NodeBox {
    NodeBox {
        origin,
        width: NODE_WIDTH,
        port_pitch: NODE_PORT_PITCH,
        first_port_offset: NODE_FIRST_PORT_OFFSET,
    }
}

/// Pointer position relative to `ev`'s current target, in screen (DOM) units.
fn screen_point_of(ev: &ev::PointerEvent) -> CanvasPoint {
    if let Some(target) = ev.current_target() {
        let el: web_sys::Element = target.unchecked_into();
        let rect = el.get_bounding_client_rect();
        return CanvasPoint::new(
            ev.client_x() as f64 - rect.left(),
            ev.client_y() as f64 - rect.top(),
        );
    }
    CanvasPoint::new(ev.client_x() as f64, ev.client_y() as f64)
}

/// Render the node editor canvas. `graph`, `positions`, `selected`, and
/// `viewport` are reactive signals; mutations surface through the callbacks
/// and are written back by the parent.
#[component]
pub fn NodeCanvas(
    graph: ReadSignal<NodeGraph>,
    positions: ReadSignal<BTreeMap<NodeId, CanvasPoint>>,
    selected: ReadSignal<BTreeMap<NodeId, bool>>,
    viewport: ReadSignal<Viewport>,
    #[prop(default = (0.1, 4.0))] zoom_range: (f64, f64),
    #[prop(optional)] on_node_move: Option<Callback<(NodeId, CanvasPoint)>>,
    #[prop(optional)] on_connect: Option<Callback<Connection>>,
    #[prop(optional)] on_node_delete: Option<Callback<NodeId>>,
    #[prop(optional)] on_node_select: Option<Callback<(NodeId, bool)>>,
    #[prop(optional)] on_viewport_change: Option<Callback<Viewport>>,
) -> impl IntoView {
    let (pending, set_pending) = signal::<Option<PendingConnection>>(None);
    // Last pointer position while panning the background, in screen units.
    let panning: StoredValue<Option<CanvasPoint>> = StoredValue::new(None);
    // Active node drag, if any (see [`NodeDrag`]).
    let drag: StoredValue<Option<NodeDrag>> = StoredValue::new(None);

    let mut canvas_style = StyleBuilder::new();
    canvas_style
        .add("width", "100%")
        .add("height", "100%")
        .add("background-color", "var(--mingot-canvas-bg, #f9fafb)")
        .add("cursor", "grab")
        .add("touch-action", "none");
    let canvas_style = canvas_style.build();

    let on_background_pointerdown = move |ev: ev::PointerEvent| {
        // Primary (left) or middle button pans; port/node drags stop
        // propagation before this handler, so this is a background press.
        if ev.button() <= 1 {
            ev.prevent_default();
            // prevent_default suppresses the browser's default focus
            // transfer; focus the svg (this handler's target) so keyboard
            // commands work immediately.
            if let Some(target) = ev.current_target() {
                let el: web_sys::SvgElement = target.unchecked_into();
                let _ = el.focus();
            }
            panning.set_value(Some(screen_point_of(&ev)));
        }
    };

    let on_pointermove = move |ev: ev::PointerEvent| {
        // Active node drag: handled here on the (never-replaced) root so a
        // move that outruns the node — landing on the background or another
        // element — still drives the drag. Pointer capture (taken on press)
        // routes real-pointer moves here too. Deltas use raw client
        // coordinates on both ends (the press stores raw client coords), so
        // any host/page offset cancels out.
        if let Some(mut d) = drag.get_value() {
            let client = CanvasPoint::new(ev.client_x() as f64, ev.client_y() as f64);
            let zoom = viewport.get().zoom;
            d.origin = CanvasPoint::new(
                d.origin.x + (client.x - d.last_screen.x) / zoom,
                d.origin.y + (client.y - d.last_screen.y) / zoom,
            );
            d.last_screen = client;
            drag.set_value(Some(d));
            if let Some(cb) = on_node_move {
                cb.run((d.id, d.origin));
            }
            return;
        }
        let screen = screen_point_of(&ev);
        if let Some(last) = panning.get_value() {
            let vp = viewport.get();
            // Content follows the pointer: the world point under the cursor
            // is invariant, so pan shifts by -delta/zoom.
            let delta = CanvasPoint::new(screen.x - last.x, screen.y - last.y);
            let next = Viewport {
                pan: CanvasPoint::new(vp.pan.x - delta.x / vp.zoom, vp.pan.y - delta.y / vp.zoom),
                zoom: vp.zoom,
            };
            panning.set_value(Some(screen));
            if let Some(cb) = on_viewport_change {
                cb.run(next);
            }
        } else if let Some(p) = pending.get() {
            let world = viewport.get().screen_to_canvas(screen);
            set_pending.set(Some(PendingConnection {
                to_point: world,
                ..p
            }));
        }
    };

    let on_pointerup = move |ev: ev::PointerEvent| {
        panning.set_value(None);
        drag.set_value(None);
        if let Some(p) = pending.get() {
            set_pending.set(None);
            let world = viewport.get().screen_to_canvas(screen_point_of(&ev));
            let g = graph.get();
            let pos = positions.get();
            let candidates = pos.iter().filter_map(|(id, origin)| {
                let def = g.node(*id)?;
                Some((*id, node_box(*origin), def.inputs.len()))
            });
            if let Some((to_node, to_input)) =
                hit_test_input_port(candidates, world, DEFAULT_PORT_HIT_RADIUS)
            {
                // Self-connections are rejected outright (they are cycles by
                // definition); the model otherwise records and `validate`
                // reports precision concerns.
                if to_node != p.from_node {
                    if let Some(cb) = on_connect {
                        cb.run(Connection::new(
                            p.from_node,
                            p.from_output,
                            to_node,
                            to_input,
                        ));
                    }
                }
            }
        }
    };

    let on_pointerleave = move |_ev: ev::PointerEvent| {
        // Leaving the surface aborts any in-flight pan, wire drag, or node
        // drag.
        panning.set_value(None);
        drag.set_value(None);
        set_pending.set(None);
    };

    let on_wheel = move |ev: ev::WheelEvent| {
        ev.prevent_default();
        let factor = if ev.delta_y() < 0.0 {
            WHEEL_ZOOM_STEP
        } else {
            1.0 / WHEEL_ZOOM_STEP
        };
        let screen = if let Some(target) = ev.current_target() {
            let el: web_sys::Element = target.unchecked_into();
            let rect = el.get_bounding_client_rect();
            CanvasPoint::new(
                ev.client_x() as f64 - rect.left(),
                ev.client_y() as f64 - rect.top(),
            )
        } else {
            CanvasPoint::new(ev.client_x() as f64, ev.client_y() as f64)
        };
        let next = viewport
            .get()
            .zoomed_at(screen, factor, zoom_range.0, zoom_range.1);
        if let Some(cb) = on_viewport_change {
            cb.run(next);
        }
    };

    let on_keydown = move |ev: ev::KeyboardEvent| {
        let key = ev.key();
        let selected_now: Vec<NodeId> = selected
            .get()
            .into_iter()
            .filter(|(_, v)| *v)
            .map(|(id, _)| id)
            .collect();
        if selected_now.is_empty() {
            return;
        }
        match key.as_str() {
            "Delete" | "Backspace" => {
                ev.prevent_default();
                if let Some(cb) = on_node_delete {
                    for id in selected_now {
                        cb.run(id);
                    }
                }
            }
            "ArrowLeft" | "ArrowRight" | "ArrowUp" | "ArrowDown" => {
                ev.prevent_default();
                let step = if ev.shift_key() {
                    KEYBOARD_NUDGE_FINE
                } else {
                    KEYBOARD_NUDGE
                };
                let (dx, dy) = match key.as_str() {
                    "ArrowLeft" => (-step, 0.0),
                    "ArrowRight" => (step, 0.0),
                    "ArrowUp" => (0.0, -step),
                    _ => (0.0, step),
                };
                let pos = positions.get();
                if let Some(cb) = on_node_move {
                    for id in selected_now {
                        if let Some(origin) = pos.get(&id) {
                            cb.run((id, CanvasPoint::new(origin.x + dx, origin.y + dy)));
                        }
                    }
                }
            }
            _ => {}
        }
    };

    view! {
        <svg
            class="mingot-node-canvas"
            style=canvas_style
            tabindex="0"
            role="application"
            aria-label="Node graph editor canvas"
            on:pointerdown=on_background_pointerdown
            on:pointermove=on_pointermove
            on:pointerup=on_pointerup
            on:pointerleave=on_pointerleave
            on:wheel=on_wheel
            on:keydown=on_keydown
        >
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
                        // Press: focus the canvas (prevent_default killed the
                        // browser's default), start a canvas-level drag, and
                        // echo selection. Drag state lives on the canvas so
                        // the echo-back re-render cannot interrupt the drag.
                        let on_select_cb = Callback::new(move |press: CanvasPoint| {
                            drag.set_value(Some(NodeDrag {
                                id: node_id,
                                origin: origin_val,
                                last_screen: press,
                            }));
                            if let Some(cb) = on_node_select {
                                cb.run((node_id, true));
                            }
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
                                on_select=on_select_cb
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
