mod app;
mod components;
mod docs;
mod layout;
mod pages;

fn main() {
    // Set up panic hook for better error messages in browser console
    console_error_panic_hook::set_once();

    // Debug: log that we're starting
    web_sys::console::log_1(&"Mingot demo: main() starting".into());

    // Mount the app
    leptos::mount::mount_to_body(app::App);

    // Debug: log that mount completed
    web_sys::console::log_1(&"Mingot demo: mount_to_body completed".into());
}
