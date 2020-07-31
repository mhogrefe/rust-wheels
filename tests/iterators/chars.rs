use std::char;

use common::get_expected_test_outputs;
use rust_wheels::iterators::chars::*;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random;

#[test]
fn test_chars() {
    let eo = get_expected_test_outputs();
    eo.match_vec_f_debug("random_chars", &mut random::<char>(&EXAMPLE_SEED));
}

#[test]
fn test_ascii_chars() {
    let eo = get_expected_test_outputs();
    eo.match_vec_f_debug("random_ascii_chars", &mut random_ascii_chars(&EXAMPLE_SEED));
}

#[test]
fn test_range_up_char() {
    let eo = get_expected_test_outputs();
    let r_test = |number, a| {
        eo.match_vec_f_debug(
            &format!("random_range_up_char_{}", number),
            &mut random_range_up_char(&EXAMPLE_SEED, a),
        )
    };
    r_test("i", '\0');
    r_test("ii", 'a');
    r_test("iii", 'Ш');
    r_test("iv", '\u{D7FF}');
    r_test("v", char::MAX);
}

#[test]
fn test_range_down_char() {
    let eo = get_expected_test_outputs();
    let r_test = |number, a| {
        eo.match_vec_f_debug(
            &format!("random_range_down_char_{}", number),
            &mut random_range_down_char(&EXAMPLE_SEED, a),
        )
    };
    r_test("i", '\0');
    r_test("ii", 'a');
    r_test("iii", 'Ш');
    r_test("iv", '\u{E000}');
    r_test("v", char::MAX);
}

#[test]
fn test_range_char() {
    let eo = get_expected_test_outputs();
    let r_test = |number, a, b| {
        eo.match_vec_f_debug(
            &format!("random_range_char_{}", number),
            &mut random_range_char(&EXAMPLE_SEED, a, b),
        )
    };
    r_test("i", 'a', 'z');
    r_test("ii", 'a', 'a');
    r_test("iii", '!', '9');
    r_test("iv", '\u{D7FF}', '\u{E000}');
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: a, b: A")]
fn range_char_fail_2() {
    random_range_char(&EXAMPLE_SEED, 'a', 'A');
}
