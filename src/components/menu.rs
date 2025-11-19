use crate::theme::use_theme;
use leptos::prelude::*;

#[component]
pub fn Menu(
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let opened = RwSignal::new(false);

    provide_context::<RwSignal<bool>>(opened);

    let menu_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "position: relative; \
             display: inline-block; \
             background-color: {}; \
             border-radius: {};",
            scheme_colors.background, theme_val.radius.sm
        )
    };

    let class_str = format!("mingot-menu {}", class.unwrap_or_default());

    view! {
        <div
            class=class_str
            style=move || {
                if let Some(s) = style.as_ref() {
                    format!("{}; {}", menu_styles(), s)
                } else {
                    menu_styles()
                }
            }
        >

            {children()}
        </div>
    }
}

#[component]
pub fn MenuTarget(
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let opened = use_context::<RwSignal<bool>>().unwrap_or_else(|| RwSignal::new(false));

    let handle_click = move |_| {
        opened.update(|o| *o = !*o);
    };

    let target_styles = "cursor: pointer;".to_string();
    let class_str = format!("mingot-menu-target {}", class.unwrap_or_default());

    view! {
        <div
            class=class_str
            style=move || {
                if let Some(s) = style.as_ref() {
                    format!("{}; {}", target_styles, s)
                } else {
                    target_styles.clone()
                }
            }

            on:click=handle_click
        >
            {children()}
        </div>
    }
}

#[component]
pub fn MenuDropdown(
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let opened = use_context::<RwSignal<bool>>().unwrap_or_else(|| RwSignal::new(false));

    let dropdown_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let is_open = opened.get();

        let display = if is_open { "block" } else { "none" };

        format!(
            "position: absolute; \
             top: 100%; \
             left: 0; \
             margin-top: 0.25rem; \
             min-width: 200px; \
             background-color: {}; \
             border: 1px solid {}; \
             border-radius: {}; \
             box-shadow: {}; \
             z-index: 1000; \
             padding: {}; \
             display: {};",
            scheme_colors.background,
            scheme_colors.border,
            theme_val.radius.sm,
            theme_val.shadows.md,
            theme_val.spacing.xs,
            display
        )
    };

    let class_str = format!("mingot-menu-dropdown {}", class.unwrap_or_default());

    view! {
        <div
            class=class_str
            style=move || {
                if let Some(s) = style.as_ref() {
                    format!("{}; {}", dropdown_styles(), s)
                } else {
                    dropdown_styles()
                }
            }
        >

            {children()}
        </div>
    }
}

#[component]
pub fn MenuItem(
    #[prop(optional, into)] icon: Option<String>,
    #[prop(optional, into)] on_click: Option<Callback<()>>,
    #[prop(optional)] disabled: bool,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let opened = use_context::<RwSignal<bool>>().unwrap_or_else(|| RwSignal::new(false));

    let item_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        let cursor = if disabled { "not-allowed" } else { "pointer" };
        let opacity = if disabled { "0.5" } else { "1" };

        format!(
            "display: flex; \
             align-items: center; \
             gap: {}; \
             padding: {} {}; \
             border-radius: {}; \
             font-size: {}; \
             color: {}; \
             cursor: {}; \
             opacity: {}; \
             transition: background-color 0.15s ease; \
             user-select: none; \
             white-space: nowrap;",
            theme_val.spacing.sm,
            theme_val.spacing.xs,
            theme_val.spacing.sm,
            theme_val.radius.sm,
            theme_val.typography.font_sizes.sm,
            scheme_colors.text,
            cursor,
            opacity
        )
    };

    let handle_click = move |_| {
        if !disabled {
            if let Some(callback) = on_click {
                callback.run(());
            }
            opened.set(false);
        }
    };

    let class_str = format!("mingot-menu-item {}", class.unwrap_or_default());

    view! {
        <div
            class=class_str
            style=move || {
                if let Some(s) = style.as_ref() {
                    format!("{}; {}", item_styles(), s)
                } else {
                    item_styles()
                }
            }

            on:click=handle_click
        >
            {icon.as_ref().map(|i| view! { <span>{i.clone()}</span> })}
            <span>{children()}</span>
        </div>
    }
}

#[component]
pub fn MenuDivider(
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();

    let divider_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "height: 1px; \
             background-color: {}; \
             margin: {} 0;",
            scheme_colors.border, theme_val.spacing.xs
        )
    };

    let class_str = format!("mingot-menu-divider {}", class.unwrap_or_default());

    view! {
        <div
            class=class_str
            style=move || {
                if let Some(s) = style.as_ref() {
                    format!("{}; {}", divider_styles(), s)
                } else {
                    divider_styles()
                }
            }
        ></div>
    }
}

#[component]
pub fn MenuLabel(
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();

    let label_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let text_secondary = scheme_colors
            .get_color("gray", 6)
            .unwrap_or_else(|| "#868e96".to_string());
        format!(
            "font-size: {}; \
             font-weight: {}; \
             color: {}; \
             padding: {} {}; \
             text-transform: uppercase; \
             letter-spacing: 0.5px;",
            theme_val.typography.font_sizes.xs,
            theme_val.typography.font_weights.bold,
            text_secondary,
            theme_val.spacing.xs,
            theme_val.spacing.sm
        )
    };

    let class_str = format!("mingot-menu-label {}", class.unwrap_or_default());

    view! {
        <div
            class=class_str
            style=move || {
                if let Some(s) = style.as_ref() {
                    format!("{}; {}", label_styles(), s)
                } else {
                    label_styles()
                }
            }
        >

            {children()}
        </div>
    }
}
