use std::str::FromStr;

use malachite_nz::integer::Integer;

use common::get_expected_test_outputs;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers::*;

/*
#[test]
fn test_range_up_increasing_integer() {
    let eo = get_expected_test_outputs();
    let test = |number, a| {
        eo.match_vec(
            &format!("exhaustive_range_up_increasing_integer_{}", number),
            &mut range_up_increasing_integer(Integer::from_str(a).unwrap()),
        )
    };
    test("i", "0");
    test("ii", "5");
    test("iii", "-5");
}

#[test]
fn test_range_down_decreasing_integer() {
    let eo = get_expected_test_outputs();
    let test = |number, a| {
        eo.match_vec(
            &format!("exhaustive_range_down_decreasing_integer_{}", number),
            &mut range_down_decreasing_integer(Integer::from_str(a).unwrap()),
        )
    };
    test("i", "0");
    test("ii", "5");
    test("iii", "-5");
}

#[test]
fn test_range_increasing_integer() {
    let eo = get_expected_test_outputs();
    let test = |number, a, b| {
        eo.match_vec(
            &format!("exhaustive_range_increasing_integer_{}", number),
            &mut range_increasing(Integer::from_str(a).unwrap(), Integer::from_str(b).unwrap()),
        )
    };
    test("i", "0", "0");
    test("ii", "0", "10");
    test("iii", "10", "20");
    test("iv", "10", "10");
    test("v", "-10", "-10");
    test("vi", "-20", "-10");
    test("vii", "-100", "100");
}

fn range_increasing_integer_fail_helper(a: &str, b: &str) {
    range_increasing(Integer::from_str(a).unwrap(), Integer::from_str(b).unwrap());
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
fn range_increasing_integer_fail_1() {
    range_increasing_integer_fail_helper("10", "9")
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
fn range_increasing_integer_fail_2() {
    range_increasing_integer_fail_helper("-9", "-10")
}

#[test]
fn test_range_decreasing_integer() {
    let eo = get_expected_test_outputs();
    let test = |number, a, b| {
        eo.match_vec(
            &format!("exhaustive_range_decreasing_integer_{}", number),
            &mut range_decreasing(Integer::from_str(a).unwrap(), Integer::from_str(b).unwrap()),
        )
    };
    test("i", "0", "0");
    test("ii", "0", "10");
    test("iii", "10", "20");
    test("iv", "10", "10");
    test("v", "-10", "-10");
    test("vi", "-20", "-10");
    test("vii", "-100", "100");
}

fn range_decreasing_integer_fail_helper(a: &str, b: &str) {
    range_decreasing(Integer::from_str(a).unwrap(), Integer::from_str(b).unwrap());
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
fn range_decreasing_integer_fail_1() {
    range_decreasing_integer_fail_helper("10", "9")
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
fn range_decreasing_integer_fail_2() {
    range_decreasing_integer_fail_helper("-9", "-10")
}*/

#[test]
fn test_range_integer() {
    let eo = get_expected_test_outputs();
    let e_test = |number, a, b| {
        // eo.match_vec(
        //     &format!("exhaustive_range_integer_{}", number),
        //     &mut exhaustive_range_integer(
        //         Integer::from_str(a).unwrap(),
        //         Integer::from_str(b).unwrap(),
        //     ),
        // );
        eo.match_vec_f(
            &format!("random_range_integer_{}", number),
            &mut random_range_integer(
                &EXAMPLE_SEED,
                Integer::from_str(a).unwrap(),
                Integer::from_str(b).unwrap(),
            ),
        );
    };
    e_test("i", "0", "0");
    e_test("ii", "0", "10");
    e_test("iii", "10", "20");
    e_test("iv", "10", "10");
    e_test("v", "-10", "-10");
    e_test("vi", "-20", "-10");
    e_test("vii", "-100", "100");
}

/*
fn exhaustive_range_integer_fail_helper(a: &str, b: &str) {
    exhaustive_range_integer(Integer::from_str(a).unwrap(), Integer::from_str(b).unwrap());
}*/

fn random_range_integer_fail_helper(a: &str, b: &str) {
    random_range_integer(
        &EXAMPLE_SEED,
        Integer::from_str(a).unwrap(),
        Integer::from_str(b).unwrap(),
    );
}

/*
#[test]
#[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
fn range_integer_fail_1() {
    exhaustive_range_integer_fail_helper("10", "9")
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
fn range_integer_fail_2() {
    exhaustive_range_integer_fail_helper("-9", "-10")
}*/

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
fn range_integer_fail_3() {
    random_range_integer_fail_helper("10", "9")
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
fn range_integer_fail_4() {
    random_range_integer_fail_helper("-9", "-10")
}

#[test]
fn test_range_up_integer() {
    let eo = get_expected_test_outputs();
    let e_test = |number, scale, a| {
        eo.match_vec_f(
            &format!("random_range_up_integer_{}", number),
            &mut random_range_up_integer(&EXAMPLE_SEED, scale, Integer::from_str(a).unwrap()),
        );
    };
    e_test("i", 0, "0");
    e_test("ii", 10, "0");
    e_test("iii", 0, "16");
    e_test("iv", 10, "16");
    e_test("v", 0, "1000000000000");
    e_test("vi", 10, "1000000000000");
    e_test("vii", 0, "-16");
    e_test("viii", 10, "-16");
    e_test("ix", 0, "-1000000000000");
    e_test("x", 10, "-1000000000000");
}

#[test]
fn test_range_down_integer() {
    let eo = get_expected_test_outputs();
    let e_test = |number, scale, a| {
        eo.match_vec_f(
            &format!("random_range_down_integer_{}", number),
            &mut random_range_down_integer(&EXAMPLE_SEED, scale, Integer::from_str(a).unwrap()),
        );
    };
    e_test("i", 0, "0");
    e_test("ii", 10, "0");
    e_test("iii", 0, "16");
    e_test("iv", 10, "16");
    e_test("v", 0, "1000000000000");
    e_test("vi", 10, "1000000000000");
    e_test("vii", 0, "-16");
    e_test("viii", 10, "-16");
    e_test("ix", 0, "-1000000000000");
    e_test("x", 10, "-1000000000000");
}
