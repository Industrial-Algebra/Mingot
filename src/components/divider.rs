use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DividerOrientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DividerLabelPosition {
    Left,
    Center,
    Right,
}

impl DividerLabelPosition {
    fn as_str(&self) -> &'static str {
        match self {
            DividerLabelPosition::Left => "flex-start",
            DividerLabelPosition::Center => "center",
            DividerLabelPosition::Right => "flex-end",
        }
    }
}

#[component]
pub fn Divider(
    #[prop(optional)] orientation: Option<DividerOrientation>,
    #[prop(optional)] label: Option<String>,
    #[prop(optional)] label_position: Option<DividerLabelPosition>,
    #[prop(optional)] size: Option<String>,
    #[prop(optional)] color: Option<String>,
    #[prop(optional)] variant: Option<DividerVariant>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let orientation = orientation.unwrap_or(DividerOrientation::Horizontal);
    let label_position = label_position.unwrap_or(DividerLabelPosition::Center);
    let variant = variant.unwrap_or(DividerVariant::Solid);

    let has_label = label.is_some();
    let color_clone = color.clone();
    let size_clone = size.clone();

    let divider_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        let border_color = if let Some(c) = color_clone.as_ref() {
            c.clone()
        } else {
            scheme_colors.border.clone()
        };

        let border_width = if let Some(s) = size_clone.as_ref() {
            s.clone()
        } else {
            "1px".to_string()
        };

        let border_style = match variant {
            DividerVariant::Solid => "solid",
            DividerVariant::Dashed => "dashed",
            DividerVariant::Dotted => "dotted",
        };

        match orientation {
            DividerOrientation::Horizontal => {
                if has_label {
                    builder
                        .add("display", "flex")
                        .add("align-items", "center")
                        .add("justify-content", label_position.as_str())
                        .add("gap", theme_val.spacing.sm);
                } else {
                    builder
                        .add("border-top", format!("{} {} {}", border_width, border_style, border_color))
                        .add("border-bottom", "none")
                        .add("border-left", "none")
                        .add("border-right", "none")
                        .add("margin", format!("{} 0", theme_val.spacing.md));
                }
            }
            DividerOrientation::Vertical => {
                builder
                    .add("border-left", format!("{} {} {}", border_width, border_style, border_color))
                    .add("border-top", "none")
                    .add("border-bottom", "none")
                    .add("border-right", "none")
                    .add("height", "100%")
                    .add("min-height", "1rem")
                    .add("margin", format!("0 {}", theme_val.spacing.md));
            }
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let line_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        let border_color = if let Some(c) = color.as_ref() {
            c.clone()
        } else {
            scheme_colors.border.clone()
        };

        let border_width = if let Some(s) = size.as_ref() {
            s.clone()
        } else {
            "1px".to_string()
        };

        let border_style = match variant {
            DividerVariant::Solid => "solid",
            DividerVariant::Dashed => "dashed",
            DividerVariant::Dotted => "dotted",
        };

        format!(
            "flex: 1; \
             border-top: {} {} {}; \
             height: 0;",
            border_width, border_style, border_color
        )
    };

    let label_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        format!(
            "color: {}; \
             font-size: {}; \
             white-space: nowrap; \
             padding: 0 {};",
            scheme_colors.get_color("gray", 6).unwrap_or_else(|| "#868e96".to_string()),
            theme_val.typography.font_sizes.sm,
            theme_val.spacing.xs
        )
    };

    let class_str = format!("mingot-divider {}", class.unwrap_or_default());

    if orientation == DividerOrientation::Horizontal && has_label {
        let label_text = label.unwrap();
        view! {
            <div class=class_str style=divider_styles>
                {if label_position == DividerLabelPosition::Center || label_position == DividerLabelPosition::Right {
                    view! { <div style=line_styles()></div> }.into_any()
                } else {
                    view! {}.into_any()
                }}
                <span style=label_styles>{label_text}</span>
                {if label_position == DividerLabelPosition::Center || label_position == DividerLabelPosition::Left {
                    view! { <div style=line_styles()></div> }.into_any()
                } else {
                    view! {}.into_any()
                }}
            </div>
        }.into_any()
    } else {
        view! {
            <hr class=class_str style=divider_styles />
        }.into_any()
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DividerVariant {
    Solid,
    Dashed,
    Dotted,
}
