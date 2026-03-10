use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq)]
pub struct Typography {
    pub font_family: Cow<'static, str>,
    pub font_family_monospace: Cow<'static, str>,
    pub font_sizes: FontSizes,
    pub line_heights: LineHeights,
    pub font_weights: FontWeights,
}

impl Default for Typography {
    fn default() -> Self {
        Self {
            font_family: Cow::Borrowed("-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif, 'Apple Color Emoji', 'Segoe UI Emoji'"),
            font_family_monospace: Cow::Borrowed("ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace"),
            font_sizes: FontSizes::default(),
            line_heights: LineHeights::default(),
            font_weights: FontWeights::default(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FontSizes {
    pub xs: Cow<'static, str>,
    pub sm: Cow<'static, str>,
    pub md: Cow<'static, str>,
    pub lg: Cow<'static, str>,
    pub xl: Cow<'static, str>,
    pub xxl: Cow<'static, str>,
}

impl Default for FontSizes {
    fn default() -> Self {
        Self {
            xs: Cow::Borrowed("0.75rem"),  // 12px
            sm: Cow::Borrowed("0.875rem"), // 14px
            md: Cow::Borrowed("1rem"),     // 16px
            lg: Cow::Borrowed("1.125rem"), // 18px
            xl: Cow::Borrowed("1.25rem"),  // 20px
            xxl: Cow::Borrowed("2rem"),    // 32px
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LineHeights {
    pub xs: Cow<'static, str>,
    pub sm: Cow<'static, str>,
    pub md: Cow<'static, str>,
    pub lg: Cow<'static, str>,
    pub xl: Cow<'static, str>,
}

impl Default for LineHeights {
    fn default() -> Self {
        Self {
            xs: Cow::Borrowed("1"),
            sm: Cow::Borrowed("1.25"),
            md: Cow::Borrowed("1.5"),
            lg: Cow::Borrowed("1.625"),
            xl: Cow::Borrowed("1.75"),
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
