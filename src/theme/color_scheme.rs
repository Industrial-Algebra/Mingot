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
