use rust_wheels::iterators::primitive_ints::*;

use common::{get_expected_test_outputs, TestOutput};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::{random_x, range_decreasing_x, range_increasing_x};
use rust_wheels::prim_utils::traits::{PrimInt, PrimSignedInt, PrimUnsignedInt};

macro_rules! prim_fail {
    ($t: ty, $range_increasing_fail: ident, $range_decreasing_fail: ident) => {
        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $range_increasing_fail() {
            range_increasing_x::<$t>(10, 9);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $range_decreasing_fail() {
            range_decreasing_x::<$t>(10, 9);
        }
    }
}

prim_fail!(u8, range_increasing_u8_fail, range_decreasing_u8_fail);
prim_fail!(u16, range_increasing_u16_fail, range_decreasing_u16_fail);
prim_fail!(u32, range_increasing_u32_fail, range_decreasing_u32_fail);
prim_fail!(u64, range_increasing_u64_fail, range_decreasing_u64_fail);
prim_fail!(
    usize,
    range_increasing_usize_fail,
    range_decreasing_usize_fail
);
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
prim_fail!(
    isize,
    range_increasing_isize_fail_1,
    range_decreasing_isize_fail_1
);

macro_rules! prim_fail_u {
    ($t: ty, $random_range_fail_1: ident, $random_range_fail_2: ident) => {
        #[test]
        #[should_panic(expected = "Range::new called with `low >= high")]
        fn $random_range_fail_1() {
            random_range::<$t>(&EXAMPLE_SEED[..], 10, 9);
        }

        #[test]
        #[should_panic(expected = "Range::new called with `low >= high")]
        fn $random_range_fail_2() {
            random_range::<$t>(&EXAMPLE_SEED[..], 10, 9);
        }
    }
}

prim_fail_u!(u8, range_fail_u8_1, range_fail_u8_2);
prim_fail_u!(u16, range_fail_u16_1, range_fail_u16_2);
prim_fail_u!(u32, range_fail_u32_1, range_fail_u32_2);
prim_fail_u!(u64, range_fail_u64_1, range_fail_u64_2);
prim_fail_u!(usize, range_fail_usize_1, range_fail_usize_2);

macro_rules! prim_fail_i {
    (
        $t: ty,
        $range_increasing_fail: ident,
        $range_decreasing_fail: ident,
        $random_range_fail_1: ident,
        $random_range_fail_2: ident,
        $random_range_fail_3: ident,
        $random_range_fail_4: ident
    ) => {
        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
        fn $range_increasing_fail() {
            range_increasing_x::<i8>(-9, -10);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
        fn $range_decreasing_fail() {
            range_decreasing_x::<i8>(-9, -10);
        }

        #[test]
        #[should_panic(expected = "Range::new called with `low >= high")]
        fn $random_range_fail_1() {
            random_range::<$t>(&EXAMPLE_SEED[..], 10, 9);
        }

        #[test]
        #[should_panic(expected = "Range::new called with `low >= high")]
        fn $random_range_fail_2() {
            random_range::<$t>(&EXAMPLE_SEED[..], 10, 9,);
        }

        #[test]
        #[should_panic(expected = "Range::new called with `low >= high")]
        fn $random_range_fail_3() {
            random_range::<$t>(&EXAMPLE_SEED[..], -9, -10);
        }

        #[test]
        #[should_panic(expected = "Range::new called with `low >= high")]
        fn $random_range_fail_4() {
            random_range::<$t>(&EXAMPLE_SEED[..], -9, -10);
        }
    }
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
prim_fail_i!(
    isize,
    range_increasing_isize_fail_2,
    range_decreasing_isize_fail_2,
    random_range_fail_isize_1,
    random_range_fail_isize_2,
    random_range_fail_isize_3,
    random_range_fail_isize_4
);

fn positive_u_helper<T: PrimUnsignedInt>(eo: &TestOutput) {
    eo.match_vec(
        &format!("exhaustive_positive_{}s", T::name()),
        &mut exhaustive_positive_x::<T>(),
    );
    eo.match_vec_f(
        &format!("random_positive_{}s", T::name()),
        &mut random_positive_u::<T>(&EXAMPLE_SEED[..]),
    );
}

#[test]
fn test_positive_u() {
    let eo = get_expected_test_outputs();
    positive_u_helper::<u8>(&eo);
    positive_u_helper::<u16>(&eo);
    positive_u_helper::<u32>(&eo);
    positive_u_helper::<u64>(&eo);
}

fn positive_i_helper<T: PrimSignedInt>(eo: &TestOutput) {
    eo.match_vec(
        &format!("exhaustive_positive_{}s", T::name()),
        &mut exhaustive_positive_x::<T>(),
    );
    eo.match_vec_f(
        &format!("random_positive_{}s", T::name()),
        &mut random_positive_i::<T>(&EXAMPLE_SEED[..]),
    );
}

#[test]
fn test_positive_i() {
    let eo = get_expected_test_outputs();
    positive_i_helper::<i8>(&eo);
    positive_i_helper::<i16>(&eo);
    positive_i_helper::<i32>(&eo);
    positive_i_helper::<i64>(&eo);
}

fn negative_i_helper<T: PrimSignedInt>(eo: &TestOutput) {
    eo.match_vec(
        &format!("exhaustive_negative_{}s", T::name()),
        &mut exhaustive_negative_i::<T>(),
    );
    eo.match_vec_f(
        &format!("random_negative_{}s", T::name()),
        &mut random_negative_i::<T>(&EXAMPLE_SEED[..]),
    );
}

#[test]
fn test_negative_i() {
    let eo = get_expected_test_outputs();
    negative_i_helper::<i8>(&eo);
    negative_i_helper::<i16>(&eo);
    negative_i_helper::<i32>(&eo);
    negative_i_helper::<i64>(&eo);
}

fn natural_i_helper<T: PrimSignedInt>(eo: &TestOutput) {
    eo.match_vec(
        &format!("exhaustive_natural_{}s", T::name()),
        &mut exhaustive_natural_i::<T>(),
    );
    eo.match_vec_f(
        &format!("random_natural_{}s", T::name()),
        &mut random_natural_i::<T>(&EXAMPLE_SEED[..]),
    );
}

#[test]
fn test_natural_i() {
    let eo = get_expected_test_outputs();
    natural_i_helper::<i8>(&eo);
    natural_i_helper::<i16>(&eo);
    natural_i_helper::<i32>(&eo);
    natural_i_helper::<i64>(&eo);
}

fn nonzero_i_helper<T: PrimSignedInt>(eo: &TestOutput) {
    eo.match_vec(
        &format!("exhaustive_nonzero_{}s", T::name()),
        &mut exhaustive_nonzero_i::<T>(),
    );
    eo.match_vec_f(
        &format!("random_nonzero_{}s", T::name()),
        &mut random_nonzero_i::<T>(&EXAMPLE_SEED[..]),
    );
}

#[test]
fn test_nonzero_i() {
    let eo = get_expected_test_outputs();
    nonzero_i_helper::<i8>(&eo);
    nonzero_i_helper::<i16>(&eo);
    nonzero_i_helper::<i32>(&eo);
    nonzero_i_helper::<i64>(&eo);
}

fn all_u_helper<T: PrimUnsignedInt>(eo: &TestOutput) {
    eo.match_vec(
        &format!("exhaustive_{}s", T::name()),
        &mut exhaustive_u::<T>(),
    );
    eo.match_vec_f(
        &format!("random_{}s", T::name()),
        &mut random_x::<T>(&EXAMPLE_SEED[..]),
    );
}

#[test]
fn test_all_u() {
    let eo = get_expected_test_outputs();
    all_u_helper::<u8>(&eo);
    all_u_helper::<u16>(&eo);
    all_u_helper::<u32>(&eo);
    all_u_helper::<u64>(&eo);
}

fn all_i_helper<T: PrimSignedInt>(eo: &TestOutput) {
    eo.match_vec(
        &format!("exhaustive_{}s", T::name()),
        &mut exhaustive_i::<T>(),
    );
    eo.match_vec_f(
        &format!("random_{}s", T::name()),
        &mut random_x::<T>(&EXAMPLE_SEED[..]),
    );
}

#[test]
fn test_all_i() {
    let eo = get_expected_test_outputs();
    all_i_helper::<i8>(&eo);
    all_i_helper::<i16>(&eo);
    all_i_helper::<i32>(&eo);
    all_i_helper::<i64>(&eo);
}

fn range_up_increasing_u_helper<T: PrimUnsignedInt>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec(
            &format!("exhaustive_range_up_increasing_{}_{}", T::name(), number),
            &mut range_up_increasing_x(a),
        )
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
}

#[test]
fn test_range_up_increasing_u() {
    let eo = get_expected_test_outputs();
    range_up_increasing_u_helper::<u8>(&eo);
    range_up_increasing_u_helper::<u16>(&eo);
    range_up_increasing_u_helper::<u32>(&eo);
    range_up_increasing_u_helper::<u64>(&eo);
}

fn range_up_increasing_i_helper<T: PrimSignedInt>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec(
            &format!("exhaustive_range_up_increasing_{}_{}", T::name(), number),
            &mut range_up_increasing_x(a),
        )
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
    test("iv", T::from_i8(-5));
    test("v", T::min_value());
}

#[test]
fn test_range_up_increasing_i() {
    let eo = get_expected_test_outputs();
    range_up_increasing_i_helper::<i8>(&eo);
    range_up_increasing_i_helper::<i16>(&eo);
    range_up_increasing_i_helper::<i32>(&eo);
    range_up_increasing_i_helper::<i64>(&eo);
}

fn range_up_decreasing_u_helper<T: PrimUnsignedInt>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec(
            &format!("exhaustive_range_up_decreasing_{}_{}", T::name(), number),
            &mut range_up_decreasing_x(a),
        )
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
}

