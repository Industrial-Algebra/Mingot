use crate::theme::use_theme;
use crate::utils::StyleBuilder;
use leptos::prelude::*;

#[component]
pub fn AppShell(
    #[prop(optional)] header: Option<Children>,
    #[prop(optional)] navbar: Option<Children>,
    #[prop(optional)] aside: Option<Children>,
    #[prop(optional)] footer: Option<Children>,
    #[prop(optional, into)] padding: Option<String>,
    #[prop(optional, into)] class: Option<String>,
    #[prop(optional, into)] style: Option<String>,
    children: Children,
) -> impl IntoView {
    let theme = use_theme();

    let shell_styles =
        move || "display: flex; flex-direction: column; min-height: 100vh;".to_string();

    let main_wrapper_styles = move || "display: flex; flex: 1; overflow: hidden;".to_string();

    let navbar_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "width: 260px; \
             flex-shrink: 0; \
             overflow-y: auto; \
             background-color: {}; \
             border-right: 1px solid {};",
            scheme_colors.background, scheme_colors.border
        )
    };

    let aside_styles = move || {
        let theme_val = theme.get();
        let scheme_colors = crate::theme::get_scheme_colors(&theme_val);
        format!(
            "width: 260px; \
             flex-shrink: 0; \
             overflow-y: auto; \
             background-color: {}; \
             border-left: 1px solid {};",
            scheme_colors.background, scheme_colors.border
        )
    };

    let main_styles = move || {
        let theme_val = theme.get();
        let mut builder = StyleBuilder::new();

        builder.add("flex", "1").add("overflow-y", "auto");

        if let Some(p) = padding.as_ref() {
            builder.add("padding", p);
        } else {
            builder.add("padding", theme_val.spacing.md);
        }

        builder.build()
    };

    let class_str = format!("mingot-appshell {}", class.unwrap_or_default());

    view! {
        <div class=class_str style=move || {
            if let Some(s) = style.as_ref() {
                format!("{}; {}", shell_styles(), s)
            } else {
                shell_styles()
            }
        }>
            {if let Some(h) = header {
                view! { <div class="mingot-appshell-header">{h()}</div> }.into_any()
            } else {
                ().into_any()
            }}

            <div class="mingot-appshell-main-wrapper" style=main_wrapper_styles>
                {if let Some(n) = navbar {
                    view! {
                        <nav class="mingot-appshell-navbar" style=navbar_styles>
                            {n()}
                        </nav>
                    }.into_any()
                } else {
                    ().into_any()
                }}

                <main class="mingot-appshell-main" style=main_styles>
                    {children()}
                </main>

                {if let Some(a) = aside {
                    view! {
                        <aside class="mingot-appshell-aside" style=aside_styles>
                            {a()}
                        </aside>
                    }.into_any()
                } else {
                    ().into_any()
                }}
            </div>

            {if let Some(f) = footer {
                view! { <div class="mingot-appshell-footer">{f()}</div> }.into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}
