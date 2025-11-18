use super::{Theme, ThemeContext};
use leptos::prelude::*;

#[component]
pub fn MingotProvider(
    #[prop(optional)] theme: Option<Theme>,
    children: Children,
) -> impl IntoView {
    let theme = theme.unwrap_or_default();
    let theme_signal = RwSignal::new(theme);

    provide_context::<ThemeContext>(theme_signal);

    view! {
        <div class="mingot-provider">
            {children()}
        </div>
    }
}

pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>()
        .expect("use_theme must be used within a MingotProvider")
}
