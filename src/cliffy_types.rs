//! Shared type aliases for Cliffy integration
//!
//! These types provide compile-time feature gating for Cliffy props.
//! When the `cliffy` feature is disabled, stub types are used that
//! have no effect at runtime.

// ============================================================================
// BEHAVIOR TYPES - For numeric/scalar state
// ============================================================================

/// Behavior type for f64 values (slider, number input, etc.)
#[cfg(feature = "cliffy")]
pub type BehaviorF64 = cliffy_core::Behavior<f64>;

#[cfg(not(feature = "cliffy"))]
#[derive(Clone)]
pub struct BehaviorF64;

#[cfg(not(feature = "cliffy"))]
impl BehaviorF64 {
    #[allow(dead_code)]
    pub fn set(&self, _value: f64) {}
    #[allow(dead_code)]
    pub fn sample(&self) -> f64 {
        0.0
    }
}

/// Behavior type for (f64, f64) tuple values (range slider, etc.)
#[cfg(feature = "cliffy")]
pub type BehaviorF64Tuple = cliffy_core::Behavior<(f64, f64)>;

#[cfg(not(feature = "cliffy"))]
#[derive(Clone)]
pub struct BehaviorF64Tuple;

#[cfg(not(feature = "cliffy"))]
impl BehaviorF64Tuple {
    #[allow(dead_code)]
    pub fn set(&self, _value: (f64, f64)) {}
    #[allow(dead_code)]
    pub fn sample(&self) -> (f64, f64) {
        (0.0, 0.0)
    }
}

/// Behavior type for String values (text input, number input string, etc.)
#[cfg(feature = "cliffy")]
pub type BehaviorString = cliffy_core::Behavior<String>;

#[cfg(not(feature = "cliffy"))]
#[derive(Clone)]
pub struct BehaviorString;

#[cfg(not(feature = "cliffy"))]
impl BehaviorString {
    #[allow(dead_code)]
    pub fn set(&self, _value: String) {}
    #[allow(dead_code)]
    pub fn sample(&self) -> String {
        String::new()
    }
}

/// Behavior type for i64 values
#[cfg(feature = "cliffy")]
pub type BehaviorI64 = cliffy_core::Behavior<i64>;

#[cfg(not(feature = "cliffy"))]
#[derive(Clone)]
pub struct BehaviorI64;

#[cfg(not(feature = "cliffy"))]
impl BehaviorI64 {
    #[allow(dead_code)]
    pub fn set(&self, _value: i64) {}
    #[allow(dead_code)]
    pub fn sample(&self) -> i64 {
        0
    }
}

/// Behavior type for bool values
#[cfg(feature = "cliffy")]
pub type BehaviorBool = cliffy_core::Behavior<bool>;

#[cfg(not(feature = "cliffy"))]
#[derive(Clone)]
pub struct BehaviorBool;

#[cfg(not(feature = "cliffy"))]
impl BehaviorBool {
    #[allow(dead_code)]
    pub fn set(&self, _value: bool) {}
    #[allow(dead_code)]
    pub fn sample(&self) -> bool {
        false
    }
}

// Note: More complex types like Vec<f64>, tuples with Vec, etc. are not supported
// by cliffy-core's Behavior type. For components like VectorInput and CoordinateInput,
// use individual BehaviorF64 props for each coordinate component if needed.
