use super::{
    BorderScale, Breakpoints, ColorPalette, ColorSchemeMode, ColorShades, FontSizes, FontWeights,
    LayoutTokens, LineHeights, RadiusScale, ShadowScale, Spacing, Theme, Typography,
};
use std::borrow::Cow;

/// Builder for creating customized themes with a fluent API.
///
/// Start from defaults with `ThemeBuilder::new()` or inherit from
/// an existing theme with `ThemeBuilder::from(base)`.
///
/// # Example
/// ```rust,ignore
/// let theme = ThemeBuilder::new()
///     .primary_color("indigo")
///     .color_scheme(ColorSchemeMode::Dark)
///     .font_family("'JetBrains Mono', monospace")
///     .spacing_md("1.25rem")
///     .build();
/// ```
pub struct ThemeBuilder {
    theme: Theme,
}

impl ThemeBuilder {
    /// Create a new builder starting from the default theme.
    pub fn new() -> Self {
        Self {
            theme: Theme::default(),
        }
    }

    /// Create a builder starting from an existing theme.
    pub fn from(base: Theme) -> Self {
        Self { theme: base }
    }

    /// Set the primary color name (must match a key in ColorPalette).
    pub fn primary_color(mut self, name: impl Into<String>) -> Self {
        self.theme.colors.primary_color = name.into();
        self
    }

    /// Set the color scheme mode.
    pub fn color_scheme(mut self, mode: ColorSchemeMode) -> Self {
        self.theme.color_scheme = mode;
        self
    }

    // --- Spacing ---

    /// Replace the entire spacing scale.
    pub fn spacing(mut self, spacing: Spacing) -> Self {
        self.theme.spacing = spacing;
        self
    }

    /// Set spacing xs value.
    pub fn spacing_xs(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.spacing.xs = val.into();
        self
    }

    /// Set spacing sm value.
    pub fn spacing_sm(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.spacing.sm = val.into();
        self
    }

    /// Set spacing md value.
    pub fn spacing_md(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.spacing.md = val.into();
        self
    }

    /// Set spacing lg value.
    pub fn spacing_lg(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.spacing.lg = val.into();
        self
    }

    /// Set spacing xl value.
    pub fn spacing_xl(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.spacing.xl = val.into();
        self
    }

    // --- Radius ---

    /// Replace the entire radius scale.
    pub fn radius(mut self, radius: RadiusScale) -> Self {
        self.theme.radius = radius;
        self
    }

    /// Set radius md value.
    pub fn radius_md(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.radius.md = val.into();
        self
    }

    /// Set radius sm value.
    pub fn radius_sm(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.radius.sm = val.into();
        self
    }

    /// Set radius xs value.
    pub fn radius_xs(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.radius.xs = val.into();
        self
    }

    /// Set radius lg value.
    pub fn radius_lg(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.radius.lg = val.into();
        self
    }

    /// Set radius xl value.
    pub fn radius_xl(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.radius.xl = val.into();
        self
    }

    // --- Shadows ---

    /// Replace the entire shadow scale.
    pub fn shadows(mut self, shadows: ShadowScale) -> Self {
        self.theme.shadows = shadows;
        self
    }

    /// Set shadow xs value.
    pub fn shadow_xs(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.shadows.xs = val.into();
        self
    }

    /// Set shadow sm value.
    pub fn shadow_sm(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.shadows.sm = val.into();
        self
    }

    /// Set shadow md value.
    pub fn shadow_md(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.shadows.md = val.into();
        self
    }

    /// Set shadow lg value.
    pub fn shadow_lg(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.shadows.lg = val.into();
        self
    }

    /// Set shadow xl value.
    pub fn shadow_xl(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.shadows.xl = val.into();
        self
    }

    // --- Typography ---

    /// Replace the entire typography configuration.
    pub fn typography(mut self, typography: Typography) -> Self {
        self.theme.typography = typography;
        self
    }

    /// Set the primary font family.
    pub fn font_family(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.typography.font_family = val.into();
        self
    }

    /// Set the monospace font family.
    pub fn font_family_monospace(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.typography.font_family_monospace = val.into();
        self
    }

    /// Replace font sizes.
    pub fn font_sizes(mut self, sizes: FontSizes) -> Self {
        self.theme.typography.font_sizes = sizes;
        self
    }

    /// Replace line heights.
    pub fn line_heights(mut self, heights: LineHeights) -> Self {
        self.theme.typography.line_heights = heights;
        self
    }

    /// Replace font weights.
    pub fn font_weights(mut self, weights: FontWeights) -> Self {
        self.theme.typography.font_weights = weights;
        self
    }

    // --- Colors ---

    /// Replace the entire color palette.
    pub fn colors(mut self, colors: ColorPalette) -> Self {
        self.theme.colors = colors;
        self
    }

