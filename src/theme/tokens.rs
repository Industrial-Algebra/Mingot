//! Design token export/import for theme interoperability.
//!
//! Gated behind the `theme-tokens` feature flag.
//! Tokens follow a flat key-value structure inspired by the
//! W3C Design Tokens Community Group format.

use super::{
    Breakpoints, ColorPalette, ColorScheme, ColorSchemeMode, ColorShades, FontSizes, FontWeights,
    LineHeights, RadiusScale, ShadowScale, Spacing, Theme, Typography,
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

/// A serializable representation of a Mingot theme.
///
/// Use [`DesignTokens::from_theme`] to export and
/// [`DesignTokens::to_theme`] to import.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DesignTokens {
    /// Primary color palette name.
    pub primary_color: String,
    /// Active color scheme mode.
    pub color_scheme: String,

    // --- Colors ---
    pub light: SchemeTokens,
    pub dark: SchemeTokens,

    // --- Spacing ---
    pub spacing: ScaleTokens,
    // --- Radius ---
    pub radius: ScaleTokens,
    // --- Shadows ---
    pub shadows: ScaleTokens,
    // --- Breakpoints ---
    pub breakpoints: ScaleTokens,

    // --- Typography ---
    pub font_family: String,
    pub font_family_monospace: String,
    pub font_sizes: FontSizeTokens,
    pub line_heights: ScaleTokens,
    pub font_weights: FontWeightTokens,
}

/// Color scheme tokens (light or dark).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SchemeTokens {
    pub background: String,
    pub text: String,
    pub border: String,
    pub white: String,
    pub black: String,
    pub colors: HashMap<String, Vec<String>>,
}

/// Generic xs-xl scale tokens.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScaleTokens {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
}

/// Font size tokens (xs-xxl).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FontSizeTokens {
    pub xs: String,
    pub sm: String,
    pub md: String,
    pub lg: String,
    pub xl: String,
    pub xxl: String,
}

/// Font weight tokens.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FontWeightTokens {
    pub normal: u16,
    pub medium: u16,
    pub semibold: u16,
    pub bold: u16,
}

impl DesignTokens {
    /// Export a theme to design tokens.
    pub fn from_theme(theme: &Theme) -> Self {
        Self {
            primary_color: theme.colors.primary_color.clone(),
            color_scheme: match theme.color_scheme {
                ColorSchemeMode::Light => "light".to_string(),
                ColorSchemeMode::Dark => "dark".to_string(),
                ColorSchemeMode::Auto => "auto".to_string(),
            },
            light: scheme_to_tokens(&theme.colors.light),
            dark: scheme_to_tokens(&theme.colors.dark),
            spacing: ScaleTokens {
                xs: theme.spacing.xs.to_string(),
                sm: theme.spacing.sm.to_string(),
                md: theme.spacing.md.to_string(),
                lg: theme.spacing.lg.to_string(),
                xl: theme.spacing.xl.to_string(),
            },
            radius: ScaleTokens {
                xs: theme.radius.xs.to_string(),
                sm: theme.radius.sm.to_string(),
                md: theme.radius.md.to_string(),
                lg: theme.radius.lg.to_string(),
                xl: theme.radius.xl.to_string(),
            },
            shadows: ScaleTokens {
                xs: theme.shadows.xs.to_string(),
                sm: theme.shadows.sm.to_string(),
                md: theme.shadows.md.to_string(),
                lg: theme.shadows.lg.to_string(),
                xl: theme.shadows.xl.to_string(),
            },
            breakpoints: ScaleTokens {
                xs: theme.breakpoints.xs.to_string(),
                sm: theme.breakpoints.sm.to_string(),
                md: theme.breakpoints.md.to_string(),
                lg: theme.breakpoints.lg.to_string(),
                xl: theme.breakpoints.xl.to_string(),
            },
            font_family: theme.typography.font_family.to_string(),
            font_family_monospace: theme.typography.font_family_monospace.to_string(),
            font_sizes: FontSizeTokens {
                xs: theme.typography.font_sizes.xs.to_string(),
                sm: theme.typography.font_sizes.sm.to_string(),
                md: theme.typography.font_sizes.md.to_string(),
                lg: theme.typography.font_sizes.lg.to_string(),
                xl: theme.typography.font_sizes.xl.to_string(),
                xxl: theme.typography.font_sizes.xxl.to_string(),
            },
            line_heights: ScaleTokens {
                xs: theme.typography.line_heights.xs.to_string(),
                sm: theme.typography.line_heights.sm.to_string(),
                md: theme.typography.line_heights.md.to_string(),
                lg: theme.typography.line_heights.lg.to_string(),
                xl: theme.typography.line_heights.xl.to_string(),
            },
            font_weights: FontWeightTokens {
                normal: theme.typography.font_weights.normal,
                medium: theme.typography.font_weights.medium,
                semibold: theme.typography.font_weights.semibold,
                bold: theme.typography.font_weights.bold,
            },
        }
    }

