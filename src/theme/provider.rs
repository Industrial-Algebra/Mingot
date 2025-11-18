use super::{ColorSchemeMode, Theme, ThemeContext};
use leptos::prelude::*;

#[component]
pub fn MingotProvider(
    #[prop(optional)] theme: Option<Theme>,
    children: Children,
) -> impl IntoView {
    let theme = theme.unwrap_or_default();
    let theme_signal = RwSignal::new(theme);

    provide_context::<ThemeContext>(theme_signal);

    // Apply background color and text color based on theme
    let root_style = move || {
        let theme = theme_signal.get();
        let colors = super::get_scheme_colors(&theme);
        format!(
            "background-color: {}; color: {}; min-height: 100vh;",
            colors.background, colors.text
        )
    };

    view! {
        <div class="mingot-provider" style=root_style>
            {children()}
        </div>
    }
}

pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>()
        .expect("use_theme must be used within a MingotProvider")
}

/// Hook to get a function to toggle the color scheme
pub fn use_color_scheme_toggle() -> impl Fn() {
    let theme = use_theme();

    move || {
        theme.update(|t| {
            t.color_scheme = match t.color_scheme {
                ColorSchemeMode::Light => ColorSchemeMode::Dark,
                ColorSchemeMode::Dark => ColorSchemeMode::Light,
                ColorSchemeMode::Auto => ColorSchemeMode::Dark,
            };
        });
    }
}

/// Hook to get a function to set the color scheme
pub fn use_set_color_scheme() -> impl Fn(ColorSchemeMode) {
    let theme = use_theme();

    move |mode: ColorSchemeMode| {
        theme.update(|t| {
            t.color_scheme = mode;
        });
    }
}

/// Hook to get the current active color scheme
pub fn use_color_scheme() -> impl Fn() -> ColorSchemeMode + Clone {
    let theme = use_theme();

    move || theme.get().color_scheme
}
