use crate::components::number_input::{NumberInputPrecision, ParseError};
use crate::components::parameter_slider::{ParameterSliderScale, ParameterSliderSize};
use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;
use std::collections::HashMap;
use wasm_bindgen::JsCast;

/// Definition of a single parameter for the grid
#[derive(Clone, Debug, PartialEq)]
pub struct ParameterDef {
    /// Unique identifier for the parameter
    pub id: String,
    /// Display label
    pub label: String,
    /// Minimum value
    pub min: String,
    /// Maximum value
    pub max: String,
    /// Step increment
    pub step: String,
    /// Default value
    pub default: String,
    /// Current value (if different from default)
    pub value: Option<String>,
    /// Precision type
    pub precision: NumberInputPrecision,
    /// Scale type (linear or logarithmic)
    pub scale: ParameterSliderScale,
    /// Optional group name for organizing parameters
    pub group: Option<String>,
    /// Whether parameter is read-only
    pub read_only: bool,
    /// Display precision (decimal places)
    pub display_precision: usize,
}

impl ParameterDef {
    /// Create a new parameter definition
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            min: "0".to_string(),
            max: "100".to_string(),
            step: "1".to_string(),
            default: "50".to_string(),
            value: None,
            precision: NumberInputPrecision::Decimal(6),
            scale: ParameterSliderScale::Linear,
            group: None,
            read_only: false,
            display_precision: 2,
        }
    }

    /// Set the range (min, max)
    pub fn range(mut self, min: impl Into<String>, max: impl Into<String>) -> Self {
        self.min = min.into();
        self.max = max.into();
        self
    }

    /// Set the step value
    pub fn step(mut self, step: impl Into<String>) -> Self {
        self.step = step.into();
        self
    }

    /// Set the default value
    pub fn default(mut self, default: impl Into<String>) -> Self {
        self.default = default.into();
        self
    }

    /// Set the current value
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Set the precision type
    pub fn precision(mut self, precision: NumberInputPrecision) -> Self {
        self.precision = precision;
        self
    }

    /// Set logarithmic scale
    pub fn logarithmic(mut self) -> Self {
        self.scale = ParameterSliderScale::Logarithmic;
        self
    }

    /// Set the group
    pub fn group(mut self, group: impl Into<String>) -> Self {
        self.group = Some(group.into());
        self
    }

    /// Set as read-only
    pub fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }

    /// Set display precision
    pub fn display_precision(mut self, precision: usize) -> Self {
        self.display_precision = precision;
        self
    }

    /// Get the effective current value
    pub fn current_value(&self) -> &str {
        self.value.as_ref().unwrap_or(&self.default)
    }
}

/// Preset configuration for saving/loading parameter states
#[derive(Clone, Debug, PartialEq)]
pub struct ParameterPreset {
    /// Name of the preset
    pub name: String,
    /// Parameter values (id -> value)
    pub values: HashMap<String, String>,
}

impl ParameterPreset {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            values: HashMap::new(),
        }
    }

    pub fn with_value(mut self, id: impl Into<String>, value: impl Into<String>) -> Self {
        self.values.insert(id.into(), value.into());
        self
    }
}

/// Size variants for the ParameterGrid
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ParameterGridSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

/// Layout direction for the grid
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ParameterGridLayout {
    #[default]
    Vertical,
    Horizontal,
    Grid {
        columns: usize,
    },
}

