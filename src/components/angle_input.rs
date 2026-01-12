//! AngleInput - High-precision angle entry with multiple unit support
//!
//! Supports degrees, radians, gradians, turns, and DMS (degrees-minutes-seconds) format.
//! Features automatic conversion between units and optional normalization.

use crate::components::input::{InputSize, InputVariant};
use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;
use std::f64::consts::PI;

/// Angle unit types
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AngleUnit {
    #[default]
    Degrees,
    Radians,
    Gradians,
    Turns,
    /// Degrees-Minutes-Seconds format (e.g., 45°30'15")
    DMS,
}

impl AngleUnit {
    /// Get the display suffix for this unit
    pub fn suffix(&self) -> &'static str {
        match self {
            AngleUnit::Degrees => "°",
            AngleUnit::Radians => " rad",
            AngleUnit::Gradians => " grad",
            AngleUnit::Turns => " turns",
            AngleUnit::DMS => "",
        }
    }

    /// Get the full name of this unit
    pub fn name(&self) -> &'static str {
        match self {
            AngleUnit::Degrees => "Degrees",
            AngleUnit::Radians => "Radians",
            AngleUnit::Gradians => "Gradians",
            AngleUnit::Turns => "Turns",
            AngleUnit::DMS => "DMS",
        }
    }
}

/// Normalization mode for angles
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum AngleNormalization {
    /// No normalization, allow any value
    #[default]
    None,
    /// Normalize to 0° to 360° (or equivalent in other units)
    ZeroTo360,
    /// Normalize to -180° to 180° (or equivalent in other units)
    NegativeTo180,
}

/// Represents a parsed DMS (Degrees-Minutes-Seconds) value
#[derive(Clone, Debug, PartialEq)]
pub struct DMS {
    pub degrees: i32,
    pub minutes: u32,
    pub seconds: f64,
    pub negative: bool,
}

impl DMS {
    /// Create a new DMS value
    pub fn new(degrees: i32, minutes: u32, seconds: f64) -> Self {
        let negative = degrees < 0;
        Self {
            degrees: degrees.abs(),
            minutes,
            seconds,
            negative,
        }
    }

    /// Convert to decimal degrees
    pub fn to_degrees(&self) -> f64 {
        let value = self.degrees as f64 + (self.minutes as f64 / 60.0) + (self.seconds / 3600.0);
        if self.negative {
            -value
        } else {
            value
        }
    }

    /// Create from decimal degrees
    pub fn from_degrees(degrees: f64) -> Self {
        let negative = degrees < 0.0;
        let abs_degrees = degrees.abs();

        let d = abs_degrees.floor() as i32;
        let remaining = (abs_degrees - d as f64) * 60.0;
        let m = remaining.floor() as u32;
        let s = (remaining - m as f64) * 60.0;

        Self {
            degrees: d,
            minutes: m,
            seconds: s,
            negative,
        }
    }
}

impl std::fmt::Display for DMS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = if self.negative { "-" } else { "" };
        if self.seconds == 0.0 && self.minutes == 0 {
            write!(f, "{}{}°", sign, self.degrees)
        } else if self.seconds == 0.0 {
            write!(f, "{}{}°{}'", sign, self.degrees, self.minutes)
        } else if self.seconds.fract() == 0.0 {
            write!(
                f,
                "{}{}°{}'{}\"",
                sign, self.degrees, self.minutes, self.seconds as i32
            )
        } else {
            write!(
                f,
                "{}{}°{}'{:.2}\"",
                sign, self.degrees, self.minutes, self.seconds
            )
        }
    }
}

/// Parse a DMS string (e.g., "45°30'15\"" or "45d30m15s")
fn parse_dms(input: &str) -> Option<DMS> {
    let trimmed = input.trim();

    // Handle negative
    let (negative, value) = if let Some(stripped) = trimmed.strip_prefix('-') {
        (true, stripped)
    } else {
        (false, trimmed)
    };

    // Try different formats
    // Format: 45°30'15" or 45°30'15.5"
    if let Some(result) = parse_dms_symbols(value) {
        return Some(DMS { negative, ..result });
    }

    // Format: 45d30m15s or 45d30m15.5s
    if let Some(result) = parse_dms_letters(value) {
        return Some(DMS { negative, ..result });
    }

    // Format: 45 30 15 (space separated)
    if let Some(result) = parse_dms_spaces(value) {
        return Some(DMS { negative, ..result });
    }

    None
}

