//! Geometric animation system using GPU-accelerated rotor SLERP.
//!
//! Provides frame-budget-aware geometric animations that interpolate
//! between GA3 states using spherical linear interpolation. Animations
//! are driven by `requestAnimationFrame` and produce reactive Leptos signals.
//!
//! # Feature Flag
//!
//! This module requires the `cliffy-full` feature:
//!
//! ```toml
//! mingot = { version = "0.7", features = ["cliffy-full"] }
//! ```

use std::time::Duration;

/// Frame budget for animation computations.
///
/// Controls how much time is allocated for GPU/SIMD computation per frame.
/// If computation exceeds the budget, the animation system may skip frames
/// to maintain UI responsiveness.
#[derive(Clone, Copy, Debug)]
pub struct FrameBudget {
    /// Maximum time for GPU compute per frame in milliseconds (default: 8ms for 60fps).
    pub compute_ms: f64,
    /// Target frames per second (default: 60).
    pub target_fps: f64,
}

impl Default for FrameBudget {
    fn default() -> Self {
        Self {
            compute_ms: 8.0,
            target_fps: 60.0,
        }
    }
}

impl FrameBudget {
    /// Create a frame budget for a target frame rate.
    ///
    /// Allocates half the frame time for compute, leaving the rest for rendering.
    pub fn for_fps(fps: f64) -> Self {
        Self {
            compute_ms: (1000.0 / fps) / 2.0,
            target_fps: fps,
        }
    }

    /// Frame interval in milliseconds.
    pub fn frame_interval_ms(&self) -> f64 {
        1000.0 / self.target_fps
    }
}

/// Easing function type.
///
/// Maps input t (0.0 to 1.0) to output (typically 0.0 to 1.0).
pub type EasingFn = fn(f64) -> f64;

/// Built-in easing functions for geometric animations.
pub mod easing {
    /// Linear interpolation (no easing).
    pub fn linear(t: f64) -> f64 {
        t
    }

    /// Quadratic ease-in (accelerating from zero).
    pub fn ease_in_quad(t: f64) -> f64 {
        t * t
    }

    /// Quadratic ease-out (decelerating to zero).
    pub fn ease_out_quad(t: f64) -> f64 {
        t * (2.0 - t)
    }

    /// Quadratic ease-in-out (accelerating then decelerating).
    pub fn ease_in_out_quad(t: f64) -> f64 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            -1.0 + (4.0 - 2.0 * t) * t
        }
    }

    /// Cubic ease-in (accelerating from zero).
    pub fn ease_in_cubic(t: f64) -> f64 {
        t * t * t
    }

    /// Cubic ease-out (decelerating to zero).
    pub fn ease_out_cubic(t: f64) -> f64 {
        let u = t - 1.0;
        u * u * u + 1.0
    }

    /// Cubic ease-in-out (accelerating then decelerating).
    pub fn ease_in_out_cubic(t: f64) -> f64 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            let u = 2.0 * t - 2.0;
            (u * u * u + 2.0) / 2.0
        }
    }

    /// Sinusoidal ease-in.
    pub fn ease_in_sine(t: f64) -> f64 {
        1.0 - (t * std::f64::consts::FRAC_PI_2).cos()
    }

    /// Sinusoidal ease-out.
    pub fn ease_out_sine(t: f64) -> f64 {
        (t * std::f64::consts::FRAC_PI_2).sin()
    }
}

/// Configuration for a geometric animation.
///
/// Describes an animation that interpolates between GA3 states over time,
/// using GPU-accelerated SLERP when available.
#[derive(Clone)]
pub struct AnimationConfig {
    /// Animation duration.
    pub duration: Duration,
    /// Frame budget for computation.
    pub budget: FrameBudget,
    /// Easing function.
    pub easing: EasingFn,
    /// Whether the animation should loop.
    pub looping: bool,
    /// Whether the animation should play in reverse on alternate loops.
    pub ping_pong: bool,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(1000),
            budget: FrameBudget::default(),
            easing: easing::linear,
            looping: false,
            ping_pong: false,
        }
    }
}

impl AnimationConfig {
    /// Create an animation config with a duration in milliseconds.
    pub fn duration_ms(ms: u64) -> Self {
        Self {
            duration: Duration::from_millis(ms),
            ..Default::default()
        }
    }

