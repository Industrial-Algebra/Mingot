use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CardPadding {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl CardPadding {
    fn value(&self, theme: &crate::theme::Theme) -> &str {
        match self {
            CardPadding::Xs => theme.spacing.xs,
            CardPadding::Sm => theme.spacing.sm,
            CardPadding::Md => theme.spacing.md,
            CardPadding::Lg => theme.spacing.lg,
            CardPadding::Xl => theme.spacing.xl,
        }
    }
}

#[component]
pub fn Card(
    #[prop(optional)] padding: Option<CardPadding>,
    #[prop(optional, into)] radius: Option<String>,
    #[prop(optional)] with_border: bool,
    #[prop(optional, into)] shadow: Option<String>,
    #[prop(optional, into)] as_: Option<String>,
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let padding = padding.unwrap_or(CardPadding::Md);
    let is_link = as_.as_ref().map(|s| s == "a").unwrap_or(false);

    let card_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        builder
            .add("background-color", scheme_colors.background.clone())
            .add("padding", padding.value(&theme_val))
            .add("display", "flex")
            .add("flex-direction", "column")
            .add("transition", "box-shadow 0.15s ease");

        // Add link-specific styles
        if is_link {
            builder
                .add("text-decoration", "none")
                .add("color", "inherit")
                .add("cursor", "pointer");
        }

        if let Some(r) = radius.as_ref() {
            builder.add("border-radius", r);
        } else {
            builder.add("border-radius", theme_val.radius.md);
        }

        if with_border {
            builder.add(
                "border",
                format!("1px solid {}", scheme_colors.border.clone()),
            );
        }

        if let Some(s) = shadow.as_ref() {
            builder.add("box-shadow", s);
        } else {
            builder.add("box-shadow", theme_val.shadows.sm);
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-card {}", class.unwrap_or_default());

    if is_link {
        view! {
            <a href=href.unwrap_or_else(|| "#".to_string()) class=class_str style=card_styles>
                {children()}
            </a>
        }
        .into_any()
    } else {
        view! {
            <div class=class_str style=card_styles>
                {children()}
            </div>
        }
        .into_any()
    }
}

#[component]
pub fn CardSection(
    #[prop(optional)] with_border: bool,
    #[prop(optional)] inherit_padding: bool,
    #[prop(optional, into)] padding: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();

    let section_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        if !inherit_padding {
            builder.add(
                "margin",
                format!("-{} -{}", theme_val.spacing.md, theme_val.spacing.md),
            );
        }

        if let Some(p) = padding.as_ref() {
            builder.add("padding", p);
        } else if !inherit_padding {
            builder.add("padding", theme_val.spacing.md);
        }

        if with_border {
            builder.add(
                "border-top",
                format!("1px solid {}", scheme_colors.border.clone()),
            );
            builder.add(
                "border-bottom",
                format!("1px solid {}", scheme_colors.border.clone()),
            );
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-card-section {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=section_styles>
            {children()}
        </div>
    }
}
