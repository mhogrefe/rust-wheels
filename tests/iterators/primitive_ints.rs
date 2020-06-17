use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use rand::distributions::range::SampleRange;
use rand::Rand;

use common::{get_expected_test_outputs, TestOutput};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::{random, range_decreasing, range_increasing};
use rust_wheels::iterators::primitive_ints::*;

macro_rules! prim_fail {
    ($t:ty, $range_increasing_fail:ident, $range_decreasing_fail:ident) => {
        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $range_increasing_fail() {
            range_increasing::<$t>(10, 9);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $range_decreasing_fail() {
            range_decreasing::<$t>(10, 9);
        }
    };
}

prim_fail!(u8, range_increasing_u8_fail, range_decreasing_u8_fail);
prim_fail!(u16, range_increasing_u16_fail, range_decreasing_u16_fail);
prim_fail!(u32, range_increasing_u32_fail, range_decreasing_u32_fail);
prim_fail!(u64, range_increasing_u64_fail, range_decreasing_u64_fail);
prim_fail!(i8, range_increasing_i8_fail_1, range_decreasing_i8_fail_1);
prim_fail!(
    i16,
    range_increasing_i16_fail_1,
    range_decreasing_i16_fail_1
);
prim_fail!(
    i32,
    range_increasing_i32_fail_1,
    range_decreasing_i32_fail_1
);
prim_fail!(
    i64,
    range_increasing_i64_fail_1,
    range_decreasing_i64_fail_1
);

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
        $range_increasing_fail:ident,
        $range_decreasing_fail:ident,
        $random_range_fail_1:ident,
        $random_range_fail_2:ident,
        $random_range_fail_3:ident,
        $random_range_fail_4:ident
    ) => {
        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
        fn $range_increasing_fail() {
            range_increasing::<i8>(-9, -10);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
        fn $range_decreasing_fail() {
            range_decreasing::<i8>(-9, -10);
        }

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
    range_increasing_i8_fail_2,
    range_decreasing_i8_fail_2,
    random_range_fail_i8_1,
    random_range_fail_i8_2,
    random_range_fail_i8_3,
    random_range_fail_i8_4
);
prim_fail_i!(
    i16,
    range_increasing_i16_fail_2,
    range_decreasing_i16_fail_2,
    random_range_fail_i16_1,
    random_range_fail_i16_2,
    random_range_fail_i16_3,
    random_range_fail_i16_4
);
prim_fail_i!(
    i32,
    range_increasing_i32_fail_2,
    range_decreasing_i32_fail_2,
    random_range_fail_i32_1,
    random_range_fail_i32_2,
    random_range_fail_i32_3,
    random_range_fail_i32_4
);
prim_fail_i!(
    i64,
    range_increasing_i64_fail_2,
    range_decreasing_i64_fail_2,
    random_range_fail_i64_1,
    random_range_fail_i64_2,
    random_range_fail_i64_3,
    random_range_fail_i64_4
);

fn positive_unsigned_helper<T: PrimitiveUnsigned + Rand>(eo: &TestOutput) {
    eo.match_vec(
        &format!("exhaustive_positive_{}s", T::NAME),
        &mut exhaustive_positive::<T>(),
    );
    eo.match_vec_f(
        &format!("random_positive_{}s", T::NAME),
        &mut random_positive_unsigned::<T>(&EXAMPLE_SEED),
    );
    // eo.match_vec_f_binary(
    //     &format!("special_random_positive_{}s", T::NAME),
    //     &mut special_random_positive_unsigned::<T>(&EXAMPLE_SEED),
    // );
}

#[test]
fn test_positive_unsigned() {
    let eo = get_expected_test_outputs();
    positive_unsigned_helper::<u8>(&eo);
    positive_unsigned_helper::<u16>(&eo);
    positive_unsigned_helper::<u32>(&eo);
    positive_unsigned_helper::<u64>(&eo);
}

fn positive_signed_helper<T: PrimitiveSigned + Rand>(eo: &TestOutput)
where
    <T as PrimitiveSigned>::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    eo.match_vec(
        &format!("exhaustive_positive_{}s", T::NAME),
        &mut exhaustive_positive::<T>(),
    );
    eo.match_vec_f(
        &format!("random_positive_{}s", T::NAME),
        &mut random_positive_signed::<T>(&EXAMPLE_SEED),
    );
    // eo.match_vec_f_binary(
    //     &format!("special_random_positive_{}s", T::NAME),
    //     &mut special_random_positive_signed::<T>(&EXAMPLE_SEED),
    // );
}

