//! Vector input component for row/column vector entry.
//!
//! Supports various vector notations with magnitude, direction,
//! and operation displays.

use crate::components::input::InputSize;
use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;
use std::f64::consts::PI;

/// Vector notation style
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum VectorNotation {
    /// Row vector [x, y, z]
    #[default]
    Row,
    /// Column vector (displayed vertically)
    Column,
    /// Angle brackets ⟨x, y, z⟩
    AngleBrackets,
    /// Parentheses (x, y, z)
    Parentheses,
    /// Unit vector notation (î, ĵ, k̂ basis)
    UnitVector,
}

impl VectorNotation {
    /// Get the left delimiter
    pub fn left(&self) -> &'static str {
        match self {
            VectorNotation::Row => "[",
            VectorNotation::Column => "[",
            VectorNotation::AngleBrackets => "⟨",
            VectorNotation::Parentheses => "(",
            VectorNotation::UnitVector => "",
        }
    }

    /// Get the right delimiter
    pub fn right(&self) -> &'static str {
        match self {
            VectorNotation::Row => "]",
            VectorNotation::Column => "]",
            VectorNotation::AngleBrackets => "⟩",
            VectorNotation::Parentheses => ")",
            VectorNotation::UnitVector => "",
        }
    }

    /// Check if this notation displays vertically
    pub fn is_vertical(&self) -> bool {
        matches!(self, VectorNotation::Column)
    }
}

/// Represents a mathematical vector
#[derive(Clone, Debug, PartialEq)]
pub struct Vector {
    /// Vector components
    components: Vec<f64>,
}

impl Default for Vector {
    fn default() -> Self {
        Self::zeros(3)
    }
}

impl Vector {
    /// Create a vector with all zeros
    pub fn zeros(dimensions: usize) -> Self {
        Self {
            components: vec![0.0; dimensions],
        }
    }

    /// Create a vector from components
    pub fn new(components: Vec<f64>) -> Self {
        Self { components }
    }

    /// Create a 2D vector
    pub fn new_2d(x: f64, y: f64) -> Self {
        Self::new(vec![x, y])
    }

    /// Create a 3D vector
    pub fn new_3d(x: f64, y: f64, z: f64) -> Self {
        Self::new(vec![x, y, z])
    }

    /// Get the number of dimensions
    pub fn dimensions(&self) -> usize {
        self.components.len()
    }

    /// Get a component by index
    pub fn get(&self, index: usize) -> Option<f64> {
        self.components.get(index).copied()
    }

    /// Set a component by index
    pub fn set(&mut self, index: usize, value: f64) {
        if index < self.components.len() {
            self.components[index] = value;
        }
    }

    /// Get x component (first)
    pub fn x(&self) -> f64 {
        self.components.first().copied().unwrap_or(0.0)
    }

    /// Get y component (second)
    pub fn y(&self) -> f64 {
        self.components.get(1).copied().unwrap_or(0.0)
    }

    /// Get z component (third)
    pub fn z(&self) -> f64 {
        self.components.get(2).copied().unwrap_or(0.0)
    }

    /// Calculate magnitude (length)
    pub fn magnitude(&self) -> f64 {
        self.components.iter().map(|x| x * x).sum::<f64>().sqrt()
    }

    /// Calculate squared magnitude (avoids sqrt)
    pub fn magnitude_squared(&self) -> f64 {
        self.components.iter().map(|x| x * x).sum()
    }

    /// Normalize to unit vector
    pub fn normalize(&self) -> Option<Vector> {
        let mag = self.magnitude();
        if mag < 1e-10 {
            return None;
        }
        Some(Vector::new(
            self.components.iter().map(|x| x / mag).collect(),
        ))
    }

    /// Check if this is a unit vector
    pub fn is_unit(&self) -> bool {
        (self.magnitude() - 1.0).abs() < 1e-10
    }

    /// Calculate dot product with another vector
    pub fn dot(&self, other: &Vector) -> Option<f64> {
        if self.dimensions() != other.dimensions() {
            return None;
        }
        Some(
            self.components
                .iter()
                .zip(other.components.iter())
                .map(|(a, b)| a * b)
                .sum(),
        )
    }

    /// Calculate cross product (3D only)
    pub fn cross(&self, other: &Vector) -> Option<Vector> {
        if self.dimensions() != 3 || other.dimensions() != 3 {
            return None;
        }
        let x = self.y() * other.z() - self.z() * other.y();
        let y = self.z() * other.x() - self.x() * other.z();
        let z = self.x() * other.y() - self.y() * other.x();
        Some(Vector::new_3d(x, y, z))
    }

