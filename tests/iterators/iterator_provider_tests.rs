extern crate rust_wheels_lib;

use common;
use common::TestOutput;
use self::rust_wheels_lib::iterators::iterator_provider::IteratorProvider;

fn prepare_test() -> (TestOutput, IteratorProvider, IteratorProvider) {
    (common::get_expected_test_outputs(),
     IteratorProvider::Exhaustive,
     IteratorProvider::example_random())
}

macro_rules! test_integer_range {
    (
        $t: ty,
        $ts: expr,
        $rui: ident,
        $rud: ident,
        $rdi: ident,
        $rdd: ident,
        $ri: ident,
        $rd: ident,
        $i: ident,
        $d: ident,
        $pos: ident,
        $all: ident,
        $rui_th: ident,
        $rud_th: ident,
        $rdi_th: ident,
        $rdd_th: ident,
        $ri_th: ident,
        $rd_th: ident,
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
        $ri_f: ident,
        $rd_f: ident,
        $max: expr
    ) => {
        fn $rui_th(eo: &TestOutput, p: &IteratorProvider, key: &str, a: $t) {
            eo.match_vec(key, &mut p.$rui(a));
        }

        #[test]
        fn $rui_t() {
            let (eo, p, _) = prepare_test();
            let s = "exhaustive_range_up_increasing";
            $rui_th(&eo, &p, &format!("{}_{}_i", s, $ts), 0);
            $rui_th(&eo, &p, &format!("{}_{}_ii", s, $ts), 5);
            $rui_th(&eo, &p, &format!("{}_{}_iii", s, $ts), $max);
        }

        fn $rud_th(eo: &TestOutput, p: &IteratorProvider, key: &str, a: $t) {
            eo.match_vec(key, &mut p.$rud(a));
        }

        #[test]
        fn $rud_t() {
            let (eo, p, _) = prepare_test();
            let s = "exhaustive_range_up_decreasing";
            $rud_th(&eo, &p, &format!("{}_{}_i", s, $ts), 0);
            $rud_th(&eo, &p, &format!("{}_{}_ii", s, $ts), 5);
            $rud_th(&eo, &p, &format!("{}_{}_iii", s, $ts), $max);
        }

        fn $rdi_th(eo: &TestOutput, p: &IteratorProvider, key: &str, a: $t) {
            eo.match_vec(key, &mut p.$rdi(a));
        }

        #[test]
        fn $rdi_t() {
            let (eo, p, _) = prepare_test();
            let s = "exhaustive_range_down_increasing";
            $rdi_th(&eo, &p, &format!("{}_{}_i", s, $ts), 0);
            $rdi_th(&eo, &p, &format!("{}_{}_ii", s, $ts), 5);
            $rdi_th(&eo, &p, &format!("{}_{}_iii", s, $ts), $max);
        }

        fn $rdd_th(eo: &TestOutput, p: &IteratorProvider, key: &str, a: $t) {
            eo.match_vec(key, &mut p.$rdd(a));
        }

        #[test]
        fn $rdd_t() {
            let (eo, p, _) = prepare_test();
            let s = "exhaustive_range_down_decreasing";
            $rdd_th(&eo, &p, &format!("{}_{}_i", s, $ts), 0);
            $rdd_th(&eo, &p, &format!("{}_{}_ii", s, $ts), 5);
            $rdd_th(&eo, &p, &format!("{}_{}_iii", s, $ts), $max);
        }

        fn $ri_th(eo: &TestOutput, p: &IteratorProvider, key: &str, a: $t, b: $t) {
            eo.match_vec(key, &mut p.$ri(a, b));
        }

        #[test]
        fn $ri_t() {
            let (eo, p, _) = prepare_test();
            let s = "exhaustive_range_increasing";
            $ri_th(&eo, &p, &format!("{}_{}_i", s, $ts), 0, 0);
            $ri_th(&eo, &p, &format!("{}_{}_ii", s, $ts), 0, 10);
            $ri_th(&eo, &p, &format!("{}_{}_iii", s, $ts), 10, 20);
            $ri_th(&eo, &p, &format!("{}_{}_iv", s, $ts), 10, 10);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $ri_f() {
            IteratorProvider::Exhaustive.$ri(10, 9);
        }

        fn $rd_th(eo: &TestOutput, p: &IteratorProvider, key: &str, a: $t, b: $t) {
            eo.match_vec(key, &mut p.$rd(a, b));
        }

        #[test]
        fn $rd_t() {
            let (eo, p, _) = prepare_test();
            let s = "exhaustive_range_decreasing";
            $rd_th(&eo, &p, &format!("{}_{}_i", s, $ts), 0, 0);
            $rd_th(&eo, &p, &format!("{}_{}_ii", s, $ts), 0, 10);
            $rd_th(&eo, &p, &format!("{}_{}_iii", s, $ts), 10, 20);
            $rd_th(&eo, &p, &format!("{}_{}_iv", s, $ts), 10, 10);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $rd_f() {
            IteratorProvider::Exhaustive.$rd(10, 9);
        }

        #[test]
        fn $i_t() {
            let (eo, p, _) = prepare_test();
            eo.match_vec(&format!("exhaustive_{}s_increasing", $ts), &mut p.$i());
        }

        #[test]
        fn $d_t() {
            let (eo, p, _) = prepare_test();
            eo.match_vec(&format!("exhaustive_{}s_decreasing", $ts), &mut p.$d());
        }

        #[test]
        fn $pos_t() {
            let (eo, ep, rp) = prepare_test();
            eo.match_vec(&format!("exhaustive_positive_{}s", $ts), &mut ep.$pos());
            eo.match_vec_f(&format!("random_positive_{}s", $ts), &mut rp.$pos());
        }

        #[test]
        fn $all_t() {
            let (eo, ep, rp) = prepare_test();
            eo.match_vec(&format!("exhaustive_{}s", $ts), &mut ep.$all());
            eo.match_vec_f(&format!("random_{}s", $ts), &mut rp.$all());
        }
    }
}

test_integer_range!(u8,
                    "u8",
                    range_up_increasing_u8,
                    range_up_decreasing_u8,
                    range_down_increasing_u8,
                    range_down_decreasing_u8,
                    range_increasing_u8,
                    range_decreasing_u8,
                    u8s_increasing,
                    u8s_decreasing,
                    positive_u8s,
                    u8s,
                    range_up_increasing_u8_helper,
                    range_up_decreasing_u8_helper,
                    range_down_increasing_u8_helper,
                    range_down_decreasing_u8_helper,
                    range_increasing_u8_helper,
                    range_decreasing_u8_helper,
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
                    range_increasing_u8_fail,
                    range_decreasing_u8_fail,
                    u8::max_value());
test_integer_range!(u16,
                    "u16",
                    range_up_increasing_u16,
                    range_up_decreasing_u16,
                    range_down_increasing_u16,
                    range_down_decreasing_u16,
                    range_increasing_u16,
                    range_decreasing_u16,
                    u16s_increasing,
                    u16s_decreasing,
                    positive_u16s,
                    u16s,
                    range_up_increasing_u16_helper,
                    range_up_decreasing_u16_helper,
                    range_down_increasing_u16_helper,
                    range_down_decreasing_u16_helper,
                    range_increasing_u16_helper,
                    range_decreasing_u16_helper,
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
                    range_increasing_u16_fail,
                    range_decreasing_u16_fail,
                    u16::max_value());
test_integer_range!(u32,
                    "u32",
                    range_up_increasing_u32,
                    range_up_decreasing_u32,
                    range_down_increasing_u32,
                    range_down_decreasing_u32,
                    range_increasing_u32,
                    range_decreasing_u32,
                    u32s_increasing,
                    u32s_decreasing,
                    positive_u32s,
                    u32s,
                    range_up_increasing_u32_helper,
                    range_up_decreasing_u32_helper,
                    range_down_increasing_u32_helper,
                    range_down_decreasing_u32_helper,
                    range_increasing_u32_helper,
                    range_decreasing_u32_helper,
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
                    range_increasing_u32_fail,
                    range_decreasing_u32_fail,
                    u32::max_value());
test_integer_range!(u64,
                    "u64",
                    range_up_increasing_u64,
                    range_up_decreasing_u64,
                    range_down_increasing_u64,
                    range_down_decreasing_u64,
                    range_increasing_u64,
                    range_decreasing_u64,
                    u64s_increasing,
                    u64s_decreasing,
                    positive_u64s,
                    u64s,
                    range_up_increasing_u64_helper,
                    range_up_decreasing_u64_helper,
                    range_down_increasing_u64_helper,
                    range_down_decreasing_u64_helper,
                    range_increasing_u64_helper,
                    range_decreasing_u64_helper,
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
                    range_increasing_u64_fail,
                    range_decreasing_u64_fail,
                    u64::max_value());
test_integer_range!(i8,
                    "i8",
                    range_up_increasing_i8,
                    range_up_decreasing_i8,
                    range_down_increasing_i8,
                    range_down_decreasing_i8,
                    range_increasing_i8,
                    range_decreasing_i8,
                    i8s_increasing,
                    i8s_decreasing,
                    positive_i8s,
                    i8s,
                    range_up_increasing_i8_helper,
                    range_up_decreasing_i8_helper,
                    range_down_increasing_i8_helper,
                    range_down_decreasing_i8_helper,
                    range_increasing_i8_helper,
                    range_decreasing_i8_helper,
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
                    range_increasing_i8_fail,
                    range_decreasing_i8_fail,
                    i8::max_value());
test_integer_range!(i16,
                    "i16",
                    range_up_increasing_i16,
                    range_up_decreasing_i16,
                    range_down_increasing_i16,
                    range_down_decreasing_i16,
                    range_increasing_i16,
                    range_decreasing_i16,
                    i16s_increasing,
                    i16s_decreasing,
                    positive_i16s,
                    i16s,
                    range_up_increasing_i16_helper,
                    range_up_decreasing_i16_helper,
                    range_down_increasing_i16_helper,
                    range_down_decreasing_i16_helper,
                    range_increasing_i16_helper,
                    range_decreasing_i16_helper,
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
                    range_increasing_i16_fail,
                    range_decreasing_i16_fail,
                    i16::max_value());
test_integer_range!(i32,
                    "i32",
                    range_up_increasing_i32,
                    range_up_decreasing_i32,
                    range_down_increasing_i32,
                    range_down_decreasing_i32,
                    range_increasing_i32,
                    range_decreasing_i32,
                    i32s_increasing,
                    i32s_decreasing,
                    positive_i32s,
                    i32s,
                    range_up_increasing_i32_helper,
                    range_up_decreasing_i32_helper,
                    range_down_increasing_i32_helper,
                    range_down_decreasing_i32_helper,
                    range_increasing_i32_helper,
                    range_decreasing_i32_helper,
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
                    range_increasing_i32_fail,
                    range_decreasing_i32_fail,
                    i32::max_value());
test_integer_range!(i64,
                    "i64",
                    range_up_increasing_i64,
                    range_up_decreasing_i64,
                    range_down_increasing_i64,
                    range_down_decreasing_i64,
                    range_increasing_i64,
                    range_decreasing_i64,
                    i64s_increasing,
                    i64s_decreasing,
                    positive_i64s,
                    i64s,
                    range_up_increasing_i64_helper,
                    range_up_decreasing_i64_helper,
                    range_down_increasing_i64_helper,
                    range_down_decreasing_i64_helper,
                    range_increasing_i64_helper,
                    range_decreasing_i64_helper,
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
                    range_increasing_i64_fail,
                    range_decreasing_i64_fail,
                    i64::max_value());

macro_rules! test_integer_range_i {
    (
        $t: ty,
        $ts: expr,
        $rui: ident,
        $rud: ident,
        $rdi: ident,
        $rdd: ident,
        $ri: ident,
        $rd: ident,
        $neg: ident,
        $nat: ident,
        $nz: ident,
        $rui_th: ident,
        $rud_th: ident,
        $rdi_th: ident,
        $rdd_th: ident,
        $ri_th: ident,
        $rd_th: ident,
        $rui_t: ident,
        $rud_t: ident,
        $rdi_t: ident,
        $rdd_t: ident,
        $ri_t: ident,
        $rd_t: ident,
        $neg_t: ident,
        $nat_t: ident,
        $nz_t: ident,
        $ri_f: ident,
        $rd_f: ident,
        $min: expr
    ) => {
        #[test]
        fn $rui_t() {
            let (eo, p, _) = prepare_test();
            let s = "exhaustive_range_up_increasing";
            $rui_th(&eo, &p, &format!("{}_{}_iv", s, $ts), -5);
            $rui_th(&eo, &p, &format!("{}_{}_v", s, $ts), $min);
        }

        #[test]
        fn $rud_t() {
            let (eo, p, _) = prepare_test();
            let s = "exhaustive_range_up_decreasing";
            $rud_th(&eo, &p, &format!("{}_{}_iv", s, $ts), -5);
            $rud_th(&eo, &p, &format!("{}_{}_v", s, $ts), $min);
        }

        #[test]
        fn $rdi_t() {
            let (eo, p, _) = prepare_test();
            let s = "exhaustive_range_down_increasing";
            $rdi_th(&eo, &p, &format!("{}_{}_iv", s, $ts), -5);
            $rdi_th(&eo, &p, &format!("{}_{}_v", s, $ts), $min);
        }

        #[test]
        fn $rdd_t() {
            let (eo, p, _) = prepare_test();
            let s = "exhaustive_range_down_decreasing";
            $rdd_th(&eo, &p, &format!("{}_{}_iv", s, $ts), -5);
            $rdd_th(&eo, &p, &format!("{}_{}_v", s, $ts), $min);
        }

        #[test]
        fn $ri_t() {
            let (eo, p, _) = prepare_test();
            let s = "exhaustive_range_increasing";
            $ri_th(&eo, &p, &format!("{}_{}_v", s, $ts), -10, -10);
            $ri_th(&eo, &p, &format!("{}_{}_vi", s, $ts), -20, -10);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $ri_f() {
            IteratorProvider::Exhaustive.$ri(10, 9);
        }

        #[test]
        fn $rd_t() {
            let (eo, p, _) = prepare_test();
            let s = "exhaustive_range_decreasing";
            $rd_th(&eo, &p, &format!("{}_{}_v", s, $ts), -10, -10);
            $rd_th(&eo, &p, &format!("{}_{}_vi", s, $ts), -20, -10);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
        fn $rd_f() {
            IteratorProvider::Exhaustive.$rd(-9, -10);
        }

        #[test]
        fn $neg_t() {
            let (eo, ep, rp) = prepare_test();
            eo.match_vec(&format!("exhaustive_negative_{}s", $ts), &mut ep.$neg());
            eo.match_vec_f(&format!("random_negative_{}s", $ts), &mut rp.$neg());
        }

        #[test]
        fn $nat_t() {
            let (eo, ep, rp) = prepare_test();
            eo.match_vec(&format!("exhaustive_natural_{}s", $ts), &mut ep.$nat());
            eo.match_vec_f(&format!("random_natural_{}s", $ts), &mut rp.$nat());
        }

        #[test]
        fn $nz_t() {
            let (eo, p, _) = prepare_test();
            eo.match_vec(&format!("exhaustive_nonzero_{}s", $ts), &mut p.$nz());
        }
    }
}

test_integer_range_i!(i8,
                      "i8",
                      range_up_increasing_i8,
                      range_up_decreasing_i8,
                      range_down_increasing_i8,
                      range_down_decreasing_i8,
                      range_increasing_i8,
                      range_decreasing_i8,
                      negative_i8s,
                      natural_i8s,
                      nonzero_i8s,
                      range_up_increasing_i8_helper,
                      range_up_decreasing_i8_helper,
                      range_down_increasing_i8_helper,
                      range_down_decreasing_i8_helper,
                      range_increasing_i8_helper,
                      range_decreasing_i8_helper,
                      test_range_up_increasing_i8_i,
                      test_range_up_decreasing_i8_i,
                      test_range_down_increasing_i8_i,
                      test_range_down_decreasing_i8_i,
                      test_range_increasing_i8_i,
                      test_range_decreasing_i8_i,
                      test_negative_i8s,
                      test_natural_i8s,
                      test_nonzero_i8s,
                      range_increasing_i8_fail_i,
                      range_decreasing_i8_fail_i,
                      i8::min_value());
test_integer_range_i!(i16,
                      "i16",
                      range_up_increasing_i16,
                      range_up_decreasing_i16,
                      range_down_increasing_i16,
                      range_down_decreasing_i16,
                      range_increasing_i16,
                      range_decreasing_i16,
                      negative_i16s,
                      natural_i16s,
                      nonzero_i16s,
                      range_up_increasing_i16_helper,
                      range_up_decreasing_i16_helper,
                      range_down_increasing_i16_helper,
                      range_down_decreasing_i16_helper,
                      range_increasing_i16_helper,
                      range_decreasing_i16_helper,
                      test_range_up_increasing_i16_i,
                      test_range_up_decreasing_i16_i,
                      test_range_down_increasing_i16_i,
                      test_range_down_decreasing_i16_i,
                      test_range_increasing_i16_i,
                      test_range_decreasing_i16_i,
                      test_negative_i16s,
                      test_natural_i16s,
                      test_nonzero_i16s,
                      range_increasing_i16_fail_i,
                      range_decreasing_i16_fail_i,
                      i16::min_value());
test_integer_range_i!(i32,
                      "i32",
                      range_up_increasing_i32,
                      range_up_decreasing_i32,
                      range_down_increasing_i32,
                      range_down_decreasing_i32,
                      range_increasing_i32,
                      range_decreasing_i32,
                      negative_i32s,
                      natural_i32s,
                      nonzero_i32s,
                      range_up_increasing_i32_helper,
                      range_up_decreasing_i32_helper,
                      range_down_increasing_i32_helper,
                      range_down_decreasing_i32_helper,
                      range_increasing_i32_helper,
                      range_decreasing_i32_helper,
                      test_range_up_increasing_i32_i,
                      test_range_up_decreasing_i32_i,
                      test_range_down_increasing_i32_i,
                      test_range_down_decreasing_i32_i,
                      test_range_increasing_i32_i,
                      test_range_decreasing_i32_i,
                      test_negative_i32s,
                      test_natural_i32s,
                      test_nonzero_i32s,
                      range_increasing_i32_fail_i,
                      range_decreasing_i32_fail_i,
                      i32::min_value());
test_integer_range_i!(i64,
                      "i64",
                      range_up_increasing_i64,
                      range_up_decreasing_i64,
                      range_down_increasing_i64,
                      range_down_decreasing_i64,
                      range_increasing_i64,
                      range_decreasing_i64,
                      negative_i64s,
                      natural_i64s,
                      nonzero_i64s,
                      range_up_increasing_i64_helper,
                      range_up_decreasing_i64_helper,
                      range_down_increasing_i64_helper,
                      range_down_decreasing_i64_helper,
                      range_increasing_i64_helper,
                      range_decreasing_i64_helper,
                      test_range_up_increasing_i64_i,
                      test_range_up_decreasing_i64_i,
                      test_range_down_increasing_i64_i,
                      test_range_down_decreasing_i64_i,
                      test_range_increasing_i64_i,
                      test_range_decreasing_i64_i,
                      test_negative_i64s,
                      test_natural_i64s,
                      test_nonzero_i64s,
                      range_increasing_i64_fail_i,
                      range_decreasing_i64_fail_i,
                      i64::min_value());