#[test]
fn test_positive_signed() {
    let eo = get_expected_test_outputs();
    positive_signed_helper::<i8>(&eo);
    positive_signed_helper::<i16>(&eo);
    positive_signed_helper::<i32>(&eo);
    positive_signed_helper::<i64>(&eo);
}

fn negative_signed_helper<T: PrimitiveSigned + Rand>(eo: &TestOutput)
where
    <T as PrimitiveSigned>::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    eo.match_vec(
        &format!("exhaustive_negative_{}s", T::NAME),
        &mut exhaustive_negative_signed::<T>(),
    );
    eo.match_vec_f(
        &format!("random_negative_{}s", T::NAME),
        &mut random_negative_signed::<T>(&EXAMPLE_SEED),
    );
    // eo.match_vec_f_binary(
    //     &format!("special_random_negative_{}s", T::NAME),
    //     &mut special_random_negative_signed::<T>(&EXAMPLE_SEED),
    // );
}

#[test]
fn test_negative_signed() {
    let eo = get_expected_test_outputs();
    negative_signed_helper::<i8>(&eo);
    negative_signed_helper::<i16>(&eo);
    negative_signed_helper::<i32>(&eo);
    negative_signed_helper::<i64>(&eo);
}

fn natural_signed_helper<T: PrimitiveSigned + Rand>(eo: &TestOutput)
where
    <T as PrimitiveSigned>::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    eo.match_vec(
        &format!("exhaustive_natural_{}s", T::NAME),
        &mut exhaustive_natural_signed::<T>(),
    );
    eo.match_vec_f(
        &format!("random_natural_{}s", T::NAME),
        &mut random_natural_signed::<T>(&EXAMPLE_SEED),
    );
    // eo.match_vec_f_binary(
    //     &format!("special_random_natural_{}s", T::NAME),
    //     &mut special_random_natural_signed::<T>(&EXAMPLE_SEED),
    // );
}

#[test]
fn test_natural_signed() {
    let eo = get_expected_test_outputs();
    natural_signed_helper::<i8>(&eo);
    natural_signed_helper::<i16>(&eo);
    natural_signed_helper::<i32>(&eo);
    natural_signed_helper::<i64>(&eo);
}

fn nonzero_signed_helper<T: PrimitiveSigned + Rand>(eo: &TestOutput)
where
    <T as PrimitiveSigned>::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    eo.match_vec(
        &format!("exhaustive_nonzero_{}s", T::NAME),
        &mut exhaustive_nonzero_signed::<T>(),
    );
    eo.match_vec_f(
        &format!("random_nonzero_{}s", T::NAME),
        &mut random_nonzero_signed::<T>(&EXAMPLE_SEED),
    );
    // eo.match_vec_f_binary(
    //     &format!("special_random_nonzero_{}s", T::NAME),
    //     &mut special_random_nonzero_signed::<T>(&EXAMPLE_SEED),
    // );
}

#[test]
fn test_nonzero_signed() {
    let eo = get_expected_test_outputs();
    nonzero_signed_helper::<i8>(&eo);
    nonzero_signed_helper::<i16>(&eo);
    nonzero_signed_helper::<i32>(&eo);
    nonzero_signed_helper::<i64>(&eo);
}

fn all_unsigned_helper<T: PrimitiveUnsigned + Rand>(eo: &TestOutput) {
    eo.match_vec(
        &format!("exhaustive_{}s", T::NAME),
        &mut exhaustive_unsigned::<T>(),
    );
    eo.match_vec_f(
        &format!("random_{}s", T::NAME),
        &mut random::<T>(&EXAMPLE_SEED),
    );
    // eo.match_vec_f_binary(
    //     &format!("special_random_{}s", T::NAME),
    //     &mut special_random_unsigned::<T>(&EXAMPLE_SEED),
    // );
}

