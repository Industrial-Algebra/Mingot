// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! Pure layout geometry for the node editor: port positions, connection
//! Bezier control points, and the pan/zoom viewport transform.
//!
//! This module is intentionally DOM-free so it can be unit-tested directly.
//! The SVG components (7B) consume these functions to render nodes, ports,
//! and connection wires, and to convert pointer coordinates to canvas space.

use crate::node_graph::graph::NodeId;

/// A position on the canvas, in canvas (logical) coordinates.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct CanvasPoint {
    pub x: f64,
    pub y: f64,
}

impl CanvasPoint {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Euclidean distance to `other`.
    pub fn distance_to(self, other: CanvasPoint) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }
}

/// Box dimensions of a node as laid out by the editor.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NodeBox {
    /// Top-left of the node body.
    pub origin: CanvasPoint,
    pub width: f64,
    /// Per-port vertical pitch (distance between consecutive port centers).
    pub port_pitch: f64,
    /// Vertical offset from the node top to the first port center.
    pub first_port_offset: f64,
}

impl NodeBox {
    /// Center of input port `idx` (0-based), or the node top if out of range.
    pub fn input_port_center(&self, idx: usize) -> CanvasPoint {
        CanvasPoint::new(
            self.origin.x,
            self.origin.y + self.first_port_offset + self.port_pitch * idx as f64,
        )
    }

    /// Center of output port `idx` (0-based), or the node top if out of range.
    pub fn output_port_center(&self, idx: usize) -> CanvasPoint {
        CanvasPoint::new(
            self.origin.x + self.width,
            self.origin.y + self.first_port_offset + self.port_pitch * idx as f64,
        )
    }
}

/// Pointer capture radius (canvas units) for dropping a connection on a port.
pub const DEFAULT_PORT_HIT_RADIUS: f64 = 14.0;

/// Find the input port nearest to `point` within `radius`.
///
/// Candidates are `(node id, node box, input port count)` tuples; the canvas
/// builds them from its graph and position map. Ties resolve to the first
/// candidate (lowest node id, then lowest port index) via a strict
/// less-than comparison on the running minimum.
pub fn hit_test_input_port<I>(
    candidates: I,
    point: CanvasPoint,
    radius: f64,
) -> Option<(NodeId, u32)>
where
    I: IntoIterator<Item = (NodeId, NodeBox, usize)>,
{
    let mut best: Option<(f64, NodeId, u32)> = None;
    for (id, node_box, input_count) in candidates {
        for idx in 0..input_count {
            let dist = node_box.input_port_center(idx).distance_to(point);
            if dist <= radius && best.is_none_or(|(b, _, _)| dist < b) {
                best = Some((dist, id, idx as u32));
            }
        }
    }
    best.map(|(_, id, idx)| (id, idx))
}

/// Pan and zoom of the canvas viewport.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Viewport {
    /// Canvas point currently at the top-left of the viewport.
    pub pan: CanvasPoint,
    /// Scale factor; 1.0 = no zoom.
    pub zoom: f64,
}

impl Default for Viewport {
    fn default() -> Self {
        Self {
            pan: CanvasPoint::new(0.0, 0.0),
            zoom: 1.0,
        }
    }
}

impl Viewport {
    /// Convert a screen (DOM) point to canvas coordinates.
    pub fn screen_to_canvas(&self, screen: CanvasPoint) -> CanvasPoint {
        CanvasPoint::new(
            (screen.x / self.zoom) + self.pan.x,
            (screen.y / self.zoom) + self.pan.y,
        )
    }

    /// Convert a canvas point to screen (DOM) coordinates.
    pub fn canvas_to_screen(&self, canvas: CanvasPoint) -> CanvasPoint {
        CanvasPoint::new(
            (canvas.x - self.pan.x) * self.zoom,
            (canvas.y - self.pan.y) * self.zoom,
        )
    }

    /// Clamp zoom into `[min, max]`, keeping pan unchanged.
    pub fn clamp_zoom(&self, min: f64, max: f64) -> Self {
        Self {
            pan: self.pan,
            zoom: self.zoom.clamp(min, max),
        }
    }

    /// Zoom by `factor` about a screen point, clamped to `[min, max]`.
    ///
    /// The canvas point under the cursor stays fixed: pan is adjusted so the
    /// cursor's world coordinates are invariant across the zoom change.
    pub fn zoomed_at(&self, screen: CanvasPoint, factor: f64, min: f64, max: f64) -> Self {
        let world = self.screen_to_canvas(screen);
        let new_zoom = (self.zoom * factor).clamp(min, max);
        Self {
            pan: CanvasPoint::new(world.x - screen.x / new_zoom, world.y - screen.y / new_zoom),
            zoom: new_zoom,
        }
    }
}

