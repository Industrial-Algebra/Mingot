mod builder;
mod color_scheme;
mod colors;
mod override_provider;
pub mod presets;
mod provider;
mod spacing;
#[cfg(feature = "theme-tokens")]
pub mod tokens;
mod typography;
pub mod validation;

pub use builder::*;
pub use color_scheme::*;
pub use colors::*;
pub use override_provider::*;
pub use provider::*;
pub use spacing::*;
pub use typography::*;
pub use validation::*;

use leptos::prelude::*;
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Default)]
pub struct Theme {
    pub colors: ColorPalette,
    pub spacing: Spacing,
    pub typography: Typography,
    pub radius: RadiusScale,
    pub shadows: ShadowScale,
    pub breakpoints: Breakpoints,
    pub color_scheme: ColorSchemeMode,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RadiusScale {
    pub xs: Cow<'static, str>,
    pub sm: Cow<'static, str>,
    pub md: Cow<'static, str>,
    pub lg: Cow<'static, str>,
    pub xl: Cow<'static, str>,
}

impl Default for RadiusScale {
    fn default() -> Self {
        Self {
            xs: Cow::Borrowed("0.125rem"),
            sm: Cow::Borrowed("0.25rem"),
            md: Cow::Borrowed("0.5rem"),
            lg: Cow::Borrowed("1rem"),
            xl: Cow::Borrowed("2rem"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShadowScale {
    pub xs: Cow<'static, str>,
    pub sm: Cow<'static, str>,
    pub md: Cow<'static, str>,
    pub lg: Cow<'static, str>,
    pub xl: Cow<'static, str>,
}

impl Default for ShadowScale {
    fn default() -> Self {
        Self {
            xs: Cow::Borrowed("0 0.0625rem 0.1875rem rgba(0, 0, 0, 0.05), 0 0.0625rem 0.125rem rgba(0, 0, 0, 0.1)"),
            sm: Cow::Borrowed("0 0.0625rem 0.1875rem rgba(0, 0, 0, 0.05), rgba(0, 0, 0, 0.05) 0 0.625rem 0.9375rem -0.3125rem, rgba(0, 0, 0, 0.04) 0 0.4375rem 0.4375rem -0.3125rem"),
            md: Cow::Borrowed("0 0.0625rem 0.1875rem rgba(0, 0, 0, 0.05), rgba(0, 0, 0, 0.05) 0 1.25rem 1.5625rem -0.3125rem, rgba(0, 0, 0, 0.04) 0 0.625rem 0.625rem -0.3125rem"),
            lg: Cow::Borrowed("0 0.0625rem 0.1875rem rgba(0, 0, 0, 0.05), rgba(0, 0, 0, 0.05) 0 1.75rem 1.4375rem -0.4375rem, rgba(0, 0, 0, 0.04) 0 0.75rem 0.75rem -0.4375rem"),
            xl: Cow::Borrowed("0 0.0625rem 0.1875rem rgba(0, 0, 0, 0.05), rgba(0, 0, 0, 0.05) 0 2.25rem 1.75rem -0.4375rem, rgba(0, 0, 0, 0.04) 0 1.0625rem 1.0625rem -0.4375rem"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Breakpoints {
    pub xs: Cow<'static, str>,
    pub sm: Cow<'static, str>,
    pub md: Cow<'static, str>,
    pub lg: Cow<'static, str>,
    pub xl: Cow<'static, str>,
}

impl Default for Breakpoints {
    fn default() -> Self {
        Self {
            xs: Cow::Borrowed("36em"),
            sm: Cow::Borrowed("48em"),
            md: Cow::Borrowed("62em"),
            lg: Cow::Borrowed("75em"),
            xl: Cow::Borrowed("88em"),
        }
    }
}

pub type ThemeContext = RwSignal<Theme>;

/// Helper to get the active color scheme colors based on current theme
pub fn get_scheme_colors(theme: &Theme) -> &ColorScheme {
    match theme.color_scheme.resolve() {
        ActiveColorScheme::Light => &theme.colors.light,
        ActiveColorScheme::Dark => &theme.colors.dark,
    }
}

/// Convert a theme into a list of CSS custom property (variable) name-value pairs.
///
/// All variable names are namespaced with `--mingot-`.
/// This is a pure function usable in tests without a DOM.
pub fn theme_to_css_vars(theme: &Theme) -> Vec<(String, String)> {
    let mut vars = Vec::new();
    let scheme = get_scheme_colors(theme);
    let is_dark = theme.color_scheme.resolve().is_dark();

    // Core colors
    vars.push(("--mingot-background".into(), scheme.background.clone()));
    vars.push(("--mingot-text".into(), scheme.text.clone()));
    vars.push(("--mingot-border".into(), scheme.border.clone()));
    vars.push(("--mingot-white".into(), scheme.white.clone()));
    vars.push(("--mingot-black".into(), scheme.black.clone()));

    // Primary colors (use theme's primary_color key)
    let primary_key = &theme.colors.primary_color;
    if let Some(primary) = scheme.get_color(primary_key, 6) {
        vars.push(("--mingot-primary".into(), primary));
    }
    if let Some(primary_light) = scheme.get_color(primary_key, if is_dark { 1 } else { 0 }) {
        vars.push(("--mingot-primary-light".into(), primary_light));
    }

    // Semantic colors
    if let Some(success) = scheme.get_color("green", 6) {
        vars.push(("--mingot-success".into(), success));
    }
    if let Some(error) = scheme.get_color("red", 6) {
        vars.push(("--mingot-error".into(), error));
    }
    if let Some(warning) = scheme.get_color("yellow", 6) {
        vars.push(("--mingot-warning".into(), warning));
    }

    // Surface colors (different shade indices for dark mode)
    let (s0, s1, s2, hover_idx) = if is_dark { (1, 2, 3, 3) } else { (0, 1, 2, 1) };
    if let Some(surface) = scheme.get_color("gray", s0) {
        vars.push(("--mingot-surface-0".into(), surface));
    }
    if let Some(surface1) = scheme.get_color("gray", s1) {
        vars.push(("--mingot-surface-1".into(), surface1));
    }
    if let Some(surface2) = scheme.get_color("gray", s2) {
        vars.push(("--mingot-surface-2".into(), surface2));
    }
    if let Some(hover) = scheme.get_color("gray", hover_idx) {
        vars.push(("--mingot-hover-bg".into(), hover));
    }

    // Text dimmed
    let dimmed_idx = if is_dark { 7 } else { 6 };
    if let Some(dimmed) = scheme.get_color("gray", dimmed_idx) {
        vars.push(("--mingot-text-dimmed".into(), dimmed));
    }

    // Spacing
    vars.push(("--mingot-spacing-xs".into(), theme.spacing.xs.to_string()));
    vars.push(("--mingot-spacing-sm".into(), theme.spacing.sm.to_string()));
    vars.push(("--mingot-spacing-md".into(), theme.spacing.md.to_string()));
    vars.push(("--mingot-spacing-lg".into(), theme.spacing.lg.to_string()));
    vars.push(("--mingot-spacing-xl".into(), theme.spacing.xl.to_string()));

    // Radius
    vars.push(("--mingot-radius-xs".into(), theme.radius.xs.to_string()));
    vars.push(("--mingot-radius-sm".into(), theme.radius.sm.to_string()));
    vars.push(("--mingot-radius-md".into(), theme.radius.md.to_string()));
    vars.push(("--mingot-radius-lg".into(), theme.radius.lg.to_string()));
    vars.push(("--mingot-radius-xl".into(), theme.radius.xl.to_string()));

    // Shadows
    vars.push(("--mingot-shadow-xs".into(), theme.shadows.xs.to_string()));
    vars.push(("--mingot-shadow-sm".into(), theme.shadows.sm.to_string()));
    vars.push(("--mingot-shadow-md".into(), theme.shadows.md.to_string()));
    vars.push(("--mingot-shadow-lg".into(), theme.shadows.lg.to_string()));
    vars.push(("--mingot-shadow-xl".into(), theme.shadows.xl.to_string()));

    // Typography
    vars.push((
        "--mingot-font-family".into(),
        theme.typography.font_family.to_string(),
    ));
    vars.push((
        "--mingot-font-family-mono".into(),
        theme.typography.font_family_monospace.to_string(),
    ));
    vars.push((
        "--mingot-font-size-xs".into(),
        theme.typography.font_sizes.xs.to_string(),
    ));
    vars.push((
        "--mingot-font-size-sm".into(),
        theme.typography.font_sizes.sm.to_string(),
    ));
    vars.push((
        "--mingot-font-size-md".into(),
        theme.typography.font_sizes.md.to_string(),
    ));
    vars.push((
        "--mingot-font-size-lg".into(),
        theme.typography.font_sizes.lg.to_string(),
    ));
    vars.push((
        "--mingot-font-size-xl".into(),
        theme.typography.font_sizes.xl.to_string(),
    ));
    vars.push((
        "--mingot-font-size-xxl".into(),
        theme.typography.font_sizes.xxl.to_string(),
    ));

    vars
}

#[cfg(test)]
mod css_var_tests {
    use super::*;

    #[test]
    fn test_theme_to_css_vars_default() {
        let theme = Theme::default();
        let vars = theme_to_css_vars(&theme);

        // Should produce a non-empty set of variables
        assert!(!vars.is_empty());

        // Check some expected vars exist
        let var_map: std::collections::HashMap<_, _> = vars.into_iter().collect();
        assert_eq!(var_map.get("--mingot-background").unwrap(), "#ffffff");
        assert_eq!(var_map.get("--mingot-text").unwrap(), "#000000");
        assert_eq!(var_map.get("--mingot-spacing-md").unwrap(), "1rem");
        assert_eq!(var_map.get("--mingot-radius-md").unwrap(), "0.5rem");
        assert!(var_map.contains_key("--mingot-font-family"));
        assert!(var_map.contains_key("--mingot-primary"));
    }

    #[test]
    fn test_theme_to_css_vars_dark_mode() {
        let theme = Theme {
            color_scheme: ColorSchemeMode::Dark,
            ..Theme::default()
        };
        let vars = theme_to_css_vars(&theme);
        let var_map: std::collections::HashMap<_, _> = vars.into_iter().collect();

        // Dark mode should use dark scheme colors
        assert_eq!(var_map.get("--mingot-background").unwrap(), "#1a1b1e");
        assert_eq!(var_map.get("--mingot-text").unwrap(), "#c1c2c5");
    }

    #[test]
    fn test_theme_to_css_vars_has_all_spacing() {
        let theme = Theme::default();
        let vars = theme_to_css_vars(&theme);
        let var_map: std::collections::HashMap<_, _> = vars.into_iter().collect();

        for size in &["xs", "sm", "md", "lg", "xl"] {
            assert!(
                var_map.contains_key(&format!("--mingot-spacing-{}", size) as &str),
                "Missing --mingot-spacing-{}",
                size
            );
        }
    }

    #[test]
    fn test_theme_to_css_vars_has_all_radius() {
        let theme = Theme::default();
        let vars = theme_to_css_vars(&theme);
        let var_map: std::collections::HashMap<_, _> = vars.into_iter().collect();

        for size in &["xs", "sm", "md", "lg", "xl"] {
            assert!(
                var_map.contains_key(&format!("--mingot-radius-{}", size) as &str),
                "Missing --mingot-radius-{}",
                size
            );
        }
    }

    #[test]
    fn test_theme_to_css_vars_has_all_font_sizes() {
        let theme = Theme::default();
        let vars = theme_to_css_vars(&theme);
        let var_map: std::collections::HashMap<_, _> = vars.into_iter().collect();

        for size in &["xs", "sm", "md", "lg", "xl", "xxl"] {
            assert!(
                var_map.contains_key(&format!("--mingot-font-size-{}", size) as &str),
                "Missing --mingot-font-size-{}",
                size
            );
        }
    }

    #[test]
    fn test_theme_to_css_vars_has_all_shadows() {
        let theme = Theme::default();
        let vars = theme_to_css_vars(&theme);
        let var_map: std::collections::HashMap<_, _> = vars.into_iter().collect();

        for size in &["xs", "sm", "md", "lg", "xl"] {
            assert!(
                var_map.contains_key(&format!("--mingot-shadow-{}", size) as &str),
                "Missing --mingot-shadow-{}",
                size
            );
        }
    }

    #[test]
    fn test_theme_to_css_vars_shadow_values_match() {
        let theme = Theme::default();
        let vars = theme_to_css_vars(&theme);
        let var_map: std::collections::HashMap<_, _> = vars.into_iter().collect();

        assert_eq!(var_map["--mingot-shadow-xs"], theme.shadows.xs.as_ref());
        assert_eq!(var_map["--mingot-shadow-sm"], theme.shadows.sm.as_ref());
        assert_eq!(var_map["--mingot-shadow-md"], theme.shadows.md.as_ref());
        assert_eq!(var_map["--mingot-shadow-lg"], theme.shadows.lg.as_ref());
        assert_eq!(var_map["--mingot-shadow-xl"], theme.shadows.xl.as_ref());
    }
}
