#[cfg(feature = "gmp")]
use malachite_gmp::integer::Integer;
#[cfg(feature = "native")]
use malachite_native::integer::Integer;
use common::{get_expected_test_outputs, TestOutput};
use rust_wheels::io::readers::parse_vec;
use rust_wheels::iterators::iterator_provider::{EXAMPLE_SEED, exhaustive_bools,
                                                exhaustive_from_vector, IteratorProvider,
                                                random_bools};
use rust_wheels::prim_utils::traits::*;
use std::char;
use std::str::FromStr;

fn prepare_test() -> (TestOutput, IteratorProvider, IteratorProvider) {
    (get_expected_test_outputs(), IteratorProvider::Exhaustive, IteratorProvider::example_random())
}

macro_rules! prim_fail {
    ($t: ty, $range_increasing_fail: ident, $range_decreasing_fail: ident) => {
        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $range_increasing_fail() {
            IteratorProvider::Exhaustive.range_increasing_x::<$t>(10, 9);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $range_decreasing_fail() {
            IteratorProvider::Exhaustive.range_decreasing_x::<$t>(10, 9);
        }
    }
}

prim_fail!(u8, range_increasing_u8_fail, range_decreasing_u8_fail);
prim_fail!(u16, range_increasing_u16_fail, range_decreasing_u16_fail);
prim_fail!(u32, range_increasing_u32_fail, range_decreasing_u32_fail);
prim_fail!(u64, range_increasing_u64_fail, range_decreasing_u64_fail);
prim_fail!(usize,
           range_increasing_usize_fail,
           range_decreasing_usize_fail);
prim_fail!(i8, range_increasing_i8_fail_1, range_decreasing_i8_fail_1);
prim_fail!(i16,
           range_increasing_i16_fail_1,
           range_decreasing_i16_fail_1);
prim_fail!(i32,
           range_increasing_i32_fail_1,
           range_decreasing_i32_fail_1);
prim_fail!(i64,
           range_increasing_i64_fail_1,
           range_decreasing_i64_fail_1);
prim_fail!(isize,
           range_increasing_isize_fail_1,
           range_decreasing_isize_fail_1);

macro_rules! prim_fail_u {
    ($t: ty, $range_fail_1: ident, $range_fail_2: ident) => {
        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $range_fail_1() {
            IteratorProvider::Exhaustive.range_u::<$t>(10, 9);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $range_fail_2() {
            IteratorProvider::example_random().range_u::<$t>(10, 9);
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
        $range_fail_1: ident,
        $range_fail_2: ident,
        $range_fail_3: ident,
        $range_fail_4: ident
    ) => {
        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
        fn $range_increasing_fail() {
            IteratorProvider::Exhaustive.range_increasing_x::<i8>(-9, -10);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
        fn $range_decreasing_fail() {
            IteratorProvider::Exhaustive.range_decreasing_x::<i8>(-9, -10);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $range_fail_1() {
            IteratorProvider::Exhaustive.range_i::<$t>(10, 9);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $range_fail_2() {
            IteratorProvider::example_random().range_i::<$t>(10, 9);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
        fn $range_fail_3() {
            IteratorProvider::Exhaustive.range_i::<$t>(-9, -10);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
        fn $range_fail_4() {
            IteratorProvider::example_random().range_i::<$t>(-9, -10);
        }
    }
}

prim_fail_i!(i8,
             range_increasing_i8_fail_2,
             range_decreasing_i8_fail_2,
             range_fail_i8_1,
             range_fail_i8_2,
             range_fail_i8_3,
             range_fail_i8_4);
prim_fail_i!(i16,
             range_increasing_i16_fail_2,
             range_decreasing_i16_fail_2,
             range_fail_i16_1,
             range_fail_i16_2,
             range_fail_i16_3,
             range_fail_i16_4);
prim_fail_i!(i32,
             range_increasing_i32_fail_2,
             range_decreasing_i32_fail_2,
             range_fail_i32_1,
             range_fail_i32_2,
             range_fail_i32_3,
             range_fail_i32_4);
prim_fail_i!(i64,
             range_increasing_i64_fail_2,
             range_decreasing_i64_fail_2,
             range_fail_i64_1,
             range_fail_i64_2,
             range_fail_i64_3,
             range_fail_i64_4);
prim_fail_i!(isize,
             range_increasing_isize_fail_2,
             range_decreasing_isize_fail_2,
             range_fail_isize_1,
             range_fail_isize_2,
             range_fail_isize_3,
             range_fail_isize_4);

#[test]
fn test_bools() {
    let eo = get_expected_test_outputs();
    eo.match_vec("exhaustive_bools", &mut exhaustive_bools());
    eo.match_vec_f("random_bools", &mut random_bools(&EXAMPLE_SEED[..]));
}

fn range_up_increasing_u_helper<T: PrimUnsignedInt>(eo: &TestOutput, p: &IteratorProvider) {
    let test = |number, a| {
        eo.match_vec(&format!("exhaustive_range_up_increasing_{}_{}", T::name(), number),
                     &mut p.range_up_increasing_x(a))
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
}

#[test]
fn test_range_up_increasing_u() {
    let (eo, p, _) = prepare_test();
    range_up_increasing_u_helper::<u8>(&eo, &p);
    range_up_increasing_u_helper::<u16>(&eo, &p);
    range_up_increasing_u_helper::<u32>(&eo, &p);
    range_up_increasing_u_helper::<u64>(&eo, &p);
}

fn range_up_increasing_i_helper<T: PrimSignedInt>(eo: &TestOutput, p: &IteratorProvider) {
    let test = |number, a| {
        eo.match_vec(&format!("exhaustive_range_up_increasing_{}_{}", T::name(), number),
                     &mut p.range_up_increasing_x(a))
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
    test("iv", T::from_i8(-5));
    test("v", T::min_value());
}

#[test]
fn test_range_up_increasing_i() {
    let (eo, p, _) = prepare_test();
    range_up_increasing_i_helper::<i8>(&eo, &p);
    range_up_increasing_i_helper::<i16>(&eo, &p);
    range_up_increasing_i_helper::<i32>(&eo, &p);
    range_up_increasing_i_helper::<i64>(&eo, &p);
}

fn range_up_decreasing_u_helper<T: PrimUnsignedInt>(eo: &TestOutput, p: &IteratorProvider) {
    let test = |number, a| {
        eo.match_vec(&format!("exhaustive_range_up_decreasing_{}_{}", T::name(), number),
                     &mut p.range_up_decreasing_x(a))
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
}

#[test]
fn test_range_up_decreasing_u() {
    let (eo, p, _) = prepare_test();
    range_up_decreasing_u_helper::<u8>(&eo, &p);
    range_up_decreasing_u_helper::<u16>(&eo, &p);
    range_up_decreasing_u_helper::<u32>(&eo, &p);
    range_up_decreasing_u_helper::<u64>(&eo, &p);
}

fn range_up_decreasing_i_helper<T: PrimSignedInt>(eo: &TestOutput, p: &IteratorProvider) {
    let test = |number, a| {
        eo.match_vec(&format!("exhaustive_range_up_decreasing_{}_{}", T::name(), number),
                     &mut p.range_up_decreasing_x(a))
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
    test("iv", T::from_i8(-5));
    test("v", T::min_value());
}

#[test]
fn test_range_up_decreasing_i() {
    let (eo, p, _) = prepare_test();
    range_up_decreasing_i_helper::<i8>(&eo, &p);
    range_up_decreasing_i_helper::<i16>(&eo, &p);
    range_up_decreasing_i_helper::<i32>(&eo, &p);
    range_up_decreasing_i_helper::<i64>(&eo, &p);
}

fn range_down_increasing_u_helper<T: PrimUnsignedInt>(eo: &TestOutput, p: &IteratorProvider) {
    let test = |number, a| {
        eo.match_vec(&format!("exhaustive_range_down_increasing_{}_{}", T::name(), number),
                     &mut p.range_down_increasing_x(a))
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
}

#[test]
fn test_range_down_increasing_u() {
    let (eo, p, _) = prepare_test();
    range_down_increasing_u_helper::<u8>(&eo, &p);
    range_down_increasing_u_helper::<u16>(&eo, &p);
    range_down_increasing_u_helper::<u32>(&eo, &p);
    range_down_increasing_u_helper::<u64>(&eo, &p);
}

fn range_down_increasing_i_helper<T: PrimSignedInt>(eo: &TestOutput, p: &IteratorProvider) {
    let test = |number, a| {
        eo.match_vec(&format!("exhaustive_range_down_increasing_{}_{}", T::name(), number),
                     &mut p.range_down_increasing_x(a))
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
    test("iv", T::from_i8(-5));
    test("v", T::min_value());
}

#[test]
fn test_range_down_increasing_i() {
    let (eo, p, _) = prepare_test();
    range_down_increasing_i_helper::<i8>(&eo, &p);
    range_down_increasing_i_helper::<i16>(&eo, &p);
    range_down_increasing_i_helper::<i32>(&eo, &p);
    range_down_increasing_i_helper::<i64>(&eo, &p);
}

fn range_down_decreasing_u_helper<T: PrimUnsignedInt>(eo: &TestOutput, p: &IteratorProvider) {
    let test = |number, a| {
        eo.match_vec(&format!("exhaustive_range_down_decreasing_{}_{}", T::name(), number),
                     &mut p.range_down_decreasing_x(a))
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
}

#[test]
fn test_range_down_decreasing_u() {
    let (eo, p, _) = prepare_test();
    range_down_decreasing_u_helper::<u8>(&eo, &p);
    range_down_decreasing_u_helper::<u16>(&eo, &p);
    range_down_decreasing_u_helper::<u32>(&eo, &p);
    range_down_decreasing_u_helper::<u64>(&eo, &p);
}

fn range_down_decreasing_i_helper<T: PrimSignedInt>(eo: &TestOutput, p: &IteratorProvider) {
    let test = |number, a| {
        eo.match_vec(&format!("exhaustive_range_down_decreasing_{}_{}", T::name(), number),
                     &mut p.range_down_decreasing_x(a))
    };
    test("i", T::from_u8(0));
    test("ii", T::from_u8(5));
    test("iii", T::max_value());
    test("iv", T::from_i8(-5));
    test("v", T::min_value());
}

#[test]
fn test_range_down_decreasing_i() {
    let (eo, p, _) = prepare_test();
    range_down_decreasing_i_helper::<i8>(&eo, &p);
    range_down_decreasing_i_helper::<i16>(&eo, &p);
    range_down_decreasing_i_helper::<i32>(&eo, &p);
    range_down_decreasing_i_helper::<i64>(&eo, &p);
}

fn range_increasing_u_helper<T: PrimUnsignedInt>(eo: &TestOutput, p: &IteratorProvider) {
    let test = |number, a, b| {
        eo.match_vec(&format!("exhaustive_range_increasing_{}_{}", T::name(), number),
                     &mut p.range_increasing_x(a, b))
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
    let (eo, p, _) = prepare_test();
    range_increasing_u_helper::<u8>(&eo, &p);
    range_increasing_u_helper::<u16>(&eo, &p);
    range_increasing_u_helper::<u32>(&eo, &p);
    range_increasing_u_helper::<u64>(&eo, &p);
}

fn range_increasing_i_helper<T: PrimSignedInt>(eo: &TestOutput, p: &IteratorProvider) {
    let test = |number, a, b| {
        eo.match_vec(&format!("exhaustive_range_increasing_{}_{}", T::name(), number),
                     &mut p.range_increasing_x(a, b))
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
    test("xi",
         T::min_value() + T::from_u8(1),
         T::max_value() - T::from_u8(1));
}

#[test]
fn test_range_increasing_i() {
    let (eo, p, _) = prepare_test();
    range_increasing_i_helper::<i8>(&eo, &p);
    range_increasing_i_helper::<i16>(&eo, &p);
    range_increasing_i_helper::<i32>(&eo, &p);
    range_increasing_i_helper::<i64>(&eo, &p);
}

fn range_decreasing_u_helper<T: PrimUnsignedInt>(eo: &TestOutput, p: &IteratorProvider) {
    let test = |number, a, b| {
        eo.match_vec(&format!("exhaustive_range_decreasing_{}_{}", T::name(), number),
                     &mut p.range_decreasing_x(a, b))
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
    let (eo, p, _) = prepare_test();
    range_decreasing_u_helper::<u8>(&eo, &p);
    range_decreasing_u_helper::<u16>(&eo, &p);
    range_decreasing_u_helper::<u32>(&eo, &p);
    range_decreasing_u_helper::<u64>(&eo, &p);
}

fn range_decreasing_i_helper<T: PrimSignedInt>(eo: &TestOutput, p: &IteratorProvider) {
    let test = |number, a, b| {
        eo.match_vec(&format!("exhaustive_range_decreasing_{}_{}", T::name(), number),
                     &mut p.range_decreasing_x(a, b))
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
    test("xi",
         T::min_value() + T::from_u8(1),
         T::max_value() - T::from_u8(1));
}

#[test]
fn test_range_decreasing_i() {
    let (eo, p, _) = prepare_test();
    range_decreasing_i_helper::<i8>(&eo, &p);
    range_decreasing_i_helper::<i16>(&eo, &p);
    range_decreasing_i_helper::<i32>(&eo, &p);
    range_decreasing_i_helper::<i64>(&eo, &p);
}

fn x_increasing_helper<T: PrimInt>(eo: &TestOutput, p: &IteratorProvider) {
    eo.match_vec(&format!("exhaustive_{}s_increasing", T::name()),
                 &mut p.x_increasing::<T>());
}

#[test]
fn test_x_increasing() {
    let (eo, p, _) = prepare_test();
    x_increasing_helper::<u8>(&eo, &p);
    x_increasing_helper::<u16>(&eo, &p);
    x_increasing_helper::<u32>(&eo, &p);
    x_increasing_helper::<u64>(&eo, &p);
    x_increasing_helper::<i8>(&eo, &p);
    x_increasing_helper::<i16>(&eo, &p);
    x_increasing_helper::<i32>(&eo, &p);
    x_increasing_helper::<i64>(&eo, &p);
}

fn x_decreasing_helper<T: PrimInt>(eo: &TestOutput, p: &IteratorProvider) {
    eo.match_vec(&format!("exhaustive_{}s_decreasing", T::name()),
                 &mut p.x_decreasing::<T>());
}

#[test]
fn test_x_decreasing() {
    let (eo, p, _) = prepare_test();
    x_decreasing_helper::<u8>(&eo, &p);
    x_decreasing_helper::<u16>(&eo, &p);
    x_decreasing_helper::<u32>(&eo, &p);
    x_decreasing_helper::<u64>(&eo, &p);
    x_decreasing_helper::<i8>(&eo, &p);
    x_decreasing_helper::<i16>(&eo, &p);
    x_decreasing_helper::<i32>(&eo, &p);
    x_decreasing_helper::<i64>(&eo, &p);
}

fn positive_u_helper<T: PrimUnsignedInt>(eo: &TestOutput,
                                         ep: &IteratorProvider,
                                         rp: &IteratorProvider) {
    eo.match_vec(&format!("exhaustive_positive_{}s", T::name()),
                 &mut ep.positive_u::<T>());
    eo.match_vec_f(&format!("random_positive_{}s", T::name()),
                   &mut rp.positive_u::<T>());
}

#[test]
fn test_positive_u() {
    let (eo, ep, rp) = prepare_test();
    positive_u_helper::<u8>(&eo, &ep, &rp);
    positive_u_helper::<u16>(&eo, &ep, &rp);
    positive_u_helper::<u32>(&eo, &ep, &rp);
    positive_u_helper::<u64>(&eo, &ep, &rp);
}

fn positive_i_helper<T: PrimSignedInt>(eo: &TestOutput,
                                       ep: &IteratorProvider,
                                       rp: &IteratorProvider) {
    eo.match_vec(&format!("exhaustive_positive_{}s", T::name()),
                 &mut ep.positive_i::<T>());
    eo.match_vec_f(&format!("random_positive_{}s", T::name()),
                   &mut rp.positive_i::<T>());
}

#[test]
fn test_positive_i() {
    let (eo, ep, rp) = prepare_test();
    positive_i_helper::<i8>(&eo, &ep, &rp);
    positive_i_helper::<i16>(&eo, &ep, &rp);
    positive_i_helper::<i32>(&eo, &ep, &rp);
    positive_i_helper::<i64>(&eo, &ep, &rp);
}

fn negative_i_helper<T: PrimSignedInt>(eo: &TestOutput,
                                       ep: &IteratorProvider,
                                       rp: &IteratorProvider) {
    eo.match_vec(&format!("exhaustive_negative_{}s", T::name()),
                 &mut ep.negative_i::<T>());
    eo.match_vec_f(&format!("random_negative_{}s", T::name()),
                   &mut rp.negative_i::<T>());
}

#[test]
fn test_negative_i() {
    let (eo, ep, rp) = prepare_test();
    negative_i_helper::<i8>(&eo, &ep, &rp);
    negative_i_helper::<i16>(&eo, &ep, &rp);
    negative_i_helper::<i32>(&eo, &ep, &rp);
    negative_i_helper::<i64>(&eo, &ep, &rp);
}

fn natural_i_helper<T: PrimSignedInt>(eo: &TestOutput,
                                      ep: &IteratorProvider,
                                      rp: &IteratorProvider) {
    eo.match_vec(&format!("exhaustive_natural_{}s", T::name()),
                 &mut ep.natural_i::<T>());
    eo.match_vec_f(&format!("random_natural_{}s", T::name()),
                   &mut rp.natural_i::<T>());
}

#[test]
fn test_natural_i() {
    let (eo, ep, rp) = prepare_test();
    natural_i_helper::<i8>(&eo, &ep, &rp);
    natural_i_helper::<i16>(&eo, &ep, &rp);
    natural_i_helper::<i32>(&eo, &ep, &rp);
    natural_i_helper::<i64>(&eo, &ep, &rp);
}

fn nonzero_i_helper<T: PrimSignedInt>(eo: &TestOutput,
                                      ep: &IteratorProvider,
                                      rp: &IteratorProvider) {
    eo.match_vec(&format!("exhaustive_nonzero_{}s", T::name()),
                 &mut ep.nonzero_i::<T>());
    eo.match_vec_f(&format!("random_nonzero_{}s", T::name()),
                   &mut rp.nonzero_i::<T>());
}

#[test]
fn test_nonzero_i() {
    let (eo, ep, rp) = prepare_test();
    nonzero_i_helper::<i8>(&eo, &ep, &rp);
    nonzero_i_helper::<i16>(&eo, &ep, &rp);
    nonzero_i_helper::<i32>(&eo, &ep, &rp);
    nonzero_i_helper::<i64>(&eo, &ep, &rp);
}

fn all_u_helper<T: PrimUnsignedInt>(eo: &TestOutput,
                                    ep: &IteratorProvider,
                                    rp: &IteratorProvider) {
    eo.match_vec(&format!("exhaustive_{}s", T::name()), &mut ep.all_u::<T>());
    eo.match_vec_f(&format!("random_{}s", T::name()), &mut rp.all_u::<T>());
}

#[test]
fn test_all_u() {
    let (eo, ep, rp) = prepare_test();
    all_u_helper::<u8>(&eo, &ep, &rp);
    all_u_helper::<u16>(&eo, &ep, &rp);
    all_u_helper::<u32>(&eo, &ep, &rp);
    all_u_helper::<u64>(&eo, &ep, &rp);
}

fn all_i_helper<T: PrimSignedInt>(eo: &TestOutput, ep: &IteratorProvider, rp: &IteratorProvider) {
    eo.match_vec(&format!("exhaustive_{}s", T::name()), &mut ep.all_i::<T>());
    eo.match_vec_f(&format!("random_{}s", T::name()), &mut rp.all_i::<T>());
}

#[test]
fn test_all_i() {
    let (eo, ep, rp) = prepare_test();
    all_i_helper::<i8>(&eo, &ep, &rp);
    all_i_helper::<i16>(&eo, &ep, &rp);
    all_i_helper::<i32>(&eo, &ep, &rp);
    all_i_helper::<i64>(&eo, &ep, &rp);
}

fn range_up_u_helper<T: PrimUnsignedInt>(eo: &TestOutput,
                                         ep: &IteratorProvider,
                                         rp: &IteratorProvider) {
    let e_test = |number, a| {
        eo.match_vec(&format!("exhaustive_range_up_{}_{}", T::name(), number),
                     &mut ep.range_up_u::<T>(a))
    };
    e_test("i", T::from_u8(0));
    e_test("ii", T::from_u8(5));
    e_test("iii", T::max_value());
    let r_test = |number, a| {
        eo.match_vec_f(&format!("random_range_up_{}_{}", T::name(), number),
                       &mut rp.range_up_u::<T>(a))
    };
    r_test("i", T::from_u8(0));
    r_test("ii", T::from_u8(5));
    r_test("iii", T::max_value());
}

#[test]
fn test_range_up_u() {
    let (eo, ep, rp) = prepare_test();
    range_up_u_helper::<u8>(&eo, &ep, &rp);
    range_up_u_helper::<u16>(&eo, &ep, &rp);
    range_up_u_helper::<u32>(&eo, &ep, &rp);
    range_up_u_helper::<u64>(&eo, &ep, &rp);
}

fn range_up_i_helper<T: PrimSignedInt>(eo: &TestOutput,
                                       ep: &IteratorProvider,
                                       rp: &IteratorProvider) {
    let e_test = |number, a| {
        eo.match_vec(&format!("exhaustive_range_up_{}_{}", T::name(), number),
                     &mut ep.range_up_i::<T>(a))
    };
    e_test("i", T::from_u8(0));
    e_test("ii", T::from_u8(5));
    e_test("iii", T::max_value());
    e_test("iv", T::from_i8(-5));
    e_test("v", T::min_value());
    let r_test = |number, a| {
        eo.match_vec_f(&format!("random_range_up_{}_{}", T::name(), number),
                       &mut rp.range_up_i::<T>(a))
    };
    r_test("i", T::from_u8(0));
    r_test("ii", T::from_u8(5));
    r_test("iii", T::max_value());
    r_test("iv", T::from_i8(-5));
    r_test("v", T::min_value());
}

#[test]
fn test_range_up_i() {
    let (eo, ep, rp) = prepare_test();
    range_up_i_helper::<i8>(&eo, &ep, &rp);
    range_up_i_helper::<i16>(&eo, &ep, &rp);
    range_up_i_helper::<i32>(&eo, &ep, &rp);
    range_up_i_helper::<i64>(&eo, &ep, &rp);
}

fn range_down_u_helper<T: PrimUnsignedInt>(eo: &TestOutput,
                                           ep: &IteratorProvider,
                                           rp: &IteratorProvider) {
    let e_test = |number, a| {
        eo.match_vec(&format!("exhaustive_range_down_{}_{}", T::name(), number),
                     &mut ep.range_down_u::<T>(a))
    };
    e_test("i", T::from_u8(0));
    e_test("ii", T::from_u8(5));
    e_test("iii", T::max_value());
    let r_test = |number, a| {
        eo.match_vec_f(&format!("random_range_down_{}_{}", T::name(), number),
                       &mut rp.range_down_u::<T>(a))
    };
    r_test("i", T::from_u8(0));
    r_test("ii", T::from_u8(5));
    r_test("iii", T::max_value());
}

#[test]
fn test_range_down_u() {
    let (eo, ep, rp) = prepare_test();
    range_down_u_helper::<u8>(&eo, &ep, &rp);
    range_down_u_helper::<u16>(&eo, &ep, &rp);
    range_down_u_helper::<u32>(&eo, &ep, &rp);
    range_down_u_helper::<u64>(&eo, &ep, &rp);
}

fn range_down_i_helper<T: PrimSignedInt>(eo: &TestOutput,
                                         ep: &IteratorProvider,
                                         rp: &IteratorProvider) {
    let e_test = |number, a| {
        eo.match_vec(&format!("exhaustive_range_down_{}_{}", T::name(), number),
                     &mut ep.range_down_i::<T>(a))
    };
    e_test("i", T::from_u8(0));
    e_test("ii", T::from_u8(5));
    e_test("iii", T::max_value());
    e_test("iv", T::from_i8(-5));
    e_test("v", T::min_value());
    let r_test = |number, a| {
        eo.match_vec_f(&format!("random_range_down_{}_{}", T::name(), number),
                       &mut rp.range_down_i::<T>(a))
    };
    r_test("i", T::from_u8(0));
    r_test("ii", T::from_u8(5));
    r_test("iii", T::max_value());
    r_test("iv", T::from_i8(-5));
    r_test("v", T::min_value());
}

#[test]
fn test_range_down_i() {
    let (eo, ep, rp) = prepare_test();
    range_down_i_helper::<i8>(&eo, &ep, &rp);
    range_down_i_helper::<i16>(&eo, &ep, &rp);
    range_down_i_helper::<i32>(&eo, &ep, &rp);
    range_down_i_helper::<i64>(&eo, &ep, &rp);
}

fn range_u_helper<T: PrimUnsignedInt>(eo: &TestOutput,
                                      ep: &IteratorProvider,
                                      rp: &IteratorProvider) {
    let e_test = |number, a, b| {
        eo.match_vec(&format!("exhaustive_range_{}_{}", T::name(), number),
                     &mut ep.range_u::<T>(a, b))
    };
    e_test("i", T::from_u8(0), T::from_u8(0));
    e_test("ii", T::from_u8(0), T::from_u8(10));
    e_test("iii", T::from_u8(10), T::from_u8(20));
    e_test("iv", T::from_u8(10), T::from_u8(10));
    e_test("v", T::from_u8(0), T::max_value());
    e_test("vi", T::from_u8(0), T::max_value() - T::from_u8(1));
    let r_test = |number, a, b| {
        eo.match_vec_f(&format!("random_range_{}_{}", T::name(), number),
                       &mut rp.range_u::<T>(a, b))
    };
    r_test("i", T::from_u8(0), T::from_u8(0));
    r_test("ii", T::from_u8(0), T::from_u8(10));
    r_test("iii", T::from_u8(10), T::from_u8(20));
    r_test("iv", T::from_u8(10), T::from_u8(10));
    r_test("v", T::from_u8(0), T::max_value());
    r_test("vi", T::from_u8(0), T::max_value() - T::from_u8(1));
}

#[test]
fn test_range_u() {
    let (eo, ep, rp) = prepare_test();
    range_u_helper::<u8>(&eo, &ep, &rp);
    range_u_helper::<u16>(&eo, &ep, &rp);
    range_u_helper::<u32>(&eo, &ep, &rp);
    range_u_helper::<u64>(&eo, &ep, &rp);
}

fn range_i_helper<T: PrimSignedInt>(eo: &TestOutput,
                                    ep: &IteratorProvider,
                                    rp: &IteratorProvider) {
    let e_test = |number, a, b| {
        eo.match_vec(&format!("exhaustive_range_{}_{}", T::name(), number),
                     &mut ep.range_i::<T>(a, b))
    };
    e_test("i", T::from_u8(0), T::from_u8(0));
    e_test("ii", T::from_u8(0), T::from_u8(10));
    e_test("iii", T::from_u8(10), T::from_u8(20));
    e_test("iv", T::from_u8(10), T::from_u8(10));
    e_test("v", T::from_u8(0), T::max_value());
    e_test("vi", T::from_u8(0), T::max_value() - T::from_u8(1));
    e_test("vii", T::from_i8(-10), T::from_i8(-10));
    e_test("viii", T::from_i8(-20), T::from_i8(-10));
    e_test("ix", T::from_i8(-100), T::from_u8(100));
    e_test("x", T::min_value(), T::max_value());
    e_test("xi",
           T::min_value() + T::from_u8(1),
           T::max_value() - T::from_u8(1));
    let r_test = |number, a, b| {
        eo.match_vec_f(&format!("random_range_{}_{}", T::name(), number),
                       &mut rp.range_i::<T>(a, b))
    };
    r_test("i", T::from_u8(0), T::from_u8(0));
    r_test("ii", T::from_u8(0), T::from_u8(10));
    r_test("iii", T::from_u8(10), T::from_u8(20));
    r_test("iv", T::from_u8(10), T::from_u8(10));
    r_test("v", T::from_u8(0), T::max_value());
    r_test("vi", T::from_u8(0), T::max_value() - T::from_u8(1));
    r_test("vii", T::from_i8(-10), T::from_i8(-10));
    r_test("viii", T::from_i8(-20), T::from_i8(-10));
    r_test("ix", T::from_i8(-100), T::from_u8(100));
    r_test("x", T::min_value(), T::max_value());
    r_test("xi",
           T::min_value() + T::from_u8(1),
           T::max_value() - T::from_u8(1));
}

#[test]
fn test_range_i() {
    let (eo, ep, rp) = prepare_test();
    range_i_helper::<i8>(&eo, &ep, &rp);
    range_i_helper::<i16>(&eo, &ep, &rp);
    range_i_helper::<i32>(&eo, &ep, &rp);
    range_i_helper::<i64>(&eo, &ep, &rp);
}

#[test]
fn test_exhaustive_generate_from_vector() {
    let eo = get_expected_test_outputs();
    let e_test = |number, xs| {
        let xs: Vec<u32> = parse_vec(xs).unwrap();
        eo.match_vec(&format!("exhaustive_exhaustive_generate_from_vector_{}", number),
                     &mut exhaustive_from_vector(xs))
    };
    e_test("i", "[]");
    e_test("ii", "[5]");
    e_test("iii", "[1, 2, 3]");
    e_test("iv", "[3, 1, 4, 1]");
}

#[test]
fn test_generate_from_vector() {
    let (eo, ep, rp) = prepare_test();
    let e_test = |number, xs| {
        let xs: Vec<u32> = parse_vec(xs).unwrap();
        eo.match_vec(&format!("exhaustive_generate_from_vector_{}", number),
                     &mut ep.generate_from_vector(xs));
    };
    e_test("i", "[]");
    e_test("ii", "[5]");
    e_test("iii", "[1, 2, 3]");
    e_test("iv", "[3, 1, 4, 1]");
    let r_test = |number, xs| {
        let xs: Vec<u32> = parse_vec(xs).unwrap();
        eo.match_vec(&format!("random_generate_from_vector_{}", number),
                     &mut rp.generate_from_vector(xs));
    };
    r_test("i", "[5]");
    r_test("ii", "[1, 2, 3]");
    r_test("iii", "[3, 1, 4, 1]");
}

#[test]
#[should_panic(expected = "Cannot randomly generate values from an empty Vec.")]
fn generate_from_vector_fail() {
    &mut IteratorProvider::example_random().generate_from_vector(Vec::<u32>::new());
}

#[test]
fn test_chars_increasing() {
    let (eo, p, _) = prepare_test();
    eo.match_vec_debug("exhaustive_chars_increasing", &mut p.chars_increasing());
}

#[test]
fn test_chars_decreasing() {
    let (eo, p, _) = prepare_test();
    eo.match_vec_debug("exhaustive_chars_decreasing", &mut p.chars_decreasing());
}

#[test]
fn test_ascii_chars_increasing() {
    let (eo, p, _) = prepare_test();
    eo.match_vec_debug("exhaustive_ascii_chars_increasing",
                       &mut p.ascii_chars_increasing());
}

#[test]
fn test_ascii_chars_decreasing() {
    let (eo, p, _) = prepare_test();
    eo.match_vec_debug("exhaustive_ascii_chars_decreasing",
                       &mut p.ascii_chars_decreasing());
}

#[test]
fn test_range_up_increasing_char() {
    let (eo, p, _) = prepare_test();
    let test = |number, a| {
        eo.match_vec_debug(&format!("exhaustive_range_up_increasing_char_{}", number),
                           &mut p.range_up_increasing_char(a))
    };
    test("i", '\0');
    test("ii", 'a');
    test("iii", 'Ш');
    test("iv", '\u{D7FF}');
    test("v", char::MAX);
}

#[test]
fn test_range_up_decreasing_char() {
    let (eo, p, _) = prepare_test();
    let test = |number, a| {
        eo.match_vec_debug(&format!("exhaustive_range_up_decreasing_char_{}", number),
                           &mut p.range_up_decreasing_char(a))
    };
    test("i", '\0');
    test("ii", 'a');
    test("iii", 'Ш');
    test("iv", '\u{D7FF}');
    test("v", char::MAX);
}

#[test]
fn test_range_down_increasing_char() {
    let (eo, p, _) = prepare_test();
    let test = |number, a| {
        eo.match_vec_debug(&format!("exhaustive_range_down_increasing_char_{}", number),
                           &mut p.range_down_increasing_char(a))
    };
    test("i", '\0');
    test("ii", 'a');
    test("iii", 'Ш');
    test("iv", '\u{E000}');
    test("v", char::MAX);
}

#[test]
fn test_range_down_decreasing_char() {
    let (eo, p, _) = prepare_test();
    let test = |number, a| {
        eo.match_vec_debug(&format!("exhaustive_range_down_decreasing_char_{}", number),
                           &mut p.range_down_decreasing_char(a))
    };
    test("i", '\0');
    test("ii", 'a');
    test("iii", 'Ш');
    test("iv", '\u{E000}');
    test("v", char::MAX);
}

#[test]
fn test_range_increasing_char() {
    let (eo, p, _) = prepare_test();
    let test = |number, a, b| {
        eo.match_vec_debug(&format!("exhaustive_range_increasing_char_{}", number),
                           &mut p.range_increasing_char(a, b))
    };
    test("i", 'a', 'z');
    test("ii", 'a', 'a');
    test("iii", '!', '9');
    test("iv", '\u{D7FF}', '\u{E000}');
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: a, b: A")]
fn range_increasing_char_fail() {
    IteratorProvider::Exhaustive.range_increasing_char('a', 'A');
}

#[test]
fn test_range_decreasing_char() {
    let (eo, p, _) = prepare_test();
    let test = |number, a, b| {
        eo.match_vec_debug(&format!("exhaustive_range_decreasing_char_{}", number),
                           &mut p.range_decreasing_char(a, b))
    };
    test("i", 'a', 'z');
    test("ii", 'a', 'a');
    test("iii", '!', '9');
    test("iv", '\u{D7FF}', '\u{E000}');
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: a, b: A")]
fn range_decreasing_char_fail() {
    IteratorProvider::Exhaustive.range_decreasing_char('a', 'A');
}

#[test]
fn test_chars() {
    let (eo, ep, rp) = prepare_test();
    eo.match_vec_debug("exhaustive_chars", &mut ep.chars());
    eo.match_vec_f_debug("random_chars", &mut rp.chars());
}

#[test]
fn test_ascii_chars() {
    let (eo, ep, rp) = prepare_test();
    eo.match_vec_debug("exhaustive_ascii_chars", &mut ep.ascii_chars());
    eo.match_vec_f_debug("random_ascii_chars", &mut rp.ascii_chars());
}

#[test]
fn test_range_up_char() {
    let (eo, ep, rp) = prepare_test();
    let e_test = |number, a| {
        eo.match_vec_debug(&format!("exhaustive_range_up_char_{}", number),
                           &mut ep.range_up_char(a))
    };
    e_test("i", '\0');
    e_test("ii", 'a');
    e_test("iii", 'Ш');
    e_test("iv", '\u{D7FF}');
    e_test("v", char::MAX);
    let r_test = |number, a| {
        eo.match_vec_f_debug(&format!("random_range_up_char_{}", number),
                             &mut rp.range_up_char(a))
    };
    r_test("i", '\0');
    r_test("ii", 'a');
    r_test("iii", 'Ш');
    r_test("iv", '\u{D7FF}');
    r_test("v", char::MAX);
}

#[test]
fn test_range_down_char() {
    let (eo, ep, rp) = prepare_test();
    let e_test = |number, a| {
        eo.match_vec_debug(&format!("exhaustive_range_down_char_{}", number),
                           &mut ep.range_down_char(a))
    };
    e_test("i", '\0');
    e_test("ii", 'a');
    e_test("iii", 'Ш');
    e_test("iv", '\u{E000}');
    e_test("v", char::MAX);
    let r_test = |number, a| {
        eo.match_vec_f_debug(&format!("random_range_down_char_{}", number),
                             &mut rp.range_down_char(a))
    };
    r_test("i", '\0');
    r_test("ii", 'a');
    r_test("iii", 'Ш');
    r_test("iv", '\u{E000}');
    r_test("v", char::MAX);
}

#[test]
fn test_range_char() {
    let (eo, ep, rp) = prepare_test();
    let e_test = |number, a, b| {
        eo.match_vec_debug(&format!("exhaustive_range_char_{}", number),
                           &mut ep.range_char(a, b))
    };
    e_test("i", 'a', 'z');
    e_test("ii", 'a', 'a');
    e_test("iii", '!', '9');
    e_test("iv", '\u{D7FF}', '\u{E000}');
    let r_test = |number, a, b| {
        eo.match_vec_f_debug(&format!("random_range_char_{}", number),
                             &mut rp.range_char(a, b))
    };
    r_test("i", 'a', 'z');
    r_test("ii", 'a', 'a');
    r_test("iii", '!', '9');
    r_test("iv", '\u{D7FF}', '\u{E000}');
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: a, b: A")]
fn range_char_fail() {
    IteratorProvider::Exhaustive.range_char('a', 'A');
}

#[test]
fn test_range_up_increasing_integer() {
    let (eo, p, _) = prepare_test();
    let test = |number, a| {
        eo.match_vec(&format!("exhaustive_range_up_increasing_integer_{}", number),
                     &mut p.range_up_increasing_integer(Integer::from_str(a).unwrap()))
    };
    test("i", "0");
    test("ii", "5");
    test("iii", "-5");
}

#[test]
fn test_range_down_decreasing_integer() {
    let (eo, p, _) = prepare_test();
    let test = |number, a| {
        eo.match_vec(&format!("exhaustive_range_down_decreasing_integer_{}", number),
                     &mut p.range_down_decreasing_integer(Integer::from_str(a).unwrap()))
    };
    test("i", "0");
    test("ii", "5");
    test("iii", "-5");
}

#[test]
fn test_range_increasing_integer() {
    let (eo, p, _) = prepare_test();
    let test = |number, a, b| {
        eo.match_vec(&format!("exhaustive_range_increasing_integer_{}", number),
                     &mut p.range_increasing_integer(Integer::from_str(a).unwrap(),
                                                     Integer::from_str(b).unwrap()))
    };
    test("i", "0", "0");
    test("ii", "0", "10");
    test("iii", "10", "20");
    test("iv", "10", "10");
    test("v", "-10", "-10");
    test("vi", "-20", "-10");
    test("vii", "-100", "100");
}

fn range_increasing_integer_fail_helper(p: &IteratorProvider, a: &str, b: &str) {
    p.range_increasing_integer(Integer::from_str(a).unwrap(), Integer::from_str(b).unwrap());
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
fn range_increasing_integer_fail_1() {
    range_increasing_integer_fail_helper(&IteratorProvider::Exhaustive, "10", "9")
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
fn range_increasing_integer_fail_2() {
    range_increasing_integer_fail_helper(&IteratorProvider::Exhaustive, "-9", "-10")
}

#[test]
fn test_range_decreasing_integer() {
    let (eo, p, _) = prepare_test();
    let test = |number, a, b| {
        eo.match_vec(&format!("exhaustive_range_decreasing_integer_{}", number),
                     &mut p.range_decreasing_integer(Integer::from_str(a).unwrap(),
                                                     Integer::from_str(b).unwrap()))
    };
    test("i", "0", "0");
    test("ii", "0", "10");
    test("iii", "10", "20");
    test("iv", "10", "10");
    test("v", "-10", "-10");
    test("vi", "-20", "-10");
    test("vii", "-100", "100");
}

fn range_decreasing_integer_fail_helper(p: &IteratorProvider, a: &str, b: &str) {
    p.range_decreasing_integer(Integer::from_str(a).unwrap(), Integer::from_str(b).unwrap());
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
fn range_decreasing_integer_fail_1() {
    range_decreasing_integer_fail_helper(&IteratorProvider::Exhaustive, "10", "9")
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
fn range_decreasing_integer_fail_2() {
    let p = IteratorProvider::Exhaustive;
    range_decreasing_integer_fail_helper(&p, "-9", "-10")
}

#[test]
fn test_range_integer() {
    let (eo, ep, rp) = prepare_test();
    let e_test = |number, a, b| {
        eo.match_vec(&format!("exhaustive_range_integer_{}", number),
                     &mut ep.range_integer(Integer::from_str(a).unwrap(),
                                           Integer::from_str(b).unwrap()))
    };
    e_test("i", "0", "0");
    e_test("ii", "0", "10");
    e_test("iii", "10", "20");
    e_test("iv", "10", "10");
    e_test("v", "-10", "-10");
    e_test("vi", "-20", "-10");
    e_test("vii", "-100", "100");
    let r_test = |number, a, b| {
        eo.match_vec_f(&format!("random_range_integer_{}", number),
                       &mut rp.range_integer(Integer::from_str(a).unwrap(),
                                             Integer::from_str(b).unwrap()))
    };
    r_test("i", "0", "0");
    r_test("ii", "0", "10");
    r_test("iii", "10", "20");
    r_test("iv", "10", "10");
    r_test("v", "-10", "-10");
    r_test("vi", "-20", "-10");
    r_test("vii", "-100", "100");
}

fn range_integer_fail_helper(p: &IteratorProvider, a: &str, b: &str) {
    p.range_integer(Integer::from_str(a).unwrap(), Integer::from_str(b).unwrap());
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
fn range_integer_fail_1() {
    range_integer_fail_helper(&IteratorProvider::Exhaustive, "10", "9")
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
fn range_integer_fail_2() {
    range_integer_fail_helper(&IteratorProvider::Exhaustive, "-9", "-10")
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
fn range_integer_fail_3() {
    range_integer_fail_helper(&IteratorProvider::example_random(), "10", "9")
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
fn range_integer_fail_4() {
    range_integer_fail_helper(&IteratorProvider::example_random(), "-9", "-10")
}

#[test]
fn test_orderings_increasing() {
    let (eo, p, _) = prepare_test();
    eo.match_vec_debug("exhaustive_orderings_increasing",
                       &mut p.orderings_increasing());
}

#[test]
fn test_orderings() {
    let (eo, ep, rp) = prepare_test();
    eo.match_vec_debug("exhaustive_orderings", &mut ep.orderings());
    eo.match_vec_f_debug("random_orderings", &mut rp.orderings());
}

#[test]
fn test_positive_u32s_geometric() {
    let (eo, _, p) = prepare_test();
    let test = |number, scale| {
        eo.match_vec_f(&format!("random_positive_u32s_geometric_{}", number),
                       &mut p.positive_u32s_geometric(scale))
    };
    test("i", 0);
    test("ii", 1);
    test("iii", 2);
    test("iv", 3);
    test("v", 10);
    test("vi", 100);
}

#[test]
fn test_natural_u32s_geometric() {
    let (eo, _, p) = prepare_test();
    let test = |number, scale| {
        eo.match_vec_f(&format!("random_natural_u32s_geometric_{}", number),
                       &mut p.natural_u32s_geometric(scale))
    };
    test("i", 0);
    test("ii", 1);
    test("iii", 2);
    test("iv", 3);
    test("v", 10);
    test("vi", 100);
}

#[test]
fn test_negative_i32s_geometric() {
    let (eo, _, p) = prepare_test();
    let test = |number, scale| {
        eo.match_vec_f(&format!("random_negative_i32s_geometric_{}", number),
                       &mut p.negative_i32s_geometric(scale))
    };
    test("i", 0);
    test("ii", 1);
    test("iii", 2);
    test("iv", 3);
    test("v", 10);
    test("vi", 100);
}
/*
#[test]
fn test_nonzero_i32s_geometric() {
    let (eo, _, p) = prepare_test();
    let test = |number, scale| {
        eo.match_vec_f(&format!("random_nonzero_i32s_geometric_{}", number),
                       &mut p.nonzero_i32s_geometric(scale))
    };
    test("i", 0);
    test("ii", 1);
    test("iii", 2);
    test("iv", 3);
    test("v", 10);
    test("vi", 100);
}

#[test]
fn test_i32s_geometric() {
    let (eo, _, p) = prepare_test();
    let test = |number, scale| {
        eo.match_vec_f(&format!("random_i32s_geometric_{}", number),
                       &mut p.i32s_geometric(scale))
    };
    test("i", 0);
    test("ii", 1);
    test("iii", 2);
    test("iv", 3);
    test("v", 10);
    test("vi", 100);
}
*/
