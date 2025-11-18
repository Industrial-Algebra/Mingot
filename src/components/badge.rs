use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BadgeVariant {
    Filled,
    Light,
    Outline,
    Dot,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BadgeSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl BadgeSize {
    fn padding(&self) -> &'static str {
        match self {
            BadgeSize::Xs => "0.125rem 0.375rem",
            BadgeSize::Sm => "0.25rem 0.5rem",
            BadgeSize::Md => "0.375rem 0.625rem",
            BadgeSize::Lg => "0.5rem 0.875rem",
            BadgeSize::Xl => "0.625rem 1rem",
        }
    }

    fn font_size(&self) -> &'static str {
        match self {
            BadgeSize::Xs => "0.625rem",
            BadgeSize::Sm => "0.6875rem",
            BadgeSize::Md => "0.75rem",
            BadgeSize::Lg => "0.875rem",
            BadgeSize::Xl => "1rem",
        }
    }

    fn dot_size(&self) -> &'static str {
        match self {
            BadgeSize::Xs => "4px",
            BadgeSize::Sm => "5px",
            BadgeSize::Md => "6px",
            BadgeSize::Lg => "7px",
            BadgeSize::Xl => "8px",
        }
    }
}

#[component]
pub fn Badge(
    #[prop(optional)] variant: Option<BadgeVariant>,
    #[prop(optional)] size: Option<BadgeSize>,
    #[prop(optional)] color: Option<String>,
    #[prop(optional)] radius: Option<String>,
    #[prop(optional)] full_width: bool,
    #[prop(optional)] left_section: Option<Children>,
    #[prop(optional)] right_section: Option<Children>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let variant = variant.unwrap_or(BadgeVariant::Filled);
    let size = size.unwrap_or(BadgeSize::Md);
    let color = color.unwrap_or_else(|| "blue".to_string());
    let color_clone = color.clone();

    let badge_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        let bg_color = scheme_colors
            .get_color(&color_clone, 6)
            .unwrap_or_else(|| "#228be6".to_string());
        let light_color = scheme_colors
            .get_color(&color_clone, 0)
            .unwrap_or_else(|| "#e7f5ff".to_string());

        builder
            .add("display", "inline-flex")
            .add("align-items", "center")
            .add("justify-content", "center")
            .add("gap", "0.25rem")
            .add("padding", size.padding())
            .add("font-size", size.font_size())
            .add(
                "font-weight",
                theme_val.typography.font_weights.semibold.to_string(),
            )
            .add("text-transform", "uppercase")
            .add("line-height", "1")
            .add("white-space", "nowrap")
            .add("user-select", "none");

        match variant {
            BadgeVariant::Filled => {
                builder
                    .add("background-color", bg_color)
                    .add("color", scheme_colors.white.clone());
            }
            BadgeVariant::Light => {
                builder
                    .add("background-color", light_color)
                    .add("color", bg_color);
            }
            BadgeVariant::Outline => {
                builder
                    .add("background-color", "transparent")
                    .add("color", bg_color.clone())
                    .add("border", format!("1px solid {}", bg_color));
            }
            BadgeVariant::Dot => {
                builder
                    .add("background-color", "transparent")
                    .add("color", scheme_colors.text.clone())
                    .add("text-transform", "none");
            }
        }

        if let Some(r) = radius.as_ref() {
            builder.add("border-radius", r);
        } else {
            builder.add("border-radius", "9999px");
        }

        if full_width {
            builder.add("width", "100%");
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let dot_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let bg_color = scheme_colors
            .get_color(&color, 6)
            .unwrap_or_else(|| "#228be6".to_string());

        format!(
            "width: {}; \
             height: {}; \
             border-radius: 50%; \
             background-color: {};",
            size.dot_size(),
            size.dot_size(),
            bg_color
        )
    };

    let class_str = format!("mingot-badge {}", class.unwrap_or_default());

    view! {
        <span class=class_str style=badge_styles>
            {if variant == BadgeVariant::Dot {
                view! { <span style=dot_styles></span> }.into_any()
            } else {
                ().into_any()
            }}
            {if let Some(left) = left_section {
                view! { <span>{left()}</span> }.into_any()
            } else {
                ().into_any()
            }}
            <span>{children()}</span>
            {if let Some(right) = right_section {
                view! { <span>{right()}</span> }.into_any()
            } else {
                ().into_any()
            }}
        </span>
    }
}
