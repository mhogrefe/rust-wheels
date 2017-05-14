use rust_wheels::iterators::integers::*;

use common::get_expected_test_outputs;
use malachite::integer::Integer;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use std::str::FromStr;

#[test]
fn test_range_up_increasing_integer() {
    let eo = get_expected_test_outputs();
    let test = |number, a| {
        eo.match_vec(&format!("exhaustive_range_up_increasing_integer_{}", number),
                     &mut range_up_increasing_integer(Integer::from_str(a).unwrap()))
    };
    test("i", "0");
    test("ii", "5");
    test("iii", "-5");
}

#[test]
fn test_range_down_decreasing_integer() {
    let eo = get_expected_test_outputs();
    let test = |number, a| {
        eo.match_vec(&format!("exhaustive_range_down_decreasing_integer_{}", number),
                     &mut range_down_decreasing_integer(Integer::from_str(a).unwrap()))
    };
    test("i", "0");
    test("ii", "5");
    test("iii", "-5");
}

#[test]
fn test_range_increasing_integer() {
    let eo = get_expected_test_outputs();
    let test = |number, a, b| {
        eo.match_vec(&format!("exhaustive_range_increasing_integer_{}", number),
                     &mut range_increasing_integer(Integer::from_str(a).unwrap(),
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

fn range_increasing_integer_fail_helper(a: &str, b: &str) {
    range_increasing_integer(Integer::from_str(a).unwrap(), Integer::from_str(b).unwrap());
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
        eo.match_vec(&format!("exhaustive_range_decreasing_integer_{}", number),
                     &mut range_decreasing_integer(Integer::from_str(a).unwrap(),
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

fn range_decreasing_integer_fail_helper(a: &str, b: &str) {
    range_decreasing_integer(Integer::from_str(a).unwrap(), Integer::from_str(b).unwrap());
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
}

#[test]
fn test_positive_integers() {
    let eo = get_expected_test_outputs();
    eo.match_vec("exhaustive_positive_integers",
                 &mut exhaustive_positive_integers());

    let test = |number, scale| {
        eo.match_vec_f(&format!("random_positive_integers_{}", number),
                       &mut random_positive_integers(&EXAMPLE_SEED[..], scale))
    };
    test("i", 0);
    test("ii", 1);
    test("iii", 2);
    test("iv", 3);
    test("v", 10);
    test("vi", 100);
}

#[test]
fn test_natural_integers() {
    let eo = get_expected_test_outputs();
    eo.match_vec("exhaustive_natural_integers",
                 &mut exhaustive_natural_integers());

    let test = |number, scale| {
        eo.match_vec_f(&format!("random_natural_integers_{}", number),
                       &mut random_natural_integers(&EXAMPLE_SEED[..], scale))
    };
    test("i", 0);
    test("ii", 1);
    test("iii", 2);
    test("iv", 3);
    test("v", 10);
    test("vi", 100);
}

#[test]
fn test_range_integer() {
    let eo = get_expected_test_outputs();
    let e_test = |number, a, b| {
        eo.match_vec(&format!("exhaustive_range_integer_{}", number),
                     &mut exhaustive_range_integer(Integer::from_str(a).unwrap(),
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
                       &mut random_range_integer(&EXAMPLE_SEED[..],
                                                 Integer::from_str(a).unwrap(),
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

fn exhaustive_range_integer_fail_helper(a: &str, b: &str) {
    exhaustive_range_integer(Integer::from_str(a).unwrap(), Integer::from_str(b).unwrap());
}

fn random_range_integer_fail_helper(a: &str, b: &str) {
    random_range_integer(&EXAMPLE_SEED[..],
                         Integer::from_str(a).unwrap(),
                         Integer::from_str(b).unwrap());
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
fn range_integer_fail_1() {
    exhaustive_range_integer_fail_helper("10", "9")
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: -9, b: -10")]
fn range_integer_fail_2() {
    exhaustive_range_integer_fail_helper("-9", "-10")
}

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