    /// Calculate angle between vectors (in radians)
    pub fn angle_to(&self, other: &Vector) -> Option<f64> {
        let dot = self.dot(other)?;
        let mags = self.magnitude() * other.magnitude();
        if mags < 1e-10 {
            return None;
        }
        let cos_angle = (dot / mags).clamp(-1.0, 1.0);
        Some(cos_angle.acos())
    }

    /// Calculate direction angles (for 3D vectors)
    /// Returns (alpha, beta, gamma) - angles with x, y, z axes
    pub fn direction_angles(&self) -> Option<(f64, f64, f64)> {
        if self.dimensions() != 3 {
            return None;
        }
        let mag = self.magnitude();
        if mag < 1e-10 {
            return None;
        }
        let alpha = (self.x() / mag).acos();
        let beta = (self.y() / mag).acos();
        let gamma = (self.z() / mag).acos();
        Some((alpha, beta, gamma))
    }

    /// Scale the vector by a factor
    pub fn scale(&self, factor: f64) -> Vector {
        Vector::new(self.components.iter().map(|x| x * factor).collect())
    }

    /// Add another vector
    pub fn add(&self, other: &Vector) -> Option<Vector> {
        if self.dimensions() != other.dimensions() {
            return None;
        }
        Some(Vector::new(
            self.components
                .iter()
                .zip(other.components.iter())
                .map(|(a, b)| a + b)
                .collect(),
        ))
    }

    /// Subtract another vector
    pub fn subtract(&self, other: &Vector) -> Option<Vector> {
        if self.dimensions() != other.dimensions() {
            return None;
        }
        Some(Vector::new(
            self.components
                .iter()
                .zip(other.components.iter())
                .map(|(a, b)| a - b)
                .collect(),
        ))
    }

    /// Project onto another vector
    pub fn project_onto(&self, other: &Vector) -> Option<Vector> {
        let dot_ab = self.dot(other)?;
        let dot_bb = other.dot(other)?;
        if dot_bb < 1e-10 {
            return None;
        }
        Some(other.scale(dot_ab / dot_bb))
    }

    /// Format as unit vector notation (e.g., "3î + 4ĵ - 2k̂")
    pub fn to_unit_notation(&self) -> String {
        let basis = ["î", "ĵ", "k̂", "ê₄", "ê₅", "ê₆"];
        let mut parts = Vec::new();

        for (i, &val) in self.components.iter().enumerate() {
            if val.abs() < 1e-10 {
                continue;
            }
            let basis_symbol = basis.get(i).unwrap_or(&"eₙ");
            let coef = if (val - 1.0).abs() < 1e-10 {
                String::new()
            } else if (val + 1.0).abs() < 1e-10 {
                "-".to_string()
            } else {
                format_number(val)
            };

            if parts.is_empty() {
                if val < 0.0 && (val + 1.0).abs() >= 1e-10 {
                    parts.push(format!("-{}{}", coef.trim_start_matches('-'), basis_symbol));
                } else {
                    parts.push(format!("{}{}", coef, basis_symbol));
                }
            } else if val > 0.0 {
                parts.push(format!("+ {}{}", coef, basis_symbol));
            } else {
                parts.push(format!(
                    "- {}{}",
                    coef.trim_start_matches('-'),
                    basis_symbol
                ));
            }
        }

        if parts.is_empty() {
            "0".to_string()
        } else {
            parts.join(" ")
        }
    }

    /// Format as LaTeX
    pub fn to_latex(&self, column: bool) -> String {
        let vals: Vec<String> = self.components.iter().map(|v| format_number(*v)).collect();
        if column {
            format!(
                "\\begin{{pmatrix}} {} \\end{{pmatrix}}",
                vals.join(" \\\\ ")
            )
        } else {
            format!("\\begin{{pmatrix}} {} \\end{{pmatrix}}", vals.join(" & "))
        }
    }
}

