use crate::theme::use_theme;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ProgressSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl ProgressSize {
    fn to_height(self) -> &'static str {
        match self {
            ProgressSize::Xs => "4px",
            ProgressSize::Sm => "6px",
            ProgressSize::Md => "8px",
            ProgressSize::Lg => "12px",
            ProgressSize::Xl => "16px",
        }
    }
}

#[component]
pub fn Progress(
    #[prop(into)] value: Signal<f32>,
    #[prop(optional)] size: Option<ProgressSize>,
    #[prop(optional, into)] color: Option<String>,
    #[prop(optional)] striped: bool,
    #[prop(optional)] animate: bool,
    #[prop(optional, into)] label: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let size = size.unwrap_or(ProgressSize::Md);

    let container_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "position: relative; \
             width: 100%; \
             height: {}; \
             background-color: {}; \
             border-radius: {}; \
             overflow: hidden;",
            size.to_height(),
            scheme_colors
                .get_color("gray", 1)
                .unwrap_or_else(|| "#f1f3f5".to_string()),
            theme_val.radius.sm
        )
    };

    let bar_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let progress = value.get().clamp(0.0, 100.0);

        let bg_color = if let Some(ref c) = color {
            scheme_colors
                .get_color(c, 6)
                .unwrap_or_else(|| "#228be6".to_string())
        } else {
            scheme_colors
                .get_color("blue", 6)
                .unwrap_or_else(|| "#228be6".to_string())
        };

        let background = if striped {
            format!(
                "repeating-linear-gradient(\
                    45deg, \
                    {}, \
                    {} 10px, \
                    rgba(255, 255, 255, 0.15) 10px, \
                    rgba(255, 255, 255, 0.15) 20px\
                )",
                bg_color, bg_color
            )
        } else {
            bg_color
        };

        let animation = if animate {
            "mingot-progress-animation 1s linear infinite"
        } else {
            "none"
        };

        format!(
            "height: 100%; \
             width: {}%; \
             background: {}; \
             transition: width 0.3s ease; \
             animation: {}; \
             display: flex; \
             align-items: center; \
             justify-content: center;",
            progress, background, animation
        )
    };

    let label_styles = move || {
        let theme_val = theme.get();
        format!(
            "font-size: {}; \
             font-weight: {}; \
             color: #ffffff; \
             padding: 0 {};",
            theme_val.typography.font_sizes.xs,
            theme_val.typography.font_weights.bold,
            theme_val.spacing.xs
        )
    };

    let class_str = format!("mingot-progress {}", class.unwrap_or_default());

    view! {
        <>
            <style>
                "@keyframes mingot-progress-animation {
                    0% { background-position: 0 0; }
                    100% { background-position: 40px 0; }
                }"
            </style>
            <div
                class=class_str
                style=move || {
                    if let Some(s) = style.as_ref() {
                        format!("{}; {}", container_styles(), s)
                    } else {
                        container_styles()
                    }
                }
            >

                <div class="mingot-progress-bar" style=bar_styles>
                    {label.as_ref().map(|l| {
                        view! { <span style=label_styles>{l.clone()}</span> }
                    })}
                </div>
            </div>
        </>
    }
}
