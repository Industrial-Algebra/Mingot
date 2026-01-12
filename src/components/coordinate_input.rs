//! Coordinate input component for 2D/3D coordinate entry.
//!
//! Supports Cartesian, Polar, Cylindrical, and Spherical coordinate systems
//! with automatic conversion between them.

use crate::components::input::{InputSize, InputVariant};
use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;
use std::f64::consts::PI;

/// Coordinate system types
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CoordinateSystem {
    /// 2D Cartesian (x, y)
    Cartesian2D,
    /// 3D Cartesian (x, y, z)
    #[default]
    Cartesian3D,
    /// Polar (r, θ)
    Polar,
    /// Cylindrical (r, θ, z)
    Cylindrical,
    /// Spherical (r, θ, φ)
    Spherical,
}

impl CoordinateSystem {
    /// Get the labels for each coordinate
    pub fn labels(&self) -> Vec<&'static str> {
        match self {
            CoordinateSystem::Cartesian2D => vec!["x", "y"],
            CoordinateSystem::Cartesian3D => vec!["x", "y", "z"],
            CoordinateSystem::Polar => vec!["r", "θ"],
            CoordinateSystem::Cylindrical => vec!["r", "θ", "z"],
            CoordinateSystem::Spherical => vec!["r", "θ", "φ"],
        }
    }

    /// Get the number of dimensions
    pub fn dimensions(&self) -> usize {
        match self {
            CoordinateSystem::Cartesian2D | CoordinateSystem::Polar => 2,
            CoordinateSystem::Cartesian3D
            | CoordinateSystem::Cylindrical
            | CoordinateSystem::Spherical => 3,
        }
    }

    /// Check if a coordinate is an angle (for display purposes)
    pub fn is_angle(&self, index: usize) -> bool {
        match self {
            CoordinateSystem::Cartesian2D | CoordinateSystem::Cartesian3D => false,
            CoordinateSystem::Polar => index == 1,       // θ
            CoordinateSystem::Cylindrical => index == 1, // θ
            CoordinateSystem::Spherical => index == 1 || index == 2, // θ, φ
        }
    }
}

/// Angle unit for coordinate display
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum CoordAngleUnit {
    #[default]
    Degrees,
    Radians,
}

impl CoordAngleUnit {
    /// Convert from radians to this unit
    pub fn from_radians(&self, radians: f64) -> f64 {
        match self {
            CoordAngleUnit::Degrees => radians * 180.0 / PI,
            CoordAngleUnit::Radians => radians,
        }
    }

    /// Convert to radians from this unit
    pub fn to_radians(&self, value: f64) -> f64 {
        match self {
            CoordAngleUnit::Degrees => value * PI / 180.0,
            CoordAngleUnit::Radians => value,
        }
    }

    /// Get the unit symbol
    pub fn symbol(&self) -> &'static str {
        match self {
            CoordAngleUnit::Degrees => "°",
            CoordAngleUnit::Radians => "rad",
        }
    }
}

/// Represents coordinates in any supported system
#[derive(Clone, Debug, PartialEq)]
pub struct Coordinates {
    /// The coordinate values
    pub values: Vec<f64>,
    /// The coordinate system
    pub system: CoordinateSystem,
}

impl Default for Coordinates {
    fn default() -> Self {
        Self {
            values: vec![0.0, 0.0, 0.0],
            system: CoordinateSystem::Cartesian3D,
        }
    }
}

impl Coordinates {
    /// Create new coordinates
    pub fn new(values: Vec<f64>, system: CoordinateSystem) -> Self {
        Self { values, system }
    }

    /// Create 2D Cartesian coordinates
    pub fn cartesian_2d(x: f64, y: f64) -> Self {
        Self::new(vec![x, y], CoordinateSystem::Cartesian2D)
    }

    /// Create 3D Cartesian coordinates
    pub fn cartesian_3d(x: f64, y: f64, z: f64) -> Self {
        Self::new(vec![x, y, z], CoordinateSystem::Cartesian3D)
    }

    /// Create polar coordinates (r, θ in radians)
    pub fn polar(r: f64, theta: f64) -> Self {
        Self::new(vec![r, theta], CoordinateSystem::Polar)
    }

