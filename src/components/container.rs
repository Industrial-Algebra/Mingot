use crate::utils::StyleBuilder;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ContainerSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl ContainerSize {
    fn max_width(&self) -> &'static str {
        match self {
            ContainerSize::Xs => "540px",
            ContainerSize::Sm => "720px",
            ContainerSize::Md => "960px",
            ContainerSize::Lg => "1140px",
            ContainerSize::Xl => "1320px",
        }
    }
}

#[component]
pub fn Container(
    #[prop(optional)] size: Option<ContainerSize>,
    #[prop(optional)] fluid: bool,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let size = size.unwrap_or(ContainerSize::Md);

    let container_styles = move || {
        let mut builder = StyleBuilder::new();

        builder
            .add("width", "100%")
            .add("margin-left", "auto")
            .add("margin-right", "auto")
            .add("padding-left", "1rem")
            .add("padding-right", "1rem");

        if !fluid {
            builder.add("max-width", size.max_width());
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-container {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=container_styles>
            {children()}
        </div>
    }
}
