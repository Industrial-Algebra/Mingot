//! Geometric vector state using cliffy-core
//!
//! Provides 3D vector state management with geometric algebra awareness,
//! enabling rotor-based transformations and geometric operations.

use cliffy_core::{behavior, Behavior, GeometricState, Rotor};

/// Create a vector from components and apply rotor transformation.
///
/// This uses cliffy-core's GeometricState to handle the transformation.
fn transform_vector(rotor: &Rotor, x: f64, y: f64, z: f64) -> (f64, f64, f64) {
    let state = GeometricState::from_vector(x, y, z);
    let transformed = state.apply_rotor(rotor);
    transformed.as_vector()
}

/// A geometric 3D vector state that stores vectors with GA support.
///
/// This type wraps Behavior<f64> for each component (x, y, z) and provides
/// methods for geometric operations like:
/// - Applying rotor transformations
/// - Computing magnitude and direction
/// - Vector arithmetic
///
/// # Example
///
/// ```rust,ignore
/// use mingot::cliffy::GeometricVectorState;
///
/// let vec = GeometricVectorState::new(1.0, 0.0, 0.0);
///
/// // Get components
/// assert_eq!(vec.x(), 1.0);
/// assert_eq!(vec.y(), 0.0);
/// assert_eq!(vec.z(), 0.0);
///
/// // Compute magnitude
/// assert_eq!(vec.magnitude(), 1.0);
///
/// // Apply rotation (90 degrees around Z axis)
/// let rotated = vec.rotate_xy(std::f64::consts::FRAC_PI_2);
/// // Now rotated is approximately (0, 1, 0)
/// ```
pub struct GeometricVectorState {
    x: Behavior<f64>,
    y: Behavior<f64>,
    z: Behavior<f64>,
}

