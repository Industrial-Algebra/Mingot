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

// ============================================================================
// BATCH MATRIX OPERATIONS
// ============================================================================

/// A 2D matrix represented as rows of f64 values.
pub type MatrixData = Vec<Vec<f64>>;

/// A pair of matrices for binary operations.
pub type MatrixPair = (MatrixData, MatrixData);

/// Batch matrix operations that can be GPU-accelerated.
#[derive(Clone, Debug, PartialEq)]
pub enum BatchMatrixOp {
    /// Multiply a batch of matrices
    Multiply(Vec<MatrixPair>),
    /// Compute determinants for a batch of matrices
    Determinant(Vec<MatrixData>),
    /// Transpose a batch of matrices
    Transpose(Vec<MatrixData>),
    /// Compute inverses for a batch of matrices
    Inverse(Vec<MatrixData>),
    /// Compute eigenvalues for a batch of matrices
    Eigenvalues(Vec<MatrixData>),
    /// Add matrices element-wise (batch)
    Add(Vec<MatrixPair>),
    /// Scalar multiplication (batch)
    ScalarMul(Vec<(f64, MatrixData)>),
}

/// Result of a batch matrix operation
#[derive(Clone, Debug)]
pub enum BatchMatrixResult {
    /// Results from matrix multiplication
    Matrices(Vec<MatrixData>),
    /// Scalar results (determinants, traces, etc.)
    Scalars(Vec<f64>),
    /// Complex results (eigenvalues as (real, imaginary) pairs)
    Complex(Vec<Vec<(f64, f64)>>),
    /// Error occurred during computation
    Error(String),
}

// ============================================================================
// BATCH VECTOR OPERATIONS
// ============================================================================

/// Batch vector operations that can be GPU-accelerated.
#[derive(Clone, Debug, PartialEq)]
pub enum BatchVectorOp {
    /// Dot products for pairs of vectors
    DotProduct(Vec<(Vec<f64>, Vec<f64>)>),
    /// Cross products for pairs of 3D vectors
    CrossProduct(Vec<(Vec<f64>, Vec<f64>)>),
    /// Normalize a batch of vectors
    Normalize(Vec<Vec<f64>>),
    /// Compute magnitudes for a batch of vectors
    Magnitude(Vec<Vec<f64>>),
    /// Add vectors element-wise (batch)
    Add(Vec<(Vec<f64>, Vec<f64>)>),
    /// Scalar multiplication (batch)
    ScalarMul(Vec<(f64, Vec<f64>)>),
    /// Project vectors onto other vectors (batch)
    Project(Vec<(Vec<f64>, Vec<f64>)>),
}

/// Result of a batch vector operation
#[derive(Clone, Debug)]
pub enum BatchVectorResult {
    /// Vector results
    Vectors(Vec<Vec<f64>>),
    /// Scalar results (dot products, magnitudes)
    Scalars(Vec<f64>),
    /// Error occurred during computation
    Error(String),
}

// ============================================================================
// BATCH TENSOR OPERATIONS
// ============================================================================

/// Batch tensor operations that can be GPU-accelerated.
#[derive(Clone, Debug, PartialEq)]
pub enum BatchTensorOp {
    /// Element-wise addition
    Add(Vec<(Vec<f64>, Vec<f64>)>),
    /// Element-wise multiplication (Hadamard product)
    Multiply(Vec<(Vec<f64>, Vec<f64>)>),
    /// Tensor contraction along specified axes
    Contract {
        tensors: Vec<(Vec<f64>, Vec<f64>)>,
        /// Shapes for each tensor pair
        shapes: Vec<(Vec<usize>, Vec<usize>)>,
        /// Contraction axes
        axes: (usize, usize),
    },
    /// Compute Frobenius norms
    FrobeniusNorm(Vec<Vec<f64>>),
    /// Reshape tensors (batch) - validation only, no GPU needed
    Reshape {
        data: Vec<Vec<f64>>,
        new_shapes: Vec<Vec<usize>>,
    },
}

/// Result of a batch tensor operation
#[derive(Clone, Debug)]
pub enum BatchTensorResult {
    /// Tensor results with shapes
    Tensors(Vec<(Vec<f64>, Vec<usize>)>),
    /// Scalar results (norms, etc.)
    Scalars(Vec<f64>),
    /// Error occurred during computation
    Error(String),
}

// ============================================================================
// BATCH OPERATION DISPATCHER
// ============================================================================

/// Unified batch operation type for dispatch
#[derive(Clone, Debug)]
pub enum BatchOperation {
    Matrix(BatchMatrixOp),
    Vector(BatchVectorOp),
    Tensor(BatchTensorOp),
}

/// Unified batch result type
#[derive(Clone, Debug)]
pub enum BatchResult {
    Matrix(BatchMatrixResult),
    Vector(BatchVectorResult),
    Tensor(BatchTensorResult),
}

/// Threshold for automatic GPU dispatch (number of operations)
pub const GPU_DISPATCH_THRESHOLD: usize = 100;

/// Check if batch size warrants GPU acceleration
#[allow(dead_code)]
pub fn should_use_gpu(batch_size: usize, preference: AccelerationPreference) -> bool {
    match preference {
        AccelerationPreference::CpuOnly => false,
        AccelerationPreference::PreferGpu => true,
        AccelerationPreference::PreferSimd => false,
        AccelerationPreference::Auto => batch_size >= GPU_DISPATCH_THRESHOLD,
    }
}
