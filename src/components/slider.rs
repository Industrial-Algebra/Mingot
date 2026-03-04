use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;

/// Size variants for the Slider component
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum SliderSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

/// Mark to display on the slider track
#[derive(Clone, Debug, PartialEq)]
pub struct SliderMark {
    pub value: f64,
    pub label: Option<String>,
}

impl SliderMark {
    pub fn new(value: f64) -> Self {
        Self { value, label: None }
    }

    pub fn with_label(value: f64, label: impl Into<String>) -> Self {
        Self {
            value,
            label: Some(label.into()),
        }
    }
}

/// A slider component for selecting numeric values within a range.
///
/// # Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use mingot::prelude::*;
///
/// let value = RwSignal::new(50.0);
///
/// view! {
///     <Slider
///         value=value
///         min=0.0
///         max=100.0
///         step=1.0
///         label="Volume"
///         on_change=Callback::new(move |v| value.set(v))
///     />
/// }
/// ```
#[component]
pub fn Slider(
    /// Current value of the slider
    #[prop(into)]
    value: Signal<f64>,
    /// Minimum value
    #[prop(default = 0.0)]
    min: f64,
    /// Maximum value
    #[prop(default = 100.0)]
    max: f64,
    /// Step increment (0.0 for continuous)
    #[prop(default = 1.0)]
    step: f64,
    /// Size of the slider
    #[prop(optional)]
    size: Option<SliderSize>,
    /// Label displayed above the slider
    #[prop(optional, into)]
    label: Option<String>,
    /// Whether to show the current value
    #[prop(default = false)]
    show_value: bool,
    /// Number of decimal places to display (0 for integers)
    #[prop(default = 0)]
    precision: usize,
    /// Marks to display on the track
    #[prop(optional)]
    marks: Option<Vec<SliderMark>>,
    /// Disabled state
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Callback when value changes
    #[prop(optional)]
    on_change: Option<Callback<f64>>,
    /// Callback while dragging (called frequently)
    #[prop(optional)]
    on_change_end: Option<Callback<f64>>,
    /// Additional CSS class
    #[prop(optional, into)]
    class: Option<String>,
    /// Additional inline styles
    #[prop(optional, into)]
    style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let size = size.unwrap_or_default();
    let is_dragging = RwSignal::new(false);

    // Calculate percentage from value
    let percentage = move || {
        let val = value.get();
        let range = max - min;
        if range == 0.0 {
            0.0
        } else {
            ((val - min) / range) * 100.0
        }
    };

    // Convert percentage to value, respecting step
    let percentage_to_value = move |pct: f64| {
        let raw_value = min + (pct / 100.0) * (max - min);
        if step > 0.0 {
            let steps = ((raw_value - min) / step).round();
            (min + steps * step).clamp(min, max)
        } else {
            raw_value.clamp(min, max)
        }
    };

    // Track dimensions for slider
    let (track_height, thumb_size) = match size {
        SliderSize::Xs => ("4px", "12px"),
        SliderSize::Sm => ("6px", "16px"),
        SliderSize::Md => ("8px", "20px"),
        SliderSize::Lg => ("10px", "24px"),
        SliderSize::Xl => ("12px", "28px"),
    };

    let track_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let is_disabled = disabled.get();

        let mut builder = StyleBuilder::new();
        builder
            .add("position", "relative")
            .add("width", "100%")
            .add("height", track_height)
            .add("border-radius", theme_val.radius.xl)
            .add(
                "background-color",
                scheme_colors
                    .get_color("gray", 3)
                    .unwrap_or_else(|| "#dee2e6".to_string()),
            )
            .add(
                "cursor",
                if is_disabled {
                    "not-allowed"
                } else {
                    "pointer"
                },
            )
            .add_if(is_disabled, "opacity", "0.5");

        builder.build()
    };

    let filled_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        let mut builder = StyleBuilder::new();
        builder
            .add("position", "absolute")
            .add("left", "0")
            .add("top", "0")
            .add("height", "100%")
            .add("width", format!("{}%", percentage()))
            .add("border-radius", theme_val.radius.xl)
            .add(
                "background-color",
                scheme_colors
                    .get_color("blue", 6)
                    .unwrap_or_else(|| "#228be6".to_string()),
            )
            .add(
                "transition",
                if is_dragging.get() {
                    "none"
                } else {
                    "width 0.1s ease"
                },
            );

        builder.build()
    };

    let thumb_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let is_disabled = disabled.get();

        let mut builder = StyleBuilder::new();
        builder
            .add("position", "absolute")
            .add("top", "50%")
            .add("left", format!("{}%", percentage()))
            .add("transform", "translate(-50%, -50%)")
            .add("width", thumb_size)
            .add("height", thumb_size)
            .add("border-radius", "50%")
            .add("background-color", scheme_colors.white.clone())
            .add(
                "border",
                format!(
                    "2px solid {}",
                    scheme_colors
                        .get_color("blue", 6)
                        .unwrap_or_else(|| "#228be6".to_string())
                ),
            )
            .add("box-shadow", theme_val.shadows.sm)
            .add("cursor", if is_disabled { "not-allowed" } else { "grab" })
            .add(
                "transition",
                if is_dragging.get() {
                    "none"
                } else {
                    "left 0.1s ease"
                },
            )
            .add("z-index", "1");

        if is_dragging.get() && !is_disabled {
            builder.add("cursor", "grabbing");
        }

        builder.build()
    };

    let label_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "display: flex; justify-content: space-between; align-items: center; margin-bottom: 0.5rem; font-size: {}; font-weight: {}; color: {};",
            theme_val.typography.font_sizes.sm,
            theme_val.typography.font_weights.medium,
            scheme_colors.text
        )
    };

    let mark_styles = move |mark_value: f64| {
        let pct = ((mark_value - min) / (max - min)) * 100.0;
        format!(
            "position: absolute; left: {}%; transform: translateX(-50%); top: 100%; margin-top: 0.5rem; font-size: 0.75rem;",
            pct
        )
    };

    // Handle mouse/touch interaction
    let handle_interaction = move |client_x: i32, track_element: web_sys::HtmlElement| {
        if disabled.get() {
            return;
        }

        let rect = track_element.get_bounding_client_rect();
        let track_left = rect.left();
        let track_width = rect.width();

        if track_width == 0.0 {
            return;
        }

        let relative_x = (client_x as f64) - track_left;
        let pct = (relative_x / track_width * 100.0).clamp(0.0, 100.0);
        let new_value = percentage_to_value(pct);

        if let Some(callback) = on_change {
            callback.run(new_value);
        }
    };

    let track_ref = NodeRef::<leptos::html::Div>::new();

    let handle_mouse_down = move |ev: ev::MouseEvent| {
        if disabled.get() {
            return;
        }

        ev.prevent_default();
        is_dragging.set(true);

        if let Some(track) = track_ref.get() {
            let element: web_sys::HtmlElement = track.into();
            handle_interaction(ev.client_x(), element);
        }
    };

    let handle_mouse_move = move |ev: ev::MouseEvent| {
        if !is_dragging.get() || disabled.get() {
            return;
        }

        if let Some(track) = track_ref.get() {
            let element: web_sys::HtmlElement = track.into();
            handle_interaction(ev.client_x(), element);
        }
    };

    let handle_mouse_up = move |_ev: ev::MouseEvent| {
        if is_dragging.get() {
            is_dragging.set(false);
            if let Some(callback) = on_change_end {
                callback.run(value.get());
            }
        }
    };

    // Format displayed value
    let display_value = move || {
        let val = value.get();
        if precision == 0 {
            format!("{}", val as i64)
        } else {
            format!("{:.1$}", val, precision)
        }
    };

    let wrapper_styles = move || {
        let mut styles = String::from("width: 100%;");
        if let Some(ref s) = style {
            styles.push_str(s);
        }
        styles
    };

    let class_str = format!("mingot-slider {}", class.unwrap_or_default());

    view! {
        <div
            class=class_str
            style=wrapper_styles
            on:mousemove=handle_mouse_move
            on:mouseup=handle_mouse_up
            on:mouseleave=handle_mouse_up
        >
            {label.clone().map(|l| view! {
                <div style=label_styles>
                    <span>{l}</span>
                    {show_value.then(|| view! {
                        <span style="font-weight: normal;">{display_value}</span>
                    })}
                </div>
            })}

            <div style="position: relative; padding: 0.5rem 0;">
                <div
                    node_ref=track_ref
                    class="mingot-slider-track"
                    style=track_styles
                    on:mousedown=handle_mouse_down
                >
                    <div class="mingot-slider-filled" style=filled_styles></div>
                    <div class="mingot-slider-thumb" style=thumb_styles></div>
                </div>

                {marks.map(|m| view! {
                    <div class="mingot-slider-marks">
                        {m.into_iter().map(|mark| {
                            let mark_label = mark.label.clone();
                            view! {
                                <div style=move || mark_styles(mark.value)>
                                    {mark_label.clone()}
                                </div>
                            }
                        }).collect::<Vec<_>>()}
                    </div>
                })}
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slider_mark_new() {
        let mark = SliderMark::new(50.0);
        assert_eq!(mark.value, 50.0);
        assert!(mark.label.is_none());
    }

    #[test]
    fn test_slider_mark_with_label() {
        let mark = SliderMark::with_label(75.0, "75%");
        assert_eq!(mark.value, 75.0);
        assert_eq!(mark.label, Some("75%".to_string()));
    }

    #[test]
    fn test_slider_size_default() {
        assert_eq!(SliderSize::default(), SliderSize::Md);
    }

    #[test]
    fn test_slider_size_variants() {
        let sizes = [
            SliderSize::Xs,
            SliderSize::Sm,
            SliderSize::Md,
            SliderSize::Lg,
            SliderSize::Xl,
        ];
        // Ensure all variants are distinct
        for (i, s1) in sizes.iter().enumerate() {
            for (j, s2) in sizes.iter().enumerate() {
                if i != j {
                    assert_ne!(s1, s2);
                }
            }
        }
    }
}