    /// Create cylindrical coordinates (r, θ in radians, z)
    pub fn cylindrical(r: f64, theta: f64, z: f64) -> Self {
        Self::new(vec![r, theta, z], CoordinateSystem::Cylindrical)
    }

    /// Create spherical coordinates (r, θ in radians, φ in radians)
    pub fn spherical(r: f64, theta: f64, phi: f64) -> Self {
        Self::new(vec![r, theta, phi], CoordinateSystem::Spherical)
    }

    /// Get a coordinate value by index
    pub fn get(&self, index: usize) -> Option<f64> {
        self.values.get(index).copied()
    }

    /// Set a coordinate value by index
    pub fn set(&mut self, index: usize, value: f64) {
        if index < self.values.len() {
            self.values[index] = value;
        }
    }

    /// Convert to 3D Cartesian coordinates
    pub fn to_cartesian_3d(&self) -> Coordinates {
        match self.system {
            CoordinateSystem::Cartesian2D => {
                let x = self.values.first().copied().unwrap_or(0.0);
                let y = self.values.get(1).copied().unwrap_or(0.0);
                Coordinates::cartesian_3d(x, y, 0.0)
            }
            CoordinateSystem::Cartesian3D => self.clone(),
            CoordinateSystem::Polar => {
                let r = self.values.first().copied().unwrap_or(0.0);
                let theta = self.values.get(1).copied().unwrap_or(0.0);
                let x = r * theta.cos();
                let y = r * theta.sin();
                Coordinates::cartesian_3d(x, y, 0.0)
            }
            CoordinateSystem::Cylindrical => {
                let r = self.values.first().copied().unwrap_or(0.0);
                let theta = self.values.get(1).copied().unwrap_or(0.0);
                let z = self.values.get(2).copied().unwrap_or(0.0);
                let x = r * theta.cos();
                let y = r * theta.sin();
                Coordinates::cartesian_3d(x, y, z)
            }
            CoordinateSystem::Spherical => {
                let r = self.values.first().copied().unwrap_or(0.0);
                let theta = self.values.get(1).copied().unwrap_or(0.0); // azimuthal
                let phi = self.values.get(2).copied().unwrap_or(0.0); // polar/elevation
                let x = r * phi.sin() * theta.cos();
                let y = r * phi.sin() * theta.sin();
                let z = r * phi.cos();
                Coordinates::cartesian_3d(x, y, z)
            }
        }
    }

    /// Convert from 3D Cartesian to another system
    pub fn from_cartesian_3d(x: f64, y: f64, z: f64, target: CoordinateSystem) -> Coordinates {
        match target {
            CoordinateSystem::Cartesian2D => Coordinates::cartesian_2d(x, y),
            CoordinateSystem::Cartesian3D => Coordinates::cartesian_3d(x, y, z),
            CoordinateSystem::Polar => {
                let r = (x * x + y * y).sqrt();
                let theta = y.atan2(x);
                Coordinates::polar(r, theta)
            }
            CoordinateSystem::Cylindrical => {
                let r = (x * x + y * y).sqrt();
                let theta = y.atan2(x);
                Coordinates::cylindrical(r, theta, z)
            }
            CoordinateSystem::Spherical => {
                let r = (x * x + y * y + z * z).sqrt();
                let theta = y.atan2(x); // azimuthal
                let phi = if r > 0.0 { (z / r).acos() } else { 0.0 }; // polar
                Coordinates::spherical(r, theta, phi)
            }
        }
    }

    /// Convert to another coordinate system
    pub fn convert_to(&self, target: CoordinateSystem) -> Coordinates {
        if self.system == target {
            return self.clone();
        }
        let cartesian = self.to_cartesian_3d();
        Coordinates::from_cartesian_3d(
            cartesian.values[0],
            cartesian.values[1],
            cartesian.values[2],
            target,
        )
    }

    /// Get the magnitude (distance from origin)
    pub fn magnitude(&self) -> f64 {
        let cartesian = self.to_cartesian_3d();
        let x = cartesian.values[0];
        let y = cartesian.values[1];
        let z = cartesian.values[2];
        (x * x + y * y + z * z).sqrt()
    }

