// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: AGPL-3.0-only

//! A connection wire rendered as a cubic-Bezier SVG path.

use crate::node_graph::layout::{connection_path_d, CanvasPoint};
use leptos::prelude::*;

/// Visual style of a connection wire.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ConnectionStyle {
    #[default]
    Bezier,
    Straight,
}

/// Render a connection wire between two canvas points.
#[component]
pub fn NodeConnection(
    #[prop(into)] from: CanvasPoint,
    #[prop(into)] to: CanvasPoint,
    #[prop(optional)] style: Option<ConnectionStyle>,
    #[prop(optional)] highlighted: bool,
) -> impl IntoView {
    let stroke = if highlighted {
        "var(--mingot-connection-highlight, #2563eb)"
    } else {
        "var(--mingot-connection, #6b7280)"
    };
    let style = style.unwrap_or_default();
    let d = match style {
        ConnectionStyle::Bezier => connection_path_d(from, to),
        ConnectionStyle::Straight => format!(
            "M {fx} {fy} L {tx} {ty}",
            fx = from.x,
            fy = from.y,
            tx = to.x,
            ty = to.y
        ),
    };
    view! {
        <path d=d fill="none" stroke=stroke stroke-width="2" />
    }
}
