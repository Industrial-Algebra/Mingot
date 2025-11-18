use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FooterHeight {
    Xs, // 48px
    Sm, // 60px
    Md, // 72px
    Lg, // 84px
    Xl, // 96px
}

impl FooterHeight {
    fn px(&self) -> &'static str {
        match self {
            FooterHeight::Xs => "48px",
            FooterHeight::Sm => "60px",
            FooterHeight::Md => "72px",
            FooterHeight::Lg => "84px",
            FooterHeight::Xl => "96px",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FooterPosition {
    Static,
    Fixed,
}

impl FooterPosition {
    fn as_str(&self) -> &'static str {
        match self {
            FooterPosition::Static => "static",
            FooterPosition::Fixed => "fixed",
        }
    }
}

#[component]
pub fn Footer(
    #[prop(optional)] height: Option<FooterHeight>,
    #[prop(optional)] position: Option<FooterPosition>,
    #[prop(optional)] with_border: bool,
    #[prop(optional)] padding: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let height = height.unwrap_or(FooterHeight::Md);
    let position = position.unwrap_or(FooterPosition::Static);

    let footer_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        builder
            .add("height", height.px())
            .add("position", position.as_str())
            .add("bottom", "0")
            .add("left", "0")
            .add("right", "0")
            .add("display", "flex")
            .add("align-items", "center")
            .add("background-color", scheme_colors.background.clone())
            .add("z-index", "100")
            .add("transition", "background-color 0.15s ease");

        if let Some(p) = padding.as_ref() {
            builder.add("padding", p);
        } else {
            builder.add("padding", format!("0 {}", theme_val.spacing.md));
        }

        if with_border {
            builder.add(
                "border-top",
                format!("1px solid {}", scheme_colors.border.clone()),
            );
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-footer {}", class.unwrap_or_default());

    view! {
        <footer class=class_str style=footer_styles>
            {children()}
        </footer>
    }
}