#[test]
fn test_all_unsigned() {
    let eo = get_expected_test_outputs();
    all_unsigned_helper::<u8>(&eo);
    all_unsigned_helper::<u16>(&eo);
    all_unsigned_helper::<u32>(&eo);
    all_unsigned_helper::<u64>(&eo);
}

fn all_signed_helper<T: PrimitiveSigned + Rand>(eo: &TestOutput)
where
    <T as PrimitiveSigned>::UnsignedOfEqualWidth: Rand,
    T: WrappingFrom<<T as PrimitiveSigned>::UnsignedOfEqualWidth>,
{
    eo.match_vec(
        &format!("exhaustive_{}s", T::NAME),
        &mut exhaustive_signed::<T>(),
    );
    eo.match_vec_f(
        &format!("random_{}s", T::NAME),
        &mut random::<T>(&EXAMPLE_SEED),
    );
    // eo.match_vec_f_binary(
    //     &format!("special_random_{}s", T::NAME),
    //     &mut special_random_signed::<T>(&EXAMPLE_SEED),
    // );
}

#[test]
fn test_all_signed() {
    let eo = get_expected_test_outputs();
    all_signed_helper::<i8>(&eo);
    all_signed_helper::<i16>(&eo);
    all_signed_helper::<i32>(&eo);
    all_signed_helper::<i64>(&eo);
}

fn range_up_increasing_unsigned_helper<T: PrimitiveUnsigned>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec(
            &format!("exhaustive_range_up_increasing_{}_{}", T::NAME, number),
            &mut range_up_increasing(a),
        )
    };
    test("i", T::ZERO);
    test("ii", T::from(5));
    test("iii", T::MAX);
}

#[test]
fn test_range_up_increasing_unsigned() {
    let eo = get_expected_test_outputs();
    range_up_increasing_unsigned_helper::<u8>(&eo);
    range_up_increasing_unsigned_helper::<u16>(&eo);
    range_up_increasing_unsigned_helper::<u32>(&eo);
    range_up_increasing_unsigned_helper::<u64>(&eo);
}

fn range_up_increasing_signed_helper<T: PrimitiveSigned>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec(
            &format!("exhaustive_range_up_increasing_{}_{}", T::NAME, number),
            &mut range_up_increasing(a),
        )
    };
    test("i", T::ZERO);
    test("ii", T::from(5));
    test("iii", T::MAX);
    test("iv", T::from(-5));
    test("v", T::MIN);
}

#[test]
fn test_range_up_increasing_signed() {
    let eo = get_expected_test_outputs();
    range_up_increasing_signed_helper::<i8>(&eo);
    range_up_increasing_signed_helper::<i16>(&eo);
    range_up_increasing_signed_helper::<i32>(&eo);
    range_up_increasing_signed_helper::<i64>(&eo);
}

fn range_up_decreasing_unsigned_helper<T: PrimitiveUnsigned>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec(
            &format!("exhaustive_range_up_decreasing_{}_{}", T::NAME, number),
            &mut range_up_decreasing(a),
        )
    };
    test("i", T::ZERO);
    test("ii", T::from(5));
    test("iii", T::MAX);
}

#[test]
fn test_range_up_decreasing_unsigned() {
    let eo = get_expected_test_outputs();
    range_up_decreasing_unsigned_helper::<u8>(&eo);
    range_up_decreasing_unsigned_helper::<u16>(&eo);
    range_up_decreasing_unsigned_helper::<u32>(&eo);
    range_up_decreasing_unsigned_helper::<u64>(&eo);
}

fn range_up_decreasing_signed_helper<T: PrimitiveSigned>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec(
            &format!("exhaustive_range_up_decreasing_{}_{}", T::NAME, number),
            &mut range_up_decreasing(a),
        )
    };
    test("i", T::ZERO);
    test("ii", T::from(5));
    test("iii", T::MAX);
    test("iv", T::from(-5));
    test("v", T::MIN);
}

