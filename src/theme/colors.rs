use std::collections::HashMap;

/// Contains both light and dark color schemes
#[derive(Clone, Debug, PartialEq)]
pub struct ColorPalette {
    pub primary_color: String,
    pub light: ColorScheme,
    pub dark: ColorScheme,
}

impl Default for ColorPalette {
    fn default() -> Self {
        Self {
            primary_color: "blue".to_string(),
            light: ColorScheme::light_default(),
            dark: ColorScheme::dark_default(),
        }
    }
}

/// Color scheme for a specific mode (light or dark)
#[derive(Clone, Debug, PartialEq)]
pub struct ColorScheme {
    pub colors: HashMap<String, ColorShades>,
    pub white: String,
    pub black: String,
    pub background: String,
    pub text: String,
    pub border: String,
}

impl ColorScheme {
    /// Create default light color scheme
    pub fn light_default() -> Self {
        let mut colors = HashMap::new();

        // Blue shades (default primary)
        colors.insert("blue".to_string(), ColorShades {
            shades: vec![
                "#e7f5ff".to_string(),
                "#d0ebff".to_string(),
                "#a5d8ff".to_string(),
                "#74c0fc".to_string(),
                "#4dabf7".to_string(),
                "#339af0".to_string(),
                "#228be6".to_string(),
                "#1c7ed6".to_string(),
                "#1971c2".to_string(),
                "#1864ab".to_string(),
            ],
        });

        // Gray shades
        colors.insert("gray".to_string(), ColorShades {
            shades: vec![
                "#f8f9fa".to_string(),
                "#f1f3f5".to_string(),
                "#e9ecef".to_string(),
                "#dee2e6".to_string(),
                "#ced4da".to_string(),
                "#adb5bd".to_string(),
                "#868e96".to_string(),
                "#495057".to_string(),
                "#343a40".to_string(),
                "#212529".to_string(),
            ],
        });

        // Red shades
        colors.insert("red".to_string(), ColorShades {
            shades: vec![
                "#fff5f5".to_string(),
                "#ffe3e3".to_string(),
                "#ffc9c9".to_string(),
                "#ffa8a8".to_string(),
                "#ff8787".to_string(),
                "#ff6b6b".to_string(),
                "#fa5252".to_string(),
                "#f03e3e".to_string(),
                "#e03131".to_string(),
                "#c92a2a".to_string(),
            ],
        });

        // Green shades
        colors.insert("green".to_string(), ColorShades {
            shades: vec![
                "#ebfbee".to_string(),
                "#d3f9d8".to_string(),
                "#b2f2bb".to_string(),
                "#8ce99a".to_string(),
                "#69db7c".to_string(),
                "#51cf66".to_string(),
                "#40c057".to_string(),
                "#37b24d".to_string(),
                "#2f9e44".to_string(),
                "#2b8a3e".to_string(),
            ],
        });

        // Yellow shades
        colors.insert("yellow".to_string(), ColorShades {
            shades: vec![
                "#fff9db".to_string(),
                "#fff3bf".to_string(),
                "#ffec99".to_string(),
                "#ffe066".to_string(),
                "#ffd43b".to_string(),
                "#fcc419".to_string(),
                "#fab005".to_string(),
                "#f59f00".to_string(),
                "#f08c00".to_string(),
                "#e67700".to_string(),
            ],
        });

        Self {
            colors,
            white: "#ffffff".to_string(),
            black: "#000000".to_string(),
            background: "#ffffff".to_string(),
            text: "#000000".to_string(),
            border: "#dee2e6".to_string(),
        }
    }

    /// Create default dark color scheme
    pub fn dark_default() -> Self {
        let mut colors = HashMap::new();

        // Blue shades (darker for dark mode)
        colors.insert("blue".to_string(), ColorShades {
            shades: vec![
                "#1e3a5f".to_string(),
                "#1c4d7a".to_string(),
                "#1e5f99".to_string(),
                "#2272b8".to_string(),
                "#2e85d1".to_string(),
                "#3b98e8".to_string(),
                "#4dabf7".to_string(),
                "#60bbff".to_string(),
                "#7ac8ff".to_string(),
                "#94d5ff".to_string(),
            ],
        });

        // Dark gray shades
        colors.insert("gray".to_string(), ColorShades {
            shades: vec![
                "#1a1b1e".to_string(),
                "#25262b".to_string(),
                "#2c2e33".to_string(),
                "#373a40".to_string(),
                "#424549".to_string(),
                "#5c5f66".to_string(),
                "#909296".to_string(),
                "#c1c2c5".to_string(),
                "#d4d4d4".to_string(),
                "#e5e5e5".to_string(),
            ],
        });

        // Red shades (darker for dark mode)
        colors.insert("red".to_string(), ColorShades {
            shades: vec![
                "#4a1a1a".to_string(),
                "#5f1f1f".to_string(),
                "#7d2626".to_string(),
                "#9d2e2e".to_string(),
                "#c03636".to_string(),
                "#e03e3e".to_string(),
                "#fa5252".to_string(),
                "#ff6b6b".to_string(),
                "#ff8787".to_string(),
                "#ffa8a8".to_string(),
            ],
        });

        // Green shades (darker for dark mode)
        colors.insert("green".to_string(), ColorShades {
            shades: vec![
                "#1a3a27".to_string(),
                "#1f4d30".to_string(),
                "#25603a".to_string(),
                "#2b7346".to_string(),
                "#338654".to_string(),
                "#3b9963".to_string(),
                "#40c057".to_string(),
                "#51cf66".to_string(),
                "#69db7c".to_string(),
                "#8ce99a".to_string(),
            ],
        });

        // Yellow shades (darker for dark mode)
        colors.insert("yellow".to_string(), ColorShades {
            shades: vec![
                "#4a3d0f".to_string(),
                "#5f4d13".to_string(),
                "#755e17".to_string(),
                "#8c6f1c".to_string(),
                "#a38121".to_string(),
                "#ba9227".to_string(),
                "#fab005".to_string(),
                "#fcc419".to_string(),
                "#ffd43b".to_string(),
                "#ffe066".to_string(),
            ],
        });

        Self {
            colors,
            white: "#ffffff".to_string(),
            black: "#000000".to_string(),
            background: "#1a1b1e".to_string(),
            text: "#c1c2c5".to_string(),
            border: "#373a40".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ColorShades {
    pub shades: Vec<String>,
}

impl ColorShades {
    pub fn get(&self, index: usize) -> Option<&String> {
        self.shades.get(index)
    }
}

impl ColorScheme {
    pub fn get_color(&self, color: &str, shade: usize) -> Option<String> {
        self.colors.get(color).and_then(|c| c.get(shade).cloned())
    }
}

impl ColorPalette {
    pub fn primary(&self, scheme: &ColorScheme, shade: usize) -> Option<String> {
        scheme.get_color(&self.primary_color, shade)
    }
}
