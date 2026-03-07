use crate::theme::Theme;
use crate::theme::{Spacing, ThemeBuilder};
use std::borrow::Cow;

/// Monochrome industrial theme — steel gray and deep blue,
/// tight spacing, monospace typography.
pub fn industrial() -> Theme {
    ThemeBuilder::new()
        .primary_color("gray")
        .font_family("ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace")
        .spacing(Spacing {
            xs: Cow::Borrowed("0.5rem"),
            sm: Cow::Borrowed("0.625rem"),
            md: Cow::Borrowed("0.75rem"),
            lg: Cow::Borrowed("1rem"),
            xl: Cow::Borrowed("1.5rem"),
        })
        .background("#f8f9fa", "#141517")
        .text("#212529", "#c1c2c5")
        .border("#adb5bd", "#373a40")
        .build()
}
