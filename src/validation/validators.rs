use super::{ErrorCode, ValidationError, ValidationResult, Validator};
use std::marker::PhantomData;

/// Validator that checks if a string is not empty
#[derive(Clone)]
pub struct RequiredValidator;

impl Validator<String> for RequiredValidator {
    fn validate(&self, value: &String) -> ValidationResult {
        if value.trim().is_empty() {
            Err(ValidationError::required())
        } else {
            Ok(())
        }
    }
}

pub fn required() -> RequiredValidator {
    RequiredValidator
}

/// Validator that checks minimum string length
#[derive(Clone)]
pub struct MinLengthValidator {
    min: usize,
}

impl Validator<String> for MinLengthValidator {
    fn validate(&self, value: &String) -> ValidationResult {
        if value.len() < self.min {
            Err(ValidationError::min_length(self.min))
        } else {
            Ok(())
        }
    }
}

pub fn min_length(min: usize) -> MinLengthValidator {
    MinLengthValidator { min }
}

/// Validator that checks maximum string length
#[derive(Clone)]
pub struct MaxLengthValidator {
    max: usize,
}

impl Validator<String> for MaxLengthValidator {
    fn validate(&self, value: &String) -> ValidationResult {
        if value.len() > self.max {
            Err(ValidationError::max_length(self.max))
        } else {
            Ok(())
        }
    }
}

pub fn max_length(max: usize) -> MaxLengthValidator {
    MaxLengthValidator { max }
}

/// Validator that checks email format
#[derive(Clone)]
pub struct EmailValidator;

impl Validator<String> for EmailValidator {
    fn validate(&self, value: &String) -> ValidationResult {
        if value.trim().is_empty() {
            return Ok(()); // Empty is valid, use required() to make it mandatory
        }

        // Basic email validation
        if value.contains('@') && value.contains('.') && value.len() > 3 {
            let parts: Vec<&str> = value.split('@').collect();
            if parts.len() == 2 && !parts[0].is_empty() && !parts[1].is_empty() {
                return Ok(());
            }
        }

        Err(ValidationError::invalid_email())
    }
}

pub fn email() -> EmailValidator {
    EmailValidator
}

/// Validator that checks URL format
#[derive(Clone)]
pub struct UrlValidator;

impl Validator<String> for UrlValidator {
    fn validate(&self, value: &String) -> ValidationResult {
        if value.trim().is_empty() {
            return Ok(()); // Empty is valid, use required() to make it mandatory
        }

        if value.starts_with("http://") || value.starts_with("https://") {
            Ok(())
        } else {
            Err(ValidationError::new(
                "Invalid URL format",
                ErrorCode::InvalidUrl,
            ))
        }
    }
}

pub fn url() -> UrlValidator {
    UrlValidator
}

/// Validator that checks if a value matches a pattern
#[derive(Clone)]
pub struct PatternValidator {
    pattern: String,
}

impl Validator<String> for PatternValidator {
    fn validate(&self, value: &String) -> ValidationResult {
        // For now, just check if value contains the pattern
        // In a real implementation, you'd use the regex crate
        if value.contains(&self.pattern) {
            Ok(())
        } else {
            Err(ValidationError::pattern(&self.pattern))
        }
    }
}

pub fn pattern(pattern: impl Into<String>) -> PatternValidator {
    PatternValidator {
        pattern: pattern.into(),
    }
}

/// Validator for numeric minimum value
#[derive(Clone)]
pub struct MinValueValidator<T> {
    min: T,
}

impl<T: PartialOrd + std::fmt::Display + Clone> Validator<T> for MinValueValidator<T> {
    fn validate(&self, value: &T) -> ValidationResult {
        if value < &self.min {
            Err(ValidationError::min_value(self.min.clone()))
        } else {
            Ok(())
        }
    }
}

pub fn min_value<T: PartialOrd + std::fmt::Display + Clone>(min: T) -> MinValueValidator<T> {
    MinValueValidator { min }
}

/// Validator for numeric maximum value
#[derive(Clone)]
pub struct MaxValueValidator<T> {
    max: T,
}

