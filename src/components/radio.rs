use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum RadioSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

#[component]
pub fn Radio<F>(
    #[prop(into)] value: String,
    #[prop(optional)] checked: Option<RwSignal<bool>>,
    #[prop(optional, into)] name: Option<String>,
    #[prop(optional)] size: Option<RadioSize>,
    #[prop(optional, into)] color: Option<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional)] on_change: Option<F>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
) -> impl IntoView
where
    F: Fn(String) + Copy + Send + Sync + 'static,
{
    let theme = use_theme();
    let size = size.unwrap_or(RadioSize::Md);
    let color = color.unwrap_or_else(|| "blue".to_string());

    let is_checked = checked.unwrap_or_else(|| RwSignal::new(false));

    let error_clone = error.clone();
    let color_clone = color.clone();

    let radio_size = match size {
        RadioSize::Xs => "1rem",
        RadioSize::Sm => "1.125rem",
        RadioSize::Md => "1.25rem",
        RadioSize::Lg => "1.375rem",
        RadioSize::Xl => "1.5rem",
    };

    let radio_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        let check_color = scheme_colors
            .get_color(&color_clone, 6)
            .unwrap_or_else(|| "#228be6".to_string());
        let border_color = if error_clone.is_some() {
            scheme_colors
                .get_color("red", 6)
                .unwrap_or_else(|| "#fa5252".to_string())
        } else {
            scheme_colors.border.clone()
        };

        builder
            .add("width", radio_size)
            .add("height", radio_size)
            .add("min-width", radio_size)
            .add("min-height", radio_size)
            .add("border-radius", "50%")
            .add(
                "border",
                format!(
                    "1px solid {}",
                    if is_checked.get() {
                        &check_color
                    } else {
                        &border_color
                    }
                ),
            )
            .add("background-color", scheme_colors.background.clone())
            .add("cursor", if disabled { "not-allowed" } else { "pointer" })
            .add("transition", "all 0.15s ease")
            .add("appearance", "none")
            .add("display", "inline-flex")
            .add("align-items", "center")
            .add("justify-content", "center")
            .add("position", "relative");

        if disabled {
            builder.add("opacity", "0.6");
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let dot_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let check_color = scheme_colors
            .get_color(&color, 6)
            .unwrap_or_else(|| "#228be6".to_string());

        let display = if is_checked.get() { "block" } else { "none" };
        format!(
            "display: {}; \
             width: 50%; \
             height: 50%; \
             border-radius: 50%; \
             background-color: {};",
            display, check_color
        )
    };

    let wrapper_styles =
        move || "display: flex; align-items: flex-start; gap: 0.5rem; cursor: pointer;".to_string();

    let label_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "font-size: {}; \
             font-weight: {}; \
             color: {}; \
             cursor: {}; \
             user-select: none;",
            theme_val.typography.font_sizes.sm,
            theme_val.typography.font_weights.normal,
            scheme_colors.text,
            if disabled { "not-allowed" } else { "pointer" }
        )
    };

    let description_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "font-size: {}; \
             color: {}; \
             margin-top: 0.125rem;",
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
            "margin-top: 0.25rem; \
             font-size: {}; \
             color: {};",
            theme_val.typography.font_sizes.xs,
            scheme_colors
                .get_color("red", 6)
                .unwrap_or_else(|| "#fa5252".to_string())
        )
    };

    let value_clone = value.clone();
    let handle_change = move |_ev: ev::Event| {
        if !disabled {
            is_checked.set(true);
            if let Some(callback) = &on_change {
                callback(value_clone.clone());
            }
        }
    };

    let class_str = format!("mingot-radio {}", class.unwrap_or_default());

    view! {
        <div class="mingot-radio-container">
            <label style=wrapper_styles>
                <input
                    type="radio"
                    class=class_str
                    style=radio_styles
                    checked=move || is_checked.get()
                    disabled=disabled
                    name=name.unwrap_or_default()
                    value=value
                    on:change=handle_change
                />
                <span class="mingot-radio-dot" style=dot_styles></span>

                {if label.is_some() || description.is_some() {
                    view! {
                        <div style="display: flex; flex-direction: column;">
                            {label.map(|l| view! {
                                <span style=label_styles>{l}</span>
                            })}
                            {description.map(|d| view! {
                                <span style=description_styles>{d}</span>
                            })}
                        </div>
                    }.into_any()
                } else {
                    ().into_any()
                }}
            </label>

            {error.map(|e| view! {
                <div style=error_styles>{e}</div>
            })}
        </div>
    }
}

#[component]
pub fn RadioGroup<F>(
    #[prop(optional)] _value: Option<RwSignal<String>>,
    #[prop(optional, into)] _name: Option<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional, into)] error: Option<String>,
    #[prop(optional)] _on_change: Option<F>,
    children: Children,
) -> impl IntoView
where
    F: Fn(String) + Copy + Send + Sync + 'static,
{
    let theme = use_theme();

    let label_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "display: block; \
             margin-bottom: {}; \
             font-size: {}; \
             font-weight: {}; \
             color: {};",
            theme_val.spacing.sm,
            theme_val.typography.font_sizes.sm,
            theme_val.typography.font_weights.medium,
            scheme_colors.text
        )
    };

    let description_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "margin-bottom: {}; \
             font-size: {}; \
             color: {};",
            theme_val.spacing.sm,
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
            "margin-top: {}; \
             font-size: {}; \
             color: {};",
            theme_val.spacing.xs,
            theme_val.typography.font_sizes.xs,
            scheme_colors
                .get_color("red", 6)
                .unwrap_or_else(|| "#fa5252".to_string())
        )
    };

    view! {
        <div class="mingot-radio-group" style="display: flex; flex-direction: column; gap: 0.5rem;">
            {label.map(|l| view! {
                <div style=label_styles>{l}</div>
            })}

            {description.map(|d| view! {
                <div style=description_styles>{d}</div>
            })}

            <div style="display: flex; flex-direction: column; gap: 0.75rem;">
                {children()}
            </div>

            {error.map(|e| view! {
                <div style=error_styles>{e}</div>
            })}
        </div>
    }
}
