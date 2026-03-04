use crate::components::number_input::{NumberInputPrecision, ParseError};
use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::JsCast;

/// Type of parameter value
#[derive(Clone, Debug, PartialEq)]
pub enum ParameterValue {
    /// String value
    String(String),
    /// Numeric value with precision
    Number {
        value: String,
        min: Option<String>,
        max: Option<String>,
        step: Option<String>,
        precision: NumberInputPrecision,
    },
    /// Boolean value
    Bool(bool),
    /// Color value (hex string)
    Color(String),
    /// Enum value with options
    Enum { value: String, options: Vec<String> },
    /// Group of child parameters
    Group,
    /// Action button
    Action { label: String },
    /// Read-only text
    ReadOnly(String),
}

impl Default for ParameterValue {
    fn default() -> Self {
        Self::String(String::new())
    }
}

/// A node in the parameter tree
#[derive(Clone, Debug, PartialEq)]
pub struct ParameterNode {
    /// Unique path/key for the parameter
    pub key: String,
    /// Display name
    pub name: String,
    /// Value and type
    pub value: ParameterValue,
    /// Child nodes (for groups)
    pub children: Vec<ParameterNode>,
    /// Tooltip/description
    pub tooltip: Option<String>,
    /// Whether the node is expanded (for groups)
    pub expanded: bool,
    /// Whether the node is visible (for filtering)
    pub visible: bool,
    /// Whether the node is enabled
    pub enabled: bool,
}

impl ParameterNode {
    /// Create a new parameter node
    pub fn new(key: impl Into<String>, name: impl Into<String>, value: ParameterValue) -> Self {
        Self {
            key: key.into(),
            name: name.into(),
            value,
            children: Vec::new(),
            tooltip: None,
            expanded: true,
            visible: true,
            enabled: true,
        }
    }

    /// Create a group node
    pub fn group(key: impl Into<String>, name: impl Into<String>) -> Self {
        Self::new(key, name, ParameterValue::Group)
    }

    /// Create a string parameter
    pub fn string(
        key: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self::new(key, name, ParameterValue::String(value.into()))
    }

    /// Create a number parameter
    pub fn number(
        key: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self::new(
            key,
            name,
            ParameterValue::Number {
                value: value.into(),
                min: None,
                max: None,
                step: None,
                precision: NumberInputPrecision::Decimal(6),
            },
        )
    }

    /// Create a boolean parameter
    pub fn bool(key: impl Into<String>, name: impl Into<String>, value: bool) -> Self {
        Self::new(key, name, ParameterValue::Bool(value))
    }

    /// Create a color parameter
    pub fn color(
        key: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self::new(key, name, ParameterValue::Color(value.into()))
    }

    /// Create an enum parameter
    pub fn enumeration(
        key: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
        options: Vec<String>,
    ) -> Self {
        Self::new(
            key,
            name,
            ParameterValue::Enum {
                value: value.into(),
                options,
            },
        )
    }

    /// Create an action button
    pub fn action(
        key: impl Into<String>,
        name: impl Into<String>,
        label: impl Into<String>,
    ) -> Self {
        Self::new(
            key,
            name,
            ParameterValue::Action {
                label: label.into(),
            },
        )
    }

    /// Create a read-only text node
    pub fn read_only(
        key: impl Into<String>,
        name: impl Into<String>,
        value: impl Into<String>,
    ) -> Self {
        Self::new(key, name, ParameterValue::ReadOnly(value.into()))
    }

    /// Add a child node
    pub fn with_child(mut self, child: ParameterNode) -> Self {
        self.children.push(child);
        self
    }

    /// Add multiple children
    pub fn with_children(mut self, children: Vec<ParameterNode>) -> Self {
        self.children.extend(children);
        self
    }

    /// Set tooltip
    pub fn with_tooltip(mut self, tooltip: impl Into<String>) -> Self {
        self.tooltip = Some(tooltip.into());
        self
    }

