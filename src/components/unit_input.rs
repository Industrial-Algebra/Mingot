//! UnitInput - Numeric input with unit conversion support
//!
//! Supports values with physical units, automatic conversion between compatible units,
//! and parsing of values with unit suffixes.

use crate::components::input::{InputSize, InputVariant};
use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;

/// Unit categories for grouping compatible units
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum UnitCategory {
    Length,
    Mass,
    Time,
    Temperature,
    Volume,
    Area,
    Speed,
    Force,
    Energy,
    Power,
    Pressure,
    Angle,
    Data,
    Custom,
}

/// Common units with conversion factors
#[derive(Clone, Debug, PartialEq)]
pub struct Unit {
    /// Display symbol (e.g., "m", "kg", "s")
    pub symbol: String,
    /// Full name (e.g., "meter", "kilogram", "second")
    pub name: String,
    /// Category for determining compatibility
    pub category: UnitCategory,
    /// Conversion factor to base unit (multiply by this to get base unit)
    pub to_base: f64,
    /// Offset for conversions (used for temperature)
    pub offset: f64,
}

impl Unit {
    /// Create a new unit with just a conversion factor
    pub fn new(symbol: &str, name: &str, category: UnitCategory, to_base: f64) -> Self {
        Self {
            symbol: symbol.to_string(),
            name: name.to_string(),
            category,
            to_base,
            offset: 0.0,
        }
    }

    /// Create a unit with offset (for temperature conversions)
    pub fn with_offset(
        symbol: &str,
        name: &str,
        category: UnitCategory,
        to_base: f64,
        offset: f64,
    ) -> Self {
        Self {
            symbol: symbol.to_string(),
            name: name.to_string(),
            category,
            to_base,
            offset,
        }
    }

    /// Check if this unit is compatible with another
    pub fn is_compatible(&self, other: &Unit) -> bool {
        self.category == other.category
    }
}

/// A value with an associated unit
#[derive(Clone, Debug, PartialEq)]
pub struct UnitValue {
    pub value: f64,
    pub unit: Unit,
}

impl UnitValue {
    pub fn new(value: f64, unit: Unit) -> Self {
        Self { value, unit }
    }

    /// Convert to base unit
    pub fn to_base(&self) -> f64 {
        (self.value + self.unit.offset) * self.unit.to_base
    }

    /// Convert from base unit to this unit
    pub fn from_base(base_value: f64, unit: &Unit) -> f64 {
        (base_value / unit.to_base) - unit.offset
    }

    /// Convert to another unit (returns None if incompatible)
    pub fn convert_to(&self, target_unit: &Unit) -> Option<UnitValue> {
        if !self.unit.is_compatible(target_unit) {
            return None;
        }

        let base_value = self.to_base();
        let converted_value = Self::from_base(base_value, target_unit);

        Some(UnitValue::new(converted_value, target_unit.clone()))
    }

    /// Format as string with unit
    pub fn to_string_with_unit(&self, precision: u32) -> String {
        if precision == 0 {
            format!("{:.0} {}", self.value, self.unit.symbol)
        } else {
            format!(
                "{:.prec$} {}",
                self.value,
                self.unit.symbol,
                prec = precision as usize
            )
        }
    }
}

impl std::fmt::Display for UnitValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.value, self.unit.symbol)
    }
}

/// Common length units
pub mod length {
    use super::*;

    pub fn meter() -> Unit {
        Unit::new("m", "meter", UnitCategory::Length, 1.0)
    }

    pub fn kilometer() -> Unit {
        Unit::new("km", "kilometer", UnitCategory::Length, 1000.0)
    }

    pub fn centimeter() -> Unit {
        Unit::new("cm", "centimeter", UnitCategory::Length, 0.01)
    }

    pub fn millimeter() -> Unit {
        Unit::new("mm", "millimeter", UnitCategory::Length, 0.001)
    }

    pub fn inch() -> Unit {
        Unit::new("in", "inch", UnitCategory::Length, 0.0254)
    }

    pub fn foot() -> Unit {
        Unit::new("ft", "foot", UnitCategory::Length, 0.3048)
    }

