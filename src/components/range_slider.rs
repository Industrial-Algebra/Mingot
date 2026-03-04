use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;

/// Size variants for the RangeSlider component
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum RangeSliderSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

/// A range slider component for selecting a range of values.
///
/// # Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use mingot::prelude::*;
///
/// let range = RwSignal::new((20.0, 80.0));
///
/// view! {
///     <RangeSlider
///         value=range
///         min=0.0
///         max=100.0
///         step=1.0
///         label="Price Range"
///         on_change=Callback::new(move |v| range.set(v))
///     />
/// }
/// ```
#[component]
pub fn RangeSlider(
    /// Current range value (min, max)
    #[prop(into)]
    value: Signal<(f64, f64)>,
    /// Minimum value
    #[prop(default = 0.0)]
    min: f64,
    /// Maximum value
    #[prop(default = 100.0)]
    max: f64,
    /// Step increment (0.0 for continuous)
    #[prop(default = 1.0)]
    step: f64,
    /// Minimum range between the two thumbs
    #[prop(default = 0.0)]
    min_range: f64,
    /// Size of the slider
    #[prop(optional)]
    size: Option<RangeSliderSize>,
    /// Label displayed above the slider
    #[prop(optional, into)]
    label: Option<String>,
    /// Whether to show the current values
    #[prop(default = false)]
    show_value: bool,
    /// Number of decimal places to display
    #[prop(default = 0)]
    precision: usize,
    /// Disabled state
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Callback when value changes
    #[prop(optional)]
    on_change: Option<Callback<(f64, f64)>>,
    /// Callback when dragging ends
    #[prop(optional)]
    on_change_end: Option<Callback<(f64, f64)>>,
    /// Additional CSS class
    #[prop(optional, into)]
    class: Option<String>,
    /// Additional inline styles
    #[prop(optional, into)]
    style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let size = size.unwrap_or_default();

    // Track which thumb is being dragged: None, Some(0) for left, Some(1) for right
    let dragging_thumb = RwSignal::new(Option::<usize>::None);

    // Calculate percentages from values
    let left_percentage = move || {
        let (low, _) = value.get();
        let range = max - min;
        if range == 0.0 {
            0.0
        } else {
            ((low - min) / range) * 100.0
        }
    };

    let right_percentage = move || {
        let (_, high) = value.get();
        let range = max - min;
        if range == 0.0 {
            100.0
        } else {
            ((high - min) / range) * 100.0
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

    // Track dimensions
    let (track_height, thumb_size) = match size {
        RangeSliderSize::Xs => ("4px", "12px"),
        RangeSliderSize::Sm => ("6px", "16px"),
        RangeSliderSize::Md => ("8px", "20px"),
        RangeSliderSize::Lg => ("10px", "24px"),
        RangeSliderSize::Xl => ("12px", "28px"),
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
        let is_dragging = dragging_thumb.get().is_some();

        let mut builder = StyleBuilder::new();
        builder
            .add("position", "absolute")
            .add("left", format!("{}%", left_percentage()))
            .add("top", "0")
            .add("height", "100%")
            .add(
                "width",
                format!("{}%", right_percentage() - left_percentage()),
            )
            .add("border-radius", theme_val.radius.xl)
            .add(
                "background-color",
                scheme_colors
                    .get_color("blue", 6)
                    .unwrap_or_else(|| "#228be6".to_string()),
            )
            .add(
                "transition",
                if is_dragging {
                    "none"
                } else {
                    "left 0.1s ease, width 0.1s ease"
                },
            );

        builder.build()
    };

    let make_thumb_styles = move |is_left: bool| {
        move || {
            let theme_val = theme.get();
            let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
            let is_disabled = disabled.get();
            let is_dragging = dragging_thumb.get().is_some();

            let pct = if is_left {
                left_percentage()
            } else {
                right_percentage()
            };

            let mut builder = StyleBuilder::new();
            builder
                .add("position", "absolute")
                .add("top", "50%")
                .add("left", format!("{}%", pct))
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
                .add(
                    "cursor",
                    if is_disabled {
                        "not-allowed"
                    } else if is_dragging {
                        "grabbing"
                    } else {
                        "grab"
                    },
                )
                .add(
                    "transition",
                    if is_dragging {
                        "none"
                    } else {
                        "left 0.1s ease"
                    },
                )
                .add("z-index", "1");

            builder.build()
        }
    };

    let left_thumb_styles = make_thumb_styles(true);
    let right_thumb_styles = make_thumb_styles(false);

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

    // Handle mouse/touch interaction
    let track_ref = NodeRef::<leptos::html::Div>::new();

    let handle_interaction = move |client_x: i32| {
        if disabled.get() {
            return;
        }

        let Some(track) = track_ref.get() else {
            return;
        };

        let element: web_sys::HtmlElement = track.into();
        let rect = element.get_bounding_client_rect();
        let track_left = rect.left();
        let track_width = rect.width();

        if track_width == 0.0 {
            return;
        }

        let relative_x = (client_x as f64) - track_left;
        let pct = (relative_x / track_width * 100.0).clamp(0.0, 100.0);
        let new_value = percentage_to_value(pct);

        let (current_low, current_high) = value.get();

        // Determine which thumb to move based on dragging state or proximity
        let thumb_idx = if let Some(idx) = dragging_thumb.get() {
            idx
        } else {
            // Choose thumb based on which is closer
            let low_distance = (new_value - current_low).abs();
            let high_distance = (new_value - current_high).abs();
            if low_distance <= high_distance {
                0
            } else {
                1
            }
        };

        let (new_low, new_high) = if thumb_idx == 0 {
            // Moving left thumb
            let max_low = current_high - min_range;
            (new_value.min(max_low), current_high)
        } else {
            // Moving right thumb
            let min_high = current_low + min_range;
            (current_low, new_value.max(min_high))
        };

        if let Some(callback) = on_change {
            callback.run((new_low, new_high));
        }
    };

    let handle_mouse_down = move |ev: ev::MouseEvent, thumb_idx: usize| {
        if disabled.get() {
            return;
        }
        ev.prevent_default();
        ev.stop_propagation();
        dragging_thumb.set(Some(thumb_idx));
    };

    let handle_track_mouse_down = move |ev: ev::MouseEvent| {
        if disabled.get() {
            return;
        }
        ev.prevent_default();
        handle_interaction(ev.client_x());
    };

    let handle_mouse_move = move |ev: ev::MouseEvent| {
        if dragging_thumb.get().is_none() || disabled.get() {
            return;
        }
        handle_interaction(ev.client_x());
    };

    let handle_mouse_up = move |_ev: ev::MouseEvent| {
        if dragging_thumb.get().is_some() {
            dragging_thumb.set(None);
            if let Some(callback) = on_change_end {
                callback.run(value.get());
            }
        }
    };

    // Format displayed value
    let format_value = move |val: f64| {
        if precision == 0 {
            format!("{}", val as i64)
        } else {
            format!("{:.1$}", val, precision)
        }
    };

    let display_range = move || {
        let (low, high) = value.get();
        format!("{} - {}", format_value(low), format_value(high))
    };

    let wrapper_styles = move || {
        let mut styles = String::from("width: 100%;");
        if let Some(ref s) = style {
            styles.push_str(s);
        }
        styles
    };

    let class_str = format!("mingot-range-slider {}", class.unwrap_or_default());

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
                        <span style="font-weight: normal;">{display_range}</span>
                    })}
                </div>
            })}

            <div style="position: relative; padding: 0.5rem 0;">
                <div
                    node_ref=track_ref
                    class="mingot-range-slider-track"
                    style=track_styles
                    on:mousedown=handle_track_mouse_down
                >
                    <div class="mingot-range-slider-filled" style=filled_styles></div>

                    <div
                        class="mingot-range-slider-thumb mingot-range-slider-thumb-left"
                        style=left_thumb_styles
                        on:mousedown=move |ev| handle_mouse_down(ev, 0)
                    ></div>

                    <div
                        class="mingot-range-slider-thumb mingot-range-slider-thumb-right"
                        style=right_thumb_styles
                        on:mousedown=move |ev| handle_mouse_down(ev, 1)
                    ></div>
                </div>
            </div>
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_slider_size_default() {
        assert_eq!(RangeSliderSize::default(), RangeSliderSize::Md);
    }

    #[test]
    fn test_range_slider_size_variants() {
        let sizes = [
            RangeSliderSize::Xs,
            RangeSliderSize::Sm,
            RangeSliderSize::Md,
            RangeSliderSize::Lg,
            RangeSliderSize::Xl,
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