impl GeometricVectorState {
    /// Create a new 3D vector state
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self {
            x: behavior(x),
            y: behavior(y),
            z: behavior(z),
        }
    }

    /// Create a zero vector
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }

    /// Create a unit vector along X axis
    pub fn unit_x() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    /// Create a unit vector along Y axis
    pub fn unit_y() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    /// Create a unit vector along Z axis
    pub fn unit_z() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    /// Get the X component
    pub fn x(&self) -> f64 {
        self.x.sample()
    }

    /// Get the Y component
    pub fn y(&self) -> f64 {
        self.y.sample()
    }

    /// Get the Z component
    pub fn z(&self) -> f64 {
        self.z.sample()
    }

    /// Set the X component
    pub fn set_x(&self, value: f64) {
        self.x.set(value);
    }

    /// Set the Y component
    pub fn set_y(&self, value: f64) {
        self.y.set(value);
    }

    /// Set the Z component
    pub fn set_z(&self, value: f64) {
        self.z.set(value);
    }

    /// Set all components at once
    pub fn set(&self, x: f64, y: f64, z: f64) {
        self.x.set(x);
        self.y.set(y);
        self.z.set(z);
    }

    /// Get all components as a tuple
    pub fn components(&self) -> (f64, f64, f64) {
        (self.x(), self.y(), self.z())
    }

    /// Get all components as an array
    pub fn to_array(&self) -> [f64; 3] {
        [self.x(), self.y(), self.z()]
    }

    /// Compute the magnitude (length) of the vector
    pub fn magnitude(&self) -> f64 {
        let (x, y, z) = self.components();
        (x * x + y * y + z * z).sqrt()
    }

    /// Compute the squared magnitude (avoids sqrt)
    pub fn magnitude_squared(&self) -> f64 {
        let (x, y, z) = self.components();
        x * x + y * y + z * z
    }

    /// Normalize the vector to unit length
    pub fn normalize(&self) {
        let mag = self.magnitude();
        if mag > f64::EPSILON {
            let (x, y, z) = self.components();
            self.set(x / mag, y / mag, z / mag);
        }
    }

    /// Return a new normalized vector state
    pub fn normalized(&self) -> Self {
        let mag = self.magnitude();
        if mag > f64::EPSILON {
            let (x, y, z) = self.components();
            Self::new(x / mag, y / mag, z / mag)
        } else {
            Self::zero()
        }
    }

    /// Scale the vector by a scalar
    pub fn scale(&self, factor: f64) {
        let (x, y, z) = self.components();
        self.set(x * factor, y * factor, z * factor);
    }

    /// Return a new scaled vector state
    pub fn scaled(&self, factor: f64) -> Self {
        let (x, y, z) = self.components();
        Self::new(x * factor, y * factor, z * factor)
    }

    /// Compute the dot product with another vector
    pub fn dot(&self, other: &GeometricVectorState) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    /// Compute the cross product with another vector
    pub fn cross(&self, other: &GeometricVectorState) -> Self {
        let (ax, ay, az) = self.components();
        let (bx, by, bz) = other.components();

        Self::new(ay * bz - az * by, az * bx - ax * bz, ax * by - ay * bx)
    }

    /// Apply a rotor transformation to the vector
    ///
    /// This performs the sandwich product: R v R†
    pub fn apply_rotor(&self, rotor: &Rotor) {
        let (x, y, z) = transform_vector(rotor, self.x(), self.y(), self.z());
        self.set(x, y, z);
    }

    /// Return a new vector with rotor transformation applied
    pub fn rotated(&self, rotor: &Rotor) -> Self {
        let (x, y, z) = transform_vector(rotor, self.x(), self.y(), self.z());
        Self::new(x, y, z)
    }

    /// Rotate in the XY plane (around Z axis)
    pub fn rotate_xy(&self, angle: f64) {
        let rotor = Rotor::xy(angle);
        self.apply_rotor(&rotor);
    }

    /// Rotate in the XZ plane (around Y axis)
    pub fn rotate_xz(&self, angle: f64) {
        let rotor = Rotor::xz(angle);
        self.apply_rotor(&rotor);
    }

    /// Rotate in the YZ plane (around X axis)
    pub fn rotate_yz(&self, angle: f64) {
        let rotor = Rotor::yz(angle);
        self.apply_rotor(&rotor);
    }

    /// Add another vector to this one
    pub fn add(&self, other: &GeometricVectorState) {
        self.set(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        );
    }

    /// Subtract another vector from this one
    pub fn sub(&self, other: &GeometricVectorState) {
        self.set(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        );
    }

    /// Return the sum of two vectors
    pub fn plus(&self, other: &GeometricVectorState) -> Self {
        Self::new(
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }

    /// Return the difference of two vectors
    pub fn minus(&self, other: &GeometricVectorState) -> Self {
        Self::new(
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }

    /// Negate the vector
    pub fn negate(&self) {
        self.set(-self.x(), -self.y(), -self.z());
    }

    /// Return the negated vector
    pub fn negated(&self) -> Self {
        Self::new(-self.x(), -self.y(), -self.z())
    }

    /// Linear interpolation between this vector and another
    pub fn lerp_to(&self, other: &GeometricVectorState, t: f64) -> Self {
        let t_clamped = t.clamp(0.0, 1.0);
        Self::new(
            self.x() + (other.x() - self.x()) * t_clamped,
            self.y() + (other.y() - self.y()) * t_clamped,
            self.z() + (other.z() - self.z()) * t_clamped,
        )
    }

    /// Subscribe to X component changes
    pub fn subscribe_x<F>(&self, callback: F)
    where
        F: Fn(&f64) + 'static,
    {
        self.x.subscribe(callback);
    }

    /// Subscribe to Y component changes
    pub fn subscribe_y<F>(&self, callback: F)
    where
        F: Fn(&f64) + 'static,
    {
        self.y.subscribe(callback);
    }

    /// Subscribe to Z component changes
    pub fn subscribe_z<F>(&self, callback: F)
    where
        F: Fn(&f64) + 'static,
    {
        self.z.subscribe(callback);
    }

    /// Get the underlying X behavior for advanced operations
    pub fn x_behavior(&self) -> &Behavior<f64> {
        &self.x
    }

    /// Get the underlying Y behavior for advanced operations
    pub fn y_behavior(&self) -> &Behavior<f64> {
        &self.y
    }

    /// Get the underlying Z behavior for advanced operations
    pub fn z_behavior(&self) -> &Behavior<f64> {
        &self.z
    }
}

impl Default for GeometricVectorState {
    fn default() -> Self {
        Self::zero()
    }
}

impl Clone for GeometricVectorState {
    fn clone(&self) -> Self {
        Self::new(self.x(), self.y(), self.z())
    }
}

/// A geometric 2D coordinate state with polar/Cartesian support.
///
/// Stores coordinates as (x, y) internally but provides conversions
/// to polar form (r, θ) and geometric operations.
pub struct GeometricCoordinateState2D {
    x: Behavior<f64>,
    y: Behavior<f64>,
}

impl GeometricCoordinateState2D {
    /// Create from Cartesian coordinates
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            x: behavior(x),
            y: behavior(y),
        }
    }

    /// Create from polar coordinates (r, θ in radians)
    pub fn from_polar(r: f64, theta: f64) -> Self {
        Self::new(r * theta.cos(), r * theta.sin())
    }

    /// Create origin point
    pub fn origin() -> Self {
        Self::new(0.0, 0.0)
    }

    /// Get X coordinate
    pub fn x(&self) -> f64 {
        self.x.sample()
    }

    /// Get Y coordinate
    pub fn y(&self) -> f64 {
        self.y.sample()
    }

    /// Get radius (distance from origin)
    pub fn r(&self) -> f64 {
        let (x, y) = (self.x(), self.y());
        (x * x + y * y).sqrt()
    }

    /// Get angle in radians
    pub fn theta(&self) -> f64 {
        self.y().atan2(self.x())
    }

    /// Get angle in degrees
    pub fn theta_degrees(&self) -> f64 {
        self.theta().to_degrees()
    }

    /// Set Cartesian coordinates
    pub fn set(&self, x: f64, y: f64) {
        self.x.set(x);
        self.y.set(y);
    }

    /// Set polar coordinates
    pub fn set_polar(&self, r: f64, theta: f64) {
        self.set(r * theta.cos(), r * theta.sin());
    }

    /// Rotate by angle (in radians)
    pub fn rotate(&self, angle: f64) {
        let rotor = Rotor::xy(angle);
        let (x, y, _z) = transform_vector(&rotor, self.x(), self.y(), 0.0);
        self.set(x, y);
    }

    /// Scale by factor
    pub fn scale(&self, factor: f64) {
        self.set(self.x() * factor, self.y() * factor);
    }

    /// Translate by offset
    pub fn translate(&self, dx: f64, dy: f64) {
        self.set(self.x() + dx, self.y() + dy);
    }

    /// Get as Cartesian tuple
    pub fn to_cartesian(&self) -> (f64, f64) {
        (self.x(), self.y())
    }

    /// Get as polar tuple (r, theta in radians)
    pub fn to_polar(&self) -> (f64, f64) {
        (self.r(), self.theta())
    }
}

