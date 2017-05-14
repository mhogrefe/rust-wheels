use rust_wheels::iterators::naturals::*;

use common::get_expected_test_outputs;
use malachite::natural::Natural;
use std::str::FromStr;

#[test]
fn test_range_increasing_natural() {
    let eo = get_expected_test_outputs();
    let test = |number, a, b| {
        eo.match_vec(&format!("exhaustive_range_increasing_natural_{}", number),
                     &mut range_increasing_natural(Natural::from_str(a).unwrap(),
                                                   Natural::from_str(b).unwrap()))
    };
    test("i", "0", "0");
    test("ii", "0", "10");
    test("iii", "10", "20");
    test("iv", "10", "10");
}

fn range_increasing_natural_fail_helper(a: &str, b: &str) {
    range_increasing_natural(Natural::from_str(a).unwrap(), Natural::from_str(b).unwrap());
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
fn range_increasing_natural_fail() {
    range_increasing_natural_fail_helper("10", "9")
}

#[test]
fn test_range_decreasing_natural() {
    let eo = get_expected_test_outputs();
    let test = |number, a, b| {
        eo.match_vec(&format!("exhaustive_range_decreasing_natural_{}", number),
                     &mut range_decreasing_natural(Natural::from_str(a).unwrap(),
                                                   Natural::from_str(b).unwrap()))
    };
    test("i", "0", "0");
    test("ii", "0", "10");
    test("iii", "10", "20");
    test("iv", "10", "10");
}

fn range_decreasing_natural_fail_helper(a: &str, b: &str) {
    range_decreasing_natural(Natural::from_str(a).unwrap(), Natural::from_str(b).unwrap());
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
fn range_decreasing_natural_fail() {
    range_decreasing_natural_fail_helper("10", "9")
}

#[test]
fn test_range_up_increasing_natural() {
    let eo = get_expected_test_outputs();
    let test = |number, a| {
        eo.match_vec(&format!("exhaustive_range_up_increasing_natural_{}", number),
                     &mut range_up_increasing_natural(Natural::from_str(a).unwrap()))
    };
    test("i", "0");
    test("ii", "5");
}

#[test]
fn test_range_down_increasing_natural() {
    let eo = get_expected_test_outputs();
    let test = |number, a| {
        eo.match_vec(&format!("exhaustive_range_down_increasing_natural_{}", number),
                     &mut range_down_increasing_natural(Natural::from_str(a).unwrap()))
    };
    test("i", "0");
    test("ii", "5");
}

#[test]
fn test_range_down_decreasing_natural() {
    let eo = get_expected_test_outputs();
    let test = |number, a| {
        eo.match_vec(&format!("exhaustive_range_down_decreasing_natural_{}", number),
                     &mut range_down_decreasing_natural(Natural::from_str(a).unwrap()))
    };
    test("i", "0");
    test("ii", "5");
}