#[test]
fn test_range_up_decreasing_u() {
    let eo = get_expected_test_outputs();
    range_up_decreasing_u_helper::<u8>(&eo);
    range_up_decreasing_u_helper::<u16>(&eo);
    range_up_decreasing_u_helper::<u32>(&eo);
    range_up_decreasing_u_helper::<u64>(&eo);
}

fn range_up_decreasing_i_helper<T: PrimSignedInt>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec(
            &format!("exhaustive_range_up_decreasing_{}_{}", T::name(), number),
            &mut range_up_decreasing_x(a),
        )
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
    test("iv", T::from_i8(-5));
    test("v", T::min_value());
}

#[test]
fn test_range_up_decreasing_i() {
    let eo = get_expected_test_outputs();
    range_up_decreasing_i_helper::<i8>(&eo);
    range_up_decreasing_i_helper::<i16>(&eo);
    range_up_decreasing_i_helper::<i32>(&eo);
    range_up_decreasing_i_helper::<i64>(&eo);
}

fn range_down_increasing_u_helper<T: PrimUnsignedInt>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec(
            &format!("exhaustive_range_down_increasing_{}_{}", T::name(), number),
            &mut range_down_increasing_x(a),
        )
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
}

#[test]
fn test_range_down_increasing_u() {
    let eo = get_expected_test_outputs();
    range_down_increasing_u_helper::<u8>(&eo);
    range_down_increasing_u_helper::<u16>(&eo);
    range_down_increasing_u_helper::<u32>(&eo);
    range_down_increasing_u_helper::<u64>(&eo);
}

