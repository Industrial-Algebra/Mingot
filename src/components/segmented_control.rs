use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

/// Size variants for the SegmentedControl component
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SegmentedControlSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

/// Data item for a segment in the control
#[derive(Clone, Debug, PartialEq)]
pub struct SegmentedControlItem {
    pub value: String,
    pub label: String,
    pub disabled: bool,
}

impl SegmentedControlItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            disabled: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

/// A segmented control component for selecting between multiple options.
///
/// # Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use mingot::prelude::*;
///
/// let selected = RwSignal::new("react".to_string());
///
/// view! {
///     <SegmentedControl
///         data=vec![
///             SegmentedControlItem::new("react", "React"),
///             SegmentedControlItem::new("vue", "Vue"),
///             SegmentedControlItem::new("svelte", "Svelte"),
///         ]
///         value=selected
///         on_change=Callback::new(move |v: String| selected.set(v))
///     />
/// }
/// ```
#[component]
pub fn SegmentedControl(
    /// The options to display
    data: Vec<SegmentedControlItem>,
    /// Currently selected value
    #[prop(into)]
    value: Signal<String>,
    /// Size of the control
    #[prop(optional)]
    size: Option<SegmentedControlSize>,
    /// Whether the control takes full width
    #[prop(default = false)]
    full_width: bool,
    /// Disabled state for the entire control
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Orientation
    #[prop(default = false)]
    vertical: bool,
    /// Callback when selection changes
    #[prop(optional)]
    on_change: Option<Callback<String>>,
    /// Additional CSS class
    #[prop(optional, into)]
    class: Option<String>,
    /// Additional inline styles
    #[prop(optional, into)]
    style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let size = size.unwrap_or_default();

    // Size-based dimensions
    let (height, padding, font_size) = match size {
        SegmentedControlSize::Xs => ("1.75rem", "0.25rem 0.5rem", "xs"),
        SegmentedControlSize::Sm => ("2rem", "0.25rem 0.625rem", "sm"),
        SegmentedControlSize::Md => ("2.375rem", "0.375rem 0.875rem", "sm"),
        SegmentedControlSize::Lg => ("2.75rem", "0.5rem 1rem", "md"),
        SegmentedControlSize::Xl => ("3.25rem", "0.625rem 1.25rem", "lg"),
    };

    let container_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let is_disabled = disabled.get();

        let mut builder = StyleBuilder::new();
        builder
            .add("display", "inline-flex")
            .add("flex-direction", if vertical { "column" } else { "row" })
            .add("align-items", "stretch")
            .add("position", "relative")
            .add("border-radius", theme_val.radius.sm)
            .add(
                "background-color",
                scheme_colors
                    .get_color("gray", 1)
                    .unwrap_or_else(|| "#f1f3f5".to_string()),
            )
            .add("padding", "4px")
            .add("gap", "4px")
            .add_if(full_width, "width", "100%")
            .add_if(is_disabled, "opacity", "0.6")
            .add_if(is_disabled, "pointer-events", "none");

        if let Some(ref s) = style {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let class_str = format!("mingot-segmented-control {}", class.unwrap_or_default());

    view! {
        <div
            class=class_str
            style=container_styles
            role="radiogroup"
        >
            {data.into_iter().map(|item| {
                let item_value = item.value.clone();
                let item_label = item.label.clone();
                let item_disabled = item.disabled;
                let item_value_for_check = item_value.clone();
                let item_value_for_click = item_value.clone();

                let segment_styles = move || {
                    let theme_val = theme.get();
                    let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
                    let is_selected = value.get() == item_value_for_check;
                    let is_disabled = disabled.get() || item_disabled;

                    let mut builder = StyleBuilder::new();
                    builder
                        .add("display", "flex")
                        .add("align-items", "center")
                        .add("justify-content", "center")
                        .add("height", height)
                        .add("padding", padding)
                        .add("border-radius", theme_val.radius.xs)
                        .add("border", "none")
                        .add("font-family", theme_val.typography.font_family)
                        .add("font-size", match font_size {
                            "xs" => theme_val.typography.font_sizes.xs,
                            "sm" => theme_val.typography.font_sizes.sm,
                            "md" => theme_val.typography.font_sizes.md,
                            "lg" => theme_val.typography.font_sizes.lg,
                            _ => theme_val.typography.font_sizes.sm,
                        })
                        .add("font-weight", theme_val.typography.font_weights.medium.to_string())
                        .add("cursor", if is_disabled { "not-allowed" } else { "pointer" })
                        .add("transition", "all 0.15s ease")
                        .add("white-space", "nowrap")
                        .add("user-select", "none")
                        .add_if(full_width, "flex", "1");

                    if is_selected {
                        builder
                            .add("background-color", scheme_colors.white.clone())
                            .add("color", scheme_colors.text.clone())
                            .add("box-shadow", theme_val.shadows.xs);
                    } else {
                        builder
                            .add("background-color", "transparent")
                            .add("color", scheme_colors.get_color("gray", 7).unwrap_or_else(|| "#495057".to_string()));
                    }

                    if is_disabled {
                        builder.add("opacity", "0.5");
                    }

                    builder.build()
                };

                let handle_click = move |_| {
                    if disabled.get() || item_disabled {
                        return;
                    }
                    if let Some(callback) = on_change {
                        callback.run(item_value_for_click.clone());
                    }
                };

                view! {
                    <button
                        class="mingot-segmented-control-item"
                        style=segment_styles
                        role="radio"
                        aria-checked=move || (value.get() == item_value).to_string()
                        disabled=move || disabled.get() || item_disabled
                        on:click=handle_click
                    >
                        {item_label}
                    </button>
                }
            }).collect::<Vec<_>>()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_segmented_control_item_new() {
        let item = SegmentedControlItem::new("test", "Test Label");
        assert_eq!(item.value, "test");
        assert_eq!(item.label, "Test Label");
        assert!(!item.disabled);
    }

    #[test]
    fn test_segmented_control_item_disabled() {
        let item = SegmentedControlItem::new("test", "Test").disabled();
        assert!(item.disabled);
    }

    #[test]
    fn test_segmented_control_size_default() {
        assert_eq!(SegmentedControlSize::default(), SegmentedControlSize::Md);
    }

    #[test]
    fn test_segmented_control_size_variants() {
        let sizes = [
            SegmentedControlSize::Xs,
            SegmentedControlSize::Sm,
            SegmentedControlSize::Md,
            SegmentedControlSize::Lg,
            SegmentedControlSize::Xl,
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
