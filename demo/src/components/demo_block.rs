use leptos::prelude::*;

use super::CodeBlock;

/// Interactive demo container with live component and code display
#[component]
pub fn DemoBlock(
    #[prop(optional)] title: Option<&'static str>,
    children: Children,
    #[prop(optional)] code: Option<&'static str>,
) -> impl IntoView {
    let show_code = RwSignal::new(false);

    view! {
        <div class="demo-block">
            {title.map(|t| view! {
                <div style="padding: 0.75rem 1rem; border-bottom: 1px solid #e9ecef; font-weight: 500;">
                    {t}
                </div>
            })}

            <div class="demo-preview">
                {children()}
            </div>

            {code.map(|c| view! {
                <div class="demo-code-section">
                    <div class="demo-code-header">
                        <button
                            style="padding: 0.25rem 0.5rem; font-size: 0.75rem; border: 1px solid #e9ecef; border-radius: 0.25rem; background: white; cursor: pointer;"
                            on:click=move |_| show_code.update(|v| *v = !*v)
                        >
                            {move || if show_code.get() { "Hide code" } else { "Show code" }}
                        </button>
                    </div>

                    {move || show_code.get().then(|| view! {
                        <CodeBlock code=c language="rust" />
                    })}
                </div>
            })}
        </div>
    }
}
