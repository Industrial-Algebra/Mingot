//! Shared type aliases for cliffy-gpu hardware acceleration
//!
//! These types provide compile-time feature gating for GPU-accelerated operations.
//! When the `cliffy-full` feature is disabled, stub types are used that
//! fall back to CPU-only operations.
//!
//! cliffy-gpu provides:
//! - WebGPU compute shaders for massive parallel computation
//! - SIMD-optimized CPU operations as fallback
//! - Auto dispatch between GPU and CPU based on batch size

// ============================================================================
// GPU MULTIVECTOR TYPES
// ============================================================================

/// GPU-compatible multivector representation.
/// When cliffy-full is disabled, this is a stub that wraps a simple f64 array.
#[cfg(feature = "cliffy-full")]
pub type GpuMultivector = cliffy_gpu::GpuMultivector;

#[cfg(not(feature = "cliffy-full"))]
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct GpuMultivector {
    pub coeffs: [f32; 8],
}

#[cfg(not(feature = "cliffy-full"))]
impl GpuMultivector {
    #[allow(dead_code)]
    pub fn zero() -> Self {
        Self { coeffs: [0.0; 8] }
    }

    #[allow(dead_code)]
    pub fn scalar(s: f32) -> Self {
        let mut mv = Self::zero();
        mv.coeffs[0] = s;
        mv
    }

    #[allow(dead_code)]
    pub fn from_f64(value: f64) -> Self {
        Self::scalar(value as f32)
    }

    #[allow(dead_code)]
    pub fn to_scalar(&self) -> f32 {
        self.coeffs[0]
    }
}

// ============================================================================
// SIMD BATCH TYPES
// ============================================================================

/// SIMD batch operations wrapper.
/// Provides optimized CPU operations for geometric algebra.
#[cfg(feature = "cliffy-full")]
pub use cliffy_gpu::SimdBatch;

#[cfg(not(feature = "cliffy-full"))]
pub struct SimdBatch;

#[cfg(not(feature = "cliffy-full"))]
impl SimdBatch {
    /// Batch geometric product (CPU fallback - no SIMD).
    #[allow(dead_code)]
    pub fn geometric_product(a: &[GpuMultivector], b: &[GpuMultivector]) -> Vec<GpuMultivector> {
        // Simple scalar multiplication fallback
        a.iter()
            .zip(b.iter())
            .map(|(av, bv)| {
                let mut result = GpuMultivector::zero();
                result.coeffs[0] = av.coeffs[0] * bv.coeffs[0];
                result
            })
            .collect()
    }

    /// Batch addition (CPU fallback - no SIMD).
    #[allow(dead_code)]
    pub fn add(a: &[GpuMultivector], b: &[GpuMultivector]) -> Vec<GpuMultivector> {
        a.iter()
            .zip(b.iter())
            .map(|(av, bv)| {
                let mut result = GpuMultivector::zero();
                for i in 0..8 {
                    result.coeffs[i] = av.coeffs[i] + bv.coeffs[i];
                }
                result
            })
            .collect()
    }
}

// ============================================================================
// GPU CONTEXT TYPES
// ============================================================================

/// GPU computation context.
/// When cliffy-full is disabled, this is a no-op stub.
#[cfg(not(feature = "cliffy-full"))]
pub struct GpuContext {
    _available: bool,
}

#[cfg(not(feature = "cliffy-full"))]
impl GpuContext {
    /// Check if GPU is available (always false without cliffy-full).
    #[allow(dead_code)]
    pub fn is_available() -> bool {
        false
    }

    /// Create a new GPU context (no-op without cliffy-full).
    #[allow(dead_code)]
    pub async fn new() -> Option<Self> {
        None
    }
}

// ============================================================================
// ACCELERATION PREFERENCE
// ============================================================================

/// Preference for hardware acceleration.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum AccelerationPreference {
    /// Automatically choose based on batch size and hardware availability.
    #[default]
    Auto,
    /// Prefer GPU acceleration when available.
    PreferGpu,
    /// Prefer SIMD CPU acceleration.
    PreferSimd,
    /// Use simple CPU operations only.
    CpuOnly,
}

impl AccelerationPreference {
    /// Check if this preference requests GPU acceleration.
    #[allow(dead_code)]
    pub fn wants_gpu(&self) -> bool {
        matches!(self, Self::Auto | Self::PreferGpu)
    }

    /// Check if this preference requests SIMD acceleration.
    #[allow(dead_code)]
    pub fn wants_simd(&self) -> bool {
        matches!(self, Self::Auto | Self::PreferSimd)
    }
}