/// Format a number, removing unnecessary trailing zeros
fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{:.0}", value)
    } else {
        let s = format!("{:.6}", value);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

/// Vector input component
#[component]
pub fn VectorInput(
    /// Current vector value
    #[prop(optional, into)]
    value: Option<RwSignal<Vector>>,

    /// Callback when vector changes
    #[prop(optional, into)]
    on_change: Option<Callback<Vector>>,

    /// Number of dimensions
    #[prop(optional, default = 3)]
    dimensions: usize,

    /// Whether to allow dimension changes
    #[prop(optional, default = false)]
    allow_resize: bool,

    /// Display notation
    #[prop(optional)]
    notation: VectorNotation,

    /// Whether to show magnitude and direction
    #[prop(optional, default = true)]
    show_magnitude: bool,

    /// Number of decimal places for display
    #[prop(optional, default = 4)]
    precision: usize,

    /// Input size
    #[prop(optional)]
    size: Option<InputSize>,

    /// Label text
    #[prop(optional, into)]
    label: Option<String>,

    /// Description text
    #[prop(optional, into)]
    description: Option<String>,

    /// Error message
    #[prop(optional, into)]
    error: Option<String>,

    /// Whether the input is disabled
    #[prop(optional)]
    disabled: Signal<bool>,
) -> impl IntoView {
    let theme = use_theme();

    // Internal state
    let internal_vector = value.unwrap_or_else(|| RwSignal::new(Vector::zeros(dimensions)));

    // Component input signals
    let component_inputs: RwSignal<Vec<RwSignal<String>>> = RwSignal::new(Vec::new());

    // Initialize inputs from vector
    let init_inputs = move || {
        let vec = internal_vector.get();
        let inputs: Vec<RwSignal<String>> = (0..vec.dimensions())
            .map(|i| {
                let val = vec.get(i).unwrap_or(0.0);
                RwSignal::new(format_number(val))
            })
            .collect();
        component_inputs.set(inputs);
    };

    // Initialize on first render
    Effect::new(move |_| {
        if component_inputs.get().is_empty() {
            init_inputs();
        }
    });

    // Update vector when component changes
    let update_component = move |index: usize, value: String| {
        if let Ok(num) = value.parse::<f64>() {
            let mut vec = internal_vector.get();
            vec.set(index, num);
            internal_vector.set(vec.clone());
            if let Some(cb) = on_change {
                cb.run(vec);
            }
        }
    };

    // Handle keyboard navigation
    let handle_keydown = move |index: usize, ev: ev::KeyboardEvent| {
        let vec = internal_vector.get();
        let new_index = match ev.key().as_str() {
            "ArrowUp" | "ArrowLeft" if index > 0 => index - 1,
            "ArrowDown" | "ArrowRight" if index < vec.dimensions() - 1 => index + 1,
            "Tab" if !ev.shift_key() && index < vec.dimensions() - 1 => {
                ev.prevent_default();
                index + 1
            }
            "Tab" if ev.shift_key() && index > 0 => {
                ev.prevent_default();
                index - 1
            }
            _ => return,
        };
        // Focus will be handled by the browser's natural tab order
        let _ = new_index;
    };

    // Add dimension
    let add_dimension = move |_| {
        let vec = internal_vector.get();
        let mut new_components = vec.components.clone();
        new_components.push(0.0);
        internal_vector.set(Vector::new(new_components));
        init_inputs();
    };

    // Remove dimension
    let remove_dimension = move |_| {
        let vec = internal_vector.get();
        if vec.dimensions() > 1 {
            let mut new_components = vec.components.clone();
            new_components.pop();
            internal_vector.set(Vector::new(new_components));
            init_inputs();
        }
    };

    // Component labels
    let get_label = move |index: usize| -> &'static str {
        match index {
            0 => "x",
            1 => "y",
            2 => "z",
            3 => "w",
            _ => "•",
        }
    };

    // Styles
    let container_styles = move || {
        let theme_val = theme.get();
        StyleBuilder::new()
            .add("display", "flex")
            .add("flex-direction", "column")
            .add("gap", theme_val.spacing.sm)
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

    let vector_container_styles = move || {
        StyleBuilder::new()
            .add("display", "flex")
            .add("align-items", "center")
            .add("gap", "0.25rem")
            .build()
    };

    let bracket_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("font-size", "1.5rem")
            .add("font-weight", "100")
            .add("color", scheme_colors.text.clone())
            .add("line-height", "1")
            .build()
    };

    let components_container_styles = move || {
        StyleBuilder::new()
            .add("display", "flex")
            .add(
                "flex-direction",
                if notation.is_vertical() {
                    "column"
                } else {
                    "row"
                },
            )
            .add("gap", "0.25rem")
            .add("align-items", "center")
            .build()
    };

    let component_group_styles = move || {
        StyleBuilder::new()
            .add("display", "flex")
            .add("align-items", "center")
            .add("gap", "0.125rem")
            .build()
    };

    let component_label_styles = move || {
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
            .add("min-width", "0.75rem")
            .build()
    };

    let input_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let size_vals = match size.unwrap_or(InputSize::Sm) {
            InputSize::Xs => ("0.125rem 0.25rem", "50px"),
            InputSize::Sm => ("0.25rem 0.5rem", "60px"),
            InputSize::Md => ("0.375rem 0.5rem", "70px"),
            InputSize::Lg => ("0.5rem 0.75rem", "80px"),
            InputSize::Xl => ("0.625rem 1rem", "90px"),
        };

        StyleBuilder::new()
            .add("padding", size_vals.0)
            .add(
                "border",
                format!("1px solid {}", scheme_colors.border.clone()),
            )
            .add("border-radius", theme_val.radius.sm)
            .add("background", scheme_colors.background.clone())
            .add("color", scheme_colors.text.clone())
            .add("font-size", theme_val.typography.font_sizes.sm)
            .add("width", size_vals.1)
            .add("text-align", "center")
            .add("font-family", "monospace")
            .build()
    };

    let resize_button_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("padding", "0.25rem 0.5rem")
            .add(
                "border",
                format!("1px solid {}", scheme_colors.border.clone()),
            )
            .add("border-radius", theme_val.radius.sm)
            .add("background", scheme_colors.background.clone())
            .add("color", scheme_colors.text.clone())
            .add("cursor", "pointer")
            .add("font-size", theme_val.typography.font_sizes.xs)
            .build()
    };

    let info_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("display", "flex")
            .add("gap", "1rem")
            .add("flex-wrap", "wrap")
            .add("font-size", theme_val.typography.font_sizes.sm)
            .add("font-family", "monospace")
            .add(
                "color",
                scheme_colors
                    .get_color("gray", 6)
                    .unwrap_or_else(|| "#868e96".to_string()),
            )
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

    let error_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("font-size", theme_val.typography.font_sizes.xs)
            .add(
                "color",
                scheme_colors
                    .get_color("red", 6)
                    .unwrap_or_else(|| "#fa5252".to_string()),
            )
            .build()
    };

    view! {
        <div class="mingot-vector-input" style=container_styles>
            {label.clone().map(|l| view! {
                <label style=label_styles>{l}</label>
            })}

            <div style=vector_container_styles>
                <span style=bracket_styles>{notation.left()}</span>

                <div style=components_container_styles>
                    {move || {
                        let inputs = component_inputs.get();
                        let len = inputs.len();
                        inputs.into_iter().enumerate().map(|(i, input_signal)| {
                            let sep = if notation == VectorNotation::UnitVector {
                                ""
                            } else if i < len - 1 {
                                ","
                            } else {
                                ""
                            };

                            view! {
                                <div style=component_group_styles>
                                    {(notation != VectorNotation::UnitVector).then(|| view! {
                                        <span style=component_label_styles>{get_label(i)}</span>
                                    })}
                                    <input
                                        type="text"
                                        style=input_styles
                                        prop:value=move || input_signal.get()
                                        disabled=disabled
                                        on:input=move |ev| {
                                            let val = event_target_value(&ev);
                                            input_signal.set(val.clone());
                                            update_component(i, val);
                                        }
                                        on:keydown=move |ev| {
                                            handle_keydown(i, ev);
                                        }
                                    />
                                    {(!sep.is_empty()).then(|| view! {
                                        <span style=component_label_styles>{sep}</span>
                                    })}
                                </div>
                            }
                        }).collect_view()
                    }}
                </div>

                <span style=bracket_styles>{notation.right()}</span>
            </div>

            {allow_resize.then(|| {
                view! {
                    <div style="display: flex; gap: 0.5rem;">
                        <button type="button" style=resize_button_styles on:click=add_dimension disabled=disabled>
                            {"+ Dim"}
                        </button>
                        <button type="button" style=resize_button_styles on:click=remove_dimension disabled=disabled>
                            {"- Dim"}
                        </button>
                    </div>
                }
            })}

            {show_magnitude.then(|| {
                view! {
                    <div style=info_styles>
                        {move || {
                            let vec = internal_vector.get();
                            let mut info = Vec::new();

                            // Magnitude
                            let mag = vec.magnitude();
                            info.push(format!("|v| = {:.prec$}", mag, prec = precision));

                            // Unit vector notation (for 2D/3D)
                            if vec.dimensions() <= 3 {
                                info.push(vec.to_unit_notation());
                            }

                            // Direction angles (3D only)
                            if let Some((alpha, beta, gamma)) = vec.direction_angles() {
                                info.push(format!(
                                    "α={:.1}° β={:.1}° γ={:.1}°",
                                    alpha * 180.0 / PI,
                                    beta * 180.0 / PI,
                                    gamma * 180.0 / PI
                                ));
                            }

                            info.into_iter().map(|s| view! { <span>{s}</span> }).collect_view()
                        }}
                    </div>
                }
            })}

            {description.map(|d| view! {
                <div style=description_styles>{d}</div>
            })}

            {error.map(|e| view! {
                <div style=error_styles>{e}</div>
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_zeros() {
        let v = Vector::zeros(3);
        assert_eq!(v.dimensions(), 3);
        assert_eq!(v.get(0), Some(0.0));
    }

    #[test]
    fn test_vector_new_3d() {
        let v = Vector::new_3d(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_vector_magnitude() {
        let v = Vector::new_3d(3.0, 4.0, 0.0);
        assert_eq!(v.magnitude(), 5.0);
    }

    #[test]
    fn test_vector_normalize() {
        let v = Vector::new_3d(3.0, 4.0, 0.0);
        let n = v.normalize().unwrap();
        assert!((n.magnitude() - 1.0).abs() < 1e-10);
        assert!((n.x() - 0.6).abs() < 1e-10);
        assert!((n.y() - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_vector_dot_product() {
        let v1 = Vector::new_3d(1.0, 2.0, 3.0);
        let v2 = Vector::new_3d(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), Some(32.0)); // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_vector_cross_product() {
        let v1 = Vector::new_3d(1.0, 0.0, 0.0);
        let v2 = Vector::new_3d(0.0, 1.0, 0.0);
        let cross = v1.cross(&v2).unwrap();
        assert_eq!(cross.x(), 0.0);
        assert_eq!(cross.y(), 0.0);
        assert_eq!(cross.z(), 1.0);
    }

    #[test]
    fn test_vector_angle() {
        let v1 = Vector::new_3d(1.0, 0.0, 0.0);
        let v2 = Vector::new_3d(0.0, 1.0, 0.0);
        let angle = v1.angle_to(&v2).unwrap();
        assert!((angle - PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_vector_scale() {
        let v = Vector::new_3d(1.0, 2.0, 3.0);
        let scaled = v.scale(2.0);
        assert_eq!(scaled.x(), 2.0);
        assert_eq!(scaled.y(), 4.0);
        assert_eq!(scaled.z(), 6.0);
    }

    #[test]
    fn test_vector_add() {
        let v1 = Vector::new_3d(1.0, 2.0, 3.0);
        let v2 = Vector::new_3d(4.0, 5.0, 6.0);
        let sum = v1.add(&v2).unwrap();
        assert_eq!(sum.x(), 5.0);
        assert_eq!(sum.y(), 7.0);
        assert_eq!(sum.z(), 9.0);
    }

    #[test]
    fn test_vector_subtract() {
        let v1 = Vector::new_3d(4.0, 5.0, 6.0);
        let v2 = Vector::new_3d(1.0, 2.0, 3.0);
        let diff = v1.subtract(&v2).unwrap();
        assert_eq!(diff.x(), 3.0);
        assert_eq!(diff.y(), 3.0);
        assert_eq!(diff.z(), 3.0);
    }

    #[test]
    fn test_vector_projection() {
        let v1 = Vector::new_2d(3.0, 4.0);
        let v2 = Vector::new_2d(1.0, 0.0);
        let proj = v1.project_onto(&v2).unwrap();
        assert_eq!(proj.x(), 3.0);
        assert_eq!(proj.y(), 0.0);
    }

    #[test]
    fn test_vector_unit_notation() {
        let v = Vector::new_3d(3.0, -4.0, 0.0);
        let notation = v.to_unit_notation();
        assert!(notation.contains("î"));
        assert!(notation.contains("ĵ"));
    }

    #[test]
    fn test_vector_is_unit() {
        let v = Vector::new_3d(1.0, 0.0, 0.0);
        assert!(v.is_unit());

        let v2 = Vector::new_3d(1.0, 1.0, 0.0);
        assert!(!v2.is_unit());
    }

    #[test]
    fn test_vector_notation() {
        assert_eq!(VectorNotation::Row.left(), "[");
        assert_eq!(VectorNotation::AngleBrackets.left(), "⟨");
        assert_eq!(VectorNotation::AngleBrackets.right(), "⟩");
        assert!(VectorNotation::Column.is_vertical());
        assert!(!VectorNotation::Row.is_vertical());
    }
}
