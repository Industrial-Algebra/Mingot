use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TabsVariant {
    Default,
    Outline,
    Pills,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TabsOrientation {
    Horizontal,
    Vertical,
}

#[component]
pub fn Tabs(
    #[prop(into)] active: RwSignal<String>,
    #[prop(optional)] variant: Option<TabsVariant>,
    #[prop(optional)] orientation: Option<TabsOrientation>,
    #[prop(optional)] grow: bool,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let variant = variant.unwrap_or(TabsVariant::Default);
    let orientation = orientation.unwrap_or(TabsOrientation::Horizontal);

    // Provide context
    provide_context::<RwSignal<String>>(active);
    provide_context::<Signal<TabsVariant>>(Signal::derive(move || variant));
    provide_context::<Signal<TabsOrientation>>(Signal::derive(move || orientation));
    provide_context::<Signal<bool>>(Signal::derive(move || grow));

    let tabs_styles = move || {
        let mut builder = StyleBuilder::new();

        builder.add("display", "flex");

        match orientation {
            TabsOrientation::Horizontal => {
                builder.add("flex-direction", "column");
            }
            TabsOrientation::Vertical => {
                builder.add("flex-direction", "row");
            }
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-tabs {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=tabs_styles>
            {children()}
        </div>
    }
}

#[component]
pub fn TabsList(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let variant = use_context::<Signal<TabsVariant>>()
        .unwrap_or(Signal::derive(move || TabsVariant::Default));
    let orientation = use_context::<Signal<TabsOrientation>>()
        .unwrap_or(Signal::derive(move || TabsOrientation::Horizontal));

    let list_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        builder
            .add("display", "flex")
            .add("gap", theme_val.spacing.xs);

        match orientation.get() {
            TabsOrientation::Horizontal => {
                builder.add("flex-direction", "row");
            }
            TabsOrientation::Vertical => {
                builder
                    .add("flex-direction", "column")
                    .add("min-width", "150px");
            }
        }

        match variant.get() {
            TabsVariant::Default => {
                if orientation.get() == TabsOrientation::Horizontal {
                    builder.add(
                        "border-bottom",
                        format!("2px solid {}", scheme_colors.border.clone()),
                    );
                }
            }
            TabsVariant::Outline => {
                builder
                    .add(
                        "border",
                        format!("1px solid {}", scheme_colors.border.clone()),
                    )
                    .add("border-radius", theme_val.radius.sm)
                    .add("padding", theme_val.spacing.xs);
            }
            TabsVariant::Pills => {
                builder.add("padding", theme_val.spacing.xs);
            }
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-tabs-list {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=list_styles>
            {children()}
        </div>
    }
}

#[component]
pub fn TabsTab(
    #[prop(into)] value: String,
    #[prop(optional)] icon: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let active = use_context::<RwSignal<String>>().unwrap();
    let variant = use_context::<Signal<TabsVariant>>()
        .unwrap_or(Signal::derive(move || TabsVariant::Default));
    let grow = use_context::<Signal<bool>>().unwrap_or(Signal::derive(move || false));

    let value_clone = value.clone();
    let is_active = move || active.get() == value_clone;

    let tab_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        builder
            .add("display", "inline-flex")
            .add("align-items", "center")
            .add("justify-content", "center")
            .add("gap", "0.5rem")
            .add(
                "padding",
                format!("{} {}", theme_val.spacing.sm, theme_val.spacing.md),
            )
            .add("background", "none")
            .add("border", "none")
            .add("cursor", "pointer")
            .add("font-size", theme_val.typography.font_sizes.sm)
            .add(
                "font-weight",
                theme_val.typography.font_weights.medium.to_string(),
            )
            .add("transition", "all 0.15s ease")
            .add("white-space", "nowrap")
            .add("user-select", "none");

        if grow.get() {
            builder.add("flex", "1");
        }

        match variant.get() {
            TabsVariant::Default => {
                if is_active() {
                    let active_color = scheme_colors
                        .get_color("blue", 6)
                        .unwrap_or_else(|| "#228be6".to_string());
                    builder
                        .add("color", active_color.clone())
                        .add("border-bottom", format!("2px solid {}", active_color))
                        .add("margin-bottom", "-2px");
                } else {
                    builder
                        .add("color", scheme_colors.text.clone())
                        .add("border-bottom", "2px solid transparent");
                }
            }
            TabsVariant::Outline => {
                builder.add("border-radius", theme_val.radius.sm);
                if is_active() {
                    let active_color = scheme_colors
                        .get_color("blue", 6)
                        .unwrap_or_else(|| "#228be6".to_string());
                    builder
                        .add("background-color", active_color)
                        .add("color", scheme_colors.white.clone());
                } else {
                    builder
                        .add("background-color", "transparent")
                        .add("color", scheme_colors.text.clone());
                }
            }
            TabsVariant::Pills => {
                builder.add("border-radius", "9999px");
                if is_active() {
                    let light_color = scheme_colors
                        .get_color("blue", 0)
                        .unwrap_or_else(|| "#e7f5ff".to_string());
                    let active_color = scheme_colors
                        .get_color("blue", 6)
                        .unwrap_or_else(|| "#228be6".to_string());
                    builder
                        .add("background-color", light_color)
                        .add("color", active_color);
                } else {
                    builder
                        .add("background-color", "transparent")
                        .add("color", scheme_colors.text.clone());
                }
            }
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let handle_click = move |_| {
        active.set(value.clone());
    };

    let class_str = format!("mingot-tabs-tab {}", class.unwrap_or_default());

    view! {
        <button class=class_str style=tab_styles on:click=handle_click>
            {icon.map(|i| view! { <span>{i}</span> })}
            <span>{children()}</span>
        </button>
    }
}

#[component]
pub fn TabsPanel(
    #[prop(into)] value: String,
    #[prop(optional)] padding: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();
    let active = use_context::<RwSignal<String>>().unwrap();

    let value_clone = value.clone();
    let is_active = move || active.get() == value_clone;

    let panel_styles = move || {
        let theme_val = theme.get();
        let mut builder = StyleBuilder::new();

        if !is_active() {
            builder.add("display", "none");
        }

        if let Some(p) = padding.as_ref() {
            builder.add("padding", p);
        } else {
            builder.add("padding", theme_val.spacing.md);
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-tabs-panel {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=panel_styles>
            {children()}
        </div>
    }
}
