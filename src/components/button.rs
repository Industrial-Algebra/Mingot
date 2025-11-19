use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonVariant {
    Filled,
    Outline,
    Light,
    Subtle,
    Default,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ButtonSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

#[component]
pub fn Button(
    #[prop(optional)] variant: Option<ButtonVariant>,
    #[prop(optional)] size: Option<ButtonSize>,
    #[prop(optional, into)] color: Option<String>,
    #[prop(optional, into)] radius: Option<String>,
    #[prop(optional, into)] full_width: Signal<bool>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] loading: Signal<bool>,
    #[prop(optional, into)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, into)] button_type: Option<String>,
    #[prop(optional, into)] as_: Option<String>,
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let variant = variant.unwrap_or(ButtonVariant::Filled);
    let size = size.unwrap_or(ButtonSize::Md);
    let color = color.unwrap_or_else(|| "blue".to_string());
    let button_type = button_type.unwrap_or_else(|| "button".to_string());
    let is_link = as_.as_ref().map(|s| s == "a").unwrap_or(false);

    let button_styles = move || {
        let theme_val = theme.get();
        let mut builder = StyleBuilder::new();

        // Get the active color scheme colors
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        // Get color from theme
        let bg_color = scheme_colors
            .get_color(&color, 6)
            .unwrap_or_else(|| "#228be6".to_string());
        let light_color = scheme_colors
            .get_color(&color, 0)
            .unwrap_or_else(|| "#e7f5ff".to_string());

        let is_disabled = disabled.get();
        let is_loading = loading.get();
        let is_full_width = full_width.get();

        // Base styles
        builder
            .add("display", "inline-flex")
            .add("align-items", "center")
            .add("justify-content", "center")
            .add("border", "none")
            .add(
                "cursor",
                if is_disabled || is_loading {
                    "not-allowed"
                } else {
                    "pointer"
                },
            )
            .add("font-family", theme_val.typography.font_family)
            .add(
                "font-weight",
                theme_val.typography.font_weights.semibold.to_string(),
            )
            .add("transition", "all 0.15s ease")
            .add("user-select", "none")
            .add("text-decoration", "none")
            .add("opacity", if is_disabled { "0.6" } else { "1" });

        // Size-based styles
        match size {
            ButtonSize::Xs => {
                builder
                    .add("height", "1.875rem")
                    .add("padding", "0 0.875rem")
                    .add("font-size", theme_val.typography.font_sizes.xs);
            }
            ButtonSize::Sm => {
                builder
                    .add("height", "2.25rem")
                    .add("padding", "0 1.125rem")
                    .add("font-size", theme_val.typography.font_sizes.sm);
            }
            ButtonSize::Md => {
                builder
                    .add("height", "2.625rem")
                    .add("padding", "0 1.375rem")
                    .add("font-size", theme_val.typography.font_sizes.sm);
            }
            ButtonSize::Lg => {
                builder
                    .add("height", "3.125rem")
                    .add("padding", "0 1.625rem")
                    .add("font-size", theme_val.typography.font_sizes.md);
            }
            ButtonSize::Xl => {
                builder
                    .add("height", "3.75rem")
                    .add("padding", "0 2rem")
                    .add("font-size", theme_val.typography.font_sizes.lg);
            }
        }

        // Variant-based styles
        match variant {
            ButtonVariant::Filled => {
                builder
                    .add("background-color", bg_color.clone())
                    .add("color", scheme_colors.white.clone());
            }
            ButtonVariant::Outline => {
                builder
                    .add("background-color", "transparent")
                    .add("color", bg_color.clone())
                    .add("border", format!("1px solid {}", bg_color));
            }
            ButtonVariant::Light => {
                builder
                    .add("background-color", light_color)
                    .add("color", bg_color.clone());
            }
            ButtonVariant::Subtle => {
                builder
                    .add("background-color", "transparent")
                    .add("color", bg_color.clone());
            }
            ButtonVariant::Default => {
                let border_color = scheme_colors
                    .get_color("gray", 4)
                    .unwrap_or_else(|| scheme_colors.border.clone());
                builder
                    .add("background-color", scheme_colors.background.clone())
                    .add("color", scheme_colors.text.clone())
                    .add("border", format!("1px solid {}", border_color));
            }
        }

        // Border radius
        if let Some(r) = radius.as_ref() {
            builder.add("border-radius", r);
        } else {
            builder.add("border-radius", theme_val.radius.sm);
        }

        // Full width
        builder.add_if(is_full_width, "width", "100%");

        // Custom styles
        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let handle_click = move |ev: ev::MouseEvent| {
        if !disabled.get() && !loading.get() {
            if let Some(callback) = on_click {
                callback.run(ev);
            }
        }
    };

    let class_str = format!("mingot-button {}", class.unwrap_or_default());

    if is_link {
        view! {
            <a
                href=href.unwrap_or_else(|| "#".to_string())
                class=class_str
                style=button_styles
                on:click=handle_click
            >
                {children()}
            </a>
        }
        .into_any()
    } else {
        view! {
            <button
                type=button_type
                class=class_str
                style=button_styles
                disabled=move || disabled.get() || loading.get()
                on:click=handle_click
            >
                {children()}
            </button>
        }
        .into_any()
    }
}
