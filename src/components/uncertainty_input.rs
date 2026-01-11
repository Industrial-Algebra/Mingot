//! UncertaintyInput component for entering and manipulating values with uncertainty.
//!
//! This component provides a specialized input for values with uncertainty bounds,
//! commonly used in scientific measurements. Supports symmetric (±) and asymmetric
//! uncertainty, with display options for absolute or relative uncertainty.

use leptos::prelude::*;
use leptos::tachys::html::event as ev;

use crate::theme::use_theme;
use crate::utils::style_builder::StyleBuilder;

/// Display format for uncertainty
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UncertaintyFormat {
    /// Absolute uncertainty: value ± uncertainty
    #[default]
    Absolute,
    /// Relative/percentage uncertainty: value ± x%
    Relative,
    /// Scientific notation: (value ± uncertainty) × 10^n
    Scientific,
}

/// Type of uncertainty
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UncertaintyType {
    /// Symmetric: same upper and lower bounds (±)
    #[default]
    Symmetric,
    /// Asymmetric: different upper and lower bounds (+upper/-lower)
    Asymmetric,
}

/// Size variants for the input
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum UncertaintyInputSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

/// Represents a value with uncertainty
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UncertainValue {
    /// Central/measured value
    pub value: f64,
    /// Upper uncertainty bound (positive direction)
    pub upper_uncertainty: f64,
    /// Lower uncertainty bound (positive value, represents negative direction)
    pub lower_uncertainty: f64,
}

impl Default for UncertainValue {
    fn default() -> Self {
        Self {
            value: 0.0,
            upper_uncertainty: 0.0,
            lower_uncertainty: 0.0,
        }
    }
}

impl UncertainValue {
    /// Create a new value with symmetric uncertainty
    pub fn symmetric(value: f64, uncertainty: f64) -> Self {
        Self {
            value,
            upper_uncertainty: uncertainty.abs(),
            lower_uncertainty: uncertainty.abs(),
        }
    }

    /// Create a new value with asymmetric uncertainty
    pub fn asymmetric(value: f64, upper: f64, lower: f64) -> Self {
        Self {
            value,
            upper_uncertainty: upper.abs(),
            lower_uncertainty: lower.abs(),
        }
    }

    /// Check if uncertainty is symmetric
    pub fn is_symmetric(&self) -> bool {
        (self.upper_uncertainty - self.lower_uncertainty).abs() < f64::EPSILON
    }

    /// Get the relative uncertainty as a fraction (0-1)
    pub fn relative_uncertainty(&self) -> f64 {
        if self.value.abs() < f64::EPSILON {
            0.0
        } else {
            // Use average of upper and lower for relative calculation
            ((self.upper_uncertainty + self.lower_uncertainty) / 2.0) / self.value.abs()
        }
    }

    /// Get the relative uncertainty as a percentage
    pub fn percentage_uncertainty(&self) -> f64 {
        self.relative_uncertainty() * 100.0
    }

    /// Get the upper bound of the value
    pub fn upper_bound(&self) -> f64 {
        self.value + self.upper_uncertainty
    }

    /// Get the lower bound of the value
    pub fn lower_bound(&self) -> f64 {
        self.value - self.lower_uncertainty
    }

    /// Get the range (difference between upper and lower bounds)
    pub fn range(&self) -> f64 {
        self.upper_uncertainty + self.lower_uncertainty
    }

    /// Format as symmetric string: value ± uncertainty
    pub fn to_symmetric_string(&self, decimals: u32) -> String {
        let avg_uncertainty = (self.upper_uncertainty + self.lower_uncertainty) / 2.0;
        format!(
            "{:.prec$} ± {:.prec$}",
            self.value,
            avg_uncertainty,
            prec = decimals as usize
        )
    }

    /// Format as asymmetric string: value +upper/-lower
    pub fn to_asymmetric_string(&self, decimals: u32) -> String {
        format!(
            "{:.prec$} +{:.prec$}/−{:.prec$}",
            self.value,
            self.upper_uncertainty,
            self.lower_uncertainty,
            prec = decimals as usize
        )
    }

    /// Format as percentage string: value ± x%
    pub fn to_percentage_string(&self, decimals: u32) -> String {
        format!(
            "{:.prec$} ± {:.1}%",
            self.value,
            self.percentage_uncertainty(),
            prec = decimals as usize
        )
    }

