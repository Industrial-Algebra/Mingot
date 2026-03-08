use leptos::prelude::*;
use mingot::prelude::*;
use mingot::theme::use_theme;
use wasm_bindgen::JsCast;

/// Component that injects demo-specific CSS variables based on the current theme.
/// Color/surface/primary/semantic vars are now handled by MingotProvider's --mingot-* injection.
/// This only sets demo-specific vars: --code-bg, --code-inline-bg, --success-light.
#[component]
pub fn ThemeVars() -> impl IntoView {
    let theme = use_theme();

    let _ = Effect::new(move || {
        let theme_val = theme.get();
        let is_dark = theme_val.color_scheme == ColorSchemeMode::Dark;

        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(root) = document.document_element() {
                    if let Some(style) = root.dyn_ref::<web_sys::HtmlElement>() {
                        let css_style = style.style();

                        // Code block backgrounds (demo-specific)
                        if is_dark {
                            let _ = css_style.set_property("--code-bg", "#1e1e1e");
                            let _ = css_style.set_property("--code-inline-bg", "#373a40");
                        } else {
                            let _ = css_style.set_property("--code-bg", "#1e1e1e");
                            let _ = css_style.set_property("--code-inline-bg", "#f1f3f5");
                        }

                        // Success light variant (demo-specific)
                        let scheme_colors = mingot::theme::get_scheme_colors(&theme_val);
                        if let Some(success_light) = scheme_colors.get_color("green", 1) {
                            let _ = css_style.set_property("--success-light", &success_light);
                        }
                    }
                }
            }
        }
    });
}