    /// Set collapsed state
    pub fn collapsed(mut self) -> Self {
        self.expanded = false;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    /// Set number constraints
    pub fn with_range(mut self, min_val: impl Into<String>, max_val: impl Into<String>) -> Self {
        if let ParameterValue::Number {
            min: ref mut min_opt,
            max: ref mut max_opt,
            ..
        } = self.value
        {
            *min_opt = Some(min_val.into());
            *max_opt = Some(max_val.into());
        }
        self
    }

    /// Set number step
    pub fn with_step(mut self, step: impl Into<String>) -> Self {
        if let ParameterValue::Number {
            step: ref mut step_opt,
            ..
        } = self.value
        {
            *step_opt = Some(step.into());
        }
        self
    }

    /// Set number precision
    pub fn with_precision(mut self, precision: NumberInputPrecision) -> Self {
        if let ParameterValue::Number {
            precision: ref mut prec,
            ..
        } = self.value
        {
            *prec = precision;
        }
        self
    }
}

/// Size variants for the ParameterTree
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ParameterTreeSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
}

/// Configuration for ParameterTree
#[derive(Clone, Debug, PartialEq, Default)]
pub struct ParameterTreeConfig {
    pub name_column_width: Option<String>,
    pub value_column_width: Option<String>,
    pub indent_size: Option<String>,
    pub show_header: bool,
    pub show_search: bool,
    pub searchable: bool,
}

impl ParameterTreeConfig {
    pub fn new() -> Self {
        Self {
            name_column_width: Some("50%".to_string()),
            value_column_width: Some("50%".to_string()),
            indent_size: Some("1.5rem".to_string()),
            show_header: true,
            show_search: true,
            searchable: true,
        }
    }
}