/// A grid of parameter sliders for manipulating multiple values.
///
/// Inspired by Mathematica's Manipulate controls, this component provides
/// a compact interface for adjusting multiple parameters simultaneously.
///
/// # Features
/// - Multiple parameter sliders in a grid layout
/// - Parameter grouping with collapsible sections
/// - Preset save/load functionality
/// - Reset to defaults
/// - Linked parameter updates
///
/// # Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use mingot::prelude::*;
///
/// let parameters = vec![
///     ParameterDef::new("amplitude", "Amplitude")
///         .range("0", "10")
///         .step("0.1")
///         .default("5"),
///     ParameterDef::new("frequency", "Frequency")
///         .range("0.1", "100")
///         .step("0.1")
///         .default("1")
///         .logarithmic(),
///     ParameterDef::new("phase", "Phase")
///         .range("0", "6.283")
///         .step("0.01")
///         .default("0"),
/// ];
///
/// view! {
///     <ParameterGrid
///         parameters=parameters
///         on_change=Callback::new(move |params: HashMap<String, String>| {
///             // Handle parameter changes
///         })
///     />
/// }
/// ```
#[component]
pub fn ParameterGrid(
    /// Parameter definitions
    #[prop(into)]
    parameters: Signal<Vec<ParameterDef>>,
    /// Layout direction
    #[prop(default = ParameterGridLayout::Vertical)]
    layout: ParameterGridLayout,
    /// Size of the sliders
    #[prop(optional)]
    size: Option<ParameterGridSize>,
    /// Whether to show input fields
    #[prop(default = true)]
    show_inputs: bool,
    /// Whether to show values
    #[prop(default = true)]
    show_values: bool,
    /// Whether to show reset button
    #[prop(default = true)]
    show_reset: bool,
    /// Available presets
    #[prop(optional)]
    presets: Option<Vec<ParameterPreset>>,
    /// Whether groups are initially collapsed
    #[prop(default = false)]
    groups_collapsed: bool,
    /// Disabled state
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Callback when any parameter changes
    #[prop(optional)]
    on_change: Option<Callback<HashMap<String, String>>>,
    /// Callback for individual parameter changes
    #[prop(optional)]
    on_parameter_change: Option<Callback<(String, String)>>,
    /// Callback for validation results
    #[prop(optional)]
    on_validate: Option<Callback<(String, Result<String, ParseError>)>>,
    /// Callback when preset is selected
    #[prop(optional)]
    on_preset_select: Option<Callback<String>>,
    /// Callback when reset is clicked
    #[prop(optional)]
    on_reset: Option<Callback<()>>,
    /// Additional CSS class
    #[prop(optional, into)]
    class: Option<String>,
    /// Additional inline styles
    #[prop(optional, into)]
    style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let size = size.unwrap_or_default();

    // Track current values for all parameters
    let values = RwSignal::new(HashMap::<String, String>::new());

    // Track collapsed state for groups
    let collapsed_groups = RwSignal::new(if groups_collapsed {
        parameters
            .get_untracked()
            .iter()
            .filter_map(|p| p.group.clone())
            .collect::<std::collections::HashSet<_>>()
    } else {
        std::collections::HashSet::new()
    });

    // Initialize values from parameters
    Effect::new(move |_| {
        let params = parameters.get();
        let mut new_values = HashMap::new();
        for param in params {
            new_values.insert(param.id.clone(), param.current_value().to_string());
        }
        values.set(new_values);
    });

    // Handle individual parameter change
    let handle_param_change = move |id: String, value: String| {
        values.update(|v| {
            v.insert(id.clone(), value.clone());
        });

        if let Some(callback) = on_parameter_change {
            callback.run((id, value));
        }

        if let Some(callback) = on_change {
            callback.run(values.get());
        }
    };

    // Handle reset
    let handle_reset = move |_| {
        let params = parameters.get();
        let mut new_values = HashMap::new();
        for param in params {
            new_values.insert(param.id.clone(), param.default.clone());
        }
        values.set(new_values.clone());

        if let Some(callback) = on_change {
            callback.run(new_values);
        }

        if let Some(callback) = on_reset {
            callback.run(());
        }
    };

    // Handle preset selection
    let handle_preset_select = move |preset: ParameterPreset| {
        values.update(|v| {
            for (id, value) in preset.values {
                v.insert(id, value);
            }
        });

        if let Some(callback) = on_preset_select {
            callback.run(preset.name.clone());
        }

        if let Some(callback) = on_change {
            callback.run(values.get());
        }
    };

    // Toggle group collapse
    let toggle_group = move |group: String| {
        collapsed_groups.update(|groups| {
            if groups.contains(&group) {
                groups.remove(&group);
            } else {
                groups.insert(group);
            }
        });
    };

    // Get slider size from grid size
    let slider_size = match size {
        ParameterGridSize::Xs => ParameterSliderSize::Xs,
        ParameterGridSize::Sm => ParameterSliderSize::Sm,
        ParameterGridSize::Md => ParameterSliderSize::Md,
        ParameterGridSize::Lg => ParameterSliderSize::Lg,
        ParameterGridSize::Xl => ParameterSliderSize::Xl,
    };

    // Organize parameters by group
    let grouped_parameters = Memo::new(move |_| {
        let params = parameters.get();
        let mut groups: Vec<(Option<String>, Vec<ParameterDef>)> = Vec::new();
        let mut current_group: Option<String> = None;
        let mut current_params: Vec<ParameterDef> = Vec::new();

        for param in params {
            if param.group != current_group {
                if !current_params.is_empty() {
                    groups.push((current_group.clone(), std::mem::take(&mut current_params)));
                }
                current_group = param.group.clone();
            }
            current_params.push(param);
        }

        if !current_params.is_empty() {
            groups.push((current_group, current_params));
        }

        groups
    });

    // Container styles
    let container_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        let mut builder = StyleBuilder::new();
        builder
            .add("display", "flex")
            .add("flex-direction", "column")
            .add("gap", "1rem")
            .add("padding", "1rem")
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
            .add("border-radius", theme_val.radius.md);

        let mut result = builder.build();
        if let Some(ref s) = style {
            if !result.is_empty() {
                result.push_str("; ");
            }
            result.push_str(s);
        }
        result
    };

    let header_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        format!(
            "display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem; padding-bottom: 0.5rem; border-bottom: 1px solid {};",
            scheme_colors.get_color("gray", 3).unwrap_or_else(|| "#dee2e6".to_string())
        )
    };

    let title_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        format!(
            "font-size: {}; font-weight: {}; color: {};",
            theme_val.typography.font_sizes.md,
            theme_val.typography.font_weights.semibold,
            scheme_colors.text
        )
    };

    let group_header_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        format!(
            "display: flex; align-items: center; gap: 0.5rem; padding: 0.5rem 0; cursor: pointer; font-size: {}; font-weight: {}; color: {};",
            theme_val.typography.font_sizes.sm,
            theme_val.typography.font_weights.medium,
            scheme_colors.text
        )
    };

    let grid_styles = move || match layout {
        ParameterGridLayout::Vertical => {
            "display: flex; flex-direction: column; gap: 0.75rem;".to_string()
        }
        ParameterGridLayout::Horizontal => {
            "display: flex; flex-direction: row; flex-wrap: wrap; gap: 1rem;".to_string()
        }
        ParameterGridLayout::Grid { columns } => {
            format!(
                "display: grid; grid-template-columns: repeat({}, 1fr); gap: 1rem;",
                columns
            )
        }
    };

    let button_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        format!(
            "padding: 0.25rem 0.75rem; font-size: 0.75rem; border: 1px solid {}; border-radius: {}; background-color: transparent; color: {}; cursor: pointer;",
            scheme_colors.get_color("gray", 4).unwrap_or_else(|| "#ced4da".to_string()),
            theme_val.radius.sm,
            scheme_colors.text
        )
    };

    let select_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        format!(
            "padding: 0.25rem 0.5rem; font-size: 0.75rem; border: 1px solid {}; border-radius: {}; background-color: {}; color: {};",
            scheme_colors.get_color("gray", 4).unwrap_or_else(|| "#ced4da".to_string()),
            theme_val.radius.sm,
            scheme_colors.background,
            scheme_colors.text
        )
    };

    let chevron_styles = move |is_collapsed: bool| {
        format!(
            "transform: rotate({}deg); transition: transform 0.2s ease;",
            if is_collapsed { -90 } else { 0 }
        )
    };

    let class_str = format!(
        "mingot-parameter-grid {}",
        class.clone().unwrap_or_default()
    );
    let presets_for_select = presets.clone();
    let presets_for_handler = presets.clone();

    view! {
        <div class=class_str style=container_styles>
            // Header with controls
            {(show_reset || presets.is_some()).then(|| {
                let handle_preset_change = move |ev: leptos::ev::Event| {
                    let target = ev.target().unwrap();
                    let select: web_sys::HtmlSelectElement = target.unchecked_into();
                    let selected_name = select.value();

                    if let Some(ref preset_list) = presets_for_handler {
                        if let Some(preset) = preset_list.iter().find(|p| p.name == selected_name) {
                            handle_preset_select(preset.clone());
                        }
                    }
                };

                view! {
                    <div style=header_styles>
                        <span style=title_styles>"Parameters"</span>
                        <div style="display: flex; gap: 0.5rem; align-items: center;">
                            {presets_for_select.clone().map(|preset_list| {
                                view! {
                                    <select style=select_styles on:change=handle_preset_change>
                                        <option value="" disabled selected>"Presets"</option>
                                        {preset_list.iter().map(|preset| {
                                            let name = preset.name.clone();
                                            let name_for_display = name.clone();
                                            view! {
                                                <option value=name>{name_for_display}</option>
                                            }
                                        }).collect::<Vec<_>>()}
                                    </select>
                                }
                            })}
                            {show_reset.then(|| {
                                view! {
                                    <button
                                        style=button_styles
                                        on:click=handle_reset
                                        disabled=disabled
                                    >
                                        "Reset"
                                    </button>
                                }
                            })}
                        </div>
                    </div>
                }
            })}

            // Parameter groups
            <div>
                {move || {
                    grouped_parameters.get().into_iter().map(|(group, params)| {
                        let _group_name = group.clone();
                        let is_collapsed = group.as_ref().map(|g| {
                            collapsed_groups.get().contains(g)
                        }).unwrap_or(false);

                        view! {
                            <div class="mingot-parameter-group">
                                // Group header (if group exists)
                                {group.clone().map(|g| {
                                    let group_for_toggle = g.clone();
                                    view! {
                                        <div
                                            style=group_header_styles
                                            on:click=move |_| toggle_group(group_for_toggle.clone())
                                        >
                                            <span style=move || chevron_styles(is_collapsed)>"▼"</span>
                                            <span>{g}</span>
                                        </div>
                                    }
                                })}

                                // Parameters
                                {(!is_collapsed).then(|| {
                                    view! {
                                        <div style=grid_styles>
                                            {params.into_iter().map(|param| {
                                                let param_id = param.id.clone();
                                                let param_id_for_change = param.id.clone();
                                                let param_id_for_validate = param.id.clone();

                                                let param_value = Memo::new(move |_| {
                                                    values.get().get(&param_id).cloned().unwrap_or_default()
                                                });

                                                let on_param_change = {
                                                    let id = param_id_for_change.clone();
                                                    Callback::new(move |value: String| {
                                                        handle_param_change(id.clone(), value);
                                                    })
                                                };

                                                // Create validation callback that delegates to parent if provided
                                                let on_param_validate = {
                                                    let id = param_id_for_validate.clone();
                                                    let validate_callback = on_validate;
                                                    Callback::new(move |result: Result<String, ParseError>| {
                                                        if let Some(callback) = validate_callback {
                                                            callback.run((id.clone(), result));
                                                        }
                                                    })
                                                };

                                                view! {
                                                    <crate::components::parameter_slider::ParameterSlider
                                                        value=Signal::derive(move || param_value.get())
                                                        min=param.min.clone()
                                                        max=param.max.clone()
                                                        step=param.step.clone()
                                                        precision=param.precision
                                                        scale=param.scale
                                                        size=slider_size
                                                        label=param.label.clone()
                                                        show_value=show_values
                                                        show_input=show_inputs
                                                        display_precision=param.display_precision
                                                        disabled=Signal::derive(move || disabled.get() || param.read_only)
                                                        on_change=on_param_change
                                                        on_validate=on_param_validate
                                                    />
                                                }
                                            }).collect::<Vec<_>>()}
                                        </div>
                                    }
                                })}
                            </div>
                        }
                    }).collect::<Vec<_>>()
                }}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_def_new() {
        let param = ParameterDef::new("test", "Test Parameter");
        assert_eq!(param.id, "test");
        assert_eq!(param.label, "Test Parameter");
        assert_eq!(param.min, "0");
        assert_eq!(param.max, "100");
        assert_eq!(param.step, "1");
        assert_eq!(param.default, "50");
    }

    #[test]
    fn test_parameter_def_builder() {
        let param = ParameterDef::new("amp", "Amplitude")
            .range("0", "10")
            .step("0.1")
            .default("5")
            .logarithmic()
            .group("Wave")
            .display_precision(3);

        assert_eq!(param.id, "amp");
        assert_eq!(param.min, "0");
        assert_eq!(param.max, "10");
        assert_eq!(param.step, "0.1");
        assert_eq!(param.default, "5");
        assert_eq!(param.scale, ParameterSliderScale::Logarithmic);
        assert_eq!(param.group, Some("Wave".to_string()));
        assert_eq!(param.display_precision, 3);
    }

    #[test]
    fn test_parameter_def_current_value() {
        let param1 = ParameterDef::new("test", "Test").default("42");
        assert_eq!(param1.current_value(), "42");

        let param2 = ParameterDef::new("test", "Test").default("42").value("100");
        assert_eq!(param2.current_value(), "100");
    }

    #[test]
    fn test_parameter_def_read_only() {
        let param = ParameterDef::new("test", "Test").read_only();
        assert!(param.read_only);
    }

    #[test]
    fn test_parameter_preset_new() {
        let preset = ParameterPreset::new("Default");
        assert_eq!(preset.name, "Default");
        assert!(preset.values.is_empty());
    }

    #[test]
    fn test_parameter_preset_with_values() {
        let preset = ParameterPreset::new("Custom")
            .with_value("amp", "5")
            .with_value("freq", "440");

        assert_eq!(preset.values.get("amp"), Some(&"5".to_string()));
        assert_eq!(preset.values.get("freq"), Some(&"440".to_string()));
    }

    #[test]
    fn test_parameter_grid_size_default() {
        assert_eq!(ParameterGridSize::default(), ParameterGridSize::Md);
    }

    #[test]
    fn test_parameter_grid_layout_default() {
        assert_eq!(
            ParameterGridLayout::default(),
            ParameterGridLayout::Vertical
        );
    }

    #[test]
    fn test_parameter_grid_size_variants() {
        let sizes = [
            ParameterGridSize::Xs,
            ParameterGridSize::Sm,
            ParameterGridSize::Md,
            ParameterGridSize::Lg,
            ParameterGridSize::Xl,
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