    /// Add or replace a single color in both light and dark schemes.
    pub fn add_color(
        mut self,
        name: impl Into<String>,
        light_shades: Vec<String>,
        dark_shades: Vec<String>,
    ) -> Self {
        let name = name.into();
        self.theme.colors.light.colors.insert(
            name.clone(),
            ColorShades {
                shades: light_shades,
            },
        );
        self.theme.colors.dark.colors.insert(
            name,
            ColorShades {
                shades: dark_shades,
            },
        );
        self
    }

    /// Override background colors for light and dark schemes.
    pub fn background(mut self, light: impl Into<String>, dark: impl Into<String>) -> Self {
        self.theme.colors.light.background = light.into();
        self.theme.colors.dark.background = dark.into();
        self
    }

    /// Override text colors for light and dark schemes.
    pub fn text(mut self, light: impl Into<String>, dark: impl Into<String>) -> Self {
        self.theme.colors.light.text = light.into();
        self.theme.colors.dark.text = dark.into();
        self
    }

    /// Override border colors for light and dark schemes.
    pub fn border(mut self, light: impl Into<String>, dark: impl Into<String>) -> Self {
        self.theme.colors.light.border = light.into();
        self.theme.colors.dark.border = dark.into();
        self
    }

    // --- Borders ---

    /// Replace the entire border scale.
    pub fn borders(mut self, borders: BorderScale) -> Self {
        self.theme.borders = borders;
        self
    }

    /// Set border width value.
    pub fn border_width(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.borders.width = val.into();
        self
    }

    /// Set border style value.
    pub fn border_style(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.borders.style = val.into();
        self
    }

    // --- Layout ---

    /// Replace the entire layout tokens.
    pub fn layout(mut self, layout: LayoutTokens) -> Self {
        self.theme.layout = layout;
        self
    }

    /// Set container xs max-width.
    pub fn container_xs(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.layout.container_xs = val.into();
        self
    }

    /// Set container sm max-width.
    pub fn container_sm(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.layout.container_sm = val.into();
        self
    }

    /// Set container md max-width.
    pub fn container_md(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.layout.container_md = val.into();
        self
    }

    /// Set container lg max-width.
    pub fn container_lg(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.layout.container_lg = val.into();
        self
    }

    /// Set container xl max-width.
    pub fn container_xl(mut self, val: impl Into<Cow<'static, str>>) -> Self {
        self.theme.layout.container_xl = val.into();
        self
    }

    // --- Breakpoints ---

    /// Replace the entire breakpoints scale.
    pub fn breakpoints(mut self, breakpoints: Breakpoints) -> Self {
        self.theme.breakpoints = breakpoints;
        self
    }

    /// Consume the builder and return the finished theme.
    pub fn build(self) -> Theme {
        self.theme
    }
}

