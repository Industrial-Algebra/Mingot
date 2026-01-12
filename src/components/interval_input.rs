//! Interval input component for mathematical interval notation.
//!
//! Supports open/closed/half-open intervals with proper mathematical notation.

use crate::components::input::{InputSize, InputVariant};
use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

/// Interval bounds type
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum IntervalBounds {
    /// Closed interval [a, b] - includes both endpoints
    #[default]
    Closed,
    /// Open interval (a, b) - excludes both endpoints
    Open,
    /// Half-open [a, b) - includes left, excludes right
    HalfOpenLeft,
    /// Half-open (a, b] - excludes left, includes right
    HalfOpenRight,
}

impl IntervalBounds {
    /// Get the left bracket character
    pub fn left_bracket(&self) -> &'static str {
        match self {
            IntervalBounds::Closed | IntervalBounds::HalfOpenLeft => "[",
            IntervalBounds::Open | IntervalBounds::HalfOpenRight => "(",
        }
    }

    /// Get the right bracket character
    pub fn right_bracket(&self) -> &'static str {
        match self {
            IntervalBounds::Closed | IntervalBounds::HalfOpenRight => "]",
            IntervalBounds::Open | IntervalBounds::HalfOpenLeft => ")",
        }
    }

    /// Check if left endpoint is included
    pub fn includes_left(&self) -> bool {
        matches!(self, IntervalBounds::Closed | IntervalBounds::HalfOpenLeft)
    }

    /// Check if right endpoint is included
    pub fn includes_right(&self) -> bool {
        matches!(self, IntervalBounds::Closed | IntervalBounds::HalfOpenRight)
    }
}

/// Represents a mathematical interval
#[derive(Clone, Debug, PartialEq)]
pub struct Interval {
    /// Lower bound (None represents negative infinity)
    pub min: Option<f64>,
    /// Upper bound (None represents positive infinity)
    pub max: Option<f64>,
    /// Bounds type (open, closed, half-open)
    pub bounds: IntervalBounds,
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: Some(0.0),
            max: Some(1.0),
            bounds: IntervalBounds::Closed,
        }
    }
}

impl Interval {
    /// Create a new interval
    pub fn new(min: Option<f64>, max: Option<f64>, bounds: IntervalBounds) -> Self {
        Self { min, max, bounds }
    }

    /// Create a closed interval [a, b]
    pub fn closed(min: f64, max: f64) -> Self {
        Self::new(Some(min), Some(max), IntervalBounds::Closed)
    }

    /// Create an open interval (a, b)
    pub fn open(min: f64, max: f64) -> Self {
        Self::new(Some(min), Some(max), IntervalBounds::Open)
    }

    /// Create a half-open interval [a, b)
    pub fn half_open_left(min: f64, max: f64) -> Self {
        Self::new(Some(min), Some(max), IntervalBounds::HalfOpenLeft)
    }

    /// Create a half-open interval (a, b]
    pub fn half_open_right(min: f64, max: f64) -> Self {
        Self::new(Some(min), Some(max), IntervalBounds::HalfOpenRight)
    }

    /// Create an interval from negative infinity: (-∞, b] or (-∞, b)
    pub fn from_neg_infinity(max: f64, include_max: bool) -> Self {
        Self::new(
            None,
            Some(max),
            if include_max {
                IntervalBounds::HalfOpenRight
            } else {
                IntervalBounds::Open
            },
        )
    }

    /// Create an interval to positive infinity: [a, ∞) or (a, ∞)
    pub fn to_pos_infinity(min: f64, include_min: bool) -> Self {
        Self::new(
            Some(min),
            None,
            if include_min {
                IntervalBounds::HalfOpenLeft
            } else {
                IntervalBounds::Open
            },
        )
    }

    /// Check if a value is contained in the interval
    pub fn contains(&self, value: f64) -> bool {
        let above_min = match self.min {
            None => true,
            Some(min) => {
                if self.bounds.includes_left() {
                    value >= min
                } else {
                    value > min
                }
            }
        };

        let below_max = match self.max {
            None => true,
            Some(max) => {
                if self.bounds.includes_right() {
                    value <= max
                } else {
                    value < max
                }
            }
        };

        above_min && below_max
    }

    /// Get the length of the interval (None if unbounded)
    pub fn length(&self) -> Option<f64> {
        match (self.min, self.max) {
            (Some(min), Some(max)) => Some(max - min),
            _ => None,
        }
    }

    /// Get the midpoint of the interval (None if unbounded)
    pub fn midpoint(&self) -> Option<f64> {
        match (self.min, self.max) {
            (Some(min), Some(max)) => Some((min + max) / 2.0),
            _ => None,
        }
    }

