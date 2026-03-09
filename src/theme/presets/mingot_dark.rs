use crate::theme::Theme;
use crate::theme::{ColorSchemeMode, ThemeBuilder};
use std::borrow::Cow;

/// A dark-first theme with optimised shadow values for dark backgrounds.
pub fn mingot_dark() -> Theme {
    use crate::theme::ShadowScale;

    ThemeBuilder::new()
        .color_scheme(ColorSchemeMode::Dark)
        .shadows(ShadowScale {
            xs: Cow::Borrowed("0 0.0625rem 0.1875rem rgba(0, 0, 0, 0.25)"),
            sm: Cow::Borrowed("0 0.0625rem 0.25rem rgba(0, 0, 0, 0.3), 0 0.5rem 0.75rem -0.25rem rgba(0, 0, 0, 0.25)"),
            md: Cow::Borrowed("0 0.0625rem 0.25rem rgba(0, 0, 0, 0.3), 0 1rem 1.25rem -0.25rem rgba(0, 0, 0, 0.25)"),
            lg: Cow::Borrowed("0 0.0625rem 0.25rem rgba(0, 0, 0, 0.3), 0 1.5rem 1.25rem -0.375rem rgba(0, 0, 0, 0.25)"),
            xl: Cow::Borrowed("0 0.0625rem 0.25rem rgba(0, 0, 0, 0.3), 0 2rem 1.5rem -0.375rem rgba(0, 0, 0, 0.25)"),
        })
        .build()
}
