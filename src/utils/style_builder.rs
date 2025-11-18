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