fn parse_dms_symbols(input: &str) -> Option<DMS> {
    // Match patterns like 45°30'15" or 45°30'15.5"
    let parts: Vec<&str> = input.split(['°', '\'', '"', '′', '″']).collect();

    if parts.is_empty() {
        return None;
    }

    let degrees: i32 = parts.first()?.trim().parse().ok()?;
    let minutes: u32 = parts
        .get(1)
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);
    let seconds: f64 = parts
        .get(2)
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0.0);

    Some(DMS {
        degrees,
        minutes,
        seconds,
        negative: false,
    })
}

fn parse_dms_letters(input: &str) -> Option<DMS> {
    // Match patterns like 45d30m15s
    let lower = input.to_lowercase();
    let parts: Vec<&str> = lower.split(['d', 'm', 's']).collect();

    if parts.is_empty() {
        return None;
    }

    let degrees: i32 = parts.first()?.trim().parse().ok()?;
    let minutes: u32 = parts
        .get(1)
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0);
    let seconds: f64 = parts
        .get(2)
        .and_then(|s| s.trim().parse().ok())
        .unwrap_or(0.0);

    Some(DMS {
        degrees,
        minutes,
        seconds,
        negative: false,
    })
}

fn parse_dms_spaces(input: &str) -> Option<DMS> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return None;
    }

    let degrees: i32 = parts.first()?.parse().ok()?;
    let minutes: u32 = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
    let seconds: f64 = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0.0);

    Some(DMS {
        degrees,
        minutes,
        seconds,
        negative: false,
    })
}

/// Convert angle from one unit to degrees
fn to_degrees(value: f64, from_unit: AngleUnit) -> f64 {
    match from_unit {
        AngleUnit::Degrees | AngleUnit::DMS => value,
        AngleUnit::Radians => value * 180.0 / PI,
        AngleUnit::Gradians => value * 0.9, // 360/400
        AngleUnit::Turns => value * 360.0,
    }
}

/// Convert angle from degrees to another unit
fn from_degrees(degrees: f64, to_unit: AngleUnit) -> f64 {
    match to_unit {
        AngleUnit::Degrees | AngleUnit::DMS => degrees,
        AngleUnit::Radians => degrees * PI / 180.0,
        AngleUnit::Gradians => degrees / 0.9, // 400/360
        AngleUnit::Turns => degrees / 360.0,
    }
}

/// Normalize an angle in degrees
fn normalize_degrees(degrees: f64, mode: AngleNormalization) -> f64 {
    match mode {
        AngleNormalization::None => degrees,
        AngleNormalization::ZeroTo360 => {
            let normalized = degrees % 360.0;
            if normalized < 0.0 {
                normalized + 360.0
            } else {
                normalized
            }
        }
        AngleNormalization::NegativeTo180 => {
            let mut normalized = degrees % 360.0;
            if normalized > 180.0 {
                normalized -= 360.0;
            } else if normalized < -180.0 {
                normalized += 360.0;
            }
            normalized
        }
    }
}

/// Format a numeric value with appropriate precision
fn format_angle_value(value: f64, unit: AngleUnit, precision: u32) -> String {
    match unit {
        AngleUnit::DMS => DMS::from_degrees(value).to_string(),
        _ => {
            if precision == 0 {
                format!("{:.0}", value)
            } else {
                format!("{:.prec$}", value, prec = precision as usize)
            }
        }
    }
}

/// Parse an angle string to degrees
fn parse_angle_to_degrees(input: &str, unit: AngleUnit) -> Option<f64> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return None;
    }

    match unit {
        AngleUnit::DMS => parse_dms(trimmed).map(|dms| dms.to_degrees()),
        _ => {
            // Remove any unit suffixes
            let cleaned = trimmed
                .trim_end_matches("°")
                .trim_end_matches("rad")
                .trim_end_matches("grad")
                .trim_end_matches("turns")
                .trim_end_matches("turn")
                .trim();

            cleaned.parse::<f64>().ok().map(|v| to_degrees(v, unit))
        }
    }
}