    pub fn yard() -> Unit {
        Unit::new("yd", "yard", UnitCategory::Length, 0.9144)
    }

    pub fn mile() -> Unit {
        Unit::new("mi", "mile", UnitCategory::Length, 1609.344)
    }

    pub fn all() -> Vec<Unit> {
        vec![
            meter(),
            kilometer(),
            centimeter(),
            millimeter(),
            inch(),
            foot(),
            yard(),
            mile(),
        ]
    }
}

/// Common mass units
pub mod mass {
    use super::*;

    pub fn kilogram() -> Unit {
        Unit::new("kg", "kilogram", UnitCategory::Mass, 1.0)
    }

    pub fn gram() -> Unit {
        Unit::new("g", "gram", UnitCategory::Mass, 0.001)
    }

    pub fn milligram() -> Unit {
        Unit::new("mg", "milligram", UnitCategory::Mass, 0.000001)
    }

    pub fn pound() -> Unit {
        Unit::new("lb", "pound", UnitCategory::Mass, 0.453592)
    }

    pub fn ounce() -> Unit {
        Unit::new("oz", "ounce", UnitCategory::Mass, 0.0283495)
    }

    pub fn ton() -> Unit {
        Unit::new("t", "tonne", UnitCategory::Mass, 1000.0)
    }

    pub fn all() -> Vec<Unit> {
        vec![kilogram(), gram(), milligram(), pound(), ounce(), ton()]
    }
}

/// Common time units
pub mod time {
    use super::*;

    pub fn second() -> Unit {
        Unit::new("s", "second", UnitCategory::Time, 1.0)
    }

    pub fn millisecond() -> Unit {
        Unit::new("ms", "millisecond", UnitCategory::Time, 0.001)
    }

    pub fn microsecond() -> Unit {
        Unit::new("μs", "microsecond", UnitCategory::Time, 0.000001)
    }

    pub fn minute() -> Unit {
        Unit::new("min", "minute", UnitCategory::Time, 60.0)
    }

    pub fn hour() -> Unit {
        Unit::new("h", "hour", UnitCategory::Time, 3600.0)
    }

    pub fn day() -> Unit {
        Unit::new("d", "day", UnitCategory::Time, 86400.0)
    }

    pub fn all() -> Vec<Unit> {
        vec![
            second(),
            millisecond(),
            microsecond(),
            minute(),
            hour(),
            day(),
        ]
    }
}

/// Temperature units (using Kelvin as base)
pub mod temperature {
    use super::*;

    pub fn kelvin() -> Unit {
        Unit::new("K", "kelvin", UnitCategory::Temperature, 1.0)
    }

    pub fn celsius() -> Unit {
        Unit::with_offset("°C", "celsius", UnitCategory::Temperature, 1.0, 273.15)
    }

    pub fn fahrenheit() -> Unit {
        Unit::with_offset(
            "°F",
            "fahrenheit",
            UnitCategory::Temperature,
            5.0 / 9.0,
            459.67,
        )
    }

    pub fn all() -> Vec<Unit> {
        vec![kelvin(), celsius(), fahrenheit()]
    }
}

/// Data storage units
pub mod data {
    use super::*;

    pub fn byte() -> Unit {
        Unit::new("B", "byte", UnitCategory::Data, 1.0)
    }

    pub fn kilobyte() -> Unit {
        Unit::new("KB", "kilobyte", UnitCategory::Data, 1000.0)
    }

    pub fn megabyte() -> Unit {
        Unit::new("MB", "megabyte", UnitCategory::Data, 1_000_000.0)
    }

    pub fn gigabyte() -> Unit {
        Unit::new("GB", "gigabyte", UnitCategory::Data, 1_000_000_000.0)
    }

    pub fn terabyte() -> Unit {
        Unit::new("TB", "terabyte", UnitCategory::Data, 1_000_000_000_000.0)
    }

    pub fn kibibyte() -> Unit {
        Unit::new("KiB", "kibibyte", UnitCategory::Data, 1024.0)
    }

    pub fn mebibyte() -> Unit {
        Unit::new("MiB", "mebibyte", UnitCategory::Data, 1_048_576.0)
    }

