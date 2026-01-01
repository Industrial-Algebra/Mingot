mod app;
mod components;
mod docs;
mod layout;
mod pages;

use wasm_bindgen::JsCast;

fn main() {
    // Set up panic hook for better error messages in browser console
    console_error_panic_hook::set_once();

    // Mount the app to #app, replacing the loading message
    let app_element = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.get_element_by_id("app"))
        .and_then(|e| e.dyn_into::<web_sys::HtmlElement>().ok())
        .expect("should find #app element");

    // Clear the loading message before mounting
    app_element.set_inner_html("");

    leptos::mount::mount_to(app_element, app::App).forget();
}
