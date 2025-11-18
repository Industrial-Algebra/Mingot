use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputVariant {
    Default,
    Filled,
    Unstyled,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum InputSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

#[component]
pub fn Input(
    #[prop(optional)] variant: Option<InputVariant>,
    #[prop(optional)] size: Option<InputSize>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] error: Option<String>,
    #[prop(optional)] required: bool,
    #[prop(optional)] input_type: Option<String>,
    #[prop(optional)] on_input: Option<Callback<String>>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] description: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let variant = variant.unwrap_or(InputVariant::Default);
    let size = size.unwrap_or(InputSize::Md);
    let input_type = input_type.unwrap_or_else(|| "text".to_string());

    let input_value = value.unwrap_or_else(|| RwSignal::new(String::new()));

    let error_clone = error.clone();
    let input_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        // Base styles
        builder
            .add("width", "100%")
            .add("font-family", theme_val.typography.font_family)
            .add("border-radius", theme_val.radius.sm)
            .add("transition", "all 0.15s ease")
            .add("outline", "none")
            .add("box-sizing", "border-box");

        // Size-based styles
        match size {
            InputSize::Xs => {
                builder
                    .add("height", "1.875rem")
                    .add("padding", "0 0.625rem")
                    .add("font-size", theme_val.typography.font_sizes.xs);
            }
            InputSize::Sm => {
                builder
                    .add("height", "2.25rem")
                    .add("padding", "0 0.75rem")
                    .add("font-size", theme_val.typography.font_sizes.sm);
            }
            InputSize::Md => {
                builder
                    .add("height", "2.625rem")
                    .add("padding", "0 0.875rem")
                    .add("font-size", theme_val.typography.font_sizes.sm);
            }
            InputSize::Lg => {
                builder
                    .add("height", "3.125rem")
                    .add("padding", "0 1rem")
                    .add("font-size", theme_val.typography.font_sizes.md);
            }
            InputSize::Xl => {
                builder
                    .add("height", "3.75rem")
                    .add("padding", "0 1.125rem")
                    .add("font-size", theme_val.typography.font_sizes.lg);
            }
        }

        // Variant-based styles
        match variant {
            InputVariant::Default => {
                let border_color = if error_clone.is_some() {
                    scheme_colors
                        .get_color("red", 6)
                        .unwrap_or_else(|| "#fa5252".to_string())
                } else {
                    scheme_colors.border.clone()
                };

                builder
                    .add("background-color", scheme_colors.background.clone())
                    .add("color", scheme_colors.text.clone())
                    .add("border", format!("1px solid {}", border_color));
            }
            InputVariant::Filled => {
                let bg_color = scheme_colors
                    .get_color("gray", 1)
                    .unwrap_or_else(|| "#f1f3f5".to_string());

                builder
                    .add("background-color", bg_color)
                    .add("color", scheme_colors.text.clone())
                    .add("border", "1px solid transparent");
            }
            InputVariant::Unstyled => {
                builder
                    .add("background-color", "transparent")
                    .add("color", scheme_colors.text.clone())
                    .add("border", "none")
                    .add("padding", "0");
            }
        }

        // Disabled state
        if disabled {
            builder.add("opacity", "0.6").add("cursor", "not-allowed");
        } else {
            builder.add("cursor", "text");
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let handle_input = move |ev: ev::Event| {
        let value = event_target_value(&ev);
        input_value.set(value.clone());
        if let Some(callback) = on_input {
            callback.run(value);
        }
    };

    let handle_change = move |ev: ev::Event| {
        let value = event_target_value(&ev);
        if let Some(callback) = on_change {
            callback.run(value);
        }
    };

    let label_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "display: block; margin-bottom: 0.25rem; font-size: {}; font-weight: {}; color: {};",
            theme_val.typography.font_sizes.sm,
            theme_val.typography.font_weights.medium,
            scheme_colors.text
        )
    };

    let description_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "margin-top: 0.25rem; font-size: {}; color: {};",
            theme_val.typography.font_sizes.xs,
            scheme_colors
                .get_color("gray", 6)
                .unwrap_or_else(|| "#868e96".to_string())
        )
    };

    let error_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "margin-top: 0.25rem; font-size: {}; color: {};",
            theme_val.typography.font_sizes.xs,
            scheme_colors
                .get_color("red", 6)
                .unwrap_or_else(|| "#fa5252".to_string())
        )
    };

    let class_str = format!("mingot-input {}", class.unwrap_or_default());

    view! {
        <div class="mingot-input-wrapper" style="width: 100%;">
            {label.map(|l| view! {
                <label style=label_styles>
                    {l}
                    {if required { " *" } else { "" }}
                </label>
            })}

            <input
                type=input_type
                class=class_str
                style=input_styles
                placeholder=placeholder.unwrap_or_default()
                disabled=disabled
                required=required
                prop:value=move || input_value.get()
                on:input=handle_input
                on:change=handle_change
            />

            {description.map(|d| view! {
                <div style=description_styles>{d}</div>
            })}

            {error.map(|e| view! {
                <div style=error_styles>{e}</div>
            })}
        </div>
    }
}