/// A hierarchical parameter tree editor (PyQtGraph-style).
///
/// Provides a tree-based interface for editing parameters with support for
/// multiple value types, grouping, search/filter, and save/load functionality.
///
/// # Features
/// - Hierarchical parameter organization
/// - Type-aware editors (string, number, bool, color, enum)
/// - Expand/collapse groups
/// - Search/filter parameters
/// - Save/load configurations
/// - Tooltips for descriptions
///
/// # Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use mingot::prelude::*;
///
/// let root = ParameterNode::group("root", "Parameters")
///     .with_child(
///         ParameterNode::group("wave", "Wave Settings")
///             .with_child(ParameterNode::number("amplitude", "Amplitude", "1.0")
///                 .with_range("0", "10")
///                 .with_step("0.1"))
///             .with_child(ParameterNode::number("frequency", "Frequency", "440")
///                 .with_range("20", "20000")
///                 .with_step("1"))
///     )
///     .with_child(
///         ParameterNode::group("display", "Display")
///             .with_child(ParameterNode::bool("show_grid", "Show Grid", true))
///             .with_child(ParameterNode::color("background", "Background", "#1a1a2e"))
///     );
///
/// view! {
///     <ParameterTree
///         root=root
///         on_change=Callback::new(move |path, value| {
///             // Handle parameter change
///         })
///     />
/// }
/// ```
#[component]
pub fn ParameterTree(
    /// Root parameter node
    #[prop(into)]
    root: Signal<ParameterNode>,
    /// Size variant
    #[prop(optional)]
    size: Option<ParameterTreeSize>,
    /// Configuration options
    #[prop(optional)]
    config: Option<ParameterTreeConfig>,
    /// Disabled state
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Callback when a value changes (path, new_value)
    #[prop(optional)]
    on_change: Option<Callback<(String, String)>>,
    /// Callback when a node is expanded/collapsed
    #[prop(optional)]
    on_toggle: Option<Callback<(String, bool)>>,
    /// Callback when an action is triggered
    #[prop(optional)]
    on_action: Option<Callback<String>>,
    /// Callback for validation
    #[prop(optional)]
    on_validate: Option<Callback<(String, Result<String, ParseError>)>>,
    /// Additional CSS class
    #[prop(optional, into)]
    class: Option<String>,
    /// Additional inline styles
    #[prop(optional, into)]
    style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let size = size.unwrap_or_default();
    let config = config.unwrap_or_default();

    let search_query = RwSignal::new(String::new());
    let expanded_nodes = RwSignal::new(std::collections::HashSet::<String>::new());

    // Initialize expanded state from root
    Effect::new(move |_| {
        let root_node = root.get();
        let mut expanded = std::collections::HashSet::new();
        collect_expanded_keys(&root_node, &mut expanded);
        expanded_nodes.set(expanded);
    });

    // Font sizes based on size variant
    let (font_size, row_height, input_padding) = match size {
        ParameterTreeSize::Xs => ("0.75rem", "1.5rem", "0.125rem 0.25rem"),
        ParameterTreeSize::Sm => ("0.8125rem", "1.75rem", "0.1875rem 0.375rem"),
        ParameterTreeSize::Md => ("0.875rem", "2rem", "0.25rem 0.5rem"),
        ParameterTreeSize::Lg => ("1rem", "2.5rem", "0.375rem 0.625rem"),
    };

    let indent_size = config
        .indent_size
        .clone()
        .unwrap_or_else(|| "1.5rem".to_string());
    let name_width = config
        .name_column_width
        .clone()
        .unwrap_or_else(|| "50%".to_string());
    let value_width = config
        .value_column_width
        .clone()
        .unwrap_or_else(|| "50%".to_string());

    // Container styles
    let container_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        let mut builder = StyleBuilder::new();
        builder
            .add("display", "flex")
            .add("flex-direction", "column")
            .add("background-color", scheme_colors.background.clone())
            .add(
                "border",
                format!(
                    "1px solid {}",
                    scheme_colors
                        .get_color("gray", 3)
                        .unwrap_or_else(|| "#dee2e6".to_string())
                ),
            )
            .add("border-radius", theme_val.radius.md)
            .add("font-size", font_size)
            .add("overflow", "hidden");

        let mut result = builder.build();
        if let Some(ref s) = style {
            if !result.is_empty() {
                result.push_str("; ");
            }
            result.push_str(s);
        }
        result
    };

    let search_container_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        format!(
            "padding: 0.5rem; border-bottom: 1px solid {};",
            scheme_colors
                .get_color("gray", 3)
                .unwrap_or_else(|| "#dee2e6".to_string())
        )
    };

    let search_input_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        format!(
            "width: 100%; padding: {}; font-size: {}; border: 1px solid {}; border-radius: {}; background-color: {}; color: {}; outline: none;",
            input_padding,
            font_size,
            scheme_colors.get_color("gray", 4).unwrap_or_else(|| "#ced4da".to_string()),
            theme_val.radius.sm,
            scheme_colors.background,
            scheme_colors.text
        )
    };

    let header_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        format!(
            "display: flex; padding: 0.5rem; background-color: {}; border-bottom: 1px solid {}; font-weight: {};",
            scheme_colors.get_color("gray", 1).unwrap_or_else(|| "#f8f9fa".to_string()),
            scheme_colors.get_color("gray", 3).unwrap_or_else(|| "#dee2e6".to_string()),
            theme_val.typography.font_weights.semibold
        )
    };

    let tree_container_styles = "flex: 1; overflow-y: auto;";

    let handle_search = move |ev: leptos::ev::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        search_query.set(input.value());
    };

    let toggle_node = move |key: String| {
        let is_expanded = expanded_nodes.get().contains(&key);
        expanded_nodes.update(|nodes| {
            if is_expanded {
                nodes.remove(&key);
            } else {
                nodes.insert(key.clone());
            }
        });

        if let Some(callback) = on_toggle {
            callback.run((key, !is_expanded));
        }
    };

    let class_str = format!(
        "mingot-parameter-tree {}",
        class.clone().unwrap_or_default()
    );

    view! {
        <div class=class_str style=container_styles>
            // Search bar
            {config.show_search.then(|| {
                view! {
                    <div style=search_container_styles>
                        <input
                            type="text"
                            placeholder="Search parameters..."
                            style=search_input_styles
                            on:input=handle_search
                            prop:value=move || search_query.get()
                        />
                    </div>
                }
            })}

            // Header
            {config.show_header.then(|| {
                let name_w = name_width.clone();
                let value_w = value_width.clone();
                view! {
                    <div style=header_styles>
                        <div style=format!("width: {};", name_w)>"Name"</div>
                        <div style=format!("width: {};", value_w)>"Value"</div>
                    </div>
                }
            })}

            // Tree content
            <div style=tree_container_styles>
                <ParameterTreeNode
                    node=root
                    depth=0
                    path="".to_string()
                    search_query=search_query.into()
                    expanded_nodes=expanded_nodes.into()
                    toggle_node=Callback::new(toggle_node)
                    on_change=on_change
                    on_action=on_action
                    on_validate=on_validate
                    disabled=disabled
                    font_size=font_size.to_string()
                    row_height=row_height.to_string()
                    input_padding=input_padding.to_string()
                    indent_size=indent_size.clone()
                    name_width=name_width.clone()
                    value_width=value_width.clone()
                />
            </div>
        </div>
    }
}

