extern crate rust_wheels_lib;

use common;
use common::TestOutput;
use self::rust_wheels_lib::iterators::iterator_provider::IteratorProvider;

macro_rules! test_integer_range_aux {
    ($eo: ident, $p: ident, $rui_t: ident, $rud_t: ident, $rdi_t: ident, $rdd_t: ident) => {
        $rui_t(&$eo, &mut $p);
        $rud_t(&$eo, &mut $p);
        $rdi_t(&$eo, &mut $p);
        $rdd_t(&$eo, &mut $p);
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
                            test_range_down_decreasing_u8);
}

macro_rules! test_integer_range {
    (
        $t: ty,
        $ts: expr,
        $rui: ident,
        $rud: ident,
        $rdi: ident,
        $rdd: ident,
        $rui_th: ident,
        $rud_th: ident,
        $rdi_th: ident,
        $rdd_th: ident,
        $rui_t: ident,
        $rud_t: ident,
        $rdi_t: ident,
        $rdd_t: ident,
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
    }
}

test_integer_range!(u8,
                    "u8",
                    range_up_increasing_u8,
                    range_up_decreasing_u8,
                    range_down_increasing_u8,
                    range_down_decreasing_u8,
                    range_up_increasing_u8_helper,
                    range_up_decreasing_u8_helper,
                    range_down_increasing_u8_helper,
                    range_down_decreasing_u8_helper,
                    test_range_up_increasing_u8,
                    test_range_up_decreasing_u8,
                    test_range_down_increasing_u8,
                    test_range_down_decreasing_u8,
                    u8::max_value());