    pub fn gibibyte() -> Unit {
        Unit::new("GiB", "gibibyte", UnitCategory::Data, 1_073_741_824.0)
    }

    pub fn all() -> Vec<Unit> {
        vec![
            byte(),
            kilobyte(),
            megabyte(),
            gigabyte(),
            terabyte(),
            kibibyte(),
            mebibyte(),
            gibibyte(),
        ]
    }
}

/// Parse a value with unit from string
fn parse_unit_value(input: &str, available_units: &[Unit]) -> Option<UnitValue> {
    let trimmed = input.trim();

    if trimmed.is_empty() {
        return None;
    }

    // Try to find a matching unit suffix
    for unit in available_units {
        // Check for symbol at end
        if let Some(value_str) = trimmed.strip_suffix(&unit.symbol) {
            if let Ok(value) = value_str.trim().parse::<f64>() {
                return Some(UnitValue::new(value, unit.clone()));
            }
        }

        // Also check for name (case insensitive)
        let lower = trimmed.to_lowercase();
        if let Some(value_str) = lower.strip_suffix(&unit.name.to_lowercase()) {
            if let Ok(value) = value_str.trim().parse::<f64>() {
                return Some(UnitValue::new(value, unit.clone()));
            }
        }
    }

    // Try parsing as just a number (use first unit as default)
    if let Ok(value) = trimmed.parse::<f64>() {
        if let Some(default_unit) = available_units.first() {
            return Some(UnitValue::new(value, default_unit.clone()));
        }
    }

    None
}