    /// Check if the interval is empty (min > max for closed, min >= max for open)
    pub fn is_empty(&self) -> bool {
        match (self.min, self.max) {
            (Some(min), Some(max)) => {
                if self.bounds == IntervalBounds::Open {
                    min >= max
                } else if self.bounds == IntervalBounds::Closed {
                    min > max
                } else {
                    min >= max
                }
            }
            _ => false, // Unbounded intervals are never empty
        }
    }

    /// Check if interval intersects with another
    pub fn intersects(&self, other: &Interval) -> bool {
        // Complex logic for all bound combinations
        let self_min = self.min.unwrap_or(f64::NEG_INFINITY);
        let self_max = self.max.unwrap_or(f64::INFINITY);
        let other_min = other.min.unwrap_or(f64::NEG_INFINITY);
        let other_max = other.max.unwrap_or(f64::INFINITY);

        self_min < other_max && other_min < self_max
    }

    /// Format as mathematical notation
    pub fn to_math_string(&self) -> String {
        let left = self.bounds.left_bracket();
        let right = self.bounds.right_bracket();
        let min_str = self.min.map_or("-∞".to_string(), format_number);
        let max_str = self.max.map_or("∞".to_string(), format_number);
        format!("{}{}, {}{}", left, min_str, max_str, right)
    }

    /// Format as set notation
    pub fn to_set_notation(&self) -> String {
        let min_str = self.min.map_or("-∞".to_string(), format_number);
        let max_str = self.max.map_or("∞".to_string(), format_number);

        let left_cmp = if self.bounds.includes_left() {
            "≤"
        } else {
            "<"
        };
        let right_cmp = if self.bounds.includes_right() {
            "≤"
        } else {
            "<"
        };

        format!(
            "{{x | {} {} x {} {}}}",
            min_str, left_cmp, right_cmp, max_str
        )
    }
}

/// Format a number, removing trailing zeros
fn format_number(value: f64) -> String {
    if value.fract() == 0.0 {
        format!("{:.0}", value)
    } else {
        let s = format!("{:.10}", value);
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    }
}

/// Parse an interval from string notation
pub fn parse_interval(input: &str) -> Result<Interval, String> {
    let trimmed = input.trim();

    // Try to parse mathematical notation: [a, b], (a, b), [a, b), (a, b]
    if (trimmed.starts_with('[') || trimmed.starts_with('('))
        && (trimmed.ends_with(']') || trimmed.ends_with(')'))
    {
        let left_closed = trimmed.starts_with('[');
        let right_closed = trimmed.ends_with(']');

        let bounds = match (left_closed, right_closed) {
            (true, true) => IntervalBounds::Closed,
            (false, false) => IntervalBounds::Open,
            (true, false) => IntervalBounds::HalfOpenLeft,
            (false, true) => IntervalBounds::HalfOpenRight,
        };

        // Extract the inner part
        let inner = &trimmed[1..trimmed.len() - 1];
        let parts: Vec<&str> = inner.split(',').collect();

        if parts.len() != 2 {
            return Err("Expected format: [min, max]".to_string());
        }

        let min_str = parts[0].trim();
        let max_str = parts[1].trim();

        let min = parse_bound(min_str)?;
        let max = parse_bound(max_str)?;

        return Ok(Interval::new(min, max, bounds));
    }

    Err("Invalid interval format. Use [a, b], (a, b), [a, b), or (a, b]".to_string())
}

/// Parse a single bound value
fn parse_bound(s: &str) -> Result<Option<f64>, String> {
    let trimmed = s.trim().to_lowercase();

    if trimmed == "-∞"
        || trimmed == "-inf"
        || trimmed == "-infinity"
        || trimmed == "−∞"
        || trimmed == "neginf"
    {
        return Ok(None);
    }

    if trimmed == "∞"
        || trimmed == "inf"
        || trimmed == "infinity"
        || trimmed == "+∞"
        || trimmed == "+inf"
        || trimmed == "posinf"
    {
        return Ok(None);
    }

    trimmed
        .parse::<f64>()
        .map(Some)
        .map_err(|_| format!("Invalid number: {}", s))
}

/// Display format for intervals
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum IntervalFormat {
    /// Mathematical notation: [a, b]
    #[default]
    Mathematical,
    /// Set notation: {x | a ≤ x ≤ b}
    SetNotation,
}

