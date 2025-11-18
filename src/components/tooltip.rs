use crate::theme::use_theme;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TooltipPosition {
    Top,
    Bottom,
    Left,
    Right,
}

#[component]
pub fn Tooltip(
    #[prop(into)] label: String,
    #[prop(optional)] position: Option<TooltipPosition>,
    #[prop(optional)] with_arrow: bool,
    #[prop(optional)] color: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let position = position.unwrap_or(TooltipPosition::Top);
    let is_visible = RwSignal::new(false);
    let color_clone = color.clone();

    let wrapper_styles = "position: relative; display: inline-block;".to_string();

    let tooltip_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let bg_color = if let Some(ref c) = color {
            scheme_colors
                .get_color(c, 6)
                .unwrap_or_else(|| "#000000".to_string())
        } else {
            "#000000".to_string()
        };

        let (transform_origin, transform, top, left, bottom, right) = match position {
            TooltipPosition::Top => (
                "bottom center",
                "translateX(-50%) translateY(-8px)",
                "auto",
                "50%",
                "100%",
                "auto",
            ),
            TooltipPosition::Bottom => (
                "top center",
                "translateX(-50%) translateY(8px)",
                "100%",
                "50%",
                "auto",
                "auto",
            ),
            TooltipPosition::Left => (
                "right center",
                "translateX(-8px) translateY(-50%)",
                "50%",
                "auto",
                "auto",
                "100%",
            ),
            TooltipPosition::Right => (
                "left center",
                "translateX(8px) translateY(-50%)",
                "50%",
                "100%",
                "auto",
                "auto",
            ),
        };

        let visibility = if is_visible.get() {
            "visible"
        } else {
            "hidden"
        };
        let opacity = if is_visible.get() { "1" } else { "0" };

        format!(
            "position: absolute; \
             top: {}; \
             left: {}; \
             bottom: {}; \
             right: {}; \
             transform: {}; \
             background-color: {}; \
             color: #ffffff; \
             padding: {} {}; \
             border-radius: {}; \
             font-size: {}; \
             white-space: nowrap; \
             z-index: 1000; \
             pointer-events: none; \
             visibility: {}; \
             opacity: {}; \
             transition: opacity 0.2s ease, visibility 0.2s ease; \
             transform-origin: {};",
            top,
            left,
            bottom,
            right,
            transform,
            bg_color,
            theme_val.spacing.xs,
            theme_val.spacing.sm,
            theme_val.radius.sm,
            theme_val.typography.font_sizes.sm,
            visibility,
            opacity,
            transform_origin
        )
    };

    let arrow_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let bg_color = if let Some(ref c) = color_clone {
            scheme_colors
                .get_color(c, 6)
                .unwrap_or_else(|| "#000000".to_string())
        } else {
            "#000000".to_string()
        };

        let (border_style, top, left, bottom, right) = match position {
            TooltipPosition::Top => (
                format!("5px solid {}; 5px solid transparent", bg_color),
                "auto",
                "50%",
                "-5px",
                "auto",
            ),
            TooltipPosition::Bottom => (
                format!("5px solid transparent; 5px solid {}", bg_color),
                "-5px",
                "50%",
                "auto",
                "auto",
            ),
            TooltipPosition::Left => (
                format!("5px solid transparent; 5px solid {}", bg_color),
                "50%",
                "auto",
                "auto",
                "-5px",
            ),
            TooltipPosition::Right => (
                format!("5px solid {}; 5px solid transparent", bg_color),
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

    let class_str = format!("mingot-tooltip-wrapper {}", class.unwrap_or_default());

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

            on:mouseenter=move |_| is_visible.set(true)
            on:mouseleave=move |_| is_visible.set(false)
        >
            {children()}
            <div class="mingot-tooltip" style=tooltip_styles>
                {label.clone()}
                {if with_arrow {
                    view! { <div class="mingot-tooltip-arrow" style=arrow_styles></div> }
                        .into_any()
                } else {
                    ().into_any()
                }}

            </div>
        </div>
    }
}
