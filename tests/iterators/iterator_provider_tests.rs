extern crate rust_wheels_lib;

use common;
use common::TestOutput;
use self::rust_wheels_lib::iterators::iterator_provider::IteratorProvider;

macro_rules! test_integer_range_aux {
    (
            $eo: ident,
            $p: ident,
            $rui_t: ident,
            $rud_t: ident,
            $rdi_t: ident,
            $rdd_t: ident,
            $ri_t: ident,
            $rd_t: ident,
            $i_t: ident,
            $d_t: ident
    ) => {
        $rui_t(&$eo, &mut $p);
        $rud_t(&$eo, &mut $p);
        $rdi_t(&$eo, &mut $p);
        $rdd_t(&$eo, &mut $p);
        $ri_t(&$eo, &mut $p);
        $rd_t(&$eo, &mut $p);
        $i_t(&$eo, &mut $p);
        $d_t(&$eo, &mut $p);
    }
}

#[test]
fn master_test() {
    let eo = common::get_expected_test_outputs();
    let mut p = IteratorProvider::Exhaustive;

    test_integer_range_aux!(eo,
                            p,
                            test_range_up_increasing_u8,
                            test_range_up_decreasing_u8,
                            test_range_down_increasing_u8,
                            test_range_down_decreasing_u8,
                            test_range_increasing_u8,
                            test_range_decreasing_u8,
                            test_u8s_increasing,
                            test_u8s_decreasing);
    test_integer_range_aux!(eo,
                            p,
                            test_range_up_increasing_u16,
                            test_range_up_decreasing_u16,
                            test_range_down_increasing_u16,
                            test_range_down_decreasing_u16,
                            test_range_increasing_u16,
                            test_range_decreasing_u16,
                            test_u16s_increasing,
                            test_u16s_decreasing);
    test_integer_range_aux!(eo,
                            p,
                            test_range_up_increasing_u32,
                            test_range_up_decreasing_u32,
                            test_range_down_increasing_u32,
                            test_range_down_decreasing_u32,
                            test_range_increasing_u32,
                            test_range_decreasing_u32,
                            test_u32s_increasing,
                            test_u32s_decreasing);
    test_integer_range_aux!(eo,
                            p,
                            test_range_up_increasing_u64,
                            test_range_up_decreasing_u64,
                            test_range_down_increasing_u64,
                            test_range_down_decreasing_u64,
                            test_range_increasing_u64,
                            test_range_decreasing_u64,
                            test_u64s_increasing,
                            test_u64s_decreasing);
    test_integer_range_aux!(eo,
                            p,
                            test_range_up_increasing_i8,
                            test_range_up_decreasing_i8,
                            test_range_down_increasing_i8,
                            test_range_down_decreasing_i8,
                            test_range_increasing_i8,
                            test_range_decreasing_i8,
                            test_i8s_increasing,
                            test_i8s_decreasing);
    test_integer_range_aux!(eo,
                            p,
                            test_range_up_increasing_i16,
                            test_range_up_decreasing_i16,
                            test_range_down_increasing_i16,
                            test_range_down_decreasing_i16,
                            test_range_increasing_i16,
                            test_range_decreasing_i16,
                            test_i16s_increasing,
                            test_i16s_decreasing);
    test_integer_range_aux!(eo,
                            p,
                            test_range_up_increasing_i32,
                            test_range_up_decreasing_i32,
                            test_range_down_increasing_i32,
                            test_range_down_decreasing_i32,
                            test_range_increasing_i32,
                            test_range_decreasing_i32,
                            test_i32s_increasing,
                            test_i32s_decreasing);
    test_integer_range_aux!(eo,
                            p,
                            test_range_up_increasing_i64,
                            test_range_up_decreasing_i64,
                            test_range_down_increasing_i64,
                            test_range_down_decreasing_i64,
                            test_range_increasing_i64,
                            test_range_decreasing_i64,
                            test_i64s_increasing,
                            test_i64s_decreasing);
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
        $ri_f: ident,
        $rd_f: ident,
        $max: expr
    ) => {
        fn $rui_th(eo: &TestOutput, p: &mut IteratorProvider, key: &str, a: $t) {
            eo.match_list(key, &mut p.$rui(a));
        }

        fn $rui_t(eo: &TestOutput, p: &mut IteratorProvider) {
            let s = "exhaustive_range_up_increasing";
            $rui_th(eo, p, &format!("{}_{}_i", s, $ts), 0);
            $rui_th(eo, p, &format!("{}_{}_ii", s, $ts), 5);
            $rui_th(eo, p, &format!("{}_{}_iii", s, $ts), $max);
        }

        fn $rud_th(eo: &TestOutput, p: &mut IteratorProvider, key: &str, a: $t) {
            eo.match_list(key, &mut p.$rud(a));
        }

        fn $rud_t(eo: &TestOutput, p: &mut IteratorProvider) {
            let s = "exhaustive_range_up_decreasing";
            $rud_th(eo, p, &format!("{}_{}_i", s, $ts), 0);
            $rud_th(eo, p, &format!("{}_{}_ii", s, $ts), 5);
            $rud_th(eo, p, &format!("{}_{}_iii", s, $ts), $max);
        }

        fn $rdi_th(eo: &TestOutput, p: &mut IteratorProvider, key: &str, a: $t) {
            eo.match_list(key, &mut p.$rdi(a));
        }

        fn $rdi_t(eo: &TestOutput, p: &mut IteratorProvider) {
            let s = "exhaustive_range_down_increasing";
            $rdi_th(eo, p, &format!("{}_{}_i", s, $ts), 0);
            $rdi_th(eo, p, &format!("{}_{}_ii", s, $ts), 5);
            $rdi_th(eo, p, &format!("{}_{}_iii", s, $ts), $max);
        }

        fn $rdd_th(eo: &TestOutput, p: &mut IteratorProvider, key: &str, a: $t) {
            eo.match_list(key, &mut p.$rdd(a));
        }

        fn $rdd_t(eo: &TestOutput, p: &mut IteratorProvider) {
            let s = "exhaustive_range_down_decreasing";
            $rdd_th(eo, p, &format!("{}_{}_i", s, $ts), 0);
            $rdd_th(eo, p, &format!("{}_{}_ii", s, $ts), 5);
            $rdd_th(eo, p, &format!("{}_{}_iii", s, $ts), $max);
        }

        fn $ri_th(eo: &TestOutput, p: &mut IteratorProvider, key: &str, a: $t, b: $t) {
            eo.match_list(key, &mut p.$ri(a, b));
        }

        fn $ri_t(eo: &TestOutput, p: &mut IteratorProvider) {
            let s = "exhaustive_range_increasing";
            $ri_th(eo, p, &format!("{}_{}_i", s, $ts), 0, 0);
            $ri_th(eo, p, &format!("{}_{}_ii", s, $ts), 0, 10);
            $ri_th(eo, p, &format!("{}_{}_iii", s, $ts), 10, 20);
            $ri_th(eo, p, &format!("{}_{}_iv", s, $ts), 10, 10);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $ri_f() {
            IteratorProvider::Exhaustive.$ri(10, 9);
        }

        fn $rd_th(eo: &TestOutput, p: &mut IteratorProvider, key: &str, a: $t, b: $t) {
            eo.match_list(key, &mut p.$rd(a, b));
        }

        fn $rd_t(eo: &TestOutput, p: &mut IteratorProvider) {
            let s = "exhaustive_range_decreasing";
            $rd_th(eo, p, &format!("{}_{}_i", s, $ts), 0, 0);
            $rd_th(eo, p, &format!("{}_{}_ii", s, $ts), 0, 10);
            $rd_th(eo, p, &format!("{}_{}_iii", s, $ts), 10, 20);
            $rd_th(eo, p, &format!("{}_{}_iv", s, $ts), 10, 10);
        }

        #[test]
        #[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
        fn $rd_f() {
            IteratorProvider::Exhaustive.$rd(10, 9);
        }

        fn $i_t(eo: &TestOutput, p: &mut IteratorProvider) {
            eo.match_list(&format!("exhaustive_{}s_increasing", $ts), &mut p.$i());
        }

        fn $d_t(eo: &TestOutput, p: &mut IteratorProvider) {
            eo.match_list(&format!("exhaustive_{}s_decreasing", $ts), &mut p.$d());
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
                    range_increasing_i64_fail,
                    range_decreasing_i64_fail,
                    i64::max_value());
