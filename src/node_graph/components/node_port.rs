// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! A single input or output port rendered on a node.

use crate::node_graph::precision::PortType;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;
use std::borrow::Cow;

/// Which side of a node a port sits on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PortSide {
    Input,
    Output,
}

/// Visual style hint for a port, color-coded by [`PortType`].
fn port_fill(port_type: &PortType) -> &'static str {
    match port_type {
        PortType::Integer(_) => "var(--mingot-port-integer, #3b82f6)",
        PortType::Decimal(_) => "var(--mingot-port-decimal, #10b981)",
        #[cfg(feature = "high-precision")]
        PortType::Arbitrary => "var(--mingot-port-arbitrary, #8b5cf6)",
        PortType::Text => "var(--mingot-port-text, #f59e0b)",
        PortType::Bool => "var(--mingot-port-bool, #ef4444)",
    }
}

/// Render a port: a colored circle plus its label, on the node's left (input)
/// or right (output) edge. Draggable to initiate a connection.
#[component]
pub fn NodePort(
    #[prop(into)] label: Cow<'static, str>,
    port_type: PortType,
    side: PortSide,
    index: u32,
    #[prop(optional)] connected: bool,
    #[prop(optional)] on_grab: Option<Callback<(PortSide, u32)>>,
) -> impl IntoView {
    let fill = port_fill(&port_type);
    let stroke = if connected {
        "var(--mingot-port-connected, #1f2937)"
    } else {
        "var(--mingot-port-idle, #9ca3af)"
    };
    let label_x = match side {
        PortSide::Input => 14.0,
        PortSide::Output => -14.0,
    };
    let anchor = match side {
        PortSide::Input => "start",
        PortSide::Output => "end",
    };
    let mut handle_style = StyleBuilder::new();
    handle_style.add("cursor", "crosshair");
    let handle_style = handle_style.build();

    view! {
        <g class="mingot-node-port" style=handle_style>
            <circle
                r="6"
                fill=fill
                stroke=stroke
                stroke-width="2"
                on:pointerdown=move |ev: ev::PointerEvent| {
                    ev.prevent_default();
                    if let Some(cb) = on_grab {
                        cb.run((side, index));
                    }
                }
            />
            <text x=label_x y="4" text-anchor=anchor fill="currentColor" font-size="12">
                {label.into_owned()}
            </text>
        </g>
    }
}