    /// Format in scientific notation: (value ± uncertainty) × 10^n
    pub fn to_scientific_string(&self, sig_figs: u32) -> String {
        if self.value.abs() < f64::EPSILON {
            return format!(
                "(0 ± {:.prec$}) × 10^0",
                self.upper_uncertainty,
                prec = sig_figs as usize
            );
        }

        let exponent = self.value.abs().log10().floor() as i32;
        let mantissa = self.value / 10_f64.powi(exponent);
        let uncertainty_mantissa =
            (self.upper_uncertainty + self.lower_uncertainty) / 2.0 / 10_f64.powi(exponent);

        format!(
            "({:.prec$} ± {:.prec$}) × 10^{}",
            mantissa,
            uncertainty_mantissa,
            exponent,
            prec = sig_figs as usize
        )
    }

    /// Create from absolute uncertainty
    pub fn from_absolute(value: f64, uncertainty: f64) -> Self {
        Self::symmetric(value, uncertainty)
    }

    /// Create from relative/percentage uncertainty
    pub fn from_relative(value: f64, relative_uncertainty: f64) -> Self {
        let absolute = value.abs() * relative_uncertainty;
        Self::symmetric(value, absolute)
    }

    /// Create from percentage uncertainty
    pub fn from_percentage(value: f64, percentage: f64) -> Self {
        Self::from_relative(value, percentage / 100.0)
    }
}

impl std::fmt::Display for UncertainValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_symmetric() {
            write!(f, "{}", self.to_symmetric_string(4))
        } else {
            write!(f, "{}", self.to_asymmetric_string(4))
        }
    }
}

/// Parse a symmetric uncertainty string: "value ± uncertainty"
#[allow(dead_code)]
fn parse_symmetric(input: &str) -> Option<UncertainValue> {
    let input = input.trim();

    // Try ± symbol
    if let Some((value_str, uncertainty_str)) = input.split_once('±') {
        let value: f64 = value_str.trim().parse().ok()?;
        let uncertainty_str = uncertainty_str.trim();

        // Check for percentage
        if uncertainty_str.ends_with('%') {
            let percentage: f64 = uncertainty_str.trim_end_matches('%').trim().parse().ok()?;
            return Some(UncertainValue::from_percentage(value, percentage));
        }

        let uncertainty: f64 = uncertainty_str.parse().ok()?;
        return Some(UncertainValue::symmetric(value, uncertainty));
    }

    // Try +/- format
    if let Some((value_str, uncertainty_str)) = input.split_once("+/-") {
        let value: f64 = value_str.trim().parse().ok()?;
        let uncertainty: f64 = uncertainty_str.trim().parse().ok()?;
        return Some(UncertainValue::symmetric(value, uncertainty));
    }

    None
}

/// Parse an asymmetric uncertainty string: "value +upper/-lower"
#[allow(dead_code)]
fn parse_asymmetric(input: &str) -> Option<UncertainValue> {
    let input = input.trim();

    // Find the pattern: value +upper/-lower or value +upper/−lower
    // First, try to find +number/-number or +number/−number at the end
    let re_pattern = |s: &str| -> Option<(f64, f64, f64)> {
        // Find the last occurrence of a pattern like +X/-Y or +X/−Y
        let parts: Vec<&str> = s.splitn(2, '+').collect();
        if parts.len() != 2 {
            return None;
        }

        let value: f64 = parts[0].trim().parse().ok()?;
        let rest = parts[1];

        // Split on / or /−
        let (upper_str, lower_str) = if rest.contains("/−") {
            rest.split_once("/−")?
        } else if rest.contains("/-") {
            rest.split_once("/-")?
        } else {
            return None;
        };

        let upper: f64 = upper_str.trim().parse().ok()?;
        let lower: f64 = lower_str.trim().parse().ok()?;

        Some((value, upper, lower))
    };

    re_pattern(input).map(|(value, upper, lower)| UncertainValue::asymmetric(value, upper, lower))
}

/// Parse an uncertain value from any supported format
#[allow(dead_code)]
fn parse_uncertain_value(input: &str) -> Option<UncertainValue> {
    // Try symmetric first
    if let Some(v) = parse_symmetric(input) {
        return Some(v);
    }

    // Try asymmetric
    if let Some(v) = parse_asymmetric(input) {
        return Some(v);
    }

    // Try just a plain number (no uncertainty)
    if let Ok(value) = input.trim().parse::<f64>() {
        return Some(UncertainValue::symmetric(value, 0.0));
    }

    None
}