fn range_down_increasing_i_helper<T: PrimSignedInt>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec(
            &format!("exhaustive_range_down_increasing_{}_{}", T::name(), number),
            &mut range_down_increasing_x(a),
        )
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
    test("iv", T::from_i8(-5));
    test("v", T::min_value());
}

#[test]
fn test_range_down_increasing_i() {
    let eo = get_expected_test_outputs();
    range_down_increasing_i_helper::<i8>(&eo);
    range_down_increasing_i_helper::<i16>(&eo);
    range_down_increasing_i_helper::<i32>(&eo);
    range_down_increasing_i_helper::<i64>(&eo);
}

fn range_down_decreasing_u_helper<T: PrimUnsignedInt>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec(
            &format!("exhaustive_range_down_decreasing_{}_{}", T::name(), number),
            &mut range_down_decreasing_x(a),
        )
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
}

#[test]
fn test_range_down_decreasing_u() {
    let eo = get_expected_test_outputs();
    range_down_decreasing_u_helper::<u8>(&eo);
    range_down_decreasing_u_helper::<u16>(&eo);
    range_down_decreasing_u_helper::<u32>(&eo);
    range_down_decreasing_u_helper::<u64>(&eo);
}

fn range_down_decreasing_i_helper<T: PrimSignedInt>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec(
            &format!("exhaustive_range_down_decreasing_{}_{}", T::name(), number),
            &mut range_down_decreasing_x(a),
        )
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
    test("iv", T::from_i8(-5));
    test("v", T::min_value());
}

#[test]
fn test_range_down_decreasing_i() {
    let eo = get_expected_test_outputs();
    range_down_decreasing_i_helper::<i8>(&eo);
    range_down_decreasing_i_helper::<i16>(&eo);
    range_down_decreasing_i_helper::<i32>(&eo);
    range_down_decreasing_i_helper::<i64>(&eo);
}

