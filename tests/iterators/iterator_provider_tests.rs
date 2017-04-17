use common;
use common::TestOutput;
use gmp_to_flint_adaptor_lib::integer::Integer;
use rust_wheels_lib::io::readers::parse_vec;
use rust_wheels_lib::iterators::iterator_provider::IteratorProvider;
use rust_wheels_lib::prim_utils::traits::*;
use std::char;
use std::str::FromStr;

fn prepare_test() -> (TestOutput, IteratorProvider, IteratorProvider) {
    (common::get_expected_test_outputs(),
     IteratorProvider::Exhaustive,
     IteratorProvider::example_random())
}

#[test]
fn test_bools() {
    let (eo, ep, rp) = prepare_test();
    eo.match_vec("exhaustive_bools", &mut ep.bools());
    eo.match_vec_f("random_bools", &mut rp.bools());
}

fn range_up_increasing_u_helper<T: PrimUnsignedInt>(eo: &TestOutput, p: &IteratorProvider) {
    let s = "exhaustive_range_up_increasing";
    let test = |number, a| {
        eo.match_vec(&format!("{}_{}_{}", s, T::name(), number),
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
    let s = "exhaustive_range_up_increasing";
    let test = |number, a| {
        eo.match_vec(&format!("{}_{}_{}", s, T::name(), number),
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
    let s = "exhaustive_range_up_decreasing";
    let test = |number, a| {
        eo.match_vec(&format!("{}_{}_{}", s, T::name(), number),
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
    let s = "exhaustive_range_up_decreasing";
    let test = |number, a| {
        eo.match_vec(&format!("{}_{}_{}", s, T::name(), number),
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
    let s = "exhaustive_range_down_increasing";
    let test = |number, a| {
        eo.match_vec(&format!("{}_{}_{}", s, T::name(), number),
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
    let s = "exhaustive_range_down_increasing";
    let test = |number, a| {
        eo.match_vec(&format!("{}_{}_{}", s, T::name(), number),
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
    let s = "exhaustive_range_down_decreasing";
    let test = |number, a| {
        eo.match_vec(&format!("{}_{}_{}", s, T::name(), number),
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
    let s = "exhaustive_range_down_decreasing";
    let test = |number, a| {
        eo.match_vec(&format!("{}_{}_{}", s, T::name(), number),
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

macro_rules! test_integer_range {
    (
        $t: ty,
        $ts: expr,
        $rup: ident,
        $rdn: ident,
        $r: ident,
        $rui_th: ident,
        $rud_th: ident,
        $rdi_th: ident,
        $rdd_th: ident,
        $ri_th: ident,
        $rd_th: ident,
        $rup_eth: ident,
        $rup_rth: ident,
        $rdn_eth: ident,
        $rdn_rth: ident,
        $r_eth: ident,
        $r_rth: ident,
        $rui_t: ident,
        $rud_t: ident,
        $rdi_t: ident,
        $rdd_t: ident,
        $ri_t: ident,
        $rd_t: ident,
        $i_t: ident,
        $d_t: ident,
        $pos_t: ident,
        $all_t: ident,
        $rup_t: ident,
        $rdn_t: ident,
        $r_t: ident,
        $ri_f: ident,
        $rd_f: ident,
        $max: expr
    ) => {
        fn $ri_th(eo: &TestOutput, p: &IteratorProvider, key: &str, a: $t, b: $t) {
            eo.match_vec(key, &mut p.range_increasing_x(a, b));
        }

        #[test]
        fn $ri_t() {
            let (eo, p, _) = prepare_test();
            let s = "exhaustive_range_increasing";
            $ri_th(&eo, &p, &format!("{}_{}_i", s, $ts), 0, 0);
            $ri_th(&eo, &p, &format!("{}_{}_ii", s, $ts), 0, 10);
            $ri_th(&eo, &p, &format!("{}_{}_iii", s, $ts), 10, 20);
            $ri_th(&eo, &p, &format!("{}_{}_iv", s, $ts), 10, 10);
            $ri_th(&eo, &p, &format!("{}_{}_v", s, $ts), 0, $max);
            $ri_th(&eo, &p, &format!("{}_{}_vi", s, $ts), 0, $max - 1);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $ri_f() {
            IteratorProvider::Exhaustive.range_increasing_x(10, 9);
        }

        fn $rd_th(eo: &TestOutput, p: &IteratorProvider, key: &str, a: $t, b: $t) {
            eo.match_vec(key, &mut p.range_decreasing_x(a, b));
        }

        #[test]
        fn $rd_t() {
            let (eo, p, _) = prepare_test();
            let s = "exhaustive_range_decreasing";
            $rd_th(&eo, &p, &format!("{}_{}_i", s, $ts), 0, 0);
            $rd_th(&eo, &p, &format!("{}_{}_ii", s, $ts), 0, 10);
            $rd_th(&eo, &p, &format!("{}_{}_iii", s, $ts), 10, 20);
            $rd_th(&eo, &p, &format!("{}_{}_iv", s, $ts), 10, 10);
            $rd_th(&eo, &p, &format!("{}_{}_v", s, $ts), 0, $max);
            $rd_th(&eo, &p, &format!("{}_{}_vi", s, $ts), 0, $max - 1);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $rd_f() {
            IteratorProvider::Exhaustive.range_decreasing_x(10, 9);
        }

        #[test]
        fn $i_t() {
            let (eo, p, _) = prepare_test();
            eo.match_vec(&format!("exhaustive_{}s_increasing", $ts), &mut p.x_increasing::<$t>());
        }

        #[test]
        fn $d_t() {
            let (eo, p, _) = prepare_test();
            eo.match_vec(&format!("exhaustive_{}s_decreasing", $ts), &mut p.x_decreasing::<$t>());
        }

        fn $rup_eth(eo: &TestOutput, p: &IteratorProvider, key: &str, a: $t) {
            eo.match_vec(key, &mut p.$rup(a));
        }

        fn $rup_rth(eo: &TestOutput, p: &IteratorProvider, key: &str, a: $t) {
            eo.match_vec_f(key, &mut p.$rup(a));
        }

        #[test]
        fn $rup_t() {
            let (eo, ep, rp) = prepare_test();
            let es = "exhaustive_range_up";
            $rup_eth(&eo, &ep, &format!("{}_{}_i", es, $ts), 0);
            $rup_eth(&eo, &ep, &format!("{}_{}_ii", es, $ts), 5);
            $rup_eth(&eo, &ep, &format!("{}_{}_iii", es, $ts), $max);
            let rs = "random_range_up";
            $rup_rth(&eo, &rp, &format!("{}_{}_i", rs, $ts), 0);
            $rup_rth(&eo, &rp, &format!("{}_{}_ii", rs, $ts), 5);
            $rup_rth(&eo, &rp, &format!("{}_{}_iii", rs, $ts), $max);
        }

        fn $rdn_eth(eo: &TestOutput, p: &IteratorProvider, key: &str, a: $t) {
            eo.match_vec(key, &mut p.$rdn(a));
        }

        fn $rdn_rth(eo: &TestOutput, p: &IteratorProvider, key: &str, a: $t) {
            eo.match_vec_f(key, &mut p.$rdn(a));
        }

        #[test]
        fn $rdn_t() {
            let (eo, ep, rp) = prepare_test();
            let es = "exhaustive_range_down";
            $rdn_eth(&eo, &ep, &format!("{}_{}_i", es, $ts), 0);
            $rdn_eth(&eo, &ep, &format!("{}_{}_ii", es, $ts), 5);
            $rdn_eth(&eo, &ep, &format!("{}_{}_iii", es, $ts), $max);
            let rs = "random_range_down";
            $rdn_rth(&eo, &rp, &format!("{}_{}_i", rs, $ts), 0);
            $rdn_rth(&eo, &rp, &format!("{}_{}_ii", rs, $ts), 5);
            $rdn_rth(&eo, &rp, &format!("{}_{}_iii", rs, $ts), $max);
        }

        fn $r_eth(eo: &TestOutput, p: &IteratorProvider, key: &str, a: $t, b: $t) {
            eo.match_vec(key, &mut p.$r(a, b));
        }

        fn $r_rth(eo: &TestOutput, p: &IteratorProvider, key: &str, a: $t, b: $t) {
            eo.match_vec_f(key, &mut p.$r(a, b));
        }

        #[test]
        fn $r_t() {
            let (eo, ep, rp) = prepare_test();
            let es = "exhaustive_range";
            $r_eth(&eo, &ep, &format!("{}_{}_i", es, $ts), 0, 0);
            $r_eth(&eo, &ep, &format!("{}_{}_ii", es, $ts), 0, 10);
            $r_eth(&eo, &ep, &format!("{}_{}_iii", es, $ts), 10, 20);
            $r_eth(&eo, &ep, &format!("{}_{}_iv", es, $ts), 10, 10);
            $r_eth(&eo, &ep, &format!("{}_{}_v", es, $ts), 0, $max);
            $r_eth(&eo, &ep, &format!("{}_{}_vi", es, $ts), 0, $max - 1);
            let rs = "random_range";
            $r_rth(&eo, &rp, &format!("{}_{}_i", rs, $ts), 0, 0);
            $r_rth(&eo, &rp, &format!("{}_{}_ii", rs, $ts), 0, 10);
            $r_rth(&eo, &rp, &format!("{}_{}_iii", rs, $ts), 10, 20);
            $r_rth(&eo, &rp, &format!("{}_{}_iv", rs, $ts), 10, 10);
            $r_rth(&eo, &rp, &format!("{}_{}_v", rs, $ts), 0, $max);
            $r_rth(&eo, &rp, &format!("{}_{}_vi", rs, $ts), 0, $max - 1);
        }
    }
}

test_integer_range!(u8,
                    "u8",
                    range_up_u,
                    range_down_u,
                    range_u,
                    range_up_increasing_u8_helper,
                    range_up_decreasing_u8_helper,
                    range_down_increasing_u8_helper,
                    range_down_decreasing_u8_helper,
                    range_increasing_u8_helper,
                    range_decreasing_u8_helper,
                    range_up_exhaustive_u8_helper,
                    range_up_random_u8_helper,
                    range_down_exhaustive_u8_helper,
                    range_down_random_u8_helper,
                    range_exhaustive_u8_helper,
                    range_random_u8_helper,
                    test_range_up_increasing_u8,
                    test_range_up_decreasing_u8,
                    test_range_down_increasing_u8,
                    test_range_down_decreasing_u8,
                    test_range_increasing_u8,
                    test_range_decreasing_u8,
                    test_u8s_increasing,
                    test_u8s_decreasing,
                    test_positive_u8s,
                    test_u8s,
                    test_range_up_u8,
                    test_range_down_u8,
                    test_range_u8,
                    range_increasing_u8_fail,
                    range_decreasing_u8_fail,
                    u8::max_value());
test_integer_range!(u16,
                    "u16",
                    range_up_u,
                    range_down_u,
                    range_u,
                    range_up_increasing_u16_helper,
                    range_up_decreasing_u16_helper,
                    range_down_increasing_u16_helper,
                    range_down_decreasing_u16_helper,
                    range_increasing_u16_helper,
                    range_decreasing_u16_helper,
                    range_up_exhaustive_u16_helper,
                    range_up_random_u16_helper,
                    range_down_exhaustive_u16_helper,
                    range_down_random_u16_helper,
                    range_exhaustive_u16_helper,
                    range_random_u16_helper,
                    test_range_up_increasing_u16,
                    test_range_up_decreasing_u16,
                    test_range_down_increasing_u16,
                    test_range_down_decreasing_u16,
                    test_range_increasing_u16,
                    test_range_decreasing_u16,
                    test_u16s_increasing,
                    test_u16s_decreasing,
                    test_positive_u16s,
                    test_u16s,
                    test_range_up_u16,
                    test_range_down_u16,
                    test_range_u16,
                    range_increasing_u16_fail,
                    range_decreasing_u16_fail,
                    u16::max_value());
test_integer_range!(u32,
                    "u32",
                    range_up_u,
                    range_down_u,
                    range_u,
                    range_up_increasing_u32_helper,
                    range_up_decreasing_u32_helper,
                    range_down_increasing_u32_helper,
                    range_down_decreasing_u32_helper,
                    range_increasing_u32_helper,
                    range_decreasing_u32_helper,
                    range_up_exhaustive_u32_helper,
                    range_up_random_u32_helper,
                    range_down_exhaustive_u32_helper,
                    range_down_random_u32_helper,
                    range_exhaustive_u32_helper,
                    range_random_u32_helper,
                    test_range_up_increasing_u32,
                    test_range_up_decreasing_u32,
                    test_range_down_increasing_u32,
                    test_range_down_decreasing_u32,
                    test_range_increasing_u32,
                    test_range_decreasing_u32,
                    test_u32s_increasing,
                    test_u32s_decreasing,
                    test_positive_u32s,
                    test_u32s,
                    test_range_up_u32,
                    test_range_down_u32,
                    test_range_u32,
                    range_increasing_u32_fail,
                    range_decreasing_u32_fail,
                    u32::max_value());
test_integer_range!(u64,
                    "u64",
                    range_up_u,
                    range_down_u,
                    range_u,
                    range_up_increasing_u64_helper,
                    range_up_decreasing_u64_helper,
                    range_down_increasing_u64_helper,
                    range_down_decreasing_u64_helper,
                    range_increasing_u64_helper,
                    range_decreasing_u64_helper,
                    range_up_exhaustive_u64_helper,
                    range_up_random_u64_helper,
                    range_down_exhaustive_u64_helper,
                    range_down_random_u64_helper,
                    range_exhaustive_u64_helper,
                    range_random_u64_helper,
                    test_range_up_increasing_u64,
                    test_range_up_decreasing_u64,
                    test_range_down_increasing_u64,
                    test_range_down_decreasing_u64,
                    test_range_increasing_u64,
                    test_range_decreasing_u64,
                    test_u64s_increasing,
                    test_u64s_decreasing,
                    test_positive_u64s,
                    test_u64s,
                    test_range_up_u64,
                    test_range_down_u64,
                    test_range_u64,
                    range_increasing_u64_fail,
                    range_decreasing_u64_fail,
                    u64::max_value());
test_integer_range!(i8,
                    "i8",
                    range_up_i,
                    range_down_i,
                    range_i,
                    range_up_increasing_i8_helper,
                    range_up_decreasing_i8_helper,
                    range_down_increasing_i8_helper,
                    range_down_decreasing_i8_helper,
                    range_increasing_i8_helper,
                    range_decreasing_i8_helper,
                    range_up_exhaustive_i8_helper,
                    range_up_random_i8_helper,
                    range_down_exhaustive_i8_helper,
                    range_down_random_i8_helper,
                    range_exhaustive_i8_helper,
                    range_random_i8_helper,
                    test_range_up_increasing_i8,
                    test_range_up_decreasing_i8,
                    test_range_down_increasing_i8,
                    test_range_down_decreasing_i8,
                    test_range_increasing_i8,
                    test_range_decreasing_i8,
                    test_i8s_increasing,
                    test_i8s_decreasing,
                    test_positive_i8s,
                    test_i8s,
                    test_range_up_i8,
                    test_range_down_i8,
                    test_range_i8,
                    range_increasing_i8_fail,
                    range_decreasing_i8_fail,
                    i8::max_value());
test_integer_range!(i16,
                    "i16",
                    range_up_i,
                    range_down_i,
                    range_i,
                    range_up_increasing_i16_helper,
                    range_up_decreasing_i16_helper,
                    range_down_increasing_i16_helper,
                    range_down_decreasing_i16_helper,
                    range_increasing_i16_helper,
                    range_decreasing_i16_helper,
                    range_up_exhaustive_i16_helper,
                    range_up_random_i16_helper,
                    range_down_exhaustive_i16_helper,
                    range_down_random_i16_helper,
                    range_exhaustive_i16_helper,
                    range_random_i16_helper,
                    test_range_up_increasing_i16,
                    test_range_up_decreasing_i16,
                    test_range_down_increasing_i16,
                    test_range_down_decreasing_i16,
                    test_range_increasing_i16,
                    test_range_decreasing_i16,
                    test_i16s_increasing,
                    test_i16s_decreasing,
                    test_positive_i16s,
                    test_i16s,
                    test_range_up_i16,
                    test_range_down_i16,
                    test_range_i16,
                    range_increasing_i16_fail,
                    range_decreasing_i16_fail,
                    i16::max_value());
test_integer_range!(i32,
                    "i32",
                    range_up_i,
                    range_down_i,
                    range_i,
                    range_up_increasing_i32_helper,
                    range_up_decreasing_i32_helper,
                    range_down_increasing_i32_helper,
                    range_down_decreasing_i32_helper,
                    range_increasing_i32_helper,
                    range_decreasing_i32_helper,
                    range_up_exhaustive_i32_helper,
                    range_up_random_i32_helper,
                    range_down_exhaustive_i32_helper,
                    range_down_random_i32_helper,
                    range_exhaustive_i32_helper,
                    range_random_i32_helper,
                    test_range_up_increasing_i32,
                    test_range_up_decreasing_i32,
                    test_range_down_increasing_i32,
                    test_range_down_decreasing_i32,
                    test_range_increasing_i32,
                    test_range_decreasing_i32,
                    test_i32s_increasing,
                    test_i32s_decreasing,
                    test_positive_i32s,
                    test_i32s,
                    test_range_up_i32,
                    test_range_down_i32,
                    test_range_i32,
                    range_increasing_i32_fail,
                    range_decreasing_i32_fail,
                    i32::max_value());
test_integer_range!(i64,
                    "i64",
                    range_up_i,
                    range_down_i,
                    range_i,
                    range_up_increasing_i64_helper,
                    range_up_decreasing_i64_helper,
                    range_down_increasing_i64_helper,
                    range_down_decreasing_i64_helper,
                    range_increasing_i64_helper,
                    range_decreasing_i64_helper,
                    range_up_exhaustive_i64_helper,
                    range_up_random_i64_helper,
                    range_down_exhaustive_i64_helper,
                    range_down_random_i64_helper,
                    range_exhaustive_i64_helper,
                    range_random_i64_helper,
                    test_range_up_increasing_i64,
                    test_range_up_decreasing_i64,
                    test_range_down_increasing_i64,
                    test_range_down_decreasing_i64,
                    test_range_increasing_i64,
                    test_range_decreasing_i64,
                    test_i64s_increasing,
                    test_i64s_decreasing,
                    test_positive_i64s,
                    test_i64s,
                    test_range_up_i64,
                    test_range_down_i64,
                    test_range_i64,
                    range_increasing_i64_fail,
                    range_decreasing_i64_fail,
                    i64::max_value());

macro_rules! test_integer_range_u {
    (
        $t: ty,
        $ts: expr,
        $pos_t: ident,
        $all_t: ident,
        $r_f_1: ident,
        $r_f_2: ident
    ) => {
        #[test]
        fn $pos_t() {
            let (eo, ep, rp) = prepare_test();
            eo.match_vec(&format!("exhaustive_positive_{}s", $ts), &mut ep.positive_u::<$t>());
            eo.match_vec_f(&format!("random_positive_{}s", $ts), &mut rp.positive_u::<$t>());
        }

        #[test]
        fn $all_t() {
            let (eo, ep, rp) = prepare_test();
            eo.match_vec(&format!("exhaustive_{}s", $ts), &mut ep.all_u::<$t>());
            eo.match_vec_f(&format!("random_{}s", $ts), &mut rp.all_u::<$t>());
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $r_f_1() {
            IteratorProvider::Exhaustive.range_u::<$t>(10, 9);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $r_f_2() {
            IteratorProvider::example_random().range_u::<$t>(10, 9);
        }
    }
}

test_integer_range_u!(u8,
                      "u8",
                      test_positive_u8s,
                      test_u8s,
                      range_u8_fail_1,
                      range_u8_fail_2);
test_integer_range_u!(u16,
                      "u16",
                      test_positive_u16s,
                      test_u16s,
                      range_u16_fail_1,
                      range_u16_fail_2);
test_integer_range_u!(u32,
                      "u32",
                      test_positive_u32s,
                      test_u32s,
                      range_u32_fail_1,
                      range_u32_fail_2);
test_integer_range_u!(u64,
                      "u64",
                      test_positive_u64s,
                      test_u64s,
                      range_u64_fail_1,
                      range_u64_fail_2);

macro_rules! test_integer_range_i {
    (
        $t: ty,
        $ts: expr,
        $rui: ident,
        $rud: ident,
        $rdi: ident,
        $rdd: ident,
        $pos: ident,
        $all: ident,
        $neg: ident,
        $nat: ident,
        $nz: ident,
        $rup: ident,
        $rdn: ident,
        $r: ident,
        $rui_th: ident,
        $rud_th: ident,
        $rdi_th: ident,
        $rdd_th: ident,
        $ri_th: ident,
        $rd_th: ident,
        $rup_eth: ident,
        $rup_rth: ident,
        $rdn_eth: ident,
        $rdn_rth: ident,
        $r_eth: ident,
        $r_rth: ident,
        $rui_t: ident,
        $rud_t: ident,
        $rdi_t: ident,
        $rdd_t: ident,
        $ri_t: ident,
        $rd_t: ident,
        $pos_t: ident,
        $all_t: ident,
        $neg_t: ident,
        $nat_t: ident,
        $nz_t: ident,
        $rup_t: ident,
        $rdn_t: ident,
        $r_t: ident,
        $ri_f: ident,
        $rd_f: ident,
        $r_f: ident,
        $min: expr,
        $max: expr,
        $r_f_1: ident,
        $r_f_2: ident
    ) => {
        #[test]
        fn $ri_t() {
            let (eo, p, _) = prepare_test();
            let s = "exhaustive_range_increasing";
            $ri_th(&eo, &p, &format!("{}_{}_vii", s, $ts), -10, -10);
            $ri_th(&eo, &p, &format!("{}_{}_viii", s, $ts), -20, -10);
            $ri_th(&eo, &p, &format!("{}_{}_ix", s, $ts), -100, 100);
            $ri_th(&eo, &p, &format!("{}_{}_x", s, $ts), $min, $max);
            $ri_th(&eo, &p, &format!("{}_{}_xi", s, $ts), $min + 1, $max - 1);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
        fn $ri_f() {
            IteratorProvider::Exhaustive.range_increasing_x(-9, -10);
        }

        #[test]
        fn $rd_t() {
            let (eo, p, _) = prepare_test();
            let s = "exhaustive_range_decreasing";
            $rd_th(&eo, &p, &format!("{}_{}_vii", s, $ts), -10, -10);
            $rd_th(&eo, &p, &format!("{}_{}_viii", s, $ts), -20, -10);
            $rd_th(&eo, &p, &format!("{}_{}_ix", s, $ts), -100, 100);
            $rd_th(&eo, &p, &format!("{}_{}_x", s, $ts), $min, $max);
            $rd_th(&eo, &p, &format!("{}_{}_xi", s, $ts), $min + 1, $max - 1);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
        fn $rd_f() {
            IteratorProvider::Exhaustive.range_decreasing_x(-9, -10);
        }

        #[test]
        fn $pos_t() {
            let (eo, ep, rp) = prepare_test();
            eo.match_vec(&format!("exhaustive_positive_{}s", $ts), &mut ep.positive_i::<$t>());
            eo.match_vec_f(&format!("random_positive_{}s", $ts), &mut rp.positive_i::<$t>());
        }

        #[test]
        fn $all_t() {
            let (eo, ep, rp) = prepare_test();
            eo.match_vec(&format!("exhaustive_{}s", $ts), &mut ep.all_i::<$t>());
            eo.match_vec_f(&format!("random_{}s", $ts), &mut rp.all_i::<$t>());
        }

        #[test]
        fn $neg_t() {
            let (eo, ep, rp) = prepare_test();
            eo.match_vec(&format!("exhaustive_negative_{}s", $ts), &mut ep.negative_i::<$t>());
            eo.match_vec_f(&format!("random_negative_{}s", $ts), &mut rp.negative_i::<$t>());
        }

        #[test]
        fn $nat_t() {
            let (eo, ep, rp) = prepare_test();
            eo.match_vec(&format!("exhaustive_natural_{}s", $ts), &mut ep.natural_i::<$t>());
            eo.match_vec_f(&format!("random_natural_{}s", $ts), &mut rp.natural_i::<$t>());
        }

        #[test]
        fn $nz_t() {
            let (eo, ep, rp) = prepare_test();
            eo.match_vec(&format!("exhaustive_nonzero_{}s", $ts), &mut ep.nonzero_i::<$t>());
            eo.match_vec_f(&format!("random_nonzero_{}s", $ts), &mut rp.nonzero_i::<$t>());
        }

        #[test]
        fn $rup_t() {
            let (eo, ep, rp) = prepare_test();
            let es = "exhaustive_range_up";
            $rup_eth(&eo, &ep, &format!("{}_{}_iv", es, $ts), -5);
            $rup_eth(&eo, &ep, &format!("{}_{}_v", es, $ts), $min);
            let rs = "random_range_up";
            $rup_rth(&eo, &rp, &format!("{}_{}_iv", rs, $ts), -10);
            $rup_rth(&eo, &rp, &format!("{}_{}_v", rs, $ts), $min);
        }

        #[test]
        fn $rdn_t() {
            let (eo, ep, rp) = prepare_test();
            let es = "exhaustive_range_down";
            $rdn_eth(&eo, &ep, &format!("{}_{}_iv", es, $ts), -5);
            $rdn_eth(&eo, &ep, &format!("{}_{}_v", es, $ts), $min);
            let rs = "random_range_down";
            $rdn_rth(&eo, &rp, &format!("{}_{}_iv", rs, $ts), -5);
            $rdn_rth(&eo, &rp, &format!("{}_{}_v", rs, $ts), $min);
        }

        #[test]
        fn $r_t() {
            let (eo, ep, rp) = prepare_test();
            let es = "exhaustive_range";
            $r_eth(&eo, &ep, &format!("{}_{}_vii", es, $ts), -10, -10);
            $r_eth(&eo, &ep, &format!("{}_{}_viii", es, $ts), -20, -10);
            $r_eth(&eo, &ep, &format!("{}_{}_ix", es, $ts), -100, 100);
            $r_eth(&eo, &ep, &format!("{}_{}_x", es, $ts), $min, $max);
            $r_eth(&eo, &ep, &format!("{}_{}_xi", es, $ts), $min + 1, $max - 1);
            let rs = "random_range";
            $r_rth(&eo, &rp, &format!("{}_{}_vii", rs, $ts), -10, -10);
            $r_rth(&eo, &rp, &format!("{}_{}_viii", rs, $ts), -20, -10);
            $r_rth(&eo, &rp, &format!("{}_{}_ix", rs, $ts), -100, 100);
            $r_rth(&eo, &rp, &format!("{}_{}_x", rs, $ts), $min, $max);
            $r_rth(&eo, &rp, &format!("{}_{}_xi", rs, $ts), $min + 1, $max - 1);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
        fn $r_f() {
            IteratorProvider::Exhaustive.range_i::<$t>(-9, -10);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $r_f_1() {
            IteratorProvider::Exhaustive.range_i::<$t>(10, 9);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $r_f_2() {
            IteratorProvider::example_random().range_i::<$t>(10, 9);
        }
    }
}

test_integer_range_i!(i8,
                      "i8",
                      range_up_increasing_i8,
                      range_up_decreasing_i8,
                      range_down_increasing_i8,
                      range_down_decreasing_i8,
                      positive_i8s,
                      i8s,
                      negative_i8s,
                      natural_i8s,
                      nonzero_i8s,
                      range_up_i8,
                      range_down_i8,
                      range_i8,
                      range_up_increasing_i8_helper,
                      range_up_decreasing_i8_helper,
                      range_down_increasing_i8_helper,
                      range_down_decreasing_i8_helper,
                      range_increasing_i8_helper,
                      range_decreasing_i8_helper,
                      range_up_exhaustive_i8_helper,
                      range_up_random_i8_helper,
                      range_down_exhaustive_i8_helper,
                      range_down_random_i8_helper,
                      range_exhaustive_i8_helper,
                      range_random_i8_helper,
                      test_range_up_increasing_i8_i,
                      test_range_up_decreasing_i8_i,
                      test_range_down_increasing_i8_i,
                      test_range_down_decreasing_i8_i,
                      test_range_increasing_i8_i,
                      test_range_decreasing_i8_i,
                      test_positive_i8s,
                      test_i8s,
                      test_negative_i8s,
                      test_natural_i8s,
                      test_nonzero_i8s,
                      test_range_up_i8_i,
                      test_range_down_i8_i,
                      test_range_i8_i,
                      range_increasing_i8_fail_i,
                      range_decreasing_i8_fail_i,
                      range_i8_fail_i,
                      i8::min_value(),
                      i8::max_value(),
                      range_i8_fail_1,
                      range_i8_fail_2);
test_integer_range_i!(i16,
                      "i16",
                      range_up_increasing_i16,
                      range_up_decreasing_i16,
                      range_down_increasing_i16,
                      range_down_decreasing_i16,
                      positive_i16s,
                      i16s,
                      negative_i16s,
                      natural_i16s,
                      nonzero_i16s,
                      range_up_i16,
                      range_down_i16,
                      range_i16,
                      range_up_increasing_i16_helper,
                      range_up_decreasing_i16_helper,
                      range_down_increasing_i16_helper,
                      range_down_decreasing_i16_helper,
                      range_increasing_i16_helper,
                      range_decreasing_i16_helper,
                      range_up_exhaustive_i16_helper,
                      range_up_random_i16_helper,
                      range_down_exhaustive_i16_helper,
                      range_down_random_i16_helper,
                      range_exhaustive_i16_helper,
                      range_random_i16_helper,
                      test_range_up_increasing_i16_i,
                      test_range_up_decreasing_i16_i,
                      test_range_down_increasing_i16_i,
                      test_range_down_decreasing_i16_i,
                      test_range_increasing_i16_i,
                      test_range_decreasing_i16_i,
                      test_positive_i16s,
                      test_i16s,
                      test_negative_i16s,
                      test_natural_i16s,
                      test_nonzero_i16s,
                      test_range_up_i16_i,
                      test_range_down_i16_i,
                      test_range_i16_i,
                      range_increasing_i16_fail_i,
                      range_decreasing_i16_fail_i,
                      range_i16_fail_i,
                      i16::min_value(),
                      i16::max_value(),
                      range_i16_fail_1,
                      range_i16_fail_2);
test_integer_range_i!(i32,
                      "i32",
                      range_up_increasing_i32,
                      range_up_decreasing_i32,
                      range_down_increasing_i32,
                      range_down_decreasing_i32,
                      positive_i32s,
                      i32s,
                      negative_i32s,
                      natural_i32s,
                      nonzero_i32s,
                      range_up_i32,
                      range_down_i32,
                      range_i32,
                      range_up_increasing_i32_helper,
                      range_up_decreasing_i32_helper,
                      range_down_increasing_i32_helper,
                      range_down_decreasing_i32_helper,
                      range_increasing_i32_helper,
                      range_decreasing_i32_helper,
                      range_up_exhaustive_i32_helper,
                      range_up_random_i32_helper,
                      range_down_exhaustive_i32_helper,
                      range_down_random_i32_helper,
                      range_exhaustive_i32_helper,
                      range_random_i32_helper,
                      test_range_up_increasing_i32_i,
                      test_range_up_decreasing_i32_i,
                      test_range_down_increasing_i32_i,
                      test_range_down_decreasing_i32_i,
                      test_range_increasing_i32_i,
                      test_range_decreasing_i32_i,
                      test_positive_i32s,
                      test_i32s,
                      test_negative_i32s,
                      test_natural_i32s,
                      test_nonzero_i32s,
                      test_range_up_i32_i,
                      test_range_down_i32_i,
                      test_range_i32_i,
                      range_increasing_i32_fail_i,
                      range_decreasing_i32_fail_i,
                      range_i32_fail_i,
                      i32::min_value(),
                      i32::max_value(),
                      range_i32_fail_1,
                      range_i32_fail_2);
test_integer_range_i!(i64,
                      "i64",
                      range_up_increasing_i64,
                      range_up_decreasing_i64,
                      range_down_increasing_i64,
                      range_down_decreasing_i64,
                      positive_i64s,
                      i64s,
                      negative_i64s,
                      natural_i64s,
                      nonzero_i64s,
                      range_up_i64,
                      range_down_i64,
                      range_i64,
                      range_up_increasing_i64_helper,
                      range_up_decreasing_i64_helper,
                      range_down_increasing_i64_helper,
                      range_down_decreasing_i64_helper,
                      range_increasing_i64_helper,
                      range_decreasing_i64_helper,
                      range_up_exhaustive_i64_helper,
                      range_up_random_i64_helper,
                      range_down_exhaustive_i64_helper,
                      range_down_random_i64_helper,
                      range_exhaustive_i64_helper,
                      range_random_i64_helper,
                      test_range_up_increasing_i64_i,
                      test_range_up_decreasing_i64_i,
                      test_range_down_increasing_i64_i,
                      test_range_down_decreasing_i64_i,
                      test_range_increasing_i64_i,
                      test_range_decreasing_i64_i,
                      test_positive_i64s,
                      test_i64s,
                      test_negative_i64s,
                      test_natural_i64s,
                      test_nonzero_i64s,
                      test_range_up_i64_i,
                      test_range_down_i64_i,
                      test_range_i64_i,
                      range_increasing_i64_fail_i,
                      range_decreasing_i64_fail_i,
                      range_i64_fail_i,
                      i64::min_value(),
                      i64::max_value(),
                      range_i64_fail_1,
                      range_i64_fail_2);

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

fn range_up_increasing_char_helper(eo: &TestOutput, p: &IteratorProvider, key: &str, a: char) {
    eo.match_vec_debug(key, &mut p.range_up_increasing_char(a));
}

#[test]
fn test_range_up_increasing_char() {
    let (eo, p, _) = prepare_test();
    let s = "exhaustive_range_up_increasing_char";
    range_up_increasing_char_helper(&eo, &p, &format!("{}_i", s), '\0');
    range_up_increasing_char_helper(&eo, &p, &format!("{}_ii", s), 'a');
    range_up_increasing_char_helper(&eo, &p, &format!("{}_iii", s), 'Ш');
    range_up_increasing_char_helper(&eo, &p, &format!("{}_iv", s), '\u{D7FF}');
    range_up_increasing_char_helper(&eo, &p, &format!("{}_v", s), char::MAX);
}

fn range_up_decreasing_char_helper(eo: &TestOutput, p: &IteratorProvider, key: &str, a: char) {
    eo.match_vec_debug(key, &mut p.range_up_decreasing_char(a));
}

#[test]
fn test_range_up_decreasing_char() {
    let (eo, p, _) = prepare_test();
    let s = "exhaustive_range_up_decreasing_char";
    range_up_decreasing_char_helper(&eo, &p, &format!("{}_i", s), '\0');
    range_up_decreasing_char_helper(&eo, &p, &format!("{}_ii", s), 'a');
    range_up_decreasing_char_helper(&eo, &p, &format!("{}_iii", s), 'Ш');
    range_up_decreasing_char_helper(&eo, &p, &format!("{}_iv", s), '\u{D7FF}');
    range_up_decreasing_char_helper(&eo, &p, &format!("{}_v", s), char::MAX);
}

fn range_down_increasing_char_helper(eo: &TestOutput, p: &IteratorProvider, key: &str, a: char) {
    eo.match_vec_debug(key, &mut p.range_down_increasing_char(a));
}

#[test]
fn test_range_down_increasing_char() {
    let (eo, p, _) = prepare_test();
    let s = "exhaustive_range_down_increasing_char";
    range_down_increasing_char_helper(&eo, &p, &format!("{}_i", s), '\0');
    range_down_increasing_char_helper(&eo, &p, &format!("{}_ii", s), 'a');
    range_down_increasing_char_helper(&eo, &p, &format!("{}_iii", s), 'Ш');
    range_down_increasing_char_helper(&eo, &p, &format!("{}_iv", s), '\u{E000}');
    range_down_increasing_char_helper(&eo, &p, &format!("{}_v", s), char::MAX);
}

fn range_down_decreasing_char_helper(eo: &TestOutput, p: &IteratorProvider, key: &str, a: char) {
    eo.match_vec_debug(key, &mut p.range_down_decreasing_char(a));
}

#[test]
fn test_range_down_decreasing_char() {
    let (eo, p, _) = prepare_test();
    let s = "exhaustive_range_down_decreasing_char";
    range_down_decreasing_char_helper(&eo, &p, &format!("{}_i", s), '\0');
    range_down_decreasing_char_helper(&eo, &p, &format!("{}_ii", s), 'a');
    range_down_decreasing_char_helper(&eo, &p, &format!("{}_iii", s), 'Ш');
    range_down_decreasing_char_helper(&eo, &p, &format!("{}_iv", s), '\u{E000}');
    range_down_decreasing_char_helper(&eo, &p, &format!("{}_v", s), char::MAX);
}

fn range_increasing_char_helper(eo: &TestOutput,
                                p: &IteratorProvider,
                                key: &str,
                                a: char,
                                b: char) {
    eo.match_vec_debug(key, &mut p.range_increasing_char(a, b));
}

#[test]
fn test_range_increasing_char() {
    let (eo, p, _) = prepare_test();
    let s = "exhaustive_range_increasing_char";
    range_increasing_char_helper(&eo, &p, &format!("{}_i", s), 'a', 'z');
    range_increasing_char_helper(&eo, &p, &format!("{}_ii", s), 'a', 'a');
    range_increasing_char_helper(&eo, &p, &format!("{}_iii", s), '!', '9');
    range_increasing_char_helper(&eo, &p, &format!("{}_iv", s), '\u{D7FF}', '\u{E000}');
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: a, b: A")]
fn range_increasing_char_fail() {
    IteratorProvider::Exhaustive.range_increasing_char('a', 'A');
}

fn range_decreasing_char_helper(eo: &TestOutput,
                                p: &IteratorProvider,
                                key: &str,
                                a: char,
                                b: char) {
    eo.match_vec_debug(key, &mut p.range_decreasing_char(a, b));
}

#[test]
fn test_range_decreasing_char() {
    let (eo, p, _) = prepare_test();
    let s = "exhaustive_range_decreasing_char";
    range_decreasing_char_helper(&eo, &p, &format!("{}_i", s), 'a', 'z');
    range_decreasing_char_helper(&eo, &p, &format!("{}_ii", s), 'a', 'a');
    range_decreasing_char_helper(&eo, &p, &format!("{}_iii", s), '!', '9');
    range_decreasing_char_helper(&eo, &p, &format!("{}_iv", s), '\u{D7FF}', '\u{E000}');
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

fn range_up_char_exhaustive_helper(eo: &TestOutput, p: &IteratorProvider, key: &str, a: char) {
    eo.match_vec_debug(key, &mut p.range_up_char(a));
}

fn range_up_char_random_helper(eo: &TestOutput, p: &IteratorProvider, key: &str, a: char) {
    eo.match_vec_f_debug(key, &mut p.range_up_char(a));
}

#[test]
fn test_range_up_char() {
    let (eo, ep, rp) = prepare_test();
    let es = "exhaustive_range_up_char";
    range_up_char_exhaustive_helper(&eo, &ep, &format!("{}_i", es), '\0');
    range_up_char_exhaustive_helper(&eo, &ep, &format!("{}_ii", es), 'a');
    range_up_char_exhaustive_helper(&eo, &ep, &format!("{}_iii", es), 'Ш');
    range_up_char_exhaustive_helper(&eo, &ep, &format!("{}_iv", es), '\u{D7FF}');
    range_up_char_exhaustive_helper(&eo, &ep, &format!("{}_v", es), char::MAX);
    let rs = "random_range_up_char";
    range_up_char_random_helper(&eo, &rp, &format!("{}_i", rs), '\0');
    range_up_char_random_helper(&eo, &rp, &format!("{}_ii", rs), 'a');
    range_up_char_random_helper(&eo, &rp, &format!("{}_iii", rs), 'Ш');
    range_up_char_random_helper(&eo, &rp, &format!("{}_iv", rs), '\u{D7FF}');
    range_up_char_random_helper(&eo, &rp, &format!("{}_v", rs), char::MAX);
}

fn range_down_char_exhaustive_helper(eo: &TestOutput, p: &IteratorProvider, key: &str, a: char) {
    eo.match_vec_debug(key, &mut p.range_down_char(a));
}

fn range_down_char_random_helper(eo: &TestOutput, p: &IteratorProvider, key: &str, a: char) {
    eo.match_vec_f_debug(key, &mut p.range_down_char(a));
}

#[test]
fn test_range_down_char() {
    let (eo, ep, rp) = prepare_test();
    let es = "exhaustive_range_down_char";
    range_down_char_exhaustive_helper(&eo, &ep, &format!("{}_i", es), '\0');
    range_down_char_exhaustive_helper(&eo, &ep, &format!("{}_ii", es), 'a');
    range_down_char_exhaustive_helper(&eo, &ep, &format!("{}_iii", es), 'Ш');
    range_down_char_exhaustive_helper(&eo, &ep, &format!("{}_iv", es), '\u{E000}');
    range_down_char_exhaustive_helper(&eo, &ep, &format!("{}_v", es), char::MAX);
    let rs = "random_range_down_char";
    range_down_char_random_helper(&eo, &rp, &format!("{}_i", rs), '\0');
    range_down_char_random_helper(&eo, &rp, &format!("{}_ii", rs), 'a');
    range_down_char_random_helper(&eo, &rp, &format!("{}_iii", rs), 'Ш');
    range_down_char_random_helper(&eo, &rp, &format!("{}_iv", rs), '\u{E000}');
    range_down_char_random_helper(&eo, &rp, &format!("{}_v", rs), char::MAX);
}

fn range_char_exhaustive_helper(eo: &TestOutput,
                                p: &IteratorProvider,
                                key: &str,
                                a: char,
                                b: char) {
    eo.match_vec_debug(key, &mut p.range_char(a, b));
}

fn range_char_random_helper(eo: &TestOutput, p: &IteratorProvider, key: &str, a: char, b: char) {
    eo.match_vec_f_debug(key, &mut p.range_char(a, b));
}

#[test]
fn test_range_char() {
    let (eo, ep, rp) = prepare_test();
    let es = "exhaustive_range_char";
    range_char_exhaustive_helper(&eo, &ep, &format!("{}_i", es), 'a', 'z');
    range_char_exhaustive_helper(&eo, &ep, &format!("{}_ii", es), 'a', 'a');
    range_char_exhaustive_helper(&eo, &ep, &format!("{}_iii", es), '!', '9');
    range_char_exhaustive_helper(&eo, &ep, &format!("{}_iv", es), '\u{D7FF}', '\u{E000}');
    let rs = "random_range_char";
    range_char_random_helper(&eo, &rp, &format!("{}_i", rs), 'a', 'z');
    range_char_random_helper(&eo, &rp, &format!("{}_ii", rs), 'a', 'a');
    range_char_random_helper(&eo, &rp, &format!("{}_iii", rs), '!', '9');
    range_char_random_helper(&eo, &rp, &format!("{}_iv", rs), '\u{D7FF}', '\u{E000}');
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: a, b: A")]
fn range_char_fail() {
    IteratorProvider::Exhaustive.range_char('a', 'A');
}

fn range_up_increasing_integer_helper(eo: &TestOutput, p: &IteratorProvider, key: &str, a: &str) {
    eo.match_vec(key,
                 &mut p.range_up_increasing_integer(Integer::from_str(a).unwrap()));
}

#[test]
fn test_range_up_increasing_integer() {
    let (eo, p, _) = prepare_test();
    let s = "exhaustive_range_up_increasing_integer";
    range_up_increasing_integer_helper(&eo, &p, &format!("{}_i", s), "0");
    range_up_increasing_integer_helper(&eo, &p, &format!("{}_ii", s), "5");
    range_up_increasing_integer_helper(&eo, &p, &format!("{}_iii", s), "-5");
}

fn range_down_decreasing_integer_helper(eo: &TestOutput,
                                        p: &IteratorProvider,
                                        key: &str,
                                        a: &str) {
    eo.match_vec(key,
                 &mut p.range_down_decreasing_integer(Integer::from_str(a).unwrap()));
}

#[test]
fn test_range_down_decreasing_integer() {
    let (eo, p, _) = prepare_test();
    let s = "exhaustive_range_down_decreasing_integer";
    range_down_decreasing_integer_helper(&eo, &p, &format!("{}_i", s), "0");
    range_down_decreasing_integer_helper(&eo, &p, &format!("{}_ii", s), "5");
    range_down_decreasing_integer_helper(&eo, &p, &format!("{}_iii", s), "-5");
}

fn range_increasing_integer_helper(eo: &TestOutput,
                                   p: &IteratorProvider,
                                   key: &str,
                                   a: &str,
                                   b: &str) {
    eo.match_vec(key,
                 &mut p.range_increasing_integer(Integer::from_str(a).unwrap(),
                                                 Integer::from_str(b).unwrap()));
}

#[test]
fn test_range_increasing_integer() {
    let (eo, p, _) = prepare_test();
    let s = "exhaustive_range_increasing_integer";
    range_increasing_integer_helper(&eo, &p, &format!("{}_i", s), "0", "0");
    range_increasing_integer_helper(&eo, &p, &format!("{}_ii", s), "0", "10");
    range_increasing_integer_helper(&eo, &p, &format!("{}_iii", s), "10", "20");
    range_increasing_integer_helper(&eo, &p, &format!("{}_iv", s), "10", "10");
    range_increasing_integer_helper(&eo, &p, &format!("{}_v", s), "-10", "-10");
    range_increasing_integer_helper(&eo, &p, &format!("{}_vi", s), "-20", "-10");
    range_increasing_integer_helper(&eo, &p, &format!("{}_vii", s), "-100", "100");
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

fn range_decreasing_integer_helper(eo: &TestOutput,
                                   p: &IteratorProvider,
                                   key: &str,
                                   a: &str,
                                   b: &str) {
    eo.match_vec(key,
                 &mut p.range_decreasing_integer(Integer::from_str(a).unwrap(),
                                                 Integer::from_str(b).unwrap()));
}

#[test]
fn test_range_decreasing_integer() {
    let (eo, p, _) = prepare_test();
    let s = "exhaustive_range_decreasing_integer";
    range_decreasing_integer_helper(&eo, &p, &format!("{}_i", s), "0", "0");
    range_decreasing_integer_helper(&eo, &p, &format!("{}_ii", s), "0", "10");
    range_decreasing_integer_helper(&eo, &p, &format!("{}_iii", s), "10", "20");
    range_decreasing_integer_helper(&eo, &p, &format!("{}_iv", s), "10", "10");
    range_decreasing_integer_helper(&eo, &p, &format!("{}_v", s), "-10", "-10");
    range_decreasing_integer_helper(&eo, &p, &format!("{}_vi", s), "-20", "-10");
    range_decreasing_integer_helper(&eo, &p, &format!("{}_vii", s), "-100", "100");
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

fn range_integer_exhaustive_helper(eo: &TestOutput,
                                   p: &IteratorProvider,
                                   key: &str,
                                   a: &str,
                                   b: &str) {
    eo.match_vec(key,
                 &mut p.range_integer(Integer::from_str(a).unwrap(),
                                      Integer::from_str(b).unwrap()));
}

fn range_integer_random_helper(eo: &TestOutput,
                               p: &IteratorProvider,
                               key: &str,
                               a: &str,
                               b: &str) {
    eo.match_vec_f(key,
                   &mut p.range_integer(Integer::from_str(a).unwrap(),
                                        Integer::from_str(b).unwrap()));
}

#[test]
fn test_range_integer() {
    let (eo, ep, rp) = prepare_test();
    let es = "exhaustive_range_integer";
    range_integer_exhaustive_helper(&eo, &ep, &format!("{}_i", es), "0", "0");
    range_integer_exhaustive_helper(&eo, &ep, &format!("{}_ii", es), "0", "10");
    range_integer_exhaustive_helper(&eo, &ep, &format!("{}_iii", es), "10", "20");
    range_integer_exhaustive_helper(&eo, &ep, &format!("{}_iv", es), "10", "10");
    range_integer_exhaustive_helper(&eo, &ep, &format!("{}_v", es), "-10", "-10");
    range_integer_exhaustive_helper(&eo, &ep, &format!("{}_vi", es), "-20", "-10");
    range_integer_exhaustive_helper(&eo, &ep, &format!("{}_vii", es), "-100", "100");
    let rs = "random_range_integer";
    range_integer_random_helper(&eo, &rp, &format!("{}_i", rs), "0", "0");
    range_integer_random_helper(&eo, &rp, &format!("{}_ii", rs), "0", "10");
    range_integer_random_helper(&eo, &rp, &format!("{}_iii", rs), "10", "20");
    range_integer_random_helper(&eo, &rp, &format!("{}_iv", rs), "10", "10");
    range_integer_random_helper(&eo, &rp, &format!("{}_v", rs), "-10", "-10");
    range_integer_random_helper(&eo, &rp, &format!("{}_vi", rs), "-20", "-10");
    range_integer_random_helper(&eo, &rp, &format!("{}_vii", rs), "-100", "100");
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

fn exhaustive_generate_from_vector_helper(eo: &TestOutput,
                                          p: &IteratorProvider,
                                          key: &str,
                                          xs: &str) {
    let xs: Vec<u32> = parse_vec(xs).unwrap();
    eo.match_vec(key, &mut p.exhaustive_generate_from_vector(xs));
}

#[test]
fn test_exhaustive_generate_from_vector() {
    let (eo, p, _) = prepare_test();
    let s = "exhaustive_exhaustive_generate_from_vector";
    exhaustive_generate_from_vector_helper(&eo, &p, &format!("{}_i", s), "[]");
    exhaustive_generate_from_vector_helper(&eo, &p, &format!("{}_ii", s), "[5]");
    exhaustive_generate_from_vector_helper(&eo, &p, &format!("{}_iii", s), "[1, 2, 3]");
    exhaustive_generate_from_vector_helper(&eo, &p, &format!("{}_iv", s), "[3, 1, 4, 1]");
}

fn generate_from_vector_exhaustive_helper(eo: &TestOutput,
                                          p: &IteratorProvider,
                                          key: &str,
                                          xs: &str) {
    let xs: Vec<u32> = parse_vec(xs).unwrap();
    eo.match_vec(key, &mut p.generate_from_vector(xs));
}

fn generate_from_vector_random_helper(eo: &TestOutput, p: &IteratorProvider, key: &str, xs: &str) {
    let xs: Vec<u32> = parse_vec(xs).unwrap();
    eo.match_vec_f(key, &mut p.generate_from_vector(xs));
}

#[test]
fn test_generate_from_vector() {
    let (eo, ep, rp) = prepare_test();
    let es = "exhaustive_generate_from_vector";
    generate_from_vector_exhaustive_helper(&eo, &ep, &format!("{}_i", es), "[]");
    generate_from_vector_exhaustive_helper(&eo, &ep, &format!("{}_ii", es), "[5]");
    generate_from_vector_exhaustive_helper(&eo, &ep, &format!("{}_iii", es), "[1, 2, 3]");
    generate_from_vector_exhaustive_helper(&eo, &ep, &format!("{}_iv", es), "[3, 1, 4, 1]");
    let rs = "random_generate_from_vector";
    generate_from_vector_random_helper(&eo, &rp, &format!("{}_i", rs), "[5]");
    generate_from_vector_random_helper(&eo, &rp, &format!("{}_ii", rs), "[1, 2, 3]");
    generate_from_vector_random_helper(&eo, &rp, &format!("{}_iii", rs), "[3, 1, 4, 1]");
}

#[test]
#[should_panic(expected = "Cannot randomly generate values from an empty Vec.")]
fn generate_from_vector_fail() {
    &mut IteratorProvider::example_random().generate_from_vector(Vec::<u32>::new());
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

fn positive_u32s_geometric_helper(eo: &TestOutput, p: &IteratorProvider, key: &str, scale: u32) {
    eo.match_vec_f(key, &mut p.positive_u32s_geometric(scale));
}

#[test]
fn test_positive_u32s_geometric() {
    let (eo, _, p) = prepare_test();
    let s = "random_positive_u32s_geometric";
    positive_u32s_geometric_helper(&eo, &p, &format!("{}_i", s), 0);
    positive_u32s_geometric_helper(&eo, &p, &format!("{}_ii", s), 1);
    positive_u32s_geometric_helper(&eo, &p, &format!("{}_iii", s), 2);
    positive_u32s_geometric_helper(&eo, &p, &format!("{}_iv", s), 3);
    positive_u32s_geometric_helper(&eo, &p, &format!("{}_v", s), 10);
    positive_u32s_geometric_helper(&eo, &p, &format!("{}_vi", s), 100);
}

fn natural_u32s_geometric_helper(eo: &TestOutput, p: &IteratorProvider, key: &str, scale: u32) {
    eo.match_vec_f(key, &mut p.natural_u32s_geometric(scale));
}

#[test]
fn test_natural_u32s_geometric() {
    let (eo, _, p) = prepare_test();
    let s = "random_natural_u32s_geometric";
    natural_u32s_geometric_helper(&eo, &p, &format!("{}_i", s), 0);
    natural_u32s_geometric_helper(&eo, &p, &format!("{}_ii", s), 1);
    natural_u32s_geometric_helper(&eo, &p, &format!("{}_iii", s), 2);
    natural_u32s_geometric_helper(&eo, &p, &format!("{}_iv", s), 3);
    natural_u32s_geometric_helper(&eo, &p, &format!("{}_v", s), 10);
    natural_u32s_geometric_helper(&eo, &p, &format!("{}_vi", s), 100);
}
