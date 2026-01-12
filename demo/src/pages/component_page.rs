use leptos::prelude::*;

use crate::components::PropsTable;
use crate::docs::get_component_doc;

/// Generic component documentation page
#[component]
pub fn ComponentPage(slug: &'static str) -> impl IntoView {
    let doc = get_component_doc(slug);

    view! {
        <div>
            {match doc {
                Some(doc) => view! {
                    <div>
                        <h1 class="page-title">{doc.name}</h1>
                        <p class="page-description">{doc.description}</p>

                        // Import statement
                        <div class="import-block">
                            <pre><code class="language-rust">{format!("use mingot::prelude::{{{}}};", doc.import_name)}</code></pre>
                        </div>

                        // Primary demo
                        {(doc.demo)()}

                        // Props table
                        <h2 class="section-title" id="props">"Props"</h2>
                        <PropsTable props=doc.props.clone() />
                    </div>
                }.into_any(),
                None => view! {
                    <div>
                        <h1 class="page-title">"Component Not Found"</h1>
                        <p class="page-description">
                            "Documentation for this component is coming soon. Check back later!"
                        </p>
                    </div>
                }.into_any(),
            }}
        </div>
    }
}
