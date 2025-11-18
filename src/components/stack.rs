use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StackAlign {
    Start,
    Center,
    End,
    Stretch,
}

impl StackAlign {
    fn as_str(&self) -> &'static str {
        match self {
            StackAlign::Start => "flex-start",
            StackAlign::Center => "center",
            StackAlign::End => "flex-end",
            StackAlign::Stretch => "stretch",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StackJustify {
    Start,
    Center,
    End,
    SpaceBetween,
    SpaceAround,
}

impl StackJustify {
    fn as_str(&self) -> &'static str {
        match self {
            StackJustify::Start => "flex-start",
            StackJustify::Center => "center",
            StackJustify::End => "flex-end",
            StackJustify::SpaceBetween => "space-between",
            StackJustify::SpaceAround => "space-around",
        }
    }
}

#[component]
pub fn Stack(
    #[prop(optional)] spacing: Option<String>,
    #[prop(optional)] align: Option<StackAlign>,
    #[prop(optional)] justify: Option<StackJustify>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let align = align.unwrap_or(StackAlign::Stretch);
    let justify = justify.unwrap_or(StackJustify::Start);

    let stack_styles = move || {
        let theme_val = theme.get();
        let mut builder = StyleBuilder::new();

        builder
            .add("display", "flex")
            .add("flex-direction", "column")
            .add("align-items", align.as_str())
            .add("justify-content", justify.as_str());

        // Spacing
        let gap = spacing.as_deref().unwrap_or(theme_val.spacing.md);
        builder.add("gap", gap);

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-stack {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=stack_styles>
            {children()}
        </div>
    }
}
