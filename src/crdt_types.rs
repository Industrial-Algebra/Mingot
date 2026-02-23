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

// ============================================================================
// NODE ID TYPES
// ============================================================================

/// Unique node identifier for distributed systems.
/// When cliffy feature is enabled, this will be replaced with a proper distributed ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CrdtNodeId {
    id: u128,
}

impl Default for CrdtNodeId {
    fn default() -> Self {
        Self::new()
    }
}

impl CrdtNodeId {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { id: 0 }
    }

    #[allow(dead_code)]
    pub fn from_u128(id: u128) -> Self {
        Self { id }
    }
}

// ============================================================================
// SYNC PAYLOAD TYPES
// ============================================================================

/// Sync payload for sending updates between nodes.
#[cfg(feature = "cliffy")]
pub type CrdtSyncPayload = cliffy_protocols::SyncPayload;

#[cfg(not(feature = "cliffy"))]
#[derive(Clone, Debug)]
pub struct CrdtSyncPayload {
    /// Source node ID
    pub node_id: CrdtNodeId,
    /// Operation data (serialized)
    pub data: Vec<u8>,
    /// Vector clock timestamp
    pub timestamp: u64,
}

#[cfg(not(feature = "cliffy"))]
impl Default for CrdtSyncPayload {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(feature = "cliffy"))]
impl CrdtSyncPayload {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            node_id: CrdtNodeId::new(),
            data: Vec::new(),
            timestamp: 0,
        }
    }
}

// ============================================================================
// COLLABORATIVE MAP TYPES
// ============================================================================

#[cfg(not(feature = "cliffy"))]
use std::collections::HashMap;

/// Collaborative map for key-value state with CRDT semantics.
/// Used by ParameterGrid and similar components for multi-user editing.
#[cfg(not(feature = "cliffy"))]
#[derive(Clone)]
pub struct CollaborativeMap {
    values: HashMap<String, String>,
    clock: CrdtVectorClock,
    node_id: CrdtNodeId,
}

#[cfg(not(feature = "cliffy"))]
impl Default for CollaborativeMap {
    fn default() -> Self {
        Self::new(CrdtNodeId::new())
    }
}

#[cfg(not(feature = "cliffy"))]
impl CollaborativeMap {
    #[allow(dead_code)]
    pub fn new(node_id: CrdtNodeId) -> Self {
        Self {
            values: HashMap::new(),
            clock: CrdtVectorClock::new(),
            node_id,
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, key: &str) -> Option<&String> {
        self.values.get(key)
    }

    #[allow(dead_code)]
    pub fn set(&mut self, key: String, value: String) -> Option<CrdtSyncPayload> {
        self.values.insert(key, value);
        self.clock.tick();
        // In stub mode, we don't generate real sync payloads
        None
    }

    #[allow(dead_code)]
    pub fn remove(&mut self, key: &str) -> Option<CrdtSyncPayload> {
        self.values.remove(key);
        self.clock.tick();
        None
    }

    #[allow(dead_code)]
    pub fn apply_remote(&mut self, _payload: &CrdtSyncPayload) {
        // In stub mode, remote operations are no-ops
    }

    #[allow(dead_code)]
    pub fn to_hashmap(&self) -> HashMap<String, String> {
        self.values.clone()
    }

    #[allow(dead_code)]
    pub fn node_id(&self) -> CrdtNodeId {
        self.node_id
    }
}
