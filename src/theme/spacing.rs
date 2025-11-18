#[derive(Clone, Debug, PartialEq)]
pub struct Spacing {
    pub xs: &'static str,
    pub sm: &'static str,
    pub md: &'static str,
    pub lg: &'static str,
    pub xl: &'static str,
}

impl Default for Spacing {
    fn default() -> Self {
        Self {
            xs: "0.625rem",   // 10px
            sm: "0.75rem",    // 12px
            md: "1rem",       // 16px
            lg: "1.25rem",    // 20px
            xl: "2rem",       // 32px
        }
    }
}

impl Spacing {
    pub fn get(&self, size: SpacingSize) -> &'static str {
        match size {
            SpacingSize::Xs => self.xs,
            SpacingSize::Sm => self.sm,
            SpacingSize::Md => self.md,
            SpacingSize::Lg => self.lg,
            SpacingSize::Xl => self.xl,
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
