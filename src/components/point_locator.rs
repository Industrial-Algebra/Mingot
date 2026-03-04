//! Point locator component for visual point positioning.
//!
//! Mathematica-style drag-and-drop point positioning with grid snapping
//! and precision coordinate display.

use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// A 2D point with x and y coordinates
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Point2D {
    pub x: f64,
    pub y: f64,
}

impl Point2D {
    /// Create a new point
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    /// Calculate distance to another point
    pub fn distance_to(&self, other: &Point2D) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        (dx * dx + dy * dy).sqrt()
    }

    /// Calculate distance from origin
    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    /// Snap to grid
    pub fn snap_to_grid(&self, grid_size: f64) -> Self {
        Self {
            x: (self.x / grid_size).round() * grid_size,
            y: (self.y / grid_size).round() * grid_size,
        }
    }

    /// Clamp to bounds
    pub fn clamp(&self, min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self {
        Self {
            x: self.x.clamp(min_x, max_x),
            y: self.y.clamp(min_y, max_y),
        }
    }
}

/// Bounds for the point locator
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bounds {
    pub min_x: f64,
    pub max_x: f64,
    pub min_y: f64,
    pub max_y: f64,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            min_x: -10.0,
            max_x: 10.0,
            min_y: -10.0,
            max_y: 10.0,
        }
    }
}