fn range_increasing_u_helper<T: PrimUnsignedInt>(eo: &TestOutput) {
    let test = |number, a, b| {
        eo.match_vec(
            &format!("exhaustive_range_increasing_{}_{}", T::name(), number),
            &mut range_increasing_x(a, b),
        )
    };
    test("i", T::from_u8(0), T::from_u8(0));
    test("ii", T::from_u8(0), T::from_u8(10));
    test("iii", T::from_u8(10), T::from_u8(20));
    test("iv", T::from_u8(10), T::from_u8(10));
    test("v", T::from_u8(0), T::max_value());
    test("vi", T::from_u8(0), T::max_value() - T::from_u8(1));
}

#[test]
fn test_range_increasing_u() {
    let eo = get_expected_test_outputs();
    range_increasing_u_helper::<u8>(&eo);
    range_increasing_u_helper::<u16>(&eo);
    range_increasing_u_helper::<u32>(&eo);
    range_increasing_u_helper::<u64>(&eo);
}

fn range_increasing_i_helper<T: PrimSignedInt>(eo: &TestOutput) {
    let test = |number, a, b| {
        eo.match_vec(
            &format!("exhaustive_range_increasing_{}_{}", T::name(), number),
            &mut range_increasing_x(a, b),
        )
    };
    test("i", T::from_u8(0), T::from_u8(0));
    test("ii", T::from_u8(0), T::from_u8(10));
    test("iii", T::from_u8(10), T::from_u8(20));
    test("iv", T::from_u8(10), T::from_u8(10));
    test("v", T::from_u8(0), T::max_value());
    test("vi", T::from_u8(0), T::max_value() - T::from_u8(1));
    test("vii", T::from_i8(-10), T::from_i8(-10));
    test("viii", T::from_i8(-20), T::from_i8(-10));
    test("ix", T::from_i8(-100), T::from_u8(100));
    test("x", T::min_value(), T::max_value());
    test(
        "xi",
        T::min_value() + T::from_u8(1),
        T::max_value() - T::from_u8(1),
    );
}

#[test]
fn test_range_increasing_i() {
    let eo = get_expected_test_outputs();
    range_increasing_i_helper::<i8>(&eo);
    range_increasing_i_helper::<i16>(&eo);
    range_increasing_i_helper::<i32>(&eo);
    range_increasing_i_helper::<i64>(&eo);
}

fn range_decreasing_u_helper<T: PrimUnsignedInt>(eo: &TestOutput) {
    let test = |number, a, b| {
        eo.match_vec(
            &format!("exhaustive_range_decreasing_{}_{}", T::name(), number),
            &mut range_decreasing_x(a, b),
        )
    };
    test("i", T::from_u8(0), T::from_u8(0));
    test("ii", T::from_u8(0), T::from_u8(10));
    test("iii", T::from_u8(10), T::from_u8(20));
    test("iv", T::from_u8(10), T::from_u8(10));
    test("v", T::from_u8(0), T::max_value());
    test("vi", T::from_u8(0), T::max_value() - T::from_u8(1));
}

#[test]
fn test_range_decreasing_u() {
    let eo = get_expected_test_outputs();
    range_decreasing_u_helper::<u8>(&eo);
    range_decreasing_u_helper::<u16>(&eo);
    range_decreasing_u_helper::<u32>(&eo);
    range_decreasing_u_helper::<u64>(&eo);
}

fn range_decreasing_i_helper<T: PrimSignedInt>(eo: &TestOutput) {
    let test = |number, a, b| {
        eo.match_vec(
            &format!("exhaustive_range_decreasing_{}_{}", T::name(), number),
            &mut range_decreasing_x(a, b),
        )
    };
    test("i", T::from_u8(0), T::from_u8(0));
    test("ii", T::from_u8(0), T::from_u8(10));
    test("iii", T::from_u8(10), T::from_u8(20));
    test("iv", T::from_u8(10), T::from_u8(10));
    test("v", T::from_u8(0), T::max_value());
    test("vi", T::from_u8(0), T::max_value() - T::from_u8(1));
    test("vii", T::from_i8(-10), T::from_i8(-10));
    test("viii", T::from_i8(-20), T::from_i8(-10));
    test("ix", T::from_i8(-100), T::from_u8(100));
    test("x", T::min_value(), T::max_value());
    test(
        "xi",
        T::min_value() + T::from_u8(1),
        T::max_value() - T::from_u8(1),
    );
}

#[test]
fn test_range_decreasing_i() {
    let eo = get_expected_test_outputs();
    range_decreasing_i_helper::<i8>(&eo);
    range_decreasing_i_helper::<i16>(&eo);
    range_decreasing_i_helper::<i32>(&eo);
    range_decreasing_i_helper::<i64>(&eo);
}

