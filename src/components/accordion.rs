use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AccordionVariant {
    Default,
    Contained,
    Separated,
}

#[component]
pub fn Accordion(
    #[prop(optional)] variant: Option<AccordionVariant>,
    #[prop(optional)] multiple: bool,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let variant = variant.unwrap_or(AccordionVariant::Default);

    // Provide context for accordion items
    provide_context::<Signal<AccordionVariant>>(Signal::derive(move || variant));
    provide_context::<Signal<bool>>(Signal::derive(move || multiple));

    let accordion_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        builder
            .add("display", "flex")
            .add("flex-direction", "column");

        match variant {
            AccordionVariant::Default => {
                builder.add(
                    "border-top",
                    format!("1px solid {}", scheme_colors.border.clone()),
                );
            }
            AccordionVariant::Contained => {
                builder
                    .add(
                        "border",
                        format!("1px solid {}", scheme_colors.border.clone()),
                    )
                    .add("border-radius", theme_val.radius.sm);
            }
            AccordionVariant::Separated => {
                builder.add("gap", theme_val.spacing.sm);
            }
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-accordion {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=accordion_styles>
            {children()}
        </div>
    }
}

#[component]
pub fn AccordionItem(
    #[prop(into)] _value: String,
    #[prop(into)] label: String,
    #[prop(optional)] opened: Option<RwSignal<bool>>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let variant = use_context::<Signal<AccordionVariant>>()
        .unwrap_or(Signal::derive(move || AccordionVariant::Default));

    let is_opened = opened.unwrap_or_else(|| RwSignal::new(false));

    let item_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        match variant.get() {
            AccordionVariant::Default => {
                builder.add(
                    "border-bottom",
                    format!("1px solid {}", scheme_colors.border.clone()),
                );
            }
            AccordionVariant::Contained => {}
            AccordionVariant::Separated => {
                builder
                    .add(
                        "border",
                        format!("1px solid {}", scheme_colors.border.clone()),
                    )
                    .add("border-radius", theme_val.radius.sm);
            }
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let control_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "display: flex; \
             align-items: center; \
             justify-content: space-between; \
             width: 100%; \
             padding: {} {}; \
             background: none; \
             border: none; \
             cursor: pointer; \
             text-align: left; \
             font-size: {}; \
             font-weight: {}; \
             color: {}; \
             transition: background-color 0.15s ease;",
            theme_val.spacing.md,
            theme_val.spacing.lg,
            theme_val.typography.font_sizes.md,
            theme_val.typography.font_weights.medium,
            scheme_colors.text
        )
    };

    let chevron_styles = move || {
        format!(
            "transition: transform 0.2s ease; \
             transform: rotate({});",
            if is_opened.get() { "180deg" } else { "0deg" }
        )
    };

    let panel_styles = move || {
        format!(
            "overflow: hidden; \
             max-height: {}; \
             transition: max-height 0.2s ease;",
            if is_opened.get() { "1000px" } else { "0" }
        )
    };

    let content_styles = move || {
        let theme_val = theme.get();
        format!(
            "padding: 0 {} {} {};",
            theme_val.spacing.lg, theme_val.spacing.md, theme_val.spacing.lg
        )
    };

    let handle_toggle = move |_| {
        is_opened.update(|opened| *opened = !*opened);
    };

    let class_str = format!("mingot-accordion-item {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=item_styles>
            <button
                class="mingot-accordion-control"
                style=control_styles
                on:click=handle_toggle
            >
                <span>{label}</span>
                <span style=chevron_styles>"▼"</span>
            </button>
            <div class="mingot-accordion-panel" style=panel_styles>
                <div class="mingot-accordion-content" style=content_styles>
                    {children()}
                </div>
            </div>
        </div>
    }
}
