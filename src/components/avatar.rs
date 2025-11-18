use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AvatarSize {
    Xs, // 16px
    Sm, // 26px
    Md, // 38px
    Lg, // 56px
    Xl, // 84px
}

impl AvatarSize {
    fn px(&self) -> &'static str {
        match self {
            AvatarSize::Xs => "16px",
            AvatarSize::Sm => "26px",
            AvatarSize::Md => "38px",
            AvatarSize::Lg => "56px",
            AvatarSize::Xl => "84px",
        }
    }

    fn font_size(&self) -> &'static str {
        match self {
            AvatarSize::Xs => "0.5rem",
            AvatarSize::Sm => "0.75rem",
            AvatarSize::Md => "1rem",
            AvatarSize::Lg => "1.5rem",
            AvatarSize::Xl => "2rem",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AvatarRadius {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Full,
}

impl AvatarRadius {
    fn value(&self, theme_radius: &str) -> String {
        match self {
            AvatarRadius::Xs => theme_radius.to_string(),
            AvatarRadius::Sm => theme_radius.to_string(),
            AvatarRadius::Md => theme_radius.to_string(),
            AvatarRadius::Lg => theme_radius.to_string(),
            AvatarRadius::Xl => theme_radius.to_string(),
            AvatarRadius::Full => "9999px".to_string(),
        }
    }
}

#[component]
pub fn Avatar(
    #[prop(optional)] src: Option<String>,
    #[prop(optional)] alt: Option<String>,
    #[prop(optional)] size: Option<AvatarSize>,
    #[prop(optional)] radius: Option<AvatarRadius>,
    #[prop(optional)] color: Option<String>,
    #[prop(optional)] initials: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let size = size.unwrap_or(AvatarSize::Md);
    let radius = radius.unwrap_or(AvatarRadius::Full);
    let color = color.unwrap_or_else(|| "blue".to_string());
    let src_clone = src.clone();

    let avatar_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        builder
            .add("width", size.px())
            .add("height", size.px())
            .add("min-width", size.px())
            .add("min-height", size.px())
            .add("border-radius", radius.value(theme_val.radius.md))
            .add("display", "inline-flex")
            .add("align-items", "center")
            .add("justify-content", "center")
            .add("overflow", "hidden")
            .add("user-select", "none")
            .add("position", "relative");

        if src_clone.is_none() {
            // Show background color for initials
            let bg_color = scheme_colors
                .get_color(&color, 6)
                .unwrap_or_else(|| "#228be6".to_string());
            builder
                .add("background-color", bg_color)
                .add("color", scheme_colors.white.clone())
                .add("font-size", size.font_size())
                .add(
                    "font-weight",
                    theme_val.typography.font_weights.semibold.to_string(),
                );
        } else {
            let border_color = scheme_colors.border.clone();
            builder.add("border", format!("1px solid {}", border_color));
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let image_styles = move || "width: 100%; height: 100%; object-fit: cover;".to_string();

    let class_str = format!("mingot-avatar {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=avatar_styles>
            {if let Some(image_src) = src {
                view! {
                    <img
                        src=image_src
                        alt=alt.unwrap_or_default()
                        style=image_styles
                    />
                }.into_any()
            } else if let Some(text) = initials {
                view! {
                    <span>{text}</span>
                }.into_any()
            } else {
                view! {
                    <span>"?"</span>
                }.into_any()
            }}
        </div>
    }
}

#[component]
pub fn AvatarGroup(
    #[prop(optional)] spacing: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();

    let group_styles = move || {
        let theme_val = theme.get();
        let mut builder = StyleBuilder::new();

        builder
            .add("display", "inline-flex")
            .add("flex-direction", "row-reverse");

        if let Some(s) = spacing.as_ref() {
            builder.add("gap", s);
        } else {
            // Negative margin for overlap
            builder.add("margin-left", format!("-{}", theme_val.spacing.xs));
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-avatar-group {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=group_styles>
            {children()}
        </div>
    }
}