    /// Format coordinate as string
    pub fn to_string_with_unit(&self, angle_unit: CoordAngleUnit, precision: usize) -> String {
        let labels = self.system.labels();
        let parts: Vec<String> = self
            .values
            .iter()
            .enumerate()
            .map(|(i, &v)| {
                let label = labels.get(i).unwrap_or(&"?");
                if self.system.is_angle(i) {
                    let value = angle_unit.from_radians(v);
                    format!(
                        "{}={:.prec$}{}",
                        label,
                        value,
                        angle_unit.symbol(),
                        prec = precision
                    )
                } else {
                    format!("{}={:.prec$}", label, v, prec = precision)
                }
            })
            .collect();
        format!("({})", parts.join(", "))
    }
}

/// Format a number for display
fn format_coord_number(value: f64, precision: usize) -> String {
    format!("{:.prec$}", value, prec = precision)
}

/// Coordinate input component
#[component]
pub fn CoordinateInput(
    /// Current coordinate value
    #[prop(optional, into)]
    value: Option<RwSignal<Coordinates>>,

    /// Callback when coordinates change
    #[prop(optional, into)]
    on_change: Option<Callback<Coordinates>>,

    /// Coordinate system to use
    #[prop(optional)]
    system: CoordinateSystem,

    /// Angle unit for display
    #[prop(optional)]
    angle_unit: CoordAngleUnit,

    /// Allow conversion between systems
    #[prop(optional, default = true)]
    allow_conversion: bool,

    /// Number of decimal places
    #[prop(optional, default = 4)]
    precision: usize,

    /// Input variant
    #[prop(optional)]
    _variant: Option<InputVariant>,

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
    let current_system = RwSignal::new(system);
    let internal_value = value
        .unwrap_or_else(|| RwSignal::new(Coordinates::new(vec![0.0; system.dimensions()], system)));

    // Input signals for each coordinate
    let coord_inputs: Vec<RwSignal<String>> = (0..3)
        .map(|i| {
            let v = internal_value.get_untracked();
            let val = v.values.get(i).copied().unwrap_or(0.0);
            let display_val = if v.system.is_angle(i) {
                angle_unit.from_radians(val)
            } else {
                val
            };
            RwSignal::new(format_coord_number(display_val, precision))
        })
        .collect();

    // Update coordinates when inputs change
    let update_coords = move |index: usize, new_value: String| {
        if let Ok(parsed) = new_value.parse::<f64>() {
            let sys = current_system.get();
            let mut coords = internal_value.get();

            // Convert from display unit to internal (radians for angles)
            let internal_value_num = if sys.is_angle(index) {
                angle_unit.to_radians(parsed)
            } else {
                parsed
            };

            if index < coords.values.len() {
                coords.values[index] = internal_value_num;
            }
            coords.system = sys;
            internal_value.set(coords.clone());

            if let Some(cb) = on_change {
                cb.run(coords);
            }
        }
    };

    // Clone coord_inputs for use in closures
    let coord_inputs_for_system = coord_inputs.clone();
    let coord_inputs_for_view = coord_inputs.clone();

    // Handle system change
    let change_system = Callback::new(move |new_system: CoordinateSystem| {
        let current = internal_value.get();
        let converted = current.convert_to(new_system);

        // Update input displays
        for (i, input) in coord_inputs_for_system.iter().enumerate() {
            let val = converted.values.get(i).copied().unwrap_or(0.0);
            let display_val = if new_system.is_angle(i) {
                angle_unit.from_radians(val)
            } else {
                val
            };
            input.set(format_coord_number(display_val, precision));
        }

        current_system.set(new_system);
        internal_value.set(converted.clone());

        if let Some(cb) = on_change {
            cb.run(converted);
        }
    });

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

    let coords_row_styles = move || {
        StyleBuilder::new()
            .add("display", "flex")
            .add("align-items", "center")
            .add("gap", "0.5rem")
            .add("flex-wrap", "wrap")
            .build()
    };

    let coord_group_styles = move || {
        StyleBuilder::new()
            .add("display", "flex")
            .add("align-items", "center")
            .add("gap", "0.25rem")
            .build()
    };

    let coord_label_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("font-size", theme_val.typography.font_sizes.sm)
            .add(
                "font-weight",
                theme_val.typography.font_weights.medium.to_string(),
            )
            .add("color", scheme_colors.text.clone())
            .add("min-width", "1rem")
            .build()
    };

    let input_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let size_padding = match size.unwrap_or(InputSize::Md) {
            InputSize::Xs => "0.25rem 0.5rem",
            InputSize::Sm => "0.375rem 0.75rem",
            InputSize::Md => "0.5rem 1rem",
            InputSize::Lg => "0.625rem 1.25rem",
            InputSize::Xl => "0.75rem 1.5rem",
        };

        StyleBuilder::new()
            .add("padding", size_padding)
            .add(
                "border",
                format!("1px solid {}", scheme_colors.border.clone()),
            )
            .add("border-radius", theme_val.radius.sm)
            .add("background", scheme_colors.background.clone())
            .add("color", scheme_colors.text.clone())
            .add("font-size", theme_val.typography.font_sizes.sm)
            .add("width", "80px")
            .add("text-align", "right")
            .build()
    };

    let unit_label_styles = move || {
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

    let system_selector_styles = move || {
        let theme_val = theme.get();
        StyleBuilder::new()
            .add("display", "flex")
            .add("gap", "0.25rem")
            .add("margin-top", theme_val.spacing.xs)
            .add("flex-wrap", "wrap")
            .build()
    };

    let system_button_styles = move |is_active: bool| {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("padding", "0.25rem 0.5rem")
            .add(
                "border",
                format!("1px solid {}", scheme_colors.border.clone()),
            )
            .add("border-radius", theme_val.radius.sm)
            .add(
                "background",
                if is_active {
                    scheme_colors
                        .get_color(&theme_val.colors.primary_color, 6)
                        .unwrap_or_else(|| "#228be6".to_string())
                } else {
                    scheme_colors.background.clone()
                },
            )
            .add(
                "color",
                if is_active {
                    "#ffffff".to_string()
                } else {
                    scheme_colors.text.clone()
                },
            )
            .add("cursor", "pointer")
            .add("font-size", theme_val.typography.font_sizes.xs)
            .build()
    };

    let preview_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("font-size", theme_val.typography.font_sizes.sm)
            .add(
                "color",
                scheme_colors
                    .get_color("gray", 6)
                    .unwrap_or_else(|| "#868e96".to_string()),
            )
            .add("margin-top", theme_val.spacing.xs)
            .add("font-family", "monospace")
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
            .add("margin-top", theme_val.spacing.xs)
            .build()
    };

    let all_systems = vec![
        (CoordinateSystem::Cartesian2D, "2D (x,y)"),
        (CoordinateSystem::Cartesian3D, "3D (x,y,z)"),
        (CoordinateSystem::Polar, "Polar (r,θ)"),
        (CoordinateSystem::Cylindrical, "Cylindrical (r,θ,z)"),
        (CoordinateSystem::Spherical, "Spherical (r,θ,φ)"),
    ];

    view! {
        <div class="mingot-coordinate-input" style=container_styles>
            {label.clone().map(|l| view! {
                <label style=label_styles>{l}</label>
            })}

            <div style=coords_row_styles>
                {move || {
                    let sys = current_system.get();
                    let labels = sys.labels();
                    let dims = sys.dimensions();

                    (0..dims).map(|i| {
                        let label_text = labels.get(i).unwrap_or(&"?").to_string();
                        let is_angle = sys.is_angle(i);
                        let input_signal = coord_inputs_for_view[i];

                        view! {
                            <div style=coord_group_styles>
                                <span style=coord_label_styles>{label_text}{"="}</span>
                                <input
                                    type="text"
                                    style=input_styles
                                    prop:value=move || input_signal.get()
                                    disabled=disabled
                                    on:input=move |ev| {
                                        let val = event_target_value(&ev);
                                        input_signal.set(val.clone());
                                        update_coords(i, val);
                                    }
                                />
                                {is_angle.then(|| view! {
                                    <span style=unit_label_styles>{angle_unit.symbol()}</span>
                                })}
                            </div>
                        }
                    }).collect_view()
                }}
            </div>

            {allow_conversion.then(|| {
                let systems = all_systems.clone();
                view! {
                    <div style=system_selector_styles>
                        {systems.into_iter().map(|(sys, name)| {
                            view! {
                                <button
                                    type="button"
                                    style=move || system_button_styles(current_system.get() == sys)
                                    on:click=move |_| change_system.run(sys)
                                    disabled=disabled
                                >
                                    {name}
                                </button>
                            }
                        }).collect_view()}
                    </div>
                }
            })}

            <div style=preview_styles>
                {move || {
                    let coords = internal_value.get();
                    format!("Magnitude: {:.prec$}", coords.magnitude(), prec = precision)
                }}
            </div>

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
    use std::f64::consts::PI;

    #[test]
    fn test_cartesian_2d() {
        let coords = Coordinates::cartesian_2d(3.0, 4.0);
        assert_eq!(coords.magnitude(), 5.0);
    }

    #[test]
    fn test_cartesian_3d() {
        let coords = Coordinates::cartesian_3d(1.0, 2.0, 2.0);
        assert_eq!(coords.magnitude(), 3.0);
    }

    #[test]
    fn test_polar_to_cartesian() {
        let polar = Coordinates::polar(1.0, PI / 2.0);
        let cartesian = polar.to_cartesian_3d();
        assert!((cartesian.values[0] - 0.0).abs() < 1e-10);
        assert!((cartesian.values[1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_cartesian_to_polar() {
        let cartesian = Coordinates::cartesian_2d(1.0, 1.0);
        let polar = cartesian.convert_to(CoordinateSystem::Polar);
        assert!((polar.values[0] - 2.0_f64.sqrt()).abs() < 1e-10);
        assert!((polar.values[1] - PI / 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_cylindrical_to_cartesian() {
        let cylindrical = Coordinates::cylindrical(1.0, 0.0, 5.0);
        let cartesian = cylindrical.to_cartesian_3d();
        assert!((cartesian.values[0] - 1.0).abs() < 1e-10);
        assert!((cartesian.values[1] - 0.0).abs() < 1e-10);
        assert!((cartesian.values[2] - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_spherical_to_cartesian() {
        let spherical = Coordinates::spherical(1.0, 0.0, PI / 2.0);
        let cartesian = spherical.to_cartesian_3d();
        assert!((cartesian.values[0] - 1.0).abs() < 1e-10);
        assert!((cartesian.values[1] - 0.0).abs() < 1e-10);
        assert!((cartesian.values[2] - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_angle_unit_conversion() {
        assert!((CoordAngleUnit::Degrees.from_radians(PI) - 180.0).abs() < 1e-10);
        assert!((CoordAngleUnit::Degrees.to_radians(180.0) - PI).abs() < 1e-10);
    }

    #[test]
    fn test_coordinate_system_labels() {
        assert_eq!(CoordinateSystem::Cartesian2D.labels(), vec!["x", "y"]);
        assert_eq!(CoordinateSystem::Cartesian3D.labels(), vec!["x", "y", "z"]);
        assert_eq!(CoordinateSystem::Polar.labels(), vec!["r", "θ"]);
    }

    #[test]
    fn test_coordinate_system_is_angle() {
        assert!(!CoordinateSystem::Cartesian3D.is_angle(0));
        assert!(CoordinateSystem::Polar.is_angle(1));
        assert!(CoordinateSystem::Spherical.is_angle(1));
        assert!(CoordinateSystem::Spherical.is_angle(2));
    }

    #[test]
    fn test_round_trip_conversion() {
        let original = Coordinates::cartesian_3d(1.0, 2.0, 3.0);
        let spherical = original.convert_to(CoordinateSystem::Spherical);
        let back = spherical.convert_to(CoordinateSystem::Cartesian3D);

        assert!((original.values[0] - back.values[0]).abs() < 1e-10);
        assert!((original.values[1] - back.values[1]).abs() < 1e-10);
        assert!((original.values[2] - back.values[2]).abs() < 1e-10);
    }
}
