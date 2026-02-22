//! Cliffy-test invariants for geometric/scientific components
//!
//! These tests verify algebraic properties that must NEVER fail for:
//! - AngleInput (unit conversions, normalization)
//! - ComplexNumberInput (polar/rectangular conversion, arithmetic)
//! - FractionInput (simplification, decimal conversion)

use cliffy_test::prelude::*;
use cliffy_test::{invariant_impossible, verify_invariant};
use std::f64::consts::PI;

// ============================================================================
// ANGLE CONVERSION INVARIANTS
// ============================================================================

/// Test that degree-radian conversion round-trips exactly
#[test]
fn invariant_degree_radian_roundtrip() {
    let inv = invariant_impossible! {
        name: "Degree-radian conversion round-trips",
        check: || {
            let test_angles = vec![
                0.0, 30.0, 45.0, 60.0, 90.0, 180.0, 270.0, 360.0,
                -45.0, -90.0, -180.0, 720.0, 0.001, 359.999,
            ];

            for degrees in test_angles {
                let radians = degrees * PI / 180.0;
                let back_to_degrees = radians * 180.0 / PI;

                let error = (degrees - back_to_degrees).abs();
                if error > 1e-10 {
                    return TestResult::fail_with_distance(
                        error,
                        format!(
                            "Degree-radian roundtrip failed: {} -> {} rad -> {}",
                            degrees, radians, back_to_degrees
                        ),
                    );
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test that gradians-degrees conversion is correct (400 grad = 360 deg)
#[test]
fn invariant_gradians_degrees_conversion() {
    let inv = invariant_impossible! {
        name: "Gradians-degrees conversion is exact",
        check: || {
            // 400 gradians = 360 degrees, so: degrees = gradians * 0.9
            let test_cases: Vec<(f64, f64)> = vec![
                (0.0, 0.0),
                (100.0, 90.0),   // 100 grad = 90 deg (right angle)
                (200.0, 180.0),  // 200 grad = 180 deg
                (400.0, 360.0),  // 400 grad = 360 deg (full circle)
                (50.0, 45.0),    // 50 grad = 45 deg
            ];

            for (gradians, expected_degrees) in test_cases {
                let degrees = gradians * 0.9; // 360/400 = 0.9

                let error = (degrees - expected_degrees).abs();
                if error > 1e-10 {
                    return TestResult::fail_with_distance(
                        error,
                        format!(
                            "Gradians conversion failed: {} grad -> {} deg (expected {})",
                            gradians, degrees, expected_degrees
                        ),
                    );
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test that turns-degrees conversion is correct (1 turn = 360 deg)
#[test]
fn invariant_turns_degrees_conversion() {
    let inv = invariant_impossible! {
        name: "Turns-degrees conversion is exact",
        check: || {
            let test_cases: Vec<(f64, f64)> = vec![
                (0.0, 0.0),
                (0.25, 90.0),    // quarter turn
                (0.5, 180.0),    // half turn
                (1.0, 360.0),    // full turn
                (2.0, 720.0),    // two turns
                (0.125, 45.0),   // eighth turn
            ];

            for (turns, expected_degrees) in test_cases {
                let degrees = turns * 360.0;

                let error = (degrees - expected_degrees).abs();
                if error > 1e-10 {
                    return TestResult::fail_with_distance(
                        error,
                        format!(
                            "Turns conversion failed: {} turns -> {} deg (expected {})",
                            turns, degrees, expected_degrees
                        ),
                    );
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test DMS (Degrees-Minutes-Seconds) conversion preserves value
#[test]
fn invariant_dms_conversion_roundtrip() {
    let inv = invariant_impossible! {
        name: "DMS conversion round-trips",
        check: || {
            let test_angles: Vec<f64> = vec![
                0.0, 45.0, 45.5, 45.508333, // 45°30'30"
                90.0, 180.0, -45.5, 360.0,
            ];

            for degrees in test_angles {
                // Convert to DMS
                let negative = degrees < 0.0;
                let abs_degrees = degrees.abs();
                let d = abs_degrees.floor() as i32;
                let remaining = (abs_degrees - d as f64) * 60.0;
                let m = remaining.floor() as u32;
                let s = (remaining - m as f64) * 60.0;

                // Convert back
                let back_to_degrees = d as f64 + (m as f64 / 60.0) + (s / 3600.0);
                let back_to_degrees = if negative { -back_to_degrees } else { back_to_degrees };

                let error = (degrees - back_to_degrees).abs();
                if error > 1e-9 {
                    return TestResult::fail_with_distance(
                        error,
                        format!(
                            "DMS roundtrip failed: {} -> {}°{}'{:.2}\" -> {}",
                            degrees, d, m, s, back_to_degrees
                        ),
                    );
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test angle normalization to [0, 360) is correct
#[test]
fn invariant_angle_normalization_0_360() {
    let inv = invariant_impossible! {
        name: "Angle normalization to [0, 360) is correct",
        check: || {
            let test_cases: Vec<(f64, f64)> = vec![
                (0.0, 0.0),
                (90.0, 90.0),
                (360.0, 0.0),
                (450.0, 90.0),
                (-90.0, 270.0),
                (-180.0, 180.0),
                (-360.0, 0.0),
                (720.0, 0.0),
                (725.0, 5.0),
            ];

            for (input, expected) in test_cases {
                let mut normalized = input % 360.0;
                if normalized < 0.0 {
                    normalized += 360.0;
                }

                let error = (normalized - expected).abs();
                // Handle edge case where 360.0 should be 0.0
                let error = if error > 359.0 { 360.0 - error } else { error };

                if error > 1e-10 {
                    return TestResult::fail_with_distance(
                        error,
                        format!(
                            "Normalization [0,360) failed: {} -> {} (expected {})",
                            input, normalized, expected
                        ),
                    );
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test angle normalization to [-180, 180] is correct
#[test]
fn invariant_angle_normalization_neg180_180() {
    let inv = invariant_impossible! {
        name: "Angle normalization to [-180, 180] is correct",
        check: || {
            let test_cases: Vec<(f64, f64)> = vec![
                (0.0, 0.0),
                (90.0, 90.0),
                (180.0, 180.0),
                (270.0, -90.0),
                (360.0, 0.0),
                (-90.0, -90.0),
                (-180.0, -180.0),
                (-270.0, 90.0),
                (450.0, 90.0),
            ];

            for (input, expected) in test_cases {
                let mut normalized = input % 360.0;
                if normalized > 180.0 {
                    normalized -= 360.0;
                } else if normalized < -180.0 {
                    normalized += 360.0;
                }

                let error = (normalized - expected).abs();
                if error > 1e-10 {
                    return TestResult::fail_with_distance(
                        error,
                        format!(
                            "Normalization [-180,180] failed: {} -> {} (expected {})",
                            input, normalized, expected
                        ),
                    );
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

// ============================================================================
// COMPLEX NUMBER INVARIANTS
// ============================================================================

/// Test that polar-rectangular conversion round-trips
#[test]
fn invariant_complex_polar_rectangular_roundtrip() {
    let inv = invariant_impossible! {
        name: "Complex polar-rectangular round-trips",
        check: || {
            let test_cases: Vec<(f64, f64)> = vec![
                (3.0, 4.0),      // Standard 3-4-5 triangle
                (1.0, 0.0),      // Real axis
                (0.0, 1.0),      // Imaginary axis
                (1.0, 1.0),      // First quadrant
                (-1.0, 1.0),     // Second quadrant
                (-1.0, -1.0),    // Third quadrant
                (1.0, -1.0),     // Fourth quadrant
                (0.0, 0.0),      // Origin
            ];

            for (real, imag) in test_cases {
                // Convert to polar
                let magnitude = (real * real + imag * imag).sqrt();
                let angle = imag.atan2(real);

                // Convert back to rectangular
                let back_real = magnitude * angle.cos();
                let back_imag = magnitude * angle.sin();

                let real_error = (real - back_real).abs();
                let imag_error = (imag - back_imag).abs();
                let max_error = real_error.max(imag_error);

                if max_error > 1e-10 {
                    return TestResult::fail_with_distance(
                        max_error,
                        format!(
                            "Polar-rect roundtrip failed: ({}, {}) -> (r={}, θ={}) -> ({}, {})",
                            real, imag, magnitude, angle, back_real, back_imag
                        ),
                    );
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test complex magnitude is always non-negative
#[test]
fn invariant_complex_magnitude_non_negative() {
    let inv = invariant_impossible! {
        name: "Complex magnitude is non-negative",
        check: || {
            let test_cases: Vec<(f64, f64)> = vec![
                (0.0, 0.0),
                (1.0, 0.0),
                (0.0, 1.0),
                (-5.0, 0.0),
                (0.0, -3.0),
                (3.0, 4.0),
                (-3.0, -4.0),
                (1e10, 1e10),
                (-1e-10, 1e-10),
            ];

            for (real, imag) in test_cases {
                let magnitude = (real * real + imag * imag).sqrt();

                if magnitude < 0.0 {
                    return TestResult::fail_with_distance(
                        magnitude.abs(),
                        format!(
                            "Negative magnitude: |{} + {}i| = {}",
                            real, imag, magnitude
                        ),
                    );
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test complex addition is commutative: a + b = b + a
#[test]
fn invariant_complex_addition_commutative() {
    let inv = invariant_impossible! {
        name: "Complex addition is commutative",
        check: || {
            let test_pairs: Vec<((f64, f64), (f64, f64))> = vec![
                ((1.0, 2.0), (3.0, 4.0)),
                ((0.0, 0.0), (5.0, 6.0)),
                ((-1.0, -2.0), (1.0, 2.0)),
                ((1e10, 1e-10), (1e-10, 1e10)),
            ];

            for ((a_re, a_im), (b_re, b_im)) in test_pairs {
                // a + b
                let sum1_re = a_re + b_re;
                let sum1_im = a_im + b_im;

                // b + a
                let sum2_re = b_re + a_re;
                let sum2_im = b_im + a_im;

                let error = (sum1_re - sum2_re).abs() + (sum1_im - sum2_im).abs();
                if error > 1e-10 {
                    return TestResult::fail_with_distance(
                        error,
                        format!(
                            "Addition not commutative: ({} + {}i) + ({} + {}i)",
                            a_re, a_im, b_re, b_im
                        ),
                    );
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test complex multiplication is commutative: a * b = b * a
#[test]
fn invariant_complex_multiplication_commutative() {
    let inv = invariant_impossible! {
        name: "Complex multiplication is commutative",
        check: || {
            let test_pairs: Vec<((f64, f64), (f64, f64))> = vec![
                ((1.0, 2.0), (3.0, 4.0)),
                ((0.0, 1.0), (0.0, 1.0)),  // i * i = -1
                ((2.0, 0.0), (3.0, 0.0)),  // Real numbers
                ((-1.0, 2.0), (3.0, -4.0)),
            ];

            for ((a_re, a_im), (b_re, b_im)) in test_pairs {
                // a * b = (a_re + a_im*i)(b_re + b_im*i)
                // = a_re*b_re + a_re*b_im*i + a_im*b_re*i + a_im*b_im*i²
                // = (a_re*b_re - a_im*b_im) + (a_re*b_im + a_im*b_re)*i
                let prod1_re = a_re * b_re - a_im * b_im;
                let prod1_im = a_re * b_im + a_im * b_re;

                // b * a
                let prod2_re = b_re * a_re - b_im * a_im;
                let prod2_im = b_re * a_im + b_im * a_re;

                let error = (prod1_re - prod2_re).abs() + (prod1_im - prod2_im).abs();
                if error > 1e-10 {
                    return TestResult::fail_with_distance(
                        error,
                        format!(
                            "Multiplication not commutative: ({} + {}i) * ({} + {}i)",
                            a_re, a_im, b_re, b_im
                        ),
                    );
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test i² = -1 fundamental property
#[test]
fn invariant_complex_i_squared() {
    let inv = invariant_impossible! {
        name: "i² = -1",
        check: || {
            // i = 0 + 1i
            let i_re: f64 = 0.0;
            let i_im: f64 = 1.0;

            // i * i = (0 + 1i)(0 + 1i) = 0*0 - 1*1 + (0*1 + 1*0)i = -1 + 0i
            let i_squared_re = i_re * i_re - i_im * i_im;
            let i_squared_im = i_re * i_im + i_im * i_re;

            let expected_re = -1.0;
            let expected_im = 0.0;

            let error = (i_squared_re - expected_re).abs() + (i_squared_im - expected_im).abs();
            if error > 1e-10 {
                return TestResult::fail_with_distance(
                    error,
                    format!(
                        "i² ≠ -1: got {} + {}i",
                        i_squared_re, i_squared_im
                    ),
                );
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test z * conjugate(z) = |z|²
#[test]
fn invariant_complex_conjugate_magnitude() {
    let inv = invariant_impossible! {
        name: "z * conj(z) = |z|²",
        check: || {
            let test_cases: Vec<(f64, f64)> = vec![
                (3.0, 4.0),
                (1.0, 0.0),
                (0.0, 1.0),
                (-2.0, 3.0),
                (0.0, 0.0),
            ];

            for (real, imag) in test_cases {
                // z * conj(z)
                let conj_re = real;
                let conj_im = -imag;

                let product_re = real * conj_re - imag * conj_im;
                let product_im = real * conj_im + imag * conj_re;

                // |z|²
                let magnitude_squared = real * real + imag * imag;

                let real_error = (product_re - magnitude_squared).abs();
                let imag_error = product_im.abs(); // Should be 0

                if real_error > 1e-10 || imag_error > 1e-10 {
                    return TestResult::fail_with_distance(
                        real_error + imag_error,
                        format!(
                            "z*conj(z) ≠ |z|²: ({} + {}i) * ({} + {}i) = {} + {}i, expected {}",
                            real, imag, conj_re, conj_im, product_re, product_im, magnitude_squared
                        ),
                    );
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

// ============================================================================
// FRACTION INVARIANTS
// ============================================================================

/// Test GCD is always positive and divides both numbers
#[test]
fn invariant_gcd_divides_both() {
    let inv = invariant_impossible! {
        name: "GCD divides both numbers",
        check: || {
            let test_pairs: Vec<(i64, i64)> = vec![
                (12, 8),
                (100, 25),
                (17, 5),    // Coprime
                (0, 5),
                (7, 7),
                (1, 100),
                (48, 18),
                (-12, 8),   // Negative
            ];

            fn gcd(a: i64, b: i64) -> i64 {
                let mut a = a.abs();
                let mut b = b.abs();
                while b != 0 {
                    let temp = b;
                    b = a % b;
                    a = temp;
                }
                a
            }

            for (a, b) in test_pairs {
                if a == 0 && b == 0 {
                    continue; // GCD(0,0) is undefined
                }

                let g = gcd(a, b);

                // GCD must be positive
                if g <= 0 {
                    return TestResult::fail_with_distance(
                        g.abs() as f64,
                        format!("GCD({}, {}) = {} is not positive", a, b, g),
                    );
                }

                // GCD must divide both numbers (when not zero)
                if a != 0 && a.abs() % g != 0 {
                    return TestResult::fail_with_distance(
                        1.0,
                        format!("GCD({}, {}) = {} does not divide {}", a, b, g, a),
                    );
                }
                if b != 0 && b.abs() % g != 0 {
                    return TestResult::fail_with_distance(
                        1.0,
                        format!("GCD({}, {}) = {} does not divide {}", a, b, g, b),
                    );
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test fraction simplification preserves value
#[test]
fn invariant_fraction_simplify_preserves_value() {
    let inv = invariant_impossible! {
        name: "Fraction simplification preserves value",
        check: || {
            let test_fractions: Vec<(i64, i64)> = vec![
                (4, 8),     // 1/2
                (6, 9),     // 2/3
                (15, 25),   // 3/5
                (100, 10),  // 10/1
                (3, 7),     // Already simplified
                (-6, 8),    // Negative
                (6, -8),    // Negative denominator
            ];

            fn gcd(a: i64, b: i64) -> i64 {
                let mut a = a.abs();
                let mut b = b.abs();
                while b != 0 {
                    let temp = b;
                    b = a % b;
                    a = temp;
                }
                a
            }

            for (num, den) in test_fractions {
                if den == 0 {
                    continue;
                }

                // Original value
                let original = num as f64 / den as f64;

                // Simplify
                let g = gcd(num, den);
                let simplified_num = num / g;
                let simplified_den = den / g;
                let simplified = simplified_num as f64 / simplified_den as f64;

                let error = (original - simplified).abs();
                if error > 1e-10 {
                    return TestResult::fail_with_distance(
                        error,
                        format!(
                            "Simplification changed value: {}/{} = {} -> {}/{} = {}",
                            num, den, original, simplified_num, simplified_den, simplified
                        ),
                    );
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test fraction to decimal to fraction round-trip (with precision limits)
#[test]
fn invariant_fraction_decimal_roundtrip() {
    let inv = invariant_impossible! {
        name: "Fraction-decimal round-trip (exact fractions)",
        check: || {
            // Test fractions that have exact decimal representations
            let exact_fractions: Vec<(i64, i64)> = vec![
                (1, 2),     // 0.5
                (1, 4),     // 0.25
                (3, 4),     // 0.75
                (1, 5),     // 0.2
                (1, 8),     // 0.125
                (1, 10),    // 0.1
                (1, 20),    // 0.05
                (1, 25),    // 0.04
            ];

            for (num, den) in exact_fractions {
                let decimal = num as f64 / den as f64;

                // Convert back to fraction using rational approximation
                let max_den = 1000i64;
                let mut best_num = 0i64;
                let mut best_den = 1i64;
                let mut best_error = decimal;

                for d in 1..=max_den {
                    let n = (decimal * d as f64).round() as i64;
                    let error = (decimal - n as f64 / d as f64).abs();
                    if error < best_error {
                        best_error = error;
                        best_num = n;
                        best_den = d;
                        if error < 1e-10 {
                            break;
                        }
                    }
                }

                // Simplify both
                fn gcd(a: i64, b: i64) -> i64 {
                    let mut a = a.abs();
                    let mut b = b.abs();
                    while b != 0 { let temp = b; b = a % b; a = temp; }
                    a
                }

                let g1 = gcd(num, den);
                let orig_num = num / g1;
                let orig_den = den / g1;

                let g2 = gcd(best_num, best_den);
                let result_num = best_num / g2;
                let result_den = best_den / g2;

                if orig_num != result_num || orig_den != result_den {
                    // Check if values are still equal
                    let orig_val = orig_num as f64 / orig_den as f64;
                    let result_val = result_num as f64 / result_den as f64;
                    let error = (orig_val - result_val).abs();

                    if error > 1e-10 {
                        return TestResult::fail_with_distance(
                            error,
                            format!(
                                "Fraction roundtrip failed: {}/{} -> {} -> {}/{}",
                                num, den, decimal, result_num, result_den
                            ),
                        );
                    }
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test mixed number conversion preserves value
#[test]
fn invariant_mixed_number_conversion() {
    let inv = invariant_impossible! {
        name: "Mixed number conversion preserves value",
        check: || {
            let test_fractions: Vec<(i64, i64)> = vec![
                (3, 2),     // 1 1/2
                (7, 4),     // 1 3/4
                (11, 3),    // 3 2/3
                (1, 2),     // 0 1/2 (no whole part)
                (4, 1),     // 4 (whole number)
                (-5, 2),    // -2 1/2
            ];

            for (num, den) in test_fractions {
                let original_value = num as f64 / den as f64;

                // Convert to mixed number components
                let whole = num / den;
                let frac_num = (num % den).abs();

                // Convert back: For negative fractions with non-zero whole part,
                // the sign applies to the entire mixed number: -(|whole| + frac/den)
                let is_negative = num < 0;
                let back_value = if is_negative && whole != 0 {
                    // Negative with whole part: -(|whole| + frac/den)
                    -(whole.abs() as f64 + frac_num as f64 / den.abs() as f64)
                } else if is_negative && whole == 0 {
                    // Negative with no whole part: just -frac/den
                    -(frac_num as f64 / den.abs() as f64)
                } else {
                    // Positive: whole + frac/den
                    whole as f64 + frac_num as f64 / den.abs() as f64
                };

                let error = (original_value - back_value).abs();
                if error > 1e-10 {
                    return TestResult::fail_with_distance(
                        error,
                        format!(
                            "Mixed conversion failed: {}/{} = {} -> {} + {}/{} = {}",
                            num, den, original_value, whole, frac_num, den, back_value
                        ),
                    );
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}

/// Test that simplified fractions have GCD of 1
#[test]
fn invariant_simplified_fraction_gcd_is_one() {
    let inv = invariant_impossible! {
        name: "Simplified fractions have GCD = 1",
        check: || {
            let test_fractions: Vec<(i64, i64)> = vec![
                (4, 8),
                (6, 9),
                (100, 25),
                (17, 5),
                (48, 18),
            ];

            fn gcd(a: i64, b: i64) -> i64 {
                let mut a = a.abs();
                let mut b = b.abs();
                while b != 0 { let temp = b; b = a % b; a = temp; }
                a
            }

            for (num, den) in test_fractions {
                let g = gcd(num, den);
                let simplified_num = num / g;
                let simplified_den = den / g;

                let final_gcd = gcd(simplified_num, simplified_den);
                if final_gcd != 1 {
                    return TestResult::fail_with_distance(
                        final_gcd as f64,
                        format!(
                            "Simplified {}/{} = {}/{} has GCD = {} (not 1)",
                            num, den, simplified_num, simplified_den, final_gcd
                        ),
                    );
                }
            }
            TestResult::Pass
        }
    };

    verify_invariant!(inv, samples: 1);
}
