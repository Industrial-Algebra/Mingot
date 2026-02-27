//! Cliffy ecosystem integration for Mingot components
//!
//! This module provides geometric algebra state management using cliffy-core,
//! enabling advanced mathematical operations on UI component state.
//!
//! # Feature Flag
//!
//! This module requires the `cliffy` feature to be enabled:
//!
//! ```toml
//! mingot = { version = "0.6", features = ["cliffy"] }
//! ```
//!
//! # Components
//!
//! - [`GeometricAngleState`]: Angle state with rotor support
//! - [`GeometricVectorState`]: 3D vector state with GA transformations
//! - [`GeometricCoordinateState2D`]: 2D coordinate state with polar/Cartesian support
//! - Bridge functions for Leptos signal integration

pub mod angle_state;
pub mod bridge;
pub mod vector_state;

pub use angle_state::*;
pub use bridge::*;
pub use vector_state::*;

// Re-export commonly used cliffy-core types
pub use cliffy_core::{behavior, Behavior, Rotor};

/// Convenient re-exports from cliffy-core for geometric operations
pub mod prelude {
    pub use super::angle_state::{AngleRotor, GeometricAngleState};
    pub use super::bridge::{
        bool_to_behavior, f32_to_behavior, f64_to_behavior, i32_to_behavior, i64_to_behavior,
        sync_bool_behavior_to_signal, sync_f32_behavior_to_signal, sync_f64_behavior_to_signal,
    };
    pub use super::vector_state::{GeometricCoordinateState2D, GeometricVectorState};
    pub use cliffy_core::{behavior, Behavior, Rotor};
}
