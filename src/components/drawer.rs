use crate::theme::use_theme;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DrawerPosition {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DrawerSize {
    Xs,  // 240px
    Sm,  // 320px
    Md,  // 440px
    Lg,  // 620px
    Xl,  // 780px
    Full, // 100%
}

impl DrawerSize {
    fn to_size(&self) -> &str {
        match self {
            DrawerSize::Xs => "240px",
            DrawerSize::Sm => "320px",
            DrawerSize::Md => "440px",
            DrawerSize::Lg => "620px",
            DrawerSize::Xl => "780px",
            DrawerSize::Full => "100%",
        }
    }
}

#[component]
pub fn Drawer(
    #[prop(into)] opened: Signal<bool>,
    #[prop(optional)] on_close: Option<Callback<()>>,
    #[prop(optional)] position: Option<DrawerPosition>,
    #[prop(optional)] size: Option<DrawerSize>,
    #[prop(optional)] title: Option<String>,
    #[prop(optional)] with_overlay: bool,
    #[prop(optional)] with_close_button: bool,
    #[prop(optional)] padding: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let position = position.unwrap_or(DrawerPosition::Right);
    let size = size.unwrap_or(DrawerSize::Md);
    let with_overlay = with_overlay;
    let with_close_button = with_close_button;

    let overlay_styles = move || {
        let visible = opened.get();
        format!(
            "position: fixed; \
             top: 0; \
             left: 0; \
             right: 0; \
             bottom: 0; \
             background-color: rgba(0, 0, 0, 0.5); \
             z-index: 999; \
             opacity: {}; \
             visibility: {}; \
             transition: opacity 0.3s ease, visibility 0.3s ease;",
            if visible { "1" } else { "0" },
            if visible { "visible" } else { "hidden" }
        )
    };

    let drawer_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let visible = opened.get();
        let size_val = size.to_size();

        let (width, height, transform_closed, top, left, right, bottom) = match position {
            DrawerPosition::Left => (
                size_val,
                "100%",
                "translateX(-100%)",
                "0",
                "0",
                "auto",
                "0",
            ),
            DrawerPosition::Right => (
                size_val,
                "100%",
                "translateX(100%)",
                "0",
                "auto",
                "0",
                "0",
            ),
            DrawerPosition::Top => (
                "100%",
                size_val,
                "translateY(-100%)",
                "0",
                "0",
                "0",
                "auto",
            ),
            DrawerPosition::Bottom => (
                "100%",
                size_val,
                "translateY(100%)",
                "auto",
                "0",
                "0",
                "0",
            ),
        };

        let transform = if visible {
            "translateX(0) translateY(0)"
        } else {
            transform_closed
        };

        let padding_val = padding
            .as_ref()
            .map(|p| p.as_str())
            .unwrap_or(theme_val.spacing.lg);

        format!(
            "position: fixed; \
             top: {}; \
             left: {}; \
             right: {}; \
             bottom: {}; \
             width: {}; \
             height: {}; \
             background-color: {}; \
             box-shadow: {}; \
             z-index: 1000; \
             overflow-y: auto; \
             transform: {}; \
             transition: transform 0.3s ease; \
             display: flex; \
             flex-direction: column; \
             padding: {};",
            top,
            left,
            right,
            bottom,
            width,
            height,
            scheme_colors.background,
            theme_val.shadows.xl,
            transform,
            padding_val
        )
    };

    let header_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "display: flex; \
             align-items: center; \
             justify-content: space-between; \
             margin-bottom: {}; \
             padding-bottom: {}; \
             border-bottom: 1px solid {};",
            theme_val.spacing.md, theme_val.spacing.md, scheme_colors.border
        )
    };

    let title_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "font-size: {}; \
             font-weight: {}; \
             color: {}; \
             margin: 0;",
            theme_val.typography.font_sizes.lg,
            theme_val.typography.font_weights.bold,
            scheme_colors.text
        )
    };

    let close_button_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "background: none; \
             border: none; \
             font-size: {}; \
             cursor: pointer; \
             padding: {}; \
             color: {}; \
             display: flex; \
             align-items: center; \
             justify-content: center; \
             border-radius: {}; \
             transition: background-color 0.15s ease;",
            theme_val.typography.font_sizes.lg,
            theme_val.spacing.xs,
            scheme_colors.text,
            theme_val.radius.sm
        )
    };

    let handle_overlay_click = move |_| {
        if let Some(callback) = on_close {
            callback.run(());
        }
    };

    let handle_close_click = move |_| {
        if let Some(callback) = on_close {
            callback.run(());
        }
    };

    let class_str = format!("mingot-drawer {}", class.unwrap_or_default());

    view! {
        <>
            {move || {
                if opened.get() && with_overlay {
                    view! {
                        <div class="mingot-drawer-overlay" style=overlay_styles on:click=handle_overlay_click></div>
                    }
                        .into_any()
                } else {
                    view! {}.into_any()
                }
            }}

            <div
                class=class_str.clone()
                style=move || {
                    let drawer_style = drawer_styles();
                    let display = if opened.get() { "flex" } else { "none" };
                    if let Some(s) = style.as_ref() {
                        format!("{}; display: {}; {}", drawer_style, display, s)
                    } else {
                        format!("{}; display: {};", drawer_style, display)
                    }
                }
            >

                {if title.is_some() || with_close_button {
                    view! {
                        <div class="mingot-drawer-header" style=header_styles>
                            {title.as_ref().map(|t| {
                                view! { <h2 style=title_styles>{t.clone()}</h2> }
                            })}
                            {if with_close_button {
                                view! {
                                    <button
                                        class="mingot-drawer-close"
                                        style=close_button_styles
                                        on:click=handle_close_click
                                    >
                                        "✕"
                                    </button>
                                }
                                    .into_any()
                            } else {
                                view! {}.into_any()
                            }}

                        </div>
                    }
                        .into_any()
                } else {
                    view! {}.into_any()
                }}

                <div class="mingot-drawer-body" style="flex: 1;">
                    {children()}
                </div>
            </div>
        </>
    }
}