fn x_increasing_helper<T: PrimInt>(eo: &TestOutput) {
    eo.match_vec(
        &format!("exhaustive_{}s_increasing", T::name()),
        &mut x_increasing::<T>(),
    );
}

#[test]
fn test_x_increasing() {
    let eo = get_expected_test_outputs();
    x_increasing_helper::<u8>(&eo);
    x_increasing_helper::<u16>(&eo);
    x_increasing_helper::<u32>(&eo);
    x_increasing_helper::<u64>(&eo);
    x_increasing_helper::<i8>(&eo);
    x_increasing_helper::<i16>(&eo);
    x_increasing_helper::<i32>(&eo);
    x_increasing_helper::<i64>(&eo);
}

fn x_decreasing_helper<T: PrimInt>(eo: &TestOutput) {
    eo.match_vec(
        &format!("exhaustive_{}s_decreasing", T::name()),
        &mut x_decreasing::<T>(),
    );
}

#[test]
fn test_x_decreasing() {
    let eo = get_expected_test_outputs();
    x_decreasing_helper::<u8>(&eo);
    x_decreasing_helper::<u16>(&eo);
    x_decreasing_helper::<u32>(&eo);
    x_decreasing_helper::<u64>(&eo);
    x_decreasing_helper::<i8>(&eo);
    x_decreasing_helper::<i16>(&eo);
    x_decreasing_helper::<i32>(&eo);
    x_decreasing_helper::<i64>(&eo);
}

fn random_range_up_u_helper<T: PrimUnsignedInt>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec_f(
            &format!("random_range_up_{}_{}", T::name(), number),
            &mut random_range_up::<T>(&EXAMPLE_SEED[..], a),
        )
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
}

fn random_range_up_i_helper<T: PrimSignedInt>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec_f(
            &format!("random_range_up_{}_{}", T::name(), number),
            &mut random_range_up::<T>(&EXAMPLE_SEED[..], a),
        )
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
    test("iv", T::from_i8(-5));
    test("v", T::min_value());
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

fn random_range_down_u_helper<T: PrimUnsignedInt>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec_f(
            &format!("random_range_down_{}_{}", T::name(), number),
            &mut random_range_down::<T>(&EXAMPLE_SEED[..], a),
        )
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
}

fn random_range_down_i_helper<T: PrimSignedInt>(eo: &TestOutput) {
    let test = |number, a| {
        eo.match_vec_f(
            &format!("random_range_down_{}_{}", T::name(), number),
            &mut random_range_down::<T>(&EXAMPLE_SEED[..], a),
        )
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
    test("iv", T::from_i8(-5));
    test("v", T::min_value());
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

fn random_range_u_helper<T: PrimUnsignedInt>(eo: &TestOutput) {
    let test = |number, a, b| {
        eo.match_vec_f(
            &format!("random_range_{}_{}", T::name(), number),
            &mut random_range::<T>(&EXAMPLE_SEED[..], a, b),
        )
    };
    test("i", T::from_u8(0), T::from_u8(0));
    test("ii", T::from_u8(0), T::from_u8(10));
    test("iii", T::from_u8(10), T::from_u8(20));
    test("iv", T::from_u8(10), T::from_u8(10));
    test("v", T::from_u8(0), T::max_value());
    test("vi", T::from_u8(0), T::max_value() - T::from_u8(1));
}

fn range_i_helper<T: PrimSignedInt>(eo: &TestOutput) {
    let test = |number, a, b| {
        eo.match_vec(
            &format!("exhaustive_range_{}_{}", T::name(), number),
            &mut exhaustive_range_i::<T>(a, b),
        );
        eo.match_vec_f(
            &format!("random_range_{}_{}", T::name(), number),
            &mut random_range::<T>(&EXAMPLE_SEED[..], a, b),
        )
    };
    test("i", T::from_u8(0), T::from_u8(0));
    test("ii", T::from_u8(0), T::from_u8(10));
    test("iii", T::from_u8(10), T::from_u8(20));
    test("iv", T::from_u8(10), T::from_u8(10));
    test("v", T::from_u8(0), T::max_value());
    test("vi", T::from_u8(0), T::max_value() - T::from_u8(1));
    test("vii", T::from_i8(-10), T::from_i8(-10));
    test("viii", T::from_i8(-20), T::from_i8(-10));
    test("ix", T::from_i8(-100), T::from_u8(100));
    test("x", T::min_value(), T::max_value());
    test(
        "xi",
        T::min_value() + T::from_u8(1),
        T::max_value() - T::from_u8(1),
    );
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
