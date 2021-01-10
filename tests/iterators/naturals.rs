use std::str::FromStr;

use malachite_nz::natural::Natural;

use common::get_expected_test_outputs;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::naturals::*;

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
