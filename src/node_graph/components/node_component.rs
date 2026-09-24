// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! A single node rendered on the canvas: a titled box with input ports on the
//! left edge and output ports on the right.
//!
//! Pointer handlers sit on the whole node group (body, title, and any area
//! inside the box) so every part of the node is a consistent select/drag
//! target; port groups stop propagation first and start wire drags instead.
//!
//! The node keeps **no drag state**: `prevent_default`-ed presses and any
//! parent echo-back update (selection, position) re-run the canvas render and
//! recreate this component, which would destroy local state mid-drag. Instead
//! the node reports raw pointer coordinates upward — `on_select` carries the
//! press position and `on_drag_move` the move position while the primary
//! button is held — and the canvas tracks the drag.

use super::node_port::{NodePort, PortSide};
use crate::node_graph::layout::{CanvasPoint, NodeBox};
use crate::node_graph::node::NodeDefinition;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

const DEFAULT_WIDTH: f64 = 180.0;
const DEFAULT_PORT_PITCH: f64 = 26.0;
const DEFAULT_FIRST_PORT_OFFSET: f64 = 40.0;

/// Focus the root `<svg>` that owns `ev`'s current target.
fn focus_owner_svg(ev: &ev::PointerEvent) {
    if let Some(target) = ev.current_target() {
        let el: web_sys::SvgElement = target.unchecked_into();
        if let Some(svg) = el.owner_svg_element() {
            let _ = svg.focus();
        }
    }
}

/// Render a node as an SVG group: a rounded-rect body, a title, and its ports.
#[component]
pub fn Node(
    definition: NodeDefinition,
    #[prop(into)] origin: CanvasPoint,
    #[prop(optional)] selected: bool,
    /// Press anywhere on the node (body, title, label-free areas): raw screen
    /// (client) coordinates of the press.
    #[prop(optional)]
    on_select: Option<Callback<CanvasPoint>>,
    /// Primary-button drag over the node: raw screen (client) coordinates per
    /// move event. Fires only while the button is held.
    #[prop(optional)]
    on_drag_move: Option<Callback<CanvasPoint>>,
    #[prop(optional)] on_port_grab: Option<Callback<(PortSide, u32)>>,
) -> impl IntoView {
    let box_ = NodeBox {
        origin,
        width: DEFAULT_WIDTH,
        port_pitch: DEFAULT_PORT_PITCH,
        first_port_offset: DEFAULT_FIRST_PORT_OFFSET,
    };

    let port_rows = definition.inputs.len().max(definition.outputs.len());
    let body_height = DEFAULT_FIRST_PORT_OFFSET + DEFAULT_PORT_PITCH * port_rows as f64 + 16.0;

    let stroke = if selected {
        "var(--mingot-node-selected, #2563eb)"
    } else {
        "var(--mingot-node-border, #d1d5db)"
    };
    let mut body_style = StyleBuilder::new();
    body_style.add("cursor", "grab");
    let body_style = body_style.build();

    let inputs: Vec<_> = definition
        .inputs
        .iter()
        .enumerate()
        .map(|(idx, p)| {
            (
                idx,
                p.name.clone(),
                p.port_type.clone(),
                box_.input_port_center(idx),
            )
        })
        .collect();
    let outputs: Vec<_> = definition
        .outputs
        .iter()
        .enumerate()
        .map(|(idx, p)| {
            (
                idx,
                p.name.clone(),
                p.port_type.clone(),
                box_.output_port_center(idx),
            )
        })
        .collect();
    let title = definition.title.clone().into_owned();

    view! {
        <g
            transform=format!("translate({} {})", origin.x, origin.y)
            on:pointerdown=move |ev: ev::PointerEvent| {
                // Whole-group hit area: the title text and inner areas are
                // select/drag targets just like the body rectangle.
                ev.prevent_default();
                ev.stop_propagation();
                // prevent_default suppresses the browser's default focus
                // transfer; focus the owning canvas directly so keyboard
                // commands work right after the press.
                focus_owner_svg(&ev);
                if let Some(cb) = on_select {
                    cb.run(CanvasPoint::new(ev.client_x() as f64, ev.client_y() as f64));
                }
            }
            on:pointermove=move |ev: ev::PointerEvent| {
                // Report moves only while the primary button is held; the
                // canvas decides whether a drag is in progress. No local
                // state, so echo-back re-renders cannot interrupt a drag.
                if ev.buttons() & 1 != 0 {
                    if let Some(cb) = on_drag_move {
                        cb.run(CanvasPoint::new(ev.client_x() as f64, ev.client_y() as f64));
                    }
                }
            }
        >
            <rect
                width=DEFAULT_WIDTH
                height=body_height
                rx="8"
                fill="var(--mingot-node-fill, #ffffff)"
                stroke=stroke
                stroke-width="2"
                style=body_style
            />
            <text x="10" y="22" fill="var(--mingot-node-title, #111827)" font-size="13" font-weight="600">
                {title}
            </text>

            {inputs
                .clone()
                .into_iter()
                .map(|(idx, name, port_type, center)| {
                    let on_grab = on_port_grab.unwrap_or_else(|| Callback::new(move |_| {}));
                    view! {
                        <g transform=format!("translate({} {})", center.x - origin.x, center.y - origin.y)>
                            <NodePort
                                label=name
                                port_type=port_type
                                side=PortSide::Input
                                index={idx as u32}
                                on_grab=on_grab
                            />
                        </g>
                    }
                })
                .collect_view()}

            {outputs
                .clone()
                .into_iter()
                .map(|(idx, name, port_type, center)| {
                    let on_grab = on_port_grab.unwrap_or_else(|| Callback::new(move |_| {}));
                    view! {
                        <g transform=format!("translate({} {})", center.x - origin.x, center.y - origin.y)>
                            <NodePort
                                label=name
                                port_type=port_type
                                side=PortSide::Output
                                index={idx as u32}
                                on_grab=on_grab
                            />
                        </g>
                    }
                })
                .collect_view()}
        </g>
    }
}
