use leptos::prelude::*;
use mingot::prelude::*;
use mingot::theme::{get_scheme_colors, use_theme};
use wasm_bindgen::JsCast;

/// Component that injects CSS variables based on the current theme
/// This bridges Mingot's theme system with CSS variables for styling
#[component]
pub fn ThemeVars() -> impl IntoView {
    let theme = use_theme();

    // Effect to update CSS variables when theme changes
    let _ = Effect::new(move || {
        let theme_val = theme.get();
        let scheme_colors = get_scheme_colors(&theme_val);
        let is_dark = theme_val.color_scheme == ColorSchemeMode::Dark;

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(root) = document.document_element() {
                    if let Some(style) = root.dyn_ref::<web_sys::HtmlElement>() {
                        let css_style = style.style();

                        // Core colors
                        let _ = css_style.set_property("--background", &scheme_colors.background);
                        let _ = css_style.set_property("--text-color", &scheme_colors.text);
                        let _ = css_style.set_property("--border-color", &scheme_colors.border);
                        let _ = css_style.set_property("--white", &scheme_colors.white);
                        let _ = css_style.set_property("--black", &scheme_colors.black);

                        // Surface colors - need different indices for dark mode to create contrast
                        // In dark mode, gray[0] is almost same as background, so use higher indices
                        if is_dark {
                            // Dark mode: use elevated surfaces for contrast
                            if let Some(surface) = scheme_colors.get_color("gray", 1) {
                                let _ = css_style.set_property("--surface", &surface);
                            }
                            if let Some(surface1) = scheme_colors.get_color("gray", 2) {
                                let _ = css_style.set_property("--surface-1", &surface1);
                            }
                            if let Some(surface2) = scheme_colors.get_color("gray", 3) {
                                let _ = css_style.set_property("--surface-2", &surface2);
                            }
                            // Hover background slightly lighter
                            if let Some(hover) = scheme_colors.get_color("gray", 3) {
                                let _ = css_style.set_property("--hover-bg", &hover);
                            }
                        } else {
                            // Light mode: standard surfaces
                            if let Some(surface) = scheme_colors.get_color("gray", 0) {
                                let _ = css_style.set_property("--surface", &surface);
                            }
                            if let Some(surface1) = scheme_colors.get_color("gray", 1) {
                                let _ = css_style.set_property("--surface-1", &surface1);
                            }
                            if let Some(surface2) = scheme_colors.get_color("gray", 2) {
                                let _ = css_style.set_property("--surface-2", &surface2);
                            }
                            if let Some(hover) = scheme_colors.get_color("gray", 1) {
                                let _ = css_style.set_property("--hover-bg", &hover);
                            }
                        }

                        // Text variants - in dark mode use lighter grays
                        if is_dark {
                            if let Some(dimmed) = scheme_colors.get_color("gray", 7) {
                                let _ = css_style.set_property("--text-dimmed", &dimmed);
                            }
                        } else {
                            if let Some(dimmed) = scheme_colors.get_color("gray", 6) {
                                let _ = css_style.set_property("--text-dimmed", &dimmed);
                            }
                        }

                        // Primary colors
                        if let Some(primary) = scheme_colors.get_color("blue", 6) {
                            let _ = css_style.set_property("--primary", &primary);
                        }
                        if let Some(primary_light) = scheme_colors.get_color("blue", 0) {
                            let _ = css_style.set_property("--primary-light", &primary_light);
                        }

                        // Success colors
                        if let Some(success) = scheme_colors.get_color("green", 6) {
                            let _ = css_style.set_property("--success", &success);
                        }
                        if let Some(success_light) = scheme_colors.get_color("green", 1) {
                            let _ = css_style.set_property("--success-light", &success_light);
                        }

                        // Error colors
                        if let Some(error) = scheme_colors.get_color("red", 6) {
                            let _ = css_style.set_property("--error", &error);
                        }

                        // Warning colors
                        if let Some(warning) = scheme_colors.get_color("yellow", 6) {
                            let _ = css_style.set_property("--warning", &warning);
                        }

                        // Code block backgrounds
                        if is_dark {
                            let _ = css_style.set_property("--code-bg", "#1e1e1e");
                            let _ = css_style.set_property("--code-inline-bg", "#373a40");
                        } else {
                            let _ = css_style.set_property("--code-bg", "#1e1e1e");
                            let _ = css_style.set_property("--code-inline-bg", "#f1f3f5");
                        }
                    }
                }
            }
        }
    });

    // Return empty - this component only has side effects
    view! {}
}
