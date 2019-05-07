use malachite_base::named::Named;

use common::get_expected_test_outputs;
use rust_wheels::iterators::primitive_floats::*;

macro_rules! test_primitive_floats {
    (
        $f: ident,
        $exhaustive_positive_finite_primitive_floats: ident,
        $exhaustive_negative_finite_primitive_floats: ident,
        $exhaustive_nonzero_finite_primitive_floats: ident,
        $exhaustive_positive_primitive_floats: ident,
        $exhaustive_negative_primitive_floats: ident,
        $exhaustive_nonzero_primitive_floats: ident,
        $exhaustive_primitive_floats: ident,
        $test_exhaustive_positive_finite_primitive_floats: ident,
        $test_exhaustive_negative_finite_primitive_floats: ident,
        $test_exhaustive_nonzero_finite_primitive_floats: ident,
        $test_exhaustive_positive_primitive_floats: ident,
        $test_exhaustive_negative_primitive_floats: ident,
        $test_exhaustive_nonzero_primitive_floats: ident,
        $test_exhaustive_primitive_floats: ident
    ) => {
        #[test]
        fn $test_exhaustive_positive_finite_primitive_floats() {
            let eo = get_expected_test_outputs();
            eo.match_vec(
                &format!("exhaustive_positive_finite_primitive_floats_{}", $f::NAME),
                &mut $exhaustive_positive_finite_primitive_floats(),
            );
        }

        #[test]
        fn $test_exhaustive_negative_finite_primitive_floats() {
            let eo = get_expected_test_outputs();
            eo.match_vec(
                &format!("exhaustive_negative_finite_primitive_floats_{}", $f::NAME),
                &mut $exhaustive_negative_finite_primitive_floats(),
            );
        }

        #[test]
        fn $test_exhaustive_nonzero_finite_primitive_floats() {
            let eo = get_expected_test_outputs();
            eo.match_vec(
                &format!("exhaustive_nonzero_finite_primitive_floats_{}", $f::NAME),
                &mut $exhaustive_nonzero_finite_primitive_floats(),
            );
        }

        #[test]
        fn $test_exhaustive_positive_primitive_floats() {
            let eo = get_expected_test_outputs();
            eo.match_vec(
                &format!("exhaustive_positive_primitive_floats_{}", $f::NAME),
                &mut $exhaustive_positive_primitive_floats(),
            );
        }

        #[test]
        fn $test_exhaustive_negative_primitive_floats() {
            let eo = get_expected_test_outputs();
            eo.match_vec(
                &format!("exhaustive_negative_primitive_floats_{}", $f::NAME),
                &mut $exhaustive_negative_primitive_floats(),
            );
        }

        #[test]
        fn $test_exhaustive_nonzero_primitive_floats() {
            let eo = get_expected_test_outputs();
            eo.match_vec(
                &format!("exhaustive_nonzero_primitive_floats_{}", $f::NAME),
                &mut $exhaustive_nonzero_primitive_floats(),
            );
        }

        #[test]
        fn $test_exhaustive_primitive_floats() {
            let eo = get_expected_test_outputs();
            eo.match_vec(
                &format!("exhaustive_primitive_floats_{}", $f::NAME),
                &mut $exhaustive_primitive_floats(),
            );
        }
    };
}

test_primitive_floats!(
    f32,
    exhaustive_positive_finite_f32s,
    exhaustive_negative_finite_f32s,
    exhaustive_nonzero_finite_f32s,
    exhaustive_positive_f32s,
    exhaustive_negative_f32s,
    exhaustive_nonzero_f32s,
    exhaustive_f32s,
    test_exhaustive_positive_finite_f32s,
    test_exhaustive_negative_finite_f32s,
    test_exhaustive_nonzero_finite_f32s,
    test_exhaustive_positive_f32s,
    test_exhaustive_negative_f32s,
    test_exhaustive_nonzero_f32s,
    test_exhaustive_f32s
);
test_primitive_floats!(
    f64,
    exhaustive_positive_finite_f64s,
    exhaustive_negative_finite_f64s,
    exhaustive_nonzero_finite_f64s,
    exhaustive_positive_f64s,
    exhaustive_negative_f64s,
    exhaustive_nonzero_f64s,
    exhaustive_f64s,
    test_exhaustive_positive_finite_f64s,
    test_exhaustive_negative_finite_f64s,
    test_exhaustive_nonzero_finite_f64s,
    test_exhaustive_positive_f64s,
    test_exhaustive_negative_f64s,
    test_exhaustive_nonzero_f64s,
    test_exhaustive_f64s
);
