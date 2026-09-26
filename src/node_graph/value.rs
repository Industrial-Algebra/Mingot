// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: Apache-2.0

//! Runtime values carried between node ports during execution (7C).
//!
//! Pure kernel: no Leptos, no wasm — natively testable.
//!
//! A `Value` is what flows along connections; its *precision expectations*
//! live in the port types of the [`NodeDefinition`](crate::node_graph::node::NodeDefinition),
//! not in the value itself. The engine checks values against declared port
//! types and **refuses** lossy or incompatible transfers (the refuse-to-fire
//! numerics contract); explicit conversion nodes are the only sanctioned way
//! to change precision.
//!
//! The [`Value::Custom`] variant is the open hatch for out-of-tree value
//! kinds (e.g. media values for the planned Studio Orinth video project):
//! runtime-only, shared via `Arc`, summarized for inspectors.

use rust_decimal::Decimal;
use std::borrow::Cow;
use std::sync::Arc;

/// Category of a runtime value, mirroring [`PortType`](crate::node_graph::precision::PortType)
/// categories for engine-side coherence checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueKind {
    Integer,
    Decimal,
    /// High-precision arbitrary decimal (`high-precision` feature).
    Arbitrary,
    Text,
    Bool,
    Custom,
}

/// An out-of-tree runtime value kind (open extension seam).
///
/// Runtime-only: custom values are shared by `Arc` and never reconstructed
/// from serialized data.
pub trait CustomValue: Send + Sync {
    /// Stable kind name, e.g. `"frame"`, `"timestamp"`.
    fn kind_name(&self) -> &'static str;
    /// Human-readable summary for inspectors and reports.
    fn summary(&self) -> String;
}

/// Shared, cheaply cloneable custom value.
pub type CustomValueBox = Arc<dyn CustomValue>;

/// A runtime value flowing between node ports.
#[derive(Clone)]
pub enum Value {
    Integer(i128),
    Decimal(Decimal),
    #[cfg(feature = "high-precision")]
    Arbitrary(Decimal),
    Text(Cow<'static, str>),
    Bool(bool),
    Custom(CustomValueBox),
}

impl std::fmt::Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Integer(i) => write!(f, "Integer({i})"),
            Value::Decimal(d) => write!(f, "Decimal({d})"),
            #[cfg(feature = "high-precision")]
            Value::Arbitrary(d) => write!(f, "Arbitrary({d})"),
            Value::Text(t) => write!(f, "Text({t:?})"),
            Value::Bool(b) => write!(f, "Bool({b})"),
            Value::Custom(c) => write!(f, "Custom({} :: {})", c.kind_name(), c.summary()),
        }
    }
}

impl Value {
    /// The value's category, for coherence checks against port types.
    pub fn kind(&self) -> ValueKind {
        match self {
            Value::Integer(_) => ValueKind::Integer,
            Value::Decimal(_) => ValueKind::Decimal,
            #[cfg(feature = "high-precision")]
            Value::Arbitrary(_) => ValueKind::Arbitrary,
            Value::Text(_) => ValueKind::Text,
            Value::Bool(_) => ValueKind::Bool,
            Value::Custom(_) => ValueKind::Custom,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn kinds_classify_each_variant() {
        assert_eq!(Value::Integer(7).kind(), ValueKind::Integer);
        assert_eq!(
            Value::Decimal(Decimal::new(1234, 2)).kind(),
            ValueKind::Decimal
        );
        assert_eq!(Value::Text("hi".into()).kind(), ValueKind::Text);
        assert_eq!(Value::Bool(true).kind(), ValueKind::Bool);
    }

    #[cfg(feature = "high-precision")]
    #[test]
    fn arbitrary_is_its_own_kind() {
        assert_eq!(
            Value::Arbitrary(Decimal::new(1, 0)).kind(),
            ValueKind::Arbitrary
        );
    }

    /// A trivial custom value for tests: a named sample.
    struct SampleMedia {
        name: &'static str,
    }
    impl CustomValue for SampleMedia {
        fn kind_name(&self) -> &'static str {
            "sample"
        }
        fn summary(&self) -> String {
            format!("sample:{}", self.name)
        }
    }

    #[test]
    fn custom_values_are_shared_arc_boxes() {
        let a = Value::Custom(Arc::new(SampleMedia { name: "frame-1" }));
        let b = a.clone();
        assert_eq!(a.kind(), ValueKind::Custom);
        // Arc sharing: clones are the same allocation.
        match (&a, &b) {
            (Value::Custom(x), Value::Custom(y)) => {
                assert!(std::ptr::eq(
                    x.as_ref() as *const dyn CustomValue,
                    y.as_ref() as *const dyn CustomValue,
                ));
            }
            _ => panic!("expected custom values"),
        }
    }

    #[test]
    fn text_holds_static_and_owned_cows() {
        let s = Value::Text(Cow::Borrowed("static"));
        let o = Value::Text(Cow::Owned("owned".to_string()));
        assert!(matches!(s, Value::Text(Cow::Borrowed(_))));
        assert!(matches!(o, Value::Text(Cow::Owned(_))));
    }

    #[test]
    fn decimal_values_parse_exact() {
        // 0.1 + 0.2 is exactly 0.3 in rust_decimal — the library's pitch.
        let d = Decimal::from_str_exact("0.1").unwrap() + Decimal::from_str_exact("0.2").unwrap();
        assert_eq!(d, Decimal::from_str_exact("0.3").unwrap());
    }
}