impl Bounds {
    /// Create new bounds
    pub fn new(min_x: f64, max_x: f64, min_y: f64, max_y: f64) -> Self {
        Self {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }

    /// Create symmetric bounds centered at origin
    pub fn symmetric(range: f64) -> Self {
        Self::new(-range, range, -range, range)
    }

    /// Get the width
    pub fn width(&self) -> f64 {
        self.max_x - self.min_x
    }

    /// Get the height
    pub fn height(&self) -> f64 {
        self.max_y - self.min_y
    }

    /// Check if a point is within bounds
    pub fn contains(&self, point: &Point2D) -> bool {
        point.x >= self.min_x
            && point.x <= self.max_x
            && point.y >= self.min_y
            && point.y <= self.max_y
    }

    /// Clamp a point to these bounds
    pub fn clamp(&self, point: &Point2D) -> Point2D {
        point.clamp(self.min_x, self.max_x, self.min_y, self.max_y)
    }
}

/// Format a number for display
fn format_number(value: f64, precision: usize) -> String {
    if precision == 0 {
        format!("{:.0}", value)
    } else {
        format!("{:.prec$}", value, prec = precision)
    }
}

/// Point locator component
#[component]
pub fn PointLocator(
    /// Current point value
    #[prop(optional, into)]
    value: Option<RwSignal<Point2D>>,

    /// Callback when point changes
    #[prop(optional, into)]
    on_change: Option<Callback<Point2D>>,

    /// Coordinate bounds
    #[prop(optional)]
    bounds: Bounds,

    /// Grid snap size (None for no snapping)
    #[prop(optional, into)]
    snap_to_grid: Option<f64>,

    /// Canvas width in pixels
    #[prop(optional, default = 300)]
    width: u32,

    /// Canvas height in pixels
    #[prop(optional, default = 300)]
    height: u32,

    /// Show grid lines
    #[prop(optional, default = true)]
    show_grid: bool,

    /// Show axis labels
    #[prop(optional, default = true)]
    show_labels: bool,

    /// Show crosshair cursor
    #[prop(optional, default = true)]
    show_crosshair: bool,

    /// Number of decimal places for display
    #[prop(optional, default = 2)]
    precision: usize,

    /// Point color
    #[prop(optional, into)]
    point_color: Option<String>,

    /// Point radius in pixels
    #[prop(optional, default = 8.0)]
    point_radius: f64,

    /// Label text
    #[prop(optional, into)]
    label: Option<String>,

    /// Description text
    #[prop(optional, into)]
    description: Option<String>,

    /// Whether the input is disabled
    #[prop(optional)]
    disabled: Signal<bool>,
) -> impl IntoView {
    let theme = use_theme();

    // Internal state
    let internal_point = value.unwrap_or_else(|| RwSignal::new(Point2D::new(0.0, 0.0)));
    let is_dragging = RwSignal::new(false);
    let mouse_pos = RwSignal::new(None::<Point2D>);

    // Convert canvas coordinates to data coordinates
    let canvas_to_data = move |canvas_x: f64, canvas_y: f64| -> Point2D {
        let scale_x = bounds.width() / width as f64;
        let scale_y = bounds.height() / height as f64;

        let x = bounds.min_x + canvas_x * scale_x;
        // Flip Y axis (canvas Y increases downward, data Y increases upward)
        let y = bounds.max_y - canvas_y * scale_y;

        Point2D::new(x, y)
    };

    // Convert data coordinates to canvas coordinates
    let data_to_canvas = move |point: &Point2D| -> (f64, f64) {
        let scale_x = width as f64 / bounds.width();
        let scale_y = height as f64 / bounds.height();

        let canvas_x = (point.x - bounds.min_x) * scale_x;
        // Flip Y axis
        let canvas_y = (bounds.max_y - point.y) * scale_y;

        (canvas_x, canvas_y)
    };

    // Handle mouse/touch events
    let update_point = move |canvas_x: f64, canvas_y: f64| {
        if disabled.get() {
            return;
        }

        let mut point = canvas_to_data(canvas_x, canvas_y);

        // Apply grid snapping if enabled
        if let Some(grid) = snap_to_grid {
            point = point.snap_to_grid(grid);
        }

        // Clamp to bounds
        point = bounds.clamp(&point);

        internal_point.set(point);
        if let Some(cb) = on_change {
            cb.run(point);
        }
    };

    let handle_mouse_down = move |ev: ev::MouseEvent| {
        if disabled.get() {
            return;
        }
        is_dragging.set(true);

        let target = ev.target().unwrap();
        let element = target.dyn_ref::<web_sys::Element>().unwrap();
        let rect = element.get_bounding_client_rect();

        let canvas_x = ev.client_x() as f64 - rect.left();
        let canvas_y = ev.client_y() as f64 - rect.top();

        update_point(canvas_x, canvas_y);
    };

    let handle_mouse_move = move |ev: ev::MouseEvent| {
        let target = ev.target().unwrap();
        let element = target.dyn_ref::<web_sys::Element>().unwrap();
        let rect = element.get_bounding_client_rect();

        let canvas_x = ev.client_x() as f64 - rect.left();
        let canvas_y = ev.client_y() as f64 - rect.top();

        // Update mouse position for crosshair
        mouse_pos.set(Some(canvas_to_data(canvas_x, canvas_y)));

        if is_dragging.get() && !disabled.get() {
            update_point(canvas_x, canvas_y);
        }
    };

    let handle_mouse_up = move |_ev: ev::MouseEvent| {
        is_dragging.set(false);
    };

    let handle_mouse_leave = move |_ev: ev::MouseEvent| {
        is_dragging.set(false);
        mouse_pos.set(None);
    };

    // Styles
    let container_styles = move || {
        let theme_val = theme.get();
        StyleBuilder::new()
            .add("display", "flex")
            .add("flex-direction", "column")
            .add("gap", theme_val.spacing.xs)
            .build()
    };

    let label_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("font-size", theme_val.typography.font_sizes.sm)
            .add(
                "font-weight",
                theme_val.typography.font_weights.medium.to_string(),
            )
            .add("color", scheme_colors.text.clone())
            .build()
    };

