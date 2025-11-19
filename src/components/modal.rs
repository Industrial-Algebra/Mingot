use crate::theme::use_theme;
use leptos::ev;
use leptos::prelude::*;
use wasm_bindgen::JsCast;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ModalSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Full,
}

impl ModalSize {
    fn max_width(&self) -> &'static str {
        match self {
            ModalSize::Xs => "320px",
            ModalSize::Sm => "480px",
            ModalSize::Md => "640px",
            ModalSize::Lg => "800px",
            ModalSize::Xl => "1024px",
            ModalSize::Full => "100%",
        }
    }
}

#[component]
pub fn Modal<F>(
    #[prop(into)] opened: Signal<bool>,
    #[prop(optional)] on_close: Option<F>,
    #[prop(optional)] size: Option<ModalSize>,
    #[prop(optional, into)] title: Option<String>,
    #[prop(optional)] centered: bool,
    #[prop(optional)] close_on_click_outside: bool,
    #[prop(optional)] _close_on_escape: bool,
    #[prop(optional)] with_close_button: bool,
    #[prop(optional, into)] padding: Option<String>,
    children: Children,
) -> impl IntoView
where
    F: Fn() + Copy + Send + Sync + 'static,
{
    let theme = use_theme();
    let size = size.unwrap_or(ModalSize::Md);

    let overlay_styles = move || {
        let _theme_val = theme.get();

        let display = if opened.get() { "flex" } else { "none" };
        let align = if centered { "center" } else { "flex-start" };
        let padding_top = if centered { "0" } else { "5vh" };

        format!(
            "position: fixed; \
             top: 0; \
             left: 0; \
             right: 0; \
             bottom: 0; \
             background-color: rgba(0, 0, 0, 0.75); \
             display: {}; \
             align-items: {}; \
             justify-content: center; \
             padding: {} 1rem 1rem 1rem; \
             z-index: 1000; \
             overflow-y: auto;",
            display, align, padding_top
        )
    };

    let modal_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);

        let padding_val = padding.as_deref().unwrap_or(theme_val.spacing.lg);

        format!(
            "position: relative; \
             background-color: {}; \
             border-radius: {}; \
             box-shadow: {}; \
             width: 100%; \
             max-width: {}; \
             max-height: 90vh; \
             overflow-y: auto; \
             padding: {}; \
             margin: auto;",
            scheme_colors.background,
            theme_val.radius.md,
            theme_val.shadows.xl,
            size.max_width(),
            padding_val
        )
    };

    let header_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "display: flex; \
             align-items: center; \
             justify-content: space-between; \
             margin-bottom: {}; \
             font-size: {}; \
             font-weight: {}; \
             color: {};",
            theme_val.spacing.md,
            theme_val.typography.font_sizes.lg,
            theme_val.typography.font_weights.semibold,
            scheme_colors.text
        )
    };

    let close_button_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "background: none; \
             border: none; \
             cursor: pointer; \
             padding: 0.25rem; \
             color: {}; \
             font-size: 1.5rem; \
             line-height: 1; \
             opacity: 0.6; \
             transition: opacity 0.15s ease;",
            scheme_colors.text
        )
    };

    let handle_close = move || {
        if let Some(callback) = &on_close {
            callback();
        }
    };

    let handle_overlay_click = move |ev: ev::MouseEvent| {
        if close_on_click_outside {
            // Check if the click was on the overlay itself, not the modal content
            let target = ev.target();
            if let Some(element) = target.and_then(|t| t.dyn_into::<web_sys::HtmlElement>().ok()) {
                if element.class_name().contains("mingot-modal-overlay") {
                    handle_close();
                }
            }
        }
    };

    // TODO: Add escape key handler when close_on_escape is true
    // This would require using window event listeners

    view! {
        <div
            class="mingot-modal-overlay"
            style=overlay_styles
            on:click=handle_overlay_click
        >
            <div
                class="mingot-modal"
                style=modal_styles
                on:click=|ev: ev::MouseEvent| {
                    // Prevent clicks on modal from bubbling to overlay
                    ev.stop_propagation();
                }
            >
                {if title.is_some() || with_close_button {
                    view! {
                        <div class="mingot-modal-header" style=header_styles>
                            <div class="mingot-modal-title">
                                {title.unwrap_or_default()}
                            </div>
                            {if with_close_button {
                                view! {
                                    <button
                                        class="mingot-modal-close"
                                        style=close_button_styles
                                        on:click=move |_| handle_close()
                                        aria-label="Close modal"
                                    >
                                        "×"
                                    </button>
                                }.into_any()
                            } else {
                                view! { <span></span> }.into_any()
                            }}
                        </div>
                    }.into_any()
                } else {
                    ().into_any()
                }}

                <div class="mingot-modal-body">
                    {children()}
                </div>
            </div>
        </div>
    }
}
