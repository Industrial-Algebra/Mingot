use crate::theme::Theme;
use crate::theme::{Spacing, ThemeBuilder};
use std::borrow::Cow;

/// Financial theme — navy/gold palette, conservative styling,
/// comfortable spacing for data-dense interfaces.
pub fn financial() -> Theme {
    ThemeBuilder::new()
        .primary_color("indigo")
        .font_family("'Segoe UI', Tahoma, Geneva, Verdana, sans-serif")
        .spacing(Spacing {
            xs: Cow::Borrowed("0.5rem"),
            sm: Cow::Borrowed("0.75rem"),
            md: Cow::Borrowed("1rem"),
            lg: Cow::Borrowed("1.5rem"),
            xl: Cow::Borrowed("2.5rem"),
        })
        .background("#fafafa", "#0d1117")
        .text("#1a1a2e", "#d1d5db")
        .border("#c9d1d9", "#30363d")
        .build()
}
