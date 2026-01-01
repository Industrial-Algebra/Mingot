use leptos::prelude::*;
use wasm_bindgen::JsCast;

/// Button to copy text to clipboard
#[component]
pub fn CopyButton(text: &'static str) -> impl IntoView {
    let copied = RwSignal::new(false);
    let text_owned = text.to_string();

    let copy_to_clipboard = move |_| {
        let text = text_owned.clone();
        if let Some(window) = web_sys::window() {
            let clipboard = window.navigator().clipboard();
            wasm_bindgen_futures::spawn_local(async move {
                let promise = clipboard.write_text(&text);
                let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
            });
            copied.set(true);

            // Reset after 2 seconds using set_timeout_with_callback
            if let Ok(func) = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                copied.set(false);
            }) as Box<dyn Fn()>)
            .into_js_value()
            .dyn_into::<js_sys::Function>()
            {
                let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(&func, 2000);
            }
        }
    };

    view! {
        <button
            class=move || if copied.get() { "copy-button copied" } else { "copy-button" }
            on:click=copy_to_clipboard
            title="Copy to clipboard"
        >
            {move || if copied.get() { "Copied!" } else { "Copy" }}
        </button>
    }
}
