// Copyright (C) 2026 Industrial Algebra
// SPDX-License-Identifier: AGPL-3.0-only

//! Precision model and connection checking for node-graph ports.
//!
//! A port declares a [`PortType`] describing what values it emits or accepts.
//! [`check_connection`] decides whether a value flowing from a source port can
//! be represented by a target port, returning a [`ConnectionVerdict`]:
//!
//! - [`ConnectionVerdict::Ok`] — the target can represent every value the source emits.
//! - [`ConnectionVerdict::Lossy`] — the target can represent some but not all source
//!   values (precision may be lost). Allowed, but should be surfaced as a warning.
//! - [`ConnectionVerdict::Incompatible`] — the target's value category cannot hold
//!   the source's values at all (a type error, not precision loss).

use std::borrow::Cow;

/// Numeric integer width and signedness carried by an [`PortType::Integer`] port.
#[derive(Clone, Copy, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum IntKind {
    /// Unsigned 64-bit.
    U64,
    /// Unsigned 128-bit.
    U128,
    /// Signed 64-bit.
    I64,
    /// Signed 128-bit.
    I128,
}

/// What a port emits (output) or accepts (input).
#[derive(Clone, Debug, Eq, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum PortType {
    /// A fixed-width integer of a given [`IntKind`].
    Integer(IntKind),
    /// A decimal with a fixed number of fractional digits.
    Decimal(u32),
    /// Arbitrary-precision decimal via `rust_decimal` (28-29 significant digits).
    #[cfg(feature = "high-precision")]
    Arbitrary,
    /// Free-form text.
    Text,
    /// Boolean.
    Bool,
}

/// Describes the precision loss when a connection is representable but lossy.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrecisionLoss {
    pub detail: Cow<'static, str>,
}

/// Outcome of checking whether a source port's values can flow into a target port.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ConnectionVerdict {
    /// Target can exactly represent every source value.
    Ok,
    /// Target can represent some, but not all, source values.
    Lossy(PrecisionLoss),
    /// Target's value category cannot hold the source's values at all.
    Incompatible(Cow<'static, str>),
}

/// Check whether values flowing out of `source` can be represented by `target`.
pub fn check_connection(source: &PortType, target: &PortType) -> ConnectionVerdict {
    match (source, target) {
        (PortType::Integer(src), PortType::Integer(dst)) => check_integer(*src, *dst),
        (PortType::Decimal(src_places), PortType::Decimal(dst_places)) => {
            check_decimal_places(*src_places, *dst_places)
        }
        // Identical non-numeric categories are exact.
        (PortType::Text, PortType::Text) | (PortType::Bool, PortType::Bool) => {
            ConnectionVerdict::Ok
        }
        // Arbitrary-precision decimal (rust_decimal) is the widest decimal:
        // any fixed-precision decimal fits into it exactly, but an arbitrary
        // value (up to 28 fractional places) loses precision flowing into a
        // narrower fixed-precision target. Integer <-> Arbitrary is a category
        // change and falls through to Incompatible.
        #[cfg(feature = "high-precision")]
        (PortType::Arbitrary, PortType::Arbitrary) => ConnectionVerdict::Ok,
        #[cfg(feature = "high-precision")]
        (PortType::Decimal(_), PortType::Arbitrary) => ConnectionVerdict::Ok,
        #[cfg(feature = "high-precision")]
        (PortType::Arbitrary, PortType::Decimal(dst_places)) => {
            ConnectionVerdict::Lossy(PrecisionLoss {
                detail: Cow::Owned(format!(
                    "arbitrary exceeds target decimal({dst_places}) precision"
                )),
            })
        }
        // Within-category only: mixing numeric categories (integer <-> decimal)
        // or crossing into non-numeric categories (text/bool) is a hard type error.
        _ => ConnectionVerdict::Incompatible(Cow::Borrowed(
            "source and target ports have incompatible value categories",
        )),
    }
}

/// Decide decimal-flow verdict by fractional-digit count: a target with at
/// least as many fractional digits as the source holds every source value
/// exactly; fewer loses the trailing digits.
fn check_decimal_places(src_places: u32, dst_places: u32) -> ConnectionVerdict {
    if src_places <= dst_places {
        ConnectionVerdict::Ok
    } else {
        ConnectionVerdict::Lossy(PrecisionLoss {
            detail: Cow::Owned(format!(
                "decimal({src_places}) exceeds target decimal({dst_places}) precision"
            )),
        })
    }
}

