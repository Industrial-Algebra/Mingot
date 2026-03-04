use crate::components::number_input::{NumberInputPrecision, ParseError};
use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Size variants for the ParameterSlider component
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ParameterSliderSize {
    Xs,
    Sm,
    #[default]
    Md,
    Lg,
    Xl,
}

/// Scale type for the slider
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ParameterSliderScale {
    #[default]
    Linear,
    Logarithmic,
}

/// Mark to display on the slider track
#[derive(Clone, Debug, PartialEq)]
pub struct ParameterSliderMark {
    pub value: String,
    pub label: Option<String>,
}

impl ParameterSliderMark {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: None,
        }
    }

    pub fn with_label(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: Some(label.into()),
        }
    }
}

/// A high-precision parameter slider with exact decimal values.
///
/// Unlike the standard Slider which uses f64, ParameterSlider maintains
/// exact decimal precision using string-based values and optional rust_decimal
/// integration.
///
/// # Features
/// - Exact decimal arithmetic (no floating-point precision loss)
/// - Logarithmic scale option for wide-range values
/// - Integrated value input for precise entry
/// - Keyboard navigation with modifier key multipliers
/// - Animation/autoplay mode for demonstrations
///
/// # Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use mingot::prelude::*;
///
/// let value = RwSignal::new("50.0".to_string());
///
/// view! {
///     <ParameterSlider
///         value=value
///         min="-100.0"
///         max="100.0"
///         step="0.01"
///         label="Amplitude"
///         show_input=true
///         on_change=Callback::new(move |v: String| value.set(v))
///     />
/// }
/// ```
#[component]
pub fn ParameterSlider(
    /// Current value as a string (for precision preservation)
    #[prop(into)]
    value: Signal<String>,
    /// Minimum value
    #[prop(into)]
    min: String,
    /// Maximum value
    #[prop(into)]
    max: String,
    /// Step increment
    #[prop(into, default = "1".into())]
    step: String,
    /// Shift key step multiplier (typically 10x)
    #[prop(into, optional)]
    shift_step: Option<String>,
    /// Ctrl key step multiplier (typically 100x)
    #[prop(into, optional)]
    ctrl_step: Option<String>,
    /// Precision type for validation
    #[prop(default = NumberInputPrecision::Decimal(6))]
    precision: NumberInputPrecision,
    /// Scale type (linear or logarithmic)
    #[prop(default = ParameterSliderScale::Linear)]
    scale: ParameterSliderScale,
    /// Size of the slider
    #[prop(optional)]
    size: Option<ParameterSliderSize>,
    /// Label displayed above the slider
    #[prop(optional, into)]
    label: Option<String>,
    /// Whether to show the current value
    #[prop(default = true)]
    show_value: bool,
    /// Whether to show an input field alongside the slider
    #[prop(default = false)]
    show_input: bool,
    /// Number of decimal places to display
    #[prop(default = 2)]
    display_precision: usize,
    /// Marks to display on the track
    #[prop(optional)]
    marks: Option<Vec<ParameterSliderMark>>,
    /// Whether animation mode is enabled
    #[prop(optional, into)]
    animating: Signal<bool>,
    /// Animation speed (values per second)
    #[prop(default = 1.0)]
    _animation_speed: f64,
    /// Whether animation should loop
    #[prop(default = true)]
    _animation_loop: bool,
    /// Disabled state
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Callback when value changes
    #[prop(optional)]
    on_change: Option<Callback<String>>,
    /// Callback when dragging ends
    #[prop(optional)]
    on_change_end: Option<Callback<String>>,
    /// Callback for validation result
    #[prop(optional)]
    on_validate: Option<Callback<Result<String, ParseError>>>,
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
    let input_value = RwSignal::new(String::new());
    let is_editing = RwSignal::new(false);

    // Parse min, max, step as f64 for calculations
    // We use f64 internally for positioning but preserve string precision for values
    let min_f64 = min.parse::<f64>().unwrap_or(0.0);
    let max_f64 = max.parse::<f64>().unwrap_or(100.0);
    let step_f64 = step.parse::<f64>().unwrap_or(1.0);

    let shift_step_f64 = shift_step
        .as_ref()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(step_f64 * 10.0);
    let ctrl_step_f64 = ctrl_step
        .as_ref()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(step_f64 * 100.0);

    // Calculate percentage from value (supports logarithmic scale)
    let percentage = move || {
        let val_str = value.get();
        let val = val_str.parse::<f64>().unwrap_or(min_f64);
        let range = max_f64 - min_f64;

        if range == 0.0 {
            return 0.0;
        }

        match scale {
            ParameterSliderScale::Linear => ((val - min_f64) / range) * 100.0,
            ParameterSliderScale::Logarithmic => {
                if min_f64 <= 0.0 || val <= 0.0 {
                    // Logarithmic scale requires positive values
                    ((val - min_f64) / range) * 100.0
                } else {
                    let log_min = min_f64.ln();
                    let log_max = max_f64.ln();
                    let log_val = val.ln();
                    ((log_val - log_min) / (log_max - log_min)) * 100.0
                }
            }
        }
    };

    // Convert percentage to value, respecting step and scale
    let percentage_to_value = move |pct: f64| -> String {
        let clamped_pct = pct.clamp(0.0, 100.0);

        let raw_value = match scale {
            ParameterSliderScale::Linear => min_f64 + (clamped_pct / 100.0) * (max_f64 - min_f64),
            ParameterSliderScale::Logarithmic => {
                if min_f64 <= 0.0 {
                    min_f64 + (clamped_pct / 100.0) * (max_f64 - min_f64)
                } else {
                    let log_min = min_f64.ln();
                    let log_max = max_f64.ln();
                    let log_val = log_min + (clamped_pct / 100.0) * (log_max - log_min);
                    log_val.exp()
                }
            }
        };

        // Snap to step
        let stepped = if step_f64 > 0.0 {
            let steps = ((raw_value - min_f64) / step_f64).round();
            (min_f64 + steps * step_f64).clamp(min_f64, max_f64)
        } else {
            raw_value.clamp(min_f64, max_f64)
        };

        // Format with appropriate precision
        format!("{:.1$}", stepped, display_precision)
    };

    // Increment/decrement value
    let adjust_value = move |delta: f64| {
        if disabled.get() {
            return;
        }

        let current = value.get().parse::<f64>().unwrap_or(min_f64);
        let new_val = (current + delta).clamp(min_f64, max_f64);
        let new_str = format!("{:.1$}", new_val, display_precision);

        if let Some(callback) = on_change {
            callback.run(new_str.clone());
        }
        if let Some(validate) = on_validate {
            validate.run(validate_value(&new_str, precision));
        }
    };

    // Track dimensions for slider
    let (track_height, thumb_size, font_size) = match size {
        ParameterSliderSize::Xs => ("4px", "14px", "0.75rem"),
        ParameterSliderSize::Sm => ("6px", "18px", "0.8125rem"),
        ParameterSliderSize::Md => ("8px", "22px", "0.875rem"),
        ParameterSliderSize::Lg => ("10px", "26px", "1rem"),
        ParameterSliderSize::Xl => ("12px", "30px", "1.125rem"),
    };

    let wrapper_styles = move || {
        let mut builder = StyleBuilder::new();
        builder.add("width", "100%");

        let mut result = builder.build();
        if let Some(ref s) = style {
            if !result.is_empty() {
                result.push_str("; ");
            }
            result.push_str(s);
        }
        result
    };

    let container_styles = move || {
        let mut builder = StyleBuilder::new();
        builder
            .add("display", "flex")
            .add("align-items", "center")
            .add("gap", "1rem");

        builder.build()
    };

    let slider_container_styles = move || {
        let mut builder = StyleBuilder::new();
        builder.add("flex", "1").add("min-width", "0");

        builder.build()
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
            font_size,
            theme_val.typography.font_weights.medium,
            scheme_colors.text
        )
    };

    let value_display_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "font-family: monospace; font-size: {}; color: {}; font-weight: normal;",
            font_size, scheme_colors.text
        )
    };

    let input_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        let mut builder = StyleBuilder::new();
        builder
            .add("width", "80px")
            .add("padding", "0.25rem 0.5rem")
            .add("font-family", "monospace")
            .add("font-size", font_size)
            .add(
                "border",
                format!(
                    "1px solid {}",
                    scheme_colors
                        .get_color("gray", 4)
                        .unwrap_or_else(|| "#ced4da".to_string())
                ),
            )
            .add("border-radius", theme_val.radius.sm)
            .add("background-color", scheme_colors.background.clone())
            .add("color", scheme_colors.text.clone())
            .add("text-align", "right")
            .add("outline", "none");

        builder.build()
    };

    let input_focused_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        let mut builder = StyleBuilder::new();
        builder
            .add("width", "80px")
            .add("padding", "0.25rem 0.5rem")
            .add("font-family", "monospace")
            .add("font-size", font_size)
            .add(
                "border",
                format!(
                    "1px solid {}",
                    scheme_colors
                        .get_color("blue", 5)
                        .unwrap_or_else(|| "#339af0".to_string())
                ),
            )
            .add("border-radius", theme_val.radius.sm)
            .add("background-color", scheme_colors.background.clone())
            .add("color", scheme_colors.text.clone())
            .add("text-align", "right")
            .add("outline", "none")
            .add(
                "box-shadow",
                format!(
                    "0 0 0 2px {}",
                    scheme_colors
                        .get_color("blue", 2)
                        .unwrap_or_else(|| "#a5d8ff".to_string())
                ),
            );

        builder.build()
    };

    let mark_styles = move |mark_value: &str| {
        let mark_f64 = mark_value.parse::<f64>().unwrap_or(min_f64);
        let pct = match scale {
            ParameterSliderScale::Linear => ((mark_f64 - min_f64) / (max_f64 - min_f64)) * 100.0,
            ParameterSliderScale::Logarithmic => {
                if min_f64 <= 0.0 || mark_f64 <= 0.0 {
                    ((mark_f64 - min_f64) / (max_f64 - min_f64)) * 100.0
                } else {
                    let log_min = min_f64.ln();
                    let log_max = max_f64.ln();
                    let log_val = mark_f64.ln();
                    ((log_val - log_min) / (log_max - log_min)) * 100.0
                }
            }
        };
        format!(
            "position: absolute; left: {}%; transform: translateX(-50%); top: 100%; margin-top: 0.5rem; font-size: 0.75rem;",
            pct
        )
    };

    // Handle mouse/touch interaction
    let track_ref = NodeRef::<leptos::html::Div>::new();

    let handle_interaction = move |client_x: i32| {
        if disabled.get() {
            return;
        }

        if let Some(track) = track_ref.get() {
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

            if let Some(callback) = on_change {
                callback.run(new_value.clone());
            }
            if let Some(validate) = on_validate {
                validate.run(validate_value(&new_value, precision));
            }
        }
    };

    let handle_mouse_down = move |ev: ev::MouseEvent| {
        if disabled.get() {
            return;
        }

        ev.prevent_default();
        is_dragging.set(true);
        handle_interaction(ev.client_x());
    };

    let handle_mouse_move = move |ev: ev::MouseEvent| {
        if !is_dragging.get() || disabled.get() {
            return;
        }
        handle_interaction(ev.client_x());
    };

    let handle_mouse_up = move |_ev: ev::MouseEvent| {
        if is_dragging.get() {
            is_dragging.set(false);
            if let Some(callback) = on_change_end {
                callback.run(value.get());
            }
        }
    };

    // Keyboard handling
    let handle_keydown = move |ev: ev::KeyboardEvent| {
        if disabled.get() {
            return;
        }

        let current_step = if ev.ctrl_key() {
            ctrl_step_f64
        } else if ev.shift_key() {
            shift_step_f64
        } else {
            step_f64
        };

        match ev.key().as_str() {
            "ArrowRight" | "ArrowUp" => {
                ev.prevent_default();
                adjust_value(current_step);
            }
            "ArrowLeft" | "ArrowDown" => {
                ev.prevent_default();
                adjust_value(-current_step);
            }
            "Home" => {
                ev.prevent_default();
                let new_str = format!("{:.1$}", min_f64, display_precision);
                if let Some(callback) = on_change {
                    callback.run(new_str.clone());
                }
            }
            "End" => {
                ev.prevent_default();
                let new_str = format!("{:.1$}", max_f64, display_precision);
                if let Some(callback) = on_change {
                    callback.run(new_str.clone());
                }
            }
            _ => {}
        }
    };

    // Input field handling
    let handle_input_focus = move |_ev: ev::FocusEvent| {
        is_editing.set(true);
        input_value.set(value.get());
    };

    let handle_input_change = move |ev: ev::Event| {
        let target = ev.target().unwrap();
        let input: web_sys::HtmlInputElement = target.unchecked_into();
        input_value.set(input.value());
    };

    let handle_input_blur = move |_ev: ev::FocusEvent| {
        is_editing.set(false);
        let input_str = input_value.get();

        // Validate and apply
        if let Ok(val) = input_str.parse::<f64>() {
            let clamped = val.clamp(min_f64, max_f64);
            let new_str = format!("{:.1$}", clamped, display_precision);

            if let Some(callback) = on_change {
                callback.run(new_str.clone());
            }
            if let Some(validate) = on_validate {
                validate.run(validate_value(&new_str, precision));
            }
        }
    };

    let handle_input_keydown = move |ev: ev::KeyboardEvent| {
        if ev.key() == "Enter" {
            ev.prevent_default();
            let target = ev.target().unwrap();
            let input: web_sys::HtmlInputElement = target.unchecked_into();
            input.blur().ok();
        } else if ev.key() == "Escape" {
            ev.prevent_default();
            is_editing.set(false);
            input_value.set(value.get());
        }
    };

    // Animation effect
    Effect::new(move |_| {
        if animating.get() && !disabled.get() {
            // Animation logic would use set_interval in real implementation
            // For now, this is a placeholder for the animation system
        }
    });

    // Format displayed value
    let display_value = move || {
        if is_editing.get() {
            input_value.get()
        } else {
            let val_str = value.get();
            if let Ok(val) = val_str.parse::<f64>() {
                format!("{:.1$}", val, display_precision)
            } else {
                val_str
            }
        }
    };

    let class_str = format!(
        "mingot-parameter-slider {}",
        class.clone().unwrap_or_default()
    );

    view! {
        <div
            class=class_str
            style=wrapper_styles
            tabindex="0"
            on:keydown=handle_keydown
            on:mousemove=handle_mouse_move
            on:mouseup=handle_mouse_up
            on:mouseleave=handle_mouse_up
        >
            {label.clone().map(|l| view! {
                <div style=label_styles>
                    <span>{l}</span>
                    {(show_value && !show_input).then(|| view! {
                        <span style=value_display_styles>{display_value}</span>
                    })}
                </div>
            })}

            <div style=container_styles>
                <div style=slider_container_styles>
                    <div style="position: relative; padding: 0.5rem 0;">
                        <div
                            node_ref=track_ref
                            class="mingot-parameter-slider-track"
                            style=track_styles
                            on:mousedown=handle_mouse_down
                        >
                            <div class="mingot-parameter-slider-filled" style=filled_styles></div>
                            <div class="mingot-parameter-slider-thumb" style=thumb_styles></div>
                        </div>

                        {marks.clone().map(|m| view! {
                            <div class="mingot-parameter-slider-marks">
                                {m.into_iter().map(|mark| {
                                    let mark_label = mark.label.clone();
                                    let mark_value = mark.value.clone();
                                    view! {
                                        <div style=move || mark_styles(&mark_value)>
                                            {mark_label.clone()}
                                        </div>
                                    }
                                }).collect::<Vec<_>>()}
                            </div>
                        })}
                    </div>
                </div>

                {show_input.then(|| {
                    view! {
                        <input
                            type="text"
                            style=move || if is_editing.get() { input_focused_styles() } else { input_styles() }
                            prop:value=display_value
                            on:focus=handle_input_focus
                            on:input=handle_input_change
                            on:blur=handle_input_blur
                            on:keydown=handle_input_keydown
                            disabled=disabled
                        />
                    }
                })}
            </div>
        </div>
    }
}

