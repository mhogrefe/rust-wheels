use std::str::FromStr;

use malachite_nz::natural::Natural;

use common::get_expected_test_outputs;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::*;

/*
#[test]
fn test_range_increasing_natural() {
    let eo = get_expected_test_outputs();
    let test = |number, a, b| {
        eo.match_vec(
            &format!("exhaustive_range_increasing_natural_{}", number),
            &mut range_increasing(Natural::from_str(a).unwrap(), Natural::from_str(b).unwrap()),
        )
    };
    test("i", "0", "0");
    test("ii", "0", "10");
    test("iii", "10", "20");
    test("iv", "10", "10");
}

fn range_increasing_natural_fail_helper(a: &str, b: &str) {
    range_increasing(Natural::from_str(a).unwrap(), Natural::from_str(b).unwrap());
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
fn range_increasing_natural_fail() {
    range_increasing_natural_fail_helper("10", "9")
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
        eo.match_vec(
            &format!("exhaustive_range_up_increasing_natural_{}", number),
            &mut range_up_increasing_natural(Natural::from_str(a).unwrap()),
        )
    };
    test("i", "0");
    test("ii", "5");
}
*/

#[test]
fn test_positive_naturals() {
    let eo = get_expected_test_outputs();
    let test = |number, scale| {
        eo.match_vec_f(
            &format!("random_positive_naturals_{}", number),
            &mut random_positive_naturals(&EXAMPLE_SEED, scale),
        );
        // eo.match_vec_f_binary(
        //     &format!("special_random_positive_naturals_{}", number),
        //     &mut special_random_positive_naturals(&EXAMPLE_SEED, scale),
        // );
    };
    test("i", 0);
    test("ii", 1);
    test("iii", 2);
    test("iv", 3);
    test("v", 10);
    test("vi", 100);
}

#[test]
fn test_naturals() {
    let eo = get_expected_test_outputs();
    let test = |number, scale| {
        eo.match_vec_f(
            &format!("random_naturals_{}", number),
            &mut random_naturals(&EXAMPLE_SEED, scale),
        );
        // eo.match_vec_f_binary(
        //     &format!("special_random_naturals_{}", number),
        //     &mut special_random_naturals(&EXAMPLE_SEED, scale),
        // );
    };
    test("i", 0);
    test("ii", 1);
    test("iii", 2);
    test("iv", 3);
    test("v", 10);
    test("vi", 100);
}

#[test]
fn test_range_natural() {
    let eo = get_expected_test_outputs();
    let e_test = |number, a, b| {
        eo.match_vec_f(
            &format!("random_range_natural_{}", number),
            &mut random_range_natural(
                &EXAMPLE_SEED,
                Natural::from_str(a).unwrap(),
                Natural::from_str(b).unwrap(),
            ),
        );
    };
    e_test("i", "0", "0");
    e_test("ii", "0", "10");
    e_test("iii", "10", "20");
    e_test("iv", "10", "10");
}

fn random_range_natural_fail_helper(a: &str, b: &str) {
    random_range_natural(
        &EXAMPLE_SEED,
        Natural::from_str(a).unwrap(),
        Natural::from_str(b).unwrap(),
    );
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: 10, b: 9")]
fn range_natural_fail() {
    random_range_natural_fail_helper("10", "9")
}

#[test]
fn test_range_up_natural() {
    let eo = get_expected_test_outputs();
    let e_test = |number, scale, a| {
        eo.match_vec_f(
            &format!("random_range_up_natural_{}", number),
            &mut random_range_up_natural(&EXAMPLE_SEED, scale, Natural::from_str(a).unwrap()),
        );
    };
    e_test("i", 0, "0");
    e_test("ii", 10, "0");
    e_test("iii", 0, "16");
    e_test("iv", 10, "16");
    e_test("v", 0, "1000000000000");
    e_test("vi", 10, "1000000000000");
}
