use crate::components::input::{InputSize, InputVariant};
use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;

/// A password input component with visibility toggle
///
/// # Example
/// ```rust,ignore
/// let password = RwSignal::new(String::new());
///
/// <PasswordInput
///     value=password.into()
///     label="Password"
///     placeholder="Enter your password"
///     on_input=Callback::new(move |val| password.set(val))
/// />
/// ```
#[component]
pub fn PasswordInput(
    /// Input variant style
    #[prop(optional)]
    variant: Option<InputVariant>,
    /// Input size
    #[prop(optional)]
    size: Option<InputSize>,
    /// Placeholder text
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Current value (reactive)
    #[prop(optional, into)]
    value: Signal<String>,
    /// Whether the input is disabled
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Error message to display
    #[prop(optional, into)]
    error: Option<String>,
    /// Whether the field is required
    #[prop(optional)]
    required: bool,
    /// Callback fired on input
    #[prop(optional)]
    on_input: Option<Callback<String>>,
    /// Callback fired on change (blur)
    #[prop(optional)]
    on_change: Option<Callback<String>>,
    /// Additional CSS classes
    #[prop(optional, into)]
    class: Option<String>,
    /// Additional inline styles
    #[prop(optional, into)]
    style: Option<String>,
    /// Label text
    #[prop(optional, into)]
    label: Option<String>,
    /// Description text
    #[prop(optional, into)]
    description: Option<String>,
    /// Autocomplete attribute
    #[prop(optional, into)]
    autocomplete: Option<String>,
    /// Whether password is initially visible
    #[prop(optional)]
    visible: bool,
    /// Whether to show the visibility toggle button (default: true)
    #[prop(optional)]
    toggle_visibility: Option<bool>,
) -> impl IntoView {
    let theme = use_theme();
    let variant = variant.unwrap_or(InputVariant::Default);
    let size = size.unwrap_or(InputSize::Md);
    let toggle_visibility = toggle_visibility.unwrap_or(true);

    // State for password visibility
    let (is_visible, set_is_visible) = signal(visible);

    let error_clone = error.clone();
    let input_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let mut builder = StyleBuilder::new();
        let is_disabled = disabled.get();

        // Base styles
        builder
            .add("width", "100%")
            .add("font-family", theme_val.typography.font_family)
            .add("border-radius", theme_val.radius.sm)
            .add("transition", "all 0.15s ease")
            .add("outline", "none")
            .add("box-sizing", "border-box")
            .add("padding-right", "2.5rem"); // Space for toggle button

        // Size-based styles
        match size {
            InputSize::Xs => {
                builder
                    .add("height", "1.875rem")
                    .add("padding-left", "0.625rem")
                    .add("font-size", theme_val.typography.font_sizes.xs);
            }
            InputSize::Sm => {
                builder
                    .add("height", "2.25rem")
                    .add("padding-left", "0.75rem")
                    .add("font-size", theme_val.typography.font_sizes.sm);
            }
            InputSize::Md => {
                builder
                    .add("height", "2.625rem")
                    .add("padding-left", "0.875rem")
                    .add("font-size", theme_val.typography.font_sizes.sm);
            }
            InputSize::Lg => {
                builder
                    .add("height", "3.125rem")
                    .add("padding-left", "1rem")
                    .add("font-size", theme_val.typography.font_sizes.md);
            }
            InputSize::Xl => {
                builder
                    .add("height", "3.75rem")
                    .add("padding-left", "1.125rem")
                    .add("font-size", theme_val.typography.font_sizes.lg);
            }
        }

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
                    .add("padding", "0");
            }
        }

        // Disabled state
        if is_disabled {
            builder.add("opacity", "0.6").add("cursor", "not-allowed");
        } else {
            builder.add("cursor", "text");
        }

        if let Some(s) = style.as_ref() {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let toggle_button_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        format!(
            "position: absolute; \
             right: 0.5rem; \
             top: 50%; \
             transform: translateY(-50%); \
             background: none; \
             border: none; \
             cursor: pointer; \
             padding: 0.25rem; \
             display: flex; \
             align-items: center; \
             justify-content: center; \
             color: {}; \
             opacity: 0.7; \
             transition: opacity 0.15s ease;",
            scheme_colors
                .get_color("gray", 6)
                .unwrap_or_else(|| "#868e96".to_string())
        )
    };

    let handle_input = move |ev: ev::Event| {
        let input_value = event_target_value(&ev);
        if let Some(callback) = on_input {
            callback.run(input_value);
        }
    };

    let handle_change = move |ev: ev::Event| {
        let change_value = event_target_value(&ev);
        if let Some(callback) = on_change {
            callback.run(change_value);
        }
    };

    let toggle_password = move |_: ev::MouseEvent| {
        set_is_visible.update(|v| *v = !*v);
    };

    let label_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "display: block; margin-bottom: 0.25rem; font-size: {}; font-weight: {}; color: {};",
            theme_val.typography.font_sizes.sm,
            theme_val.typography.font_weights.medium,
            scheme_colors.text
        )
    };

    let description_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "margin-top: 0.25rem; font-size: {}; color: {};",
            theme_val.typography.font_sizes.xs,
            scheme_colors
                .get_color("gray", 6)
                .unwrap_or_else(|| "#868e96".to_string())
        )
    };

    let error_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "margin-top: 0.25rem; font-size: {}; color: {};",
            theme_val.typography.font_sizes.xs,
            scheme_colors
                .get_color("red", 6)
                .unwrap_or_else(|| "#fa5252".to_string())
        )
    };

    let class_str = format!("mingot-password-input {}", class.unwrap_or_default());

    // Eye icons as inline SVG
    let eye_open_icon = r#"<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z"></path><circle cx="12" cy="12" r="3"></circle></svg>"#;

    let eye_closed_icon = r#"<svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"></path><line x1="1" y1="1" x2="23" y2="23"></line></svg>"#;

    view! {
        <div class="mingot-password-input-wrapper" style="width: 100%;">
            {label.map(|l| {
                view! {
                    <label style=label_styles>
                        {l}
                        {if required { " *" } else { "" }}
                    </label>
                }
            })}

            <div style="position: relative; width: 100%;">
                <input
                    type=move || if is_visible.get() { "text" } else { "password" }
                    class=class_str
                    style=input_styles
                    placeholder=placeholder.unwrap_or_default()
                    disabled=move || disabled.get()
                    required=required
                    prop:value=move || value.get()
                    on:input=handle_input
                    on:change=handle_change
                    autocomplete=autocomplete.unwrap_or_else(|| "current-password".to_string())
                />

                {toggle_visibility.then(|| {
                    view! {
                        <button
                            type="button"
                            style=toggle_button_styles
                            on:click=toggle_password
                            tabindex=-1
                            aria-label=move || {
                                if is_visible.get() { "Hide password" } else { "Show password" }
                            }
                        >

                            <span inner_html=move || {
                                if is_visible.get() { eye_closed_icon } else { eye_open_icon }
                            }></span>
                        </button>
                    }
                })}
            </div>

            {description.map(|d| view! { <div style=description_styles>{d}</div> })}

            {error.map(|e| view! { <div style=error_styles>{e}</div> })}
        </div>
    }
}
