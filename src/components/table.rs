use crate::components::{Button, ButtonVariant};
use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;
use std::sync::Arc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SortDirection {
    Ascending,
    Descending,
    None,
}

impl SortDirection {
    pub fn toggle(&self) -> Self {
        match self {
            SortDirection::None => SortDirection::Ascending,
            SortDirection::Ascending => SortDirection::Descending,
            SortDirection::Descending => SortDirection::Ascending,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            SortDirection::Ascending => "↑",
            SortDirection::Descending => "↓",
            SortDirection::None => "↕",
        }
    }
}

/// Column definition for table
pub struct TableColumn<T>
where
    T: Clone + 'static,
{
    pub key: String,
    pub header: String,
    pub render: Arc<dyn Fn(&T) -> AnyView + Send + Sync>,
    pub sortable: bool,
    pub width: Option<String>,
}

impl<T: Clone + 'static> Clone for TableColumn<T> {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            header: self.header.clone(),
            render: Arc::clone(&self.render),
            sortable: self.sortable,
            width: self.width.clone(),
        }
    }
}

impl<T: Clone + 'static> TableColumn<T> {
    pub fn new<IV>(
        key: impl Into<String>,
        header: impl Into<String>,
        render: impl Fn(&T) -> IV + Send + Sync + 'static,
    ) -> Self
    where
        IV: IntoView + 'static,
    {
        let render_fn = move |item: &T| render(item).into_any();
        Self {
            key: key.into(),
            header: header.into(),
            render: Arc::new(render_fn),
            sortable: false,
            width: None,
        }
    }

    pub fn sortable(mut self, sortable: bool) -> Self {
        self.sortable = sortable;
        self
    }

    pub fn width(mut self, width: impl Into<String>) -> Self {
        self.width = Some(width.into());
        self
    }
}

