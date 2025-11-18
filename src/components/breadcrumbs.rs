use crate::theme::use_theme;
use leptos::prelude::*;

#[derive(Clone, Debug)]
pub struct BreadcrumbItem {
    pub label: String,
    pub href: Option<String>,
}

impl BreadcrumbItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: None,
        }
    }

    pub fn href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }
}

#[component]
pub fn Breadcrumbs(
    #[prop(into)] items: Vec<BreadcrumbItem>,
    #[prop(optional)] separator: Option<String>,
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let separator = separator.unwrap_or_else(|| "/".to_string());

    let breadcrumbs_styles =
        "display: flex; align-items: center; gap: 0.5rem; flex-wrap: wrap;".to_string();

    let item_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "font-size: {}; \
             color: {}; \
             text-decoration: none; \
             transition: color 0.15s ease;",
            theme_val.typography.font_sizes.sm, scheme_colors.text
        )
    };

    let link_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let link_color = scheme_colors
            .get_color("blue", 6)
            .unwrap_or_else(|| "#228be6".to_string());
        format!(
            "font-size: {}; \
             color: {}; \
             text-decoration: none; \
             cursor: pointer; \
             transition: color 0.15s ease;",
            theme_val.typography.font_sizes.sm, link_color
        )
    };

    let separator_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let text_secondary = scheme_colors
            .get_color("gray", 6)
            .unwrap_or_else(|| "#868e96".to_string());
        format!(
            "font-size: {}; \
             color: {}; \
             user-select: none;",
            theme_val.typography.font_sizes.sm, text_secondary
        )
    };

    let class_str = format!("mingot-breadcrumbs {}", class.unwrap_or_default());
    let items_len = items.len();

    view! {
        <nav
            class=class_str
            style=move || {
                if let Some(s) = style.as_ref() {
                    format!("{}; {}", breadcrumbs_styles, s)
                } else {
                    breadcrumbs_styles.clone()
                }
            }

            aria-label="breadcrumb"
        >
            {items
                .into_iter()
                .enumerate()
                .map(|(index, item)| {
                    let is_last = index == items_len - 1;
                    let separator_clone = separator.clone();
                    view! {
                        <>
                            {if let Some(href) = item.href {
                                view! {
                                    <a href=href style=link_styles>
                                        {item.label}
                                    </a>
                                }
                                    .into_any()
                            } else {
                                view! { <span style=item_styles>{item.label}</span> }.into_any()
                            }}

                            {if !is_last {
                                view! {
                                    <span class="mingot-breadcrumb-separator" style=separator_styles>
                                        {separator_clone}
                                    </span>
                                }
                                    .into_any()
                            } else {
                                ().into_any()
                            }}

                        </>
                    }
                })
                .collect::<Vec<_>>()}

        </nav>
    }
}