/// Recursive tree node component
#[component]
fn ParameterTreeNode(
    node: Signal<ParameterNode>,
    depth: usize,
    path: String,
    search_query: Signal<String>,
    expanded_nodes: Signal<std::collections::HashSet<String>>,
    toggle_node: Callback<String>,
    on_change: Option<Callback<(String, String)>>,
    on_action: Option<Callback<String>>,
    on_validate: Option<Callback<(String, Result<String, ParseError>)>>,
    disabled: Signal<bool>,
    font_size: String,
    row_height: String,
    input_padding: String,
    indent_size: String,
    name_width: String,
    value_width: String,
) -> impl IntoView {
    let theme = use_theme();

    // Clone string parameters for use in multiple closures
    let font_size_for_name = font_size.clone();
    let font_size_for_input = font_size.clone();
    let font_size_for_select = font_size.clone();
    let font_size_for_button = font_size.clone();
    let font_size_for_readonly = font_size.clone();
    let row_height_for_row = row_height.clone();
    let row_height_for_children = row_height.clone();
    let input_padding_for_input = input_padding.clone();
    let input_padding_for_select = input_padding.clone();
    let input_padding_for_button = input_padding.clone();
    let indent_size_for_children = indent_size.clone();
    let name_width_for_children = name_width.clone();
    let value_width_for_children = value_width.clone();

    let node_key = Memo::new(move |_| node.get().key.clone());
    let node_name = Memo::new(move |_| node.get().name.clone());
    let node_value = Memo::new(move |_| node.get().value.clone());
    let node_children = Memo::new(move |_| node.get().children.clone());
    let node_enabled = Memo::new(move |_| node.get().enabled);
    let node_tooltip = Memo::new(move |_| node.get().tooltip.clone());

    let full_path = if path.is_empty() {
        node_key.get_untracked()
    } else {
        format!("{}.{}", path, node_key.get_untracked())
    };

    let is_expanded = Memo::new(move |_| expanded_nodes.get().contains(&node_key.get()));

    let is_visible = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        if query.is_empty() {
            return true;
        }
        node_name.get().to_lowercase().contains(&query)
            || node_key.get().to_lowercase().contains(&query)
    });

    let is_group = Memo::new(move |_| matches!(node_value.get(), ParameterValue::Group));

    let has_children = Memo::new(move |_| !node_children.get().is_empty());

    let row_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        format!(
            "display: flex; align-items: center; min-height: {}; padding: 0 0.5rem; border-bottom: 1px solid {}; background-color: {};",
            row_height_for_row,
            scheme_colors.get_color("gray", 2).unwrap_or_else(|| "#e9ecef".to_string()),
            if is_group.get() {
                scheme_colors.get_color("gray", 1).unwrap_or_else(|| "#f8f9fa".to_string())
            } else {
                scheme_colors.background.clone()
            }
        )
    };

    let name_cell_styles = move || {
        format!(
            "display: flex; align-items: center; width: {}; padding-left: calc({} * {});",
            name_width, indent_size, depth
        )
    };

    let value_cell_styles = move || format!("width: {}; padding: 0.25rem;", value_width);

    let expand_button_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        format!(
            "display: flex; align-items: center; justify-content: center; width: 1.25rem; height: 1.25rem; margin-right: 0.25rem; cursor: pointer; color: {}; user-select: none;",
            scheme_colors.get_color("gray", 6).unwrap_or_else(|| "#868e96".to_string())
        )
    };

    let name_text_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        format!(
            "font-size: {}; color: {}; font-weight: {};",
            font_size_for_name,
            scheme_colors.text,
            if is_group.get() {
                theme_val.typography.font_weights.semibold
            } else {
                theme_val.typography.font_weights.normal
            }
        )
    };

    // Helper function to get input styles
    let get_input_styles = {
        move || {
            let theme_val = theme.get();
            let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

            format!(
                "width: 100%; padding: {}; font-size: {}; border: 1px solid {}; border-radius: {}; background-color: {}; color: {};",
                input_padding_for_input,
                font_size_for_input,
                scheme_colors.get_color("gray", 4).unwrap_or_else(|| "#ced4da".to_string()),
                theme_val.radius.sm,
                scheme_colors.background,
                scheme_colors.text
            )
        }
    };

    let get_color_input_styles = move || {
        let theme_val = theme.get();

        format!(
            "width: 100%; height: 1.5rem; border: none; border-radius: {}; cursor: pointer;",
            theme_val.radius.sm
        )
    };

    let get_select_styles = {
        move || {
            let theme_val = theme.get();
            let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

            format!(
                "width: 100%; padding: {}; font-size: {}; border: 1px solid {}; border-radius: {}; background-color: {}; color: {};",
                input_padding_for_select,
                font_size_for_select,
                scheme_colors.get_color("gray", 4).unwrap_or_else(|| "#ced4da".to_string()),
                theme_val.radius.sm,
                scheme_colors.background,
                scheme_colors.text
            )
        }
    };

    let get_button_styles = {
        move || {
            let theme_val = theme.get();
            let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

            format!(
                "padding: {}; font-size: {}; border: 1px solid {}; border-radius: {}; background-color: {}; color: {}; cursor: pointer;",
                input_padding_for_button,
                font_size_for_button,
                scheme_colors.get_color("blue", 5).unwrap_or_else(|| "#339af0".to_string()),
                theme_val.radius.sm,
                scheme_colors.get_color("blue", 6).unwrap_or_else(|| "#228be6".to_string()),
                "#ffffff"
            )
        }
    };

    let get_readonly_styles = {
        move || {
            let theme_val = theme.get();
            let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

            format!(
                "font-size: {}; color: {}; font-style: italic;",
                font_size_for_readonly,
                scheme_colors
                    .get_color("gray", 6)
                    .unwrap_or_else(|| "#868e96".to_string())
            )
        }
    };

    // Event handlers
    let full_path_for_change = full_path.clone();
    let handle_value_change = move |new_value: String| {
        if let Some(callback) = on_change {
            callback.run((full_path_for_change.clone(), new_value));
        }
    };

    let full_path_for_action = full_path.clone();

    let key_for_toggle = node_key.get_untracked();
    let handle_toggle = move |_| {
        toggle_node.run(key_for_toggle.clone());
    };

    // Value editor based on type
    let value_editor = move || {
        let val = node_value.get();
        let is_disabled = disabled.get() || !node_enabled.get();

        match val {
            ParameterValue::String(s) => {
                let s_clone = s.clone();
                let input_style = get_input_styles();
                let on_input = {
                    let handle = handle_value_change.clone();
                    move |ev: leptos::ev::Event| {
                        let target = ev.target().unwrap();
                        let input: web_sys::HtmlInputElement = target.unchecked_into();
                        handle(input.value());
                    }
                };

                view! {
                    <input
                        type="text"
                        style=input_style
                        prop:value=s_clone
                        on:input=on_input
                        disabled=is_disabled
                    />
                }
                .into_any()
            }
            ParameterValue::Number {
                value,
                min,
                max,
                step,
                ..
            } => {
                let value_clone = value.clone();
                let min_attr = min.unwrap_or_default();
                let max_attr = max.unwrap_or_default();
                let step_attr = step.unwrap_or_else(|| "any".to_string());
                let input_style = get_input_styles();

                let on_input = {
                    let handle = handle_value_change.clone();
                    move |ev: leptos::ev::Event| {
                        let target = ev.target().unwrap();
                        let input: web_sys::HtmlInputElement = target.unchecked_into();
                        handle(input.value());
                    }
                };

                view! {
                    <input
                        type="number"
                        style=input_style
                        prop:value=value_clone
                        min=min_attr
                        max=max_attr
                        step=step_attr
                        on:input=on_input
                        disabled=is_disabled
                    />
                }
                .into_any()
            }
            ParameterValue::Bool(checked) => {
                let on_change = {
                    let handle = handle_value_change.clone();
                    move |ev: leptos::ev::Event| {
                        let target = ev.target().unwrap();
                        let input: web_sys::HtmlInputElement = target.unchecked_into();
                        handle(input.checked().to_string());
                    }
                };

                view! {
                    <input
                        type="checkbox"
                        style="cursor: pointer;"
                        prop:checked=checked
                        on:change=on_change
                        disabled=is_disabled
                    />
                }
                .into_any()
            }
            ParameterValue::Color(color) => {
                let color_clone = color.clone();
                let color_style = get_color_input_styles();

                let on_input = {
                    let handle = handle_value_change.clone();
                    move |ev: leptos::ev::Event| {
                        let target = ev.target().unwrap();
                        let input: web_sys::HtmlInputElement = target.unchecked_into();
                        handle(input.value());
                    }
                };

                view! {
                    <input
                        type="color"
                        style=color_style
                        prop:value=color_clone
                        on:input=on_input
                        disabled=is_disabled
                    />
                }
                .into_any()
            }
            ParameterValue::Enum { value, options } => {
                let value_clone = value.clone();
                let select_style = get_select_styles();

                let on_change = {
                    let handle = handle_value_change.clone();
                    move |ev: leptos::ev::Event| {
                        let target = ev.target().unwrap();
                        let select: web_sys::HtmlSelectElement = target.unchecked_into();
                        handle(select.value());
                    }
                };

                view! {
                    <select
                        style=select_style
                        on:change=on_change
                        disabled=is_disabled
                    >
                        {options.into_iter().map(|opt| {
                            let is_selected = opt == value_clone;
                            let opt_for_display = opt.clone();
                            view! {
                                <option value=opt selected=is_selected>{opt_for_display}</option>
                            }
                        }).collect::<Vec<_>>()}
                    </select>
                }
                .into_any()
            }
            ParameterValue::Action { label } => {
                let button_style = get_button_styles();
                let action_path = full_path_for_action.clone();
                let handle_action_click = move |_| {
                    if let Some(callback) = on_action {
                        callback.run(action_path.clone());
                    }
                };
                view! {
                    <button
                        style=button_style
                        on:click=handle_action_click
                        disabled=is_disabled
                    >
                        {label}
                    </button>
                }
                .into_any()
            }
            ParameterValue::ReadOnly(text) => {
                let readonly_style = get_readonly_styles();
                view! {
                    <span style=readonly_style>{text}</span>
                }
                .into_any()
            }
            ParameterValue::Group => view! {
                <span></span>
            }
            .into_any(),
        }
    };

    // Child nodes
    let children_view = move || {
        if !is_expanded.get() || node_children.get().is_empty() {
            return view! { <div></div> }.into_any();
        }

        let children = node_children.get();
        let child_depth = depth + 1;
        let child_path = full_path.clone();

        view! {
            <div class="parameter-tree-children">
                {children.into_iter().map(|child| {
                    let child_signal = RwSignal::new(child);
                    view! {
                        <ParameterTreeNode
                            node=child_signal.into()
                            depth=child_depth
                            path=child_path.clone()
                            search_query=search_query
                            expanded_nodes=expanded_nodes
                            toggle_node=toggle_node
                            on_change=on_change
                            on_action=on_action
                            on_validate=on_validate
                            disabled=disabled
                            font_size=font_size.clone()
                            row_height=row_height_for_children.clone()
                            input_padding=input_padding.clone()
                            indent_size=indent_size_for_children.clone()
                            name_width=name_width_for_children.clone()
                            value_width=value_width_for_children.clone()
                        />
                    }
                }).collect::<Vec<_>>()}
            </div>
        }
        .into_any()
    };

    view! {
        <div class="parameter-tree-node" style=move || if is_visible.get() { "" } else { "display: none;" }>
            // Row
            <div style=row_styles title=move || node_tooltip.get().unwrap_or_default()>
                // Name column
                <div style=name_cell_styles>
                    // Expand/collapse button
                    {(is_group.get() && has_children.get()).then(|| {
                        view! {
                            <span style=expand_button_styles on:click=handle_toggle>
                                {move || if is_expanded.get() { "▼" } else { "▶" }}
                            </span>
                        }
                    })}
                    {(!is_group.get() || !has_children.get()).then(|| {
                        view! {
                            <span style="width: 1.25rem; margin-right: 0.25rem;"></span>
                        }
                    })}
                    <span style=name_text_styles>{move || node_name.get()}</span>
                </div>

                // Value column
                <div style=value_cell_styles>
                    {value_editor}
                </div>
            </div>

            // Children
            {children_view}
        </div>
    }
}

