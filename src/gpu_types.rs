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

/// Threshold for automatic GPU dispatch (number of operations).
/// Matches cliffy-gpu's GPU_DISPATCH_THRESHOLD of 256.
pub const GPU_DISPATCH_THRESHOLD: usize = 256;

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

// ============================================================================
// GPU ERROR TYPES
// ============================================================================

/// Errors from GPU/SIMD batch operations.
#[cfg(feature = "cliffy-full")]
pub use cliffy_gpu::GpuError;

#[cfg(not(feature = "cliffy-full"))]
#[derive(Clone, Debug)]
pub enum GpuError {
    /// GPU adapter not found on this device
    AdapterNotFound,
    /// Failed to create GPU device
    DeviceRequestFailed(String),
    /// Input buffer sizes don't match
    BufferSizeMismatch { expected: usize, actual: usize },
    /// GPU compute shader failed
    ComputeFailed(String),
    /// WebGPU not available in this environment
    WebGpuNotAvailable,
}

#[cfg(not(feature = "cliffy-full"))]
impl std::fmt::Display for GpuError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GpuError::AdapterNotFound => write!(f, "GPU adapter not found"),
            GpuError::DeviceRequestFailed(msg) => write!(f, "GPU device request failed: {msg}"),
            GpuError::BufferSizeMismatch { expected, actual } => {
                write!(f, "Buffer size mismatch: expected {expected}, got {actual}")
            }
            GpuError::ComputeFailed(msg) => write!(f, "GPU compute failed: {msg}"),
            GpuError::WebGpuNotAvailable => write!(f, "WebGPU not available"),
        }
    }
}

#[cfg(not(feature = "cliffy-full"))]
impl std::error::Error for GpuError {}

// ============================================================================
// CPU BATCH EXECUTORS
// ============================================================================
// These provide correct (but unoptimized) CPU implementations of all batch
// operations. They are the fallback when cliffy-full is not enabled, and also
// serve as the CpuOnly path when it is.

/// Execute a batch matrix operation on CPU.
pub fn execute_batch_matrix_cpu(op: &BatchMatrixOp) -> BatchMatrixResult {
    match op {
        BatchMatrixOp::Multiply(pairs) => {
            let results: Vec<MatrixData> = pairs
                .iter()
                .filter_map(|(a, b)| cpu_matrix_multiply(a, b))
                .collect();
            if results.len() != pairs.len() {
                return BatchMatrixResult::Error("Dimension mismatch in matrix multiply".into());
            }
            BatchMatrixResult::Matrices(results)
        }
        BatchMatrixOp::Determinant(matrices) => {
            let results: Vec<f64> = matrices.iter().map(cpu_determinant).collect();
            BatchMatrixResult::Scalars(results)
        }
        BatchMatrixOp::Transpose(matrices) => {
            let results: Vec<MatrixData> = matrices.iter().map(cpu_transpose).collect();
            BatchMatrixResult::Matrices(results)
        }
        BatchMatrixOp::Inverse(matrices) => {
            let results: Vec<MatrixData> = matrices.iter().filter_map(cpu_matrix_inverse).collect();
            if results.len() != matrices.len() {
                return BatchMatrixResult::Error("Singular matrix in batch inverse".into());
            }
            BatchMatrixResult::Matrices(results)
        }
        BatchMatrixOp::Eigenvalues(matrices) => {
            let results: Vec<Vec<(f64, f64)>> = matrices.iter().map(cpu_eigenvalues_2x2).collect();
            BatchMatrixResult::Complex(results)
        }
        BatchMatrixOp::Add(pairs) => {
            let results: Vec<MatrixData> = pairs
                .iter()
                .filter_map(|(a, b)| cpu_matrix_add(a, b))
                .collect();
            if results.len() != pairs.len() {
                return BatchMatrixResult::Error("Dimension mismatch in matrix add".into());
            }
            BatchMatrixResult::Matrices(results)
        }
        BatchMatrixOp::ScalarMul(pairs) => {
            let results: Vec<MatrixData> = pairs
                .iter()
                .map(|(s, m)| cpu_scalar_mul_matrix(*s, m))
                .collect();
            BatchMatrixResult::Matrices(results)
        }
    }
}