/// Interval input component
#[component]
pub fn IntervalInput(
    /// Current interval value
    #[prop(optional, into)]
    value: Option<RwSignal<Interval>>,

    /// Callback when interval changes
    #[prop(optional, into)]
    on_change: Option<Callback<Interval>>,

    /// Default bounds type
    #[prop(optional)]
    bounds: IntervalBounds,

    /// Display format
    #[prop(optional)]
    format: IntervalFormat,

    /// Allow infinity bounds
    #[prop(optional, default = true)]
    allow_infinity: bool,

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

    /// Number of decimal places to display
    #[prop(optional, default = 4)]
    _decimal_places: u32,
) -> impl IntoView {
    let theme = use_theme();

    // Internal state
    let internal_value =
        value.unwrap_or_else(|| RwSignal::new(Interval::new(Some(0.0), Some(1.0), bounds)));
    let min_input = RwSignal::new(
        internal_value
            .get_untracked()
            .min
            .map_or(String::new(), format_number),
    );
    let max_input = RwSignal::new(
        internal_value
            .get_untracked()
            .max
            .map_or(String::new(), format_number),
    );
    let current_bounds = RwSignal::new(internal_value.get_untracked().bounds);

    // Update interval when inputs change
    let update_interval = move || {
        let min = if min_input.get().trim().is_empty() && allow_infinity {
            None
        } else {
            min_input.get().trim().parse::<f64>().ok()
        };

        let max = if max_input.get().trim().is_empty() && allow_infinity {
            None
        } else {
            max_input.get().trim().parse::<f64>().ok()
        };

        let interval = Interval::new(min, max, current_bounds.get());
        internal_value.set(interval.clone());
        if let Some(cb) = on_change {
            cb.run(interval);
        }
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

    let input_row_styles = move || {
        StyleBuilder::new()
            .add("display", "flex")
            .add("align-items", "center")
            .add("gap", "0.5rem")
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
            .add("text-align", "center")
            .build()
    };

    let bracket_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        StyleBuilder::new()
            .add("font-size", "1.5rem")
            .add("font-weight", "bold")
            .add("color", scheme_colors.text.clone())
            .add("line-height", "1")
            .build()
    };

    let bounds_selector_styles = move || {
        let theme_val = theme.get();
        StyleBuilder::new()
            .add("display", "flex")
            .add("gap", "0.25rem")
            .add("margin-top", theme_val.spacing.xs)
            .build()
    };

    let bounds_button_styles = move |is_active: bool| {
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

    view! {
        <div class="mingot-interval-input" style=container_styles>
            {label.clone().map(|l| view! {
                <label style=label_styles>{l}</label>
            })}

            <div style=input_row_styles>
                <span style=bracket_styles>{move || current_bounds.get().left_bracket()}</span>
                <input
                    type="text"
                    style=input_styles
                    prop:value=move || min_input.get()
                    placeholder=move || if allow_infinity { "-∞" } else { "min" }
                    disabled=disabled
                    on:input=move |ev| {
                        min_input.set(event_target_value(&ev));
                        update_interval();
                    }
                />
                <span style=move || {
                    let theme_val = theme.get();
                    let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
                    format!("color: {};", scheme_colors.text.clone())
                }>{","}</span>
                <input
                    type="text"
                    style=input_styles
                    prop:value=move || max_input.get()
                    placeholder=move || if allow_infinity { "∞" } else { "max" }
                    disabled=disabled
                    on:input=move |ev| {
                        max_input.set(event_target_value(&ev));
                        update_interval();
                    }
                />
                <span style=bracket_styles>{move || current_bounds.get().right_bracket()}</span>
            </div>

            <div style=bounds_selector_styles>
                <button
                    type="button"
                    style=move || bounds_button_styles(current_bounds.get() == IntervalBounds::Closed)
                    on:click=move |_| {
                        current_bounds.set(IntervalBounds::Closed);
                        update_interval();
                    }
                    disabled=disabled
                >
                    {"[a, b]"}
                </button>
                <button
                    type="button"
                    style=move || bounds_button_styles(current_bounds.get() == IntervalBounds::Open)
                    on:click=move |_| {
                        current_bounds.set(IntervalBounds::Open);
                        update_interval();
                    }
                    disabled=disabled
                >
                    {"(a, b)"}
                </button>
                <button
                    type="button"
                    style=move || bounds_button_styles(current_bounds.get() == IntervalBounds::HalfOpenLeft)
                    on:click=move |_| {
                        current_bounds.set(IntervalBounds::HalfOpenLeft);
                        update_interval();
                    }
                    disabled=disabled
                >
                    {"[a, b)"}
                </button>
                <button
                    type="button"
                    style=move || bounds_button_styles(current_bounds.get() == IntervalBounds::HalfOpenRight)
                    on:click=move |_| {
                        current_bounds.set(IntervalBounds::HalfOpenRight);
                        update_interval();
                    }
                    disabled=disabled
                >
                    {"(a, b]"}
                </button>
            </div>

            <div style=preview_styles>
                {move || {
                    let interval = internal_value.get();
                    match format {
                        IntervalFormat::Mathematical => interval.to_math_string(),
                        IntervalFormat::SetNotation => interval.to_set_notation(),
                    }
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

    #[test]
    fn test_interval_closed() {
        let interval = Interval::closed(0.0, 10.0);
        assert!(interval.contains(0.0));
        assert!(interval.contains(5.0));
        assert!(interval.contains(10.0));
        assert!(!interval.contains(-1.0));
        assert!(!interval.contains(11.0));
    }

    #[test]
    fn test_interval_open() {
        let interval = Interval::open(0.0, 10.0);
        assert!(!interval.contains(0.0));
        assert!(interval.contains(5.0));
        assert!(!interval.contains(10.0));
    }

    #[test]
    fn test_interval_half_open_left() {
        let interval = Interval::half_open_left(0.0, 10.0);
        assert!(interval.contains(0.0));
        assert!(interval.contains(5.0));
        assert!(!interval.contains(10.0));
    }

    #[test]
    fn test_interval_half_open_right() {
        let interval = Interval::half_open_right(0.0, 10.0);
        assert!(!interval.contains(0.0));
        assert!(interval.contains(5.0));
        assert!(interval.contains(10.0));
    }

    #[test]
    fn test_interval_length() {
        let interval = Interval::closed(0.0, 10.0);
        assert_eq!(interval.length(), Some(10.0));

        let unbounded = Interval::to_pos_infinity(0.0, true);
        assert_eq!(unbounded.length(), None);
    }

    #[test]
    fn test_interval_midpoint() {
        let interval = Interval::closed(0.0, 10.0);
        assert_eq!(interval.midpoint(), Some(5.0));
    }

    #[test]
    fn test_interval_is_empty() {
        let empty = Interval::closed(10.0, 0.0);
        assert!(empty.is_empty());

        let valid = Interval::closed(0.0, 10.0);
        assert!(!valid.is_empty());
    }

    #[test]
    fn test_interval_intersects() {
        let a = Interval::closed(0.0, 10.0);
        let b = Interval::closed(5.0, 15.0);
        assert!(a.intersects(&b));

        let c = Interval::closed(20.0, 30.0);
        assert!(!a.intersects(&c));
    }

    #[test]
    fn test_interval_to_math_string() {
        let closed = Interval::closed(0.0, 10.0);
        assert_eq!(closed.to_math_string(), "[0, 10]");

        let open = Interval::open(0.0, 10.0);
        assert_eq!(open.to_math_string(), "(0, 10)");

        let half_open = Interval::half_open_left(0.0, 10.0);
        assert_eq!(half_open.to_math_string(), "[0, 10)");
    }

    #[test]
    fn test_interval_to_set_notation() {
        let closed = Interval::closed(0.0, 10.0);
        assert_eq!(closed.to_set_notation(), "{x | 0 ≤ x ≤ 10}");
    }

    #[test]
    fn test_parse_interval() {
        let closed = parse_interval("[0, 10]").unwrap();
        assert_eq!(closed.min, Some(0.0));
        assert_eq!(closed.max, Some(10.0));
        assert_eq!(closed.bounds, IntervalBounds::Closed);

        let open = parse_interval("(0, 10)").unwrap();
        assert_eq!(open.bounds, IntervalBounds::Open);

        let half_open = parse_interval("[0, 10)").unwrap();
        assert_eq!(half_open.bounds, IntervalBounds::HalfOpenLeft);
    }

    #[test]
    fn test_parse_interval_with_infinity() {
        let inf = parse_interval("(-inf, 10]").unwrap();
        assert_eq!(inf.min, None);
        assert_eq!(inf.max, Some(10.0));
    }

    #[test]
    fn test_interval_infinity_bounds() {
        let to_inf = Interval::to_pos_infinity(0.0, true);
        assert!(to_inf.contains(0.0));
        assert!(to_inf.contains(1000000.0));
        assert!(!to_inf.contains(-1.0));
        assert_eq!(to_inf.to_math_string(), "[0, ∞)");

        let from_neg_inf = Interval::from_neg_infinity(10.0, true);
        assert!(from_neg_inf.contains(-1000000.0));
        assert!(from_neg_inf.contains(10.0));
        assert!(!from_neg_inf.contains(11.0));
    }
}
