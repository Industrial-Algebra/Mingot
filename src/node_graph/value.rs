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

use crate::node_graph::precision::PortType;
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

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Integer(a), Value::Integer(b)) => a == b,
            (Value::Decimal(a), Value::Decimal(b)) => a == b,
            #[cfg(feature = "high-precision")]
            (Value::Arbitrary(a), Value::Arbitrary(b)) => a == b,
            (Value::Text(a), Value::Text(b)) => a == b,
            (Value::Bool(a), Value::Bool(b)) => a == b,
            (Value::Custom(a), Value::Custom(b)) => std::ptr::eq(
                a.as_ref() as *const dyn CustomValue,
                b.as_ref() as *const dyn CustomValue,
            ),
            _ => false,
        }
    }
}

impl Value {
    /// Whether this value can sit on a port of the declared type *exactly*.
    ///
    /// Used by the engine to hold operations to their declared outputs:
    /// an integer must fit the kind's range, a decimal's scale must not
    /// exceed the port's fractional-digit count. Cross-category values
    /// never match (the transfer verdict governs edges, not this).
    pub fn matches(&self, ty: &PortType) -> bool {
        use crate::node_graph::precision::IntKind;
        match (self, ty) {
            (Value::Integer(v), PortType::Integer(kind)) => match kind {
                IntKind::U64 => (0..=i128::from(u64::MAX)).contains(v),
                IntKind::U128 => *v >= 0,
                IntKind::I64 => (i64::MIN as i128..=i64::MAX as i128).contains(v),
                IntKind::I128 => true,
            },
            (Value::Decimal(d), PortType::Decimal(n)) => d.scale() <= *n,
            #[cfg(feature = "high-precision")]
            (Value::Arbitrary(_), PortType::Arbitrary) => true,
            (Value::Text(_), PortType::Text) => true,
            (Value::Bool(_), PortType::Bool) => true,
            _ => false,
        }
    }

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
    fn value_equality_is_structural_except_custom_identity() {
        assert_eq!(Value::Integer(2), Value::Integer(2));
        assert_ne!(Value::Integer(2), Value::Bool(true));
        let a = Arc::new(SampleMedia { name: "x" });
        let b = a.clone();
        let c = Arc::new(SampleMedia { name: "x" });
        assert_eq!(Value::Custom(a), Value::Custom(b)); // same allocation
        assert_ne!(
            Value::Custom(c),
            Value::Custom(Arc::new(SampleMedia { name: "x" })) // distinct allocation
        );
    }

    #[test]
    fn text_holds_static_and_owned_cows() {
        let s = Value::Text(Cow::Borrowed("static"));
        let o = Value::Text(Cow::Owned("owned".to_string()));
        assert!(matches!(s, Value::Text(Cow::Borrowed(_))));
        assert!(matches!(o, Value::Text(Cow::Owned(_))));
    }

    #[test]
    fn matches_enforces_integer_range_per_kind() {
        use crate::node_graph::precision::IntKind;
        assert!(Value::Integer(42).matches(&PortType::Integer(IntKind::U64)));
        assert!(!Value::Integer(-1).matches(&PortType::Integer(IntKind::U64)));
        assert!(!Value::Integer(i128::from(u64::MAX) + 1).matches(&PortType::Integer(IntKind::U64)));
        assert!(Value::Integer(-1).matches(&PortType::Integer(IntKind::I64)));
        assert!(!Value::Integer(i64::MAX as i128 + 1).matches(&PortType::Integer(IntKind::I64)));
        assert!(Value::Integer(i64::MAX as i128 + 1).matches(&PortType::Integer(IntKind::I128)));
    }

    #[test]
    fn matches_enforces_decimal_scale_not_category_crossings() {
        let two = Decimal::from_str_exact("1.23").unwrap();
        assert!(Value::Decimal(two).matches(&PortType::Decimal(2)));
        assert!(!Value::Decimal(two).matches(&PortType::Decimal(1)));
        assert!(!Value::Decimal(two).matches(&PortType::Text));
        assert!(!Value::Bool(true).matches(&PortType::Text));
    }

    #[test]
    fn decimal_values_parse_exact() {
        // 0.1 + 0.2 is exactly 0.3 in rust_decimal — the library's pitch.
        let d = Decimal::from_str_exact("0.1").unwrap() + Decimal::from_str_exact("0.2").unwrap();
        assert_eq!(d, Decimal::from_str_exact("0.3").unwrap());
    }
}
