use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextWeight {
    Normal,
    Medium,
    Semibold,
    Bold,
}

#[component]
pub fn Text(
    #[prop(optional)] size: Option<TextSize>,
    #[prop(optional)] weight: Option<TextWeight>,
    #[prop(optional)] color: Option<String>,
    #[prop(optional)] italic: bool,
    #[prop(optional)] underline: bool,
    #[prop(optional)] align: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let size = size.unwrap_or(TextSize::Md);
    let weight = weight.unwrap_or(TextWeight::Normal);

    let text_styles = move || {
        let theme_val = theme.get();
        let mut builder = StyleBuilder::new();

        // Font size
        let font_size = match size {
            TextSize::Xs => theme_val.typography.font_sizes.xs,
            TextSize::Sm => theme_val.typography.font_sizes.sm,
            TextSize::Md => theme_val.typography.font_sizes.md,
            TextSize::Lg => theme_val.typography.font_sizes.lg,
            TextSize::Xl => theme_val.typography.font_sizes.xl,
        };
        builder.add("font-size", font_size);

        // Font weight
        let font_weight = match weight {
            TextWeight::Normal => theme_val.typography.font_weights.normal,
            TextWeight::Medium => theme_val.typography.font_weights.medium,
            TextWeight::Semibold => theme_val.typography.font_weights.semibold,
            TextWeight::Bold => theme_val.typography.font_weights.bold,
        };
        builder.add("font-weight", font_weight.to_string());

        // Color
        if let Some(c) = color.as_ref() {
            // Try to get from theme colors first
            if let Some(theme_color) = theme_val.colors.get_color(c, 6) {
                builder.add("color", theme_color);
            } else {
                builder.add("color", c);
            }
        }

        // Font style
        builder.add_if(italic, "font-style", "italic");

        // Text decoration
        builder.add_if(underline, "text-decoration", "underline");

        // Text align
        if let Some(a) = align.as_ref() {
            builder.add("text-align", a);
        }

        builder.add("font-family", theme_val.typography.font_family);
        builder.add("line-height", theme_val.typography.line_heights.md);

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-text {}", class.unwrap_or_default());

    view! {
        <span class=class_str style=text_styles>
            {children()}
        </span>
    }
}