#[test]
fn test_range_up_decreasing_signed() {
    let eo = get_expected_test_outputs();
    range_up_decreasing_signed_helper::<i8>(&eo);
    range_up_decreasing_signed_helper::<i16>(&eo);
    range_up_decreasing_signed_helper::<i32>(&eo);
    range_up_decreasing_signed_helper::<i64>(&eo);
}

fn range_down_increasing_unsigned_helper<T: PrimitiveUnsigned>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec(
            &format!("exhaustive_range_down_increasing_{}_{}", T::NAME, number),
            &mut range_down_increasing(a),
        )
    };
    test("i", T::ZERO);
    test("ii", T::from(5));
    test("iii", T::MAX);
}

#[test]
fn test_range_down_increasing_unsigned() {
    let eo = get_expected_test_outputs();
    range_down_increasing_unsigned_helper::<u8>(&eo);
    range_down_increasing_unsigned_helper::<u16>(&eo);
    range_down_increasing_unsigned_helper::<u32>(&eo);
    range_down_increasing_unsigned_helper::<u64>(&eo);
}

fn range_down_increasing_signed_helper<T: PrimitiveSigned>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec(
            &format!("exhaustive_range_down_increasing_{}_{}", T::NAME, number),
            &mut range_down_increasing(a),
        )
    };
    test("i", T::ZERO);
    test("ii", T::from(5));
    test("iii", T::MAX);
    test("iv", T::from(-5));
    test("v", T::MIN);
}

#[test]
fn test_range_down_increasing_signed() {
    let eo = get_expected_test_outputs();
    range_down_increasing_signed_helper::<i8>(&eo);
    range_down_increasing_signed_helper::<i16>(&eo);
    range_down_increasing_signed_helper::<i32>(&eo);
    range_down_increasing_signed_helper::<i64>(&eo);
}

fn range_down_decreasing_unsigned_helper<T: PrimitiveUnsigned>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec(
            &format!("exhaustive_range_down_decreasing_{}_{}", T::NAME, number),
            &mut range_down_decreasing(a),
        )
    };
    test("i", T::ZERO);
    test("ii", T::from(5));
    test("iii", T::MAX);
}

#[test]
fn test_range_down_decreasing_unsigned() {
    let eo = get_expected_test_outputs();
    range_down_decreasing_unsigned_helper::<u8>(&eo);
    range_down_decreasing_unsigned_helper::<u16>(&eo);
    range_down_decreasing_unsigned_helper::<u32>(&eo);
    range_down_decreasing_unsigned_helper::<u64>(&eo);
}

fn range_down_decreasing_signed_helper<T: PrimitiveSigned>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec(
            &format!("exhaustive_range_down_decreasing_{}_{}", T::NAME, number),
            &mut range_down_decreasing(a),
        )
    };
    test("i", T::ZERO);
    test("ii", T::from(5));
    test("iii", T::MAX);
    test("iv", T::from(-5));
    test("v", T::MIN);
}

#[test]
fn test_range_down_decreasing_signed() {
    let eo = get_expected_test_outputs();
    range_down_decreasing_signed_helper::<i8>(&eo);
    range_down_decreasing_signed_helper::<i16>(&eo);
    range_down_decreasing_signed_helper::<i32>(&eo);
    range_down_decreasing_signed_helper::<i64>(&eo);
}

fn range_increasing_unsigned_helper<T: PrimitiveUnsigned>(eo: &TestOutput) {
    let test = |number, a, b| {
        eo.match_vec(
            &format!("exhaustive_range_increasing_{}_{}", T::NAME, number),
            &mut range_increasing(a, b),
        )
    };
    test("i", T::ZERO, T::ZERO);
    test("ii", T::ZERO, T::from(10));
    test("iii", T::from(10), T::from(20));
    test("iv", T::from(10), T::from(10));
    test("v", T::ZERO, T::MAX);
    test("vi", T::ZERO, T::MAX - T::ONE);
}

