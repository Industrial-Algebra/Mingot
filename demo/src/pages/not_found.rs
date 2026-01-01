use leptos::prelude::*;
use mingot::prelude::*;

#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div style="text-align: center; padding: 4rem 2rem;">
            <h1 style="font-size: 4rem; margin-bottom: 0.5rem;">"404"</h1>
            <p style="font-size: 1.25rem; color: #868e96; margin-bottom: 2rem;">
                "Page not found"
            </p>
            <a href="/">
                <Button variant=ButtonVariant::Filled>
                    "Go Home"
                </Button>
            </a>
        </div>
    }
}
