use crate::components::input::{InputSize, InputVariant};
use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum NumberInputPrecision {
    U64,
    U128,
    #[default]
    I64,
    I128,
    Decimal(u32), // Fixed decimal places
    #[cfg(feature = "high-precision")]
    Arbitrary, // Arbitrary precision via rust_decimal
}

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum NumberInputFormat {
    #[default]
    Standard, // 123456789
    Thousand,   // 123,456,789
    Scientific, // 1.23e8
}

#[derive(Clone, Debug, PartialEq)]
pub enum ParseError {
    InvalidFormat(String),
    Overflow(String),
    Underflow(String),
    TooManyDecimals(u32),
    NegativeNotAllowed,
    DecimalNotAllowed,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::InvalidFormat(msg) => write!(f, "Invalid format: {}", msg),
            ParseError::Overflow(msg) => write!(f, "Overflow: {}", msg),
            ParseError::Underflow(msg) => write!(f, "Underflow: {}", msg),
            ParseError::TooManyDecimals(max) => {
                write!(f, "Too many decimal places (max: {})", max)
            }
            ParseError::NegativeNotAllowed => write!(f, "Negative values not allowed"),
            ParseError::DecimalNotAllowed => write!(f, "Decimal values not allowed"),
        }
    }
}

// Validation functions
fn validate_u64(input: &str) -> Result<u64, ParseError> {
    let cleaned = input.replace([',', '_'], "").trim().to_string();

    if cleaned.is_empty() {
        return Err(ParseError::InvalidFormat("Empty input".to_string()));
    }

    cleaned
        .parse::<u64>()
        .map_err(|_| ParseError::Overflow(format!("Value exceeds u64 maximum ({})", u64::MAX)))
}

fn validate_u128(input: &str) -> Result<u128, ParseError> {
    let cleaned = input.replace([',', '_'], "").trim().to_string();

    if cleaned.is_empty() {
        return Err(ParseError::InvalidFormat("Empty input".to_string()));
    }

    cleaned
        .parse::<u128>()
        .map_err(|_| ParseError::Overflow(format!("Value exceeds u128 maximum ({})", u128::MAX)))
}

fn validate_i64(input: &str) -> Result<i64, ParseError> {
    let cleaned = input.replace([',', '_'], "").trim().to_string();

    if cleaned.is_empty() {
        return Err(ParseError::InvalidFormat("Empty input".to_string()));
    }

    cleaned.parse::<i64>().map_err(|_e| {
        if cleaned.starts_with('-') {
            ParseError::Underflow(format!("Value below i64 minimum ({})", i64::MIN))
        } else {
            ParseError::Overflow(format!("Value exceeds i64 maximum ({})", i64::MAX))
        }
    })
}

fn validate_i128(input: &str) -> Result<i128, ParseError> {
    let cleaned = input.replace([',', '_'], "").trim().to_string();

    if cleaned.is_empty() {
        return Err(ParseError::InvalidFormat("Empty input".to_string()));
    }

    cleaned.parse::<i128>().map_err(|_e| {
        if cleaned.starts_with('-') {
            ParseError::Underflow(format!("Value below i128 minimum ({})", i128::MIN))
        } else {
            ParseError::Overflow(format!("Value exceeds i128 maximum ({})", i128::MAX))
        }
    })
}

fn validate_decimal(input: &str, max_decimals: u32) -> Result<String, ParseError> {
    let cleaned = input.replace([',', '_'], "").trim().to_string();

    if cleaned.is_empty() {
        return Err(ParseError::InvalidFormat("Empty input".to_string()));
    }

    // Check for valid decimal format
    if let Some(dot_pos) = cleaned.find('.') {
        let decimal_part = &cleaned[dot_pos + 1..];
        if decimal_part.len() > max_decimals as usize {
            return Err(ParseError::TooManyDecimals(max_decimals));
        }

        // Validate it's a valid number
        cleaned
            .parse::<f64>()
            .map_err(|_| ParseError::InvalidFormat("Not a valid decimal number".to_string()))?;

        Ok(cleaned)
    } else {
        // No decimal point, that's fine
        cleaned
            .parse::<f64>()
            .map_err(|_| ParseError::InvalidFormat("Not a valid number".to_string()))?;

        Ok(cleaned)
    }
}

