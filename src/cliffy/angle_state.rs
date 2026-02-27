//! Geometric angle state using cliffy-core
//!
//! Provides angle state management with geometric algebra awareness,
//! enabling rotor-based rotations and geometric composition.

use cliffy_core::{behavior, Behavior, Rotor};
use std::f64::consts::PI;

/// A geometric angle state that stores angles with rotor support.
///
/// This type wraps a `Behavior<f64>` for the angle value (in radians)
/// and provides methods for geometric operations like:
/// - Creating rotors from the angle
/// - Interpolating between angles using SLERP
/// - Composing rotations
///
/// # Example
///
/// ```rust,ignore
/// use mingot::cliffy::GeometricAngleState;
/// use std::f64::consts::PI;
///
/// let angle = GeometricAngleState::new(PI / 4.0);
///
/// // Get current angle
/// assert_eq!(angle.radians(), PI / 4.0);
/// assert_eq!(angle.degrees(), 45.0);
///
/// // Create a rotor for 3D rotation
/// let rotor = angle.to_rotor_xy();
///
/// // Update angle
/// angle.set_degrees(90.0);
/// ```
pub struct GeometricAngleState {
    /// The underlying behavior storing angle in radians
    angle: Behavior<f64>,
}

impl GeometricAngleState {
    /// Create a new geometric angle state from radians
    pub fn new(radians: f64) -> Self {
        Self {
            angle: behavior(radians),
        }
    }

    /// Create a new geometric angle state from degrees
    pub fn from_degrees(degrees: f64) -> Self {
        Self::new(degrees * PI / 180.0)
    }

    /// Create a zero angle state
    pub fn zero() -> Self {
        Self::new(0.0)
    }

    /// Get the current angle in radians
    pub fn radians(&self) -> f64 {
        self.angle.sample()
    }

    /// Get the current angle in degrees
    pub fn degrees(&self) -> f64 {
        self.radians() * 180.0 / PI
    }

    /// Set the angle in radians
    pub fn set(&self, radians: f64) {
        self.angle.set(radians);
    }

    /// Set the angle in degrees
    pub fn set_degrees(&self, degrees: f64) {
        self.set(degrees * PI / 180.0);
    }

    /// Update the angle using a transformation function
    pub fn update<F>(&self, f: F)
    where
        F: FnOnce(f64) -> f64,
    {
        self.angle.update(f);
    }

    /// Add to the current angle (in radians)
    pub fn add(&self, radians: f64) {
        self.update(|current| current + radians);
    }

    /// Add to the current angle (in degrees)
    pub fn add_degrees(&self, degrees: f64) {
        self.add(degrees * PI / 180.0);
    }

    /// Create a rotor for rotation in the XY plane (around Z axis)
    ///
    /// This is the most common rotation for 2D angle inputs.
    pub fn to_rotor_xy(&self) -> Rotor {
        Rotor::xy(self.radians())
    }

    /// Create a rotor for rotation in the XZ plane (around Y axis)
    pub fn to_rotor_xz(&self) -> Rotor {
        Rotor::xz(self.radians())
    }

    /// Create a rotor for rotation in the YZ plane (around X axis)
    pub fn to_rotor_yz(&self) -> Rotor {
        Rotor::yz(self.radians())
    }

    /// Create a rotor from axis and angle
    ///
    /// The axis vector (x, y, z) defines the rotation axis.
    pub fn to_rotor_axis(&self, x: f64, y: f64, z: f64) -> Rotor {
        Rotor::from_axis_angle(x, y, z, self.radians())
    }

    /// Set the angle from a rotor
    ///
    /// Extracts the rotation angle from the rotor.
    pub fn set_from_rotor(&self, rotor: &Rotor) {
        self.set(rotor.angle());
    }

    /// Compose this angle with another using geometric product
    ///
    /// This effectively adds the rotations together when using rotors.
    pub fn compose(&self, other: &GeometricAngleState) -> GeometricAngleState {
        let r1 = self.to_rotor_xy();
        let r2 = other.to_rotor_xy();
        let composed = r1.then(&r2);
        let result = GeometricAngleState::zero();
        result.set_from_rotor(&composed);
        result
    }

    /// Interpolate to another angle using spherical linear interpolation (SLERP)
    ///
    /// `t` should be in range [0, 1] where:
    /// - t=0 returns this angle
    /// - t=1 returns the other angle
    /// - t=0.5 returns the midpoint
    pub fn slerp_to(&self, other: &GeometricAngleState, t: f64) -> GeometricAngleState {
        let r1 = self.to_rotor_xy();
        let r2 = other.to_rotor_xy();
        let interpolated = r1.slerp_to(&r2, t);
        let result = GeometricAngleState::zero();
        result.set_from_rotor(&interpolated);
        result
    }

