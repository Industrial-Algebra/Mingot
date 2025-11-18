use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;
use leptos::ev;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextareaVariant {
    Default,
    Filled,
    Unstyled,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TextareaSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

#[component]
pub fn Textarea(
    #[prop(optional)] variant: Option<TextareaVariant>,
    #[prop(optional)] size: Option<TextareaSize>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] error: Option<String>,
    #[prop(optional)] required: bool,
    #[prop(optional)] rows: Option<u32>,
    #[prop(optional)] auto_size: bool,
    #[prop(optional)] _min_rows: Option<u32>,
    #[prop(optional)] _max_rows: Option<u32>,
    #[prop(optional)] on_input: Option<Callback<String>>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] description: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let variant = variant.unwrap_or(TextareaVariant::Default);
    let size = size.unwrap_or(TextareaSize::Md);
    let rows = rows.unwrap_or(3);

    let textarea_value = value.unwrap_or_else(|| RwSignal::new(String::new()));

    let error_clone = error.clone();
    let textarea_styles = move || {
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
            .add("box-sizing", "border-box")
            .add("resize", "vertical");

        // Size-based styles
        match size {
            TextareaSize::Xs => {
                builder
                    .add("padding", "0.375rem 0.625rem")
                    .add("font-size", theme_val.typography.font_sizes.xs)
                    .add("line-height", theme_val.typography.line_heights.md);
            }
            TextareaSize::Sm => {
                builder
                    .add("padding", "0.5rem 0.75rem")
                    .add("font-size", theme_val.typography.font_sizes.sm)
                    .add("line-height", theme_val.typography.line_heights.md);
            }
            TextareaSize::Md => {
                builder
                    .add("padding", "0.625rem 0.875rem")
                    .add("font-size", theme_val.typography.font_sizes.sm)
                    .add("line-height", theme_val.typography.line_heights.md);
            }
            TextareaSize::Lg => {
                builder
                    .add("padding", "0.75rem 1rem")
                    .add("font-size", theme_val.typography.font_sizes.md)
                    .add("line-height", theme_val.typography.line_heights.md);
            }
            TextareaSize::Xl => {
                builder
                    .add("padding", "0.875rem 1.125rem")
                    .add("font-size", theme_val.typography.font_sizes.lg)
                    .add("line-height", theme_val.typography.line_heights.md);
            }
        }

        // Variant-based styles
        match variant {
            TextareaVariant::Default => {
                let border_color = if error_clone.is_some() {
                    scheme_colors.get_color("red", 6).unwrap_or_else(|| "#fa5252".to_string())
                } else {
                    scheme_colors.border.clone()
                };

                builder
                    .add("background-color", scheme_colors.background.clone())
                    .add("color", scheme_colors.text.clone())
                    .add("border", format!("1px solid {}", border_color));
            }
            TextareaVariant::Filled => {
                let bg_color = scheme_colors.get_color("gray", 1).unwrap_or_else(|| "#f1f3f5".to_string());

                builder
                    .add("background-color", bg_color)
                    .add("color", scheme_colors.text.clone())
                    .add("border", "1px solid transparent");
            }
            TextareaVariant::Unstyled => {
                builder
                    .add("background-color", "transparent")
                    .add("color", scheme_colors.text.clone())
                    .add("border", "none")
                    .add("padding", "0");
            }
        }

        // Disabled state
        if disabled {
            builder
                .add("opacity", "0.6")
                .add("cursor", "not-allowed");
        } else {
            builder.add("cursor", "text");
        }

        // Auto-size
        if auto_size {
            builder.add("resize", "none");
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let handle_input = move |ev: ev::Event| {
        let value = event_target_value(&ev);
        textarea_value.set(value.clone());
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
            scheme_colors.get_color("gray", 6).unwrap_or_else(|| "#868e96".to_string())
        )
    };

    let error_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "margin-top: 0.25rem; font-size: {}; color: {};",
            theme_val.typography.font_sizes.xs,
            scheme_colors.get_color("red", 6).unwrap_or_else(|| "#fa5252".to_string())
        )
    };

    let class_str = format!("mingot-textarea {}", class.unwrap_or_default());

    view! {
        <div class="mingot-textarea-wrapper" style="width: 100%;">
            {label.map(|l| view! {
                <label style=label_styles>
                    {l}
                    {if required { " *" } else { "" }}
                </label>
            })}

            <textarea
                class=class_str
                style=textarea_styles
                placeholder=placeholder.unwrap_or_default()
                disabled=disabled
                required=required
                rows=rows
                prop:value=move || textarea_value.get()
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