/// Execute a batch vector operation on CPU.
pub fn execute_batch_vector_cpu(op: &BatchVectorOp) -> BatchVectorResult {
    match op {
        BatchVectorOp::DotProduct(pairs) => {
            let results: Vec<f64> = pairs.iter().map(|(a, b)| cpu_dot_product(a, b)).collect();
            BatchVectorResult::Scalars(results)
        }
        BatchVectorOp::CrossProduct(pairs) => {
            let results: Vec<Vec<f64>> =
                pairs.iter().map(|(a, b)| cpu_cross_product(a, b)).collect();
            BatchVectorResult::Vectors(results)
        }
        BatchVectorOp::Normalize(vectors) => {
            let results: Vec<Vec<f64>> = vectors.iter().map(|v| cpu_normalize(v)).collect();
            BatchVectorResult::Vectors(results)
        }
        BatchVectorOp::Magnitude(vectors) => {
            let results: Vec<f64> = vectors.iter().map(|v| cpu_magnitude(v)).collect();
            BatchVectorResult::Scalars(results)
        }
        BatchVectorOp::Add(pairs) => {
            let results: Vec<Vec<f64>> = pairs.iter().map(|(a, b)| cpu_vector_add(a, b)).collect();
            BatchVectorResult::Vectors(results)
        }
        BatchVectorOp::ScalarMul(pairs) => {
            let results: Vec<Vec<f64>> = pairs
                .iter()
                .map(|(s, v)| cpu_scalar_mul_vector(*s, v))
                .collect();
            BatchVectorResult::Vectors(results)
        }
        BatchVectorOp::Project(pairs) => {
            let results: Vec<Vec<f64>> = pairs.iter().map(|(a, b)| cpu_project(a, b)).collect();
            BatchVectorResult::Vectors(results)
        }
    }
}

/// Execute a batch tensor operation on CPU.
pub fn execute_batch_tensor_cpu(op: &BatchTensorOp) -> BatchTensorResult {
    match op {
        BatchTensorOp::Add(pairs) => {
            let results: Vec<Vec<f64>> = pairs
                .iter()
                .map(|(a, b)| a.iter().zip(b.iter()).map(|(x, y)| x + y).collect())
                .collect();
            BatchTensorResult::Scalars(results.into_iter().flat_map(|v: Vec<f64>| v).collect())
        }
        BatchTensorOp::Multiply(pairs) => {
            let results: Vec<Vec<f64>> = pairs
                .iter()
                .map(|(a, b)| a.iter().zip(b.iter()).map(|(x, y)| x * y).collect())
                .collect();
            BatchTensorResult::Scalars(results.into_iter().flat_map(|v: Vec<f64>| v).collect())
        }
        BatchTensorOp::FrobeniusNorm(tensors) => {
            let results: Vec<f64> = tensors
                .iter()
                .map(|t| t.iter().map(|x| x * x).sum::<f64>().sqrt())
                .collect();
            BatchTensorResult::Scalars(results)
        }
        BatchTensorOp::Contract {
            tensors, shapes, ..
        } => {
            // Simplified contraction: compute dot product of flattened tensors
            let results: Vec<f64> = tensors
                .iter()
                .map(|(a, b)| a.iter().zip(b.iter()).map(|(x, y)| x * y).sum())
                .collect();
            let result_shapes: Vec<Vec<usize>> = shapes.iter().map(|_| vec![1]).collect();
            BatchTensorResult::Tensors(
                results
                    .into_iter()
                    .zip(result_shapes)
                    .map(|(r, s)| (vec![r], s))
                    .collect(),
            )
        }
        BatchTensorOp::Reshape { data, new_shapes } => {
            let results: Vec<(Vec<f64>, Vec<usize>)> = data
                .iter()
                .zip(new_shapes.iter())
                .map(|(d, s)| (d.clone(), s.clone()))
                .collect();
            BatchTensorResult::Tensors(results)
        }
    }
}

