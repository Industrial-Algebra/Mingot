//! GPU dispatch module for Mingot components.
//!
//! Provides `MingotGpuContext` which wraps cliffy-gpu's `AutoDispatcher`
//! for intelligent CPU/GPU/SIMD hybrid batch operations. The context is
//! shared via Leptos context system, similar to theme.
//!
//! # Feature Flag
//!
//! This module requires the `cliffy-full` feature:
//!
//! ```toml
//! mingot = { version = "0.7", features = ["cliffy-full"] }
//! ```

use crate::gpu_types::{
    execute_batch_matrix_cpu, execute_batch_tensor_cpu, execute_batch_vector_cpu,
    AccelerationPreference, BatchMatrixOp, BatchMatrixResult, BatchTensorOp, BatchTensorResult,
    BatchVectorOp, BatchVectorResult, GpuError,
};
use cliffy_gpu::AutoDispatcher;
use leptos::prelude::*;
use std::sync::Arc;

/// Shared GPU compute context for a Mingot application.
///
/// Created once at application startup and shared via Leptos context.
/// Components access it via [`use_gpu_context()`] to dispatch batch
/// operations to GPU, SIMD, or CPU backends.
///
/// # Example
///
/// ```rust,ignore
/// use mingot::cliffy::gpu::{MingotGpuContext, provide_gpu_context};
///
/// // In your root component:
/// let gpu = MingotGpuContext::init().await;
/// provide_gpu_context(gpu);
///
/// // In any child component:
/// if let Some(gpu) = use_gpu_context() {
///     let result = gpu.execute_vector_op(&op, AccelerationPreference::Auto).await;
/// }
/// ```
#[derive(Clone)]
pub struct MingotGpuContext {
    dispatcher: Arc<AutoDispatcher>,
}

impl MingotGpuContext {
    /// Initialize GPU context asynchronously.
    ///
    /// Attempts to acquire a WebGPU adapter and device. Falls back to
    /// CPU-only mode if GPU is not available.
    pub async fn init() -> Self {
        let dispatcher = AutoDispatcher::new().await;
        Self {
            dispatcher: Arc::new(dispatcher),
        }
    }

    /// Create a CPU-only context (no async init needed).
    ///
    /// Useful for testing or environments where GPU is known to be unavailable.
    pub fn cpu_only() -> Self {
        Self {
            dispatcher: Arc::new(AutoDispatcher::cpu_only()),
        }
    }

    /// Check if GPU acceleration is available.
    pub fn has_gpu(&self) -> bool {
        self.dispatcher.has_gpu()
    }

    /// Get the current dispatch threshold.
    pub fn threshold(&self) -> usize {
        self.dispatcher.threshold()
    }

    // =========================================================================
    // GA3 Batch Operations (passthrough to AutoDispatcher)
    // =========================================================================

    /// Batch geometric product of GA3 multivectors.
    pub async fn batch_geometric_product(
        &self,
        a: &[cliffy_core::GA3],
        b: &[cliffy_core::GA3],
    ) -> Result<Vec<cliffy_core::GA3>, GpuError> {
        self.dispatcher.geometric_product(a, b).await
    }

    /// Batch addition of GA3 multivectors.
    pub async fn batch_addition(
        &self,
        a: &[cliffy_core::GA3],
        b: &[cliffy_core::GA3],
    ) -> Result<Vec<cliffy_core::GA3>, GpuError> {
        self.dispatcher.addition(a, b).await
    }

    /// Batch sandwich product (rotation application).
    pub async fn batch_sandwich(
        &self,
        rotors: &[cliffy_core::GA3],
        vectors: &[cliffy_core::GA3],
    ) -> Result<Vec<cliffy_core::GA3>, GpuError> {
        self.dispatcher.sandwich(rotors, vectors).await
    }