#[test]
fn test_range_increasing_unsigned() {
    let eo = get_expected_test_outputs();
    range_increasing_unsigned_helper::<u8>(&eo);
    range_increasing_unsigned_helper::<u16>(&eo);
    range_increasing_unsigned_helper::<u32>(&eo);
    range_increasing_unsigned_helper::<u64>(&eo);
}

fn range_increasing_signed_helper<T: PrimitiveSigned>(eo: &TestOutput) {
    let test = |number, a, b| {
        eo.match_vec(
            &format!("exhaustive_range_increasing_{}_{}", T::NAME, number),
            &mut range_increasing(a, b),
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
fn test_range_increasing_signed() {
    let eo = get_expected_test_outputs();
    range_increasing_signed_helper::<i8>(&eo);
    range_increasing_signed_helper::<i16>(&eo);
    range_increasing_signed_helper::<i32>(&eo);
    range_increasing_signed_helper::<i64>(&eo);
}

fn range_decreasing_unsigned_helper<T: PrimitiveUnsigned>(eo: &TestOutput) {
    let test = |number, a, b| {
        eo.match_vec(
            &format!("exhaustive_range_decreasing_{}_{}", T::NAME, number),
            &mut range_decreasing(a, b),
        )
    };
    test("i", T::ZERO, T::ZERO);
    test("ii", T::ZERO, T::from(10));
    test("iii", T::from(10), T::from(20));
    test("iv", T::from(10), T::from(10));
    test("v", T::ZERO, T::MAX);
    test("vi", T::ZERO, T::MAX - T::ONE);
}

#[test]
fn test_range_decreasing_unsigned() {
    let eo = get_expected_test_outputs();
    range_decreasing_unsigned_helper::<u8>(&eo);
    range_decreasing_unsigned_helper::<u16>(&eo);
    range_decreasing_unsigned_helper::<u32>(&eo);
    range_decreasing_unsigned_helper::<u64>(&eo);
}

fn range_decreasing_signed_helper<T: PrimitiveSigned>(eo: &TestOutput) {
    let test = |number, a, b| {
        eo.match_vec(
            &format!("exhaustive_range_decreasing_{}_{}", T::NAME, number),
            &mut range_decreasing(a, b),
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
fn test_range_decreasing_signed() {
    let eo = get_expected_test_outputs();
    range_decreasing_signed_helper::<i8>(&eo);
    range_decreasing_signed_helper::<i16>(&eo);
    range_decreasing_signed_helper::<i32>(&eo);
    range_decreasing_signed_helper::<i64>(&eo);
}

fn increasing_helper<T: PrimitiveInteger>(eo: &TestOutput) {
    eo.match_vec(
        &format!("exhaustive_{}s_increasing", T::NAME),
        &mut increasing::<T>(),
    );
}

#[test]
fn test_increasing() {
    let eo = get_expected_test_outputs();
    increasing_helper::<u8>(&eo);
    increasing_helper::<u16>(&eo);
    increasing_helper::<u32>(&eo);
    increasing_helper::<u64>(&eo);
    increasing_helper::<i8>(&eo);
    increasing_helper::<i16>(&eo);
    increasing_helper::<i32>(&eo);
    increasing_helper::<i64>(&eo);
}

fn decreasing_helper<T: PrimitiveInteger>(eo: &TestOutput) {
    eo.match_vec(
        &format!("exhaustive_{}s_decreasing", T::NAME),
        &mut decreasing::<T>(),
    );
}

#[test]
fn test_decreasing() {
    let eo = get_expected_test_outputs();
    decreasing_helper::<u8>(&eo);
    decreasing_helper::<u16>(&eo);
    decreasing_helper::<u32>(&eo);
    decreasing_helper::<u64>(&eo);
    decreasing_helper::<i8>(&eo);
    decreasing_helper::<i16>(&eo);
    decreasing_helper::<i32>(&eo);
    decreasing_helper::<i64>(&eo);
}

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
        eo.match_vec(
            &format!("exhaustive_range_{}_{}", T::NAME, number),
            &mut exhaustive_range_signed::<T>(a, b),
        );
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