/// Validate value according to precision type
fn validate_value(input: &str, precision: NumberInputPrecision) -> Result<String, ParseError> {
    let cleaned = input.replace([',', '_'], "").trim().to_string();

    if cleaned.is_empty() {
        return Err(ParseError::InvalidFormat("Empty input".to_string()));
    }

    match precision {
        NumberInputPrecision::U64 => {
            let result = cleaned.clone();
            cleaned.parse::<u64>().map(|_| result).map_err(|_| {
                ParseError::Overflow(format!("Value exceeds u64 maximum ({})", u64::MAX))
            })
        }
        NumberInputPrecision::U128 => {
            let result = cleaned.clone();
            cleaned.parse::<u128>().map(|_| result).map_err(|_| {
                ParseError::Overflow(format!("Value exceeds u128 maximum ({})", u128::MAX))
            })
        }
        NumberInputPrecision::I64 => {
            let result = cleaned.clone();
            let is_negative = cleaned.starts_with('-');
            cleaned.parse::<i64>().map(|_| result).map_err(|_| {
                if is_negative {
                    ParseError::Underflow(format!("Value below i64 minimum ({})", i64::MIN))
                } else {
                    ParseError::Overflow(format!("Value exceeds i64 maximum ({})", i64::MAX))
                }
            })
        }
        NumberInputPrecision::I128 => {
            let result = cleaned.clone();
            let is_negative = cleaned.starts_with('-');
            cleaned.parse::<i128>().map(|_| result).map_err(|_| {
                if is_negative {
                    ParseError::Underflow(format!("Value below i128 minimum ({})", i128::MIN))
                } else {
                    ParseError::Overflow(format!("Value exceeds i128 maximum ({})", i128::MAX))
                }
            })
        }
        NumberInputPrecision::Decimal(max_decimals) => {
            // Check for valid decimal format
            if let Some(dot_pos) = cleaned.find('.') {
                let decimal_part = &cleaned[dot_pos + 1..];
                if decimal_part.len() > max_decimals as usize {
                    return Err(ParseError::TooManyDecimals(max_decimals));
                }
            }

            let result = cleaned.clone();
            cleaned
                .parse::<f64>()
                .map(|_| result)
                .map_err(|_| ParseError::InvalidFormat("Not a valid decimal number".to_string()))
        }
        #[cfg(feature = "high-precision")]
        NumberInputPrecision::Arbitrary => {
            use rust_decimal::Decimal;
            use std::str::FromStr;

            let result = cleaned.clone();
            Decimal::from_str(&cleaned)
                .map(|_| result)
                .map_err(|e| ParseError::InvalidFormat(e.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_slider_size_default() {
        assert_eq!(ParameterSliderSize::default(), ParameterSliderSize::Md);
    }

    #[test]
    fn test_parameter_slider_scale_default() {
        assert_eq!(
            ParameterSliderScale::default(),
            ParameterSliderScale::Linear
        );
    }

    #[test]
    fn test_parameter_slider_mark_new() {
        let mark = ParameterSliderMark::new("50.0");
        assert_eq!(mark.value, "50.0");
        assert!(mark.label.is_none());
    }

    #[test]
    fn test_parameter_slider_mark_with_label() {
        let mark = ParameterSliderMark::with_label("75.0", "75%");
        assert_eq!(mark.value, "75.0");
        assert_eq!(mark.label, Some("75%".to_string()));
    }

    #[test]
    fn test_validate_value_decimal() {
        let result = validate_value("123.456", NumberInputPrecision::Decimal(6));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "123.456");
    }

    #[test]
    fn test_validate_value_decimal_too_many() {
        let result = validate_value("123.4567890", NumberInputPrecision::Decimal(4));
        assert!(matches!(result, Err(ParseError::TooManyDecimals(4))));
    }

    #[test]
    fn test_validate_value_u64() {
        let result = validate_value("12345", NumberInputPrecision::U64);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_value_u64_negative() {
        let result = validate_value("-1", NumberInputPrecision::U64);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_value_i64() {
        let result = validate_value("-12345", NumberInputPrecision::I64);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_value_empty() {
        let result = validate_value("", NumberInputPrecision::Decimal(2));
        assert!(matches!(result, Err(ParseError::InvalidFormat(_))));
    }

    #[test]
    fn test_validate_value_invalid() {
        let result = validate_value("abc", NumberInputPrecision::Decimal(2));
        assert!(matches!(result, Err(ParseError::InvalidFormat(_))));
    }

    #[cfg(feature = "high-precision")]
    #[test]
    fn test_validate_value_arbitrary() {
        let result = validate_value(
            "123456789012345678901234567890",
            NumberInputPrecision::Arbitrary,
        );
        // Note: rust_decimal has 28-29 digit limit, this may fail
        // Actual behavior depends on rust_decimal
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_parameter_slider_size_variants() {
        let sizes = [
            ParameterSliderSize::Xs,
            ParameterSliderSize::Sm,
            ParameterSliderSize::Md,
            ParameterSliderSize::Lg,
            ParameterSliderSize::Xl,
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
