use crate::theme::use_theme;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PopoverPosition {
    Top,
    Bottom,
    Left,
    Right,
}

#[component]
pub fn Popover(
    #[prop(optional)] opened: Option<RwSignal<bool>>,
    #[prop(optional)] position: Option<PopoverPosition>,
    #[prop(optional)] with_arrow: bool,
    #[prop(optional)] width: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let is_opened = opened.unwrap_or_else(|| RwSignal::new(false));
    let position = position.unwrap_or(PopoverPosition::Bottom);

    provide_context::<RwSignal<bool>>(is_opened);
    provide_context::<Signal<PopoverPosition>>(Signal::derive(move || position));
    provide_context::<Signal<bool>>(Signal::derive(move || with_arrow));
    provide_context::<Signal<Option<String>>>(Signal::derive(move || width.clone()));

    let wrapper_styles = "position: relative; display: inline-block;".to_string();
    let class_str = format!("mingot-popover {}", class.unwrap_or_default());

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
        >

            {children()}
        </div>
    }
}

#[component]
pub fn PopoverTarget(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let is_opened = use_context::<RwSignal<bool>>().unwrap_or_else(|| RwSignal::new(false));

    let handle_click = move |_| {
        is_opened.update(|o| *o = !*o);
    };

    let target_styles = "cursor: pointer;".to_string();
    let class_str = format!("mingot-popover-target {}", class.unwrap_or_default());

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
pub fn PopoverDropdown(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let is_opened = use_context::<RwSignal<bool>>().unwrap_or_else(|| RwSignal::new(false));
    let position = use_context::<Signal<PopoverPosition>>()
        .unwrap_or_else(|| Signal::derive(move || PopoverPosition::Bottom));
    let with_arrow = use_context::<Signal<bool>>().unwrap_or_else(|| Signal::derive(move || false));
    let width =
        use_context::<Signal<Option<String>>>().unwrap_or_else(|| Signal::derive(move || None));

    let dropdown_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let is_open = is_opened.get();

        let (top, left, bottom, right, transform) = match position.get() {
            PopoverPosition::Top => (
                "auto",
                "50%",
                "100%",
                "auto",
                "translateX(-50%) translateY(-8px)",
            ),
            PopoverPosition::Bottom => (
                "100%",
                "50%",
                "auto",
                "auto",
                "translateX(-50%) translateY(8px)",
            ),
            PopoverPosition::Left => (
                "50%",
                "auto",
                "auto",
                "100%",
                "translateX(-8px) translateY(-50%)",
            ),
            PopoverPosition::Right => (
                "50%",
                "100%",
                "auto",
                "auto",
                "translateX(8px) translateY(-50%)",
            ),
        };

        let display = if is_open { "block" } else { "none" };
        let width_str = width.get().unwrap_or_else(|| "260px".to_string());

        format!(
            "position: absolute; \
             top: {}; \
             left: {}; \
             bottom: {}; \
             right: {}; \
             transform: {}; \
             width: {}; \
             background-color: {}; \
             border: 1px solid {}; \
             border-radius: {}; \
             box-shadow: {}; \
             padding: {}; \
             z-index: 1000; \
             display: {};",
            top,
            left,
            bottom,
            right,
            transform,
            width_str,
            scheme_colors.background,
            scheme_colors.border,
            theme_val.radius.md,
            theme_val.shadows.lg,
            theme_val.spacing.md,
            display
        )
    };

    let arrow_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        let (border_style, top, left, bottom, right) = match position.get() {
            PopoverPosition::Top => (
                format!(
                    "5px solid {}; 5px solid transparent",
                    scheme_colors.background
                ),
                "auto",
                "50%",
                "-5px",
                "auto",
            ),
            PopoverPosition::Bottom => (
                format!(
                    "5px solid transparent; 5px solid {}",
                    scheme_colors.background
                ),
                "-5px",
                "50%",
                "auto",
                "auto",
            ),
            PopoverPosition::Left => (
                format!(
                    "5px solid transparent; 5px solid {}",
                    scheme_colors.background
                ),
                "50%",
                "auto",
                "auto",
                "-5px",
            ),
            PopoverPosition::Right => (
                format!(
                    "5px solid {}; 5px solid transparent",
                    scheme_colors.background
                ),
                "50%",
                "-5px",
                "auto",
                "auto",
            ),
        };

        format!(
            "position: absolute; \
             top: {}; \
             left: {}; \
             bottom: {}; \
             right: {}; \
             width: 0; \
             height: 0; \
             border: {}; \
             transform: translate(-50%, -50%);",
            top, left, bottom, right, border_style
        )
    };

    let class_str = format!("mingot-popover-dropdown {}", class.unwrap_or_default());

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

            {move || {
                if with_arrow.get() {
                    view! { <div class="mingot-popover-arrow" style=arrow_styles></div> }
                        .into_any()
                } else {
                    ().into_any()
                }
            }}

        </div>
    }
}
