#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ColorSchemeMode {
    Light,
    Dark,
    Auto,
}

impl Default for ColorSchemeMode {
    fn default() -> Self {
        Self::Light
    }
}

impl ColorSchemeMode {
    /// Get the active color scheme (resolves Auto based on system preference)
    pub fn resolve(&self) -> ActiveColorScheme {
        match self {
            ColorSchemeMode::Light => ActiveColorScheme::Light,
            ColorSchemeMode::Dark => ActiveColorScheme::Dark,
            ColorSchemeMode::Auto => {
                // In a real implementation, this would check system preferences
                // For now, default to light
                ActiveColorScheme::Light
            }
        }
    }
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
        // Auto currently defaults to Light
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
