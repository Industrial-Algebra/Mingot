use crate::components::input::{InputSize, InputVariant};
use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Information about a selected file
#[derive(Clone, Debug)]
pub struct FileInfo {
    pub name: String,
    pub size: u64,
    pub file_type: String,
}

impl FileInfo {
    pub fn from_file(file: &web_sys::File) -> Self {
        Self {
            name: file.name(),
            size: file.size() as u64,
            file_type: file.type_(),
        }
    }

    /// Format file size in human-readable form
    pub fn formatted_size(&self) -> String {
        if self.size < 1024 {
            format!("{} B", self.size)
        } else if self.size < 1024 * 1024 {
            format!("{:.1} KB", self.size as f64 / 1024.0)
        } else if self.size < 1024 * 1024 * 1024 {
            format!("{:.1} MB", self.size as f64 / (1024.0 * 1024.0))
        } else {
            format!("{:.1} GB", self.size as f64 / (1024.0 * 1024.0 * 1024.0))
        }
    }
}

/// A file input component for uploading files.
///
/// # Example
/// ```rust,ignore
/// use leptos::prelude::*;
/// use mingot::prelude::*;
///
/// let files = RwSignal::new(Vec::<FileInfo>::new());
///
/// view! {
///     <FileInput
///         label="Upload files"
///         accept=".pdf,.doc,.docx"
///         multiple=true
///         on_change=Callback::new(move |f: Vec<FileInfo>| files.set(f))
///     />
/// }
/// ```
#[component]
pub fn FileInput(
    /// Label displayed above the input
    #[prop(optional, into)]
    label: Option<String>,
    /// Description text below the input
    #[prop(optional, into)]
    description: Option<String>,
    /// Placeholder text when no file is selected
    #[prop(optional, into)]
    placeholder: Option<String>,
    /// Accepted file types (e.g., ".pdf,.doc" or "image/*")
    #[prop(optional, into)]
    accept: Option<String>,
    /// Whether multiple files can be selected
    #[prop(default = false)]
    multiple: bool,
    /// Whether to show a clear button
    #[prop(default = true)]
    clearable: bool,
    /// Input variant style
    #[prop(optional)]
    variant: Option<InputVariant>,
    /// Input size
    #[prop(optional)]
    size: Option<InputSize>,
    /// Disabled state
    #[prop(optional, into)]
    disabled: Signal<bool>,
    /// Error message
    #[prop(optional, into)]
    error: Option<String>,
    /// Required field
    #[prop(default = false)]
    required: bool,
    /// Callback when files are selected
    #[prop(optional)]
    on_change: Option<Callback<Vec<FileInfo>>>,
    /// Additional CSS class
    #[prop(optional, into)]
    class: Option<String>,
    /// Additional inline styles
    #[prop(optional, into)]
    style: Option<String>,
) -> impl IntoView {
    let theme = use_theme();
    let variant = variant.unwrap_or(InputVariant::Default);
    let size = size.unwrap_or(InputSize::Md);
    let placeholder = placeholder.unwrap_or_else(|| {
        if multiple {
            "Choose files...".to_string()
        } else {
            "Choose file...".to_string()
        }
    });

    let selected_files = RwSignal::new(Vec::<FileInfo>::new());
    let input_ref = NodeRef::<leptos::html::Input>::new();

    let error_clone = error.clone();
    let input_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        let is_disabled = disabled.get();

        let mut builder = StyleBuilder::new();

        // Base styles
        builder
            .add("display", "flex")
            .add("align-items", "center")
            .add("gap", "0.5rem")
            .add("width", "100%")
            .add("font-family", theme_val.typography.font_family)
            .add("border-radius", theme_val.radius.sm)
            .add("transition", "all 0.15s ease")
            .add("box-sizing", "border-box")
            .add(
                "cursor",
                if is_disabled {
                    "not-allowed"
                } else {
                    "pointer"
                },
            );

        // Size-based styles
        match size {
            InputSize::Xs => {
                builder
                    .add("min-height", "1.875rem")
                    .add("padding", "0 0.625rem")
                    .add("font-size", theme_val.typography.font_sizes.xs);
            }
            InputSize::Sm => {
                builder
                    .add("min-height", "2.25rem")
                    .add("padding", "0 0.75rem")
                    .add("font-size", theme_val.typography.font_sizes.sm);
            }
            InputSize::Md => {
                builder
                    .add("min-height", "2.625rem")
                    .add("padding", "0 0.875rem")
                    .add("font-size", theme_val.typography.font_sizes.sm);
            }
            InputSize::Lg => {
                builder
                    .add("min-height", "3.125rem")
                    .add("padding", "0 1rem")
                    .add("font-size", theme_val.typography.font_sizes.md);
            }
            InputSize::Xl => {
                builder
                    .add("min-height", "3.75rem")
                    .add("padding", "0 1.125rem")
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
            builder.add("opacity", "0.6");
        }

        if let Some(ref s) = style {
            return format!("{}; {}", builder.build(), s);
        }

        builder.build()
    };

    let handle_change = move |ev: ev::Event| {
        let target = ev.target();
        let input: web_sys::HtmlInputElement = target
            .and_then(|t| t.dyn_into::<web_sys::HtmlInputElement>().ok())
            .expect("Expected HtmlInputElement");

        let files = input.files();
        let mut file_list = Vec::new();

        if let Some(files) = files {
            for i in 0..files.length() {
                if let Some(file) = files.get(i) {
                    file_list.push(FileInfo::from_file(&file));
                }
            }
        }

        selected_files.set(file_list.clone());

        if let Some(callback) = on_change {
            callback.run(file_list);
        }
    };

    let handle_click = move |_| {
        if disabled.get() {
            return;
        }
        if let Some(input) = input_ref.get() {
            input.click();
        }
    };

    let handle_clear = move |ev: ev::MouseEvent| {
        ev.stop_propagation();
        if disabled.get() {
            return;
        }

        if let Some(input) = input_ref.get() {
            input.set_value("");
        }

        selected_files.set(Vec::new());

        if let Some(callback) = on_change {
            callback.run(Vec::new());
        }
    };

    let display_text = move || {
        let files = selected_files.get();
        if files.is_empty() {
            placeholder.clone()
        } else if files.len() == 1 {
            files[0].name.clone()
        } else {
            format!("{} files selected", files.len())
        }
    };

    let has_files = move || !selected_files.get().is_empty();

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

    let icon_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "flex-shrink: 0; color: {};",
            scheme_colors
                .get_color("gray", 6)
                .unwrap_or_else(|| "#868e96".to_string())
        )
    };

    let clear_button_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "display: flex; align-items: center; justify-content: center; padding: 0.25rem; margin-left: auto; border: none; background: transparent; cursor: pointer; border-radius: {}; color: {};",
            theme_val.radius.sm,
            scheme_colors
                .get_color("gray", 6)
                .unwrap_or_else(|| "#868e96".to_string())
        )
    };

    let class_str = format!("mingot-file-input {}", class.unwrap_or_default());

    view! {
        <div class=class_str style="width: 100%;">
            {label.map(|l| view! {
                <label style=label_styles>
                    {l}
                    {if required { " *" } else { "" }}
                </label>
            })}

            <div
                style=input_styles
                on:click=handle_click
            >
                // Upload icon
                <svg style=icon_styles width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                    <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"></path>
                    <polyline points="17 8 12 3 7 8"></polyline>
                    <line x1="12" y1="3" x2="12" y2="15"></line>
                </svg>

                // Display text
                <span style="flex: 1; overflow: hidden; text-overflow: ellipsis; white-space: nowrap;">
                    {display_text}
                </span>

                // Clear button
                {move || {
                    if clearable && has_files() && !disabled.get() {
                        Some(view! {
                            <button
                                type="button"
                                style=clear_button_styles
                                on:click=handle_clear
                                aria-label="Clear selection"
                            >
                                <svg width="14" height="14" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                                    <line x1="18" y1="6" x2="6" y2="18"></line>
                                    <line x1="6" y1="6" x2="18" y2="18"></line>
                                </svg>
                            </button>
                        })
                    } else {
                        None
                    }
                }}
            </div>

            // Hidden file input
            <input
                node_ref=input_ref
                type="file"
                accept=accept
                multiple=multiple
                disabled=move || disabled.get()
                style="display: none;"
                on:change=handle_change
            />

            {description.map(|d| view! {
                <div style=description_styles>{d}</div>
            })}

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
    fn test_file_info_formatted_size_bytes() {
        let info = FileInfo {
            name: "test.txt".to_string(),
            size: 512,
            file_type: "text/plain".to_string(),
        };
        assert_eq!(info.formatted_size(), "512 B");
    }

    #[test]
    fn test_file_info_formatted_size_kb() {
        let info = FileInfo {
            name: "test.txt".to_string(),
            size: 2048,
            file_type: "text/plain".to_string(),
        };
        assert_eq!(info.formatted_size(), "2.0 KB");
    }

    #[test]
    fn test_file_info_formatted_size_mb() {
        let info = FileInfo {
            name: "test.txt".to_string(),
            size: 5 * 1024 * 1024,
            file_type: "text/plain".to_string(),
        };
        assert_eq!(info.formatted_size(), "5.0 MB");
    }

    #[test]
    fn test_file_info_formatted_size_gb() {
        let info = FileInfo {
            name: "test.txt".to_string(),
            size: 2 * 1024 * 1024 * 1024,
            file_type: "text/plain".to_string(),
        };
        assert_eq!(info.formatted_size(), "2.0 GB");
    }
}
