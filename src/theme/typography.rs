#[derive(Clone, Debug, PartialEq)]
pub struct Typography {
    pub font_family: &'static str,
    pub font_family_monospace: &'static str,
    pub font_sizes: FontSizes,
    pub line_heights: LineHeights,
    pub font_weights: FontWeights,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            font_family: "-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji'",
            font_family_monospace: "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace",
            font_sizes: FontSizes::default(),
            line_heights: LineHeights::default(),
            font_weights: FontWeights::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FontSizes {
    pub xs: &'static str,
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
}

impl Default for FontSizes {
    fn default() -> Self {
        Self {
            xs: "0.75rem",    // 12px
            sm: "0.875rem",   // 14px
            md: "1rem",       // 16px
            lg: "1.125rem",   // 18px
            xl: "1.25rem",    // 20px
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineHeights {
    pub xs: &'static str,
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
}

impl Default for LineHeights {
    fn default() -> Self {
        Self {
            xs: "1",
            sm: "1.25",
            md: "1.5",
            lg: "1.625",
            xl: "1.75",
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FontWeights {
    pub normal: u16,
    pub medium: u16,
    pub semibold: u16,
    pub bold: u16,
}

impl Default for FontWeights {
    fn default() -> Self {
        Self {
            normal: 400,
            medium: 500,
            semibold: 600,
            bold: 700,
        }
    }
}