#[component]
pub fn Table<T>(
    #[prop(into)] columns: Vec<TableColumn<T>>,
    #[prop(into)] data: Signal<Vec<T>>,
    #[prop(optional)] striped: bool,
    #[prop(optional)] highlight_on_hover: bool,
    #[prop(optional)] with_border: bool,
    #[prop(optional)] with_column_borders: bool,
    #[prop(optional)] sort_column: Option<RwSignal<Option<String>>>,
    #[prop(optional)] sort_direction: Option<RwSignal<SortDirection>>,
    #[prop(optional)] on_sort: Option<Callback<(String, SortDirection)>>,
    #[prop(optional, into)] empty_message: Option<String>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
{
    let theme = use_theme();

    let current_sort_column = sort_column.unwrap_or_else(|| RwSignal::new(None));
    let current_sort_direction =
        sort_direction.unwrap_or_else(|| RwSignal::new(SortDirection::None));

    let handle_header_click = move |column_key: String, sortable: bool| {
        if !sortable {
            return;
        }

        let new_direction = if current_sort_column.get().as_ref() == Some(&column_key) {
            current_sort_direction.get().toggle()
        } else {
            SortDirection::Ascending
        };

        current_sort_column.set(Some(column_key.clone()));
        current_sort_direction.set(new_direction);

        if let Some(callback) = on_sort {
            callback.run((column_key, new_direction));
        }
    };

    let table_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        builder
            .add("width", "100%")
            .add("border-collapse", "collapse")
            .add("font-family", theme_val.typography.font_family)
            .add("font-size", theme_val.typography.font_sizes.sm)
            .add("color", scheme_colors.text.clone());

        if with_border {
            builder.add(
                "border",
                format!("1px solid {}", scheme_colors.border.clone()),
            );
        }

        builder.build()
    };

    let th_styles = move |sortable: bool| {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        let bg_color = scheme_colors
            .get_color("gray", 1)
            .unwrap_or_else(|| "#f1f3f5".to_string());

        builder
            .add(
                "padding",
                format!("{} {}", theme_val.spacing.sm, theme_val.spacing.md),
            )
            .add("text-align", "left")
            .add(
                "font-weight",
                theme_val.typography.font_weights.semibold.to_string(),
            )
            .add("background-color", bg_color)
            .add("user-select", "none");

        if with_column_borders {
            builder.add(
                "border",
                format!("1px solid {}", scheme_colors.border.clone()),
            );
        } else {
            builder.add(
                "border-bottom",
                format!("2px solid {}", scheme_colors.border.clone()),
            );
        }

        if sortable {
            builder
                .add("cursor", "pointer")
                .add("transition", "background-color 0.15s ease");
        }

        builder.build()
    };

    let td_styles = move |row_index: usize| {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        builder.add(
            "padding",
            format!("{} {}", theme_val.spacing.sm, theme_val.spacing.md),
        );

        if striped && row_index % 2 == 1 {
            let stripe_color = scheme_colors
                .get_color("gray", 0)
                .unwrap_or_else(|| "#f8f9fa".to_string());
            builder.add("background-color", stripe_color);
        }

        if with_column_borders {
            builder.add(
                "border",
                format!("1px solid {}", scheme_colors.border.clone()),
            );
        } else {
            builder.add(
                "border-bottom",
                format!("1px solid {}", scheme_colors.border.clone()),
            );
        }

        builder.build()
    };

    let tr_hover_styles = move || {
        if !highlight_on_hover {
            return String::new();
        }

        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let hover_color = scheme_colors
            .get_color("gray", 1)
            .unwrap_or_else(|| "#f1f3f5".to_string());

        format!("tr:hover td {{ background-color: {}; }}", hover_color)
    };

    let th_hover_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let hover_color = scheme_colors
            .get_color("gray", 2)
            .unwrap_or_else(|| "#e9ecef".to_string());

        format!("th.sortable:hover {{ background-color: {}; }}", hover_color)
    };

    view! {
        <div class="mingot-table-wrapper">
            <style>
                {tr_hover_styles}
                {th_hover_styles}
            </style>
            <table class="mingot-table" style=table_styles>
                <thead>
                    <tr>
                        {columns.iter().map(|col| {
                            let key = col.key.clone();
                            let key_for_sort = key.clone();
                            let header = col.header.clone();
                            let sortable = col.sortable;
                            let width = col.width.clone();

                            let is_current_sort = move || {
                                current_sort_column.get().as_ref() == Some(&key_for_sort)
                            };

                            let sort_icon = move || {
                                if !sortable {
                                    return String::new();
                                }
                                if is_current_sort() {
                                    format!(" {}", current_sort_direction.get().icon())
                                } else {
                                    " ↕".to_string()
                                }
                            };

                            let key_clone = key.clone();
                            view! {
                                <th
                                    class=move || if sortable { "sortable" } else { "" }
                                    style=move || {
                                        let mut style = th_styles(sortable);
                                        if let Some(w) = &width {
                                            style = format!("{}; width: {}", style, w);
                                        }
                                        style
                                    }
                                    on:click=move |_| handle_header_click(key_clone.clone(), sortable)
                                >
                                    {header}
                                    <span style="opacity: 0.5; margin-left: 0.25rem;">
                                        {sort_icon}
                                    </span>
                                </th>
                            }
                        }).collect::<Vec<_>>()}
                    </tr>
                </thead>
                <tbody>
                    <For
                        each=move || {
                            let rows = data.get();
                            if rows.is_empty() {
                                vec![]
                            } else {
                                rows.into_iter().enumerate().collect::<Vec<_>>()
                            }
                        }
                        key=|(i, _)| *i
                        children={
                            let columns_clone = columns.clone();
                            move |(row_index, row)| {
                                view! {
                                    <tr>
                                        {columns_clone.iter().map(|col| {
                                            let cell_content = (col.render)(&row);
                                            view! {
                                                <td style=td_styles(row_index)>
                                                    {cell_content}
                                                </td>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </tr>
                                }
                            }
                        }
                    />
                    {
                        let columns_len = columns.len();
                        move || {
                            if data.get().is_empty() {
                                let message = empty_message.clone().unwrap_or_else(|| "No data available".to_string());
                                view! {
                                    <tr>
                                        <td colspan=columns_len style="text-align: center; padding: 2rem; opacity: 0.6;">
                                            {message}
                                        </td>
                                    </tr>
                                }.into_any()
                            } else {
                                ().into_any()
                            }
                        }
                    }
                </tbody>
            </table>
        </div>
    }
}

#[component]
pub fn Pagination(
    #[prop(into)] current_page: Signal<usize>,
    #[prop(into)] total_pages: Signal<usize>,
    #[prop(into)] on_page_change: Callback<usize>,
    #[prop(optional)] show_edges: bool,
    #[prop(optional)] siblings: Option<usize>,
) -> impl IntoView {
    let theme = use_theme();
    let siblings = siblings.unwrap_or(1);

    let handle_page_change = move |page: usize| {
        on_page_change.run(page);
    };

    let pagination_styles = move || {
        "display: flex; align-items: center; gap: 0.5rem; justify-content: center; margin-top: 1rem;"
            .to_string()
    };

    let page_button_styles = move |is_active: bool| {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();

        builder
            .add("min-width", "2.5rem")
            .add("height", "2.5rem")
            .add("padding", "0 0.75rem")
            .add("display", "inline-flex")
            .add("align-items", "center")
            .add("justify-content", "center")
            .add("border-radius", theme_val.radius.sm)
            .add("font-size", theme_val.typography.font_sizes.sm)
            .add(
                "font-weight",
                theme_val.typography.font_weights.medium.to_string(),
            )
            .add("cursor", "pointer")
            .add("transition", "all 0.15s ease")
            .add("border", "1px solid transparent")
            .add("user-select", "none");

        if is_active {
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

        builder.build()
    };

    let generate_page_numbers = move || {
        let current = current_page.get();
        let total = total_pages.get();
        let mut pages = Vec::new();

        if total <= 7 {
            // Show all pages if total is small
            for i in 1..=total {
                pages.push(Some(i));
            }
        } else {
            // Always show first page
            if show_edges {
                pages.push(Some(1));
            }

            // Calculate range around current page
            let start = (current.saturating_sub(siblings)).max(if show_edges { 2 } else { 1 });
            let end = (current + siblings).min(if show_edges { total - 1 } else { total });

            // Add ellipsis after first page if needed
            if show_edges && start > 2 {
                pages.push(None);
            }

            // Add pages around current
            for i in start..=end {
                pages.push(Some(i));
            }

            // Add ellipsis before last page if needed
            if show_edges && end < total - 1 {
                pages.push(None);
            }

            // Always show last page
            if show_edges {
                pages.push(Some(total));
            }
        }

        pages
    };

    view! {
        <div class="mingot-pagination" style=pagination_styles>
            <Button
                variant=ButtonVariant::Subtle
                disabled={current_page.get() <= 1}
                on_click=Callback::new(move |_| {
                    let new_page = current_page.get().saturating_sub(1);
                    if new_page >= 1 {
                        handle_page_change(new_page);
                    }
                })
            >
                "‹"
            </Button>

            {move || generate_page_numbers().into_iter().map(|page_opt| {
                if let Some(page) = page_opt {
                    let is_active = move || page == current_page.get();
                    view! {
                        <button
                            class="mingot-pagination-page"
                            style=move || page_button_styles(is_active())
                            on:click=move |_| handle_page_change(page)
                        >
                            {page.to_string()}
                        </button>
                    }.into_any()
                } else {
                    view! {
                        <span style="padding: 0 0.25rem; opacity: 0.5;">"..."</span>
                    }.into_any()
                }
            }).collect::<Vec<_>>()}

            <Button
                variant=ButtonVariant::Subtle
                disabled={current_page.get() >= total_pages.get()}
                on_click=Callback::new(move |_| {
                    let new_page = current_page.get() + 1;
                    if new_page <= total_pages.get() {
                        handle_page_change(new_page);
                    }
                })
            >
                "›"
            </Button>
        </div>
    }
}

/// Helper component to combine Table with Pagination
#[component]
pub fn TableWithPagination<T>(
    #[prop(into)] columns: Vec<TableColumn<T>>,
    #[prop(into)] data: Signal<Vec<T>>,
    #[prop(into)] current_page: RwSignal<usize>,
    #[prop(into)] page_size: Signal<usize>,
    #[prop(optional)] striped: bool,
    #[prop(optional)] highlight_on_hover: bool,
    #[prop(optional)] with_border: bool,
    #[prop(optional)] with_column_borders: bool,
    #[prop(optional)] sort_column: Option<RwSignal<Option<String>>>,
    #[prop(optional)] sort_direction: Option<RwSignal<SortDirection>>,
    #[prop(optional)] on_sort: Option<Callback<(String, SortDirection)>>,
    #[prop(optional, into)] empty_message: Option<String>,
) -> impl IntoView
where
    T: Clone + Send + Sync + 'static,
{
    let total_items = Signal::derive(move || data.get().len());
    let total_pages = Signal::derive(move || {
        let total = total_items.get();
        let size = page_size.get();
        if size == 0 {
            1
        } else {
            total.div_ceil(size)
        }
    });

    let paginated_data = Signal::derive(move || {
        let all_data = data.get();
        let page = current_page.get();
        let size = page_size.get();

        if size == 0 {
            return all_data;
        }

        let start = (page - 1) * size;
        let end = (start + size).min(all_data.len());

        if start >= all_data.len() {
            Vec::new()
        } else {
            all_data[start..end].to_vec()
        }
    });

    let table_view = match (sort_column, sort_direction, on_sort, empty_message) {
        (Some(sc), Some(sd), Some(os), Some(em)) => view! {
            <Table
                columns=columns.clone()
                data=paginated_data
                striped=striped
                highlight_on_hover=highlight_on_hover
                with_border=with_border
                with_column_borders=with_column_borders
                sort_column=sc
                sort_direction=sd
                on_sort=os
                empty_message=em
            />
        }
        .into_any(),
        (Some(sc), Some(sd), Some(os), None) => view! {
            <Table
                columns=columns.clone()
                data=paginated_data
                striped=striped
                highlight_on_hover=highlight_on_hover
                with_border=with_border
                with_column_borders=with_column_borders
                sort_column=sc
                sort_direction=sd
                on_sort=os
            />
        }
        .into_any(),
        (_, _, _, Some(em)) => view! {
            <Table
                columns=columns.clone()
                data=paginated_data
                striped=striped
                highlight_on_hover=highlight_on_hover
                with_border=with_border
                with_column_borders=with_column_borders
                empty_message=em
            />
        }
        .into_any(),
        _ => view! {
            <Table
                columns=columns.clone()
                data=paginated_data
                striped=striped
                highlight_on_hover=highlight_on_hover
                with_border=with_border
                with_column_borders=with_column_borders
            />
        }
        .into_any(),
    };

    view! {
        <div class="mingot-table-with-pagination">
            {table_view}

            {move || {
                if total_items.get() > 0 {
                    view! {
                        <Pagination
                            current_page=Signal::from(current_page)
                            total_pages=total_pages
                            on_page_change=Callback::new(move |page: usize| {
                                current_page.set(page);
                            })
                        />
                    }.into_any()
                } else {
                    ().into_any()
                }
            }}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_direction_toggle() {
        assert_eq!(SortDirection::None.toggle(), SortDirection::Ascending);
        assert_eq!(SortDirection::Ascending.toggle(), SortDirection::Descending);
        assert_eq!(SortDirection::Descending.toggle(), SortDirection::Ascending);
    }

    #[test]
    fn test_sort_direction_icon() {
        assert_eq!(SortDirection::None.icon(), "↕");
        assert_eq!(SortDirection::Ascending.icon(), "↑");
        assert_eq!(SortDirection::Descending.icon(), "↓");
    }

    #[test]
    fn test_sort_direction_toggle_cycle() {
        let mut dir = SortDirection::None;
        dir = dir.toggle();
        assert_eq!(dir, SortDirection::Ascending);
        dir = dir.toggle();
        assert_eq!(dir, SortDirection::Descending);
        dir = dir.toggle();
        assert_eq!(dir, SortDirection::Ascending);
    }

    #[derive(Clone)]
    struct TestData {
        id: i32,
        name: String,
    }

    #[test]
    fn test_table_column_builder() {
        let column = TableColumn::new("id", "ID", |item: &TestData| {
            view! { <span>{item.id}</span> }
        })
        .sortable(true)
        .width("100px");

        assert_eq!(column.key, "id");
        assert_eq!(column.header, "ID");
        assert!(column.sortable);
        assert_eq!(column.width, Some("100px".to_string()));
    }

    #[test]
    fn test_table_column_default_sortable() {
        let column = TableColumn::new("name", "Name", |item: &TestData| {
            view! { <span>{item.name.clone()}</span> }
        });

        assert!(!column.sortable);
        assert_eq!(column.width, None);
    }

    #[test]
    fn test_table_column_clone() {
        let column1 = TableColumn::new("id", "ID", |item: &TestData| {
            view! { <span>{item.id}</span> }
        })
        .sortable(true)
        .width("50px");

        let column2 = column1.clone();

        assert_eq!(column1.key, column2.key);
        assert_eq!(column1.header, column2.header);
        assert_eq!(column1.sortable, column2.sortable);
        assert_eq!(column1.width, column2.width);
    }
}
