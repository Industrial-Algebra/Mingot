mod app;
mod components;
mod docs;
mod layout;
mod pages;

fn main() {
    // Set up panic hook for better error messages in browser console
    console_error_panic_hook::set_once();

    // Mount the app
    leptos::mount::mount_to_body(app::App);
}
