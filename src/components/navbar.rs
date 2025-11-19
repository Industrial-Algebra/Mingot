use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NavbarOrientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NavbarVariant {
    Default,
    Subtle,
    Pills,
}

#[component]
pub fn Navbar(
    #[prop(optional)] orientation: Option<NavbarOrientation>,
    #[prop(optional, into)] spacing: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let orientation = orientation.unwrap_or(NavbarOrientation::Horizontal);

    let navbar_styles = move || {
        let theme_val = theme.get();
        let mut builder = StyleBuilder::new();

        builder.add("display", "flex");

        match orientation {
            NavbarOrientation::Horizontal => {
                builder.add("flex-direction", "row");
                builder.add("align-items", "center");
            }
            NavbarOrientation::Vertical => {
                builder.add("flex-direction", "column");
                builder.add("align-items", "stretch");
            }
        }

        if let Some(s) = spacing.as_ref() {
            builder.add("gap", s);
        } else {
            builder.add("gap", theme_val.spacing.sm);
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-navbar {}", class.unwrap_or_default());

    view! {
        <nav class=class_str style=navbar_styles>
            {children()}
        </nav>
    }
}

#[component]
pub fn NavbarLink(
    #[prop(into)] href: String,
    #[prop(optional)] active: bool,
    #[prop(optional)] variant: Option<NavbarVariant>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] on_click: Option<Callback<leptos::ev::MouseEvent>>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let variant = variant.unwrap_or(NavbarVariant::Default);

    let link_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        builder
            .add("display", "inline-flex")
            .add("align-items", "center")
            .add("text-decoration", "none")
            .add("font-size", theme_val.typography.font_sizes.sm)
            .add(
                "font-weight",
                theme_val.typography.font_weights.medium.to_string(),
            )
            .add("cursor", if disabled { "not-allowed" } else { "pointer" })
            .add("transition", "all 0.15s ease")
            .add("user-select", "none")
            .add("white-space", "nowrap");

        if disabled {
            builder.add("opacity", "0.4");
        }

        match variant {
            NavbarVariant::Default => {
                builder.add(
                    "padding",
                    format!("{} {}", theme_val.spacing.xs, theme_val.spacing.sm),
                );

                if active {
                    let active_color = scheme_colors
                        .get_color("blue", 6)
                        .unwrap_or_else(|| "#228be6".to_string());
                    builder.add("color", active_color.clone());
                    builder.add("border-bottom", format!("2px solid {}", active_color));
                } else {
                    builder.add("color", scheme_colors.text.clone());
                    builder.add("border-bottom", "2px solid transparent");
                }
            }
            NavbarVariant::Subtle => {
                builder.add(
                    "padding",
                    format!("{} {}", theme_val.spacing.xs, theme_val.spacing.sm),
                );
                builder.add("border-radius", theme_val.radius.sm);

                if active {
                    let light_color = scheme_colors
                        .get_color("blue", 0)
                        .unwrap_or_else(|| "#e7f5ff".to_string());
                    let active_color = scheme_colors
                        .get_color("blue", 6)
                        .unwrap_or_else(|| "#228be6".to_string());
                    builder.add("background-color", light_color);
                    builder.add("color", active_color);
                } else {
                    builder.add("background-color", "transparent");
                    builder.add("color", scheme_colors.text.clone());
                }
            }
            NavbarVariant::Pills => {
                builder.add(
                    "padding",
                    format!("{} {}", theme_val.spacing.xs, theme_val.spacing.md),
                );
                builder.add("border-radius", "9999px");

                if active {
                    let active_color = scheme_colors
                        .get_color("blue", 6)
                        .unwrap_or_else(|| "#228be6".to_string());
                    builder.add("background-color", active_color);
                    builder.add("color", scheme_colors.white.clone());
                } else {
                    builder.add("background-color", "transparent");
                    builder.add("color", scheme_colors.text.clone());
                }
            }
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let handle_click = move |ev: leptos::ev::MouseEvent| {
        if !disabled {
            if let Some(callback) = on_click {
                callback.run(ev);
            }
        }
    };

    let class_str = format!("mingot-navbar-link {}", class.unwrap_or_default());

    view! {
        <a
            href=href
            class=class_str
            style=link_styles
            on:click=handle_click
        >
            {children()}
        </a>
    }
}

#[component]
pub fn NavbarBrand(
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();

    let brand_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        builder
            .add("display", "inline-flex")
            .add("align-items", "center")
            .add("text-decoration", "none")
            .add("font-size", theme_val.typography.font_sizes.lg)
            .add(
                "font-weight",
                theme_val.typography.font_weights.bold.to_string(),
            )
            .add("color", scheme_colors.text.clone())
            .add("margin-right", theme_val.spacing.lg);

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-navbar-brand {}", class.unwrap_or_default());

    if let Some(h) = href {
        view! {
            <a href=h class=class_str style=brand_styles>
                {children()}
            </a>
        }
        .into_any()
    } else {
        view! {
            <div class=class_str style=brand_styles>
                {children()}
            </div>
        }
        .into_any()
    }
}
