//! FractionInput - Exact rational number input with multiple format support
//!
//! Supports fraction notation (1/2), decimal (0.5), and mixed numbers (1 1/2).
//! Features automatic simplification and bidirectional conversion.

use crate::components::input::{InputSize, InputVariant};
use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;

/// Display format for fractions
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum FractionDisplayFormat {
    /// Simple fraction: 3/2
    #[default]
    Fraction,
    /// Mixed number: 1 1/2
    MixedNumber,
    /// Decimal: 1.5
    Decimal,
}

/// Represents an exact fraction (rational number)
#[derive(Clone, Debug, PartialEq)]
pub struct Fraction {
    pub numerator: i64,
    pub denominator: i64,
}

impl Default for Fraction {
    fn default() -> Self {
        Self {
            numerator: 0,
            denominator: 1,
        }
    }
}

impl Fraction {
    /// Create a new fraction
    pub fn new(numerator: i64, denominator: i64) -> Self {
        if denominator == 0 {
            panic!("Denominator cannot be zero");
        }
        Self {
            numerator,
            denominator,
        }
    }

    /// Create a fraction from a whole number
    pub fn from_whole(n: i64) -> Self {
        Self {
            numerator: n,
            denominator: 1,
        }
    }

    /// Create a fraction from a mixed number (whole + fraction)
    pub fn from_mixed(whole: i64, numerator: i64, denominator: i64) -> Self {
        if denominator == 0 {
            panic!("Denominator cannot be zero");
        }
        let sign = if whole < 0 || numerator < 0 { -1 } else { 1 };
        let whole_abs = whole.abs();
        let num_abs = numerator.abs();

        Self {
            numerator: sign * (whole_abs * denominator.abs() + num_abs),
            denominator: denominator.abs(),
        }
    }

    /// Compute the greatest common divisor using Euclidean algorithm
    fn gcd(a: i64, b: i64) -> i64 {
        let mut a = a.abs();
        let mut b = b.abs();
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    /// Simplify the fraction to lowest terms
    pub fn simplify(&self) -> Self {
        if self.numerator == 0 {
            return Self {
                numerator: 0,
                denominator: 1,
            };
        }

        let gcd = Self::gcd(self.numerator, self.denominator);
        let mut num = self.numerator / gcd;
        let mut den = self.denominator / gcd;

        // Ensure denominator is positive
        if den < 0 {
            num = -num;
            den = -den;
        }

        Self {
            numerator: num,
            denominator: den,
        }
    }

    /// Convert to decimal
    pub fn to_decimal(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }

    /// Create from decimal with specified precision (max denominator)
    pub fn from_decimal(value: f64, max_denominator: i64) -> Self {
        if value == 0.0 {
            return Self::default();
        }

        let negative = value < 0.0;
        let value = value.abs();

        // Use continued fraction algorithm for best rational approximation
        let mut best_num = 0i64;
        let mut best_den = 1i64;
        let mut best_error = value;

        for den in 1..=max_denominator {
            let num = (value * den as f64).round() as i64;
            let error = (value - num as f64 / den as f64).abs();

            if error < best_error {
                best_error = error;
                best_num = num;
                best_den = den;

                if error < 1e-10 {
                    break;
                }
            }
        }

        Self {
            numerator: if negative { -best_num } else { best_num },
            denominator: best_den,
        }
        .simplify()
    }

    /// Check if this represents a whole number
    pub fn is_whole(&self) -> bool {
        self.numerator % self.denominator == 0
    }

    /// Get the whole number part
    pub fn whole_part(&self) -> i64 {
        self.numerator / self.denominator
    }

    /// Get the fractional part (numerator of the proper fraction)
    pub fn fractional_numerator(&self) -> i64 {
        (self.numerator % self.denominator).abs()
    }

    /// Check if the fraction is negative
    pub fn is_negative(&self) -> bool {
        (self.numerator < 0) != (self.denominator < 0)
    }

    /// Format as a simple fraction string
    pub fn to_fraction_string(&self) -> String {
        let simplified = self.simplify();
        if simplified.denominator == 1 {
            format!("{}", simplified.numerator)
        } else {
            format!("{}/{}", simplified.numerator, simplified.denominator)
        }
    }

    /// Format as a mixed number string
    pub fn to_mixed_string(&self) -> String {
        let simplified = self.simplify();

        if simplified.denominator == 1 {
            return format!("{}", simplified.numerator);
        }

        let whole = simplified.whole_part();
        let frac_num = simplified.fractional_numerator();

        if whole == 0 {
            format!("{}/{}", simplified.numerator, simplified.denominator)
        } else if frac_num == 0 {
            format!("{}", whole)
        } else {
            let sign = if simplified.is_negative() { "-" } else { "" };
            format!(
                "{}{} {}/{}",
                sign,
                whole.abs(),
                frac_num,
                simplified.denominator.abs()
            )
        }
    }

    /// Format as decimal string with specified precision
    pub fn to_decimal_string(&self, precision: u32) -> String {
        format!("{:.prec$}", self.to_decimal(), prec = precision as usize)
    }
}

impl std::fmt::Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_fraction_string())
    }
}