impl Default for GeometricCoordinateState2D {
    fn default() -> Self {
        Self::origin()
    }
}

impl Clone for GeometricCoordinateState2D {
    fn clone(&self) -> Self {
        Self::new(self.x(), self.y())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_vector_new() {
        let v = GeometricVectorState::new(1.0, 2.0, 3.0);
        assert_eq!(v.x(), 1.0);
        assert_eq!(v.y(), 2.0);
        assert_eq!(v.z(), 3.0);
    }

    #[test]
    fn test_vector_magnitude() {
        let v = GeometricVectorState::new(3.0, 4.0, 0.0);
        assert!((v.magnitude() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_vector_normalize() {
        let v = GeometricVectorState::new(3.0, 4.0, 0.0);
        v.normalize();
        assert!((v.magnitude() - 1.0).abs() < 1e-10);
        assert!((v.x() - 0.6).abs() < 1e-10);
        assert!((v.y() - 0.8).abs() < 1e-10);
    }

    #[test]
    fn test_vector_dot() {
        let v1 = GeometricVectorState::new(1.0, 0.0, 0.0);
        let v2 = GeometricVectorState::new(0.0, 1.0, 0.0);
        assert!((v1.dot(&v2)).abs() < 1e-10); // Perpendicular

        let v3 = GeometricVectorState::new(1.0, 2.0, 3.0);
        let v4 = GeometricVectorState::new(4.0, 5.0, 6.0);
        assert!((v3.dot(&v4) - 32.0).abs() < 1e-10); // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_vector_cross() {
        let v1 = GeometricVectorState::unit_x();
        let v2 = GeometricVectorState::unit_y();
        let cross = v1.cross(&v2);
        assert!((cross.x()).abs() < 1e-10);
        assert!((cross.y()).abs() < 1e-10);
        assert!((cross.z() - 1.0).abs() < 1e-10); // x × y = z
    }

    #[test]
    fn test_vector_rotate_xy() {
        let v = GeometricVectorState::new(1.0, 0.0, 0.0);
        v.rotate_xy(PI / 2.0); // 90 degrees
        assert!((v.x()).abs() < 1e-10);
        assert!((v.y() - 1.0).abs() < 1e-10);
        assert!((v.z()).abs() < 1e-10);
    }

    #[test]
    fn test_vector_lerp() {
        let v1 = GeometricVectorState::new(0.0, 0.0, 0.0);
        let v2 = GeometricVectorState::new(10.0, 20.0, 30.0);
        let mid = v1.lerp_to(&v2, 0.5);
        assert!((mid.x() - 5.0).abs() < 1e-10);
        assert!((mid.y() - 10.0).abs() < 1e-10);
        assert!((mid.z() - 15.0).abs() < 1e-10);
    }

    #[test]
    fn test_coordinate_2d_cartesian() {
        let c = GeometricCoordinateState2D::new(3.0, 4.0);
        assert!((c.r() - 5.0).abs() < 1e-10);
    }

    #[test]
    fn test_coordinate_2d_polar() {
        let c = GeometricCoordinateState2D::from_polar(5.0, PI / 4.0);
        assert!((c.r() - 5.0).abs() < 1e-10);
        assert!((c.theta() - PI / 4.0).abs() < 1e-10);
    }

    #[test]
    fn test_coordinate_2d_rotate() {
        let c = GeometricCoordinateState2D::new(1.0, 0.0);
        c.rotate(PI / 2.0);
        assert!((c.x()).abs() < 1e-10);
        assert!((c.y() - 1.0).abs() < 1e-10);
    }
}
