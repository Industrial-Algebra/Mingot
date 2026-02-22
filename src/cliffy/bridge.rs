//! Bridge utilities between Leptos signals and Cliffy behaviors
//!
//! These functions help connect the Leptos reactive system with
//! Cliffy's FRP primitives for geometric state management.

use cliffy_core::{behavior, Behavior};
use leptos::prelude::*;

/// Create a Cliffy Behavior<f64> from an f64 value.
///
/// # Example
///
/// ```rust,ignore
/// use mingot::cliffy::f64_to_behavior;
///
/// let behavior = f64_to_behavior(0.0);
/// behavior.set(45.0);
/// assert_eq!(behavior.sample(), 45.0);
/// ```
pub fn f64_to_behavior(value: f64) -> Behavior<f64> {
    behavior(value)
}

/// Create a Cliffy Behavior<f32> from an f32 value.
pub fn f32_to_behavior(value: f32) -> Behavior<f32> {
    behavior(value)
}

/// Create a Cliffy Behavior<bool> from a bool value.
pub fn bool_to_behavior(value: bool) -> Behavior<bool> {
    behavior(value)
}

/// Create a Cliffy Behavior<i32> from an i32 value.
pub fn i32_to_behavior(value: i32) -> Behavior<i32> {
    behavior(value)
}

/// Create a Cliffy Behavior<i64> from an i64 value.
pub fn i64_to_behavior(value: i64) -> Behavior<i64> {
    behavior(value)
}

/// Sync a Cliffy Behavior<f64> to a Leptos WriteSignal<f64>.
///
/// When the behavior changes, the signal will be updated.
/// This is useful for displaying Cliffy state in Leptos UI.
///
/// # Example
///
/// ```rust,ignore
/// use mingot::cliffy::{f64_to_behavior, sync_f64_behavior_to_signal};
/// use leptos::prelude::*;
///
/// let behavior = f64_to_behavior(45.0);
/// let (_, set_angle) = signal(0.0f64);
///
/// sync_f64_behavior_to_signal(&behavior, set_angle);
/// ```
pub fn sync_f64_behavior_to_signal(behavior: &Behavior<f64>, signal: WriteSignal<f64>) {
    let initial = behavior.sample();
    signal.set(initial);

    behavior.subscribe(move |value| {
        signal.set(*value);
    });
}

/// Sync a Cliffy Behavior<f32> to a Leptos WriteSignal<f32>.
pub fn sync_f32_behavior_to_signal(behavior: &Behavior<f32>, signal: WriteSignal<f32>) {
    let initial = behavior.sample();
    signal.set(initial);

    behavior.subscribe(move |value| {
        signal.set(*value);
    });
}

/// Sync a Cliffy Behavior<bool> to a Leptos WriteSignal<bool>.
pub fn sync_bool_behavior_to_signal(behavior: &Behavior<bool>, signal: WriteSignal<bool>) {
    let initial = behavior.sample();
    signal.set(initial);

    behavior.subscribe(move |value| {
        signal.set(*value);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_f64_to_behavior() {
        let behavior = f64_to_behavior(42.0);
        assert!((behavior.sample() - 42.0).abs() < 1e-10);
    }

    #[test]
    fn test_f32_to_behavior() {
        let behavior = f32_to_behavior(42.0);
        assert!((behavior.sample() - 42.0).abs() < 1e-5);
    }

    #[test]
    fn test_bool_to_behavior() {
        let behavior = bool_to_behavior(true);
        assert!(behavior.sample());
    }

    #[test]
    fn test_i32_to_behavior() {
        let behavior = i32_to_behavior(42);
        assert_eq!(behavior.sample(), 42);
    }

    #[test]
    fn test_i64_to_behavior() {
        let behavior = i64_to_behavior(42);
        assert_eq!(behavior.sample(), 42);
    }
}
