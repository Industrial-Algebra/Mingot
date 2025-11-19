use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GroupAlign {
    Start,
    Center,
    End,
    Baseline,
}

impl GroupAlign {
    fn as_str(&self) -> &'static str {
        match self {
            GroupAlign::Start => "flex-start",
            GroupAlign::Center => "center",
            GroupAlign::End => "flex-end",
            GroupAlign::Baseline => "baseline",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GroupJustify {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
}

impl GroupJustify {
    fn as_str(&self) -> &'static str {
        match self {
            GroupJustify::Start => "flex-start",
            GroupJustify::Center => "center",
            GroupJustify::End => "flex-end",
            GroupJustify::SpaceBetween => "space-between",
            GroupJustify::SpaceAround => "space-around",
        }
    }
}

#[component]
pub fn Group(
    #[prop(optional, into)] spacing: Option<String>,
    #[prop(optional)] align: Option<GroupAlign>,
    #[prop(optional)] justify: Option<GroupJustify>,
    #[prop(optional)] wrap: bool,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let align = align.unwrap_or(GroupAlign::Center);
    let justify = justify.unwrap_or(GroupJustify::Start);

    let group_styles = move || {
        let theme_val = theme.get();
        let mut builder = StyleBuilder::new();

        builder
            .add("display", "flex")
            .add("flex-direction", "row")
            .add("align-items", align.as_str())
            .add("justify-content", justify.as_str());

        // Spacing
        let gap = spacing.as_deref().unwrap_or(theme_val.spacing.md);
        builder.add("gap", gap);

        // Wrap
        if wrap {
            builder.add("flex-wrap", "wrap");
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-group {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=group_styles>
            {children()}
        </div>
    }
}
