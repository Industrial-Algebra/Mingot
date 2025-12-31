use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;

/// ActionIcon variant determines the visual style
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ActionIconVariant {
    /// Solid background with white icon
    #[default]
    Filled,
    /// Transparent background with colored border
    Outline,
    /// Light background with colored icon
    Light,
    /// Transparent background, no border
    Subtle,
    /// Transparent, icon only
    Transparent,
}

/// ActionIcon size determines the dimensions
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ActionIconSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

impl ActionIconSize {
    fn to_size(self) -> &'static str {
        match self {
            ActionIconSize::Xs => "1.125rem",
            ActionIconSize::Sm => "1.375rem",
            ActionIconSize::Md => "1.75rem",
            ActionIconSize::Lg => "2.125rem",
            ActionIconSize::Xl => "2.75rem",
        }
    }

    fn to_icon_size(self) -> &'static str {
        match self {
            ActionIconSize::Xs => "0.75rem",
            ActionIconSize::Sm => "0.875rem",
            ActionIconSize::Md => "1rem",
            ActionIconSize::Lg => "1.25rem",
            ActionIconSize::Xl => "1.5rem",
        }
    }
}

/// An icon-only button component
///
/// ActionIcon is used for icon-only buttons like close, settings, menu toggles, etc.
///
/// # Example
/// ```rust,ignore
/// <ActionIcon on_click=Callback::new(|_| {})>
///     <span>"X"</span>
/// </ActionIcon>
///
/// <ActionIcon
///     variant=ActionIconVariant::Outline
///     color="red"
///     size=ActionIconSize::Lg
///     on_click=Callback::new(|_| {})
/// >
///     <TrashIcon />
/// </ActionIcon>
/// ```
#[component]
pub fn ActionIcon(
    /// Visual variant of the button
    #[prop(optional)]
    variant: Option<ActionIconVariant>,
    /// Size of the button
    #[prop(optional)]
    size: Option<ActionIconSize>,
    /// Color (theme color name)
    #[prop(optional, into)]
    color: Option<String>,
    /// Border radius (CSS value)
    #[prop(optional, into)]
    radius: Option<String>,
    /// Whether the button is disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Whether the button is in a loading state
    #[prop(optional, into)]
    loading: Signal<bool>,
    /// Click handler
    #[prop(optional)]
    on_click: Option<Callback<ev::MouseEvent>>,
    /// Accessible label for screen readers
    #[prop(optional, into)]
    aria_label: Option<String>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Additional inline styles
    #[prop(optional, into)]
    style: Option<String>,
    /// Icon content (children)
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let variant = variant.unwrap_or_default();
    let size = size.unwrap_or_default();
    let color = color.unwrap_or_else(|| "blue".to_string());

    let button_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        let is_disabled = disabled.get();
        let is_loading = loading.get();

        // Get colors from theme
        let bg_color = scheme_colors
            .get_color(&color, 6)
            .unwrap_or_else(|| "#228be6".to_string());
        let light_color = scheme_colors
            .get_color(&color, 0)
            .unwrap_or_else(|| "#e7f5ff".to_string());
        let hover_color = scheme_colors
            .get_color(&color, 7)
            .unwrap_or_else(|| "#1c7ed6".to_string());

        // Base styles
        builder
            .add("display", "inline-flex")
            .add("align-items", "center")
            .add("justify-content", "center")
            .add("border", "none")
            .add("padding", "0")
            .add(
                "cursor",
                if is_disabled || is_loading {
                    "not-allowed"
                } else {
                    "pointer"
                },
            )
            .add("transition", "all 0.15s ease")
            .add("user-select", "none")
            .add("opacity", if is_disabled { "0.6" } else { "1" })
            .add("width", size.to_size())
            .add("height", size.to_size())
            .add("min-width", size.to_size())
            .add("min-height", size.to_size());

        // Icon sizing
        builder.add("font-size", size.to_icon_size());

        // Variant-based styles
        match variant {
            ActionIconVariant::Filled => {
                builder
                    .add("background-color", bg_color.clone())
                    .add("color", scheme_colors.white.clone());
            }
            ActionIconVariant::Outline => {
                builder
                    .add("background-color", "transparent")
                    .add("color", bg_color.clone())
                    .add("border", format!("1px solid {}", bg_color));
            }
            ActionIconVariant::Light => {
                builder
                    .add("background-color", light_color)
                    .add("color", bg_color.clone());
            }
            ActionIconVariant::Subtle => {
                builder
                    .add("background-color", "transparent")
                    .add("color", bg_color.clone());
            }
            ActionIconVariant::Transparent => {
                builder
                    .add("background-color", "transparent")
                    .add("color", bg_color.clone());
            }
        }

        // Border radius
        if let Some(ref r) = radius {
            builder.add("border-radius", r);
        } else {
            builder.add("border-radius", theme_val.radius.sm);
        }

        // Hover state (CSS variable for hover, but we'll keep it simple)
        let _ = hover_color; // Acknowledge unused for now - CSS hover would need separate handling

        if let Some(s) = style.as_ref() {
            format!("{}; {}", builder.build(), s)
        } else {
            builder.build()
        }
    };

    let handle_click = move |ev: ev::MouseEvent| {
        if !disabled.get() && !loading.get() {
            if let Some(callback) = on_click {
                callback.run(ev);
            }
        }
    };

    let class_str = format!("mingot-action-icon {}", class.unwrap_or_default());

    view! {
        <button
            type="button"
            class=class_str
            style=button_styles
            disabled=move || disabled.get() || loading.get()
            on:click=handle_click
            aria-label=aria_label
        >
            {children()}
        </button>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_icon_size_values() {
        assert_eq!(ActionIconSize::Xs.to_size(), "1.125rem");
        assert_eq!(ActionIconSize::Sm.to_size(), "1.375rem");
        assert_eq!(ActionIconSize::Md.to_size(), "1.75rem");
        assert_eq!(ActionIconSize::Lg.to_size(), "2.125rem");
        assert_eq!(ActionIconSize::Xl.to_size(), "2.75rem");
    }

    #[test]
    fn test_action_icon_variant_default() {
        assert_eq!(ActionIconVariant::default(), ActionIconVariant::Filled);
    }

    #[test]
    fn test_action_icon_size_default() {
        assert_eq!(ActionIconSize::default(), ActionIconSize::Md);
    }
}