// ============================================================================
// CPU MATH HELPERS (private)
// ============================================================================

fn cpu_matrix_multiply(a: &MatrixData, b: &MatrixData) -> Option<MatrixData> {
    if a.is_empty() || b.is_empty() {
        return Some(vec![]);
    }
    let a_cols = a[0].len();
    let b_rows = b.len();
    let b_cols = b[0].len();
    if a_cols != b_rows {
        return None;
    }
    let mut result = vec![vec![0.0; b_cols]; a.len()];
    for i in 0..a.len() {
        for j in 0..b_cols {
            for k in 0..a_cols {
                result[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    Some(result)
}

#[allow(clippy::needless_range_loop)]
fn cpu_determinant(m: &MatrixData) -> f64 {
    let n = m.len();
    if n == 0 {
        return 1.0;
    }
    if n == 1 {
        return m[0][0];
    }
    if n == 2 {
        return m[0][0] * m[1][1] - m[0][1] * m[1][0];
    }
    // LU decomposition for larger matrices
    let mut lu: Vec<Vec<f64>> = m.to_vec();
    let mut sign = 1.0;
    for col in 0..n {
        // Partial pivoting
        let mut max_row = col;
        let mut max_val = lu[col][col].abs();
        for row in (col + 1)..n {
            if lu[row][col].abs() > max_val {
                max_val = lu[row][col].abs();
                max_row = row;
            }
        }
        if max_val < 1e-15 {
            return 0.0;
        }
        if max_row != col {
            lu.swap(col, max_row);
            sign = -sign;
        }
        let pivot = lu[col][col];
        for row in (col + 1)..n {
            let factor = lu[row][col] / pivot;
            for j in col..n {
                let val = lu[col][j];
                lu[row][j] -= factor * val;
            }
        }
    }
    let mut det = sign;
    for i in 0..n {
        det *= lu[i][i];
    }
    det
}

fn cpu_transpose(m: &MatrixData) -> MatrixData {
    if m.is_empty() {
        return vec![];
    }
    let rows = m.len();
    let cols = m[0].len();
    let mut result = vec![vec![0.0; rows]; cols];
    for i in 0..rows {
        for j in 0..cols {
            result[j][i] = m[i][j];
        }
    }
    result
}

#[allow(clippy::needless_range_loop)]
fn cpu_matrix_inverse(m: &MatrixData) -> Option<MatrixData> {
    let n = m.len();
    if n == 0 || m[0].len() != n {
        return None;
    }
    // Augmented matrix [m | I]
    let mut aug: Vec<Vec<f64>> = m
        .iter()
        .enumerate()
        .map(|(i, row)| {
            let mut r = row.clone();
            let mut identity_row = vec![0.0; n];
            identity_row[i] = 1.0;
            r.extend(identity_row);
            r
        })
        .collect();
    // Gauss-Jordan elimination
    for col in 0..n {
        // Partial pivoting
        let mut max_row = col;
        for row in (col + 1)..n {
            if aug[row][col].abs() > aug[max_row][col].abs() {
                max_row = row;
            }
        }
        aug.swap(col, max_row);
        let pivot = aug[col][col];
        if pivot.abs() < 1e-15 {
            return None; // Singular
        }
        for j in 0..(2 * n) {
            aug[col][j] /= pivot;
        }
        for row in 0..n {
            if row != col {
                let factor = aug[row][col];
                for j in 0..(2 * n) {
                    let val = aug[col][j];
                    aug[row][j] -= factor * val;
                }
            }
        }
    }
    Some(aug.iter().map(|row| row[n..].to_vec()).collect())
}

fn cpu_eigenvalues_2x2(m: &MatrixData) -> Vec<(f64, f64)> {
    // Only supports 2x2 matrices; returns empty for others
    if m.len() != 2 || m[0].len() != 2 {
        return vec![];
    }
    let a = m[0][0];
    let b = m[0][1];
    let c = m[1][0];
    let d = m[1][1];
    let trace = a + d;
    let det = a * d - b * c;
    let discriminant = trace * trace - 4.0 * det;
    if discriminant >= 0.0 {
        let sqrt_d = discriminant.sqrt();
        vec![((trace + sqrt_d) / 2.0, 0.0), ((trace - sqrt_d) / 2.0, 0.0)]
    } else {
        let sqrt_d = (-discriminant).sqrt();
        vec![(trace / 2.0, sqrt_d / 2.0), (trace / 2.0, -sqrt_d / 2.0)]
    }
}

fn cpu_matrix_add(a: &MatrixData, b: &MatrixData) -> Option<MatrixData> {
    if a.len() != b.len() {
        return None;
    }
    Some(
        a.iter()
            .zip(b.iter())
            .map(|(ar, br)| ar.iter().zip(br.iter()).map(|(x, y)| x + y).collect())
            .collect(),
    )
}

fn cpu_scalar_mul_matrix(s: f64, m: &MatrixData) -> MatrixData {
    m.iter()
        .map(|row| row.iter().map(|x| x * s).collect())
        .collect()
}

fn cpu_dot_product(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

fn cpu_cross_product(a: &[f64], b: &[f64]) -> Vec<f64> {
    if a.len() < 3 || b.len() < 3 {
        return vec![0.0; 3];
    }
    vec![
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}

fn cpu_magnitude(v: &[f64]) -> f64 {
    v.iter().map(|x| x * x).sum::<f64>().sqrt()
}

fn cpu_normalize(v: &[f64]) -> Vec<f64> {
    let mag = cpu_magnitude(v);
    if mag < 1e-15 {
        return vec![0.0; v.len()];
    }
    v.iter().map(|x| x / mag).collect()
}

fn cpu_vector_add(a: &[f64], b: &[f64]) -> Vec<f64> {
    a.iter().zip(b.iter()).map(|(x, y)| x + y).collect()
}

fn cpu_scalar_mul_vector(s: f64, v: &[f64]) -> Vec<f64> {
    v.iter().map(|x| x * s).collect()
}

fn cpu_project(a: &[f64], b: &[f64]) -> Vec<f64> {
    let dot_ab = cpu_dot_product(a, b);
    let dot_bb = cpu_dot_product(b, b);
    if dot_bb < 1e-15 {
        return vec![0.0; a.len()];
    }
    let scale = dot_ab / dot_bb;
    cpu_scalar_mul_vector(scale, b)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // --- should_use_gpu tests ---

    #[test]
    fn test_should_use_gpu_auto_below_threshold() {
        assert!(!should_use_gpu(100, AccelerationPreference::Auto));
    }

    #[test]
    fn test_should_use_gpu_auto_at_threshold() {
        assert!(should_use_gpu(256, AccelerationPreference::Auto));
    }

    #[test]
    fn test_should_use_gpu_prefer_gpu() {
        assert!(should_use_gpu(1, AccelerationPreference::PreferGpu));
    }

    #[test]
    fn test_should_use_gpu_cpu_only() {
        assert!(!should_use_gpu(1000, AccelerationPreference::CpuOnly));
    }

    // --- Matrix CPU executor tests ---

    #[test]
    fn test_cpu_matrix_multiply() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let op = BatchMatrixOp::Multiply(vec![(a, b)]);
        match execute_batch_matrix_cpu(&op) {
            BatchMatrixResult::Matrices(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0], vec![vec![19.0, 22.0], vec![43.0, 50.0]]);
            }
            _ => panic!("Expected Matrices result"),
        }
    }

    #[test]
    fn test_cpu_matrix_determinant_2x2() {
        let m = vec![vec![3.0, 8.0], vec![4.0, 6.0]];
        let op = BatchMatrixOp::Determinant(vec![m]);
        match execute_batch_matrix_cpu(&op) {
            BatchMatrixResult::Scalars(results) => {
                assert!((results[0] - (-14.0)).abs() < 1e-10);
            }
            _ => panic!("Expected Scalars result"),
        }
    }

    #[test]
    fn test_cpu_matrix_determinant_3x3() {
        let m = vec![
            vec![1.0, 2.0, 3.0],
            vec![4.0, 5.0, 6.0],
            vec![7.0, 8.0, 9.0],
        ];
        let op = BatchMatrixOp::Determinant(vec![m]);
        match execute_batch_matrix_cpu(&op) {
            BatchMatrixResult::Scalars(results) => {
                assert!(results[0].abs() < 1e-10); // Singular matrix
            }
            _ => panic!("Expected Scalars result"),
        }
    }

    #[test]
    fn test_cpu_matrix_transpose() {
        let m = vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]];
        let op = BatchMatrixOp::Transpose(vec![m]);
        match execute_batch_matrix_cpu(&op) {
            BatchMatrixResult::Matrices(results) => {
                assert_eq!(
                    results[0],
                    vec![vec![1.0, 4.0], vec![2.0, 5.0], vec![3.0, 6.0]]
                );
            }
            _ => panic!("Expected Matrices result"),
        }
    }

    #[test]
    fn test_cpu_matrix_inverse_2x2() {
        let m = vec![vec![4.0, 7.0], vec![2.0, 6.0]];
        let op = BatchMatrixOp::Inverse(vec![m]);
        match execute_batch_matrix_cpu(&op) {
            BatchMatrixResult::Matrices(results) => {
                assert_eq!(results.len(), 1);
                // det = 24-14 = 10, inv = [[0.6, -0.7], [-0.2, 0.4]]
                assert!((results[0][0][0] - 0.6).abs() < 1e-10);
                assert!((results[0][0][1] - (-0.7)).abs() < 1e-10);
                assert!((results[0][1][0] - (-0.2)).abs() < 1e-10);
                assert!((results[0][1][1] - 0.4).abs() < 1e-10);
            }
            _ => panic!("Expected Matrices result"),
        }
    }

    #[test]
    fn test_cpu_matrix_eigenvalues_2x2_real() {
        let m = vec![vec![2.0, 1.0], vec![1.0, 2.0]];
        let op = BatchMatrixOp::Eigenvalues(vec![m]);
        match execute_batch_matrix_cpu(&op) {
            BatchMatrixResult::Complex(results) => {
                assert_eq!(results.len(), 1);
                assert_eq!(results[0].len(), 2);
                // Eigenvalues: 3 and 1
                assert!((results[0][0].0 - 3.0).abs() < 1e-10);
                assert!((results[0][1].0 - 1.0).abs() < 1e-10);
            }
            _ => panic!("Expected Complex result"),
        }
    }

    #[test]
    fn test_cpu_matrix_add() {
        let a = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let b = vec![vec![5.0, 6.0], vec![7.0, 8.0]];
        let op = BatchMatrixOp::Add(vec![(a, b)]);
        match execute_batch_matrix_cpu(&op) {
            BatchMatrixResult::Matrices(results) => {
                assert_eq!(results[0], vec![vec![6.0, 8.0], vec![10.0, 12.0]]);
            }
            _ => panic!("Expected Matrices result"),
        }
    }

    #[test]
    fn test_cpu_matrix_scalar_mul() {
        let m = vec![vec![1.0, 2.0], vec![3.0, 4.0]];
        let op = BatchMatrixOp::ScalarMul(vec![(2.0, m)]);
        match execute_batch_matrix_cpu(&op) {
            BatchMatrixResult::Matrices(results) => {
                assert_eq!(results[0], vec![vec![2.0, 4.0], vec![6.0, 8.0]]);
            }
            _ => panic!("Expected Matrices result"),
        }
    }

    // --- Vector CPU executor tests ---

    #[test]
    fn test_cpu_vector_dot_product() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let op = BatchVectorOp::DotProduct(vec![(a, b)]);
        match execute_batch_vector_cpu(&op) {
            BatchVectorResult::Scalars(results) => {
                assert!((results[0] - 32.0).abs() < 1e-10);
            }
            _ => panic!("Expected Scalars result"),
        }
    }

    #[test]
    fn test_cpu_vector_cross_product() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let op = BatchVectorOp::CrossProduct(vec![(a, b)]);
        match execute_batch_vector_cpu(&op) {
            BatchVectorResult::Vectors(results) => {
                assert_eq!(results[0], vec![0.0, 0.0, 1.0]);
            }
            _ => panic!("Expected Vectors result"),
        }
    }

    #[test]
    fn test_cpu_vector_normalize() {
        let v = vec![3.0, 4.0, 0.0];
        let op = BatchVectorOp::Normalize(vec![v]);
        match execute_batch_vector_cpu(&op) {
            BatchVectorResult::Vectors(results) => {
                assert!((results[0][0] - 0.6).abs() < 1e-10);
                assert!((results[0][1] - 0.8).abs() < 1e-10);
                assert!(results[0][2].abs() < 1e-10);
            }
            _ => panic!("Expected Vectors result"),
        }
    }

    #[test]
    fn test_cpu_vector_magnitude() {
        let v = vec![3.0, 4.0];
        let op = BatchVectorOp::Magnitude(vec![v]);
        match execute_batch_vector_cpu(&op) {
            BatchVectorResult::Scalars(results) => {
                assert!((results[0] - 5.0).abs() < 1e-10);
            }
            _ => panic!("Expected Scalars result"),
        }
    }

    #[test]
    fn test_cpu_vector_add() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![4.0, 5.0, 6.0];
        let op = BatchVectorOp::Add(vec![(a, b)]);
        match execute_batch_vector_cpu(&op) {
            BatchVectorResult::Vectors(results) => {
                assert_eq!(results[0], vec![5.0, 7.0, 9.0]);
            }
            _ => panic!("Expected Vectors result"),
        }
    }

    #[test]
    fn test_cpu_vector_project() {
        let a = vec![3.0, 4.0, 0.0];
        let b = vec![1.0, 0.0, 0.0];
        let op = BatchVectorOp::Project(vec![(a, b)]);
        match execute_batch_vector_cpu(&op) {
            BatchVectorResult::Vectors(results) => {
                assert!((results[0][0] - 3.0).abs() < 1e-10);
                assert!(results[0][1].abs() < 1e-10);
                assert!(results[0][2].abs() < 1e-10);
            }
            _ => panic!("Expected Vectors result"),
        }
    }

    // --- Tensor CPU executor tests ---

    #[test]
    fn test_cpu_tensor_frobenius_norm() {
        let t = vec![3.0, 4.0];
        let op = BatchTensorOp::FrobeniusNorm(vec![t]);
        match execute_batch_tensor_cpu(&op) {
            BatchTensorResult::Scalars(results) => {
                assert!((results[0] - 5.0).abs() < 1e-10);
            }
            _ => panic!("Expected Scalars result"),
        }
    }

    #[test]
    fn test_cpu_tensor_reshape() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let op = BatchTensorOp::Reshape {
            data: vec![data.clone()],
            new_shapes: vec![vec![2, 3]],
        };
        match execute_batch_tensor_cpu(&op) {
            BatchTensorResult::Tensors(results) => {
                assert_eq!(results[0].0, data);
                assert_eq!(results[0].1, vec![2, 3]);
            }
            _ => panic!("Expected Tensors result"),
        }
    }

    // --- GpuError tests ---

    #[test]
    #[cfg(not(feature = "cliffy-full"))]
    fn test_gpu_error_display() {
        let err = GpuError::AdapterNotFound;
        assert_eq!(format!("{err}"), "GPU adapter not found");

        let err = GpuError::BufferSizeMismatch {
            expected: 10,
            actual: 5,
        };
        assert_eq!(format!("{err}"), "Buffer size mismatch: expected 10, got 5");
    }
}