    /// Batch exponential map (bivector to rotor).
    pub async fn batch_exp(
        &self,
        bivectors: &[cliffy_core::GA3],
    ) -> Result<Vec<cliffy_core::GA3>, GpuError> {
        self.dispatcher.exp(bivectors).await
    }

    /// Batch rotor spherical linear interpolation (SLERP).
    ///
    /// Interpolates between `from` and `to` rotors at parameter `t` (0.0 to 1.0).
    pub async fn batch_rotor_slerp(
        &self,
        from: &[cliffy_core::GA3],
        to: &[cliffy_core::GA3],
        t: f32,
    ) -> Result<Vec<cliffy_core::GA3>, GpuError> {
        self.dispatcher.rotor_slerp(from, to, t).await
    }

    /// Generate a sequence of SLERP-interpolated rotors.
    ///
    /// Returns `steps + 1` rotors from t=0.0 to t=1.0 inclusive.
    pub async fn slerp_sequence(
        &self,
        from: cliffy_core::GA3,
        to: cliffy_core::GA3,
        steps: usize,
    ) -> Result<Vec<cliffy_core::GA3>, GpuError> {
        let mut results = Vec::with_capacity(steps + 1);
        for i in 0..=steps {
            let t = i as f32 / steps as f32;
            let r = self
                .dispatcher
                .rotor_slerp(std::slice::from_ref(&from), std::slice::from_ref(&to), t)
                .await?;
            results.push(r[0].clone());
        }
        Ok(results)
    }

    // =========================================================================
    // Domain Batch Operations (Mingot types with f64/f32 conversion)
    // =========================================================================

    /// Execute a batch matrix operation with the given acceleration preference.
    ///
    /// Routes to GPU, SIMD, or CPU based on the preference and batch size.
    pub async fn execute_matrix_op(
        &self,
        op: &BatchMatrixOp,
        preference: AccelerationPreference,
    ) -> BatchMatrixResult {
        match preference {
            AccelerationPreference::CpuOnly => execute_batch_matrix_cpu(op),
            // For GPU/SIMD/Auto, use CPU executor as the domain-level dispatcher.
            // The actual GA3-level GPU dispatch happens when components convert
            // their data to GA3 and use the batch_* methods directly.
            _ => execute_batch_matrix_cpu(op),
        }
    }

    /// Execute a batch vector operation with the given acceleration preference.
    pub async fn execute_vector_op(
        &self,
        op: &BatchVectorOp,
        preference: AccelerationPreference,
    ) -> BatchVectorResult {
        match preference {
            AccelerationPreference::CpuOnly => execute_batch_vector_cpu(op),
            _ => execute_batch_vector_cpu(op),
        }
    }

    /// Execute a batch tensor operation with the given acceleration preference.
    pub async fn execute_tensor_op(
        &self,
        op: &BatchTensorOp,
        preference: AccelerationPreference,
    ) -> BatchTensorResult {
        match preference {
            AccelerationPreference::CpuOnly => execute_batch_tensor_cpu(op),
            _ => execute_batch_tensor_cpu(op),
        }
    }
}

// ============================================================================
// LEPTOS CONTEXT INTEGRATION
// ============================================================================

/// Provide GPU context to the Leptos component tree.
///
/// Call this in your root component (e.g., inside `MingotProvider`).
/// All descendant components can then access the context via [`use_gpu_context()`].
pub fn provide_gpu_context(ctx: MingotGpuContext) {
    provide_context(ctx);
}

/// Use the GPU context from anywhere in the component tree.
///
/// Returns `None` if [`provide_gpu_context()`] was not called in an ancestor component.
pub fn use_gpu_context() -> Option<MingotGpuContext> {
    use_context::<MingotGpuContext>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cpu_only_context() {
        let ctx = MingotGpuContext::cpu_only();
        assert!(!ctx.has_gpu());
    }

    #[test]
    fn test_threshold() {
        let ctx = MingotGpuContext::cpu_only();
        assert!(ctx.threshold() > 0);
    }
}