/// Decide whether an integer `source` range is exactly representable by `target`.
///
/// The verdict is range containment: `Ok` iff every value the source can emit
/// lies within the target's representable range; `Lossy` if some values fall
/// outside (truncation/overflow risk); never `Incompatible` (same value category).
fn check_integer(source: IntKind, target: IntKind) -> ConnectionVerdict {
    use IntKind::*;
    let lossy = |why: &'static str| {
        ConnectionVerdict::Lossy(PrecisionLoss {
            detail: Cow::Borrowed(why),
        })
    };
    match (source, target) {
        // Same kind: exact.
        (U64, U64) | (U128, U128) | (I64, I64) | (I128, I128) => ConnectionVerdict::Ok,
        // Narrower unsigned -> wider unsigned: exact.
        (U64, U128) => ConnectionVerdict::Ok,
        // Wider unsigned -> narrower unsigned: lossy.
        (U128, U64) => lossy("u128 exceeds u64 range"),
        // Unsigned -> signed: exact only if the signed range is wide enough.
        //   u64 max = 2^64-1 fits in i128 (max 2^127-1) but not i64 (max 2^63-1).
        (U64, I128) => ConnectionVerdict::Ok,
        (U64, I64) => lossy("u64 exceeds i64 positive range"),
        //   u128 max = 2^128-1 exceeds every signed range.
        (U128, I64) | (U128, I128) => lossy("u128 exceeds signed range"),
        // Signed -> unsigned: always lossy (negatives cannot be held).
        (I64, U64) | (I64, U128) | (I128, U64) | (I128, U128) => {
            lossy("signed source may be negative")
        }
        // Narrower signed -> wider signed: exact.
        (I64, I128) => ConnectionVerdict::Ok,
        // Wider signed -> narrower signed: lossy.
        (I128, I64) => lossy("i128 exceeds i64 range"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wider_unsigned_target_exactly_holds_narrower_source() {
        let verdict = check_connection(
            &PortType::Integer(IntKind::U64),
            &PortType::Integer(IntKind::U128),
        );
        assert_eq!(verdict, ConnectionVerdict::Ok);
    }

    #[test]
    fn narrower_unsigned_target_loses_precision_from_wider_source() {
        let verdict = check_connection(
            &PortType::Integer(IntKind::U128),
            &PortType::Integer(IntKind::U64),
        );
        assert!(matches!(verdict, ConnectionVerdict::Lossy(_)));
    }

    // --- Integer matrix verification (one cohesive design: range containment) ---
    // Every distinct outcome in the 4x4 integer grid, to confirm the
    // exhaustive match is internally consistent and correct.

    #[test]
    fn integer_matrix_unsigned_into_wider_signed_is_exact() {
        // u64 (max 2^64-1) fits in i128 (max 2^127-1).
        assert_eq!(
            check_connection(
                &PortType::Integer(IntKind::U64),
                &PortType::Integer(IntKind::I128)
            ),
            ConnectionVerdict::Ok
        );
    }

    #[test]
    fn integer_matrix_unsigned_into_narrow_signed_is_lossy() {
        // u64 max exceeds i64 positive max (2^63-1).
        let verdict = check_connection(
            &PortType::Integer(IntKind::U64),
            &PortType::Integer(IntKind::I64),
        );
        assert!(matches!(verdict, ConnectionVerdict::Lossy(_)));
    }

    #[test]
    fn integer_matrix_signed_into_unsigned_is_always_lossy() {
        // Negatives cannot be held by an unsigned target, regardless of width.
        for src in [IntKind::I64, IntKind::I128] {
            for dst in [IntKind::U64, IntKind::U128] {
                let verdict = check_connection(&PortType::Integer(src), &PortType::Integer(dst));
                assert!(
                    matches!(verdict, ConnectionVerdict::Lossy(_)),
                    "signed {src:?} -> unsigned {dst:?} should be Lossy, got {verdict:?}"
                );
            }
        }
    }

    #[test]
    fn integer_matrix_signed_into_wider_signed_is_exact() {
        assert_eq!(
            check_connection(
                &PortType::Integer(IntKind::I64),
                &PortType::Integer(IntKind::I128)
            ),
            ConnectionVerdict::Ok
        );
    }

    #[test]
    fn integer_matrix_signed_into_narrower_signed_is_lossy() {
        let verdict = check_connection(
            &PortType::Integer(IntKind::I128),
            &PortType::Integer(IntKind::I64),
        );
        assert!(matches!(verdict, ConnectionVerdict::Lossy(_)));
    }

    #[test]
    fn integer_matrix_same_kind_is_exact() {
        for k in [IntKind::U64, IntKind::U128, IntKind::I64, IntKind::I128] {
            assert_eq!(
                check_connection(&PortType::Integer(k), &PortType::Integer(k)),
                ConnectionVerdict::Ok,
                "{k:?} -> {k:?} should be exact"
            );
        }
    }

    #[test]
    fn integer_matrix_is_antisymmetric_for_widths() {
        // Where a width ordering exists (same signedness), exactly one direction
        // is Ok and the other is Lossy. This is the "ordered precision" property.
        let width_pairs = [(IntKind::U64, IntKind::U128), (IntKind::I64, IntKind::I128)];
        for (narrow, wide) in width_pairs {
            let forward = check_connection(&PortType::Integer(narrow), &PortType::Integer(wide));
            let backward = check_connection(&PortType::Integer(wide), &PortType::Integer(narrow));
            assert_eq!(forward, ConnectionVerdict::Ok);
            assert!(matches!(backward, ConnectionVerdict::Lossy(_)));
        }
    }

    // --- Decimal precision ordering: compared by number of fractional digits ---

    #[test]
    fn fewer_decimal_places_into_more_is_exact() {
        let verdict = check_connection(&PortType::Decimal(2), &PortType::Decimal(8));
        assert_eq!(verdict, ConnectionVerdict::Ok);
    }

    #[test]
    fn more_decimal_places_into_fewer_is_lossy() {
        let verdict = check_connection(&PortType::Decimal(8), &PortType::Decimal(2));
        assert!(matches!(verdict, ConnectionVerdict::Lossy(_)));
    }

    #[test]
    fn equal_decimal_places_is_exact() {
        assert_eq!(
            check_connection(&PortType::Decimal(4), &PortType::Decimal(4)),
            ConnectionVerdict::Ok
        );
    }

    // --- Same-category non-numeric: exact only with identical category ---

    #[test]
    fn text_into_text_is_exact() {
        assert_eq!(
            check_connection(&PortType::Text, &PortType::Text),
            ConnectionVerdict::Ok
        );
    }

    #[test]
    fn bool_into_bool_is_exact() {
        assert_eq!(
            check_connection(&PortType::Bool, &PortType::Bool),
            ConnectionVerdict::Ok
        );
    }

    // --- Cross-category: distinct value categories never coerce ---
    // Mingot's no-silent-loss philosophy: type changes require an explicit
    // conversion node, so they are hard rejections (Incompatible), not warnings.

    #[test]
    fn integer_into_decimal_is_incompatible() {
        assert!(matches!(
            check_connection(&PortType::Integer(IntKind::U64), &PortType::Decimal(2)),
            ConnectionVerdict::Incompatible(_)
        ));
    }

    #[test]
    fn decimal_into_integer_is_incompatible() {
        assert!(matches!(
            check_connection(&PortType::Decimal(2), &PortType::Integer(IntKind::U128)),
            ConnectionVerdict::Incompatible(_)
        ));
    }

    #[test]
    fn text_into_numeric_is_incompatible() {
        assert!(matches!(
            check_connection(&PortType::Text, &PortType::Integer(IntKind::U64)),
            ConnectionVerdict::Incompatible(_)
        ));
    }

    #[test]
    fn numeric_into_bool_is_incompatible() {
        assert!(matches!(
            check_connection(&PortType::Decimal(4), &PortType::Bool),
            ConnectionVerdict::Incompatible(_)
        ));
    }

    // --- Arbitrary-precision decimal (rust_decimal): the widest decimal ---
    #[cfg(feature = "high-precision")]
    mod arbitrary {
        use super::*;

        #[test]
        fn arbitrary_into_arbitrary_is_exact() {
            assert_eq!(
                check_connection(&PortType::Arbitrary, &PortType::Arbitrary),
                ConnectionVerdict::Ok
            );
        }

        #[test]
        fn fixed_decimal_into_arbitrary_is_exact() {
            // Arbitrary is the widest decimal; any fixed-precision decimal fits.
            assert_eq!(
                check_connection(&PortType::Decimal(2), &PortType::Arbitrary),
                ConnectionVerdict::Ok
            );
        }

        #[test]
        fn arbitrary_into_narrower_decimal_is_lossy() {
            // Arbitrary may carry up to 28 fractional places; a narrower
            // fixed-precision target loses them.
            assert!(matches!(
                check_connection(&PortType::Arbitrary, &PortType::Decimal(2)),
                ConnectionVerdict::Lossy(_)
            ));
        }

        #[test]
        fn arbitrary_into_integer_is_incompatible() {
            assert!(matches!(
                check_connection(&PortType::Arbitrary, &PortType::Integer(IntKind::I128)),
                ConnectionVerdict::Incompatible(_)
            ));
        }

        #[test]
        fn integer_into_arbitrary_is_incompatible() {
            assert!(matches!(
                check_connection(&PortType::Integer(IntKind::U64), &PortType::Arbitrary),
                ConnectionVerdict::Incompatible(_)
            ));
        }
    }
}
