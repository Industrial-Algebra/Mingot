// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! A single node rendered on the canvas: a titled box with input ports on the
//! left edge and output ports on the right.

use super::node_port::{NodePort, PortSide};
use crate::node_graph::layout::{CanvasPoint, NodeBox};
use crate::node_graph::node::NodeDefinition;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;

const DEFAULT_WIDTH: f64 = 180.0;
const DEFAULT_PORT_PITCH: f64 = 26.0;
const DEFAULT_FIRST_PORT_OFFSET: f64 = 40.0;

/// Render a node as an SVG group: a rounded-rect body, a title, and its ports.
#[component]
pub fn Node(
    definition: NodeDefinition,
    #[prop(into)] origin: CanvasPoint,
    #[prop(optional)] selected: bool,
    #[prop(optional)] on_move: Option<Callback<CanvasPoint>>,
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

    let dragging = StoredValue::new(false);
    let drag_last = StoredValue::new(origin);

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
        <g transform=format!("translate({} {})", origin.x, origin.y)>
            <rect
                width=DEFAULT_WIDTH
                height=body_height
                rx="8"
                fill="var(--mingot-node-fill, #ffffff)"
                stroke=stroke
                stroke-width="2"
                style=body_style
                on:pointerdown=move |ev: ev::PointerEvent| {
                    ev.prevent_default();
                    dragging.set_value(true);
                    drag_last.set_value(origin);
                }
                on:pointermove=move |ev: ev::PointerEvent| {
                    if dragging.get_value() {
                        let dx = ev.movement_x() as f64;
                        let dy = ev.movement_y() as f64;
                        let last = drag_last.get_value();
                        let next = CanvasPoint::new(last.x + dx, last.y + dy);
                        drag_last.set_value(next);
                        if let Some(cb) = on_move {
                            cb.run(next);
                        }
                    }
                }
                on:pointerup=move |_ev: ev::PointerEvent| {
                    dragging.set_value(false);
                }
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
