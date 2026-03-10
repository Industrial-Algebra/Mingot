use crate::theme::Theme;
use crate::theme::{RadiusScale, ShadowScale, Spacing, ThemeBuilder};
use std::borrow::Cow;

/// Scientific theme — serif headings, generous spacing, high contrast,
/// minimal shadows for a clean, academic feel.
pub fn scientific() -> Theme {
    ThemeBuilder::new()
        .primary_color("indigo")
        .font_family("'Georgia', 'Times New Roman', serif")
        .spacing(Spacing {
            xs: Cow::Borrowed("0.75rem"),
            sm: Cow::Borrowed("1rem"),
            md: Cow::Borrowed("1.5rem"),
            lg: Cow::Borrowed("2rem"),
            xl: Cow::Borrowed("3rem"),
        })
        .radius(RadiusScale {
            xs: Cow::Borrowed("0"),
            sm: Cow::Borrowed("0.125rem"),
            md: Cow::Borrowed("0.25rem"),
            lg: Cow::Borrowed("0.5rem"),
            xl: Cow::Borrowed("1rem"),
        })
        .shadows(ShadowScale {
            xs: Cow::Borrowed("none"),
            sm: Cow::Borrowed("0 0.0625rem 0.125rem rgba(0, 0, 0, 0.06)"),
            md: Cow::Borrowed("0 0.0625rem 0.25rem rgba(0, 0, 0, 0.08)"),
            lg: Cow::Borrowed("0 0.125rem 0.5rem rgba(0, 0, 0, 0.1)"),
            xl: Cow::Borrowed("0 0.25rem 1rem rgba(0, 0, 0, 0.12)"),
        })
        .background("#ffffff", "#1a1b1e")
        .text("#000000", "#e5e5e5")
        .build()
}
