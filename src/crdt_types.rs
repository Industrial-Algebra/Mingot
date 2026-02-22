//! Shared type aliases for cliffy-protocols CRDT integration
//!
//! These types provide compile-time feature gating for CRDT props.
//! When the `cliffy` feature is disabled, stub types are used that
//! have no effect at runtime.
//!
//! cliffy-protocols provides geometric algebra-based CRDTs:
//! - `GeometricCRDT<T>`: Operation-based CRDT with geometric transforms
//! - `VectorClock`: Causal ordering for distributed operations
//! - `SyncState`: P2P synchronization state

// ============================================================================
// GEOMETRIC CRDT TYPES
// ============================================================================

/// Geometric CRDT type using GA3 multivector state.
/// Used for collaborative numeric parameters via scalar component.
#[cfg(feature = "cliffy")]
pub type GeometricCrdtGA3 = cliffy_protocols::GeometricCRDT;

#[cfg(not(feature = "cliffy"))]
#[derive(Clone)]
pub struct GeometricCrdtGA3 {
    value: f64,
}

#[cfg(not(feature = "cliffy"))]
impl Default for GeometricCrdtGA3 {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(feature = "cliffy"))]
impl GeometricCrdtGA3 {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { value: 0.0 }
    }

    #[allow(dead_code)]
    pub fn value(&self) -> f64 {
        self.value
    }

    #[allow(dead_code)]
    pub fn set(&mut self, value: f64) {
        self.value = value;
    }
}

// ============================================================================
// VECTOR CLOCK TYPES
// ============================================================================

/// Vector clock for causal ordering in distributed systems.
#[cfg(feature = "cliffy")]
pub type CrdtVectorClock = cliffy_protocols::VectorClock;

#[cfg(not(feature = "cliffy"))]
#[derive(Clone, Default)]
pub struct CrdtVectorClock {
    _timestamp: u64,
}

#[cfg(not(feature = "cliffy"))]
impl CrdtVectorClock {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { _timestamp: 0 }
    }

    #[allow(dead_code)]
    pub fn tick(&mut self) {
        self._timestamp += 1;
    }

    #[allow(dead_code)]
    pub fn timestamp(&self) -> u64 {
        self._timestamp
    }
}

// ============================================================================
// SYNC STATE TYPES
// ============================================================================

/// Synchronization state for collaborative editing.
#[cfg(feature = "cliffy")]
pub type CrdtSyncState = cliffy_protocols::SyncState;

#[cfg(not(feature = "cliffy"))]
#[derive(Clone)]
pub struct CrdtSyncState {
    _connected: bool,
}

#[cfg(not(feature = "cliffy"))]
impl Default for CrdtSyncState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(feature = "cliffy"))]
impl CrdtSyncState {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { _connected: false }
    }

    #[allow(dead_code)]
    pub fn is_syncing(&self) -> bool {
        false
    }
}

// ============================================================================
// COLLABORATIVE STATE WRAPPER
// ============================================================================

/// Wrapper for collaborative state that can be synced across multiple users.
/// This provides a simplified interface for component integration.
#[cfg(not(feature = "cliffy"))]
#[derive(Clone)]
pub struct CollaborativeState<T: Clone> {
    local_value: T,
    _clock: CrdtVectorClock,
}

#[cfg(not(feature = "cliffy"))]
impl<T: Clone + Default> Default for CollaborativeState<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

#[cfg(not(feature = "cliffy"))]
impl<T: Clone> CollaborativeState<T> {
    #[allow(dead_code)]
    pub fn new(initial: T) -> Self {
        Self {
            local_value: initial,
            _clock: CrdtVectorClock::new(),
        }
    }

    #[allow(dead_code)]
    pub fn get(&self) -> &T {
        &self.local_value
    }

    #[allow(dead_code)]
    pub fn set(&mut self, value: T) {
        self.local_value = value;
        self._clock.tick();
    }

    #[allow(dead_code)]
    pub fn into_inner(self) -> T {
        self.local_value
    }
}
