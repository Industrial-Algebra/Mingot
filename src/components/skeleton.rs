use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

/// A placeholder component that displays a loading skeleton
///
/// Used to indicate content is loading while preserving layout structure.
///
/// # Example
/// ```rust,ignore
/// // Basic skeleton
/// <Skeleton height="20px" />
///
/// // Circle skeleton for avatars
/// <Skeleton height="50px" width="50px" circle=true />
///
/// // Text skeleton with animation disabled
/// <Skeleton height="1rem" width="70%" animate=false />
/// ```
#[component]
pub fn Skeleton(
    /// Height of the skeleton (CSS value)
    #[prop(optional, into)]
    height: Option<String>,
    /// Width of the skeleton (CSS value), defaults to 100%
    #[prop(optional, into)]
    width: Option<String>,
    /// Border radius (CSS value)
    #[prop(optional, into)]
    radius: Option<String>,
    /// Whether to render as a circle
    #[prop(optional)]
    circle: bool,
    /// Whether to animate the skeleton (default: true)
    #[prop(optional)]
    animate: Option<bool>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Additional inline styles
    #[prop(optional, into)]
    style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let animate = animate.unwrap_or(true);

    let skeleton_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        // Base color from theme
        let base_color = scheme_colors
            .get_color("gray", 2)
            .unwrap_or_else(|| "#e9ecef".to_string());
        let highlight_color = scheme_colors
            .get_color("gray", 0)
            .unwrap_or_else(|| "#f8f9fa".to_string());

        // Dimensions
        let h = height.as_deref().unwrap_or("auto");
        let w = if circle {
            height.as_deref().unwrap_or("auto")
        } else {
            width.as_deref().unwrap_or("100%")
        };

        builder.add("height", h).add("width", w);

        // Border radius
        if circle {
            builder.add("border-radius", "50%");
        } else if let Some(ref r) = radius {
            builder.add("border-radius", r);
        } else {
            builder.add("border-radius", theme_val.radius.sm);
        }

        // Background
        if animate {
            builder.add(
                "background",
                format!(
                    "linear-gradient(90deg, {} 25%, {} 50%, {} 75%)",
                    base_color, highlight_color, base_color
                ),
            );
            builder.add("background-size", "200% 100%");
            builder.add(
                "animation",
                "mingot-skeleton-shimmer 1.5s ease-in-out infinite",
            );
        } else {
            builder.add("background-color", base_color);
        }

        // Display
        builder
            .add("display", "block")
            .add("overflow", "hidden")
            .add("position", "relative");

        if let Some(s) = style.as_ref() {
            format!("{}; {}", builder.build(), s)
        } else {
            builder.build()
        }
    };

    let class_str = format!("mingot-skeleton {}", class.unwrap_or_default());

    view! {
        <>
            <style>
                "@keyframes mingot-skeleton-shimmer {
                    0% { background-position: 200% 0; }
                    100% { background-position: -200% 0; }
                }"
            </style>
            <div class=class_str style=skeleton_styles></div>
        </>
    }
}

/// A group of skeleton elements for text placeholder
#[component]
pub fn SkeletonText(
    /// Number of lines to display
    #[prop(optional)]
    lines: Option<u32>,
    /// Gap between lines (CSS value)
    #[prop(optional, into)]
    gap: Option<String>,
    /// Whether to animate (default: true)
    #[prop(optional)]
    animate: Option<bool>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Additional inline styles
    #[prop(optional, into)]
    style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let lines = lines.unwrap_or(3);
    let animate = animate.unwrap_or(true);

    let container_styles = move || {
        let theme_val = theme.get();
        let g = gap.as_deref().unwrap_or(theme_val.spacing.xs);
        let mut builder = StyleBuilder::new();
        builder
            .add("display", "flex")
            .add("flex-direction", "column")
            .add("gap", g);

        if let Some(s) = style.as_ref() {
            format!("{}; {}", builder.build(), s)
        } else {
            builder.build()
        }
    };

    let class_str = format!("mingot-skeleton-text {}", class.unwrap_or_default());

    // Generate widths for realistic look
    let widths: Vec<String> = (0..lines)
        .map(|i| {
            if i == lines - 1 {
                "60%".to_string()
            } else {
                "100%".to_string()
            }
        })
        .collect();

    view! {
        <div class=class_str style=container_styles>
            {widths
                .into_iter()
                .map(|width| {
                    view! { <Skeleton height="1em".to_string() width=width animate=animate /> }
                })
                .collect_view()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_skeleton_default_lines() {
        // Default is 3 lines when None is provided
        fn get_lines(lines: Option<u32>) -> u32 {
            lines.unwrap_or(3)
        }
        assert_eq!(get_lines(None), 3);
        assert_eq!(get_lines(Some(5)), 5);
    }
}
