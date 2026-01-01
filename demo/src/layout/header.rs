use leptos::prelude::*;
use mingot::prelude::*;
use wasm_bindgen::JsCast;

/// Site header with logo, version badge, theme toggle, and links
#[component]
pub fn Header() -> impl IntoView {
    let color_scheme = use_color_scheme();
    let color_scheme_for_effect = color_scheme.clone();
    let toggle_theme = use_color_scheme_toggle();

    // Toggle highlight.js theme when color scheme changes
    let _ = Effect::new(move || {
        let is_dark = color_scheme_for_effect() == ColorSchemeMode::Dark;
        if let Some(window) = web_sys::window() {
            let _ = js_sys::Reflect::get(&window, &"setHljsTheme".into())
                .ok()
                .and_then(|f| f.dyn_into::<js_sys::Function>().ok())
                .map(|f: js_sys::Function| f.call1(&window, &is_dark.into()));
        }
    });

    view! {
        <header class="docs-header">
            <div style="display: flex; justify-content: space-between; align-items: center; height: 100%; padding: 0 1rem; max-width: 100%; margin: 0 auto;">
                // Logo and title
                <a href="/" style="display: flex; align-items: center; gap: 0.75rem; text-decoration: none; color: inherit;">
                    <span style="font-size: 1.5rem; font-weight: 700;">
                        "Mingot"
                    </span>
                    <span style="font-size: 0.75rem; padding: 0.125rem 0.5rem; background: #228be6; color: white; border-radius: 0.25rem;">
                        "v0.3.0"
                    </span>
                </a>

                // Right side actions
                <div style="display: flex; align-items: center; gap: 0.5rem;">
                    // Theme toggle
                    <button
                        style="display: flex; align-items: center; justify-content: center; width: 2rem; height: 2rem; border: none; background: transparent; cursor: pointer; border-radius: 0.25rem; font-size: 1.25rem;"
                        on:click=move |_| toggle_theme()
                        title="Toggle theme"
                    >
                        {move || {
                            if color_scheme() == ColorSchemeMode::Dark {
                                "☀️"
                            } else {
                                "🌙"
                            }
                        }}
                    </button>

                    // GitHub link
                    <a
                        href="https://github.com/Industrial-Algebra/Mingot"
                        target="_blank"
                        rel="noopener noreferrer"
                        style="display: flex; align-items: center; justify-content: center; width: 2rem; height: 2rem; color: inherit; text-decoration: none; border-radius: 0.25rem;"
                        title="View on GitHub"
                    >
                        <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
                            <path d="M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"/>
                        </svg>
                    </a>

                    // crates.io link
                    <a
                        href="https://crates.io/crates/mingot"
                        target="_blank"
                        rel="noopener noreferrer"
                        style="display: flex; align-items: center; justify-content: center; padding: 0.25rem 0.5rem; color: inherit; text-decoration: none; border-radius: 0.25rem; font-size: 0.75rem; font-weight: 500; border: 1px solid currentColor;"
                        title="View on crates.io"
                    >
                        "crates.io"
                    </a>
                </div>
            </div>
        </header>
    }
}
