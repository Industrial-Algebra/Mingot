use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CheckboxSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

#[component]
pub fn Checkbox<F>(
    #[prop(optional)] checked: Option<RwSignal<bool>>,
    #[prop(optional)] size: Option<CheckboxSize>,
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
    F: Fn(bool) + Copy + Send + Sync + 'static,
{
    let theme = use_theme();
    let size = size.unwrap_or(CheckboxSize::Md);
    let color = color.unwrap_or_else(|| "blue".to_string());

    let is_checked = checked.unwrap_or_else(|| RwSignal::new(false));

    let error_clone = error.clone();

    let checkbox_size = match size {
        CheckboxSize::Xs => "1rem",
        CheckboxSize::Sm => "1.125rem",
        CheckboxSize::Md => "1.25rem",
        CheckboxSize::Lg => "1.375rem",
        CheckboxSize::Xl => "1.5rem",
    };

    let checkbox_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        let check_color = scheme_colors
            .get_color(&color, 6)
            .unwrap_or_else(|| "#228be6".to_string());
        let border_color = if error_clone.is_some() {
            scheme_colors
                .get_color("red", 6)
                .unwrap_or_else(|| "#fa5252".to_string())
        } else {
            scheme_colors.border.clone()
        };

        builder
            .add("width", checkbox_size)
            .add("height", checkbox_size)
            .add("min-width", checkbox_size)
            .add("min-height", checkbox_size)
            .add("border-radius", theme_val.radius.sm)
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
            .add(
                "background-color",
                if is_checked.get() {
                    check_color.clone()
                } else {
                    scheme_colors.background.clone()
                },
            )
            .add("cursor", if disabled { "not-allowed" } else { "pointer" })
            .add("transition", "all 0.15s ease")
            .add("appearance", "none")
            .add("display", "inline-flex")
            .add("align-items", "center")
            .add("justify-content", "center");

        if disabled {
            builder.add("opacity", "0.6");
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let checkmark_styles = move || {
        let display = if is_checked.get() { "block" } else { "none" };
        format!(
            "display: {}; \
             width: 0.4em; \
             height: 0.7em; \
             border: solid white; \
             border-width: 0 0.15em 0.15em 0; \
             transform: rotate(45deg); \
             margin-top: -0.1em;",
            display
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

    let handle_change = move |_ev: ev::Event| {
        if !disabled {
            let new_value = !is_checked.get();
            is_checked.set(new_value);
            if let Some(callback) = &on_change {
                callback(new_value);
            }
        }
    };

    let class_str = format!("mingot-checkbox {}", class.unwrap_or_default());

    view! {
        <div class="mingot-checkbox-container">
            <label style=wrapper_styles>
                <input
                    type="checkbox"
                    class=class_str
                    style=checkbox_styles
                    checked=move || is_checked.get()
                    disabled=disabled
                    on:change=handle_change
                />
                <span class="mingot-checkbox-checkmark" style=checkmark_styles></span>

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