    /// Convert design tokens back into a theme.
    pub fn to_theme(&self) -> Theme {
        Theme {
            colors: ColorPalette {
                primary_color: self.primary_color.clone(),
                light: tokens_to_scheme(&self.light),
                dark: tokens_to_scheme(&self.dark),
            },
            spacing: Spacing {
                xs: Cow::Owned(self.spacing.xs.clone()),
                sm: Cow::Owned(self.spacing.sm.clone()),
                md: Cow::Owned(self.spacing.md.clone()),
                lg: Cow::Owned(self.spacing.lg.clone()),
                xl: Cow::Owned(self.spacing.xl.clone()),
            },
            typography: Typography {
                font_family: Cow::Owned(self.font_family.clone()),
                font_family_monospace: Cow::Owned(self.font_family_monospace.clone()),
                font_sizes: FontSizes {
                    xs: Cow::Owned(self.font_sizes.xs.clone()),
                    sm: Cow::Owned(self.font_sizes.sm.clone()),
                    md: Cow::Owned(self.font_sizes.md.clone()),
                    lg: Cow::Owned(self.font_sizes.lg.clone()),
                    xl: Cow::Owned(self.font_sizes.xl.clone()),
                    xxl: Cow::Owned(self.font_sizes.xxl.clone()),
                },
                line_heights: LineHeights {
                    xs: Cow::Owned(self.line_heights.xs.clone()),
                    sm: Cow::Owned(self.line_heights.sm.clone()),
                    md: Cow::Owned(self.line_heights.md.clone()),
                    lg: Cow::Owned(self.line_heights.lg.clone()),
                    xl: Cow::Owned(self.line_heights.xl.clone()),
                },
                font_weights: FontWeights {
                    normal: self.font_weights.normal,
                    medium: self.font_weights.medium,
                    semibold: self.font_weights.semibold,
                    bold: self.font_weights.bold,
                },
            },
            radius: RadiusScale {
                xs: Cow::Owned(self.radius.xs.clone()),
                sm: Cow::Owned(self.radius.sm.clone()),
                md: Cow::Owned(self.radius.md.clone()),
                lg: Cow::Owned(self.radius.lg.clone()),
                xl: Cow::Owned(self.radius.xl.clone()),
            },
            shadows: ShadowScale {
                xs: Cow::Owned(self.shadows.xs.clone()),
                sm: Cow::Owned(self.shadows.sm.clone()),
                md: Cow::Owned(self.shadows.md.clone()),
                lg: Cow::Owned(self.shadows.lg.clone()),
                xl: Cow::Owned(self.shadows.xl.clone()),
            },
            breakpoints: Breakpoints {
                xs: Cow::Owned(self.breakpoints.xs.clone()),
                sm: Cow::Owned(self.breakpoints.sm.clone()),
                md: Cow::Owned(self.breakpoints.md.clone()),
                lg: Cow::Owned(self.breakpoints.lg.clone()),
                xl: Cow::Owned(self.breakpoints.xl.clone()),
            },
            color_scheme: match self.color_scheme.as_str() {
                "dark" => ColorSchemeMode::Dark,
                "auto" => ColorSchemeMode::Auto,
                _ => ColorSchemeMode::Light,
            },
        }
    }

    /// Serialize to JSON string.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Deserialize from a JSON string.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }
}

fn scheme_to_tokens(scheme: &ColorScheme) -> SchemeTokens {
    let mut colors = HashMap::new();
    for (name, shades) in &scheme.colors {
        colors.insert(name.clone(), shades.shades.clone());
    }
    SchemeTokens {
        background: scheme.background.clone(),
        text: scheme.text.clone(),
        border: scheme.border.clone(),
        white: scheme.white.clone(),
        black: scheme.black.clone(),
        colors,
    }
}

fn tokens_to_scheme(tokens: &SchemeTokens) -> ColorScheme {
    let mut colors = HashMap::new();
    for (name, shades) in &tokens.colors {
        colors.insert(
            name.clone(),
            ColorShades {
                shades: shades.clone(),
            },
        );
    }
    ColorScheme {
        background: tokens.background.clone(),
        text: tokens.text.clone(),
        border: tokens.border.clone(),
        white: tokens.white.clone(),
        black: tokens.black.clone(),
        colors,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_roundtrip_default_theme() {
        let theme = Theme::default();
        let tokens = DesignTokens::from_theme(&theme);
        let recovered = tokens.to_theme();
        assert_eq!(theme, recovered);
    }

    #[test]
    fn test_json_roundtrip() {
        let theme = Theme::default();
        let tokens = DesignTokens::from_theme(&theme);
        let json = tokens.to_json().unwrap();
        let recovered = DesignTokens::from_json(&json).unwrap();
        assert_eq!(tokens, recovered);
    }

    #[test]
    fn test_json_contains_expected_fields() {
        let theme = Theme::default();
        let tokens = DesignTokens::from_theme(&theme);
        let json = tokens.to_json().unwrap();

        assert!(json.contains("\"primaryColor\""));
        assert!(json.contains("\"colorScheme\""));
        assert!(json.contains("\"fontFamily\""));
        assert!(json.contains("\"spacing\""));
        assert!(json.contains("\"radius\""));
        assert!(json.contains("\"light\""));
        assert!(json.contains("\"dark\""));
    }

    #[test]
    fn test_color_scheme_serialization() {
        let light = Theme::default();
        assert_eq!(DesignTokens::from_theme(&light).color_scheme, "light");

        let dark = Theme {
            color_scheme: ColorSchemeMode::Dark,
            ..Theme::default()
        };
        assert_eq!(DesignTokens::from_theme(&dark).color_scheme, "dark");

        let auto = Theme {
            color_scheme: ColorSchemeMode::Auto,
            ..Theme::default()
        };
        assert_eq!(DesignTokens::from_theme(&auto).color_scheme, "auto");
    }

    #[test]
    fn test_preset_roundtrip() {
        use crate::theme::presets;
        let presets_list = vec![
            presets::mingot_default(),
            presets::mingot_dark(),
            presets::industrial(),
            presets::scientific(),
            presets::financial(),
        ];
        for theme in presets_list {
            let tokens = DesignTokens::from_theme(&theme);
            let json = tokens.to_json().unwrap();
            let recovered = DesignTokens::from_json(&json).unwrap().to_theme();
            assert_eq!(theme, recovered);
        }
    }
}
