use crate::components::loader::{Loader, LoaderSize, LoaderVariant};
use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

/// A loading overlay component that covers its parent container
///
/// LoadingOverlay displays a centered loader over its content with a semi-transparent backdrop.
/// The parent container should have `position: relative` for proper positioning.
///
/// # Example
/// ```rust,ignore
/// let loading = RwSignal::new(false);
///
/// <div style="position: relative;">
///     <LoadingOverlay visible=loading.into() />
///     <p>"Content here"</p>
/// </div>
///
/// // With custom loader
/// <div style="position: relative;">
///     <LoadingOverlay
///         visible=loading.into()
///         loader_variant=LoaderVariant::Dots
///         loader_size=LoaderSize::Lg
///     />
///     <p>"Content here"</p>
/// </div>
/// ```
#[component]
pub fn LoadingOverlay(
    /// Whether the overlay is visible
    #[prop(into)]
    visible: Signal<bool>,
    /// Loader variant to display
    #[prop(optional)]
    loader_variant: Option<LoaderVariant>,
    /// Loader size
    #[prop(optional)]
    loader_size: Option<LoaderSize>,
    /// Loader color (theme color name)
    #[prop(optional, into)]
    loader_color: Option<String>,
    /// Overlay background color (CSS value)
    #[prop(optional, into)]
    overlay_color: Option<String>,
    /// Overlay opacity (0-1)
    #[prop(optional)]
    overlay_opacity: Option<f32>,
    /// Border radius to match parent container
    #[prop(optional, into)]
    radius: Option<String>,
    /// Z-index of the overlay
    #[prop(optional)]
    z_index: Option<i32>,
    /// Transition duration in milliseconds
    #[prop(optional)]
    transition_duration: Option<u32>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Additional inline styles
    #[prop(optional, into)]
    style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let overlay_opacity = overlay_opacity.unwrap_or(0.75);
    let z_index = z_index.unwrap_or(400);
    let transition_duration = transition_duration.unwrap_or(200);

    let overlay_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let is_visible = visible.get();

        let bg_color = overlay_color
            .clone()
            .unwrap_or_else(|| scheme_colors.background.clone());

        let mut builder = StyleBuilder::new();
        builder
            .add("position", "absolute")
            .add("top", "0")
            .add("left", "0")
            .add("right", "0")
            .add("bottom", "0")
            .add("display", "flex")
            .add("align-items", "center")
            .add("justify-content", "center")
            .add("z-index", z_index.to_string())
            .add(
                "background-color",
                format!("rgba({}, {})", hex_to_rgb(&bg_color), overlay_opacity),
            )
            .add("opacity", if is_visible { "1" } else { "0" }.to_string())
            .add(
                "visibility",
                if is_visible { "visible" } else { "hidden" }.to_string(),
            )
            .add(
                "transition",
                format!(
                    "opacity {}ms ease, visibility {}ms ease",
                    transition_duration, transition_duration
                ),
            )
            .add("pointer-events", if is_visible { "all" } else { "none" });

        // Border radius
        if let Some(ref r) = radius {
            builder.add("border-radius", r);
        } else {
            builder.add("border-radius", theme_val.radius.sm);
        }

        if let Some(s) = style.as_ref() {
            format!("{}; {}", builder.build(), s)
        } else {
            builder.build()
        }
    };

    let class_str = format!("mingot-loading-overlay {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=overlay_styles aria-busy=move || visible.get().to_string()>
            <Loader
                variant=loader_variant.unwrap_or_default()
                size=loader_size.unwrap_or(LoaderSize::Md)
                color=loader_color.unwrap_or_else(|| "blue".to_string())
            />
        </div>
    }
}

/// Convert hex color to RGB values for use with rgba()
fn hex_to_rgb(hex: &str) -> String {
    let hex = hex.trim_start_matches('#');

    // Handle different hex formats
    if hex.len() == 6 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[0..2], 16),
            u8::from_str_radix(&hex[2..4], 16),
            u8::from_str_radix(&hex[4..6], 16),
        ) {
            return format!("{}, {}, {}", r, g, b);
        }
    } else if hex.len() == 3 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&hex[0..1].repeat(2), 16),
            u8::from_str_radix(&hex[1..2].repeat(2), 16),
            u8::from_str_radix(&hex[2..3].repeat(2), 16),
        ) {
            return format!("{}, {}, {}", r, g, b);
        }
    }

    // Fallback for non-hex colors (white)
    "255, 255, 255".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_rgb_6_digit() {
        assert_eq!(hex_to_rgb("#ffffff"), "255, 255, 255");
        assert_eq!(hex_to_rgb("#000000"), "0, 0, 0");
        assert_eq!(hex_to_rgb("#228be6"), "34, 139, 230");
    }

    #[test]
    fn test_hex_to_rgb_3_digit() {
        assert_eq!(hex_to_rgb("#fff"), "255, 255, 255");
        assert_eq!(hex_to_rgb("#000"), "0, 0, 0");
    }

    #[test]
    fn test_hex_to_rgb_without_hash() {
        assert_eq!(hex_to_rgb("ffffff"), "255, 255, 255");
        assert_eq!(hex_to_rgb("228be6"), "34, 139, 230");
    }

    #[test]
    fn test_hex_to_rgb_invalid() {
        assert_eq!(hex_to_rgb("invalid"), "255, 255, 255");
        assert_eq!(hex_to_rgb("rgb(255, 255, 255)"), "255, 255, 255");
    }
}
