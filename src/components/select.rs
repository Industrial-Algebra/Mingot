use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;
use leptos::ev;

#[derive(Clone, Debug, PartialEq)]
pub struct SelectOption {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SelectVariant {
    Default,
    Filled,
    Unstyled,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SelectSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

#[component]
pub fn Select(
    #[prop(optional)] variant: Option<SelectVariant>,
    #[prop(optional)] size: Option<SelectSize>,
    #[prop(optional)] placeholder: Option<String>,
    #[prop(optional)] value: Option<RwSignal<String>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] error: Option<String>,
    #[prop(optional)] required: bool,
    #[prop(into)] options: Vec<SelectOption>,
    #[prop(optional)] on_change: Option<Callback<String>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] description: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let variant = variant.unwrap_or(SelectVariant::Default);
    let size = size.unwrap_or(SelectSize::Md);

    let select_value = value.unwrap_or_else(|| RwSignal::new(String::new()));

    let error_clone = error.clone();
    let select_styles = move || {
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
            .add("appearance", "none")
            .add("background-image", "url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='12' height='12' viewBox='0 0 12 12'%3E%3Cpath fill='%23666' d='M6 9L1 4h10z'/%3E%3C/svg%3E\")")
            .add("background-repeat", "no-repeat")
            .add("background-position", "right 0.75rem center")
            .add("padding-right", "2.5rem");

        // Size-based styles
        match size {
            SelectSize::Xs => {
                builder
                    .add("height", "1.875rem")
                    .add("padding-left", "0.625rem")
                    .add("font-size", theme_val.typography.font_sizes.xs);
            }
            SelectSize::Sm => {
                builder
                    .add("height", "2.25rem")
                    .add("padding-left", "0.75rem")
                    .add("font-size", theme_val.typography.font_sizes.sm);
            }
            SelectSize::Md => {
                builder
                    .add("height", "2.625rem")
                    .add("padding-left", "0.875rem")
                    .add("font-size", theme_val.typography.font_sizes.sm);
            }
            SelectSize::Lg => {
                builder
                    .add("height", "3.125rem")
                    .add("padding-left", "1rem")
                    .add("font-size", theme_val.typography.font_sizes.md);
            }
            SelectSize::Xl => {
                builder
                    .add("height", "3.75rem")
                    .add("padding-left", "1.125rem")
                    .add("font-size", theme_val.typography.font_sizes.lg);
            }
        }

        // Variant-based styles
        match variant {
            SelectVariant::Default => {
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
            SelectVariant::Filled => {
                let bg_color = scheme_colors.get_color("gray", 1).unwrap_or_else(|| "#f1f3f5".to_string());

                builder
                    .add("background-color", bg_color)
                    .add("color", scheme_colors.text.clone())
                    .add("border", "1px solid transparent");
            }
            SelectVariant::Unstyled => {
                builder
                    .add("background-color", "transparent")
                    .add("color", scheme_colors.text.clone())
                    .add("border", "none")
                    .add("padding-left", "0");
            }
        }

        // Disabled state
        if disabled {
            builder
                .add("opacity", "0.6")
                .add("cursor", "not-allowed");
        } else {
            builder.add("cursor", "pointer");
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let handle_change = move |ev: ev::Event| {
        let value = event_target_value(&ev);
        select_value.set(value.clone());
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

    let class_str = format!("mingot-select {}", class.unwrap_or_default());

    view! {
        <div class="mingot-select-wrapper" style="width: 100%;">
            {label.map(|l| view! {
                <label style=label_styles>
                    {l}
                    {if required { " *" } else { "" }}
                </label>
            })}

            <select
                class=class_str
                style=select_styles
                disabled=disabled
                required=required
                prop:value=move || select_value.get()
                on:change=handle_change
            >
                {placeholder.map(|p| view! {
                    <option value="" disabled=true selected=true>{p}</option>
                })}

                {options.into_iter().map(|opt| view! {
                    <option value=opt.value.clone() disabled=opt.disabled>
                        {opt.label}
                    </option>
                }).collect::<Vec<_>>()}
            </select>

            {description.map(|d| view! {
                <div style=description_styles>{d}</div>
            })}

            {error.map(|e| view! {
                <div style=error_styles>{e}</div>
            })}
        </div>
    }
}
