use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq)]
pub struct Spacing {
    pub xs: Cow<'static, str>,
    pub sm: Cow<'static, str>,
    pub md: Cow<'static, str>,
    pub lg: Cow<'static, str>,
    pub xl: Cow<'static, str>,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            xs: Cow::Borrowed("0.625rem"), // 10px
            sm: Cow::Borrowed("0.75rem"),  // 12px
            md: Cow::Borrowed("1rem"),     // 16px
            lg: Cow::Borrowed("1.25rem"),  // 20px
            xl: Cow::Borrowed("2rem"),     // 32px
        }
    }
}

impl Spacing {
    pub fn get(&self, size: SpacingSize) -> &str {
        match size {
            SpacingSize::Xs => &self.xs,
            SpacingSize::Sm => &self.sm,
            SpacingSize::Md => &self.md,
            SpacingSize::Lg => &self.lg,
            SpacingSize::Xl => &self.xl,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SpacingSize {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
}

impl From<&str> for SpacingSize {
    fn from(s: &str) -> Self {
        match s {
            "xs" => SpacingSize::Xs,
            "sm" => SpacingSize::Sm,
            "md" => SpacingSize::Md,
            "lg" => SpacingSize::Lg,
            "xl" => SpacingSize::Xl,
            _ => SpacingSize::Md,
        }
    }
}
