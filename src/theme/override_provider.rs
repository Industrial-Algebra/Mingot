use super::{ColorSchemeMode, RadiusScale, Spacing, ThemeContext, Typography};
use leptos::prelude::*;

/// A scoped theme override component.
///
/// Reads the parent `ThemeContext`, applies the provided overrides to produce
/// a derived theme, and provides it as a new `ThemeContext` for its children.
///
/// Any prop that is `None` (the default) inherits the parent value unchanged.
///
/// # Example
/// ```rust,ignore
/// <MingotProvider>
///     <Text>"Normal theme"</Text>
///     <ThemeOverride color_scheme=ColorSchemeMode::Dark>
///         <Text>"Dark mode inside here"</Text>
///     </ThemeOverride>
/// </MingotProvider>
/// ```
#[component]
pub fn ThemeOverride(
    /// Override the color scheme for this subtree.
    #[prop(optional, into)]
    color_scheme: Option<ColorSchemeMode>,
    /// Override the primary color name for this subtree.
    #[prop(optional, into)]
    primary_color: Option<String>,
    /// Override the spacing scale for this subtree.
    #[prop(optional, into)]
    spacing: Option<Spacing>,
    /// Override the radius scale for this subtree.
    #[prop(optional, into)]
    radius: Option<RadiusScale>,
    /// Override the typography for this subtree.
    #[prop(optional, into)]
    typography: Option<Typography>,
    children: Children,
) -> impl IntoView {
    let parent_theme =
        use_context::<ThemeContext>().expect("ThemeOverride must be used within a MingotProvider");

    let derived = RwSignal::new({
        let mut t = parent_theme.get_untracked();
        if let Some(cs) = color_scheme {
            t.color_scheme = cs;
        }
        if let Some(ref pc) = primary_color {
            t.colors.primary_color = pc.clone();
        }
        if let Some(ref sp) = spacing {
            t.spacing = sp.clone();
        }
        if let Some(ref r) = radius {
            t.radius = r.clone();
        }
        if let Some(ref ty) = typography {
            t.typography = ty.clone();
        }
        t
    });

    // Keep the derived theme in sync when the parent changes
    let _ = Effect::new(move || {
        let mut t = parent_theme.get();
        if let Some(cs) = color_scheme {
            t.color_scheme = cs;
        }
        if let Some(ref pc) = primary_color {
            t.colors.primary_color = pc.clone();
        }
        if let Some(ref sp) = spacing {
            t.spacing = sp.clone();
        }
        if let Some(ref r) = radius {
            t.radius = r.clone();
        }
        if let Some(ref ty) = typography {
            t.typography = ty.clone();
        }
        derived.set(t);
    });

    provide_context::<ThemeContext>(derived);

    view! { <>{children()}</> }
}
