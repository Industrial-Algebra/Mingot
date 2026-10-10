// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! Echo-back wiring for the component API convention
//! (read-signals in, intent-callbacks out; parent owns state).
//!
//! Components that accept an optional external `ReadSignal<T>` keep an
//! internal echo signal for their own rendering; [`echo_signal`] seeds the
//! internal signal from the external one (or a default) and keeps it synced
//! external → internal. User intents fire the component's `on_change`
//! callbacks; the external signal is never written by the component.

use leptos::prelude::*;

/// Create a component-owned echo signal for an optional external read-signal.
///
/// - Seeds from `external` (current value, untracked) when present, else
///   `default`.
/// - When `external` is `Some`, installs an effect that propagates external
///   changes into the returned signal, guarded so redundant writes (equal
///   values) are skipped.
///
/// The returned `RwSignal` is the component's internal state: render from it,
/// write it on user action, and fire the intent callback on the same path.
pub fn echo_signal<T>(external: Option<ReadSignal<T>>, default: T) -> RwSignal<T>
where
    T: Clone + PartialEq + Send + Sync + 'static,
{
    let internal = RwSignal::new(match &external {
        Some(ext) => ext.get_untracked(),
        None => default,
    });
    if let Some(ext) = external {
        Effect::new(move |_| {
            let v = ext.get();
            if v != internal.get_untracked() {
                internal.set(v);
            }
        });
    }
    internal
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Native tests need a global executor for effect spawning.
    fn init_executor() {
        use any_spawner::Executor;
        let _ = Executor::init_futures_executor();
    }

    /// Pump queued effect re-runs so assertions observe settled state.
    fn flush_effects() {
        use any_spawner::Executor;
        Executor::poll_local();
    }

    #[test]
    fn seeds_from_external_when_present() {
        init_executor();
        let (ext, _set) = signal(true);
        let internal = echo_signal(Some(ext), false);
        assert!(
            internal.get_untracked(),
            "must seed from external, not default"
        );
    }

    #[test]
    fn seeds_from_default_when_external_absent() {
        let internal = echo_signal(None, true);
        assert!(internal.get_untracked());
    }

    #[test]
    fn external_change_propagates_to_internal() {
        init_executor();
        let (ext, set_ext) = signal(1u32);
        let internal = echo_signal(Some(ext), 0);
        assert_eq!(internal.get_untracked(), 1);
        set_ext.set(5);
        flush_effects();
        assert_eq!(internal.get_untracked(), 5, "external must sync inward");
    }

    #[test]
    fn equal_external_write_is_skipped_not_looped() {
        init_executor();
        // Guard: setting external to its current value must not re-write
        // internal (observable via a write counter on a derived effect).
        let (ext, set_ext) = signal(7u32);
        let internal = echo_signal(Some(ext), 0);
        let (writes, set_writes) = signal(0u32);
        Effect::new(move |_| {
            let _ = internal.get();
            set_writes.update(|w| *w += 1);
        });
        flush_effects(); // settle the counter's initial run before measuring
        let before = writes.get_untracked();
        set_ext.set(7); // same value
        flush_effects();
        assert_eq!(writes.get_untracked(), before, "equal value must not write");
        set_ext.set(9); // different value
        flush_effects();
        assert!(
            writes.get_untracked() > before,
            "real change must propagate"
        );
        assert_eq!(internal.get_untracked(), 9);
    }

    #[test]
    fn no_external_no_sync_but_writable() {
        let internal = echo_signal(None, "start".to_string());
        assert_eq!(internal.get_untracked(), "start");
        internal.set("edited".to_string());
        assert_eq!(internal.get_untracked(), "edited");
    }
}