    let canvas_container_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("position", "relative")
            .add("width", format!("{}px", width))
            .add("height", format!("{}px", height))
            .add(
                "border",
                format!("1px solid {}", scheme_colors.border.clone()),
            )
            .add("border-radius", theme_val.radius.sm)
            .add("background", scheme_colors.background.clone())
            .add(
                "cursor",
                if disabled.get() {
                    "not-allowed"
                } else {
                    "crosshair"
                },
            )
            .add("user-select", "none")
            .add("touch-action", "none")
            .build()
    };

    let svg_styles = StyleBuilder::new()
        .add("position", "absolute")
        .add("top", "0")
        .add("left", "0")
        .add("width", "100%")
        .add("height", "100%")
        .build();

    let coord_display_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("font-size", theme_val.typography.font_sizes.sm)
            .add("font-family", "monospace")
            .add("color", scheme_colors.text.clone())
            .add("padding", "0.25rem 0.5rem")
            .add("background", scheme_colors.background.clone())
            .add(
                "border",
                format!("1px solid {}", scheme_colors.border.clone()),
            )
            .add("border-radius", theme_val.radius.sm)
            .add("margin-top", theme_val.spacing.xs)
            .build()
    };

    let description_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("font-size", theme_val.typography.font_sizes.xs)
            .add(
                "color",
                scheme_colors
                    .get_color("gray", 6)
                    .unwrap_or_else(|| "#868e96".to_string()),
            )
            .build()
    };

    // Clone values needed for closures
    let point_color_val = point_color.clone();

    view! {
        <div class="mingot-point-locator" style=container_styles>
            {label.clone().map(|l| view! {
                <label style=label_styles>{l}</label>
            })}

            <div
                style=canvas_container_styles
                on:mousedown=handle_mouse_down
                on:mousemove=handle_mouse_move
                on:mouseup=handle_mouse_up
                on:mouseleave=handle_mouse_leave
            >
                <svg
                    style=svg_styles
                    viewBox=format!("0 0 {} {}", width, height)
                    xmlns="http://www.w3.org/2000/svg"
                >
                    // Grid lines
                    {move || {
                        if !show_grid {
                            return view! { <g></g> }.into_any();
                        }

                        let theme_val = theme.get();
                        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
                        let border_color = scheme_colors.border.clone();
                        let grid_step = snap_to_grid.unwrap_or(1.0);

                        // Calculate grid lines
                        let mut lines = Vec::new();

                        // Vertical lines
                        let mut x = bounds.min_x;
                        while x <= bounds.max_x {
                            let (cx, _) = data_to_canvas(&Point2D::new(x, 0.0));
                            lines.push((cx, 0.0, cx, height as f64, x == 0.0));
                            x += grid_step;
                        }

                        // Horizontal lines
                        let mut y = bounds.min_y;
                        while y <= bounds.max_y {
                            let (_, cy) = data_to_canvas(&Point2D::new(0.0, y));
                            lines.push((0.0, cy, width as f64, cy, y == 0.0));
                            y += grid_step;
                        }

                        view! {
                            <g class="grid-lines">
                                {lines.into_iter().map(|(x1, y1, x2, y2, is_axis)| {
                                    let stroke_width = if is_axis { "1.5" } else { "0.5" };
                                    let opacity = if is_axis { "0.6" } else { "0.3" };
                                    view! {
                                        <line
                                            x1=x1
                                            y1=y1
                                            x2=x2
                                            y2=y2
                                            stroke=border_color.clone()
                                            stroke-width=stroke_width
                                            opacity=opacity
                                        />
                                    }
                                }).collect_view()}
                            </g>
                        }.into_any()
                    }}

                    // Axis labels
                    {move || {
                        if !show_labels {
                            return view! { <g></g> }.into_any();
                        }

                        let theme_val = theme.get();
                        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
                        let text_color = scheme_colors.text.clone();
                        let (origin_x, origin_y) = data_to_canvas(&Point2D::new(0.0, 0.0));

                        view! {
                            <g class="axis-labels">
                                // X axis label
                                <text
                                    x=width as f64 - 15.0
                                    y=origin_y + 15.0
                                    fill=text_color.clone()
                                    font-size="12"
                                    text-anchor="end"
                                >
                                    {"x"}
                                </text>
                                // Y axis label
                                <text
                                    x=origin_x + 10.0
                                    y="15"
                                    fill=text_color
                                    font-size="12"
                                >
                                    {"y"}
                                </text>
                            </g>
                        }.into_any()
                    }}

                    // Crosshair at mouse position
                    {move || {
                        if !show_crosshair {
                            return view! { <g></g> }.into_any();
                        }

                        let theme_val = theme.get();
                        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
                        let border_color = scheme_colors.border.clone();

                        mouse_pos.get().map(|mp| {
                            let (cx, cy) = data_to_canvas(&mp);
                            view! {
                                <g class="crosshair" opacity="0.5">
                                    <line
                                        x1=cx
                                        y1="0"
                                        x2=cx
                                        y2=height
                                        stroke=border_color.clone()
                                        stroke-width="1"
                                        stroke-dasharray="4,4"
                                    />
                                    <line
                                        x1="0"
                                        y1=cy
                                        x2=width
                                        y2=cy
                                        stroke=border_color
                                        stroke-width="1"
                                        stroke-dasharray="4,4"
                                    />
                                </g>
                            }
                        }).into_any()
                    }}

                    // Point marker
                    {move || {
                        let point = internal_point.get();
                        let (cx, cy) = data_to_canvas(&point);
                        let theme_val = theme.get();
                        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
                        let point_col = point_color_val
                            .clone()
                            .unwrap_or_else(|| scheme_colors.get_color(&theme_val.colors.primary_color, 6).unwrap_or_else(|| "#228be6".to_string()));

                        view! {
                            <g class="point-marker">
                                // Outer ring
                                <circle
                                    cx=cx
                                    cy=cy
                                    r=point_radius + 2.0
                                    fill="none"
                                    stroke=point_col.clone()
                                    stroke-width="2"
                                    opacity="0.5"
                                />
                                // Inner circle
                                <circle
                                    cx=cx
                                    cy=cy
                                    r=point_radius
                                    fill=point_col.clone()
                                    stroke="white"
                                    stroke-width="2"
                                />
                                // Center dot
                                <circle
                                    cx=cx
                                    cy=cy
                                    r="2"
                                    fill="white"
                                />
                            </g>
                        }
                    }}
                </svg>
            </div>

            // Coordinate display
            <div style=coord_display_styles>
                {move || {
                    let point = internal_point.get();
                    format!(
                        "({}, {})",
                        format_number(point.x, precision),
                        format_number(point.y, precision)
                    )
                }}
                {move || {
                    mouse_pos.get().map(|mp| {
                        let snapped = if let Some(grid) = snap_to_grid {
                            mp.snap_to_grid(grid)
                        } else {
                            mp
                        };
                        format!(
                            " → ({}, {})",
                            format_number(snapped.x, precision),
                            format_number(snapped.y, precision)
                        )
                    })
                }}
            </div>

            {description.map(|d| view! {
                <div style=description_styles>{d}</div>
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point_new() {
        let p = Point2D::new(3.0, 4.0);
        assert_eq!(p.x, 3.0);
        assert_eq!(p.y, 4.0);
    }

    #[test]
    fn test_point_distance() {
        let p1 = Point2D::new(0.0, 0.0);
        let p2 = Point2D::new(3.0, 4.0);
        assert_eq!(p1.distance_to(&p2), 5.0);
    }

    #[test]
    fn test_point_magnitude() {
        let p = Point2D::new(3.0, 4.0);
        assert_eq!(p.magnitude(), 5.0);
    }

    #[test]
    fn test_point_snap_to_grid() {
        let p = Point2D::new(2.3, 4.7);
        let snapped = p.snap_to_grid(1.0);
        assert_eq!(snapped.x, 2.0);
        assert_eq!(snapped.y, 5.0);

        let snapped_half = p.snap_to_grid(0.5);
        assert_eq!(snapped_half.x, 2.5);
        assert_eq!(snapped_half.y, 4.5);
    }

    #[test]
    fn test_point_clamp() {
        let p = Point2D::new(15.0, -15.0);
        let clamped = p.clamp(-10.0, 10.0, -10.0, 10.0);
        assert_eq!(clamped.x, 10.0);
        assert_eq!(clamped.y, -10.0);
    }

    #[test]
    fn test_bounds_default() {
        let b = Bounds::default();
        assert_eq!(b.min_x, -10.0);
        assert_eq!(b.max_x, 10.0);
        assert_eq!(b.width(), 20.0);
        assert_eq!(b.height(), 20.0);
    }

    #[test]
    fn test_bounds_symmetric() {
        let b = Bounds::symmetric(5.0);
        assert_eq!(b.min_x, -5.0);
        assert_eq!(b.max_x, 5.0);
        assert_eq!(b.min_y, -5.0);
        assert_eq!(b.max_y, 5.0);
    }

    #[test]
    fn test_bounds_contains() {
        let b = Bounds::symmetric(10.0);
        assert!(b.contains(&Point2D::new(0.0, 0.0)));
        assert!(b.contains(&Point2D::new(10.0, 10.0)));
        assert!(!b.contains(&Point2D::new(11.0, 0.0)));
    }

    #[test]
    fn test_bounds_clamp() {
        let b = Bounds::symmetric(10.0);
        let p = Point2D::new(15.0, -15.0);
        let clamped = b.clamp(&p);
        assert_eq!(clamped.x, 10.0);
        assert_eq!(clamped.y, -10.0);
    }
}
