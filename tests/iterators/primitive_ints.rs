use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use rand::distributions::range::SampleRange;
use rand::Rand;

use common::{get_expected_test_outputs, TestOutput};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::primitive_ints::*;

macro_rules! prim_fail_u {
    ($t:ty, $random_range_fail_1:ident, $random_range_fail_2:ident) => {
        #[test]
        #[should_panic(expected = "Range::new called with `low >= high")]
        fn $random_range_fail_1() {
            random_range::<$t>(&EXAMPLE_SEED, 10, 9);
        }

        #[test]
        #[should_panic(expected = "Range::new called with `low >= high")]
        fn $random_range_fail_2() {
            random_range::<$t>(&EXAMPLE_SEED, 10, 9);
        }
    };
}

prim_fail_u!(u8, range_fail_u8_1, range_fail_u8_2);
prim_fail_u!(u16, range_fail_u16_1, range_fail_u16_2);
prim_fail_u!(u32, range_fail_u32_1, range_fail_u32_2);
prim_fail_u!(u64, range_fail_u64_1, range_fail_u64_2);

macro_rules! prim_fail_i {
    (
        $t:ty,
        $random_range_fail_1:ident,
        $random_range_fail_2:ident,
        $random_range_fail_3:ident,
        $random_range_fail_4:ident
    ) => {
        #[test]
        #[should_panic(expected = "Range::new called with `low >= high")]
        fn $random_range_fail_1() {
            random_range::<$t>(&EXAMPLE_SEED, 10, 9);
        }

        #[test]
        #[should_panic(expected = "Range::new called with `low >= high")]
        fn $random_range_fail_2() {
            random_range::<$t>(&EXAMPLE_SEED, 10, 9);
        }

        #[test]
        #[should_panic(expected = "Range::new called with `low >= high")]
        fn $random_range_fail_3() {
            random_range::<$t>(&EXAMPLE_SEED, -9, -10);
        }

        #[test]
        #[should_panic(expected = "Range::new called with `low >= high")]
        fn $random_range_fail_4() {
            random_range::<$t>(&EXAMPLE_SEED, -9, -10);
        }
    };
}

prim_fail_i!(
    i8,
    random_range_fail_i8_1,
    random_range_fail_i8_2,
    random_range_fail_i8_3,
    random_range_fail_i8_4
);
prim_fail_i!(
    i16,
    random_range_fail_i16_1,
    random_range_fail_i16_2,
    random_range_fail_i16_3,
    random_range_fail_i16_4
);
prim_fail_i!(
    i32,
    random_range_fail_i32_1,
    random_range_fail_i32_2,
    random_range_fail_i32_3,
    random_range_fail_i32_4
);
prim_fail_i!(
    i64,
    random_range_fail_i64_1,
    random_range_fail_i64_2,
    random_range_fail_i64_3,
    random_range_fail_i64_4
);

fn random_range_up_u_helper<T: PrimitiveUnsigned + Rand + SampleRange>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec_f(
            &format!("random_range_up_{}_{}", T::NAME, number),
            &mut random_range_up::<T>(&EXAMPLE_SEED, a),
        )
    };
    test("i", T::ZERO);
    test("ii", T::from(5));
    test("iii", T::MAX);
}

fn random_range_up_i_helper<T: PrimitiveSigned + Rand + SampleRange>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec_f(
            &format!("random_range_up_{}_{}", T::NAME, number),
            &mut random_range_up::<T>(&EXAMPLE_SEED, a),
        )
    };
    test("i", T::ZERO);
    test("ii", T::from(5));
    test("iii", T::MAX);
    test("iv", T::from(-5));
    test("v", T::MIN);
}

#[test]
fn test_random_range_up() {
    let eo = get_expected_test_outputs();
    random_range_up_u_helper::<u8>(&eo);
    random_range_up_u_helper::<u16>(&eo);
    random_range_up_u_helper::<u32>(&eo);
    random_range_up_u_helper::<u64>(&eo);
    random_range_up_i_helper::<i8>(&eo);
    random_range_up_i_helper::<i16>(&eo);
    random_range_up_i_helper::<i32>(&eo);
    random_range_up_i_helper::<i64>(&eo);
}

fn random_range_down_u_helper<T: PrimitiveUnsigned + Rand + SampleRange>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec_f(
            &format!("random_range_down_{}_{}", T::NAME, number),
            &mut random_range_down::<T>(&EXAMPLE_SEED, a),
        )
    };
    test("i", T::ZERO);
    test("ii", T::from(5));
    test("iii", T::MAX);
}

fn random_range_down_i_helper<T: PrimitiveSigned + Rand + SampleRange>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec_f(
            &format!("random_range_down_{}_{}", T::NAME, number),
            &mut random_range_down::<T>(&EXAMPLE_SEED, a),
        )
    };
    test("i", T::ZERO);
    test("ii", T::from(5));
    test("iii", T::MAX);
    test("iv", T::from(-5));
    test("v", T::MIN);
}

#[test]
fn test_random_range_down() {
    let eo = get_expected_test_outputs();
    random_range_down_u_helper::<u8>(&eo);
    random_range_down_u_helper::<u16>(&eo);
    random_range_down_u_helper::<u32>(&eo);
    random_range_down_u_helper::<u64>(&eo);
    random_range_down_i_helper::<i8>(&eo);
    random_range_down_i_helper::<i16>(&eo);
    random_range_down_i_helper::<i32>(&eo);
    random_range_down_i_helper::<i64>(&eo);
}

fn random_range_u_helper<T: PrimitiveUnsigned + Rand + SampleRange>(eo: &TestOutput) {
    let test = |number, a, b| {
        eo.match_vec_f(
            &format!("random_range_{}_{}", T::NAME, number),
            &mut random_range::<T>(&EXAMPLE_SEED, a, b),
        )
    };
    test("i", T::ZERO, T::ZERO);
    test("ii", T::ZERO, T::from(10));
    test("iii", T::from(10), T::from(20));
    test("iv", T::from(10), T::from(10));
    test("v", T::ZERO, T::MAX);
    test("vi", T::ZERO, T::MAX - T::ONE);
}

fn range_i_helper<T: PrimitiveSigned + Rand + SampleRange>(eo: &TestOutput) {
    let test = |number, a, b| {
        eo.match_vec_f(
            &format!("random_range_{}_{}", T::NAME, number),
            &mut random_range::<T>(&EXAMPLE_SEED, a, b),
        )
    };
    test("i", T::ZERO, T::ZERO);
    test("ii", T::ZERO, T::from(10));
    test("iii", T::from(10), T::from(20));
    test("iv", T::from(10), T::from(10));
    test("v", T::ZERO, T::MAX);
    test("vi", T::ZERO, T::MAX - T::ONE);
    test("vii", T::from(-10), T::from(-10));
    test("viii", T::from(-20), T::from(-10));
    test("ix", T::from(-100), T::from(100));
    test("x", T::MIN, T::MAX);
    test("xi", T::MIN + T::ONE, T::MAX - T::ONE);
}

#[test]
fn test_range() {
    let eo = get_expected_test_outputs();
    random_range_u_helper::<u8>(&eo);
    random_range_u_helper::<u16>(&eo);
    random_range_u_helper::<u32>(&eo);
    random_range_u_helper::<u64>(&eo);
    range_i_helper::<i8>(&eo);
    range_i_helper::<i16>(&eo);
    range_i_helper::<i32>(&eo);
    range_i_helper::<i64>(&eo);
}
