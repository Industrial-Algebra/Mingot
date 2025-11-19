use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HeaderHeight {
    Xs, // 48px
    Sm, // 60px
    Md, // 72px
    Lg, // 84px
    Xl, // 96px
}

impl HeaderHeight {
    fn px(&self) -> &'static str {
        match self {
            HeaderHeight::Xs => "48px",
            HeaderHeight::Sm => "60px",
            HeaderHeight::Md => "72px",
            HeaderHeight::Lg => "84px",
            HeaderHeight::Xl => "96px",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum HeaderPosition {
    Static,
    Fixed,
    Sticky,
}

impl HeaderPosition {
    fn as_str(&self) -> &'static str {
        match self {
            HeaderPosition::Static => "static",
            HeaderPosition::Fixed => "fixed",
            HeaderPosition::Sticky => "sticky",
        }
    }
}

#[component]
pub fn Header(
    #[prop(optional)] height: Option<HeaderHeight>,
    #[prop(optional)] position: Option<HeaderPosition>,
    #[prop(optional)] with_border: bool,
    #[prop(optional, into)] padding: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let height = height.unwrap_or(HeaderHeight::Md);
    let position = position.unwrap_or(HeaderPosition::Static);

    let header_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        builder
            .add("height", height.px())
            .add("position", position.as_str())
            .add("top", "0")
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
                "border-bottom",
                format!("1px solid {}", scheme_colors.border.clone()),
            );
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-header {}", class.unwrap_or_default());

    view! {
        <header class=class_str style=header_styles>
            {children()}
        </header>
    }
}