/// Cubic-Bezier control points for a connection wire from `from` to `to`.
///
/// Control handles extend horizontally from each endpoint, producing the
/// classic node-editor S-curve. The handle length scales with horizontal
/// separation, clamped to keep short connections tidy.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BezierHandles {
    pub from_ctrl: CanvasPoint,
    pub to_ctrl: CanvasPoint,
}

/// Compute Bezier control handles for a connection from `from` to `to`.
pub fn connection_handles(from: CanvasPoint, to: CanvasPoint) -> BezierHandles {
    // Horizontal distance drives handle length; clamp so adjacent ports
    // don't produce exaggerated curves and distant ones stay bounded.
    let dx = (to.x - from.x).abs();
    let handle = dx.clamp(40.0, 150.0) * 0.5;
    BezierHandles {
        from_ctrl: CanvasPoint::new(from.x + handle, from.y),
        to_ctrl: CanvasPoint::new(to.x - handle, to.y),
    }
}

/// Format an SVG cubic-Bezier path `d` attribute for a connection.
pub fn connection_path_d(from: CanvasPoint, to: CanvasPoint) -> String {
    let h = connection_handles(from, to);
    format!(
        "M {fx} {fy} C {c1x} {c1y}, {c2x} {c2y}, {tx} {ty}",
        fx = from.x,
        fy = from.y,
        c1x = h.from_ctrl.x,
        c1y = h.from_ctrl.y,
        c2x = h.to_ctrl.x,
        c2y = h.to_ctrl.y,
        tx = to.x,
        ty = to.y,
    )
}

/// Location of a node on the canvas.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct NodeLayout {
    pub id: NodeId,
    pub origin: CanvasPoint,
}

