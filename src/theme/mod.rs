mod color_scheme;
mod colors;
mod provider;
mod spacing;
mod typography;

pub use color_scheme::*;
pub use colors::*;
pub use provider::*;
pub use spacing::*;
pub use typography::*;

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