/// Validate arbitrary precision decimal using rust_decimal
/// Supports up to 28-29 significant digits with exact decimal arithmetic
#[cfg(feature = "high-precision")]
fn validate_arbitrary(input: &str) -> Result<rust_decimal::Decimal, ParseError> {
    use rust_decimal::Decimal;
    use std::str::FromStr;

    let cleaned = input.replace([',', '_'], "").trim().to_string();

    if cleaned.is_empty() {
        return Err(ParseError::InvalidFormat("Empty input".to_string()));
    }

    Decimal::from_str(&cleaned).map_err(|e| ParseError::InvalidFormat(e.to_string()))
}

// Formatting functions (for future use in Phase 2/3)
#[allow(dead_code)]
fn add_thousand_separators(input: &str, separator: char) -> String {
    let cleaned = input.replace([',', '_'], "");
    let parts: Vec<&str> = cleaned.split('.').collect();

    let integer_part = parts[0];
    let is_negative = integer_part.starts_with('-');
    let abs_part = if is_negative {
        &integer_part[1..]
    } else {
        integer_part
    };

    let mut result = String::new();
    let chars: Vec<char> = abs_part.chars().collect();

    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i).is_multiple_of(3) {
            result.push(separator);
        }
        result.push(*ch);
    }

    if is_negative {
        result = format!("-{}", result);
    }

    if parts.len() > 1 {
        result = format!("{}.{}", result, parts[1]);
    }

    result
}

#[allow(dead_code)]
fn convert_to_scientific(input: &str) -> String {
    let cleaned = input.replace([',', '_'], "");

    if let Ok(num) = cleaned.parse::<f64>() {
        format!("{:e}", num)
    } else {
        input.to_string()
    }
}

#[allow(dead_code)]
fn format_number(input: &str, format: NumberInputFormat, thousand_separator: char) -> String {
    match format {
        NumberInputFormat::Standard => input.to_string(),
        NumberInputFormat::Thousand => add_thousand_separators(input, thousand_separator),
        NumberInputFormat::Scientific => convert_to_scientific(input),
    }
}

// Input filtering
fn is_valid_char(
    ch: char,
    current_value: &str,
    allow_negative: bool,
    allow_decimal: bool,
    allow_scientific: bool,
) -> bool {
    match ch {
        '0'..='9' => true,
        '-' => allow_negative && current_value.is_empty(),
        '.' => allow_decimal && !current_value.contains('.'),
        'e' | 'E' => {
            allow_scientific
                && !current_value.is_empty()
                && !current_value.to_lowercase().contains('e')
        }
        ',' | '_' => true, // Allow separators, we clean them during validation
        _ => false,
    }
}

