use leptos::prelude::*;

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
                        <table class="props-table">
                            <thead>
                                <tr>
                                    <th>"Name"</th>
                                    <th>"Type"</th>
                                    <th>"Default"</th>
                                    <th>"Description"</th>
                                </tr>
                            </thead>
                            <tbody>
                                {doc.props.iter().map(|prop| view! {
                                    <tr>
                                        <td>
                                            <code>{prop.name}</code>
                                            {prop.required.then(|| view! {
                                                <span style="margin-left: 0.25rem; font-size: 0.625rem; padding: 0.0625rem 0.25rem; background: #fa5252; color: white; border-radius: 0.125rem;">
                                                    "required"
                                                </span>
                                            })}
                                        </td>
                                        <td><code>{prop.prop_type}</code></td>
                                        <td>
                                            {prop.default.map(|d| view! { <code>{d}</code> }.into_any())
                                                .unwrap_or_else(|| view! { <span style="color: #868e96;">"-"</span> }.into_any())}
                                        </td>
                                        <td>{prop.description}</td>
                                    </tr>
                                }).collect_view()}
                            </tbody>
                        </table>
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
