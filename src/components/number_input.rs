use crate::components::input::{InputSize, InputVariant};
use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

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
    Thousand,    // 123,456,789
    Scientific,  // 1.23456789e8
    Engineering, // 123.456789e6 (exponents divisible by 3)
}

/// Locale presets for number formatting
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum NumberInputLocale {
    #[default]
    US, // 1,234,567.89
    EU,     // 1.234.567,89
    Swiss,  // 1'234'567.89
    Indian, // 12,34,567.89
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

// Formatting functions
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

fn convert_to_scientific(input: &str) -> String {
    let cleaned = input.replace([',', '_'], "");

    if let Ok(num) = cleaned.parse::<f64>() {
        format!("{:e}", num)
    } else {
        input.to_string()
    }
}

/// Convert to engineering notation (exponents divisible by 3)
fn convert_to_engineering(input: &str) -> String {
    let cleaned = input.replace([',', '_'], "");

    if let Ok(num) = cleaned.parse::<f64>() {
        if num == 0.0 {
            return "0".to_string();
        }

        let abs_num = num.abs();
        let log10 = abs_num.log10().floor() as i32;
        // Round down to nearest multiple of 3
        let exp = (log10 / 3) * 3;
        let mantissa = num / 10_f64.powi(exp);

        if exp == 0 {
            format!("{}", mantissa)
        } else {
            format!("{}e{}", mantissa, exp)
        }
    } else {
        input.to_string()
    }
}

/// Get locale-specific separators
fn get_locale_separators(locale: NumberInputLocale) -> (char, char) {
    match locale {
        NumberInputLocale::US => (',', '.'),
        NumberInputLocale::EU => ('.', ','),
        NumberInputLocale::Swiss => ('\'', '.'),
        NumberInputLocale::Indian => (',', '.'),
    }
}

/// Add thousand separators with Indian numbering system (12,34,567)
fn add_indian_separators(input: &str) -> String {
    let cleaned = input.replace([',', '_'], "");
    let parts: Vec<&str> = cleaned.split('.').collect();

    let integer_part = parts[0];
    let is_negative = integer_part.starts_with('-');
    let abs_part = if is_negative {
        &integer_part[1..]
    } else {
        integer_part
    };

    let len = abs_part.len();
    if len <= 3 {
        return cleaned;
    }

    let mut result = String::new();
    let chars: Vec<char> = abs_part.chars().collect();

    // First group of 3 from the right, then groups of 2
    for (i, ch) in chars.iter().enumerate() {
        let pos_from_right = len - i;
        if i > 0 && (pos_from_right == 3 || (pos_from_right > 3 && (pos_from_right - 3) % 2 == 0)) {
            result.push(',');
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

fn format_number(input: &str, format: NumberInputFormat, thousand_separator: char) -> String {
    match format {
        NumberInputFormat::Standard => input.to_string(),
        NumberInputFormat::Thousand => add_thousand_separators(input, thousand_separator),
        NumberInputFormat::Scientific => convert_to_scientific(input),
        NumberInputFormat::Engineering => convert_to_engineering(input),
    }
}

/// Format number according to locale settings
fn format_number_locale(
    input: &str,
    format: NumberInputFormat,
    locale: NumberInputLocale,
) -> String {
    let (thousand_sep, decimal_sep) = get_locale_separators(locale);

    match format {
        NumberInputFormat::Thousand => {
            // For locale formatting, we need to handle decimal separator carefully
            let cleaned = input.replace([',', '_'], "");
            let parts: Vec<&str> = cleaned.split('.').collect();

            // Format integer part with thousand separators
            let integer_formatted = if matches!(locale, NumberInputLocale::Indian) {
                add_indian_separators(parts[0])
            } else {
                add_thousand_separators(parts[0], thousand_sep)
            };

            // Rejoin with locale's decimal separator
            if parts.len() > 1 {
                format!("{}{}{}", integer_formatted, decimal_sep, parts[1])
            } else {
                integer_formatted
            }
        }
        NumberInputFormat::Standard => input.to_string(),
        NumberInputFormat::Scientific => convert_to_scientific(input),
        NumberInputFormat::Engineering => convert_to_engineering(input),
    }
}

/// Calculate the number of decimal places in a value
fn count_decimal_places(input: &str) -> u32 {
    let cleaned = input.replace([',', '_'], "");
    if let Some(dot_pos) = cleaned.find('.') {
        (cleaned.len() - dot_pos - 1) as u32
    } else {
        0
    }
}

/// Calculate the total number of significant digits
fn count_significant_digits(input: &str) -> usize {
    let cleaned = input.replace([',', '_', '.', '-', '+'], "");
    // Remove leading zeros
    let trimmed = cleaned.trim_start_matches('0');
    if trimmed.is_empty() {
        1 // "0" has 1 significant digit
    } else {
        trimmed.len()
    }
}

/// Get a human-readable precision description
fn get_precision_description(precision: NumberInputPrecision) -> &'static str {
    match precision {
        NumberInputPrecision::U64 => "Unsigned 64-bit integer (0 to 18.4 quintillion)",
        NumberInputPrecision::U128 => "Unsigned 128-bit integer (0 to 340 undecillion)",
        NumberInputPrecision::I64 => "Signed 64-bit integer (±9.2 quintillion)",
        NumberInputPrecision::I128 => "Signed 128-bit integer (±170 undecillion)",
        NumberInputPrecision::Decimal(_) => "Fixed decimal precision",
        #[cfg(feature = "high-precision")]
        NumberInputPrecision::Arbitrary => "Arbitrary precision (28-29 significant digits)",
    }
}

/// Check if a value is approaching the type's limits
fn check_overflow_warning(
    input: &str,
    precision: NumberInputPrecision,
    threshold_percent: f64,
) -> Option<String> {
    if input.is_empty() {
        return None;
    }

    let cleaned = input.replace([',', '_'], "");

    match precision {
        NumberInputPrecision::U64 => {
            if let Ok(val) = cleaned.parse::<u64>() {
                let max = u64::MAX as f64;
                let current = val as f64;
                if current / max > threshold_percent {
                    return Some(format!(
                        "Value is {:.1}% of maximum ({})",
                        (current / max) * 100.0,
                        u64::MAX
                    ));
                }
            }
        }
        NumberInputPrecision::U128 => {
            if let Ok(val) = cleaned.parse::<u128>() {
                // Use string length comparison for u128 since it can't fit in f64
                let max_len = u128::MAX.to_string().len();
                let val_len = val.to_string().len();
                if val_len >= max_len - 1 {
                    return Some(format!("Value is approaching maximum ({})", u128::MAX));
                }
            }
        }
        NumberInputPrecision::I64 => {
            if let Ok(val) = cleaned.parse::<i64>() {
                let max = i64::MAX as f64;
                let min = i64::MIN as f64;
                let current = val as f64;
                if current / max > threshold_percent {
                    return Some(format!(
                        "Value is {:.1}% of maximum ({})",
                        (current / max) * 100.0,
                        i64::MAX
                    ));
                }
                if current / min > threshold_percent {
                    return Some(format!(
                        "Value is {:.1}% of minimum ({})",
                        (current / min) * 100.0,
                        i64::MIN
                    ));
                }
            }
        }
        NumberInputPrecision::I128 => {
            if let Ok(val) = cleaned.parse::<i128>() {
                let max_len = i128::MAX.to_string().len();
                let val_str = val.to_string();
                let val_len = if val_str.starts_with('-') {
                    val_str.len() - 1
                } else {
                    val_str.len()
                };
                if val_len >= max_len - 1 {
                    if val >= 0 {
                        return Some(format!("Value is approaching maximum ({})", i128::MAX));
                    } else {
                        return Some(format!("Value is approaching minimum ({})", i128::MIN));
                    }
                }
            }
        }
        NumberInputPrecision::Decimal(places) => {
            // Check if we're approaching decimal place limit
            let current_places = count_decimal_places(&cleaned);
            if current_places >= places {
                return Some(format!("At maximum {} decimal places", places));
            }
        }
        #[cfg(feature = "high-precision")]
        NumberInputPrecision::Arbitrary => {
            let sig_digits = count_significant_digits(&cleaned);
            if sig_digits >= 27 {
                return Some(format!("Using {} of ~28-29 significant digits", sig_digits));
            }
        }
    }

    None
}

/// Detect if a string is in scientific notation
fn is_scientific_notation(input: &str) -> bool {
    let lower = input.to_lowercase();
    lower.contains('e') && {
        // Must have digits before and after 'e'
        if let Some(e_pos) = lower.find('e') {
            let before = &lower[..e_pos];
            let after = &lower[e_pos + 1..];
            !before.is_empty()
                && before.chars().any(|c| c.is_ascii_digit())
                && !after.is_empty()
                && (after.starts_with('+')
                    || after.starts_with('-')
                    || after.chars().next().is_some_and(|c| c.is_ascii_digit()))
        } else {
            false
        }
    }
}

/// Convert scientific notation to decimal representation
/// e.g., "1.23e8" -> "123000000", "1.5e-3" -> "0.0015"
fn convert_scientific_to_decimal(input: &str, max_decimals: Option<u32>) -> Option<String> {
    let lower = input.to_lowercase().replace(' ', "");
    let e_pos = lower.find('e')?;

    let mantissa_str = &lower[..e_pos];
    let exponent_str = &lower[e_pos + 1..];

    // Parse mantissa
    let mantissa: f64 = mantissa_str.parse().ok()?;

    // Parse exponent (handle optional + sign)
    let exponent: i32 = exponent_str.trim_start_matches('+').parse().ok()?;

    // Calculate the actual value
    let value = mantissa * 10f64.powi(exponent);

    // Format without scientific notation
    if exponent >= 0 && value.fract() == 0.0 {
        // Integer result
        Some(format!("{:.0}", value))
    } else {
        // Determine decimal places needed
        let decimals = if let Some(max) = max_decimals {
            max as usize
        } else {
            // Calculate based on mantissa decimal places and exponent
            let mantissa_decimals = mantissa_str
                .find('.')
                .map(|pos| mantissa_str.len() - pos - 1)
                .unwrap_or(0);
            if exponent < 0 {
                (mantissa_decimals as i32 - exponent) as usize
            } else {
                mantissa_decimals.saturating_sub(exponent as usize)
            }
        };

        let formatted = format!("{:.prec$}", value, prec = decimals);
        // Trim trailing zeros after decimal point if needed
        let trimmed = if formatted.contains('.') {
            formatted.trim_end_matches('0').trim_end_matches('.')
        } else {
            &formatted
        };
        Some(trimmed.to_string())
    }
}

/// Detect the format of a pasted number and return metadata
#[derive(Debug, Clone, PartialEq)]
pub enum DetectedFormat {
    Standard,
    ThousandSeparated,
    Scientific,
    Engineering,
    Currency(String),
    Percentage,
}

/// Detect the format of input text
/// Note: Currently used in tests, will be used for context menu format conversion
#[allow(dead_code)]
fn detect_paste_format(input: &str) -> DetectedFormat {
    let trimmed = input.trim();

    // Check for currency
    if trimmed.starts_with(['$', '€', '£', '¥', '₹'])
        || trimmed.contains("USD")
        || trimmed.contains("EUR")
        || trimmed.contains("GBP")
    {
        let symbol = if trimmed.starts_with('$') || trimmed.contains("USD") {
            "USD"
        } else if trimmed.starts_with('€') || trimmed.contains("EUR") {
            "EUR"
        } else if trimmed.starts_with('£') || trimmed.contains("GBP") {
            "GBP"
        } else if trimmed.starts_with('¥') {
            "JPY"
        } else {
            "INR"
        };
        return DetectedFormat::Currency(symbol.to_string());
    }

    // Check for percentage
    if trimmed.ends_with('%') {
        return DetectedFormat::Percentage;
    }

    // Check for scientific notation
    if is_scientific_notation(trimmed) {
        // Engineering notation has exponents divisible by 3
        let lower = trimmed.to_lowercase();
        if let Some(e_pos) = lower.find('e') {
            let exp_str = &lower[e_pos + 1..];
            if let Ok(exp) = exp_str.trim_start_matches('+').parse::<i32>() {
                if exp % 3 == 0 {
                    return DetectedFormat::Engineering;
                }
            }
        }
        return DetectedFormat::Scientific;
    }

    // Check for thousand separators
    if trimmed.contains(',') || trimmed.contains('\'') || trimmed.contains(' ') {
        return DetectedFormat::ThousandSeparated;
    }

    DetectedFormat::Standard
}

/// Normalize a pasted number by detecting and handling various formats
/// Handles: thousand separators, alternate decimal separators, currency symbols, whitespace
fn normalize_pasted_number(input: &str, decimal_separator: char) -> String {
    // Trim whitespace
    let trimmed = input.trim();

    // Remove common currency symbols and their variations
    let without_currency = trimmed
        .replace(['$', '€', '£', '¥', '₹'], "")
        .replace("USD", "")
        .replace("EUR", "")
        .replace("GBP", "")
        .trim()
        .to_string();

    // Remove percentage sign
    let without_percent = without_currency.trim_end_matches('%').trim().to_string();

    // Remove thousand separators (commas, spaces, apostrophes, underscores)
    let without_thousands = without_percent.replace([',', ' ', '\'', '_'], "");

    // Handle alternate decimal separators
    // If the user's decimal separator is not '.', replace it
    if decimal_separator != '.' {
        without_thousands.replace(decimal_separator, ".")
    } else {
        without_thousands
    }
}

/// Enhanced paste normalization with scientific notation conversion
fn normalize_pasted_number_enhanced(
    input: &str,
    decimal_separator: char,
    convert_scientific: bool,
    max_decimals: Option<u32>,
) -> String {
    let basic_normalized = normalize_pasted_number(input, decimal_separator);

    // Convert scientific notation if enabled and detected
    if convert_scientific && is_scientific_notation(&basic_normalized) {
        convert_scientific_to_decimal(&basic_normalized, max_decimals).unwrap_or(basic_normalized)
    } else {
        basic_normalized
    }
}

/// Increment/decrement operations for different precision types
fn increment_value(
    current: &str,
    step: &str,
    precision: NumberInputPrecision,
    is_increment: bool,
    min: Option<&str>,
    max: Option<&str>,
) -> String {
    if current.is_empty() {
        return if is_increment {
            step.to_string()
        } else {
            format!("-{}", step)
        };
    }

    let cleaned = current.replace([',', '_'], "");
    let step_cleaned = step.replace([',', '_'], "");

    let result = match precision {
        NumberInputPrecision::U64 => {
            let current_val: u64 = cleaned.parse().unwrap_or(0);
            let step_val: u64 = step_cleaned.parse().unwrap_or(1);

            let new_val = if is_increment {
                current_val.saturating_add(step_val)
            } else {
                current_val.saturating_sub(step_val)
            };

            // Apply min/max bounds
            let bounded = apply_bounds_u64(new_val, min, max);
            bounded.to_string()
        }
        NumberInputPrecision::U128 => {
            let current_val: u128 = cleaned.parse().unwrap_or(0);
            let step_val: u128 = step_cleaned.parse().unwrap_or(1);

            let new_val = if is_increment {
                current_val.saturating_add(step_val)
            } else {
                current_val.saturating_sub(step_val)
            };

            let bounded = apply_bounds_u128(new_val, min, max);
            bounded.to_string()
        }
        NumberInputPrecision::I64 => {
            let current_val: i64 = cleaned.parse().unwrap_or(0);
            let step_val: i64 = step_cleaned.parse().unwrap_or(1);

            let new_val = if is_increment {
                current_val.saturating_add(step_val)
            } else {
                current_val.saturating_sub(step_val)
            };

            let bounded = apply_bounds_i64(new_val, min, max);
            bounded.to_string()
        }
        NumberInputPrecision::I128 => {
            let current_val: i128 = cleaned.parse().unwrap_or(0);
            let step_val: i128 = step_cleaned.parse().unwrap_or(1);

            let new_val = if is_increment {
                current_val.saturating_add(step_val)
            } else {
                current_val.saturating_sub(step_val)
            };

            let bounded = apply_bounds_i128(new_val, min, max);
            bounded.to_string()
        }
        NumberInputPrecision::Decimal(places) => {
            let current_val: f64 = cleaned.parse().unwrap_or(0.0);
            let step_val: f64 = step_cleaned.parse().unwrap_or(1.0);

            let new_val = if is_increment {
                current_val + step_val
            } else {
                current_val - step_val
            };

            let bounded = apply_bounds_f64(new_val, min, max);
            format!("{:.1$}", bounded, places as usize)
        }
        #[cfg(feature = "high-precision")]
        NumberInputPrecision::Arbitrary => {
            use rust_decimal::Decimal;
            use std::str::FromStr;

            let current_val = Decimal::from_str(&cleaned).unwrap_or(Decimal::ZERO);
            let step_val = Decimal::from_str(&step_cleaned).unwrap_or(Decimal::ONE);

            let new_val = if is_increment {
                current_val + step_val
            } else {
                current_val - step_val
            };

            let bounded = apply_bounds_decimal(new_val, min, max);
            bounded.to_string()
        }
    };

    result
}

fn apply_bounds_u64(value: u64, min: Option<&str>, max: Option<&str>) -> u64 {
    let mut result = value;
    if let Some(min_str) = min {
        if let Ok(min_val) = min_str.replace([',', '_'], "").parse::<u64>() {
            result = result.max(min_val);
        }
    }
    if let Some(max_str) = max {
        if let Ok(max_val) = max_str.replace([',', '_'], "").parse::<u64>() {
            result = result.min(max_val);
        }
    }
    result
}

fn apply_bounds_u128(value: u128, min: Option<&str>, max: Option<&str>) -> u128 {
    let mut result = value;
    if let Some(min_str) = min {
        if let Ok(min_val) = min_str.replace([',', '_'], "").parse::<u128>() {
            result = result.max(min_val);
        }
    }
    if let Some(max_str) = max {
        if let Ok(max_val) = max_str.replace([',', '_'], "").parse::<u128>() {
            result = result.min(max_val);
        }
    }
    result
}

fn apply_bounds_i64(value: i64, min: Option<&str>, max: Option<&str>) -> i64 {
    let mut result = value;
    if let Some(min_str) = min {
        if let Ok(min_val) = min_str.replace([',', '_'], "").parse::<i64>() {
            result = result.max(min_val);
        }
    }
    if let Some(max_str) = max {
        if let Ok(max_val) = max_str.replace([',', '_'], "").parse::<i64>() {
            result = result.min(max_val);
        }
    }
    result
}

fn apply_bounds_i128(value: i128, min: Option<&str>, max: Option<&str>) -> i128 {
    let mut result = value;
    if let Some(min_str) = min {
        if let Ok(min_val) = min_str.replace([',', '_'], "").parse::<i128>() {
            result = result.max(min_val);
        }
    }
    if let Some(max_str) = max {
        if let Ok(max_val) = max_str.replace([',', '_'], "").parse::<i128>() {
            result = result.min(max_val);
        }
    }
    result
}

fn apply_bounds_f64(value: f64, min: Option<&str>, max: Option<&str>) -> f64 {
    let mut result = value;
    if let Some(min_str) = min {
        if let Ok(min_val) = min_str.replace([',', '_'], "").parse::<f64>() {
            if result < min_val {
                result = min_val;
            }
        }
    }
    if let Some(max_str) = max {
        if let Ok(max_val) = max_str.replace([',', '_'], "").parse::<f64>() {
            if result > max_val {
                result = max_val;
            }
        }
    }
    result
}

#[cfg(feature = "high-precision")]
fn apply_bounds_decimal(
    value: rust_decimal::Decimal,
    min: Option<&str>,
    max: Option<&str>,
) -> rust_decimal::Decimal {
    use rust_decimal::Decimal;
    use std::str::FromStr;

    let mut result = value;
    if let Some(min_str) = min {
        if let Ok(min_val) = Decimal::from_str(&min_str.replace([',', '_'], "")) {
            if result < min_val {
                result = min_val;
            }
        }
    }
    if let Some(max_str) = max {
        if let Ok(max_val) = Decimal::from_str(&max_str.replace([',', '_'], "")) {
            if result > max_val {
                result = max_val;
            }
        }
    }
    result
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
    #[prop(optional, into)] min: Option<String>,
    #[prop(optional, into)] max: Option<String>,

    // Increment/decrement controls
    /// Whether to show +/- controls
    #[prop(default = false)]
    show_controls: bool,
    /// Step size for increment/decrement (default: "1")
    #[prop(optional, into)]
    step: Option<String>,
    /// Step size when Shift is held (default: 10x step)
    #[prop(optional, into)]
    shift_step: Option<String>,
    /// Step size when Ctrl is held (default: 100x step)
    #[prop(optional, into)]
    ctrl_step: Option<String>,
    /// Whether to allow mouse wheel to change value
    #[prop(default = false)]
    allow_mouse_wheel: bool,

    // Display formatting
    /// Format to apply to displayed value (on blur)
    #[prop(optional)]
    format: Option<NumberInputFormat>,
    /// Locale preset for formatting (overrides decimal/thousand separator)
    #[prop(optional)]
    locale: Option<NumberInputLocale>,
    /// Decimal separator character (default: '.')
    #[prop(default = '.')]
    decimal_separator: char,
    /// Thousand separator character (default: ',')
    #[prop(default = ',')]
    thousand_separator: char,
    /// Whether to auto-format value on blur
    #[prop(default = false)]
    format_on_blur: bool,
    /// Threshold for auto-switching to scientific notation (e.g., 1e12 = 1 trillion)
    /// Set to 0 to disable auto-switch
    #[prop(default = 0.0)]
    auto_scientific_threshold: f64,

    // Visual indicators
    /// Whether to show precision type indicator below the input
    #[prop(default = false)]
    show_precision_indicator: bool,
    /// Whether to show warning when approaching type limits
    #[prop(default = false)]
    show_overflow_warning: bool,
    /// Threshold for overflow warning (0.0-1.0, default 0.9 = 90% of max)
    #[prop(default = 0.9)]
    overflow_warning_threshold: f64,

    // Enhanced input handling
    /// Whether to detect and convert pasted number formats automatically
    #[prop(default = true)]
    allow_paste_format_detection: bool,
    /// Whether to convert scientific notation on paste to decimal (e.g., 1.23e8 -> 123000000)
    #[prop(default = false)]
    convert_scientific_on_paste: bool,
    /// Enable undo/redo with Ctrl+Z/Ctrl+Y
    #[prop(default = true)]
    enable_undo_redo: bool,
    /// Maximum undo history size
    #[prop(default = 50)]
    undo_history_size: usize,

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

    // Undo/redo state management
    let undo_stack: RwSignal<Vec<String>> = RwSignal::new(Vec::new());
    let redo_stack: RwSignal<Vec<String>> = RwSignal::new(Vec::new());

    // Helper to push current value to undo stack before changes
    let push_undo = move |old_value: String| {
        if enable_undo_redo {
            undo_stack.update(|stack| {
                // Don't push duplicates
                if stack.last() != Some(&old_value) {
                    stack.push(old_value);
                    // Limit stack size
                    if stack.len() > undo_history_size {
                        stack.remove(0);
                    }
                }
            });
            // Clear redo stack on new change
            redo_stack.set(Vec::new());
        }
    };

    // Undo handler
    let handle_undo = move || {
        if !enable_undo_redo {
            return false;
        }
        let mut undone = false;
        undo_stack.update(|stack| {
            if let Some(prev_value) = stack.pop() {
                // Push current to redo
                redo_stack.update(|redo| {
                    redo.push(number_value.get());
                });
                number_value.set(prev_value.clone());
                undone = true;

                if let Some(callback) = on_change {
                    callback.run(prev_value.clone());
                }
                if let Some(callback) = on_valid_change {
                    callback.run(Ok(prev_value));
                }
            }
        });
        undone
    };

    // Redo handler
    let handle_redo = move || {
        if !enable_undo_redo {
            return false;
        }
        let mut redone = false;
        redo_stack.update(|stack| {
            if let Some(next_value) = stack.pop() {
                // Push current to undo
                undo_stack.update(|undo| {
                    undo.push(number_value.get());
                });
                number_value.set(next_value.clone());
                redone = true;

                if let Some(callback) = on_change {
                    callback.run(next_value.clone());
                }
                if let Some(callback) = on_valid_change {
                    callback.run(Ok(next_value));
                }
            }
        });
        redone
    };

    // Get max decimals from precision for scientific conversion
    let max_decimals_for_paste = match precision {
        NumberInputPrecision::Decimal(n) => Some(n),
        _ => None,
    };

    // Store min/max as owned strings for use in closures
    let min_value = min.clone();
    let max_value = max.clone();
    let step_value = step.unwrap_or_else(|| "1".to_string());
    let shift_step_value = shift_step.unwrap_or_else(|| {
        // Default shift step is 10x the step
        let step_num: f64 = step_value.parse().unwrap_or(1.0);
        format!("{}", step_num * 10.0)
    });
    let ctrl_step_value = ctrl_step.unwrap_or_else(|| {
        // Default ctrl step is 100x the step
        let step_num: f64 = step_value.parse().unwrap_or(1.0);
        format!("{}", step_num * 100.0)
    });

    // Clone values for use in multiple closures
    let min_for_increment = min_value.clone();
    let max_for_increment = max_value.clone();
    let step_for_increment = step_value.clone();
    let shift_step_for_increment = shift_step_value.clone();
    let ctrl_step_for_increment = ctrl_step_value.clone();

    let min_for_wheel = min_value.clone();
    let max_for_wheel = max_value.clone();
    let step_for_wheel = step_value.clone();
    let shift_step_for_wheel = shift_step_value.clone();
    let ctrl_step_for_wheel = ctrl_step_value.clone();

    let min_for_keyboard = min_value.clone();
    let max_for_keyboard = max_value.clone();
    let step_for_keyboard = step_value.clone();
    let shift_step_for_keyboard = shift_step_value.clone();
    let ctrl_step_for_keyboard = ctrl_step_value.clone();

    // Increment/decrement handler
    // use_shift: Shift key held (10x step)
    // use_ctrl: Ctrl key held (100x step)
    let handle_step = move |is_increment: bool, use_shift: bool, use_ctrl: bool| {
        if disabled.get() {
            return;
        }

        let current = number_value.get();

        // Push to undo stack before changing
        push_undo(current.clone());

        let step_to_use = if use_ctrl {
            &ctrl_step_for_increment
        } else if use_shift {
            &shift_step_for_increment
        } else {
            &step_for_increment
        };

        let new_value = increment_value(
            &current,
            step_to_use,
            precision,
            is_increment,
            min_for_increment.as_deref(),
            max_for_increment.as_deref(),
        );

        number_value.set(new_value.clone());

        if let Some(callback) = on_change {
            callback.run(new_value.clone());
        }

        if let Some(callback) = on_valid_change {
            callback.run(Ok(new_value));
        }
    };

    // Create clones for button handlers - detect Shift/Ctrl from mouse event
    let handle_increment = {
        let handle_step = handle_step.clone();
        move |ev: ev::MouseEvent| handle_step(true, ev.shift_key(), ev.ctrl_key())
    };

    let handle_decrement = {
        let handle_step = handle_step.clone();
        move |ev: ev::MouseEvent| handle_step(false, ev.shift_key(), ev.ctrl_key())
    };

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

    // Keyboard handler for arrow up/down and undo/redo
    let handle_keydown = move |ev: ev::KeyboardEvent| {
        if disabled.get() {
            return;
        }

        let key = ev.key();
        let use_shift = ev.shift_key();
        let use_ctrl = ev.ctrl_key() || ev.meta_key(); // Support Cmd on Mac

        match key.as_str() {
            // Undo: Ctrl+Z (or Cmd+Z on Mac)
            "z" | "Z" if use_ctrl && !use_shift => {
                ev.prevent_default();
                handle_undo();
            }
            // Redo: Ctrl+Y or Ctrl+Shift+Z (or Cmd equivalents on Mac)
            "y" | "Y" if use_ctrl => {
                ev.prevent_default();
                handle_redo();
            }
            "z" | "Z" if use_ctrl && use_shift => {
                ev.prevent_default();
                handle_redo();
            }
            "ArrowUp" => {
                ev.prevent_default();
                let current = number_value.get();

                // Push to undo stack before changing
                push_undo(current.clone());

                let step_to_use = if use_ctrl {
                    &ctrl_step_for_keyboard
                } else if use_shift {
                    &shift_step_for_keyboard
                } else {
                    &step_for_keyboard
                };

                let new_value = increment_value(
                    &current,
                    step_to_use,
                    precision,
                    true,
                    min_for_keyboard.as_deref(),
                    max_for_keyboard.as_deref(),
                );

                number_value.set(new_value.clone());

                if let Some(callback) = on_change {
                    callback.run(new_value.clone());
                }

                if let Some(callback) = on_valid_change {
                    callback.run(Ok(new_value));
                }
            }
            "ArrowDown" => {
                ev.prevent_default();
                let current = number_value.get();

                // Push to undo stack before changing
                push_undo(current.clone());

                let step_to_use = if use_ctrl {
                    &ctrl_step_for_keyboard
                } else if use_shift {
                    &shift_step_for_keyboard
                } else {
                    &step_for_keyboard
                };

                let new_value = increment_value(
                    &current,
                    step_to_use,
                    precision,
                    false,
                    min_for_keyboard.as_deref(),
                    max_for_keyboard.as_deref(),
                );

                number_value.set(new_value.clone());

                if let Some(callback) = on_change {
                    callback.run(new_value.clone());
                }

                if let Some(callback) = on_valid_change {
                    callback.run(Ok(new_value));
                }
            }
            _ => {}
        }
    };

    // Wheel handler for mouse wheel scrolling
    let handle_wheel = move |ev: ev::WheelEvent| {
        if !allow_mouse_wheel || disabled.get() {
            return;
        }

        // Only handle wheel when input is focused
        let target = ev.target();
        if let Some(target) = target {
            if let Ok(element) = target.dyn_into::<web_sys::HtmlElement>() {
                // Check if we're the focused element
                if let Some(document) = web_sys::window().and_then(|w| w.document()) {
                    if let Some(active) = document.active_element() {
                        if let Ok(active_element) = active.dyn_into::<web_sys::HtmlElement>() {
                            if element != active_element {
                                return;
                            }
                        } else {
                            return;
                        }
                    } else {
                        return;
                    }
                }
            }
        }

        ev.prevent_default();

        let use_shift = ev.shift_key();
        let use_ctrl = ev.ctrl_key();
        let is_increment = ev.delta_y() < 0.0; // Scroll up = increment

        let current = number_value.get();

        // Push to undo stack before changing
        push_undo(current.clone());

        let step_to_use = if use_ctrl {
            &ctrl_step_for_wheel
        } else if use_shift {
            &shift_step_for_wheel
        } else {
            &step_for_wheel
        };

        let new_value = increment_value(
            &current,
            step_to_use,
            precision,
            is_increment,
            min_for_wheel.as_deref(),
            max_for_wheel.as_deref(),
        );

        number_value.set(new_value.clone());

        if let Some(callback) = on_change {
            callback.run(new_value.clone());
        }

        if let Some(callback) = on_valid_change {
            callback.run(Ok(new_value));
        }
    };

    // Track whether we're focused (to show raw vs formatted value)
    let is_focused = RwSignal::new(false);

    // Handle blur - apply formatting if enabled
    let handle_blur = move |_ev: ev::FocusEvent| {
        is_focused.set(false);

        if !format_on_blur && auto_scientific_threshold == 0.0 {
            return;
        }

        let current = number_value.get();
        if current.is_empty() {
            return;
        }

        // Check for auto-scientific notation switch
        let mut format_type = format.unwrap_or(NumberInputFormat::Thousand);
        if auto_scientific_threshold > 0.0 {
            if let Ok(num) = current.replace([',', '_'], "").parse::<f64>() {
                if num.abs() >= auto_scientific_threshold {
                    format_type = NumberInputFormat::Scientific;
                }
            }
        }

        // Apply formatting with locale support if specified
        let formatted = if let Some(loc) = locale {
            format_number_locale(&current, format_type, loc)
        } else {
            format_number(&current, format_type, thousand_separator)
        };

        // Update the displayed value
        number_value.set(formatted);
    };

    // Handle focus - remove formatting to allow editing
    let handle_focus = move |_ev: ev::FocusEvent| {
        is_focused.set(true);

        if !format_on_blur && auto_scientific_threshold == 0.0 {
            return;
        }

        // Get the separators to strip (either from locale or explicit props)
        let (thou_sep, dec_sep) = if let Some(loc) = locale {
            get_locale_separators(loc)
        } else {
            (thousand_separator, decimal_separator)
        };

        // Strip formatting on focus to allow editing
        let current = number_value.get();

        // Also strip 'e' notation separators and locale-specific separators
        let cleaned = current
            .replace([thou_sep, '_', '\'', ' '], "")
            .trim()
            .to_string();

        // Handle alternate decimal separators
        let cleaned = if dec_sep != '.' {
            cleaned.replace(dec_sep, ".")
        } else {
            cleaned
        };

        number_value.set(cleaned);
    };

    // Handle paste - detect and normalize pasted values
    let handle_paste = move |ev: ev::ClipboardEvent| {
        if disabled.get() {
            return;
        }

        // Get clipboard data from the underlying web_sys event
        let clipboard_event: &web_sys::ClipboardEvent = ev.as_ref();
        if let Some(clipboard_data) = clipboard_event.clipboard_data() {
            if let Ok(pasted_text) = clipboard_data.get_data("text/plain") {
                ev.prevent_default();

                // Push current value to undo stack before changing
                push_undo(number_value.get());

                // Use enhanced normalization if format detection is enabled
                let cleaned = if allow_paste_format_detection {
                    normalize_pasted_number_enhanced(
                        &pasted_text,
                        decimal_separator,
                        convert_scientific_on_paste,
                        max_decimals_for_paste,
                    )
                } else {
                    normalize_pasted_number(&pasted_text, decimal_separator)
                };

                // Filter to valid characters
                let filtered: String = cleaned
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

                // Validate and set
                let validation_result = validate_input(filtered.clone());
                number_value.set(filtered.clone());

                if let Some(callback) = on_change {
                    callback.run(filtered.clone());
                }

                if let Some(callback) = on_valid_change {
                    callback.run(validation_result);
                }
            }
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

    // Control button container styles
    let controls_styles = move || {
        let theme_val = theme.get();
        StyleBuilder::new()
            .add("position", "absolute")
            .add("right", "1px")
            .add("top", "1px")
            .add("bottom", "1px")
            .add("display", "flex")
            .add("flex-direction", "column")
            .add(
                "border-radius",
                format!("0 {} {} 0", theme_val.radius.sm, theme_val.radius.sm),
            )
            .add("overflow", "hidden")
            .build()
    };

    // Control button styles (for +/- buttons)
    let control_button_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let is_disabled = disabled.get();

        StyleBuilder::new()
            .add("display", "flex")
            .add("align-items", "center")
            .add("justify-content", "center")
            .add("width", "1.5rem")
            .add("flex", "1")
            .add("border", "none")
            .add(
                "background-color",
                scheme_colors
                    .get_color("gray", 1)
                    .unwrap_or_else(|| "#f1f3f5".to_string()),
            )
            .add("color", scheme_colors.text.clone())
            .add(
                "cursor",
                if is_disabled {
                    "not-allowed"
                } else {
                    "pointer"
                },
            )
            .add("font-size", theme_val.typography.font_sizes.sm)
            .add(
                "font-weight",
                theme_val.typography.font_weights.medium.to_string(),
            )
            .add("user-select", "none")
            .add("transition", "background-color 0.15s ease")
            .add_if(is_disabled, "opacity", "0.5")
            .build()
    };

    // Input wrapper styles (for positioning controls)
    let input_wrapper_styles =
        move || "position: relative; display: flex; align-items: stretch;".to_string();

    // Adjust input padding when controls are shown
    let error_clone2 = error.clone();
    let input_with_controls_styles = move || {
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

        // Size-based styles with extra padding for controls
        match size {
            InputSize::Xs => {
                builder
                    .add("height", "1.875rem")
                    .add(
                        "padding",
                        if show_controls {
                            "0 2rem 0 0.625rem"
                        } else {
                            "0 0.625rem"
                        },
                    )
                    .add("font-size", theme_val.typography.font_sizes.xs);
            }
            InputSize::Sm => {
                builder
                    .add("height", "2.25rem")
                    .add(
                        "padding",
                        if show_controls {
                            "0 2rem 0 0.75rem"
                        } else {
                            "0 0.75rem"
                        },
                    )
                    .add("font-size", theme_val.typography.font_sizes.sm);
            }
            InputSize::Md => {
                builder
                    .add("height", "2.625rem")
                    .add(
                        "padding",
                        if show_controls {
                            "0 2rem 0 0.875rem"
                        } else {
                            "0 0.875rem"
                        },
                    )
                    .add("font-size", theme_val.typography.font_sizes.sm);
            }
            InputSize::Lg => {
                builder
                    .add("height", "3.125rem")
                    .add(
                        "padding",
                        if show_controls {
                            "0 2rem 0 1rem"
                        } else {
                            "0 1rem"
                        },
                    )
                    .add("font-size", theme_val.typography.font_sizes.md);
            }
            InputSize::Xl => {
                builder
                    .add("height", "3.75rem")
                    .add(
                        "padding",
                        if show_controls {
                            "0 2rem 0 1.125rem"
                        } else {
                            "0 1.125rem"
                        },
                    )
                    .add("font-size", theme_val.typography.font_sizes.lg);
            }
        }

        // Variant-based styles
        match variant {
            InputVariant::Default => {
                let border_color = if error_clone2.is_some() {
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

    let class_str = format!("mingot-number-input {}", class.unwrap_or_default());

    view! {
        <div class="mingot-number-input-wrapper" style="width: 100%;">
            {label.map(|l| view! {
                <label style=label_styles>
                    {l}
                    {if required { " *" } else { "" }}
                </label>
            })}

            <div style=input_wrapper_styles>
                <input
                    type="text"
                    inputmode="decimal"
                    class=class_str
                    style=input_with_controls_styles
                    placeholder=placeholder.unwrap_or_default()
                    disabled=move || disabled.get()
                    required=required
                    prop:value=move || number_value.get()
                    on:input=handle_input
                    on:keydown=handle_keydown
                    on:wheel=handle_wheel
                    on:focus=handle_focus
                    on:blur=handle_blur
                    on:paste=handle_paste
                />

                // Increment/decrement controls
                {show_controls.then(|| {
                    let inc_styles = control_button_styles;
                    let dec_styles = control_button_styles;
                    view! {
                        <div class="mingot-number-input-controls" style=controls_styles>
                            <button
                                type="button"
                                class="mingot-number-input-increment"
                                style=inc_styles
                                disabled=move || disabled.get()
                                on:click=handle_increment.clone()
                                aria-label="Increment"
                                tabindex="-1"
                            >
                                <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                                    <polyline points="18 15 12 9 6 15"></polyline>
                                </svg>
                            </button>
                            <button
                                type="button"
                                class="mingot-number-input-decrement"
                                style=dec_styles
                                disabled=move || disabled.get()
                                on:click=handle_decrement.clone()
                                aria-label="Decrement"
                                tabindex="-1"
                            >
                                <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2.5">
                                    <polyline points="6 9 12 15 18 9"></polyline>
                                </svg>
                            </button>
                        </div>
                    }
                })}
            </div>

            {description.map(|d| view! {
                <div style=description_styles>{d}</div>
            })}

            // Precision indicator
            {show_precision_indicator.then(|| {
                let precision_description = get_precision_description(precision);
                let indicator_styles = move || {
                    let theme_val = theme.get();
                    let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
                    format!(
                        "margin-top: 0.25rem; font-size: {}; color: {}; display: flex; align-items: center; gap: 0.25rem;",
                        theme_val.typography.font_sizes.xs,
                        scheme_colors
                            .get_color("blue", 6)
                            .unwrap_or_else(|| "#228be6".to_string())
                    )
                };
                view! {
                    <div
                        class="mingot-number-input-precision-indicator"
                        style=indicator_styles
                        role="status"
                        aria-label=format!("Precision: {}", precision_description)
                    >
                        <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <circle cx="12" cy="12" r="10"></circle>
                            <line x1="12" y1="16" x2="12" y2="12"></line>
                            <line x1="12" y1="8" x2="12.01" y2="8"></line>
                        </svg>
                        <span>{precision_description}</span>
                    </div>
                }
            })}

            // Overflow warning
            {show_overflow_warning.then(|| {
                let warning_styles = move || {
                    let theme_val = theme.get();
                    let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
                    format!(
                        "margin-top: 0.25rem; font-size: {}; color: {}; display: flex; align-items: center; gap: 0.25rem;",
                        theme_val.typography.font_sizes.xs,
                        scheme_colors
                            .get_color("orange", 6)
                            .unwrap_or_else(|| "#fd7e14".to_string())
                    )
                };
                let warning_message = move || {
                    check_overflow_warning(&number_value.get(), precision, overflow_warning_threshold)
                };
                view! {
                    {move || warning_message().map(|msg| view! {
                        <div
                            class="mingot-number-input-overflow-warning"
                            style=warning_styles
                            role="alert"
                            aria-live="polite"
                        >
                            <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                <path d="M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"></path>
                                <line x1="12" y1="9" x2="12" y2="13"></line>
                                <line x1="12" y1="17" x2="12.01" y2="17"></line>
                            </svg>
                            <span>{msg}</span>
                        </div>
                    })}
                }
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
    fn test_increment_u64() {
        let result = increment_value("10", "1", NumberInputPrecision::U64, true, None, None);
        assert_eq!(result, "11");

        let result = increment_value("10", "5", NumberInputPrecision::U64, true, None, None);
        assert_eq!(result, "15");
    }

    #[test]
    fn test_decrement_u64() {
        let result = increment_value("10", "1", NumberInputPrecision::U64, false, None, None);
        assert_eq!(result, "9");

        // Test saturation at 0
        let result = increment_value("0", "1", NumberInputPrecision::U64, false, None, None);
        assert_eq!(result, "0");
    }

    #[test]
    fn test_increment_with_min_max() {
        // Test max bound
        let result = increment_value(
            "95",
            "10",
            NumberInputPrecision::U64,
            true,
            None,
            Some("100"),
        );
        assert_eq!(result, "100");

        // Test min bound
        let result = increment_value("5", "10", NumberInputPrecision::U64, false, Some("0"), None);
        assert_eq!(result, "0");
    }

    #[test]
    fn test_increment_i64_negative() {
        let result = increment_value("-10", "1", NumberInputPrecision::I64, true, None, None);
        assert_eq!(result, "-9");

        let result = increment_value("-10", "1", NumberInputPrecision::I64, false, None, None);
        assert_eq!(result, "-11");
    }

    #[test]
    fn test_increment_decimal() {
        let result = increment_value(
            "1.5",
            "0.1",
            NumberInputPrecision::Decimal(2),
            true,
            None,
            None,
        );
        assert_eq!(result, "1.60");

        let result = increment_value(
            "1.5",
            "0.1",
            NumberInputPrecision::Decimal(2),
            false,
            None,
            None,
        );
        assert_eq!(result, "1.40");
    }

    #[test]
    fn test_increment_empty_value() {
        let result = increment_value("", "1", NumberInputPrecision::I64, true, None, None);
        assert_eq!(result, "1");

        let result = increment_value("", "1", NumberInputPrecision::I64, false, None, None);
        assert_eq!(result, "-1");
    }

    #[test]
    fn test_increment_with_thousand_separators() {
        // Input with separators should work
        let result = increment_value("1,000", "1", NumberInputPrecision::U64, true, None, None);
        assert_eq!(result, "1001");
    }

    #[test]
    fn test_increment_with_large_step() {
        // Test 10x step (Shift behavior)
        let result = increment_value("100", "10", NumberInputPrecision::U64, true, None, None);
        assert_eq!(result, "110");

        // Test 100x step (Ctrl behavior)
        let result = increment_value("100", "100", NumberInputPrecision::U64, true, None, None);
        assert_eq!(result, "200");

        // Test decrement with large step
        let result = increment_value("1000", "100", NumberInputPrecision::U64, false, None, None);
        assert_eq!(result, "900");
    }

    #[test]
    fn test_increment_decimal_with_large_step() {
        // Test 10x step with decimals
        let result = increment_value(
            "1.50",
            "1.0",
            NumberInputPrecision::Decimal(2),
            true,
            None,
            None,
        );
        assert_eq!(result, "2.50");

        // Test 100x step with decimals
        let result = increment_value(
            "1.50",
            "10.0",
            NumberInputPrecision::Decimal(2),
            true,
            None,
            None,
        );
        assert_eq!(result, "11.50");
    }

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
    fn test_format_number_standard() {
        assert_eq!(
            format_number("1234567", NumberInputFormat::Standard, ','),
            "1234567"
        );
    }

    #[test]
    fn test_format_number_thousand() {
        assert_eq!(
            format_number("1234567", NumberInputFormat::Thousand, ','),
            "1,234,567"
        );
        assert_eq!(
            format_number("1234567.89", NumberInputFormat::Thousand, ','),
            "1,234,567.89"
        );
    }

    #[test]
    fn test_format_number_scientific() {
        // Scientific notation
        let result = format_number("1234567", NumberInputFormat::Scientific, ',');
        assert!(result.contains("e") || result.contains("E"));
    }

    #[test]
    fn test_normalize_pasted_number_basic() {
        // Basic number
        assert_eq!(normalize_pasted_number("123.45", '.'), "123.45");
        // With thousand separators
        assert_eq!(normalize_pasted_number("1,234,567.89", '.'), "1234567.89");
        // With whitespace
        assert_eq!(normalize_pasted_number("  123.45  ", '.'), "123.45");
    }

    #[test]
    fn test_normalize_pasted_number_currency() {
        // Dollar sign
        assert_eq!(normalize_pasted_number("$1,234.56", '.'), "1234.56");
        // Euro sign (all separators removed, then decimal can be swapped if needed)
        assert_eq!(normalize_pasted_number("€1.234,56", '.'), "1.23456");
        // Pound sign
        assert_eq!(normalize_pasted_number("£1,000", '.'), "1000");
    }

    #[test]
    fn test_normalize_pasted_number_alternate_separators() {
        // European format with comma as decimal (user set decimal_separator to ',')
        assert_eq!(normalize_pasted_number("1.234,56", ','), "1.23456");
        // Underscore separators
        assert_eq!(normalize_pasted_number("1_000_000", '.'), "1000000");
        // Apostrophe separators (Swiss format)
        assert_eq!(normalize_pasted_number("1'234'567", '.'), "1234567");
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

    #[test]
    fn test_format_number_engineering() {
        // Engineering notation (exponents divisible by 3)
        let result = convert_to_engineering("1234567");
        assert!(result.contains("e6")); // 1.234567e6

        let result = convert_to_engineering("1000000000");
        assert!(result.contains("e9")); // 1e9

        let result = convert_to_engineering("123");
        assert!(!result.contains("e")); // No exponent for small numbers

        let result = convert_to_engineering("0.000123");
        assert!(result.contains("e-3") || result.contains("e-6")); // Engineering notation for small
    }

    #[test]
    fn test_locale_separators() {
        assert_eq!(get_locale_separators(NumberInputLocale::US), (',', '.'));
        assert_eq!(get_locale_separators(NumberInputLocale::EU), ('.', ','));
        assert_eq!(get_locale_separators(NumberInputLocale::Swiss), ('\'', '.'));
        assert_eq!(get_locale_separators(NumberInputLocale::Indian), (',', '.'));
    }

    #[test]
    fn test_indian_number_format() {
        assert_eq!(add_indian_separators("1234567"), "12,34,567");
        assert_eq!(add_indian_separators("12345678"), "1,23,45,678");
        assert_eq!(add_indian_separators("123"), "123");
        assert_eq!(add_indian_separators("1234"), "1,234");
    }

    #[test]
    fn test_count_decimal_places() {
        assert_eq!(count_decimal_places("123"), 0);
        assert_eq!(count_decimal_places("123.45"), 2);
        assert_eq!(count_decimal_places("123.456789"), 6);
        assert_eq!(count_decimal_places("0.1"), 1);
    }

    #[test]
    fn test_count_significant_digits() {
        assert_eq!(count_significant_digits("123"), 3);
        assert_eq!(count_significant_digits("00123"), 3);
        assert_eq!(count_significant_digits("123.456"), 6);
        assert_eq!(count_significant_digits("0"), 1);
        assert_eq!(count_significant_digits("-123.45"), 5);
    }

    #[test]
    fn test_overflow_warning_u64() {
        // No warning for small values
        assert!(check_overflow_warning("1000", NumberInputPrecision::U64, 0.9).is_none());

        // Warning for values close to max
        let large_val = format!("{}", u64::MAX - 100);
        assert!(check_overflow_warning(&large_val, NumberInputPrecision::U64, 0.9).is_some());
    }

    #[test]
    fn test_format_number_locale() {
        // US format
        assert_eq!(
            format_number_locale(
                "1234567.89",
                NumberInputFormat::Thousand,
                NumberInputLocale::US
            ),
            "1,234,567.89"
        );

        // EU format (swap separators)
        assert_eq!(
            format_number_locale(
                "1234567.89",
                NumberInputFormat::Thousand,
                NumberInputLocale::EU
            ),
            "1.234.567,89"
        );

        // Swiss format
        assert_eq!(
            format_number_locale(
                "1234567.89",
                NumberInputFormat::Thousand,
                NumberInputLocale::Swiss
            ),
            "1'234'567.89"
        );

        // Indian format
        assert_eq!(
            format_number_locale(
                "1234567",
                NumberInputFormat::Thousand,
                NumberInputLocale::Indian
            ),
            "12,34,567"
        );
    }

    #[test]
    fn test_is_scientific_notation() {
        // Valid scientific notation
        assert!(is_scientific_notation("1.23e8"));
        assert!(is_scientific_notation("1.23E8"));
        assert!(is_scientific_notation("1.23e+8"));
        assert!(is_scientific_notation("1.23e-8"));
        assert!(is_scientific_notation("123E10"));
        assert!(is_scientific_notation("-1.5e3"));

        // Not scientific notation
        assert!(!is_scientific_notation("123.45"));
        assert!(!is_scientific_notation("hello"));
        assert!(!is_scientific_notation("e8")); // No mantissa
        assert!(!is_scientific_notation("1.23e")); // No exponent
    }

    #[test]
    fn test_convert_scientific_to_decimal() {
        // Positive exponents
        assert_eq!(
            convert_scientific_to_decimal("1.23e8", None),
            Some("123000000".to_string())
        );
        assert_eq!(
            convert_scientific_to_decimal("1e3", None),
            Some("1000".to_string())
        );
        assert_eq!(
            convert_scientific_to_decimal("2.5e2", None),
            Some("250".to_string())
        );

        // Negative exponents
        assert_eq!(
            convert_scientific_to_decimal("1.5e-3", None),
            Some("0.0015".to_string())
        );
        assert_eq!(
            convert_scientific_to_decimal("1e-2", None),
            Some("0.01".to_string())
        );

        // With explicit + sign
        assert_eq!(
            convert_scientific_to_decimal("1.5e+3", None),
            Some("1500".to_string())
        );

        // With max decimals constraint
        assert_eq!(
            convert_scientific_to_decimal("1.23456e-2", Some(4)),
            Some("0.0123".to_string())
        );

        // Invalid input
        assert_eq!(convert_scientific_to_decimal("abc", None), None);
    }

    #[test]
    fn test_detect_paste_format() {
        // Currency formats
        assert!(matches!(
            detect_paste_format("$1,234.56"),
            DetectedFormat::Currency(s) if s == "USD"
        ));
        assert!(matches!(
            detect_paste_format("€1.234,56"),
            DetectedFormat::Currency(s) if s == "EUR"
        ));
        assert!(matches!(
            detect_paste_format("£1,000"),
            DetectedFormat::Currency(s) if s == "GBP"
        ));

        // Percentage
        assert_eq!(detect_paste_format("50%"), DetectedFormat::Percentage);
        assert_eq!(detect_paste_format("12.5%"), DetectedFormat::Percentage);

        // Scientific notation
        assert_eq!(detect_paste_format("1.23e8"), DetectedFormat::Scientific);
        assert_eq!(detect_paste_format("1.5e-4"), DetectedFormat::Scientific);

        // Engineering notation (exponent divisible by 3)
        assert_eq!(detect_paste_format("1.5e6"), DetectedFormat::Engineering);
        assert_eq!(detect_paste_format("2.3e-3"), DetectedFormat::Engineering);

        // Thousand separated
        assert_eq!(
            detect_paste_format("1,234,567"),
            DetectedFormat::ThousandSeparated
        );
        assert_eq!(
            detect_paste_format("1'234'567"),
            DetectedFormat::ThousandSeparated
        );

        // Standard
        assert_eq!(detect_paste_format("12345"), DetectedFormat::Standard);
        assert_eq!(detect_paste_format("123.45"), DetectedFormat::Standard);
    }

    #[test]
    fn test_normalize_pasted_number_enhanced_basic() {
        // Without scientific conversion
        assert_eq!(
            normalize_pasted_number_enhanced("$1,234.56", '.', false, None),
            "1234.56"
        );

        // With scientific conversion enabled but no scientific notation
        assert_eq!(
            normalize_pasted_number_enhanced("1,234.56", '.', true, None),
            "1234.56"
        );
    }

    #[test]
    fn test_normalize_pasted_number_enhanced_scientific() {
        // Scientific notation NOT converted when disabled
        assert_eq!(
            normalize_pasted_number_enhanced("1.23e8", '.', false, None),
            "1.23e8"
        );

        // Scientific notation converted when enabled
        assert_eq!(
            normalize_pasted_number_enhanced("1.23e8", '.', true, None),
            "123000000"
        );

        // With negative exponent
        assert_eq!(
            normalize_pasted_number_enhanced("1.5e-3", '.', true, None),
            "0.0015"
        );

        // With max decimals
        assert_eq!(
            normalize_pasted_number_enhanced("1.23456e-2", '.', true, Some(4)),
            "0.0123"
        );
    }

    #[test]
    fn test_normalize_pasted_number_percentage() {
        // Percentage sign should be stripped
        assert_eq!(normalize_pasted_number("50%", '.'), "50");
        assert_eq!(normalize_pasted_number("12.5%", '.'), "12.5");
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

    #[test]
    fn test_increment_arbitrary() {
        let result = increment_value(
            "1.23456789012345678901234567",
            "0.00000000000000000000000001",
            NumberInputPrecision::Arbitrary,
            true,
            None,
            None,
        );
        // rust_decimal should handle this with high precision
        assert!(result.contains("1.234567890123456789"));
    }

    #[test]
    fn test_increment_arbitrary_with_bounds() {
        let result = increment_value(
            "99.99",
            "0.01",
            NumberInputPrecision::Arbitrary,
            true,
            None,
            Some("100.00"),
        );
        assert_eq!(result, "100.00");
    }
}
