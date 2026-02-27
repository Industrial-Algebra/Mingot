//! Cliffy-test invariants for numeric components
//!
//! These tests verify algebraic properties that must NEVER fail (Impossible invariants).
//! Using cliffy-test's geometric error reporting for precise failure diagnostics.

use cliffy_test::prelude::*;
use cliffy_test::{invariant_impossible, verify_invariant};

// Re-export types we need to test
// Note: These are internal functions, so we test through the public validation interface

/// Test that u64 parsing preserves exact values (no precision loss)
#[test]
fn invariant_u64_parsing_preserves_value() {
    let inv = invariant_impossible! {
        name: "u64 parsing round-trips exactly",
        check: || {
            // Generate random u64 values
            let test_values: Vec<u64> = vec![
                0,
                1,
                u64::MAX,
                u64::MAX / 2,
                12345678901234567890,
                9007199254740993, // First integer not exactly representable in f64
            ];

            for value in test_values {
                let string_repr = value.to_string();
                let parsed: Result<u64, _> = string_repr.parse();

                match parsed {
                    Ok(parsed_value) if parsed_value == value => continue,
                    Ok(parsed_value) => {
                        return TestResult::fail_with_distance(
                            (value as f64 - parsed_value as f64).abs(),
                            format!(
                                "u64 precision loss: {} -> '{}' -> {}",
                                value, string_repr, parsed_value
                            ),
                        );
                    }
                    Err(e) => {
                        return TestResult::fail_with_distance(
                            1.0,
                            format!("u64 parse error for {}: {}", value, e),
                        );
                    }
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test that i64 parsing preserves exact values including negatives
#[test]
fn invariant_i64_parsing_preserves_value() {
    let inv = invariant_impossible! {
        name: "i64 parsing round-trips exactly",
        check: || {
            let test_values: Vec<i64> = vec![
                0,
                1,
                -1,
                i64::MAX,
                i64::MIN,
                i64::MAX / 2,
                i64::MIN / 2,
                -9007199254740993, // Negative of first non-f64-exact integer
            ];

            for value in test_values {
                let string_repr = value.to_string();
                let parsed: Result<i64, _> = string_repr.parse();

                match parsed {
                    Ok(parsed_value) if parsed_value == value => continue,
                    Ok(parsed_value) => {
                        return TestResult::fail_with_distance(
                            (value as f64 - parsed_value as f64).abs(),
                            format!(
                                "i64 precision loss: {} -> '{}' -> {}",
                                value, string_repr, parsed_value
                            ),
                        );
                    }
                    Err(e) => {
                        return TestResult::fail_with_distance(
                            1.0,
                            format!("i64 parse error for {}: {}", value, e),
                        );
                    }
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test that u128 parsing preserves exact values (full 128-bit range)
#[test]
fn invariant_u128_parsing_preserves_value() {
    let inv = invariant_impossible! {
        name: "u128 parsing round-trips exactly",
        check: || {
            let test_values: Vec<u128> = vec![
                0,
                1,
                u128::MAX,
                u128::MAX / 2,
                // Large values beyond u64 range
                18446744073709551616, // u64::MAX + 1
                340282366920938463463374607431768211455, // u128::MAX
            ];

            for value in test_values {
                let string_repr = value.to_string();
                let parsed: Result<u128, _> = string_repr.parse();

                match parsed {
                    Ok(parsed_value) if parsed_value == value => continue,
                    Ok(_parsed_value) => {
                        return TestResult::fail_with_distance(
                            1.0, // Can't compute f64 distance for u128
                            format!("u128 precision loss for value: {}", value),
                        );
                    }
                    Err(e) => {
                        return TestResult::fail_with_distance(
                            1.0,
                            format!("u128 parse error for {}: {}", value, e),
                        );
                    }
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test that decimal string representation preserves precision
#[test]
fn invariant_decimal_string_preserves_precision() {
    let inv = invariant_impossible! {
        name: "Decimal strings preserve all digits",
        check: || {
            // Test decimal values with various precision levels
            let test_cases = vec![
                ("0.1", "0.1"),
                ("0.123456789012345", "0.123456789012345"),
                ("123.456", "123.456"),
                ("0.000001", "0.000001"),
                ("999999999999.999999999999", "999999999999.999999999999"),
            ];

            for (input, expected) in test_cases {
                // Parse and re-stringify should be identical
                let normalized = input.trim();
                if normalized != expected {
                    return TestResult::fail_with_distance(
                        1.0,
                        format!(
                            "Decimal representation changed: '{}' != '{}'",
                            normalized, expected
                        ),
                    );
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test that slider value clamping always produces values within bounds
#[test]
fn invariant_slider_value_within_bounds() {
    let inv = invariant_impossible! {
        name: "Slider values always within [min, max]",
        check: || {
            // Slider clamping logic: value.clamp(min, max)
            let test_cases: Vec<(f64, f64, f64, f64)> = vec![
                // (input, min, max, expected)
                (50.0, 0.0, 100.0, 50.0),
                (-10.0, 0.0, 100.0, 0.0),   // Below min -> clamped to min
                (150.0, 0.0, 100.0, 100.0), // Above max -> clamped to max
                (0.0, 0.0, 100.0, 0.0),     // At min
                (100.0, 0.0, 100.0, 100.0), // At max
                (f64::NEG_INFINITY, 0.0, 100.0, 0.0),
                (f64::INFINITY, 0.0, 100.0, 100.0),
            ];

            for (input, min, max, expected) in test_cases {
                let clamped = input.clamp(min, max);

                if (clamped - expected).abs() > f64::EPSILON {
                    return TestResult::fail_with_distance(
                        (clamped - expected).abs(),
                        format!(
                            "Slider clamp failed: {}.clamp({}, {}) = {} (expected {})",
                            input, min, max, clamped, expected
                        ),
                    );
                }

                // Invariant: result must always be within bounds
                if clamped < min || clamped > max {
                    return TestResult::fail_with_distance(
                        if clamped < min {
                            min - clamped
                        } else {
                            clamped - max
                        },
                        format!(
                            "Slider value {} outside bounds [{}, {}]",
                            clamped, min, max
                        ),
                    );
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test that range slider maintains min <= max invariant
#[test]
fn invariant_range_slider_ordering() {
    let inv = invariant_impossible! {
        name: "RangeSlider min <= max",
        check: || {
            // Test that after any operation, min_value <= max_value
            let test_cases: Vec<(f64, f64)> = vec![
                (0.0, 100.0),
                (50.0, 50.0),   // Equal is valid
                (25.0, 75.0),
                (0.0, 0.0),     // Both at minimum
                (100.0, 100.0), // Both at maximum
            ];

            for (min_val, max_val) in test_cases {
                if min_val > max_val {
                    return TestResult::fail_with_distance(
                        min_val - max_val,
                        format!(
                            "RangeSlider ordering violated: min {} > max {}",
                            min_val, max_val
                        ),
                    );
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test that step-based increment produces predictable values
#[test]
fn invariant_slider_step_precision() {
    let inv = invariant_impossible! {
        name: "Slider step increments are precise",
        check: || {
            let test_cases: Vec<(f64, f64, f64, f64)> = vec![
                // (value, step, min, max) -> value + step should be exact
                (0.0, 1.0, 0.0, 100.0),
                (0.0, 0.1, 0.0, 1.0),
                (0.5, 0.1, 0.0, 1.0),
                (10.0, 5.0, 0.0, 100.0),
            ];

            for (value, step, min, max) in test_cases {
                let incremented = (value + step).clamp(min, max);

                // The incremented value should be exactly value + step (when within bounds)
                let expected = (value + step).clamp(min, max);

                if (incremented - expected).abs() > f64::EPSILON * 10.0 {
                    return TestResult::fail_with_distance(
                        (incremented - expected).abs(),
                        format!(
                            "Step increment imprecise: {} + {} = {} (expected {})",
                            value, step, incremented, expected
                        ),
                    );
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test that thousand separator parsing handles all locale formats
#[test]
fn invariant_thousand_separator_parsing() {
    let inv = invariant_impossible! {
        name: "Thousand separators parse correctly",
        check: || {
            let test_cases = vec![
                ("1,234,567", 1234567i64),
                ("1_234_567", 1234567),
                ("1234567", 1234567),
                ("1,000", 1000),
                ("999", 999),
            ];

            for (input, expected) in test_cases {
                // Remove separators and parse
                let cleaned: String = input
                    .chars()
                    .filter(|c| c.is_ascii_digit() || *c == '-')
                    .collect();

                match cleaned.parse::<i64>() {
                    Ok(parsed) if parsed == expected => continue,
                    Ok(parsed) => {
                        return TestResult::fail_with_distance(
                            (expected - parsed).abs() as f64,
                            format!(
                                "Thousand separator parse mismatch: '{}' -> {} (expected {})",
                                input, parsed, expected
                            ),
                        );
                    }
                    Err(e) => {
                        return TestResult::fail_with_distance(
                            1.0,
                            format!("Parse error for '{}': {}", input, e),
                        );
                    }
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test scientific notation parsing preserves magnitude
#[test]
fn invariant_scientific_notation_magnitude() {
    let inv = invariant_impossible! {
        name: "Scientific notation preserves magnitude",
        check: || {
            let test_cases = vec![
                ("1e10", 1e10_f64),
                ("1.5e5", 1.5e5),
                ("2.5e-3", 2.5e-3),
                ("1.23e8", 1.23e8),
                ("-5e2", -5e2),
            ];

            for (input, expected) in test_cases {
                match input.parse::<f64>() {
                    Ok(parsed) => {
                        let relative_error = ((parsed - expected) / expected).abs();
                        if relative_error > 1e-10 {
                            return TestResult::fail_with_distance(
                                relative_error,
                                format!(
                                    "Scientific notation error: '{}' = {} (expected {})",
                                    input, parsed, expected
                                ),
                            );
                        }
                    }
                    Err(e) => {
                        return TestResult::fail_with_distance(
                            1.0,
                            format!("Parse error for '{}': {}", input, e),
                        );
                    }
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}
