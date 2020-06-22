use std::char;

use malachite_base::exhaustive::range::{range_decreasing, range_increasing};

use common::get_expected_test_outputs;
use rust_wheels::iterators::chars::*;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random;

#[test]
fn test_chars_increasing() {
    let eo = get_expected_test_outputs();
    eo.match_vec_debug("exhaustive_chars_increasing", &mut chars_increasing());
}

#[test]
fn test_chars_decreasing() {
    let eo = get_expected_test_outputs();
    eo.match_vec_debug("exhaustive_chars_decreasing", &mut chars_decreasing());
}

#[test]
fn test_ascii_chars_increasing() {
    let eo = get_expected_test_outputs();
    eo.match_vec_debug(
        "exhaustive_ascii_chars_increasing",
        &mut ascii_chars_increasing(),
    );
}

#[test]
fn test_ascii_chars_decreasing() {
    let eo = get_expected_test_outputs();
    eo.match_vec_debug(
        "exhaustive_ascii_chars_decreasing",
        &mut ascii_chars_decreasing(),
    );
}

#[test]
fn test_range_up_increasing_char() {
    let eo = get_expected_test_outputs();
    let test = |number, a| {
        eo.match_vec_debug(
            &format!("exhaustive_range_up_increasing_char_{}", number),
            &mut range_up_increasing_char(a),
        )
    };
    test("i", '\0');
    test("ii", 'a');
    test("iii", 'Ш');
    test("iv", '\u{D7FF}');
    test("v", char::MAX);
}

#[test]
fn test_range_up_decreasing_char() {
    let eo = get_expected_test_outputs();
    let test = |number, a| {
        eo.match_vec_debug(
            &format!("exhaustive_range_up_decreasing_char_{}", number),
            &mut range_up_decreasing_char(a),
        )
    };
    test("i", '\0');
    test("ii", 'a');
    test("iii", 'Ш');
    test("iv", '\u{D7FF}');
    test("v", char::MAX);
}

#[test]
fn test_range_down_increasing_char() {
    let eo = get_expected_test_outputs();
    let test = |number, a| {
        eo.match_vec_debug(
            &format!("exhaustive_range_down_increasing_char_{}", number),
            &mut range_down_increasing_char(a),
        )
    };
    test("i", '\0');
    test("ii", 'a');
    test("iii", 'Ш');
    test("iv", '\u{E000}');
    test("v", char::MAX);
}

#[test]
fn test_range_down_decreasing_char() {
    let eo = get_expected_test_outputs();
    let test = |number, a| {
        eo.match_vec_debug(
            &format!("exhaustive_range_down_decreasing_char_{}", number),
            &mut range_down_decreasing_char(a),
        )
    };
    test("i", '\0');
    test("ii", 'a');
    test("iii", 'Ш');
    test("iv", '\u{E000}');
    test("v", char::MAX);
}

#[test]
fn test_range_increasing_char() {
    let eo = get_expected_test_outputs();
    let test = |number, a, b| {
        eo.match_vec_debug(
            &format!("exhaustive_range_increasing_char_{}", number),
            &mut range_increasing(a, b),
        )
    };
    test("i", 'a', 'z');
    test("ii", 'a', 'a');
    test("iii", '!', '9');
    test("iv", '\u{D7FF}', '\u{E000}');
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: a, b: A")]
fn range_increasing_char_fail() {
    range_increasing('a', 'A');
}

#[test]
fn test_range_decreasing_char() {
    let eo = get_expected_test_outputs();
    let test = |number, a, b| {
        eo.match_vec_debug(
            &format!("exhaustive_range_decreasing_char_{}", number),
            &mut range_decreasing(a, b),
        )
    };
    test("i", 'a', 'z');
    test("ii", 'a', 'a');
    test("iii", '!', '9');
    test("iv", '\u{D7FF}', '\u{E000}');
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: a, b: A")]
fn range_decreasing_char_fail() {
    range_decreasing('a', 'A');
}

#[test]
fn test_chars() {
    let eo = get_expected_test_outputs();
    eo.match_vec_debug("exhaustive_chars", &mut exhaustive_chars());
    eo.match_vec_f_debug("random_chars", &mut random::<char>(&EXAMPLE_SEED));
}

#[test]
fn test_ascii_chars() {
    let eo = get_expected_test_outputs();
    eo.match_vec_debug("exhaustive_ascii_chars", &mut exhaustive_ascii_chars());
    eo.match_vec_f_debug("random_ascii_chars", &mut random_ascii_chars(&EXAMPLE_SEED));
}

#[test]
fn test_range_up_char() {
    let eo = get_expected_test_outputs();
    let e_test = |number, a| {
        eo.match_vec_debug(
            &format!("exhaustive_range_up_char_{}", number),
            &mut exhaustive_range_up_char(a),
        )
    };
    e_test("i", '\0');
    e_test("ii", 'a');
    e_test("iii", 'Ш');
    e_test("iv", '\u{D7FF}');
    e_test("v", char::MAX);
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
    let e_test = |number, a| {
        eo.match_vec_debug(
            &format!("exhaustive_range_down_char_{}", number),
            &mut exhaustive_range_down_char(a),
        )
    };
    e_test("i", '\0');
    e_test("ii", 'a');
    e_test("iii", 'Ш');
    e_test("iv", '\u{E000}');
    e_test("v", char::MAX);
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
    let e_test = |number, a, b| {
        eo.match_vec_debug(
            &format!("exhaustive_range_char_{}", number),
            &mut range_increasing(a, b),
        )
    };
    e_test("i", 'a', 'z');
    e_test("ii", 'a', 'a');
    e_test("iii", '!', '9');
    e_test("iv", '\u{D7FF}', '\u{E000}');
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
fn range_char_fail_1() {
    range_increasing('a', 'A');
}

#[test]
#[should_panic(expected = "a must be less than or equal to b. a: a, b: A")]
fn range_char_fail_2() {
    random_range_char(&EXAMPLE_SEED, 'a', 'A');
}
