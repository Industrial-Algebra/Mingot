#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum ColorSchemeMode {
    #[default]
    Light,
    Dark,
    Auto,
}

impl ColorSchemeMode {
    /// Get the active color scheme (resolves Auto based on system preference).
    pub fn resolve(&self) -> ActiveColorScheme {
        match self {
            ColorSchemeMode::Light => ActiveColorScheme::Light,
            ColorSchemeMode::Dark => ActiveColorScheme::Dark,
            ColorSchemeMode::Auto => detect_system_preference(),
        }
    }
}

/// Detect the system color scheme preference via `matchMedia`.
///
/// Returns `ActiveColorScheme::Dark` when the user's OS or browser prefers
/// dark mode, `ActiveColorScheme::Light` otherwise. Falls back to Light when
/// running outside a browser (e.g. SSR or native tests).
#[cfg(target_arch = "wasm32")]
fn detect_system_preference() -> ActiveColorScheme {
    use wasm_bindgen::JsCast;

    let dark = web_sys::window()
        .and_then(|w| w.match_media("(prefers-color-scheme: dark)").ok().flatten())
        .and_then(|mql| Some(mql.unchecked_into::<web_sys::MediaQueryList>().matches()))
        .unwrap_or(false);

    if dark {
        ActiveColorScheme::Dark
    } else {
        ActiveColorScheme::Light
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn detect_system_preference() -> ActiveColorScheme {
    // Outside the browser, default to Light.
    ActiveColorScheme::Light
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ActiveColorScheme {
    Light,
    Dark,
}

impl ActiveColorScheme {
    pub fn is_dark(&self) -> bool {
        matches!(self, ActiveColorScheme::Dark)
    }

    pub fn is_light(&self) -> bool {
        matches!(self, ActiveColorScheme::Light)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_scheme_mode_default() {
        assert_eq!(ColorSchemeMode::default(), ColorSchemeMode::Light);
    }

    #[test]
    fn test_light_mode_resolve() {
        let mode = ColorSchemeMode::Light;
        assert_eq!(mode.resolve(), ActiveColorScheme::Light);
    }

    #[test]
    fn test_dark_mode_resolve() {
        let mode = ColorSchemeMode::Dark;
        assert_eq!(mode.resolve(), ActiveColorScheme::Dark);
    }

    #[test]
    fn test_auto_mode_resolve() {
        let mode = ColorSchemeMode::Auto;
        // In non-wasm tests, Auto defaults to Light
        assert_eq!(mode.resolve(), ActiveColorScheme::Light);
    }

    #[test]
    fn test_active_scheme_is_dark() {
        assert!(ActiveColorScheme::Dark.is_dark());
        assert!(!ActiveColorScheme::Light.is_dark());
    }

    #[test]
    fn test_active_scheme_is_light() {
        assert!(ActiveColorScheme::Light.is_light());
        assert!(!ActiveColorScheme::Dark.is_light());
    }

    #[test]
    fn test_color_scheme_mode_clone() {
        let mode1 = ColorSchemeMode::Dark;
        let mode2 = mode1;
        assert_eq!(mode1, mode2);
    }

    #[test]
    fn test_active_color_scheme_clone() {
        let scheme1 = ActiveColorScheme::Dark;
        let scheme2 = scheme1;
        assert_eq!(scheme1, scheme2);
    }
}
