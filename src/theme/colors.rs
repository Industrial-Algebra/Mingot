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
        colors.insert(
            "blue".to_string(),
            ColorShades {
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
            },
        );

        // Gray shades
        colors.insert(
            "gray".to_string(),
            ColorShades {
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
            },
        );

        // Red shades
        colors.insert(
            "red".to_string(),
            ColorShades {
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
            },
        );

        // Green shades
        colors.insert(
            "green".to_string(),
            ColorShades {
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
            },
        );

        // Yellow shades
        colors.insert(
            "yellow".to_string(),
            ColorShades {
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
            },
        );

        // Orange shades
        colors.insert(
            "orange".to_string(),
            ColorShades {
                shades: vec![
                    "#fff4e6".to_string(),
                    "#ffe8cc".to_string(),
                    "#ffd8a8".to_string(),
                    "#ffc078".to_string(),
                    "#ffa94d".to_string(),
                    "#ff922b".to_string(),
                    "#fd7e14".to_string(),
                    "#e8590c".to_string(),
                    "#d9480f".to_string(),
                    "#c92a05".to_string(),
                ],
            },
        );

        // Cyan shades
        colors.insert(
            "cyan".to_string(),
            ColorShades {
                shades: vec![
                    "#e3fafc".to_string(),
                    "#c5f6fa".to_string(),
                    "#99e9f2".to_string(),
                    "#66d9e8".to_string(),
                    "#3bc9db".to_string(),
                    "#22b8cf".to_string(),
                    "#15aabf".to_string(),
                    "#1098ad".to_string(),
                    "#0c8599".to_string(),
                    "#0b7285".to_string(),
                ],
            },
        );

        // Violet shades
        colors.insert(
            "violet".to_string(),
            ColorShades {
                shades: vec![
                    "#f3f0ff".to_string(),
                    "#e5dbff".to_string(),
                    "#d0bfff".to_string(),
                    "#b197fc".to_string(),
                    "#9775fa".to_string(),
                    "#845ef7".to_string(),
                    "#7950f2".to_string(),
                    "#7048e8".to_string(),
                    "#6741d9".to_string(),
                    "#5f3dc4".to_string(),
                ],
            },
        );

        // Pink shades
        colors.insert(
            "pink".to_string(),
            ColorShades {
                shades: vec![
                    "#fff0f6".to_string(),
                    "#ffdeeb".to_string(),
                    "#fcc2d7".to_string(),
                    "#faa2c1".to_string(),
                    "#f783ac".to_string(),
                    "#f06595".to_string(),
                    "#e64980".to_string(),
                    "#d6336c".to_string(),
                    "#c2255c".to_string(),
                    "#a61e4d".to_string(),
                ],
            },
        );

        // Teal shades
        colors.insert(
            "teal".to_string(),
            ColorShades {
                shades: vec![
                    "#e6fcf5".to_string(),
                    "#c3fae8".to_string(),
                    "#96f2d7".to_string(),
                    "#63e6be".to_string(),
                    "#38d9a9".to_string(),
                    "#20c997".to_string(),
                    "#12b886".to_string(),
                    "#0ca678".to_string(),
                    "#099268".to_string(),
                    "#087f5b".to_string(),
                ],
            },
        );

        // Indigo shades
        colors.insert(
            "indigo".to_string(),
            ColorShades {
                shades: vec![
                    "#edf2ff".to_string(),
                    "#dbe4ff".to_string(),
                    "#bac8ff".to_string(),
                    "#91a7ff".to_string(),
                    "#748ffc".to_string(),
                    "#5c7cfa".to_string(),
                    "#4c6ef5".to_string(),
                    "#4263eb".to_string(),
                    "#3b5bdb".to_string(),
                    "#364fc7".to_string(),
                ],
            },
        );

        // Lime shades
        colors.insert(
            "lime".to_string(),
            ColorShades {
                shades: vec![
                    "#f4fce3".to_string(),
                    "#e9fac8".to_string(),
                    "#d8f5a2".to_string(),
                    "#c0eb75".to_string(),
                    "#a9e34b".to_string(),
                    "#94d82d".to_string(),
                    "#82c91e".to_string(),
                    "#74b816".to_string(),
                    "#66a80f".to_string(),
                    "#5c940d".to_string(),
                ],
            },
        );

        // Grape shades
        colors.insert(
            "grape".to_string(),
            ColorShades {
                shades: vec![
                    "#f8f0fc".to_string(),
                    "#f3d9fa".to_string(),
                    "#eebefa".to_string(),
                    "#e599f7".to_string(),
                    "#da77f2".to_string(),
                    "#cc5de8".to_string(),
                    "#be4bdb".to_string(),
                    "#ae3ec9".to_string(),
                    "#9c36b5".to_string(),
                    "#862e9c".to_string(),
                ],
            },
        );

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
        colors.insert(
            "blue".to_string(),
            ColorShades {
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
            },
        );

        // Dark gray shades
        colors.insert(
            "gray".to_string(),
            ColorShades {
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
            },
        );

        // Red shades (darker for dark mode)
        colors.insert(
            "red".to_string(),
            ColorShades {
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
            },
        );

        // Green shades (darker for dark mode)
        colors.insert(
            "green".to_string(),
            ColorShades {
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
            },
        );

        // Yellow shades (darker for dark mode)
        colors.insert(
            "yellow".to_string(),
            ColorShades {
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
            },
        );

        // Orange shades (darker for dark mode)
        colors.insert(
            "orange".to_string(),
            ColorShades {
                shades: vec![
                    "#4a2a0a".to_string(),
                    "#5f3510".to_string(),
                    "#754216".to_string(),
                    "#8c501c".to_string(),
                    "#a36022".to_string(),
                    "#ba7128".to_string(),
                    "#fd7e14".to_string(),
                    "#ff922b".to_string(),
                    "#ffa94d".to_string(),
                    "#ffc078".to_string(),
                ],
            },
        );

        // Cyan shades (darker for dark mode)
        colors.insert(
            "cyan".to_string(),
            ColorShades {
                shades: vec![
                    "#0d3b40".to_string(),
                    "#104d54".to_string(),
                    "#156068".to_string(),
                    "#1a737d".to_string(),
                    "#1f8691".to_string(),
                    "#2599a6".to_string(),
                    "#15aabf".to_string(),
                    "#22b8cf".to_string(),
                    "#3bc9db".to_string(),
                    "#66d9e8".to_string(),
                ],
            },
        );

        // Violet shades (darker for dark mode)
        colors.insert(
            "violet".to_string(),
            ColorShades {
                shades: vec![
                    "#2b1a4e".to_string(),
                    "#362163".to_string(),
                    "#422978".to_string(),
                    "#4f318d".to_string(),
                    "#5c3aa2".to_string(),
                    "#6944b8".to_string(),
                    "#7950f2".to_string(),
                    "#845ef7".to_string(),
                    "#9775fa".to_string(),
                    "#b197fc".to_string(),
                ],
            },
        );

        // Pink shades (darker for dark mode)
        colors.insert(
            "pink".to_string(),
            ColorShades {
                shades: vec![
                    "#4a1a30".to_string(),
                    "#5f1f3d".to_string(),
                    "#7d264f".to_string(),
                    "#9d2e62".to_string(),
                    "#c03676".to_string(),
                    "#e03e8a".to_string(),
                    "#e64980".to_string(),
                    "#f06595".to_string(),
                    "#f783ac".to_string(),
                    "#faa2c1".to_string(),
                ],
            },
        );

        // Teal shades (darker for dark mode)
        colors.insert(
            "teal".to_string(),
            ColorShades {
                shades: vec![
                    "#0d3b2e".to_string(),
                    "#104d3b".to_string(),
                    "#156049".to_string(),
                    "#1a7357".to_string(),
                    "#1f8666".to_string(),
                    "#259975".to_string(),
                    "#12b886".to_string(),
                    "#20c997".to_string(),
                    "#38d9a9".to_string(),
                    "#63e6be".to_string(),
                ],
            },
        );

        // Indigo shades (darker for dark mode)
        colors.insert(
            "indigo".to_string(),
            ColorShades {
                shades: vec![
                    "#1a2250".to_string(),
                    "#212b66".to_string(),
                    "#29357d".to_string(),
                    "#314094".to_string(),
                    "#3a4cab".to_string(),
                    "#4358c2".to_string(),
                    "#4c6ef5".to_string(),
                    "#5c7cfa".to_string(),
                    "#748ffc".to_string(),
                    "#91a7ff".to_string(),
                ],
            },
        );

        // Lime shades (darker for dark mode)
        colors.insert(
            "lime".to_string(),
            ColorShades {
                shades: vec![
                    "#2a3a0d".to_string(),
                    "#354d10".to_string(),
                    "#426015".to_string(),
                    "#50731a".to_string(),
                    "#5e861f".to_string(),
                    "#6c9925".to_string(),
                    "#82c91e".to_string(),
                    "#94d82d".to_string(),
                    "#a9e34b".to_string(),
                    "#c0eb75".to_string(),
                ],
            },
        );

        // Grape shades (darker for dark mode)
        colors.insert(
            "grape".to_string(),
            ColorShades {
                shades: vec![
                    "#371a47".to_string(),
                    "#45215a".to_string(),
                    "#54296e".to_string(),
                    "#633182".to_string(),
                    "#723a96".to_string(),
                    "#8244ab".to_string(),
                    "#be4bdb".to_string(),
                    "#cc5de8".to_string(),
                    "#da77f2".to_string(),
                    "#e599f7".to_string(),
                ],
            },
        );

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_palette_default() {
        let palette = ColorPalette::default();
        assert_eq!(palette.primary_color, "blue");
        assert!(!palette.light.colors.is_empty());
        assert!(!palette.dark.colors.is_empty());
    }

    const ALL_COLOR_NAMES: &[&str] = &[
        "blue", "gray", "red", "green", "yellow", "orange", "cyan", "violet", "pink", "teal",
        "indigo", "lime", "grape",
    ];

    #[test]
    fn test_light_scheme_colors() {
        let scheme = ColorScheme::light_default();

        // Test basic colors
        assert_eq!(scheme.white, "#ffffff");
        assert_eq!(scheme.black, "#000000");
        assert_eq!(scheme.background, "#ffffff");
        assert_eq!(scheme.text, "#000000");

        // Test that all 13 color palettes exist
        for name in ALL_COLOR_NAMES {
            assert!(
                scheme.colors.contains_key(*name),
                "Light scheme missing color: {}",
                name
            );
        }
        assert_eq!(scheme.colors.len(), 13);
    }

    #[test]
    fn test_dark_scheme_colors() {
        let scheme = ColorScheme::dark_default();

        // Test basic colors
        assert_eq!(scheme.white, "#ffffff");
        assert_eq!(scheme.black, "#000000");
        assert_eq!(scheme.background, "#1a1b1e");
        assert_eq!(scheme.text, "#c1c2c5");

        // Test that all 13 color palettes exist
        for name in ALL_COLOR_NAMES {
            assert!(
                scheme.colors.contains_key(*name),
                "Dark scheme missing color: {}",
                name
            );
        }
        assert_eq!(scheme.colors.len(), 13);
    }

    #[test]
    fn test_all_palettes_have_10_shades() {
        let light = ColorScheme::light_default();
        let dark = ColorScheme::dark_default();

        for name in ALL_COLOR_NAMES {
            let light_shades = light.colors.get(*name).unwrap();
            assert_eq!(
                light_shades.shades.len(),
                10,
                "Light {} should have 10 shades",
                name
            );

            let dark_shades = dark.colors.get(*name).unwrap();
            assert_eq!(
                dark_shades.shades.len(),
                10,
                "Dark {} should have 10 shades",
                name
            );
        }
    }

    #[test]
    fn test_get_color() {
        let scheme = ColorScheme::light_default();

        // Test getting blue shade 6 (primary)
        let blue = scheme.get_color("blue", 6);
        assert_eq!(blue, Some("#228be6".to_string()));

        // Test getting gray shade 6
        let gray = scheme.get_color("gray", 6);
        assert_eq!(gray, Some("#868e96".to_string()));

        // Test invalid shade
        let invalid = scheme.get_color("blue", 100);
        assert_eq!(invalid, None);

        // Test invalid color
        let invalid_color = scheme.get_color("purple", 6);
        assert_eq!(invalid_color, None);
    }

    #[test]
    fn test_color_shades_get() {
        let shades = ColorShades {
            shades: vec!["#aaa".to_string(), "#bbb".to_string(), "#ccc".to_string()],
        };

        assert_eq!(shades.get(0), Some(&"#aaa".to_string()));
        assert_eq!(shades.get(1), Some(&"#bbb".to_string()));
        assert_eq!(shades.get(2), Some(&"#ccc".to_string()));
        assert_eq!(shades.get(3), None);
    }

    #[test]
    fn test_palette_primary() {
        let palette = ColorPalette::default();
        let scheme = ColorScheme::light_default();

        // Since primary_color is "blue" by default, should get blue shade
        let primary = palette.primary(&scheme, 6);
        assert_eq!(primary, Some("#228be6".to_string()));
    }

    #[test]
    fn test_all_shades_have_10_colors() {
        let scheme = ColorScheme::light_default();

        for (name, shades) in &scheme.colors {
            assert_eq!(
                shades.shades.len(),
                10,
                "Color {} should have 10 shades",
                name
            );
        }
    }

    #[test]
    fn test_dark_scheme_different_from_light() {
        let light = ColorScheme::light_default();
        let dark = ColorScheme::dark_default();

        // Background and text should be different
        assert_ne!(light.background, dark.background);
        assert_ne!(light.text, dark.text);
        assert_ne!(light.border, dark.border);
    }
}
