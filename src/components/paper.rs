use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PaperPadding {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl PaperPadding {
    fn value(&self, theme: &crate::theme::Theme) -> &str {
        match self {
            PaperPadding::Xs => theme.spacing.xs,
            PaperPadding::Sm => theme.spacing.sm,
            PaperPadding::Md => theme.spacing.md,
            PaperPadding::Lg => theme.spacing.lg,
            PaperPadding::Xl => theme.spacing.xl,
        }
    }
}

#[component]
pub fn Paper(
    #[prop(optional)] padding: Option<PaperPadding>,
    #[prop(optional)] radius: Option<String>,
    #[prop(optional)] with_border: bool,
    #[prop(optional)] shadow: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let padding = padding.unwrap_or(PaperPadding::Md);

    let paper_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        builder
            .add("background-color", scheme_colors.background.clone())
            .add("padding", padding.value(&theme_val));

        if let Some(r) = radius.as_ref() {
            builder.add("border-radius", r);
        } else {
            builder.add("border-radius", theme_val.radius.sm);
        }

        if with_border {
            builder.add(
                "border",
                format!("1px solid {}", scheme_colors.border.clone()),
            );
        }

        if let Some(s) = shadow.as_ref() {
            builder.add("box-shadow", s);
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-paper {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=paper_styles>
            {children()}
        </div>
    }
}
