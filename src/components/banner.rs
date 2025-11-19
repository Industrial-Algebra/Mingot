use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BannerVariant {
    Info,
    Success,
    Warning,
    Error,
    Default,
}

impl BannerVariant {
    fn color_name(&self) -> &'static str {
        match self {
            BannerVariant::Info => "blue",
            BannerVariant::Success => "green",
            BannerVariant::Warning => "yellow",
            BannerVariant::Error => "red",
            BannerVariant::Default => "gray",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BannerPosition {
    Static,
    Fixed,
    Sticky,
}

impl BannerPosition {
    fn as_str(&self) -> &'static str {
        match self {
            BannerPosition::Static => "static",
            BannerPosition::Fixed => "fixed",
            BannerPosition::Sticky => "sticky",
        }
    }
}

#[component]
pub fn Banner(
    #[prop(optional)] variant: Option<BannerVariant>,
    #[prop(optional)] position: Option<BannerPosition>,
    #[prop(optional)] with_border: bool,
    #[prop(optional)] dismissible: bool,
    #[prop(optional)] opened: Option<RwSignal<bool>>,
    #[prop(optional, into)] on_close: Option<Callback<()>>,
    #[prop(optional, into)] icon: Option<String>,
    #[prop(optional, into)] padding: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let variant = variant.unwrap_or(BannerVariant::Info);
    let position = position.unwrap_or(BannerPosition::Static);

    let is_opened = opened.unwrap_or_else(|| RwSignal::new(true));

    let banner_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        if !is_opened.get() {
            return "display: none;".to_string();
        }

        let bg_color = scheme_colors
            .get_color(variant.color_name(), 0)
            .unwrap_or_else(|| "#e7f5ff".to_string());
        let text_color = scheme_colors
            .get_color(variant.color_name(), 7)
            .unwrap_or_else(|| "#1864ab".to_string());
        let border_color = scheme_colors
            .get_color(variant.color_name(), 4)
            .unwrap_or_else(|| "#74c0fc".to_string());

        builder
            .add("position", position.as_str())
            .add("top", "0")
            .add("left", "0")
            .add("right", "0")
            .add("display", "flex")
            .add("align-items", "center")
            .add("justify-content", "center")
            .add("background-color", bg_color)
            .add("color", text_color)
            .add("z-index", "200")
            .add("transition", "all 0.15s ease");

        if let Some(p) = padding.as_ref() {
            builder.add("padding", p);
        } else {
            builder.add(
                "padding",
                format!("{} {}", theme_val.spacing.sm, theme_val.spacing.md),
            );
        }

        if with_border {
            builder.add("border-bottom", format!("1px solid {}", border_color));
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let content_styles = move || {
        "display: flex; align-items: center; gap: 0.75rem; flex: 1; max-width: 1200px;".to_string()
    };

    let close_button_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let text_color = scheme_colors
            .get_color(variant.color_name(), 7)
            .unwrap_or_else(|| "#1864ab".to_string());

        format!(
            "background: none; \
             border: none; \
             cursor: pointer; \
             padding: 0.25rem; \
             color: {}; \
             font-size: 1.25rem; \
             line-height: 1; \
             opacity: 0.7; \
             transition: opacity 0.15s ease; \
             margin-left: 1rem;",
            text_color
        )
    };

    let icon_styles = move || "font-size: 1.25rem; line-height: 1;".to_string();

    let handle_close = move || {
        is_opened.set(false);
        if let Some(callback) = on_close {
            callback.run(());
        }
    };

    let class_str = format!("mingot-banner {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=banner_styles>
            <div class="mingot-banner-content" style=content_styles>
                {icon.map(|i| view! {
                    <span class="mingot-banner-icon" style=icon_styles>{i}</span>
                })}
                <div style="flex: 1;">
                    {children()}
                </div>
            </div>
            {if dismissible {
                view! {
                    <button
                        class="mingot-banner-close"
                        style=close_button_styles
                        on:click=move |_| handle_close()
                        aria-label="Close banner"
                    >
                        "×"
                    </button>
                }.into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}
