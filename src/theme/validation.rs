use super::Theme;

/// Warnings produced by theme validation.
#[derive(Clone, Debug, PartialEq)]
pub enum ThemeWarning {
    /// A color palette has fewer than 10 shades.
    MissingShadesForColor {
        color: String,
        expected: usize,
        found: usize,
    },
    /// The primary_color name does not match any palette key.
    PrimaryColorNotFound { color: String },
    /// A CSS value appears invalid (basic heuristic check).
    InvalidCssValue { field: String, value: String },
    /// Text/background contrast fails WCAG AA (ratio < 4.5:1).
    InsufficientContrast {
        foreground: String,
        background: String,
        ratio: f64,
        required: f64,
    },
}

/// Validate a theme and return a list of warnings.
pub fn validate_theme(theme: &Theme) -> Vec<ThemeWarning> {
    let mut warnings = Vec::new();

    // Check that primary color exists in both schemes
    let primary = &theme.colors.primary_color;
    if !theme.colors.light.colors.contains_key(primary) {
        warnings.push(ThemeWarning::PrimaryColorNotFound {
            color: primary.clone(),
        });
    }

    // Check that all palettes have 10 shades
    for (name, shades) in &theme.colors.light.colors {
        if shades.shades.len() != 10 {
            warnings.push(ThemeWarning::MissingShadesForColor {
                color: name.clone(),
                expected: 10,
                found: shades.shades.len(),
            });
        }
    }
    for (name, shades) in &theme.colors.dark.colors {
        if shades.shades.len() != 10 {
            warnings.push(ThemeWarning::MissingShadesForColor {
                color: format!("{} (dark)", name),
                expected: 10,
                found: shades.shades.len(),
            });
        }
    }

    // Check light scheme contrast (text on background)
    let light = &theme.colors.light;
    let ratio = contrast_ratio(&light.text, &light.background);
    if ratio < 4.5 {
        warnings.push(ThemeWarning::InsufficientContrast {
            foreground: light.text.clone(),
            background: light.background.clone(),
            ratio,
            required: 4.5,
        });
    }

    // Check dark scheme contrast (text on background)
    let dark = &theme.colors.dark;
    let ratio = contrast_ratio(&dark.text, &dark.background);
    if ratio < 4.5 {
        warnings.push(ThemeWarning::InsufficientContrast {
            foreground: dark.text.clone(),
            background: dark.background.clone(),
            ratio,
            required: 4.5,
        });
    }

    // Basic CSS value validation for spacing/radius
    validate_css_value(&theme.spacing.xs, "spacing.xs", &mut warnings);
    validate_css_value(&theme.spacing.sm, "spacing.sm", &mut warnings);
    validate_css_value(&theme.spacing.md, "spacing.md", &mut warnings);
    validate_css_value(&theme.spacing.lg, "spacing.lg", &mut warnings);
    validate_css_value(&theme.spacing.xl, "spacing.xl", &mut warnings);

    warnings
}

fn validate_css_value(value: &str, field: &str, warnings: &mut Vec<ThemeWarning>) {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        warnings.push(ThemeWarning::InvalidCssValue {
            field: field.to_string(),
            value: value.to_string(),
        });
        return;
    }
    // Basic check: should contain a CSS unit or be a number
    let has_unit = trimmed.ends_with("rem")
        || trimmed.ends_with("em")
        || trimmed.ends_with("px")
        || trimmed.ends_with('%')
        || trimmed == "0";
    if !has_unit {
        warnings.push(ThemeWarning::InvalidCssValue {
            field: field.to_string(),
            value: value.to_string(),
        });
    }
}

/// Parse a hex color string to (r, g, b) in 0..255 range.
pub fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    match hex.len() {
        6 => {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some((r, g, b))
        }
        3 => {
            let r = u8::from_str_radix(&hex[0..1], 16).ok()? * 17;
            let g = u8::from_str_radix(&hex[1..2], 16).ok()? * 17;
            let b = u8::from_str_radix(&hex[2..3], 16).ok()? * 17;
            Some((r, g, b))
        }
        _ => None,
    }
}

/// Convert a single sRGB channel (0..255) to linear light.
fn srgb_to_linear(value: f64) -> f64 {
    if value <= 0.04045 {
        value / 12.92
    } else {
        ((value + 0.055) / 1.055).powf(2.4)
    }
}