/// Collect expanded node keys recursively
fn collect_expanded_keys(node: &ParameterNode, keys: &mut std::collections::HashSet<String>) {
    if node.expanded {
        keys.insert(node.key.clone());
    }
    for child in &node.children {
        collect_expanded_keys(child, keys);
    }
}

/// Convert tree to flat HashMap of values
pub fn tree_to_values(node: &ParameterNode, prefix: &str) -> HashMap<String, String> {
    let mut values = HashMap::new();
    let path = if prefix.is_empty() {
        node.key.clone()
    } else {
        format!("{}.{}", prefix, node.key)
    };

    match &node.value {
        ParameterValue::String(s) => {
            values.insert(path.clone(), s.clone());
        }
        ParameterValue::Number { value, .. } => {
            values.insert(path.clone(), value.clone());
        }
        ParameterValue::Bool(b) => {
            values.insert(path.clone(), b.to_string());
        }
        ParameterValue::Color(c) => {
            values.insert(path.clone(), c.clone());
        }
        ParameterValue::Enum { value, .. } => {
            values.insert(path.clone(), value.clone());
        }
        ParameterValue::ReadOnly(s) => {
            values.insert(path.clone(), s.clone());
        }
        _ => {}
    }

    for child in &node.children {
        values.extend(tree_to_values(child, &path));
    }

    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_node_new() {
        let node = ParameterNode::new("test", "Test", ParameterValue::String("value".to_string()));
        assert_eq!(node.key, "test");
        assert_eq!(node.name, "Test");
        assert!(node.expanded);
        assert!(node.visible);
        assert!(node.enabled);
    }

    #[test]
    fn test_parameter_node_group() {
        let node = ParameterNode::group("group", "Group Name");
        assert!(matches!(node.value, ParameterValue::Group));
    }

    #[test]
    fn test_parameter_node_string() {
        let node = ParameterNode::string("str", "String Param", "hello");
        if let ParameterValue::String(s) = node.value {
            assert_eq!(s, "hello");
        } else {
            panic!("Expected String value");
        }
    }

    #[test]
    fn test_parameter_node_number() {
        let node = ParameterNode::number("num", "Number Param", "42.5");
        if let ParameterValue::Number { value, .. } = node.value {
            assert_eq!(value, "42.5");
        } else {
            panic!("Expected Number value");
        }
    }

    #[test]
    fn test_parameter_node_bool() {
        let node = ParameterNode::bool("flag", "Boolean Param", true);
        if let ParameterValue::Bool(b) = node.value {
            assert!(b);
        } else {
            panic!("Expected Bool value");
        }
    }

    #[test]
    fn test_parameter_node_color() {
        let node = ParameterNode::color("bg", "Background", "#ff0000");
        if let ParameterValue::Color(c) = node.value {
            assert_eq!(c, "#ff0000");
        } else {
            panic!("Expected Color value");
        }
    }

    #[test]
    fn test_parameter_node_enum() {
        let node = ParameterNode::enumeration(
            "type",
            "Type",
            "option1",
            vec!["option1".to_string(), "option2".to_string()],
        );
        if let ParameterValue::Enum { value, options } = node.value {
            assert_eq!(value, "option1");
            assert_eq!(options.len(), 2);
        } else {
            panic!("Expected Enum value");
        }
    }

    #[test]
    fn test_parameter_node_action() {
        let node = ParameterNode::action("reset", "Reset", "Reset All");
        if let ParameterValue::Action { label } = node.value {
            assert_eq!(label, "Reset All");
        } else {
            panic!("Expected Action value");
        }
    }

    #[test]
    fn test_parameter_node_with_children() {
        let node = ParameterNode::group("parent", "Parent")
            .with_child(ParameterNode::string("child1", "Child 1", "a"))
            .with_child(ParameterNode::string("child2", "Child 2", "b"));

        assert_eq!(node.children.len(), 2);
    }

    #[test]
    fn test_parameter_node_with_tooltip() {
        let node = ParameterNode::string("test", "Test", "value").with_tooltip("This is a tooltip");
        assert_eq!(node.tooltip, Some("This is a tooltip".to_string()));
    }

    #[test]
    fn test_parameter_node_collapsed() {
        let node = ParameterNode::group("test", "Test").collapsed();
        assert!(!node.expanded);
    }

    #[test]
    fn test_parameter_node_disabled() {
        let node = ParameterNode::string("test", "Test", "value").disabled();
        assert!(!node.enabled);
    }

    #[test]
    fn test_parameter_node_with_range() {
        let node = ParameterNode::number("num", "Number", "50")
            .with_range("0", "100")
            .with_step("1");

        if let ParameterValue::Number { min, max, step, .. } = node.value {
            assert_eq!(min, Some("0".to_string()));
            assert_eq!(max, Some("100".to_string()));
            assert_eq!(step, Some("1".to_string()));
        } else {
            panic!("Expected Number value");
        }
    }

    #[test]
    fn test_parameter_tree_size_default() {
        assert_eq!(ParameterTreeSize::default(), ParameterTreeSize::Md);
    }

    #[test]
    fn test_tree_to_values() {
        let tree = ParameterNode::group("root", "Root")
            .with_child(ParameterNode::string("name", "Name", "John"))
            .with_child(ParameterNode::number("age", "Age", "30"));

        let values = tree_to_values(&tree, "");

        assert_eq!(values.get("root.name"), Some(&"John".to_string()));
        assert_eq!(values.get("root.age"), Some(&"30".to_string()));
    }

    #[test]
    fn test_tree_to_values_nested() {
        let tree = ParameterNode::group("root", "Root").with_child(
            ParameterNode::group("settings", "Settings")
                .with_child(ParameterNode::bool("enabled", "Enabled", true)),
        );

        let values = tree_to_values(&tree, "");

        assert_eq!(
            values.get("root.settings.enabled"),
            Some(&"true".to_string())
        );
    }

    #[test]
    fn test_parameter_value_default() {
        let value = ParameterValue::default();
        assert!(matches!(value, ParameterValue::String(s) if s.is_empty()));
    }

    #[test]
    fn test_parameter_tree_config_new() {
        let config = ParameterTreeConfig::new();
        assert!(config.show_header);
        assert!(config.show_search);
        assert!(config.searchable);
    }
}