/// Format a number with specified decimal places
fn format_value(value: f64, decimals: u32) -> String {
    format!("{:.1$}", value, decimals as usize)
}

/// UncertaintyInput component properties
#[component]
pub fn UncertaintyInput(
    /// Current uncertain value
    #[prop(optional, into)]
    value: Option<Signal<UncertainValue>>,
    /// Default value if not controlled
    #[prop(default = UncertainValue::default())]
    default_value: UncertainValue,
    /// Callback when value changes
    #[prop(optional, into)]
    on_change: Option<Callback<UncertainValue>>,
    /// Display format
    #[prop(default = UncertaintyFormat::Absolute)]
    format: UncertaintyFormat,
    /// Uncertainty type (symmetric or asymmetric)
    #[prop(default = UncertaintyType::Symmetric)]
    uncertainty_type: UncertaintyType,
    /// Allow switching between symmetric and asymmetric
    #[prop(default = true)]
    allow_type_switch: bool,
    /// Number of decimal places
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
    #[prop(default = UncertaintyInputSize::Md)]
    size: UncertaintyInputSize,
    /// Is the input disabled?
    #[prop(default = false)]
    disabled: bool,
    /// Is the input required?
    #[prop(default = false)]
    required: bool,
    /// Placeholder for value
    #[prop(default = "Value".into())]
    value_placeholder: String,
    /// Placeholder for uncertainty
    #[prop(default = "Uncertainty".into())]
    uncertainty_placeholder: String,
    /// Show additional info (bounds, relative uncertainty)
    #[prop(default = true)]
    show_info: bool,
    /// Additional CSS class
    #[prop(optional, into)]
    class: Option<String>,
) -> impl IntoView {
    let theme = use_theme();

    // Internal state
    let uncertain_value = RwSignal::new(value.map_or(default_value, |v| v.get()));
    let current_type = RwSignal::new(uncertainty_type);
    let has_error = RwSignal::new(false);

    // Input state
    let value_input = RwSignal::new(format_value(uncertain_value.get().value, decimal_places));
    let upper_input = RwSignal::new(format_value(
        uncertain_value.get().upper_uncertainty,
        decimal_places,
    ));
    let lower_input = RwSignal::new(format_value(
        uncertain_value.get().lower_uncertainty,
        decimal_places,
    ));

    // Sync with external value
    if let Some(ext_value) = value {
        Effect::new(move || {
            let v = ext_value.get();
            uncertain_value.set(v);
            value_input.set(format_value(v.value, decimal_places));
            upper_input.set(format_value(v.upper_uncertainty, decimal_places));
            lower_input.set(format_value(v.lower_uncertainty, decimal_places));
        });
    }

    // Update value from inputs
    let update_value = move || {
        let val_str = value_input.get();
        let upper_str = upper_input.get();
        let lower_str = lower_input.get();

        if let Ok(val) = val_str.parse::<f64>() {
            let upper = upper_str.parse::<f64>().unwrap_or(0.0);
            let lower = if current_type.get() == UncertaintyType::Symmetric {
                upper
            } else {
                lower_str.parse::<f64>().unwrap_or(0.0)
            };

            let new_value = UncertainValue::asymmetric(val, upper, lower);
            uncertain_value.set(new_value);
            has_error.set(false);

            // If symmetric, sync the lower input
            if current_type.get() == UncertaintyType::Symmetric {
                lower_input.set(format_value(upper, decimal_places));
            }

            if let Some(cb) = on_change {
                cb.run(new_value);
            }
        } else {
            has_error.set(true);
        }
    };

    // Handle type switch
    let handle_type_switch = move |new_type: UncertaintyType| {
        current_type.set(new_type);
        if new_type == UncertaintyType::Symmetric {
            // Sync lower to upper when switching to symmetric
            lower_input.set(upper_input.get());
            update_value();
        }
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
            UncertaintyInputSize::Xs => {
                ("1.625rem", theme_val.typography.font_sizes.xs, "0 0.5rem")
            }
            UncertaintyInputSize::Sm => {
                ("1.875rem", theme_val.typography.font_sizes.sm, "0 0.625rem")
            }
            UncertaintyInputSize::Md => {
                ("2.25rem", theme_val.typography.font_sizes.sm, "0 0.75rem")
            }
            UncertaintyInputSize::Lg => ("2.625rem", theme_val.typography.font_sizes.md, "0 1rem"),
            UncertaintyInputSize::Xl => ("3rem", theme_val.typography.font_sizes.lg, "0 1.25rem"),
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
            .add(
                "background-color",
                scheme_colors
                    .get_color("white", 0)
                    .unwrap_or_else(|| "#ffffff".to_string()),
            )
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
            UncertaintyInputSize::Xs => ("0.625rem", "0.25rem", "1.5rem"),
            UncertaintyInputSize::Sm => ("0.75rem", "0.375rem", "1.75rem"),
            UncertaintyInputSize::Md => ("0.875rem", "0.5rem", "2.25rem"),
            UncertaintyInputSize::Lg => ("1rem", "0.625rem", "2.5rem"),
            UncertaintyInputSize::Xl => ("1.125rem", "0.75rem", "2.75rem"),
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

    let type_switch_styles = move || {
        StyleBuilder::new()
            .add("display", "flex")
            .add("gap", "0.25rem")
            .add("margin-top", "0.25rem")
            .build()
    };

    let type_btn_styles = move |is_active: bool| {
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

            {move || {
                let is_symmetric = current_type.get() == UncertaintyType::Symmetric;

                let value_handler = move |ev: ev::Event| {
                    value_input.set(event_target_value(&ev));
                };
                let upper_handler = move |ev: ev::Event| {
                    upper_input.set(event_target_value(&ev));
                    if current_type.get() == UncertaintyType::Symmetric {
                        lower_input.set(event_target_value(&ev));
                    }
                };
                let lower_handler = move |ev: ev::Event| {
                    lower_input.set(event_target_value(&ev));
                };
                let blur_handler = move |_| {
                    update_value();
                };

                let value_placeholder_clone = value_placeholder.clone();
                let uncertainty_placeholder_clone = uncertainty_placeholder.clone();

                if is_symmetric {
                    view! {
                        <div style=input_row_styles()>
                            <input
                                type="text"
                                style=input_styles()
                                value=move || value_input.get()
                                on:input=value_handler
                                on:blur=blur_handler
                                placeholder=value_placeholder_clone
                                disabled=disabled
                            />
                            <span style=operator_styles()>"±"</span>
                            <input
                                type="text"
                                style=input_styles()
                                value=move || upper_input.get()
                                on:input=upper_handler
                                on:blur=blur_handler
                                placeholder=uncertainty_placeholder_clone
                                disabled=disabled
                            />
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div style=input_row_styles()>
                            <input
                                type="text"
                                style=input_styles()
                                value=move || value_input.get()
                                on:input=value_handler
                                on:blur=blur_handler
                                placeholder=value_placeholder_clone
                                disabled=disabled
                            />
                            <span style=operator_styles()>"+"</span>
                            <input
                                type="text"
                                style=input_styles()
                                value=move || upper_input.get()
                                on:input=upper_handler
                                on:blur=blur_handler
                                placeholder="Upper"
                                disabled=disabled
                            />
                            <span style=operator_styles()>"/"</span>
                            <span style=operator_styles()>"−"</span>
                            <input
                                type="text"
                                style=input_styles()
                                value=move || lower_input.get()
                                on:input=lower_handler
                                on:blur=blur_handler
                                placeholder="Lower"
                                disabled=disabled
                            />
                        </div>
                    }.into_any()
                }
            }}

            {allow_type_switch.then(|| {
                view! {
                    <div style=type_switch_styles>
                        <button
                            type="button"
                            style=move || type_btn_styles(current_type.get() == UncertaintyType::Symmetric)
                            on:click=move |_| handle_type_switch(UncertaintyType::Symmetric)
                            disabled=disabled
                        >
                            "± (symmetric)"
                        </button>
                        <button
                            type="button"
                            style=move || type_btn_styles(current_type.get() == UncertaintyType::Asymmetric)
                            on:click=move |_| handle_type_switch(UncertaintyType::Asymmetric)
                            disabled=disabled
                        >
                            "+/− (asymmetric)"
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
            {show_info.then(|| {
                view! {
                    <div style=info_styles>
                        <span>{move || {
                            let v = uncertain_value.get();
                            match format {
                                UncertaintyFormat::Absolute => {
                                    if v.is_symmetric() {
                                        v.to_symmetric_string(decimal_places)
                                    } else {
                                        v.to_asymmetric_string(decimal_places)
                                    }
                                }
                                UncertaintyFormat::Relative => v.to_percentage_string(decimal_places),
                                UncertaintyFormat::Scientific => v.to_scientific_string(decimal_places),
                            }
                        }}</span>
                        <span>{move || {
                            let v = uncertain_value.get();
                            format!("Range: [{:.prec$}, {:.prec$}]", v.lower_bound(), v.upper_bound(), prec = decimal_places as usize)
                        }}</span>
                        <span>{move || {
                            let v = uncertain_value.get();
                            format!("Relative: {:.2}%", v.percentage_uncertainty())
                        }}</span>
                    </div>
                }
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uncertain_value_symmetric() {
        let v = UncertainValue::symmetric(10.0, 0.5);
        assert_eq!(v.value, 10.0);
        assert_eq!(v.upper_uncertainty, 0.5);
        assert_eq!(v.lower_uncertainty, 0.5);
        assert!(v.is_symmetric());
    }

    #[test]
    fn test_uncertain_value_asymmetric() {
        let v = UncertainValue::asymmetric(10.0, 0.5, 0.3);
        assert_eq!(v.value, 10.0);
        assert_eq!(v.upper_uncertainty, 0.5);
        assert_eq!(v.lower_uncertainty, 0.3);
        assert!(!v.is_symmetric());
    }

    #[test]
    fn test_uncertain_value_bounds() {
        let v = UncertainValue::symmetric(10.0, 0.5);
        assert_eq!(v.upper_bound(), 10.5);
        assert_eq!(v.lower_bound(), 9.5);
        assert_eq!(v.range(), 1.0);
    }

    #[test]
    fn test_uncertain_value_relative() {
        let v = UncertainValue::symmetric(100.0, 5.0);
        assert!((v.relative_uncertainty() - 0.05).abs() < 1e-10);
        assert!((v.percentage_uncertainty() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_uncertain_value_from_percentage() {
        let v = UncertainValue::from_percentage(100.0, 5.0);
        assert_eq!(v.value, 100.0);
        assert!((v.upper_uncertainty - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_uncertain_value_to_symmetric_string() {
        let v = UncertainValue::symmetric(10.0, 0.5);
        let s = v.to_symmetric_string(2);
        assert!(s.contains("10.00"));
        assert!(s.contains("0.50"));
        assert!(s.contains("±"));
    }

    #[test]
    fn test_uncertain_value_to_asymmetric_string() {
        let v = UncertainValue::asymmetric(10.0, 0.5, 0.3);
        let s = v.to_asymmetric_string(2);
        assert!(s.contains("10.00"));
        assert!(s.contains("+0.50"));
        assert!(s.contains("0.30"));
    }

    #[test]
    fn test_parse_symmetric() {
        let v = parse_symmetric("10 ± 0.5").unwrap();
        assert_eq!(v.value, 10.0);
        assert_eq!(v.upper_uncertainty, 0.5);
        assert!(v.is_symmetric());
    }

    #[test]
    fn test_parse_symmetric_percentage() {
        let v = parse_symmetric("100 ± 5%").unwrap();
        assert_eq!(v.value, 100.0);
        assert!((v.upper_uncertainty - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_parse_symmetric_plus_minus() {
        let v = parse_symmetric("10 +/- 0.5").unwrap();
        assert_eq!(v.value, 10.0);
        assert_eq!(v.upper_uncertainty, 0.5);
    }

    #[test]
    fn test_parse_asymmetric() {
        let v = parse_asymmetric("10 +0.5/-0.3").unwrap();
        assert_eq!(v.value, 10.0);
        assert_eq!(v.upper_uncertainty, 0.5);
        assert_eq!(v.lower_uncertainty, 0.3);
    }

    #[test]
    fn test_parse_uncertain_value() {
        let v = parse_uncertain_value("10 ± 0.5").unwrap();
        assert_eq!(v.value, 10.0);
        assert_eq!(v.upper_uncertainty, 0.5);
    }

    #[test]
    fn test_parse_uncertain_value_plain_number() {
        let v = parse_uncertain_value("42").unwrap();
        assert_eq!(v.value, 42.0);
        assert_eq!(v.upper_uncertainty, 0.0);
    }

    #[test]
    fn test_uncertain_value_display() {
        let v = UncertainValue::symmetric(10.0, 0.5);
        let s = v.to_string();
        assert!(s.contains("10"));
        assert!(s.contains("±"));
        assert!(s.contains("0.5"));
    }

    #[test]
    fn test_uncertain_value_scientific_string() {
        let v = UncertainValue::symmetric(4560.0, 120.0);
        let s = v.to_scientific_string(2);
        assert!(s.contains("×"));
        assert!(s.contains("10^3"));
    }
}