impl Default for ThemeBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_default() {
        let theme = ThemeBuilder::new().build();
        assert_eq!(theme, Theme::default());
    }

    #[test]
    fn test_builder_primary_color() {
        let theme = ThemeBuilder::new().primary_color("red").build();
        assert_eq!(theme.colors.primary_color, "red");
    }

    #[test]
    fn test_builder_color_scheme() {
        let theme = ThemeBuilder::new()
            .color_scheme(ColorSchemeMode::Dark)
            .build();
        assert_eq!(theme.color_scheme, ColorSchemeMode::Dark);
    }

    #[test]
    fn test_builder_spacing_individual() {
        let theme = ThemeBuilder::new()
            .spacing_md("1.5rem")
            .spacing_lg("2rem")
            .build();
        assert_eq!(&*theme.spacing.md, "1.5rem");
        assert_eq!(&*theme.spacing.lg, "2rem");
        // Others unchanged
        assert_eq!(&*theme.spacing.xs, "0.625rem");
    }

    #[test]
    fn test_builder_font_family() {
        let theme = ThemeBuilder::new()
            .font_family("'JetBrains Mono', monospace")
            .build();
        assert_eq!(
            &*theme.typography.font_family,
            "'JetBrains Mono', monospace"
        );
    }

    #[test]
    fn test_builder_add_color() {
        let theme = ThemeBuilder::new()
            .add_color(
                "custom",
                vec!["#111".to_string(); 10],
                vec!["#222".to_string(); 10],
            )
            .build();
        assert!(theme.colors.light.colors.contains_key("custom"));
        assert!(theme.colors.dark.colors.contains_key("custom"));
    }

    #[test]
    fn test_builder_background_text() {
        let theme = ThemeBuilder::new()
            .background("#fafafa", "#111111")
            .text("#333333", "#eeeeee")
            .build();
        assert_eq!(theme.colors.light.background, "#fafafa");
        assert_eq!(theme.colors.dark.background, "#111111");
        assert_eq!(theme.colors.light.text, "#333333");
        assert_eq!(theme.colors.dark.text, "#eeeeee");
    }

    #[test]
    fn test_builder_from_existing() {
        let base = ThemeBuilder::new()
            .primary_color("green")
            .color_scheme(ColorSchemeMode::Dark)
            .build();

        let derived = ThemeBuilder::from(base.clone()).spacing_md("2rem").build();

        assert_eq!(derived.colors.primary_color, "green");
        assert_eq!(derived.color_scheme, ColorSchemeMode::Dark);
        assert_eq!(&*derived.spacing.md, "2rem");
    }

    #[test]
    fn test_builder_chaining() {
        let theme = ThemeBuilder::new()
            .primary_color("violet")
            .color_scheme(ColorSchemeMode::Dark)
            .font_family("serif")
            .spacing_md("1.5rem")
            .radius_md("0.75rem")
            .background("#fafafa", "#111111")
            .build();

        assert_eq!(theme.colors.primary_color, "violet");
        assert_eq!(theme.color_scheme, ColorSchemeMode::Dark);
        assert_eq!(&*theme.typography.font_family, "serif");
        assert_eq!(&*theme.spacing.md, "1.5rem");
        assert_eq!(&*theme.radius.md, "0.75rem");
    }

    #[test]
    fn test_builder_owned_strings() {
        let family = String::from("'Custom Font', sans-serif");
        let theme = ThemeBuilder::new()
            .font_family(family)
            .spacing_md(String::from("1.5rem"))
            .build();
        assert_eq!(&*theme.typography.font_family, "'Custom Font', sans-serif");
        assert_eq!(&*theme.spacing.md, "1.5rem");
    }

    #[test]
    fn test_builder_shadow_individual() {
        let theme = ThemeBuilder::new()
            .shadow_md("0 2px 8px rgba(0,0,0,0.15)")
            .build();
        assert_eq!(&*theme.shadows.md, "0 2px 8px rgba(0,0,0,0.15)");
        // Others unchanged
        assert_eq!(theme.shadows.xs, Theme::default().shadows.xs);
    }

    #[test]
    fn test_builder_shadow_chaining() {
        let theme = ThemeBuilder::new()
            .shadow_xs("none")
            .shadow_sm("0 1px 2px rgba(0,0,0,0.1)")
            .shadow_lg("0 8px 32px rgba(0,0,0,0.2)")
            .build();
        assert_eq!(&*theme.shadows.xs, "none");
        assert_eq!(&*theme.shadows.sm, "0 1px 2px rgba(0,0,0,0.1)");
        assert_eq!(&*theme.shadows.lg, "0 8px 32px rgba(0,0,0,0.2)");
    }

    #[test]
    fn test_builder_shadow_owned_string() {
        let shadow = String::from("0 4px 16px rgba(0,0,0,0.25)");
        let theme = ThemeBuilder::new().shadow_xl(shadow).build();
        assert_eq!(&*theme.shadows.xl, "0 4px 16px rgba(0,0,0,0.25)");
    }

    #[test]
    fn test_builder_border_individual() {
        let theme = ThemeBuilder::new().border_width("2px").build();
        assert_eq!(&*theme.borders.width, "2px");
        assert_eq!(&*theme.borders.style, "solid"); // default unchanged
    }

    #[test]
    fn test_builder_border_chaining() {
        let theme = ThemeBuilder::new()
            .border_width("2px")
            .border_style("dashed")
            .build();
        assert_eq!(&*theme.borders.width, "2px");
        assert_eq!(&*theme.borders.style, "dashed");
    }

    #[test]
    fn test_builder_border_owned_string() {
        let width = String::from("3px");
        let theme = ThemeBuilder::new().border_width(width).build();
        assert_eq!(&*theme.borders.width, "3px");
    }

    #[test]
    fn test_builder_container_individual() {
        let theme = ThemeBuilder::new().container_md("1024px").build();
        assert_eq!(&*theme.layout.container_md, "1024px");
        // Others unchanged
        assert_eq!(&*theme.layout.container_xs, "540px");
        assert_eq!(&*theme.layout.container_xl, "1320px");
    }

    #[test]
    fn test_builder_container_chaining() {
        let theme = ThemeBuilder::new()
            .container_xs("600px")
            .container_sm("800px")
            .container_md("1024px")
            .container_lg("1200px")
            .container_xl("1400px")
            .build();
        assert_eq!(&*theme.layout.container_xs, "600px");
        assert_eq!(&*theme.layout.container_sm, "800px");
        assert_eq!(&*theme.layout.container_md, "1024px");
        assert_eq!(&*theme.layout.container_lg, "1200px");
        assert_eq!(&*theme.layout.container_xl, "1400px");
    }
}
