use leptos::prelude::*;

use crate::docs::PropDoc;

/// Table displaying component props documentation
#[component]
pub fn PropsTable(props: Vec<PropDoc>) -> impl IntoView {
    view! {
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
                {props.into_iter().map(|prop| view! {
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
    }
}