/// UnitInput component for values with physical units
#[component]
pub fn UnitInput(
    /// Current unit value
    #[prop(optional)]
    value: Option<RwSignal<UnitValue>>,

    /// Callback when value changes
    #[prop(optional)]
    on_change: Option<Callback<UnitValue>>,

    /// Available units for this input
    #[prop(into)]
    units: Vec<Unit>,

    /// Decimal precision for display
    #[prop(default = 2)]
    precision: u32,

    /// Whether to show unit selector
    #[prop(default = true)]
    show_unit_selector: bool,

    /// Callback when unit changes
    #[prop(optional)]
    on_unit_change: Option<Callback<Unit>>,

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

    // Store units for use in closures
    let units_for_parse = units.clone();
    let units_for_select = units.clone();

    // Get default unit
    let default_unit = units.first().cloned().unwrap_or_else(length::meter);

    // Internal unit value
    let unit_value =
        value.unwrap_or_else(|| RwSignal::new(UnitValue::new(0.0, default_unit.clone())));

    // Text representation for editing
    let display_text = RwSignal::new(String::new());

    // Track if user is actively editing
    let is_editing = RwSignal::new(false);

    // Clone error for use in multiple closures
    let error_for_style = error.clone();
    let error_for_display = error.clone();

    // Initialize display text from value
    Effect::new(move || {
        if !is_editing.get() {
            let uv = unit_value.get();
            display_text.set(format!("{:.prec$}", uv.value, prec = precision as usize));
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
    let units_for_blur = units_for_parse.clone();
    let handle_blur = move |_ev: ev::FocusEvent| {
        is_editing.set(false);

        let text = display_text.get();
        let current_unit = unit_value.get().unit.clone();

        // Try to parse with unit, or just as number with current unit
        if let Some(parsed) = parse_unit_value(&text, &units_for_blur) {
            // If parsed unit is different but compatible, convert
            let final_value = if parsed.unit.symbol != current_unit.symbol {
                if let Some(converted) = parsed.convert_to(&current_unit) {
                    converted
                } else {
                    parsed
                }
            } else {
                UnitValue::new(parsed.value, current_unit.clone())
            };

            unit_value.set(final_value.clone());

            if let Some(callback) = on_change {
                callback.run(final_value.clone());
            }

            display_text.set(format!(
                "{:.prec$}",
                final_value.value,
                prec = precision as usize
            ));
        } else if let Ok(num) = text.parse::<f64>() {
            // Just a number, keep current unit
            let new_value = UnitValue::new(num, current_unit);
            unit_value.set(new_value.clone());

            if let Some(callback) = on_change {
                callback.run(new_value);
            }

            display_text.set(format!("{:.prec$}", num, prec = precision as usize));
        } else if !text.is_empty() {
            // Invalid input - revert
            let uv = unit_value.get();
            display_text.set(format!("{:.prec$}", uv.value, prec = precision as usize));
        }
    };

    // Handle unit change from selector
    let handle_unit_change = move |new_unit: Unit| {
        let current = unit_value.get();

        if current.unit.symbol != new_unit.symbol {
            // Convert value to new unit if compatible
            let new_value = if let Some(converted) = current.convert_to(&new_unit) {
                converted
            } else {
                UnitValue::new(current.value, new_unit.clone())
            };

            unit_value.set(new_value.clone());
            display_text.set(format!(
                "{:.prec$}",
                new_value.value,
                prec = precision as usize
            ));

            if let Some(callback) = on_change {
                callback.run(new_value);
            }

            if let Some(callback) = on_unit_change {
                callback.run(new_unit);
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
            InputVariant::Default => scheme_colors.background.clone(),
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
            .add("text-align", "right")
            .build()
    };

    let unit_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        StyleBuilder::new()
            .add(
                "color",
                scheme_colors
                    .get_color("gray", 6)
                    .unwrap_or_else(|| "#868e96".to_string()),
            )
            .add("margin-left", "0.5rem")
            .add("user-select", "none")
            .add("min-width", "3rem")
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
                    inputmode="decimal"
                    style=input_styles
                    placeholder=placeholder.clone().unwrap_or_else(|| "0.00".to_string())
                    prop:value=move || display_text.get()
                    prop:disabled=move || disabled.get()
                    on:input=handle_input
                    on:focus=handle_focus
                    on:blur=handle_blur
                />

                {move || {
                    if show_unit_selector && units_for_select.len() > 1 {
                        let current_unit = unit_value.get().unit;
                        let units_clone = units_for_select.clone();
                        view! {
                            <select
                                style="border: none; background: transparent; cursor: pointer; font-size: inherit; color: inherit; padding: 0 0.25rem; min-width: 3rem;"
                                on:change=move |ev| {
                                    let symbol = event_target_value(&ev);
                                    if let Some(unit) = units_clone.iter().find(|u| u.symbol == symbol) {
                                        handle_unit_change(unit.clone());
                                    }
                                }
                            >
                                {units_for_select.iter().map(|u| {
                                    let is_selected = u.symbol == current_unit.symbol;
                                    let symbol_value = u.symbol.clone();
                                    let symbol_display = u.symbol.clone();
                                    view! {
                                        <option value=symbol_value selected=is_selected>
                                            {symbol_display}
                                        </option>
                                    }
                                }).collect_view()}
                            </select>
                        }.into_any()
                    } else {
                        view! {
                            <span style=unit_styles>
                                {move || unit_value.get().unit.symbol.clone()}
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
    fn test_unit_creation() {
        let m = length::meter();
        assert_eq!(m.symbol, "m");
        assert_eq!(m.name, "meter");
        assert_eq!(m.category, UnitCategory::Length);
        assert_eq!(m.to_base, 1.0);
    }

    #[test]
    fn test_unit_compatibility() {
        let m = length::meter();
        let km = length::kilometer();
        let kg = mass::kilogram();

        assert!(m.is_compatible(&km));
        assert!(!m.is_compatible(&kg));
    }

    #[test]
    fn test_unit_value_to_base() {
        let uv = UnitValue::new(1.0, length::kilometer());
        assert!((uv.to_base() - 1000.0).abs() < 0.0001);

        let uv = UnitValue::new(100.0, length::centimeter());
        assert!((uv.to_base() - 1.0).abs() < 0.0001);
    }

    #[test]
    fn test_unit_conversion() {
        // 1 km = 1000 m
        let km_value = UnitValue::new(1.0, length::kilometer());
        let m_value = km_value.convert_to(&length::meter()).unwrap();
        assert!((m_value.value - 1000.0).abs() < 0.0001);

        // 1000 m = 1 km
        let m_value = UnitValue::new(1000.0, length::meter());
        let km_value = m_value.convert_to(&length::kilometer()).unwrap();
        assert!((km_value.value - 1.0).abs() < 0.0001);

        // 1 inch = 2.54 cm
        let inch_value = UnitValue::new(1.0, length::inch());
        let cm_value = inch_value.convert_to(&length::centimeter()).unwrap();
        assert!((cm_value.value - 2.54).abs() < 0.01);
    }

    #[test]
    fn test_incompatible_conversion() {
        let m_value = UnitValue::new(1.0, length::meter());
        let result = m_value.convert_to(&mass::kilogram());
        assert!(result.is_none());
    }

    #[test]
    fn test_temperature_conversion() {
        // 0°C = 273.15 K
        let celsius_value = UnitValue::new(0.0, temperature::celsius());
        let kelvin_value = celsius_value.convert_to(&temperature::kelvin()).unwrap();
        assert!((kelvin_value.value - 273.15).abs() < 0.01);

        // 100°C = 373.15 K
        let celsius_value = UnitValue::new(100.0, temperature::celsius());
        let kelvin_value = celsius_value.convert_to(&temperature::kelvin()).unwrap();
        assert!((kelvin_value.value - 373.15).abs() < 0.01);

        // 32°F = 0°C
        let fahrenheit_value = UnitValue::new(32.0, temperature::fahrenheit());
        let celsius_value = fahrenheit_value
            .convert_to(&temperature::celsius())
            .unwrap();
        assert!(celsius_value.value.abs() < 0.1);
    }

    #[test]
    fn test_parse_unit_value() {
        let units = length::all();

        // Value with unit symbol
        let parsed = parse_unit_value("5 km", &units).unwrap();
        assert!((parsed.value - 5.0).abs() < 0.0001);
        assert_eq!(parsed.unit.symbol, "km");

        // Value with unit symbol (no space)
        let parsed = parse_unit_value("10m", &units).unwrap();
        assert!((parsed.value - 10.0).abs() < 0.0001);
        assert_eq!(parsed.unit.symbol, "m");

        // Just a number (uses default unit)
        let parsed = parse_unit_value("42", &units).unwrap();
        assert!((parsed.value - 42.0).abs() < 0.0001);
    }

    #[test]
    fn test_unit_value_display() {
        let uv = UnitValue::new(3.25, length::meter());
        assert_eq!(uv.to_string_with_unit(2), "3.25 m");
        assert_eq!(uv.to_string_with_unit(0), "3 m");
    }

    #[test]
    fn test_data_units() {
        // 1 GB = 1000 MB
        let gb_value = UnitValue::new(1.0, data::gigabyte());
        let mb_value = gb_value.convert_to(&data::megabyte()).unwrap();
        assert!((mb_value.value - 1000.0).abs() < 0.0001);

        // 1 GiB = 1024 MiB
        let gib_value = UnitValue::new(1.0, data::gibibyte());
        let mib_value = gib_value.convert_to(&data::mebibyte()).unwrap();
        assert!((mib_value.value - 1024.0).abs() < 0.0001);
    }

    #[test]
    fn test_mass_conversion() {
        // 1 kg = 1000 g
        let kg_value = UnitValue::new(1.0, mass::kilogram());
        let g_value = kg_value.convert_to(&mass::gram()).unwrap();
        assert!((g_value.value - 1000.0).abs() < 0.0001);

        // 1 lb ≈ 0.4536 kg
        let lb_value = UnitValue::new(1.0, mass::pound());
        let kg_value = lb_value.convert_to(&mass::kilogram()).unwrap();
        assert!((kg_value.value - 0.453592).abs() < 0.001);
    }

    #[test]
    fn test_time_conversion() {
        // 1 hour = 3600 seconds
        let hour_value = UnitValue::new(1.0, time::hour());
        let sec_value = hour_value.convert_to(&time::second()).unwrap();
        assert!((sec_value.value - 3600.0).abs() < 0.0001);

        // 1 day = 24 hours
        let day_value = UnitValue::new(1.0, time::day());
        let hour_value = day_value.convert_to(&time::hour()).unwrap();
        assert!((hour_value.value - 24.0).abs() < 0.0001);
    }
}