/// Parse a fraction from various string formats
fn parse_fraction(input: &str) -> Option<Fraction> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return None;
    }

    // Try parsing as mixed number: "1 1/2" or "-2 3/4"
    if let Some(fraction) = parse_mixed_number(trimmed) {
        return Some(fraction);
    }

    // Try parsing as simple fraction: "1/2" or "-3/4"
    if let Some(fraction) = parse_simple_fraction(trimmed) {
        return Some(fraction);
    }

    // Try parsing as decimal: "0.5" or "-1.25"
    if let Some(fraction) = parse_decimal_to_fraction(trimmed) {
        return Some(fraction);
    }

    // Try parsing as whole number: "5" or "-3"
    if let Ok(whole) = trimmed.parse::<i64>() {
        return Some(Fraction::from_whole(whole));
    }

    None
}

fn parse_mixed_number(input: &str) -> Option<Fraction> {
    // Match patterns like "1 1/2" or "-2 3/4"
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.len() != 2 {
        return None;
    }

    let whole: i64 = parts[0].parse().ok()?;

    // Parse the fraction part
    let frac_parts: Vec<&str> = parts[1].split('/').collect();
    if frac_parts.len() != 2 {
        return None;
    }

    let num: i64 = frac_parts[0].parse().ok()?;
    let den: i64 = frac_parts[1].parse().ok()?;

    if den == 0 {
        return None;
    }

    Some(Fraction::from_mixed(whole, num, den))
}

fn parse_simple_fraction(input: &str) -> Option<Fraction> {
    let parts: Vec<&str> = input.split('/').collect();

    if parts.len() != 2 {
        return None;
    }

    let num: i64 = parts[0].trim().parse().ok()?;
    let den: i64 = parts[1].trim().parse().ok()?;

    if den == 0 {
        return None;
    }

    Some(Fraction::new(num, den))
}

fn parse_decimal_to_fraction(input: &str) -> Option<Fraction> {
    let value: f64 = input.parse().ok()?;

    // Don't parse if it looks like a fraction (contains /)
    if input.contains('/') {
        return None;
    }

    // Use a reasonable max denominator for conversion
    Some(Fraction::from_decimal(value, 10000))
}

