//! Built-in theme presets.
//!
//! Each preset returns a fully configured [`Theme`](super::Theme) that
//! passes validation and meets WCAG AA contrast requirements.

mod financial;
mod industrial;
mod mingot_dark;
mod mingot_default;
mod scientific;

pub use financial::financial;
pub use industrial::industrial;
pub use mingot_dark::mingot_dark;
pub use mingot_default::mingot_default;
pub use scientific::scientific;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::validation::validate_theme;

    #[test]
    fn test_mingot_default_passes_validation() {
        let warnings = validate_theme(&mingot_default());
        assert!(
            warnings.is_empty(),
            "mingot_default warnings: {:?}",
            warnings
        );
    }

    #[test]
    fn test_mingot_dark_passes_validation() {
        let warnings = validate_theme(&mingot_dark());
        assert!(warnings.is_empty(), "mingot_dark warnings: {:?}", warnings);
    }

    #[test]
    fn test_industrial_passes_validation() {
        let warnings = validate_theme(&industrial());
        assert!(warnings.is_empty(), "industrial warnings: {:?}", warnings);
    }

    #[test]
    fn test_scientific_passes_validation() {
        let warnings = validate_theme(&scientific());
        assert!(warnings.is_empty(), "scientific warnings: {:?}", warnings);
    }

    #[test]
    fn test_financial_passes_validation() {
        let warnings = validate_theme(&financial());
        assert!(warnings.is_empty(), "financial warnings: {:?}", warnings);
    }

    #[test]
    fn test_all_presets_meet_wcag_aa() {
        use crate::theme::get_scheme_colors;
        use crate::theme::validation::meets_wcag_aa;

        let presets: Vec<(&str, crate::theme::Theme)> = vec![
            ("mingot_default", mingot_default()),
            ("mingot_dark", mingot_dark()),
            ("industrial", industrial()),
            ("scientific", scientific()),
            ("financial", financial()),
        ];

        for (name, theme) in &presets {
            let colors = get_scheme_colors(theme);
            assert!(
                meets_wcag_aa(&colors.background, &colors.text),
                "Preset '{}' fails WCAG AA: bg={}, text={}",
                name,
                colors.background,
                colors.text
            );
        }
    }
}
