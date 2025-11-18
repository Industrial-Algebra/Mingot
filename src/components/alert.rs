use crate::theme::use_theme;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlertVariant {
    Filled,
    Light,
    Outline,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AlertColor {
    Info,    // blue
    Success, // green
    Warning, // yellow
    Error,   // red
}

impl AlertColor {
    fn to_color_name(&self) -> &str {
        match self {
            AlertColor::Info => "blue",
            AlertColor::Success => "green",
            AlertColor::Warning => "yellow",
            AlertColor::Error => "red",
        }
    }

    fn default_icon(&self) -> &str {
        match self {
            AlertColor::Info => "ℹ️",
            AlertColor::Success => "✓",
            AlertColor::Warning => "⚠️",
            AlertColor::Error => "✕",
        }
    }
}

#[component]
pub fn Alert(
    #[prop(optional)] variant: Option<AlertVariant>,
    #[prop(optional)] color: Option<AlertColor>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] icon: Option<String>,
    #[prop(optional)] with_close_button: bool,
    #[prop(optional)] on_close: Option<Callback<()>>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let variant = variant.unwrap_or(AlertVariant::Light);
    let color = color.unwrap_or(AlertColor::Info);
    let is_visible = RwSignal::new(true);

    let icon_display = icon.unwrap_or_else(|| color.default_icon().to_string());

    let alert_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let color_name = color.to_color_name();

        let (bg_color, text_color, border_color) = match variant {
            AlertVariant::Filled => {
                let bg = scheme_colors
                    .get_color(color_name, 6)
                    .unwrap_or_else(|| "#228be6".to_string());
                (bg.clone(), "#ffffff".to_string(), bg)
            }
            AlertVariant::Light => {
                let bg = scheme_colors
                    .get_color(color_name, 0)
                    .unwrap_or_else(|| "#e7f5ff".to_string());
                let text = scheme_colors
                    .get_color(color_name, 7)
                    .unwrap_or_else(|| "#1c7ed6".to_string());
                let border = scheme_colors
                    .get_color(color_name, 3)
                    .unwrap_or_else(|| "#74c0fc".to_string());
                (bg, text, border)
            }
            AlertVariant::Outline => {
                let text = scheme_colors
                    .get_color(color_name, 7)
                    .unwrap_or_else(|| "#1c7ed6".to_string());
                let border = scheme_colors
                    .get_color(color_name, 6)
                    .unwrap_or_else(|| "#228be6".to_string());
                ("transparent".to_string(), text, border)
            }
        };

        let display = if is_visible.get() { "flex" } else { "none" };

        format!(
            "display: {}; \
             align-items: flex-start; \
             gap: {}; \
             padding: {} {}; \
             background-color: {}; \
             color: {}; \
             border: 1px solid {}; \
             border-radius: {}; \
             border-left: 4px solid {};",
            display,
            theme_val.spacing.sm,
            theme_val.spacing.md,
            theme_val.spacing.lg,
            bg_color,
            text_color,
            border_color,
            theme_val.radius.sm,
            border_color
        )
    };

    let icon_styles = move || {
        let theme_val = theme.get();
        format!(
            "font-size: {}; \
             line-height: 1; \
             flex-shrink: 0;",
            theme_val.typography.font_sizes.lg
        )
    };

    let content_styles = "flex: 1; display: flex; flex-direction: column; gap: 0.25rem;".to_string();

    let title_styles = move || {
        let theme_val = theme.get();
        format!(
            "font-weight: {}; \
             font-size: {}; \
             margin: 0;",
            theme_val.typography.font_weights.bold, theme_val.typography.font_sizes.md
        )
    };

    let close_button_styles = move || {
        let theme_val = theme.get();
        format!(
            "background: none; \
             border: none; \
             font-size: {}; \
             cursor: pointer; \
             padding: 0; \
             opacity: 0.6; \
             transition: opacity 0.15s ease; \
             flex-shrink: 0; \
             line-height: 1;",
            theme_val.typography.font_sizes.lg
        )
    };

    let handle_close = move |_| {
        is_visible.set(false);
        if let Some(callback) = on_close {
            callback.run(());
        }
    };

    let class_str = format!("mingot-alert {}", class.unwrap_or_default());

    view! {
        <div
            class=class_str
            style=move || {
                if let Some(s) = style.as_ref() {
                    format!("{}; {}", alert_styles(), s)
                } else {
                    alert_styles()
                }
            }
        >

            <div class="mingot-alert-icon" style=icon_styles>
                {icon_display.clone()}
            </div>

            <div class="mingot-alert-content" style=content_styles>
                {title.as_ref().map(|t| {
                    view! { <div class="mingot-alert-title" style=title_styles>{t.clone()}</div> }
                })}
                <div class="mingot-alert-message">{children()}</div>
            </div>

            {if with_close_button {
                view! {
                    <button class="mingot-alert-close" style=close_button_styles on:click=handle_close>
                        "✕"
                    </button>
                }
                    .into_any()
            } else {
                view! {}.into_any()
            }}

        </div>
    }
}
