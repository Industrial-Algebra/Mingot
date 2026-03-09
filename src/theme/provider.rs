use super::{ColorSchemeMode, Theme, ThemeContext};
use leptos::prelude::*;

#[cfg(target_arch = "wasm32")]
use super::theme_to_css_vars;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;

#[component]
pub fn MingotProvider(
    #[prop(optional)] theme: Option<Theme>,
    /// Whether to inject CSS custom properties on the document root element.
    /// Defaults to `true`. Set to `false` if you manage CSS variables externally.
    #[prop(optional, default = true)]
    inject_css_vars: bool,
    children: Children,
) -> impl IntoView {
    let theme = theme.unwrap_or_default();
    let theme_signal = RwSignal::new(theme);

    provide_context::<ThemeContext>(theme_signal);

    // Inject CSS custom properties onto the document root element
    #[cfg(target_arch = "wasm32")]
    if inject_css_vars {
        let _ = Effect::new(move || {
            let theme_val = theme_signal.get();
            let vars = theme_to_css_vars(&theme_val);

            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(root) = document.document_element() {
                        if let Some(el) = root.dyn_ref::<web_sys::HtmlElement>() {
                            let style = el.style();
                            for (name, value) in &vars {
                                let _ = style.set_property(name, value);
                            }
                        }
                    }
                }
            }
        });
    }

    // Listen for system color scheme changes when Auto mode is active.
    // When the OS preference changes, we nudge the theme signal so that
    // `ColorSchemeMode::Auto.resolve()` picks up the new value.
    #[cfg(target_arch = "wasm32")]
    {
        let _ = Effect::new(move || {
            let theme_val = theme_signal.get();
            if theme_val.color_scheme != ColorSchemeMode::Auto {
                return;
            }

            if let Some(mql) = web_sys::window()
                .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
            {
                let cb = wasm_bindgen::closure::Closure::<dyn Fn(web_sys::Event)>::new(
                    move |_: web_sys::Event| {
                        // Trigger a reactive update so dependents re-evaluate resolve()
                        theme_signal.update(|_| {});
                    },
                );
                let _ = mql.add_event_listener_with_callback("change", cb.as_ref().unchecked_ref());
                // Leak the closure so it lives for the duration of the page.
                // This is fine because the listener is global and long-lived.
                cb.forget();
            }
        });
    }

    // Suppress unused variable warning in non-wasm builds
    #[cfg(not(target_arch = "wasm32"))]
    let _ = inject_css_vars;

    // Apply background color and text color based on theme
    let root_style = move || {
        let theme = theme_signal.get();
        let colors = super::get_scheme_colors(&theme);
        format!(
            "background-color: {}; color: {}; min-height: 100vh;",
            colors.background, colors.text
        )
    };

    view! {
        <div class="mingot-provider" style=root_style>
            {children()}
        </div>
    }
}

pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().expect("use_theme must be used within a MingotProvider")
}

/// Hook to get a function to toggle the color scheme
pub fn use_color_scheme_toggle() -> impl Fn() {
    let theme = use_theme();

    move || {
        theme.update(|t| {
            t.color_scheme = match t.color_scheme {
                ColorSchemeMode::Light => ColorSchemeMode::Dark,
                ColorSchemeMode::Dark => ColorSchemeMode::Light,
                ColorSchemeMode::Auto => ColorSchemeMode::Dark,
            };
        });
    }
}

/// Hook to get a function to set the color scheme
pub fn use_set_color_scheme() -> impl Fn(ColorSchemeMode) {
    let theme = use_theme();

    move |mode: ColorSchemeMode| {
        theme.update(|t| {
            t.color_scheme = mode;
        });
    }
}

/// Hook to get the current active color scheme
pub fn use_color_scheme() -> impl Fn() -> ColorSchemeMode + Clone {
    let theme = use_theme();

    move || theme.get().color_scheme
}