impl NodeLayout {
    pub fn new(id: NodeId, origin: CanvasPoint) -> Self {
        Self { id, origin }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_to_is_euclidean() {
        let a = CanvasPoint::new(0.0, 0.0);
        let b = CanvasPoint::new(3.0, 4.0);
        assert_eq!(a.distance_to(b), 5.0);
    }

    #[test]
    fn hit_test_finds_the_input_port_under_the_pointer() {
        let a = NodeBox {
            origin: CanvasPoint::new(0.0, 0.0),
            width: 180.0,
            port_pitch: 26.0,
            first_port_offset: 40.0,
        };
        let b = NodeBox {
            origin: CanvasPoint::new(300.0, 200.0),
            ..a
        };
        let hit = hit_test_input_port(
            [(NodeId(0), a, 1), (NodeId(1), b, 1)],
            CanvasPoint::new(296.0, 240.0), // near B's input 0
            DEFAULT_PORT_HIT_RADIUS,
        );
        assert_eq!(hit, Some((NodeId(1), 0)));
    }

    #[test]
    fn hit_test_resolves_the_port_index_within_a_node() {
        let nb = NodeBox {
            origin: CanvasPoint::new(0.0, 0.0),
            width: 180.0,
            port_pitch: 26.0,
            first_port_offset: 40.0,
        };
        // Second input sits at y = first_port_offset + pitch = 66.
        let hit = hit_test_input_port(
            [(NodeId(0), nb, 2)],
            CanvasPoint::new(2.0, 67.0),
            DEFAULT_PORT_HIT_RADIUS,
        );
        assert_eq!(hit, Some((NodeId(0), 1)));
    }

    #[test]
    fn hit_test_outside_radius_is_none() {
        let nb = NodeBox {
            origin: CanvasPoint::new(0.0, 0.0),
            width: 180.0,
            port_pitch: 26.0,
            first_port_offset: 40.0,
        };
        let hit = hit_test_input_port(
            [(NodeId(0), nb, 1)],
            CanvasPoint::new(500.0, 500.0),
            DEFAULT_PORT_HIT_RADIUS,
        );
        assert_eq!(hit, None);
    }

    #[test]
    fn hit_test_prefers_the_nearest_port_and_first_on_ties() {
        let nb = NodeBox {
            origin: CanvasPoint::new(0.0, 0.0),
            width: 180.0,
            port_pitch: 26.0,
            first_port_offset: 40.0,
        };
        // Exactly between input 0 (y=40) and input 1 (y=66) of the same node:
        // a tie must resolve deterministically to the lower index.
        let hit = hit_test_input_port(
            [(NodeId(0), nb, 2)],
            CanvasPoint::new(0.0, 53.0),
            DEFAULT_PORT_HIT_RADIUS,
        );
        assert_eq!(hit, Some((NodeId(0), 0)));
    }

    #[test]
    fn zoomed_at_keeps_the_world_point_under_the_cursor_fixed() {
        let vp = Viewport::default();
        let s = CanvasPoint::new(100.0, 50.0);
        let world_before = vp.screen_to_canvas(s);
        let z = vp.zoomed_at(s, 2.0, 0.1, 4.0);
        assert_eq!(z.zoom, 2.0);
        assert_eq!(z.pan, CanvasPoint::new(50.0, 25.0));
        // The world point under the cursor is unchanged.
        assert_eq!(z.screen_to_canvas(s), world_before);
    }

    #[test]
    fn zoomed_at_clamps_to_range() {
        let vp = Viewport::default();
        assert_eq!(
            vp.zoomed_at(CanvasPoint::new(0.0, 0.0), 100.0, 0.1, 4.0)
                .zoom,
            4.0
        );
        assert_eq!(
            vp.zoomed_at(CanvasPoint::new(0.0, 0.0), 0.001, 0.1, 4.0)
                .zoom,
            0.1
        );
    }

    #[test]
    fn node_box_input_ports_descend_on_the_left_edge() {
        let nb = NodeBox {
            origin: CanvasPoint::new(100.0, 50.0),
            width: 200.0,
            port_pitch: 24.0,
            first_port_offset: 40.0,
        };
        assert_eq!(nb.input_port_center(0), CanvasPoint::new(100.0, 90.0));
        assert_eq!(nb.input_port_center(2), CanvasPoint::new(100.0, 138.0));
    }

    #[test]
    fn node_box_output_ports_on_the_right_edge() {
        let nb = NodeBox {
            origin: CanvasPoint::new(100.0, 50.0),
            width: 200.0,
            port_pitch: 24.0,
            first_port_offset: 40.0,
        };
        assert_eq!(nb.output_port_center(0), CanvasPoint::new(300.0, 90.0));
        assert_eq!(nb.output_port_center(1), CanvasPoint::new(300.0, 114.0));
    }

    #[test]
    fn viewport_screen_to_canvas_and_back_round_trips() {
        let vp = Viewport {
            pan: CanvasPoint::new(50.0, -30.0),
            zoom: 2.0,
        };
        let screen = CanvasPoint::new(120.0, 80.0);
        let canvas = vp.screen_to_canvas(screen);
        assert_eq!(vp.canvas_to_screen(canvas), screen);
    }

    #[test]
    fn viewport_zoom_is_identity_at_defaults() {
        let vp = Viewport::default();
        let p = CanvasPoint::new(7.0, 11.0);
        assert_eq!(vp.screen_to_canvas(p), p);
    }

    #[test]
    fn clamp_zoom_bounds_the_scale() {
        let vp = Viewport {
            pan: CanvasPoint::new(0.0, 0.0),
            zoom: 8.0,
        };
        assert_eq!(vp.clamp_zoom(0.1, 4.0).zoom, 4.0);
        let vp = Viewport {
            pan: CanvasPoint::new(0.0, 0.0),
            zoom: 0.01,
        };
        assert_eq!(vp.clamp_zoom(0.1, 4.0).zoom, 0.1);
    }

    #[test]
    fn connection_handles_extend_horizontally_from_each_endpoint() {
        let from = CanvasPoint::new(0.0, 0.0);
        let to = CanvasPoint::new(200.0, 100.0);
        let h = connection_handles(from, to);
        // Handles are horizontal offsets: same y as their endpoint.
        assert_eq!(h.from_ctrl.y, from.y);
        assert_eq!(h.to_ctrl.y, to.y);
        // From-control is to the right of `from`; to-control left of `to`.
        assert!(h.from_ctrl.x > from.x);
        assert!(h.to_ctrl.x < to.x);
    }

    #[test]
    fn connection_path_d_starts_at_from_and_ends_at_to() {
        let from = CanvasPoint::new(10.0, 20.0);
        let to = CanvasPoint::new(110.0, 70.0);
        let d = connection_path_d(from, to);
        assert!(d.starts_with("M 10 20"), "path should start at `from`: {d}");
        assert!(d.ends_with("110 70"), "path should end at `to`: {d}");
    }

    #[test]
    fn node_layout_new_records_id_and_origin() {
        let nl = NodeLayout::new(NodeId(7), CanvasPoint::new(3.0, 4.0));
        assert_eq!(nl.id, NodeId(7));
        assert_eq!(nl.origin, CanvasPoint::new(3.0, 4.0));
    }
}