/// AngleInput component for high-precision angle entry
#[component]
pub fn AngleInput(
    /// Current angle value in degrees (internal representation)
    #[prop(optional)]
    value: Option<RwSignal<f64>>,

    /// Callback when value changes
    #[prop(optional)]
    on_change: Option<Callback<f64>>,

    /// The unit to display/edit in
    #[prop(default = AngleUnit::Degrees)]
    unit: AngleUnit,

    /// Decimal precision for display (ignored for DMS)
    #[prop(default = 2)]
    precision: u32,

    /// Whether to wrap/normalize the angle
    #[prop(optional)]
    normalization: Option<AngleNormalization>,

    /// Whether to show unit selector dropdown
    #[prop(default = false)]
    show_unit_selector: bool,

    /// Callback when unit changes (if show_unit_selector is true)
    #[prop(optional)]
    on_unit_change: Option<Callback<AngleUnit>>,

    /// Input variant styling
    #[prop(optional)]
    variant: Option<InputVariant>,

    /// Input size
    #[prop(optional)]
    size: Option<InputSize>,

    /// Placeholder text
    #[prop(optional, into)]
    placeholder: Option<String>,

    /// Whether input is disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,

    /// Error message
    #[prop(optional, into)]
    error: Option<String>,

    /// Whether field is required
    #[prop(optional)]
    required: bool,

    /// Label text
    #[prop(optional, into)]
    label: Option<String>,

    /// Description text
    #[prop(optional, into)]
    description: Option<String>,

    /// Additional CSS class
    #[prop(optional, into)]
    class: Option<String>,

    /// Additional inline styles
    #[prop(optional, into)]
    style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let variant = variant.unwrap_or(InputVariant::Default);
    let size = size.unwrap_or(InputSize::Md);
    let normalization = normalization.unwrap_or_default();

    // Internal value in degrees
    let angle_value = value.unwrap_or_else(|| RwSignal::new(0.0));

    // Current display unit (can be changed via selector)
    let current_unit = RwSignal::new(unit);

    // Text representation for editing
    let display_text = RwSignal::new(String::new());

    // Track if user is actively editing
    let is_editing = RwSignal::new(false);

    // Initialize display text from value
    Effect::new(move || {
        if !is_editing.get() {
            let degrees = angle_value.get();
            let unit = current_unit.get();
            let converted = from_degrees(degrees, unit);
            display_text.set(format_angle_value(converted, unit, precision));
        }
    });

    // Handle input changes
    let handle_input = move |ev: ev::Event| {
        let input_value = event_target_value(&ev);
        display_text.set(input_value);
    };

    // Handle focus
    let handle_focus = move |_ev: ev::FocusEvent| {
        is_editing.set(true);
    };

    // Handle blur - parse and validate
    let handle_blur = move |_ev: ev::FocusEvent| {
        is_editing.set(false);

        let text = display_text.get();
        let unit = current_unit.get();

        if let Some(degrees) = parse_angle_to_degrees(&text, unit) {
            let normalized = normalize_degrees(degrees, normalization);
            angle_value.set(normalized);

            if let Some(callback) = on_change {
                callback.run(normalized);
            }

            // Update display with formatted value
            let converted = from_degrees(normalized, unit);
            display_text.set(format_angle_value(converted, unit, precision));
        } else if !text.is_empty() {
            // Invalid input - revert to previous value
            let degrees = angle_value.get();
            let converted = from_degrees(degrees, unit);
            display_text.set(format_angle_value(converted, unit, precision));
        }
    };

    // Handle unit change
    let handle_unit_change = move |new_unit: AngleUnit| {
        let old_unit = current_unit.get();
        if old_unit != new_unit {
            current_unit.set(new_unit);

            // Convert display value to new unit
            let degrees = angle_value.get();
            let converted = from_degrees(degrees, new_unit);
            display_text.set(format_angle_value(converted, new_unit, precision));

            if let Some(callback) = on_unit_change {
                callback.run(new_unit);
            }
        }
    };

    // Clone error for use in multiple closures
    let error_for_style = error.clone();
    let error_for_display = error.clone();

    // Styles
    let input_wrapper_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        let (height, font_size, padding): (&str, &str, &str) = match size {
            InputSize::Xs => ("1.625rem", theme_val.typography.font_sizes.xs, "0 0.5rem"),
            InputSize::Sm => ("1.875rem", theme_val.typography.font_sizes.sm, "0 0.625rem"),
            InputSize::Md => ("2.25rem", theme_val.typography.font_sizes.sm, "0 0.75rem"),
            InputSize::Lg => ("2.625rem", theme_val.typography.font_sizes.md, "0 1rem"),
            InputSize::Xl => ("3rem", theme_val.typography.font_sizes.lg, "0 1.25rem"),
        };

        let border_color = if error_for_style.is_some() {
            scheme_colors
                .get_color("red", 6)
                .unwrap_or_else(|| "#fa5252".to_string())
        } else {
            scheme_colors
                .get_color("gray", 4)
                .unwrap_or_else(|| "#ced4da".to_string())
        };

        let bg_color = match variant {
            InputVariant::Default => scheme_colors
                .get_color("white", 0)
                .unwrap_or_else(|| "#ffffff".to_string()),
            InputVariant::Filled => scheme_colors
                .get_color("gray", 1)
                .unwrap_or_else(|| "#f1f3f5".to_string()),
            InputVariant::Unstyled => "transparent".to_string(),
        };

        StyleBuilder::new()
            .add("display", "flex")
            .add("align-items", "center")
            .add("height", height)
            .add("font-size", font_size)
            .add("padding", padding)
            .add("background-color", bg_color)
            .add("border", format!("1px solid {}", border_color))
            .add("border-radius", theme_val.radius.sm.to_owned())
            .add("transition", "border-color 150ms ease")
            .add_if(disabled.get(), "opacity", "0.6")
            .add_if(disabled.get(), "cursor", "not-allowed")
            .build()
    };

    let input_styles = move || {
        StyleBuilder::new()
            .add("flex", "1")
            .add("border", "none")
            .add("background", "transparent")
            .add("outline", "none")
            .add("font-family", "inherit")
            .add("font-size", "inherit")
            .add("color", "inherit")
            .add("min-width", "0")
            .build()
    };

    let suffix_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        StyleBuilder::new()
            .add(
                "color",
                scheme_colors
                    .get_color("gray", 6)
                    .unwrap_or_else(|| "#868e96".to_string()),
            )
            .add("margin-left", "0.25rem")
            .add("user-select", "none")
            .build()
    };

    let label_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "display: block; margin-bottom: 0.25rem; font-size: {}; font-weight: {}; color: {};",
            theme_val.typography.font_sizes.sm,
            theme_val.typography.font_weights.medium,
            scheme_colors.text
        )
    };

    let description_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "margin-top: 0.25rem; font-size: {}; color: {};",
            theme_val.typography.font_sizes.xs,
            scheme_colors
                .get_color("gray", 6)
                .unwrap_or_else(|| "#868e96".to_string())
        )
    };

    let error_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "margin-top: 0.25rem; font-size: {}; color: {};",
            theme_val.typography.font_sizes.xs,
            scheme_colors
                .get_color("red", 6)
                .unwrap_or_else(|| "#fa5252".to_string())
        )
    };

    let container_styles = style.clone().unwrap_or_default();
    let container_class = class.clone().unwrap_or_default();

    // Unit options for selector
    let unit_options = [
        AngleUnit::Degrees,
        AngleUnit::Radians,
        AngleUnit::Gradians,
        AngleUnit::Turns,
        AngleUnit::DMS,
    ];

    view! {
        <div class=container_class style=container_styles>
            {label.clone().map(|l| view! {
                <label style=label_styles>
                    {l}
                    {required.then(|| view! { <span style="color: #fa5252; margin-left: 0.25rem;">"*"</span> })}
                </label>
            })}

            <div style=input_wrapper_styles>
                <input
                    type="text"
                    style=input_styles
                    placeholder=placeholder.clone().unwrap_or_else(|| {
                        match current_unit.get() {
                            AngleUnit::DMS => "45°30'15\"".to_string(),
                            AngleUnit::Degrees => "0.00".to_string(),
                            AngleUnit::Radians => "0.00".to_string(),
                            AngleUnit::Gradians => "0.00".to_string(),
                            AngleUnit::Turns => "0.00".to_string(),
                        }
                    })
                    prop:value=move || display_text.get()
                    prop:disabled=move || disabled.get()
                    on:input=handle_input
                    on:focus=handle_focus
                    on:blur=handle_blur
                />

                {move || {
                    if show_unit_selector {
                        let current = current_unit.get();
                        view! {
                            <select
                                style="border: none; background: transparent; cursor: pointer; font-size: inherit; color: inherit; padding: 0 0.25rem;"
                                on:change=move |ev| {
                                    let value = event_target_value(&ev);
                                    let new_unit = match value.as_str() {
                                        "degrees" => AngleUnit::Degrees,
                                        "radians" => AngleUnit::Radians,
                                        "gradians" => AngleUnit::Gradians,
                                        "turns" => AngleUnit::Turns,
                                        "dms" => AngleUnit::DMS,
                                        _ => AngleUnit::Degrees,
                                    };
                                    handle_unit_change(new_unit);
                                }
                            >
                                {unit_options.iter().map(|u| {
                                    let value = match u {
                                        AngleUnit::Degrees => "degrees",
                                        AngleUnit::Radians => "radians",
                                        AngleUnit::Gradians => "gradians",
                                        AngleUnit::Turns => "turns",
                                        AngleUnit::DMS => "dms",
                                    };
                                    let is_selected = *u == current;
                                    view! {
                                        <option value=value selected=is_selected>
                                            {u.suffix()}
                                        </option>
                                    }
                                }).collect_view()}
                            </select>
                        }.into_any()
                    } else {
                        view! {
                            <span style=suffix_styles>
                                {move || current_unit.get().suffix()}
                            </span>
                        }.into_any()
                    }
                }}
            </div>

            {description.clone().map(|d| view! {
                <div style=description_styles>{d}</div>
            })}

            {error_for_display.clone().map(|e| view! {
                <div style=error_styles role="alert">{e}</div>
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dms_to_degrees() {
        let dms = DMS::new(45, 30, 0.0);
        assert!((dms.to_degrees() - 45.5).abs() < 0.0001);

        let dms = DMS::new(90, 0, 0.0);
        assert!((dms.to_degrees() - 90.0).abs() < 0.0001);

        let dms = DMS::new(-45, 30, 0.0);
        assert!((dms.to_degrees() - (-45.5)).abs() < 0.0001);
    }

    #[test]
    fn test_degrees_to_dms() {
        let dms = DMS::from_degrees(45.5);
        assert_eq!(dms.degrees, 45);
        assert_eq!(dms.minutes, 30);
        assert!((dms.seconds - 0.0).abs() < 0.01);

        let dms = DMS::from_degrees(45.508333);
        assert_eq!(dms.degrees, 45);
        assert_eq!(dms.minutes, 30);
        assert!((dms.seconds - 30.0).abs() < 0.1);
    }

    #[test]
    fn test_parse_dms() {
        // Symbol format
        let dms = parse_dms("45°30'15\"").unwrap();
        assert_eq!(dms.degrees, 45);
        assert_eq!(dms.minutes, 30);
        assert!((dms.seconds - 15.0).abs() < 0.01);

        // Letter format
        let dms = parse_dms("45d30m15s").unwrap();
        assert_eq!(dms.degrees, 45);
        assert_eq!(dms.minutes, 30);

        // Space format
        let dms = parse_dms("45 30 15").unwrap();
        assert_eq!(dms.degrees, 45);
        assert_eq!(dms.minutes, 30);

        // Negative
        let dms = parse_dms("-45°30'").unwrap();
        assert!(dms.negative);
        assert_eq!(dms.degrees, 45);
    }

    #[test]
    fn test_unit_conversions() {
        // Degrees to radians
        let radians = from_degrees(180.0, AngleUnit::Radians);
        assert!((radians - PI).abs() < 0.0001);

        // Radians to degrees
        let degrees = to_degrees(PI, AngleUnit::Radians);
        assert!((degrees - 180.0).abs() < 0.0001);

        // Degrees to turns
        let turns = from_degrees(360.0, AngleUnit::Turns);
        assert!((turns - 1.0).abs() < 0.0001);

        // Degrees to gradians
        let gradians = from_degrees(90.0, AngleUnit::Gradians);
        assert!((gradians - 100.0).abs() < 0.0001);
    }

    #[test]
    fn test_normalization() {
        // Zero to 360
        assert!((normalize_degrees(450.0, AngleNormalization::ZeroTo360) - 90.0).abs() < 0.0001);
        assert!((normalize_degrees(-90.0, AngleNormalization::ZeroTo360) - 270.0).abs() < 0.0001);

        // Negative to 180
        assert!(
            (normalize_degrees(270.0, AngleNormalization::NegativeTo180) - (-90.0)).abs() < 0.0001
        );
        assert!(
            (normalize_degrees(-270.0, AngleNormalization::NegativeTo180) - 90.0).abs() < 0.0001
        );

        // None
        assert!((normalize_degrees(450.0, AngleNormalization::None) - 450.0).abs() < 0.0001);
    }

    #[test]
    fn test_format_angle_value() {
        assert_eq!(format_angle_value(45.5, AngleUnit::Degrees, 2), "45.50");
        assert_eq!(format_angle_value(45.0, AngleUnit::Degrees, 0), "45");

        let dms_str = format_angle_value(45.5, AngleUnit::DMS, 2);
        assert!(dms_str.contains("45°"));
        assert!(dms_str.contains("30'"));
    }

    #[test]
    fn test_parse_angle_to_degrees() {
        assert!((parse_angle_to_degrees("90", AngleUnit::Degrees).unwrap() - 90.0).abs() < 0.0001);
        assert!((parse_angle_to_degrees("90°", AngleUnit::Degrees).unwrap() - 90.0).abs() < 0.0001);

        let pi_degrees = parse_angle_to_degrees("3.14159", AngleUnit::Radians).unwrap();
        assert!((pi_degrees - 180.0).abs() < 0.01);

        let dms_degrees = parse_angle_to_degrees("45°30'", AngleUnit::DMS).unwrap();
        assert!((dms_degrees - 45.5).abs() < 0.0001);
    }
}
