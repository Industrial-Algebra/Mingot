use leptos::prelude::*;
use leptos_router::components::{Route, Router, Routes};
use leptos_router::path;
use mingot::prelude::*;

use crate::components::ThemeVars;
use crate::layout::DocsLayout;
use crate::pages::{ComponentPage, GettingStartedPage, HomePage, NotFoundPage};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <MingotProvider>
            <ThemeVars />
            <Router>
                <DocsLayout>
                    <Routes fallback=NotFoundPage>
                        // Home
                        <Route path=path!("/") view=HomePage />
                        <Route path=path!("/getting-started") view=GettingStartedPage />

                        // Core components
                        <Route path=path!("/core/button") view=move || view! { <ComponentPage slug="button" /> } />
                        <Route path=path!("/core/action-icon") view=move || view! { <ComponentPage slug="action-icon" /> } />
                        <Route path=path!("/core/container") view=move || view! { <ComponentPage slug="container" /> } />
                        <Route path=path!("/core/divider") view=move || view! { <ComponentPage slug="divider" /> } />
                        <Route path=path!("/core/group") view=move || view! { <ComponentPage slug="group" /> } />
                        <Route path=path!("/core/stack") view=move || view! { <ComponentPage slug="stack" /> } />
                        <Route path=path!("/core/text") view=move || view! { <ComponentPage slug="text" /> } />

                        // Layout components
                        <Route path=path!("/layout/app-shell") view=move || view! { <ComponentPage slug="app-shell" /> } />
                        <Route path=path!("/layout/card") view=move || view! { <ComponentPage slug="card" /> } />
                        <Route path=path!("/layout/grid") view=move || view! { <ComponentPage slug="grid" /> } />
                        <Route path=path!("/layout/header") view=move || view! { <ComponentPage slug="header" /> } />
                        <Route path=path!("/layout/paper") view=move || view! { <ComponentPage slug="paper" /> } />

                        // Navigation components
                        <Route path=path!("/navigation/breadcrumbs") view=move || view! { <ComponentPage slug="breadcrumbs" /> } />
                        <Route path=path!("/navigation/burger") view=move || view! { <ComponentPage slug="burger" /> } />
                        <Route path=path!("/navigation/navbar") view=move || view! { <ComponentPage slug="navbar" /> } />
                        <Route path=path!("/navigation/pagination") view=move || view! { <ComponentPage slug="pagination" /> } />
                        <Route path=path!("/navigation/tabs") view=move || view! { <ComponentPage slug="tabs" /> } />

                        // Form components
                        <Route path=path!("/form/checkbox") view=move || view! { <ComponentPage slug="checkbox" /> } />
                        <Route path=path!("/form/file-input") view=move || view! { <ComponentPage slug="file-input" /> } />
                        <Route path=path!("/form/input") view=move || view! { <ComponentPage slug="input" /> } />
                        <Route path=path!("/form/number-input") view=move || view! { <ComponentPage slug="number-input" /> } />
                        <Route path=path!("/form/password-input") view=move || view! { <ComponentPage slug="password-input" /> } />
                        <Route path=path!("/form/pin-input") view=move || view! { <ComponentPage slug="pin-input" /> } />
                        <Route path=path!("/form/radio") view=move || view! { <ComponentPage slug="radio" /> } />
                        <Route path=path!("/form/range-slider") view=move || view! { <ComponentPage slug="range-slider" /> } />
                        <Route path=path!("/form/segmented-control") view=move || view! { <ComponentPage slug="segmented-control" /> } />
                        <Route path=path!("/form/select") view=move || view! { <ComponentPage slug="select" /> } />
                        <Route path=path!("/form/slider") view=move || view! { <ComponentPage slug="slider" /> } />
                        <Route path=path!("/form/switch") view=move || view! { <ComponentPage slug="switch" /> } />
                        <Route path=path!("/form/textarea") view=move || view! { <ComponentPage slug="textarea" /> } />

                        // Overlay components
                        <Route path=path!("/overlay/drawer") view=move || view! { <ComponentPage slug="drawer" /> } />
                        <Route path=path!("/overlay/loading-overlay") view=move || view! { <ComponentPage slug="loading-overlay" /> } />
                        <Route path=path!("/overlay/modal") view=move || view! { <ComponentPage slug="modal" /> } />
                        <Route path=path!("/overlay/popover") view=move || view! { <ComponentPage slug="popover" /> } />
                        <Route path=path!("/overlay/tooltip") view=move || view! { <ComponentPage slug="tooltip" /> } />

                        // Feedback components
                        <Route path=path!("/feedback/alert") view=move || view! { <ComponentPage slug="alert" /> } />
                        <Route path=path!("/feedback/loader") view=move || view! { <ComponentPage slug="loader" /> } />
                        <Route path=path!("/feedback/notification") view=move || view! { <ComponentPage slug="notification" /> } />
                        <Route path=path!("/feedback/progress") view=move || view! { <ComponentPage slug="progress" /> } />
                        <Route path=path!("/feedback/skeleton") view=move || view! { <ComponentPage slug="skeleton" /> } />

                        // Data display components
                        <Route path=path!("/data-display/accordion") view=move || view! { <ComponentPage slug="accordion" /> } />
                        <Route path=path!("/data-display/avatar") view=move || view! { <ComponentPage slug="avatar" /> } />
                        <Route path=path!("/data-display/badge") view=move || view! { <ComponentPage slug="badge" /> } />
                        <Route path=path!("/data-display/ring-progress") view=move || view! { <ComponentPage slug="ring-progress" /> } />
                        <Route path=path!("/data-display/stats") view=move || view! { <ComponentPage slug="stats" /> } />
                        <Route path=path!("/data-display/table") view=move || view! { <ComponentPage slug="table" /> } />

                        // Miscellaneous
                        <Route path=path!("/misc/error-page") view=move || view! { <ComponentPage slug="error-page" /> } />
                    </Routes>
                </DocsLayout>
            </Router>
        </MingotProvider>
    }
}