impl<T: PartialOrd + std::fmt::Display + Clone> Validator<T> for MaxValueValidator<T> {
    fn validate(&self, value: &T) -> ValidationResult {
        if value > &self.max {
            Err(ValidationError::max_value(self.max.clone()))
        } else {
            Ok(())
        }
    }
}

pub fn max_value<T: PartialOrd + std::fmt::Display + Clone>(max: T) -> MaxValueValidator<T> {
    MaxValueValidator { max }
}

/// Custom validator using a closure
#[derive(Clone)]
pub struct CustomValidator<T, F>
where
    F: Fn(&T) -> ValidationResult + Clone,
{
    func: F,
    _phantom: PhantomData<T>,
}

impl<T, F> Validator<T> for CustomValidator<T, F>
where
    F: Fn(&T) -> ValidationResult + Clone,
{
    fn validate(&self, value: &T) -> ValidationResult {
        (self.func)(value)
    }
}

pub fn custom<T, F>(func: F) -> CustomValidator<T, F>
where
    F: Fn(&T) -> ValidationResult + Clone,
{
    CustomValidator {
        func,
        _phantom: PhantomData,
    }
}

/// Validator that checks if value equals another value
#[derive(Clone)]
pub struct EqualsValidator<T> {
    expected: T,
    message: String,
}

impl<T: PartialEq> Validator<T> for EqualsValidator<T> {
    fn validate(&self, value: &T) -> ValidationResult {
        if value == &self.expected {
            Ok(())
        } else {
            Err(ValidationError::custom(&self.message))
        }
    }
}

pub fn equals<T: Clone>(expected: T, message: impl Into<String>) -> EqualsValidator<T> {
    EqualsValidator {
        expected,
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_validator() {
        let validator = required();
        assert!(validator.validate(&"hello".to_string()).is_ok());
        assert!(validator.validate(&"".to_string()).is_err());
        assert!(validator.validate(&"   ".to_string()).is_err());
    }

    #[test]
    fn test_min_length_validator() {
        let validator = min_length(5);
        assert!(validator.validate(&"hello".to_string()).is_ok());
        assert!(validator.validate(&"hello world".to_string()).is_ok());
        assert!(validator.validate(&"hi".to_string()).is_err());
    }

    #[test]
    fn test_max_length_validator() {
        let validator = max_length(10);
        assert!(validator.validate(&"hello".to_string()).is_ok());
        assert!(validator
            .validate(&"hello world is too long".to_string())
            .is_err());
    }

    #[test]
    fn test_email_validator() {
        let validator = email();
        assert!(validator.validate(&"test@example.com".to_string()).is_ok());
        assert!(validator.validate(&"invalid.email".to_string()).is_err());
        assert!(validator.validate(&"".to_string()).is_ok()); // Empty is ok
    }

    #[test]
    fn test_url_validator() {
        let validator = url();
        assert!(validator
            .validate(&"https://example.com".to_string())
            .is_ok());
        assert!(validator
            .validate(&"http://example.com".to_string())
            .is_ok());
        assert!(validator.validate(&"example.com".to_string()).is_err());
    }

    #[test]
    fn test_min_value_validator() {
        let validator = min_value(5);
        assert!(validator.validate(&10).is_ok());
        assert!(validator.validate(&5).is_ok());
        assert!(validator.validate(&3).is_err());
    }

    #[test]
    fn test_max_value_validator() {
        let validator = max_value(100);
        assert!(validator.validate(&50).is_ok());
        assert!(validator.validate(&100).is_ok());
        assert!(validator.validate(&150).is_err());
    }

    #[test]
    fn test_combined_validators() {
        let validator = required().and(min_length(3)).and(max_length(10));
        assert!(validator.validate(&"hello".to_string()).is_ok());
        assert!(validator.validate(&"".to_string()).is_err());
        assert!(validator.validate(&"hi".to_string()).is_err());
        assert!(validator.validate(&"this is too long".to_string()).is_err());
    }

    #[test]
    fn test_custom_message() {
        let validator = required().with_message("Username is required");
        let result = validator.validate(&"".to_string());
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "Username is required");
    }
}
