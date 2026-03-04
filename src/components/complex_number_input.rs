//! ComplexNumberInput component for entering and manipulating complex numbers.
//!
//! This component provides a specialized input for complex numbers with support
//! for rectangular (a + bi) and polar (r∠θ) forms, along with format conversion.

use leptos::prelude::*;
use leptos::tachys::html::event as ev;

use crate::theme::use_theme;
use crate::utils::style_builder::StyleBuilder;

/// Display format for complex numbers
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ComplexFormat {
    /// Rectangular form: a + bi
    #[default]
    Rectangular,
    /// Polar form: r∠θ
    Polar,
    /// Exponential form: r·e^(iθ)
    Exponential,
}

/// Angle unit for polar form
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PolarAngleUnit {
    #[default]
    Degrees,
    Radians,
}

/// Size variants for the input
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ComplexInputSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

/// Represents a complex number
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ComplexNumber {
    /// Real part
    pub real: f64,
    /// Imaginary part
    pub imaginary: f64,
}

impl Default for ComplexNumber {
    fn default() -> Self {
        Self {
            real: 0.0,
            imaginary: 0.0,
        }
    }
}

impl ComplexNumber {
    /// Create a new complex number from rectangular coordinates
    pub fn new(real: f64, imaginary: f64) -> Self {
        Self { real, imaginary }
    }

    /// Create a complex number from polar coordinates
    pub fn from_polar(magnitude: f64, angle_radians: f64) -> Self {
        Self {
            real: magnitude * angle_radians.cos(),
            imaginary: magnitude * angle_radians.sin(),
        }
    }

    /// Get the magnitude (modulus/absolute value)
    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imaginary * self.imaginary).sqrt()
    }

    /// Get the angle (argument/phase) in radians
    pub fn angle(&self) -> f64 {
        self.imaginary.atan2(self.real)
    }

    /// Get the angle in degrees
    pub fn angle_degrees(&self) -> f64 {
        self.angle().to_degrees()
    }

    /// Get the complex conjugate
    pub fn conjugate(&self) -> Self {
        Self {
            real: self.real,
            imaginary: -self.imaginary,
        }
    }

    /// Add two complex numbers
    pub fn add(&self, other: &Self) -> Self {
        Self {
            real: self.real + other.real,
            imaginary: self.imaginary + other.imaginary,
        }
    }

    /// Subtract two complex numbers
    pub fn sub(&self, other: &Self) -> Self {
        Self {
            real: self.real - other.real,
            imaginary: self.imaginary - other.imaginary,
        }
    }

    /// Multiply two complex numbers
    pub fn mul(&self, other: &Self) -> Self {
        Self {
            real: self.real * other.real - self.imaginary * other.imaginary,
            imaginary: self.real * other.imaginary + self.imaginary * other.real,
        }
    }

    /// Divide two complex numbers
    pub fn div(&self, other: &Self) -> Option<Self> {
        let denom = other.real * other.real + other.imaginary * other.imaginary;
        if denom == 0.0 {
            None
        } else {
            Some(Self {
                real: (self.real * other.real + self.imaginary * other.imaginary) / denom,
                imaginary: (self.imaginary * other.real - self.real * other.imaginary) / denom,
            })
        }
    }

    /// Check if this is a real number (imaginary part is zero)
    pub fn is_real(&self) -> bool {
        self.imaginary.abs() < f64::EPSILON
    }

    /// Check if this is a pure imaginary number (real part is zero)
    pub fn is_imaginary(&self) -> bool {
        self.real.abs() < f64::EPSILON
    }

    /// Format as rectangular string (a + bi)
    pub fn to_rectangular_string(&self) -> String {
        let sign = if self.imaginary >= 0.0 { "+" } else { "-" };
        format!("{} {} {}i", self.real, sign, self.imaginary.abs())
    }

    /// Format as polar string (r∠θ)
    pub fn to_polar_string(&self, degrees: bool) -> String {
        let magnitude = self.magnitude();
        let angle = if degrees {
            self.angle_degrees()
        } else {
            self.angle()
        };
        let unit = if degrees { "°" } else { " rad" };
        format!("{}∠{}{}", magnitude, angle, unit)
    }

    /// Format as exponential string (r·e^(iθ))
    pub fn to_exponential_string(&self) -> String {
        let magnitude = self.magnitude();
        let angle = self.angle();
        format!("{}·e^({}i)", magnitude, angle)
    }
}

impl std::fmt::Display for ComplexNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_rectangular_string())
    }
}

