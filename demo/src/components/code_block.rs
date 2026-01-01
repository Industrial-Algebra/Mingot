use leptos::prelude::*;
use wasm_bindgen::JsCast;

use super::CopyButton;

/// Syntax-highlighted code block using highlight.js
#[component]
pub fn CodeBlock(
    code: &'static str,
    #[prop(optional)] language: Option<&'static str>,
) -> impl IntoView {
    let language = language.unwrap_or("rust");

    // Trigger highlight.js after render
    let _ = Effect::new(move || {
        if let Some(window) = web_sys::window() {
            let _ = js_sys::Reflect::get(&window, &"highlightAllCode".into())
                .ok()
                .and_then(|f| f.dyn_into::<js_sys::Function>().ok())
                .map(|f: js_sys::Function| f.call0(&window));
        }
    });

    view! {
        <div class="code-block-wrapper" style="position: relative; margin: 1rem 0;">
            <div style="position: absolute; top: 0.5rem; right: 0.5rem; z-index: 1;">
                <CopyButton text=code />
            </div>
            <pre style="margin: 0; border-radius: 0.375rem; overflow-x: auto;">
                <code class=format!("language-{}", language)>
                    {code}
                </code>
            </pre>
        </div>
    }
}
