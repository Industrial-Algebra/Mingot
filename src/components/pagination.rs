use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

/// Size variants for the Pagination component
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum PaginationSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

/// A pagination component for navigating through pages of content.
///
/// # Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use mingot::prelude::*;
///
/// let page = RwSignal::new(1_usize);
///
/// view! {
///     <Pagination
///         total=10
///         value=page
///         on_change=Callback::new(move |p| page.set(p))
///     />
/// }
/// ```
#[component]
pub fn Pagination(
    /// Total number of pages
    total: usize,
    /// Current page (1-indexed)
    #[prop(into)]
    value: Signal<usize>,
    /// Number of siblings on each side of current page
    #[prop(default = 1)]
    siblings: usize,
    /// Number of elements at the start and end
    #[prop(default = 1)]
    boundaries: usize,
    /// Size of the pagination buttons
    #[prop(optional)]
    size: Option<PaginationSize>,
    /// Whether to show first/last page buttons
    #[prop(default = false)]
    with_edges: bool,
    /// Whether to show previous/next buttons
    #[prop(default = true)]
    with_controls: bool,
    /// Gap between buttons
    #[prop(default = "0.25rem".to_string(), into)]
    gap: String,
    /// Disabled state
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Callback when page changes
    #[prop(optional)]
    on_change: Option<Callback<usize>>,
    /// Additional CSS class
    #[prop(optional, into)]
    class: Option<String>,
    /// Additional inline styles
    #[prop(optional, into)]
    style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let size = size.unwrap_or_default();

    // Calculate which page buttons to show
    // Returns Option<usize> where None represents ellipsis dots
    let pages = move || -> Vec<Option<usize>> {
        let current = value.get();
        let total_pages = total.max(1);

        // Calculate total visible items
        let total_numbers = siblings * 2 + 3 + boundaries * 2;

        // If we can show all pages, just show them
        if total_numbers >= total_pages {
            return (1..=total_pages).map(Some).collect();
        }

        let left_sibling = (current.saturating_sub(siblings)).max(boundaries + 1);
        let right_sibling = (current + siblings).min(total_pages.saturating_sub(boundaries));

        let show_left_dots = left_sibling > boundaries + 2;
        let show_right_dots = right_sibling < total_pages.saturating_sub(boundaries + 1);

        let mut result: Vec<Option<usize>> = Vec::new();

        // Add boundary pages at the start
        for i in 1..=boundaries.min(total_pages) {
            result.push(Some(i));
        }

        // Add left dots if needed
        if show_left_dots {
            result.push(None); // Dots indicator
        } else {
            // Add pages between boundaries and left sibling
            for i in (boundaries + 1)..left_sibling {
                if i <= total_pages {
                    result.push(Some(i));
                }
            }
        }

        // Add sibling pages including current
        for i in left_sibling..=right_sibling {
            if i > boundaries && i <= total_pages.saturating_sub(boundaries) {
                result.push(Some(i));
            }
        }

        // Add right dots if needed
        if show_right_dots {
            result.push(None); // Dots indicator
        } else {
            // Add pages between right sibling and end boundaries
            for i in (right_sibling + 1)..=(total_pages.saturating_sub(boundaries)) {
                result.push(Some(i));
            }
        }

        // Add boundary pages at the end
        for i in (total_pages.saturating_sub(boundaries) + 1)..=total_pages {
            if i > boundaries && !result.contains(&Some(i)) {
                result.push(Some(i));
            }
        }

        result
    };

    let (button_size, font_size) = match size {
        PaginationSize::Xs => ("1.5rem", "xs"),
        PaginationSize::Sm => ("1.875rem", "sm"),
        PaginationSize::Md => ("2.25rem", "sm"),
        PaginationSize::Lg => ("2.625rem", "md"),
        PaginationSize::Xl => ("3rem", "lg"),
    };

    let make_button_styles = move |is_active: bool, _is_control: bool| {
        move || {
            let theme_val = theme.get();
            let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
            let is_disabled = disabled.get();

            let mut builder = StyleBuilder::new();
            builder
                .add("display", "flex")
                .add("align-items", "center")
                .add("justify-content", "center")
                .add("min-width", button_size)
                .add("height", button_size)
                .add("padding", "0 0.5rem")
                .add("border-radius", theme_val.radius.sm)
                .add("font-family", theme_val.typography.font_family)
                .add(
                    "font-size",
                    match font_size {
                        "xs" => theme_val.typography.font_sizes.xs,
                        "sm" => theme_val.typography.font_sizes.sm,
                        "md" => theme_val.typography.font_sizes.md,
                        "lg" => theme_val.typography.font_sizes.lg,
                        _ => theme_val.typography.font_sizes.sm,
                    },
                )
                .add("border", "none")
                .add(
                    "cursor",
                    if is_disabled {
                        "not-allowed"
                    } else {
                        "pointer"
                    },
                )
                .add("transition", "all 0.15s ease")
                .add("user-select", "none");

            if is_active {
                builder
                    .add(
                        "background-color",
                        scheme_colors
                            .get_color("blue", 6)
                            .unwrap_or_else(|| "#228be6".to_string()),
                    )
                    .add("color", scheme_colors.white.clone());
            } else {
                // Both control and non-control inactive buttons have the same style
                builder
                    .add("background-color", "transparent")
                    .add("color", scheme_colors.text.clone());
            }

            if is_disabled {
                builder.add("opacity", "0.5");
            }

            builder.build()
        }
    };

    let dots_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "display: flex; align-items: center; justify-content: center; min-width: {}; height: {}; color: {};",
            button_size,
            button_size,
            scheme_colors.get_color("gray", 6).unwrap_or_else(|| "#868e96".to_string())
        )
    };

    let handle_page_change = move |page: usize| {
        if disabled.get() || page < 1 || page > total {
            return;
        }
        if let Some(callback) = on_change {
            callback.run(page);
        }
    };

    let wrapper_styles = move || {
        let mut styles = format!("display: flex; align-items: center; gap: {};", gap);
        if let Some(ref s) = style {
            styles.push_str(s);
        }
        styles
    };

    let class_str = format!("mingot-pagination {}", class.unwrap_or_default());

    view! {
        <nav class=class_str style=wrapper_styles aria-label="Pagination">
            // First page button
            {with_edges.then(|| {
                let button_styles = make_button_styles(false, true);
                view! {
                    <button
                        type="button"
                        style=button_styles
                        disabled=move || disabled.get() || value.get() <= 1
                        on:click=move |_| handle_page_change(1)
                        aria-label="First page"
                    >
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <polyline points="11 17 6 12 11 7"></polyline>
                            <polyline points="18 17 13 12 18 7"></polyline>
                        </svg>
                    </button>
                }
            })}

            // Previous button
            {with_controls.then(|| {
                let button_styles = make_button_styles(false, true);
                view! {
                    <button
                        type="button"
                        style=button_styles
                        disabled=move || disabled.get() || value.get() <= 1
                        on:click=move |_| handle_page_change(value.get().saturating_sub(1))
                        aria-label="Previous page"
                    >
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <polyline points="15 18 9 12 15 6"></polyline>
                        </svg>
                    </button>
                }
            })}

            // Page buttons
            {move || {
                pages().into_iter().map(|page_opt| {
                    match page_opt {
                        Some(page) => {
                            let is_active = value.get() == page;
                            let button_styles = make_button_styles(is_active, false);
                            view! {
                                <button
                                    type="button"
                                    style=button_styles
                                    disabled=move || disabled.get()
                                    on:click=move |_| handle_page_change(page)
                                    aria-current=if is_active { Some("page") } else { None }
                                >
                                    {page}
                                </button>
                            }.into_any()
                        }
                        None => {
                            view! {
                                <span style=dots_styles>"..."</span>
                            }.into_any()
                        }
                    }
                }).collect::<Vec<_>>()
            }}

            // Next button
            {with_controls.then(|| {
                let button_styles = make_button_styles(false, true);
                view! {
                    <button
                        type="button"
                        style=button_styles
                        disabled=move || disabled.get() || (value.get() >= total)
                        on:click=move |_| handle_page_change(value.get() + 1)
                        aria-label="Next page"
                    >
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <polyline points="9 18 15 12 9 6"></polyline>
                        </svg>
                    </button>
                }
            })}

            // Last page button
            {with_edges.then(|| {
                let button_styles = make_button_styles(false, true);
                view! {
                    <button
                        type="button"
                        style=button_styles
                        disabled=move || disabled.get() || (value.get() >= total)
                        on:click=move |_| handle_page_change(total)
                        aria-label="Last page"
                    >
                        <svg width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                            <polyline points="13 17 18 12 13 7"></polyline>
                            <polyline points="6 17 11 12 6 7"></polyline>
                        </svg>
                    </button>
                }
            })}
        </nav>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pagination_size_default() {
        assert_eq!(PaginationSize::default(), PaginationSize::Md);
    }

    #[test]
    fn test_pagination_size_variants() {
        let sizes = [
            PaginationSize::Xs,
            PaginationSize::Sm,
            PaginationSize::Md,
            PaginationSize::Lg,
            PaginationSize::Xl,
        ];
        for (i, s1) in sizes.iter().enumerate() {
            for (j, s2) in sizes.iter().enumerate() {
                if i != j {
                    assert_ne!(s1, s2);
                }
            }
        }
    }
}