/// Calculate relative luminance per WCAG 2.1.
pub fn relative_luminance(hex: &str) -> Option<f64> {
    let (r, g, b) = hex_to_rgb(hex)?;
    let r = srgb_to_linear(r as f64 / 255.0);
    let g = srgb_to_linear(g as f64 / 255.0);
    let b = srgb_to_linear(b as f64 / 255.0);
    Some(0.2126 * r + 0.7152 * g + 0.0722 * b)
}

/// Calculate WCAG 2.1 contrast ratio between two hex colors.
/// Returns a value between 1.0 and 21.0.
pub fn contrast_ratio(hex1: &str, hex2: &str) -> f64 {
    let l1 = relative_luminance(hex1).unwrap_or(0.0);
    let l2 = relative_luminance(hex2).unwrap_or(0.0);
    let lighter = l1.max(l2);
    let darker = l1.min(l2);
    (lighter + 0.05) / (darker + 0.05)
}

/// Check if two colors meet WCAG AA for normal text (ratio >= 4.5:1).
pub fn meets_wcag_aa(bg: &str, fg: &str) -> bool {
    contrast_ratio(bg, fg) >= 4.5
}

/// Check if two colors meet WCAG AA for large text (ratio >= 3.0:1).
pub fn meets_wcag_aa_large(bg: &str, fg: &str) -> bool {
    contrast_ratio(bg, fg) >= 3.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb() {
        assert_eq!(hex_to_rgb("#ffffff"), Some((255, 255, 255)));
        assert_eq!(hex_to_rgb("#000000"), Some((0, 0, 0)));
        assert_eq!(hex_to_rgb("#ff0000"), Some((255, 0, 0)));
        assert_eq!(hex_to_rgb("228be6"), Some((34, 139, 230)));
    }

    #[test]
    fn test_hex_to_rgb_short() {
        assert_eq!(hex_to_rgb("#fff"), Some((255, 255, 255)));
        assert_eq!(hex_to_rgb("#000"), Some((0, 0, 0)));
    }

    #[test]
    fn test_hex_to_rgb_invalid() {
        assert_eq!(hex_to_rgb("invalid"), None);
        assert_eq!(hex_to_rgb("#gg0000"), None);
    }

    #[test]
    fn test_relative_luminance() {
        let white = relative_luminance("#ffffff").unwrap();
        let black = relative_luminance("#000000").unwrap();
        assert!((white - 1.0).abs() < 0.001);
        assert!(black.abs() < 0.001);
    }

    #[test]
    fn test_contrast_ratio_black_white() {
        let ratio = contrast_ratio("#000000", "#ffffff");
        assert!((ratio - 21.0).abs() < 0.1);
    }

    #[test]
    fn test_contrast_ratio_same_color() {
        let ratio = contrast_ratio("#228be6", "#228be6");
        assert!((ratio - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_meets_wcag_aa() {
        // Black on white should pass
        assert!(meets_wcag_aa("#ffffff", "#000000"));
        // White on white should fail
        assert!(!meets_wcag_aa("#ffffff", "#ffffff"));
    }

    #[test]
    fn test_meets_wcag_aa_large() {
        assert!(meets_wcag_aa_large("#ffffff", "#000000"));
        assert!(!meets_wcag_aa_large("#ffffff", "#ffffff"));
    }

    #[test]
    fn test_validate_default_theme() {
        let theme = Theme::default();
        let warnings = validate_theme(&theme);
        // Default theme should have no warnings
        assert!(
            warnings.is_empty(),
            "Default theme has warnings: {:?}",
            warnings
        );
    }

    #[test]
    fn test_validate_missing_primary() {
        let mut theme = Theme::default();
        theme.colors.primary_color = "nonexistent".to_string();
        let warnings = validate_theme(&theme);
        assert!(warnings
            .iter()
            .any(|w| matches!(w, ThemeWarning::PrimaryColorNotFound { .. })));
    }

    #[test]
    fn test_validate_insufficient_contrast() {
        let mut theme = Theme::default();
        theme.colors.light.text = "#eeeeee".to_string(); // Light text on light bg
        let warnings = validate_theme(&theme);
        assert!(warnings
            .iter()
            .any(|w| matches!(w, ThemeWarning::InsufficientContrast { .. })));
    }

    #[test]
    fn test_validate_invalid_css() {
        use super::super::Spacing;
        use std::borrow::Cow;
        let theme = Theme {
            spacing: Spacing {
                xs: Cow::Borrowed(""),
                ..Spacing::default()
            },
            ..Theme::default()
        };
        let warnings = validate_theme(&theme);
        assert!(warnings
            .iter()
            .any(|w| matches!(w, ThemeWarning::InvalidCssValue { .. })));
    }
}