/// Parse a complex number from string (rectangular form)
fn parse_rectangular(input: &str) -> Option<ComplexNumber> {
    let input = input.trim().to_lowercase();

    // Handle pure imaginary: "5i" or "-3i" or "i"
    if input.ends_with('i') && !input.contains('+') && !input.contains('-') {
        let imag_str = input.trim_end_matches('i');
        if imag_str.is_empty() {
            return Some(ComplexNumber::new(0.0, 1.0));
        }
        if let Ok(imag) = imag_str.parse::<f64>() {
            return Some(ComplexNumber::new(0.0, imag));
        }
    }

    // Handle "a + bi" or "a - bi" format
    // Split on + or - but keep the sign with the imaginary part
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut chars = input.chars().peekable();

    // Handle leading sign
    if chars.peek() == Some(&'-') || chars.peek() == Some(&'+') {
        current.push(chars.next().unwrap());
    }

    for ch in chars {
        if (ch == '+' || ch == '-') && !current.is_empty() {
            parts.push(current.trim().to_string());
            current = String::from(ch);
        } else {
            current.push(ch);
        }
    }
    if !current.is_empty() {
        parts.push(current.trim().to_string());
    }

    if parts.is_empty() {
        return None;
    }

    let mut real = 0.0;
    let mut imaginary = 0.0;

    for part in parts {
        let part = part.trim();
        if part.ends_with('i') {
            // Remove 'i' and all spaces for parsing
            let imag_str = part.trim_end_matches('i').replace(' ', "");
            let imag_str = imag_str.trim();
            if imag_str.is_empty() || imag_str == "+" {
                imaginary = 1.0;
            } else if imag_str == "-" {
                imaginary = -1.0;
            } else {
                imaginary = imag_str.parse().ok()?;
            }
        } else if !part.is_empty() {
            // Remove spaces for parsing
            let real_str = part.replace(' ', "");
            real = real_str.parse().ok()?;
        }
    }

    Some(ComplexNumber::new(real, imaginary))
}

/// Parse a complex number from polar form (r∠θ or r<θ)
fn parse_polar(input: &str, angle_in_degrees: bool) -> Option<ComplexNumber> {
    let input = input.trim();

    // Find the angle symbol (∠ or <)
    let parts: Vec<&str> = if input.contains('∠') {
        input.split('∠').collect()
    } else if input.contains('<') {
        input.split('<').collect()
    } else {
        return None;
    };

    if parts.len() != 2 {
        return None;
    }

    let magnitude: f64 = parts[0].trim().parse().ok()?;
    let angle_str = parts[1].trim().trim_end_matches(['°', 'd', 'r', 'a']);
    let angle_val: f64 = angle_str.trim().parse().ok()?;

    let angle_radians = if angle_in_degrees || parts[1].contains('°') || parts[1].contains('d') {
        angle_val.to_radians()
    } else {
        angle_val
    };

    Some(ComplexNumber::from_polar(magnitude, angle_radians))
}

/// Parse a complex number from any supported format
#[allow(dead_code)]
fn parse_complex(input: &str, angle_in_degrees: bool) -> Option<ComplexNumber> {
    let input = input.trim();

    // Try polar form first
    if input.contains('∠') || input.contains('<') {
        return parse_polar(input, angle_in_degrees);
    }

    // Try rectangular form
    parse_rectangular(input)
}

/// Format a number with specified decimal places
fn format_value(value: f64, decimals: u32) -> String {
    format!("{:.1$}", value, decimals as usize)
}

