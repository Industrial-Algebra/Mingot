use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct ColorScheme {
    pub primary_color: String,
    pub colors: HashMap<String, ColorShades>,
    pub white: String,
    pub black: String,
}

impl Default for ColorScheme {
    fn default() -> Self {
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
            primary_color: "blue".to_string(),
            colors,
            white: "#ffffff".to_string(),
            black: "#000000".to_string(),
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

    pub fn primary(&self, shade: usize) -> Option<String> {
        self.get_color(&self.primary_color, shade)
    }
}
