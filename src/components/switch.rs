use crate::theme::use_theme;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SwitchSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl SwitchSize {
    fn dimensions(&self) -> (u32, u32, u32) {
        // (width, height, thumb_size)
        match self {
            SwitchSize::Xs => (28, 14, 10),
            SwitchSize::Sm => (36, 18, 14),
            SwitchSize::Md => (44, 22, 18),
            SwitchSize::Lg => (52, 26, 22),
            SwitchSize::Xl => (60, 30, 26),
        }
    }
}

#[component]
pub fn Switch<F>(
    #[prop(optional)] checked: Option<RwSignal<bool>>,
    #[prop(optional)] size: Option<SwitchSize>,
    #[prop(optional, into)] color: Option<String>,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] description: Option<String>,
    #[prop(optional)] disabled: bool,
    #[prop(optional)] on_change: Option<F>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
) -> impl IntoView
where
    F: Fn(bool) + Copy + Send + Sync + 'static,
{
    let theme = use_theme();
    let size = size.unwrap_or(SwitchSize::Md);
    let is_checked = checked.unwrap_or_else(|| RwSignal::new(false));

    let (width, height, thumb_size) = size.dimensions();

    let wrapper_styles =
        "display: inline-flex; align-items: flex-start; gap: 0.75rem; cursor: pointer;".to_string();

    let switch_track_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let checked_val = is_checked.get();

        let bg_color = if checked_val {
            if let Some(ref c) = color {
                scheme_colors
                    .get_color(c, 6)
                    .unwrap_or_else(|| "#228be6".to_string())
            } else {
                scheme_colors
                    .get_color("blue", 6)
                    .unwrap_or_else(|| "#228be6".to_string())
            }
        } else {
            scheme_colors
                .get_color("gray", 4)
                .unwrap_or_else(|| "#ced4da".to_string())
        };

        let opacity = if disabled { "0.5" } else { "1" };
        let cursor = if disabled { "not-allowed" } else { "pointer" };

        format!(
            "position: relative; \
             width: {}px; \
             height: {}px; \
             background-color: {}; \
             border-radius: {}px; \
             transition: background-color 0.2s ease; \
             flex-shrink: 0; \
             opacity: {}; \
             cursor: {};",
            width,
            height,
            bg_color,
            height / 2,
            opacity,
            cursor
        )
    };

    let switch_thumb_styles = move || {
        let checked_val = is_checked.get();
        let offset = if checked_val {
            width - thumb_size - 2
        } else {
            2
        };

        format!(
            "position: absolute; \
             top: 50%; \
             left: {}px; \
             width: {}px; \
             height: {}px; \
             background-color: #ffffff; \
             border-radius: 50%; \
             transform: translateY(-50%); \
             transition: left 0.2s ease; \
             box-shadow: 0 1px 3px rgba(0, 0, 0, 0.2);",
            offset, thumb_size, thumb_size
        )
    };

    let label_wrapper_styles = "display: flex; flex-direction: column; gap: 0.25rem;".to_string();

    let label_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "font-size: {}; \
             font-weight: {}; \
             color: {}; \
             user-select: none;",
            theme_val.typography.font_sizes.sm,
            theme_val.typography.font_weights.medium,
            scheme_colors.text
        )
    };

    let description_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let text_secondary = scheme_colors
            .get_color("gray", 6)
            .unwrap_or_else(|| "#868e96".to_string());
        format!(
            "font-size: {}; \
             color: {}; \
             line-height: 1.4;",
            theme_val.typography.font_sizes.xs, text_secondary
        )
    };

    let handle_click = move |_| {
        if !disabled {
            let new_value = !is_checked.get();
            is_checked.set(new_value);
            if let Some(callback) = &on_change {
                callback(new_value);
            }
        }
    };

    let class_str = format!("mingot-switch {}", class.unwrap_or_default());

    view! {
        <div
            class=class_str
            style=move || {
                if let Some(s) = style.as_ref() {
                    format!("{}; {}", wrapper_styles, s)
                } else {
                    wrapper_styles.clone()
                }
            }

            on:click=handle_click
        >
            <div class="mingot-switch-track" style=switch_track_styles>
                <div class="mingot-switch-thumb" style=switch_thumb_styles></div>
            </div>

            {if label.is_some() || description.is_some() {
                view! {
                    <div class="mingot-switch-label-wrapper" style=label_wrapper_styles>
                        {label.as_ref().map(|l| {
                            view! { <div class="mingot-switch-label" style=label_styles>{l.clone()}</div> }
                        })}
                        {description.as_ref().map(|d| {
                            view! {
                                <div class="mingot-switch-description" style=description_styles>
                                    {d.clone()}
                                </div>
                            }
                        })}
                    </div>
                }
                    .into_any()
            } else {
                ().into_any()
            }}

        </div>
    }
}