/// FractionInput component for exact rational number entry
#[component]
pub fn FractionInput(
    /// Current fraction value
    #[prop(optional)]
    value: Option<RwSignal<Fraction>>,

    /// Callback when value changes
    #[prop(optional)]
    on_change: Option<Callback<Fraction>>,

    /// Display format for the fraction
    #[prop(default = FractionDisplayFormat::Fraction)]
    display_format: FractionDisplayFormat,

    /// Whether to automatically simplify fractions
    #[prop(default = true)]
    auto_simplify: bool,

    /// Decimal precision when displaying as decimal
    #[prop(default = 4)]
    decimal_precision: u32,

    /// Maximum denominator when converting from decimal (used for decimal-to-fraction conversion)
    #[prop(default = 10000)]
    _max_denominator: i64,

    /// Whether to show format selector
    #[prop(default = false)]
    show_format_selector: bool,

    /// Callback when display format changes
    #[prop(optional)]
    on_format_change: Option<Callback<FractionDisplayFormat>>,

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

    // Internal fraction value
    let fraction_value = value.unwrap_or_else(|| RwSignal::new(Fraction::default()));

    // Current display format
    let current_format = RwSignal::new(display_format);

    // Text representation for editing
    let display_text = RwSignal::new(String::new());

    // Track if user is actively editing
    let is_editing = RwSignal::new(false);

    // Clone error for use in multiple closures
    let error_for_style = error.clone();
    let error_for_display = error.clone();

    // Format the fraction according to current display format
    let format_fraction = move |frac: &Fraction, fmt: FractionDisplayFormat| -> String {
        let display_frac = if auto_simplify {
            frac.simplify()
        } else {
            frac.clone()
        };

        match fmt {
            FractionDisplayFormat::Fraction => display_frac.to_fraction_string(),
            FractionDisplayFormat::MixedNumber => display_frac.to_mixed_string(),
            FractionDisplayFormat::Decimal => display_frac.to_decimal_string(decimal_precision),
        }
    };

    // Initialize display text from value
    Effect::new(move || {
        if !is_editing.get() {
            let frac = fraction_value.get();
            let fmt = current_format.get();
            display_text.set(format_fraction(&frac, fmt));
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
        let fmt = current_format.get();

        if let Some(mut parsed) = parse_fraction(&text) {
            if auto_simplify {
                parsed = parsed.simplify();
            }

            fraction_value.set(parsed.clone());

            if let Some(callback) = on_change {
                callback.run(parsed.clone());
            }

            // Update display with formatted value
            display_text.set(format_fraction(&parsed, fmt));
        } else if !text.is_empty() {
            // Invalid input - revert to previous value
            let frac = fraction_value.get();
            display_text.set(format_fraction(&frac, fmt));
        }
    };

    // Handle format change
    let handle_format_change = move |new_format: FractionDisplayFormat| {
        if current_format.get() != new_format {
            current_format.set(new_format);

            // Update display to new format
            let frac = fraction_value.get();
            display_text.set(format_fraction(&frac, new_format));

            if let Some(callback) = on_format_change {
                callback.run(new_format);
            }
        }
    };

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

    // Format options for selector
    let format_options = [
        FractionDisplayFormat::Fraction,
        FractionDisplayFormat::MixedNumber,
        FractionDisplayFormat::Decimal,
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
                        match current_format.get() {
                            FractionDisplayFormat::Fraction => "1/2".to_string(),
                            FractionDisplayFormat::MixedNumber => "1 1/2".to_string(),
                            FractionDisplayFormat::Decimal => "0.5".to_string(),
                        }
                    })
                    prop:value=move || display_text.get()
                    prop:disabled=move || disabled.get()
                    on:input=handle_input
                    on:focus=handle_focus
                    on:blur=handle_blur
                />

                {move || {
                    if show_format_selector {
                        let current = current_format.get();
                        view! {
                            <select
                                style="border: none; background: transparent; cursor: pointer; font-size: 0.75rem; color: inherit; padding: 0 0.25rem;"
                                on:change=move |ev| {
                                    let value = event_target_value(&ev);
                                    let new_format = match value.as_str() {
                                        "fraction" => FractionDisplayFormat::Fraction,
                                        "mixed" => FractionDisplayFormat::MixedNumber,
                                        "decimal" => FractionDisplayFormat::Decimal,
                                        _ => FractionDisplayFormat::Fraction,
                                    };
                                    handle_format_change(new_format);
                                }
                            >
                                {format_options.iter().map(|f| {
                                    let (value, label) = match f {
                                        FractionDisplayFormat::Fraction => ("fraction", "a/b"),
                                        FractionDisplayFormat::MixedNumber => ("mixed", "n a/b"),
                                        FractionDisplayFormat::Decimal => ("decimal", "0.00"),
                                    };
                                    let is_selected = *f == current;
                                    view! {
                                        <option value=value selected=is_selected>
                                            {label}
                                        </option>
                                    }
                                }).collect_view()}
                            </select>
                        }.into_any()
                    } else {
                        view! { <span></span> }.into_any()
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
    fn test_fraction_new() {
        let f = Fraction::new(3, 4);
        assert_eq!(f.numerator, 3);
        assert_eq!(f.denominator, 4);
    }

    #[test]
    fn test_fraction_from_whole() {
        let f = Fraction::from_whole(5);
        assert_eq!(f.numerator, 5);
        assert_eq!(f.denominator, 1);
    }

    #[test]
    fn test_fraction_from_mixed() {
        let f = Fraction::from_mixed(1, 1, 2);
        assert_eq!(f.numerator, 3);
        assert_eq!(f.denominator, 2);

        let f = Fraction::from_mixed(2, 3, 4);
        assert_eq!(f.numerator, 11);
        assert_eq!(f.denominator, 4);
    }

    #[test]
    fn test_fraction_simplify() {
        let f = Fraction::new(4, 8).simplify();
        assert_eq!(f.numerator, 1);
        assert_eq!(f.denominator, 2);

        let f = Fraction::new(6, 9).simplify();
        assert_eq!(f.numerator, 2);
        assert_eq!(f.denominator, 3);

        // Negative denominator should be normalized
        let f = Fraction::new(3, -4).simplify();
        assert_eq!(f.numerator, -3);
        assert_eq!(f.denominator, 4);
    }

    #[test]
    fn test_fraction_to_decimal() {
        let f = Fraction::new(1, 2);
        assert!((f.to_decimal() - 0.5).abs() < 0.0001);

        let f = Fraction::new(3, 4);
        assert!((f.to_decimal() - 0.75).abs() < 0.0001);
    }

    #[test]
    fn test_fraction_from_decimal() {
        let f = Fraction::from_decimal(0.5, 100);
        assert_eq!(f.numerator, 1);
        assert_eq!(f.denominator, 2);

        let f = Fraction::from_decimal(0.25, 100);
        assert_eq!(f.numerator, 1);
        assert_eq!(f.denominator, 4);

        let f = Fraction::from_decimal(0.333333, 100);
        assert_eq!(f.numerator, 1);
        assert_eq!(f.denominator, 3);
    }

    #[test]
    fn test_fraction_to_string() {
        let f = Fraction::new(3, 4);
        assert_eq!(f.to_fraction_string(), "3/4");

        let f = Fraction::new(4, 1);
        assert_eq!(f.to_fraction_string(), "4");

        let f = Fraction::new(-3, 4);
        assert_eq!(f.to_fraction_string(), "-3/4");
    }

    #[test]
    fn test_fraction_to_mixed_string() {
        let f = Fraction::new(3, 2);
        assert_eq!(f.to_mixed_string(), "1 1/2");

        let f = Fraction::new(7, 4);
        assert_eq!(f.to_mixed_string(), "1 3/4");

        let f = Fraction::new(4, 1);
        assert_eq!(f.to_mixed_string(), "4");

        let f = Fraction::new(1, 2);
        assert_eq!(f.to_mixed_string(), "1/2");

        let f = Fraction::new(-5, 2);
        assert_eq!(f.to_mixed_string(), "-2 1/2");
    }

    #[test]
    fn test_parse_simple_fraction() {
        let f = parse_fraction("3/4").unwrap();
        assert_eq!(f.numerator, 3);
        assert_eq!(f.denominator, 4);

        let f = parse_fraction("-1/2").unwrap();
        assert_eq!(f.numerator, -1);
        assert_eq!(f.denominator, 2);
    }

    #[test]
    fn test_parse_mixed_number() {
        let f = parse_fraction("1 1/2").unwrap();
        assert_eq!(f.numerator, 3);
        assert_eq!(f.denominator, 2);

        let f = parse_fraction("2 3/4").unwrap();
        assert_eq!(f.numerator, 11);
        assert_eq!(f.denominator, 4);
    }

    #[test]
    fn test_parse_decimal() {
        let f = parse_fraction("0.5").unwrap().simplify();
        assert_eq!(f.numerator, 1);
        assert_eq!(f.denominator, 2);

        let f = parse_fraction("1.25").unwrap().simplify();
        assert_eq!(f.numerator, 5);
        assert_eq!(f.denominator, 4);
    }

    #[test]
    fn test_parse_whole_number() {
        let f = parse_fraction("5").unwrap();
        assert_eq!(f.numerator, 5);
        assert_eq!(f.denominator, 1);

        let f = parse_fraction("-3").unwrap();
        assert_eq!(f.numerator, -3);
        assert_eq!(f.denominator, 1);
    }

    #[test]
    fn test_gcd() {
        assert_eq!(Fraction::gcd(12, 8), 4);
        assert_eq!(Fraction::gcd(17, 5), 1);
        assert_eq!(Fraction::gcd(100, 25), 25);
    }
}