    /// Set the easing function.
    pub fn with_easing(mut self, easing: EasingFn) -> Self {
        self.easing = easing;
        self
    }

    /// Set the frame budget.
    pub fn with_budget(mut self, budget: FrameBudget) -> Self {
        self.budget = budget;
        self
    }

    /// Enable looping.
    pub fn with_loop(mut self) -> Self {
        self.looping = true;
        self
    }

    /// Enable ping-pong (reverse on alternate loops).
    pub fn with_ping_pong(mut self) -> Self {
        self.ping_pong = true;
        self.looping = true;
        self
    }

    /// Apply the easing function to a progress value.
    pub fn apply_easing(&self, t: f64) -> f64 {
        (self.easing)(t.clamp(0.0, 1.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_frame_budget_default() {
        let budget = FrameBudget::default();
        assert!((budget.compute_ms - 8.0).abs() < 1e-10);
        assert!((budget.target_fps - 60.0).abs() < 1e-10);
    }

    #[test]
    fn test_frame_budget_for_fps() {
        let budget = FrameBudget::for_fps(30.0);
        assert!((budget.target_fps - 30.0).abs() < 1e-10);
        // 33.33ms / 2 = 16.67ms compute budget
        assert!((budget.compute_ms - 16.666666666666668).abs() < 1e-5);
    }

    #[test]
    fn test_frame_interval() {
        let budget = FrameBudget::default();
        assert!((budget.frame_interval_ms() - 16.666666666666668).abs() < 1e-5);
    }

    #[test]
    fn test_easing_linear() {
        assert!((easing::linear(0.0)).abs() < 1e-10);
        assert!((easing::linear(0.5) - 0.5).abs() < 1e-10);
        assert!((easing::linear(1.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_easing_quad() {
        // ease_in_quad at t=0.5 should be 0.25
        assert!((easing::ease_in_quad(0.5) - 0.25).abs() < 1e-10);
        // ease_out_quad at t=0.5 should be 0.75
        assert!((easing::ease_out_quad(0.5) - 0.75).abs() < 1e-10);
        // ease_in_out_quad at t=0.5 should be 0.5
        assert!((easing::ease_in_out_quad(0.5) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_easing_cubic() {
        // ease_in_cubic at t=0.5 should be 0.125
        assert!((easing::ease_in_cubic(0.5) - 0.125).abs() < 1e-10);
        // All easing functions should map 0->0 and 1->1
        assert!((easing::ease_out_cubic(0.0)).abs() < 1e-10);
        assert!((easing::ease_out_cubic(1.0) - 1.0).abs() < 1e-10);
        assert!((easing::ease_in_out_cubic(0.0)).abs() < 1e-10);
        assert!((easing::ease_in_out_cubic(1.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_easing_sine() {
        assert!((easing::ease_in_sine(0.0)).abs() < 1e-10);
        assert!((easing::ease_in_sine(1.0) - 1.0).abs() < 1e-10);
        assert!((easing::ease_out_sine(0.0)).abs() < 1e-10);
        assert!((easing::ease_out_sine(1.0) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_animation_config_default() {
        let config = AnimationConfig::default();
        assert_eq!(config.duration, Duration::from_millis(1000));
        assert!(!config.looping);
        assert!(!config.ping_pong);
    }

    #[test]
    fn test_animation_config_builder() {
        let config = AnimationConfig::duration_ms(500)
            .with_easing(easing::ease_in_quad)
            .with_loop()
            .with_budget(FrameBudget::for_fps(30.0));
        assert_eq!(config.duration, Duration::from_millis(500));
        assert!(config.looping);
        assert!((config.budget.target_fps - 30.0).abs() < 1e-10);
    }

    #[test]
    fn test_animation_config_apply_easing() {
        let config = AnimationConfig::default().with_easing(easing::ease_in_quad);
        assert!((config.apply_easing(0.5) - 0.25).abs() < 1e-10);
        // Clamping
        assert!((config.apply_easing(-0.5)).abs() < 1e-10);
        assert!((config.apply_easing(1.5) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_animation_config_ping_pong_enables_loop() {
        let config = AnimationConfig::default().with_ping_pong();
        assert!(config.looping);
        assert!(config.ping_pong);
    }
}
