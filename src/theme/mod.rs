mod colors;
mod provider;
mod spacing;
mod typography;

pub use colors::*;
pub use provider::*;
pub use spacing::*;
pub use typography::*;

use leptos::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Theme {
    pub colors: ColorScheme,
    pub spacing: Spacing,
    pub typography: Typography,
    pub radius: RadiusScale,
    pub shadows: ShadowScale,
    pub breakpoints: Breakpoints,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            colors: ColorScheme::default(),
            spacing: Spacing::default(),
            typography: Typography::default(),
            radius: RadiusScale::default(),
            shadows: ShadowScale::default(),
            breakpoints: Breakpoints::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RadiusScale {
    pub xs: &'static str,
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
}

impl Default for RadiusScale {
    fn default() -> Self {
        Self {
            xs: "0.125rem",
            sm: "0.25rem",
            md: "0.5rem",
            lg: "1rem",
            xl: "2rem",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShadowScale {
    pub xs: &'static str,
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
}

impl Default for ShadowScale {
    fn default() -> Self {
        Self {
            xs: "0 0.0625rem 0.1875rem rgba(0, 0, 0, 0.05), 0 0.0625rem 0.125rem rgba(0, 0, 0, 0.1)",
            sm: "0 0.0625rem 0.1875rem rgba(0, 0, 0, 0.05), rgba(0, 0, 0, 0.05) 0 0.625rem 0.9375rem -0.3125rem, rgba(0, 0, 0, 0.04) 0 0.4375rem 0.4375rem -0.3125rem",
            md: "0 0.0625rem 0.1875rem rgba(0, 0, 0, 0.05), rgba(0, 0, 0, 0.05) 0 1.25rem 1.5625rem -0.3125rem, rgba(0, 0, 0, 0.04) 0 0.625rem 0.625rem -0.3125rem",
            lg: "0 0.0625rem 0.1875rem rgba(0, 0, 0, 0.05), rgba(0, 0, 0, 0.05) 0 1.75rem 1.4375rem -0.4375rem, rgba(0, 0, 0, 0.04) 0 0.75rem 0.75rem -0.4375rem",
            xl: "0 0.0625rem 0.1875rem rgba(0, 0, 0, 0.05), rgba(0, 0, 0, 0.05) 0 2.25rem 1.75rem -0.4375rem, rgba(0, 0, 0, 0.04) 0 1.0625rem 1.0625rem -0.4375rem",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Breakpoints {
    pub xs: &'static str,
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
}

impl Default for Breakpoints {
    fn default() -> Self {
        Self {
            xs: "36em",
            sm: "48em",
            md: "62em",
            lg: "75em",
            xl: "88em",
        }
    }
}

pub type ThemeContext = RwSignal<Theme>;