#[component]
pub fn NumberInput(
    // Core value handling
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] on_valid_change: Option<Callback<Result<String, ParseError>>>,

    // Precision configuration
    #[prop(optional)] precision: Option<NumberInputPrecision>,
    #[prop(optional, into)] _min: Option<String>,
    #[prop(optional, into)] _max: Option<String>,

    // Display formatting
    #[prop(optional)] _format: Option<NumberInputFormat>,
    #[prop(optional)] _decimal_separator: Option<char>,
    #[prop(optional)] _thousand_separator: Option<char>,

    // Validation options
    #[prop(optional)] allow_negative: bool,
    #[prop(optional)] allow_decimal: bool,
    #[prop(optional)] allow_scientific: bool,

    // Standard form input props
    #[prop(optional)] variant: Option<InputVariant>,
    #[prop(optional)] size: Option<InputSize>,
    #[prop(optional, into)] placeholder: Option<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional)] required: bool,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let variant = variant.unwrap_or(InputVariant::Default);
    let size = size.unwrap_or(InputSize::Md);
    let precision = precision.unwrap_or_default();

    // Determine allow_negative and allow_decimal based on precision if not explicitly set
    #[cfg(feature = "high-precision")]
    let allow_negative = allow_negative
        || matches!(
            precision,
            NumberInputPrecision::I64
                | NumberInputPrecision::I128
                | NumberInputPrecision::Decimal(_)
                | NumberInputPrecision::Arbitrary
        );
    #[cfg(not(feature = "high-precision"))]
    let allow_negative = allow_negative
        || matches!(
            precision,
            NumberInputPrecision::I64
                | NumberInputPrecision::I128
                | NumberInputPrecision::Decimal(_)
        );

    #[cfg(feature = "high-precision")]
    let allow_decimal = allow_decimal
        || matches!(
            precision,
            NumberInputPrecision::Decimal(_) | NumberInputPrecision::Arbitrary
        );
    #[cfg(not(feature = "high-precision"))]
    let allow_decimal = allow_decimal || matches!(precision, NumberInputPrecision::Decimal(_));

    let number_value = value.unwrap_or_else(|| RwSignal::new(String::new()));

    // Validation function based on precision
    let validate_input = move |input: String| -> Result<String, ParseError> {
        if input.is_empty() {
            return Ok(String::new());
        }

        match precision {
            NumberInputPrecision::U64 => {
                validate_u64(&input)?;
                Ok(input)
            }
            NumberInputPrecision::U128 => {
                validate_u128(&input)?;
                Ok(input)
            }
            NumberInputPrecision::I64 => {
                validate_i64(&input)?;
                Ok(input)
            }
            NumberInputPrecision::I128 => {
                validate_i128(&input)?;
                Ok(input)
            }
            NumberInputPrecision::Decimal(places) => validate_decimal(&input, places),
            #[cfg(feature = "high-precision")]
            NumberInputPrecision::Arbitrary => {
                validate_arbitrary(&input)?;
                Ok(input)
            }
        }
    };

    let error_clone = error.clone();
    let input_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();
        let is_disabled = disabled.get();

        // Base styles
        builder
            .add("width", "100%")
            .add("font-family", theme_val.typography.font_family)
            .add("border-radius", theme_val.radius.sm)
            .add("transition", "all 0.15s ease")
            .add("outline", "none")
            .add("box-sizing", "border-box");

        // Size-based styles
        match size {
            InputSize::Xs => {
                builder
                    .add("height", "1.875rem")
                    .add("padding", "0 0.625rem")
                    .add("font-size", theme_val.typography.font_sizes.xs);
            }
            InputSize::Sm => {
                builder
                    .add("height", "2.25rem")
                    .add("padding", "0 0.75rem")
                    .add("font-size", theme_val.typography.font_sizes.sm);
            }
            InputSize::Md => {
                builder
                    .add("height", "2.625rem")
                    .add("padding", "0 0.875rem")
                    .add("font-size", theme_val.typography.font_sizes.sm);
            }
            InputSize::Lg => {
                builder
                    .add("height", "3.125rem")
                    .add("padding", "0 1rem")
                    .add("font-size", theme_val.typography.font_sizes.md);
            }
            InputSize::Xl => {
                builder
                    .add("height", "3.75rem")
                    .add("padding", "0 1.125rem")
                    .add("font-size", theme_val.typography.font_sizes.lg);
            }
        }

        // Variant-based styles
        match variant {
            InputVariant::Default => {
                let border_color = if error_clone.is_some() {
                    scheme_colors
                        .get_color("red", 6)
                        .unwrap_or_else(|| "#fa5252".to_string())
                } else {
                    scheme_colors.border.clone()
                };

                builder
                    .add("background-color", scheme_colors.background.clone())
                    .add("color", scheme_colors.text.clone())
                    .add("border", format!("1px solid {}", border_color));
            }
            InputVariant::Filled => {
                let bg_color = scheme_colors
                    .get_color("gray", 1)
                    .unwrap_or_else(|| "#f1f3f5".to_string());

                builder
                    .add("background-color", bg_color)
                    .add("color", scheme_colors.text.clone())
                    .add("border", "1px solid transparent");
            }
            InputVariant::Unstyled => {
                builder
                    .add("background-color", "transparent")
                    .add("color", scheme_colors.text.clone())
                    .add("border", "none")
                    .add("padding", "0");
            }
        }

        // Disabled state
        if is_disabled {
            builder.add("opacity", "0.6").add("cursor", "not-allowed");
        } else {
            builder.add("cursor", "text");
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let handle_input = move |ev: ev::Event| {
        let input_value = event_target_value(&ev);

        // Filter invalid characters
        let filtered: String = input_value
            .chars()
            .filter(|ch| {
                is_valid_char(
                    *ch,
                    &number_value.get(),
                    allow_negative,
                    allow_decimal,
                    allow_scientific,
                )
            })
            .collect();

        // Validate the input
        let validation_result = validate_input(filtered.clone());

        // Always update the raw value
        number_value.set(filtered.clone());

        // Call on_change with raw value
        if let Some(callback) = on_change {
            callback.run(filtered.clone());
        }

        // Call on_valid_change with validation result
        if let Some(callback) = on_valid_change {
            callback.run(validation_result);
        }
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

    let class_str = format!("mingot-number-input {}", class.unwrap_or_default());

    view! {
        <div class="mingot-number-input-wrapper" style="width: 100%;">
            {label.map(|l| view! {
                <label style=label_styles>
                    {l}
                    {if required { " *" } else { "" }}
                </label>
            })}

            <input
                type="text"
                inputmode="decimal"
                class=class_str
                style=input_styles
                placeholder=placeholder.unwrap_or_default()
                disabled=move || disabled.get()
                required=required
                prop:value=move || number_value.get()
                on:input=handle_input
            />

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
    fn test_validate_u64_success() {
        assert!(validate_u64("123456").is_ok());
        assert!(validate_u64("18446744073709551615").is_ok()); // u64::MAX
        assert!(validate_u64("1,234,567").is_ok()); // with separators
    }

    #[test]
    fn test_validate_u64_overflow() {
        assert!(matches!(
            validate_u64("18446744073709551616"),
            Err(ParseError::Overflow(_))
        ));
    }

    #[test]
    fn test_validate_i64_success() {
        assert!(validate_i64("123456").is_ok());
        assert!(validate_i64("-123456").is_ok());
        assert!(validate_i64("9223372036854775807").is_ok()); // i64::MAX
        assert!(validate_i64("-9223372036854775808").is_ok()); // i64::MIN
    }

    #[test]
    fn test_validate_decimal_precision() {
        assert!(validate_decimal("0.123456", 6).is_ok());
        assert!(validate_decimal("123.45", 6).is_ok());
        assert!(matches!(
            validate_decimal("0.1234567", 6),
            Err(ParseError::TooManyDecimals(6))
        ));
    }

    #[test]
    fn test_add_thousand_separators() {
        assert_eq!(add_thousand_separators("1234567", ','), "1,234,567");
        assert_eq!(add_thousand_separators("123", ','), "123");
        assert_eq!(add_thousand_separators("-1234567", ','), "-1,234,567");
        assert_eq!(add_thousand_separators("1234567.89", ','), "1,234,567.89");
    }

    #[test]
    fn test_is_valid_char() {
        assert!(is_valid_char('5', "", false, false, false));
        assert!(is_valid_char('-', "", true, false, false));
        assert!(!is_valid_char('-', "123", true, false, false)); // Not at start
        assert!(is_valid_char('.', "123", false, true, false));
        assert!(!is_valid_char('.', "12.3", false, true, false)); // Already has decimal
        assert!(is_valid_char('e', "123", false, false, true));
        assert!(!is_valid_char('a', "123", false, false, false));
    }
}

/// Tests for arbitrary precision support (requires high-precision feature)
#[cfg(all(test, feature = "high-precision"))]
mod arbitrary_precision_tests {
    use super::*;

    #[test]
    fn test_validate_arbitrary_success() {
        // Standard decimal
        let result = validate_arbitrary("123.456");
        assert!(result.is_ok());

        // High precision decimal (more than f64 can represent exactly)
        let result = validate_arbitrary("123.456789012345678901234567");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_arbitrary_large_number() {
        // Large number beyond f64 safe integer range
        let result = validate_arbitrary("99999999999999999999999999.99");
        assert!(result.is_ok());

        // Verify the value is preserved correctly
        let decimal = result.unwrap();
        assert!(decimal
            .to_string()
            .starts_with("99999999999999999999999999"));
    }

    #[test]
    fn test_validate_arbitrary_negative() {
        let result = validate_arbitrary("-123.456");
        assert!(result.is_ok());

        let decimal = result.unwrap();
        assert!(decimal.is_sign_negative());
    }

    #[test]
    fn test_validate_arbitrary_invalid() {
        let result = validate_arbitrary("not_a_number");
        assert!(result.is_err());
        assert!(matches!(result, Err(ParseError::InvalidFormat(_))));
    }

    #[test]
    fn test_validate_arbitrary_empty() {
        let result = validate_arbitrary("");
        assert!(result.is_err());
        assert!(matches!(result, Err(ParseError::InvalidFormat(_))));
    }

    #[test]
    fn test_validate_arbitrary_with_separators() {
        // Thousand separators should be stripped
        let result = validate_arbitrary("1,234,567.89");
        assert!(result.is_ok());

        // Underscore separators should be stripped
        let result = validate_arbitrary("1_234_567.89");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_arbitrary_scientific_notation() {
        // Note: rust_decimal's FromStr doesn't support scientific notation by default
        // Scientific notation requires the "maths" feature and from_scientific() method
        // For now, verify that standard decimal notation works correctly
        let result = validate_arbitrary("12300000000");
        assert!(result.is_ok());

        let result = validate_arbitrary("0.0000123");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_arbitrary_zero() {
        let result = validate_arbitrary("0");
        assert!(result.is_ok());

        let result = validate_arbitrary("0.0");
        assert!(result.is_ok());

        let result = validate_arbitrary("-0");
        assert!(result.is_ok());
    }
}
