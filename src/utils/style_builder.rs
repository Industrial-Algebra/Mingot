use std::fmt::Write;

/// A utility for building inline style strings
pub struct StyleBuilder {
    styles: Vec<(String, String)>,
}

impl StyleBuilder {
    pub fn new() -> Self {
        Self { styles: Vec::new() }
    }

    pub fn add(&mut self, property: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.styles.push((property.into(), value.into()));
        self
    }

    pub fn add_if(&mut self, condition: bool, property: impl Into<String>, value: impl Into<String>) -> &mut Self {
        if condition {
            self.add(property, value);
        }
        self
    }

    pub fn build(&self) -> String {
        let mut result = String::new();
        for (i, (prop, val)) in self.styles.iter().enumerate() {
            if i > 0 {
                result.push_str("; ");
            }
            write!(&mut result, "{}: {}", prop, val).unwrap();
        }
        result
    }
}

impl Default for StyleBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_builder() {
        let builder = StyleBuilder::new();
        assert_eq!(builder.build(), "");
    }

    #[test]
    fn test_single_style() {
        let mut builder = StyleBuilder::new();
        builder.add("color", "red");
        assert_eq!(builder.build(), "color: red");
    }

    #[test]
    fn test_multiple_styles() {
        let mut builder = StyleBuilder::new();
        builder.add("color", "red").add("font-size", "16px");
        assert_eq!(builder.build(), "color: red; font-size: 16px");
    }

    #[test]
    fn test_add_if_true() {
        let mut builder = StyleBuilder::new();
        builder.add("color", "red").add_if(true, "font-size", "16px");
        assert_eq!(builder.build(), "color: red; font-size: 16px");
    }

    #[test]
    fn test_add_if_false() {
        let mut builder = StyleBuilder::new();
        builder.add("color", "red").add_if(false, "font-size", "16px");
        assert_eq!(builder.build(), "color: red");
    }

    #[test]
    fn test_default() {
        let builder = StyleBuilder::default();
        assert_eq!(builder.build(), "");
    }

    #[test]
    fn test_chaining() {
        let mut builder = StyleBuilder::new();
        let result = builder
            .add("display", "flex")
            .add("flex-direction", "column")
            .add("gap", "1rem")
            .build();
        assert_eq!(result, "display: flex; flex-direction: column; gap: 1rem");
    }

    #[test]
    fn test_complex_values() {
        let mut builder = StyleBuilder::new();
        builder.add("background", "linear-gradient(to right, red, blue)");
        assert_eq!(
            builder.build(),
            "background: linear-gradient(to right, red, blue)"
        );
    }

    #[test]
    fn test_numeric_values() {
        let mut builder = StyleBuilder::new();
        builder.add("width", "100px").add("height", format!("{}px", 200));
        assert_eq!(builder.build(), "width: 100px; height: 200px");
    }
}
