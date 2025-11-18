pub mod validators;

pub use validators::*;

use std::fmt;

/// Validation error with message
#[derive(Clone, Debug, PartialEq)]
pub struct ValidationError {
    pub message: String,
    pub code: ErrorCode,
}

impl ValidationError {
    pub fn new(message: impl Into<String>, code: ErrorCode) -> Self {
        Self {
            message: message.into(),
            code,
        }
    }

    pub fn required() -> Self {
        Self::new("This field is required", ErrorCode::Required)
    }

    pub fn invalid_email() -> Self {
        Self::new("Invalid email address", ErrorCode::InvalidEmail)
    }

    pub fn min_length(min: usize) -> Self {
        Self::new(
            format!("Must be at least {} characters", min),
            ErrorCode::MinLength,
        )
    }

    pub fn max_length(max: usize) -> Self {
        Self::new(
            format!("Must be at most {} characters", max),
            ErrorCode::MaxLength,
        )
    }

    pub fn min_value(min: impl fmt::Display) -> Self {
        Self::new(format!("Must be at least {}", min), ErrorCode::MinValue)
    }

    pub fn max_value(max: impl fmt::Display) -> Self {
        Self::new(format!("Must be at most {}", max), ErrorCode::MaxValue)
    }

    pub fn pattern(pattern: &str) -> Self {
        Self::new(
            format!("Must match pattern: {}", pattern),
            ErrorCode::Pattern,
        )
    }

    pub fn custom(message: impl Into<String>) -> Self {
        Self::new(message, ErrorCode::Custom)
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Standard error codes for validation failures
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ErrorCode {
    Required,
    InvalidEmail,
    InvalidUrl,
    MinLength,
    MaxLength,
    MinValue,
    MaxValue,
    Pattern,
    Custom,
}

/// Result type for validation
pub type ValidationResult<T = ()> = Result<T, ValidationError>;

/// Core validator trait
pub trait Validator<T> {
    fn validate(&self, value: &T) -> ValidationResult;

    /// Combine this validator with another using AND logic
    fn and<V: Validator<T>>(self, other: V) -> AndValidator<T, Self, V>
    where
        Self: Sized,
    {
        AndValidator {
            first: self,
            second: other,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Map validation errors to custom messages
    fn with_message(self, message: impl Into<String>) -> MessageValidator<T, Self>
    where
        Self: Sized,
    {
        MessageValidator {
            validator: self,
            message: message.into(),
            _phantom: std::marker::PhantomData,
        }
    }
}

/// Validator that combines two validators with AND logic
pub struct AndValidator<T, V1, V2> {
    first: V1,
    second: V2,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, V1: Validator<T>, V2: Validator<T>> Validator<T> for AndValidator<T, V1, V2> {
    fn validate(&self, value: &T) -> ValidationResult {
        self.first.validate(value)?;
        self.second.validate(value)?;
        Ok(())
    }
}

/// Validator that overrides the error message
pub struct MessageValidator<T, V> {
    validator: V,
    message: String,
    _phantom: std::marker::PhantomData<T>,
}

impl<T, V: Validator<T>> Validator<T> for MessageValidator<T, V> {
    fn validate(&self, value: &T) -> ValidationResult {
        self.validator
            .validate(value)
            .map_err(|_| ValidationError::custom(&self.message))
    }
}
