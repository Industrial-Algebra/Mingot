// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0
use leptos::prelude::*;

use super::{Header, Sidebar};

/// Main documentation layout with header, sidebar, and content area
#[component]
pub fn DocsLayout(children: Children) -> impl IntoView {
    view! {
        <div class="docs-layout">
            <Header />
            <Sidebar />
            <main class="docs-main">
                <div class="docs-content">
                    {children()}
                </div>
            </main>
        </div>
    }
}
