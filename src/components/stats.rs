use crate::theme::use_theme;
use leptos::prelude::*;

#[component]
pub fn Stats(
    #[prop(into)] value: String,
    #[prop(into)] label: String,
    #[prop(optional)] icon: Option<String>,
    #[prop(optional)] description: Option<String>,
    #[prop(optional)] diff: Option<f32>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();

    let stats_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "display: flex; \
             flex-direction: column; \
             gap: {}; \
             padding: {} {}; \
             background-color: {}; \
             border-radius: {}; \
             border: 1px solid {};",
            theme_val.spacing.xs,
            theme_val.spacing.md,
            theme_val.spacing.lg,
            scheme_colors.background,
            theme_val.radius.md,
            scheme_colors.border
        )
    };

    let header_styles = move || {
        "display: flex; \
         align-items: center; \
         justify-content: space-between;"
            .to_string()
    };

    let label_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let text_secondary = scheme_colors
            .get_color("gray", 6)
            .unwrap_or_else(|| "#868e96".to_string());
        format!(
            "font-size: {}; \
             color: {}; \
             font-weight: {}; \
             text-transform: uppercase; \
             letter-spacing: 0.5px;",
            theme_val.typography.font_sizes.xs, text_secondary, theme_val.typography.font_weights.medium
        )
    };

    let icon_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let text_secondary = scheme_colors
            .get_color("gray", 6)
            .unwrap_or_else(|| "#868e96".to_string());
        format!(
            "font-size: {}; \
             color: {};",
            theme_val.typography.font_sizes.lg, text_secondary
        )
    };

    let value_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "font-size: 28px; \
             font-weight: {}; \
             color: {}; \
             line-height: 1;",
            theme_val.typography.font_weights.bold, scheme_colors.text
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
             color: {};",
            theme_val.typography.font_sizes.sm, text_secondary
        )
    };

    let diff_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let diff_value = diff.unwrap_or(0.0);
        let color = if diff_value > 0.0 {
            scheme_colors
                .get_color("green", 6)
                .unwrap_or_else(|| "#37b24d".to_string())
        } else if diff_value < 0.0 {
            scheme_colors
                .get_color("red", 6)
                .unwrap_or_else(|| "#f03e3e".to_string())
        } else {
            scheme_colors
                .get_color("gray", 6)
                .unwrap_or_else(|| "#868e96".to_string())
        };

        format!(
            "font-size: {}; \
             font-weight: {}; \
             color: {};",
            theme_val.typography.font_sizes.sm, theme_val.typography.font_weights.medium, color
        )
    };

    let class_str = format!("mingot-stats {}", class.unwrap_or_default());

    view! {
        <div
            class=class_str
            style=move || {
                if let Some(s) = style.as_ref() {
                    format!("{}; {}", stats_styles(), s)
                } else {
                    stats_styles()
                }
            }
        >

            <div style=header_styles>
                <span style=label_styles>{label}</span>
                {icon.map(|i| view! { <span style=icon_styles>{i}</span> })}
            </div>

            <div style=value_styles>{value}</div>

            {description.map(|d| view! { <div style=description_styles>{d}</div> })}

            {diff.map(|d| {
                let symbol = if d > 0.0 { "↑" } else if d < 0.0 { "↓" } else { "" };
                let formatted = if d > 0.0 {
                    format!("+{:.1}%", d)
                } else {
                    format!("{:.1}%", d)
                };
                view! {
                    <div style=diff_styles>
                        <span>{symbol}</span>
                        " "
                        <span>{formatted}</span>
                        " compared to last month"
                    </div>
                }
            })}

        </div>
    }
}

#[component]
pub fn StatsGroup(
    #[prop(optional)] cols: Option<u32>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let cols = cols.unwrap_or(3);

    let group_styles = move || {
        let theme_val = theme.get();
        format!(
            "display: grid; \
             grid-template-columns: repeat({}, 1fr); \
             gap: {}; \
             width: 100%;",
            cols, theme_val.spacing.md
        )
    };


    let class_str = format!("mingot-stats-group {}", class.unwrap_or_default());

    view! {
        <div
            class=class_str
            style=move || {
                let base = group_styles();
                if let Some(s) = style.as_ref() {
                    format!("{}; {}", base, s)
                } else {
                    base
                }
            }
        >

            {children()}
        </div>
    }
}
