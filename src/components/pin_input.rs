use crate::components::input::{InputSize, InputVariant};
use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Type of characters allowed in PinInput
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum PinInputType {
    /// Only numeric characters (0-9)
    #[default]
    Number,
    /// Only alphanumeric characters (a-z, A-Z, 0-9)
    Alphanumeric,
    /// Any character
    Text,
}

/// A PIN input component for entering verification codes, OTPs, etc.
///
/// # Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use mingot::prelude::*;
///
/// let code = RwSignal::new(String::new());
///
/// view! {
///     <PinInput
///         length=6
///         value=code
///         on_complete=Callback::new(move |v: String| {
///             // Handle complete code entry
///         })
///     />
/// }
/// ```
#[component]
pub fn PinInput(
    /// Number of input fields
    #[prop(default = 4)]
    length: usize,
    /// Current value (concatenated)
    #[prop(into)]
    value: Signal<String>,
    /// Type of allowed characters
    #[prop(optional)]
    input_type: Option<PinInputType>,
    /// Whether to mask input like a password
    #[prop(default = false)]
    mask: bool,
    /// Placeholder character for empty fields
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Size of the inputs
    #[prop(optional)]
    size: Option<InputSize>,
    /// Variant style
    #[prop(optional)]
    variant: Option<InputVariant>,
    /// Whether inputs should be disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Error state
    #[prop(optional, into)]
    error: Option<String>,
    /// Callback when a single character changes
    #[prop(optional)]
    on_change: Option<Callback<String>>,
    /// Callback when all fields are filled
    #[prop(optional)]
    on_complete: Option<Callback<String>>,
    /// Additional CSS class
    #[prop(optional, into)]
    class: Option<String>,
    /// Additional inline styles
    #[prop(optional, into)]
    style: Option<String>,
    /// Gap between inputs
    #[prop(default = "0.5rem".to_string(), into)]
    gap: String,
) -> impl IntoView {
    let theme = use_theme();
    let input_type = input_type.unwrap_or_default();
    let size = size.unwrap_or(InputSize::Md);
    let variant = variant.unwrap_or(InputVariant::Default);
    let placeholder = placeholder.unwrap_or_else(|| "○".to_string());

    // Store individual input values
    let input_values = RwSignal::new(vec!["".to_string(); length]);

    // Sync with external value signal
    Effect::new(move || {
        let current_value = value.get();
        let chars: Vec<String> = current_value
            .chars()
            .take(length)
            .map(|c| c.to_string())
            .collect();
        let mut values = vec!["".to_string(); length];
        for (i, c) in chars.into_iter().enumerate() {
            values[i] = c;
        }
        input_values.set(values);
    });

    // Create refs for each input
    let input_refs: Vec<NodeRef<leptos::html::Input>> =
        (0..length).map(|_| NodeRef::new()).collect();
    let input_refs_clone = input_refs.clone();

    // Get input dimensions based on size
    let (input_size, font_size) = match size {
        InputSize::Xs => ("1.75rem", "xs"),
        InputSize::Sm => ("2.25rem", "sm"),
        InputSize::Md => ("2.75rem", "md"),
        InputSize::Lg => ("3.25rem", "lg"),
        InputSize::Xl => ("3.75rem", "xl"),
    };

    let error_clone = error.clone();
    let make_input_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let is_disabled = disabled.get();

        let mut builder = StyleBuilder::new();
        builder
            .add("width", input_size)
            .add("height", input_size)
            .add("text-align", "center")
            .add("font-family", theme_val.typography.font_family)
            .add(
                "font-size",
                match font_size {
                    "xs" => theme_val.typography.font_sizes.xs,
                    "sm" => theme_val.typography.font_sizes.sm,
                    "md" => theme_val.typography.font_sizes.md,
                    "lg" => theme_val.typography.font_sizes.lg,
                    "xl" => theme_val.typography.font_sizes.xl,
                    _ => theme_val.typography.font_sizes.md,
                },
            )
            .add(
                "font-weight",
                theme_val.typography.font_weights.medium.to_string(),
            )
            .add("border-radius", theme_val.radius.sm)
            .add("outline", "none")
            .add("transition", "all 0.15s ease")
            .add("box-sizing", "border-box");

        // Variant-based styles
        match variant {
            InputVariant::Default => {
                let border_color = if error_clone.is_some() {
                    scheme_colors
                        .get_color("red", 6)
                        .unwrap_or_else(|| "#fa5252".to_string())
                } else {
                    scheme_colors.border.clone()
                };

                builder
                    .add("background-color", scheme_colors.background.clone())
                    .add("color", scheme_colors.text.clone())
                    .add("border", format!("1px solid {}", border_color));
            }
            InputVariant::Filled => {
                let bg_color = scheme_colors
                    .get_color("gray", 1)
                    .unwrap_or_else(|| "#f1f3f5".to_string());

                builder
                    .add("background-color", bg_color)
                    .add("color", scheme_colors.text.clone())
                    .add("border", "1px solid transparent");
            }
            InputVariant::Unstyled => {
                builder
                    .add("background-color", "transparent")
                    .add("color", scheme_colors.text.clone())
                    .add("border", "none")
                    .add(
                        "border-bottom",
                        format!("2px solid {}", scheme_colors.border),
                    );
            }
        }

        if is_disabled {
            builder.add("opacity", "0.6").add("cursor", "not-allowed");
        }

        builder.build()
    };

    let is_valid_char = move |c: char| -> bool {
        match input_type {
            PinInputType::Number => c.is_ascii_digit(),
            PinInputType::Alphanumeric => c.is_ascii_alphanumeric(),
            PinInputType::Text => true,
        }
    };

    let update_value = move |index: usize, new_char: String| {
        input_values.update(|values| {
            if index < values.len() {
                values[index] = new_char;
            }
        });

        // Build combined value
        let combined: String = input_values.get().iter().map(|s| s.as_str()).collect();

        if let Some(callback) = on_change {
            callback.run(combined.clone());
        }

        // Check if complete
        if combined.len() == length && input_values.get().iter().all(|s| !s.is_empty()) {
            if let Some(callback) = on_complete {
                callback.run(combined);
            }
        }
    };

    let focus_input = move |index: usize| {
        if index < input_refs_clone.len() {
            if let Some(input) = input_refs_clone[index].get() {
                let _ = input.focus();
                input.select();
            }
        }
    };

    let wrapper_styles = move || {
        let mut styles = format!("display: flex; gap: {}; align-items: center;", gap);
        if let Some(ref s) = style {
            styles.push_str(s);
        }
        styles
    };

    let error_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "margin-top: 0.5rem; font-size: {}; color: {};",
            theme_val.typography.font_sizes.xs,
            scheme_colors
                .get_color("red", 6)
                .unwrap_or_else(|| "#fa5252".to_string())
        )
    };

    let class_str = format!("mingot-pin-input {}", class.unwrap_or_default());

    view! {
        <div class=class_str>
            <div style=wrapper_styles>
                {input_refs.into_iter().enumerate().map(|(index, node_ref)| {
                    let input_styles = make_input_styles.clone();
                    let focus_input_for_input = focus_input.clone();
                    let focus_input_for_keydown = focus_input.clone();

                    let handle_input = move |ev: ev::Event| {
                        if disabled.get() {
                            return;
                        }

                        let target = ev.target();
                        let input: web_sys::HtmlInputElement = target
                            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
                            .expect("Expected HtmlInputElement");

                        let input_value = input.value();

                        // Handle paste of multiple characters
                        if input_value.len() > 1 {
                            let chars: Vec<char> = input_value
                                .chars()
                                .filter(|&c| is_valid_char(c))
                                .take(length - index)
                                .collect();

                            for (i, c) in chars.iter().enumerate() {
                                update_value(index + i, c.to_string());
                            }

                            let next_index = (index + chars.len()).min(length - 1);
                            focus_input_for_input(next_index);
                            return;
                        }

                        // Single character input
                        if let Some(c) = input_value.chars().last() {
                            if is_valid_char(c) {
                                update_value(index, c.to_string());
                                if index < length - 1 {
                                    focus_input_for_input(index + 1);
                                }
                            } else {
                                // Clear invalid input
                                input.set_value(&input_values.get()[index]);
                            }
                        } else {
                            // Input was cleared
                            update_value(index, String::new());
                        }
                    };

                    let handle_keydown = move |ev: ev::KeyboardEvent| {
                        if disabled.get() {
                            return;
                        }

                        let key = ev.key();

                        match key.as_str() {
                            "Backspace" => {
                                let current_value = &input_values.get()[index];
                                if current_value.is_empty() && index > 0 {
                                    // Move to previous input and clear it
                                    update_value(index - 1, String::new());
                                    focus_input_for_keydown(index - 1);
                                } else {
                                    // Clear current input
                                    update_value(index, String::new());
                                }
                            }
                            "ArrowLeft" => {
                                if index > 0 {
                                    focus_input_for_keydown(index - 1);
                                }
                            }
                            "ArrowRight" => {
                                if index < length - 1 {
                                    focus_input_for_keydown(index + 1);
                                }
                            }
                            _ => {}
                        }
                    };

                    let handle_focus = move |ev: ev::FocusEvent| {
                        let target = ev.target();
                        if let Some(input) = target.and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok()) {
                            input.select();
                        }
                    };

                    let display_value = move || {
                        let values = input_values.get();
                        let val = values.get(index).cloned().unwrap_or_default();
                        if mask && !val.is_empty() {
                            "●".to_string()
                        } else {
                            val
                        }
                    };

                    view! {
                        <input
                            node_ref=node_ref
                            type="text"
                            inputmode=if matches!(input_type, PinInputType::Number) { "numeric" } else { "text" }
                            maxlength="1"
                            autocomplete="one-time-code"
                            class="mingot-pin-input-field"
                            style=input_styles
                            placeholder=placeholder.clone()
                            disabled=move || disabled.get()
                            prop:value=display_value
                            on:input=handle_input
                            on:keydown=handle_keydown
                            on:focus=handle_focus
                        />
                    }
                }).collect::<Vec<_>>()}
            </div>

            {error.map(|e| view! {
                <div style=error_styles>{e}</div>
            })}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pin_input_type_default() {
        assert_eq!(PinInputType::default(), PinInputType::Number);
    }

    #[test]
    fn test_pin_input_type_variants() {
        let types = [
            PinInputType::Number,
            PinInputType::Alphanumeric,
            PinInputType::Text,
        ];
        for (i, t1) in types.iter().enumerate() {
            for (j, t2) in types.iter().enumerate() {
                if i != j {
                    assert_ne!(t1, t2);
                }
            }
        }
    }
}