    /// Normalize the angle to [0, 2π)
    pub fn normalize_positive(&self) {
        self.update(|angle| {
            let mut normalized = angle % (2.0 * PI);
            if normalized < 0.0 {
                normalized += 2.0 * PI;
            }
            normalized
        });
    }

    /// Normalize the angle to [-π, π]
    pub fn normalize_symmetric(&self) {
        self.update(|angle| {
            let mut normalized = angle % (2.0 * PI);
            if normalized > PI {
                normalized -= 2.0 * PI;
            } else if normalized < -PI {
                normalized += 2.0 * PI;
            }
            normalized
        });
    }

    /// Subscribe to angle changes
    ///
    /// The callback receives the new angle value in radians.
    pub fn subscribe<F>(&self, callback: F)
    where
        F: Fn(&f64) + 'static,
    {
        self.angle.subscribe(callback);
    }

    /// Get the underlying behavior for advanced operations
    pub fn behavior(&self) -> &Behavior<f64> {
        &self.angle
    }
}

impl Default for GeometricAngleState {
    fn default() -> Self {
        Self::zero()
    }
}

impl Clone for GeometricAngleState {
    fn clone(&self) -> Self {
        Self::new(self.radians())
    }
}

/// Extension trait for creating rotors from angles
pub trait AngleRotor {
    /// Create a rotor from this angle value (in radians) for XY plane rotation
    fn to_rotor_xy(&self) -> Rotor;

    /// Create a rotor from this angle value (in radians) for XZ plane rotation
    fn to_rotor_xz(&self) -> Rotor;

    /// Create a rotor from this angle value (in radians) for YZ plane rotation
    fn to_rotor_yz(&self) -> Rotor;
}

impl AngleRotor for f64 {
    fn to_rotor_xy(&self) -> Rotor {
        Rotor::xy(*self)
    }

    fn to_rotor_xz(&self) -> Rotor {
        Rotor::xz(*self)
    }

    fn to_rotor_yz(&self) -> Rotor {
        Rotor::yz(*self)
    }
}

impl AngleRotor for f32 {
    fn to_rotor_xy(&self) -> Rotor {
        Rotor::xy(*self as f64)
    }

    fn to_rotor_xz(&self) -> Rotor {
        Rotor::xz(*self as f64)
    }

    fn to_rotor_yz(&self) -> Rotor {
        Rotor::yz(*self as f64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_geometric_angle_state_new() {
        let state = GeometricAngleState::new(PI / 4.0);
        assert!((state.radians() - PI / 4.0).abs() < 1e-10);
        assert!((state.degrees() - 45.0).abs() < 1e-10);
    }

    #[test]
    fn test_geometric_angle_state_from_degrees() {
        let state = GeometricAngleState::from_degrees(90.0);
        assert!((state.radians() - PI / 2.0).abs() < 1e-10);
        assert!((state.degrees() - 90.0).abs() < 1e-10);
    }

    #[test]
    fn test_geometric_angle_state_set() {
        let state = GeometricAngleState::zero();
        state.set(PI);
        assert!((state.radians() - PI).abs() < 1e-10);

        state.set_degrees(180.0);
        assert!((state.radians() - PI).abs() < 1e-10);
    }

    #[test]
    fn test_geometric_angle_state_add() {
        let state = GeometricAngleState::new(PI / 4.0);
        state.add(PI / 4.0);
        assert!((state.radians() - PI / 2.0).abs() < 1e-10);

        state.add_degrees(45.0);
        assert!((state.degrees() - 135.0).abs() < 1e-10);
    }

    #[test]
    fn test_rotor_xy() {
        let state = GeometricAngleState::new(PI / 2.0);
        let rotor = state.to_rotor_xy();
        assert!((rotor.angle() - PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_normalize_positive() {
        let state = GeometricAngleState::new(-PI / 2.0);
        state.normalize_positive();
        assert!((state.radians() - 3.0 * PI / 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_normalize_symmetric() {
        let state = GeometricAngleState::new(3.0 * PI / 2.0);
        state.normalize_symmetric();
        assert!((state.radians() - (-PI / 2.0)).abs() < 1e-10);
    }

    #[test]
    fn test_set_from_rotor() {
        let state = GeometricAngleState::zero();
        let rotor = Rotor::xy(PI / 3.0);
        state.set_from_rotor(&rotor);
        assert!((state.radians() - PI / 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_slerp() {
        let state1 = GeometricAngleState::new(0.0);
        let state2 = GeometricAngleState::new(PI);

        let midpoint = state1.slerp_to(&state2, 0.5);
        assert!((midpoint.radians() - PI / 2.0).abs() < 1e-10);
    }
}