/// ComplexNumberInput component properties
#[component]
pub fn ComplexNumberInput(
    /// Current complex number value
    #[prop(optional, into)]
    value: Option<Signal<ComplexNumber>>,
    /// Default value if not controlled
    #[prop(default = ComplexNumber::default())]
    default_value: ComplexNumber,
    /// Callback when value changes
    #[prop(optional, into)]
    on_change: Option<Callback<ComplexNumber>>,
    /// Display format
    #[prop(default = ComplexFormat::Rectangular)]
    format: ComplexFormat,
    /// Angle unit for polar display
    #[prop(default = PolarAngleUnit::Degrees)]
    angle_unit: PolarAngleUnit,
    /// Allow format switching
    #[prop(default = true)]
    allow_format_switch: bool,
    /// Number of decimal places for display
    #[prop(default = 4)]
    decimal_places: u32,
    /// Label for the input
    #[prop(optional, into)]
    label: Option<String>,
    /// Description text
    #[prop(optional, into)]
    description: Option<String>,
    /// Error message
    #[prop(optional, into)]
    error: Option<Signal<Option<String>>>,
    /// Input size
    #[prop(default = ComplexInputSize::Md)]
    size: ComplexInputSize,
    /// Is the input disabled?
    #[prop(default = false)]
    disabled: bool,
    /// Is the input required?
    #[prop(default = false)]
    required: bool,
    /// Placeholder for real part
    #[prop(default = "Real".into())]
    real_placeholder: String,
    /// Placeholder for imaginary part
    #[prop(default = "Imaginary".into())]
    imaginary_placeholder: String,
    /// Additional CSS class
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme = use_theme();

    // Internal state
    let complex_value = RwSignal::new(value.map_or(default_value, |v| v.get()));
    let current_format = RwSignal::new(format);
    let has_error = RwSignal::new(false);

    // Input state for rectangular form
    let real_input = RwSignal::new(format_value(complex_value.get().real, decimal_places));
    let imag_input = RwSignal::new(format_value(complex_value.get().imaginary, decimal_places));

    // Input state for polar form
    let magnitude_input = RwSignal::new(format_value(
        complex_value.get().magnitude(),
        decimal_places,
    ));
    let angle_input = RwSignal::new(format_value(
        if angle_unit == PolarAngleUnit::Degrees {
            complex_value.get().angle_degrees()
        } else {
            complex_value.get().angle()
        },
        decimal_places,
    ));

    // Sync with external value
    if let Some(ext_value) = value {
        Effect::new(move || {
            let v = ext_value.get();
            complex_value.set(v);
            real_input.set(format_value(v.real, decimal_places));
            imag_input.set(format_value(v.imaginary, decimal_places));
            magnitude_input.set(format_value(v.magnitude(), decimal_places));
            let angle_val = if angle_unit == PolarAngleUnit::Degrees {
                v.angle_degrees()
            } else {
                v.angle()
            };
            angle_input.set(format_value(angle_val, decimal_places));
        });
    }

    // Update value from rectangular inputs
    let update_from_rectangular = move || {
        let real_str = real_input.get();
        let imag_str = imag_input.get();

        if let (Ok(real), Ok(imag)) = (real_str.parse::<f64>(), imag_str.parse::<f64>()) {
            let new_value = ComplexNumber::new(real, imag);
            complex_value.set(new_value);
            magnitude_input.set(format_value(new_value.magnitude(), decimal_places));
            let angle_val = if angle_unit == PolarAngleUnit::Degrees {
                new_value.angle_degrees()
            } else {
                new_value.angle()
            };
            angle_input.set(format_value(angle_val, decimal_places));
            has_error.set(false);
            if let Some(cb) = on_change {
                cb.run(new_value);
            }
        } else {
            has_error.set(true);
        }
    };

    // Update value from polar inputs
    let update_from_polar = move || {
        let mag_str = magnitude_input.get();
        let ang_str = angle_input.get();

        if let (Ok(magnitude), Ok(angle)) = (mag_str.parse::<f64>(), ang_str.parse::<f64>()) {
            let angle_radians = if angle_unit == PolarAngleUnit::Degrees {
                angle.to_radians()
            } else {
                angle
            };
            let new_value = ComplexNumber::from_polar(magnitude, angle_radians);
            complex_value.set(new_value);
            real_input.set(format_value(new_value.real, decimal_places));
            imag_input.set(format_value(new_value.imaginary, decimal_places));
            has_error.set(false);
            if let Some(cb) = on_change {
                cb.run(new_value);
            }
        } else {
            has_error.set(true);
        }
    };

    // Handle format switch
    let handle_format_switch = move |new_format: ComplexFormat| {
        current_format.set(new_format);
    };

    // Clone error for use in closures
    let error_for_display = error;

    // Styles
    let wrapper_styles = move || {
        StyleBuilder::new()
            .add("display", "flex")
            .add("flex-direction", "column")
            .add("gap", "0.25rem")
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
            .add("margin-bottom", "0.25rem")
            .build()
    };

    let input_row_styles = move || {
        StyleBuilder::new()
            .add("display", "flex")
            .add("gap", "0.5rem")
            .add("align-items", "center")
            .build()
    };

    let input_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        let (height, font_size, padding): (&str, &str, &str) = match size {
            ComplexInputSize::Xs => ("1.625rem", theme_val.typography.font_sizes.xs, "0 0.5rem"),
            ComplexInputSize::Sm => ("1.875rem", theme_val.typography.font_sizes.sm, "0 0.625rem"),
            ComplexInputSize::Md => ("2.25rem", theme_val.typography.font_sizes.sm, "0 0.75rem"),
            ComplexInputSize::Lg => ("2.625rem", theme_val.typography.font_sizes.md, "0 1rem"),
            ComplexInputSize::Xl => ("3rem", theme_val.typography.font_sizes.lg, "0 1.25rem"),
        };

        let has_validation_error =
            has_error.get() || error_for_display.is_some_and(|e| e.get().is_some());
        let border_color = if has_validation_error {
            scheme_colors
                .get_color("red", 6)
                .unwrap_or_else(|| "#fa5252".to_string())
        } else {
            scheme_colors
                .get_color("gray", 4)
                .unwrap_or_else(|| "#ced4da".to_string())
        };

        StyleBuilder::new()
            .add("flex", "1")
            .add("min-width", "0")
            .add("padding", padding)
            .add("font-size", font_size)
            .add("height", height)
            .add("border", format!("1px solid {}", border_color))
            .add("border-radius", theme_val.radius.sm.to_owned())
            .add("background-color", scheme_colors.background.clone())
            .add("color", scheme_colors.text.clone())
            .add("outline", "none")
            .add("transition", "border-color 0.15s, box-shadow 0.15s")
            .add_if(disabled, "opacity", "0.6")
            .add_if(disabled, "cursor", "not-allowed")
            .build()
    };

    let operator_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let (font_size, _padding, _height): (&str, &str, &str) = match size {
            ComplexInputSize::Xs => ("0.625rem", "0.25rem", "1.5rem"),
            ComplexInputSize::Sm => ("0.75rem", "0.375rem", "1.75rem"),
            ComplexInputSize::Md => ("0.875rem", "0.5rem", "2.25rem"),
            ComplexInputSize::Lg => ("1rem", "0.625rem", "2.5rem"),
            ComplexInputSize::Xl => ("1.125rem", "0.75rem", "2.75rem"),
        };

        StyleBuilder::new()
            .add("font-size", font_size)
            .add(
                "color",
                scheme_colors
                    .get_color("gray", 6)
                    .unwrap_or_else(|| "#868e96".to_string()),
            )
            .add("font-weight", "500")
            .build()
    };

    let format_switch_styles = move || {
        StyleBuilder::new()
            .add("display", "flex")
            .add("gap", "0.25rem")
            .add("margin-top", "0.25rem")
            .build()
    };

    let format_btn_styles = move |is_active: bool| {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        let primary_color = scheme_colors
            .get_color(&theme_val.colors.primary_color, 6)
            .unwrap_or_else(|| "#228be6".to_string());
        let border_color = scheme_colors
            .get_color("gray", 4)
            .unwrap_or_else(|| "#ced4da".to_string());

        StyleBuilder::new()
            .add("padding", "0.25rem 0.5rem")
            .add("font-size", "0.75rem")
            .add("border-radius", theme_val.radius.sm.to_owned())
            .add("border", "1px solid")
            .add(
                "border-color",
                if is_active {
                    primary_color.clone()
                } else {
                    border_color
                },
            )
            .add(
                "background-color",
                if is_active {
                    primary_color
                } else {
                    "transparent".to_string()
                },
            )
            .add(
                "color",
                if is_active {
                    "white".to_string()
                } else {
                    scheme_colors
                        .get_color("gray", 6)
                        .unwrap_or_else(|| "#868e96".to_string())
                },
            )
            .add("cursor", "pointer")
            .add("transition", "all 0.15s")
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
            .add("margin-top", "0.25rem")
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
            .add("margin-top", "0.25rem")
            .build()
    };

    let info_styles = move || {
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
            .add("margin-top", "0.5rem")
            .add("display", "flex")
            .add("flex-wrap", "wrap")
            .add("gap", "1rem")
            .build()
    };

    view! {
        <div style=wrapper_styles class=class>
            {label.clone().map(|l| view! {
                <label style=label_styles>
                    {l}
                    {if required { view! { <span style="color: red;"> "*"</span> }.into_any() } else { ().into_any() }}
                </label>
            })}

            {move || match current_format.get() {
                ComplexFormat::Rectangular => {
                    let real_handler = move |ev: ev::Event| {
                        real_input.set(event_target_value(&ev));
                    };
                    let imag_handler = move |ev: ev::Event| {
                        imag_input.set(event_target_value(&ev));
                    };
                    let blur_handler = move |_| {
                        update_from_rectangular();
                    };
                    let real_placeholder_clone = real_placeholder.clone();
                    let imaginary_placeholder_clone = imaginary_placeholder.clone();
                    view! {
                        <div style=input_row_styles()>
                            <input
                                type="text"
                                style=input_styles()
                                value=move || real_input.get()
                                on:input=real_handler
                                on:blur=blur_handler
                                placeholder=real_placeholder_clone
                                disabled=disabled
                            />
                            <span style=operator_styles()>"+"</span>
                            <input
                                type="text"
                                style=input_styles()
                                value=move || imag_input.get()
                                on:input=imag_handler
                                on:blur=blur_handler
                                placeholder=imaginary_placeholder_clone
                                disabled=disabled
                            />
                            <span style=operator_styles()>"i"</span>
                        </div>
                    }.into_any()
                }
                ComplexFormat::Polar | ComplexFormat::Exponential => {
                    let mag_handler = move |ev: ev::Event| {
                        magnitude_input.set(event_target_value(&ev));
                    };
                    let ang_handler = move |ev: ev::Event| {
                        angle_input.set(event_target_value(&ev));
                    };
                    let blur_handler = move |_| {
                        update_from_polar();
                    };
                    let angle_symbol = if angle_unit == PolarAngleUnit::Degrees { "°" } else { " rad" };
                    view! {
                        <div style=input_row_styles()>
                            <input
                                type="text"
                                style=input_styles()
                                value=move || magnitude_input.get()
                                on:input=mag_handler
                                on:blur=blur_handler
                                placeholder="Magnitude"
                                disabled=disabled
                            />
                            <span style=operator_styles()>"∠"</span>
                            <input
                                type="text"
                                style=input_styles()
                                value=move || angle_input.get()
                                on:input=ang_handler
                                on:blur=blur_handler
                                placeholder="Angle"
                                disabled=disabled
                            />
                            <span style=operator_styles()>{angle_symbol}</span>
                        </div>
                    }.into_any()
                }
            }}

            {allow_format_switch.then(|| {
                view! {
                    <div style=format_switch_styles>
                        <button
                            type="button"
                            style=move || format_btn_styles(current_format.get() == ComplexFormat::Rectangular)
                            on:click=move |_| handle_format_switch(ComplexFormat::Rectangular)
                            disabled=disabled
                        >
                            "a + bi"
                        </button>
                        <button
                            type="button"
                            style=move || format_btn_styles(current_format.get() == ComplexFormat::Polar)
                            on:click=move |_| handle_format_switch(ComplexFormat::Polar)
                            disabled=disabled
                        >
                            "r∠θ"
                        </button>
                        <button
                            type="button"
                            style=move || format_btn_styles(current_format.get() == ComplexFormat::Exponential)
                            on:click=move |_| handle_format_switch(ComplexFormat::Exponential)
                            disabled=disabled
                        >
                            "re^iθ"
                        </button>
                    </div>
                }
            })}

            {description.clone().map(|d| view! {
                <div style=description_styles>{d}</div>
            })}

            {move || {
                let error_msg = if has_error.get() {
                    Some("Invalid input".to_string())
                } else {
                    error_for_display.and_then(|e| e.get())
                };
                error_msg.map(|e| view! {
                    <div style=error_styles()>{e}</div>
                })
            }}

            // Display additional info
            <div style=info_styles>
                <span>{move || format!("|z| = {}", format_value(complex_value.get().magnitude(), decimal_places))}</span>
                <span>{move || {
                    let angle = if angle_unit == PolarAngleUnit::Degrees {
                        complex_value.get().angle_degrees()
                    } else {
                        complex_value.get().angle()
                    };
                    let unit = if angle_unit == PolarAngleUnit::Degrees { "°" } else { " rad" };
                    format!("arg(z) = {}{}", format_value(angle, decimal_places), unit)
                }}</span>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_complex_new() {
        let c = ComplexNumber::new(3.0, 4.0);
        assert_eq!(c.real, 3.0);
        assert_eq!(c.imaginary, 4.0);
    }

    #[test]
    fn test_complex_magnitude() {
        let c = ComplexNumber::new(3.0, 4.0);
        assert!((c.magnitude() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_angle() {
        let c = ComplexNumber::new(1.0, 1.0);
        assert!((c.angle() - PI / 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_from_polar() {
        let c = ComplexNumber::from_polar(1.0, PI / 2.0);
        assert!(c.real.abs() < 1e-10);
        assert!((c.imaginary - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_add() {
        let a = ComplexNumber::new(1.0, 2.0);
        let b = ComplexNumber::new(3.0, 4.0);
        let c = a.add(&b);
        assert_eq!(c.real, 4.0);
        assert_eq!(c.imaginary, 6.0);
    }

    #[test]
    fn test_complex_sub() {
        let a = ComplexNumber::new(5.0, 7.0);
        let b = ComplexNumber::new(3.0, 4.0);
        let c = a.sub(&b);
        assert_eq!(c.real, 2.0);
        assert_eq!(c.imaginary, 3.0);
    }

    #[test]
    fn test_complex_mul() {
        let a = ComplexNumber::new(1.0, 2.0);
        let b = ComplexNumber::new(3.0, 4.0);
        let c = a.mul(&b);
        // (1+2i)(3+4i) = 3 + 4i + 6i + 8i² = 3 + 10i - 8 = -5 + 10i
        assert_eq!(c.real, -5.0);
        assert_eq!(c.imaginary, 10.0);
    }

    #[test]
    fn test_complex_div() {
        let a = ComplexNumber::new(1.0, 2.0);
        let b = ComplexNumber::new(3.0, 4.0);
        let c = a.div(&b).unwrap();
        // (1+2i)/(3+4i) = (1+2i)(3-4i)/((3+4i)(3-4i)) = (3-4i+6i-8i²)/(9+16)
        // = (3+2i+8)/25 = (11+2i)/25 = 0.44 + 0.08i
        assert!((c.real - 0.44).abs() < 1e-10);
        assert!((c.imaginary - 0.08).abs() < 1e-10);
    }

    #[test]
    fn test_complex_div_by_zero() {
        let a = ComplexNumber::new(1.0, 2.0);
        let b = ComplexNumber::new(0.0, 0.0);
        assert!(a.div(&b).is_none());
    }

    #[test]
    fn test_complex_conjugate() {
        let c = ComplexNumber::new(3.0, 4.0);
        let conj = c.conjugate();
        assert_eq!(conj.real, 3.0);
        assert_eq!(conj.imaginary, -4.0);
    }

    #[test]
    fn test_complex_is_real() {
        let c = ComplexNumber::new(5.0, 0.0);
        assert!(c.is_real());
        let c2 = ComplexNumber::new(5.0, 1.0);
        assert!(!c2.is_real());
    }

    #[test]
    fn test_complex_is_imaginary() {
        let c = ComplexNumber::new(0.0, 5.0);
        assert!(c.is_imaginary());
        let c2 = ComplexNumber::new(1.0, 5.0);
        assert!(!c2.is_imaginary());
    }

    #[test]
    fn test_parse_rectangular_simple() {
        let c = parse_rectangular("3 + 4i").unwrap();
        assert_eq!(c.real, 3.0);
        assert_eq!(c.imaginary, 4.0);
    }

    #[test]
    fn test_parse_rectangular_negative() {
        let c = parse_rectangular("3 - 4i").unwrap();
        assert_eq!(c.real, 3.0);
        assert_eq!(c.imaginary, -4.0);
    }

    #[test]
    fn test_parse_rectangular_pure_imaginary() {
        let c = parse_rectangular("5i").unwrap();
        assert_eq!(c.real, 0.0);
        assert_eq!(c.imaginary, 5.0);
    }

    #[test]
    fn test_parse_rectangular_just_i() {
        let c = parse_rectangular("i").unwrap();
        assert_eq!(c.real, 0.0);
        assert_eq!(c.imaginary, 1.0);
    }

    #[test]
    fn test_parse_polar() {
        let c = parse_polar("5∠90°", true).unwrap();
        assert!(c.real.abs() < 1e-10);
        assert!((c.imaginary - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_parse_polar_radians() {
        let c = parse_polar(&format!("1∠{}", PI / 2.0), false).unwrap();
        assert!(c.real.abs() < 1e-10);
        assert!((c.imaginary - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_complex_display() {
        let c = ComplexNumber::new(3.0, 4.0);
        assert_eq!(c.to_string(), "3 + 4i");
    }

    #[test]
    fn test_complex_polar_string() {
        let c = ComplexNumber::new(0.0, 5.0);
        let polar = c.to_polar_string(true);
        assert!(polar.starts_with("5"));
        assert!(polar.contains("90"));
    }
}
