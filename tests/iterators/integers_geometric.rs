use common::get_expected_test_outputs;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::*;

#[test]
fn test_range_up_geometric_u32() {
    let eo = get_expected_test_outputs();
    let test = |number, scale, min| {
        eo.match_vec_f(
            &format!("random_range_up_geometric_u32_{}", number),
            &mut range_up_geometric_u32(&EXAMPLE_SEED, scale, min),
        )
    };
    test("i", 0, 2);
    test("ii", 100, 2);
    test("iii", 0, 100);
    test("iv", 100, 100);
}

#[test]
fn test_range_up_geometric_i32() {
    let eo = get_expected_test_outputs();
    let test = |number, scale, min| {
        eo.match_vec_f(
            &format!("random_range_up_geometric_i32_{}", number),
            &mut range_up_geometric_i32(&EXAMPLE_SEED, scale, min),
        )
    };
    test("i", 0, 2);
    test("ii", 100, 2);
    test("iii", 0, 100);
    test("iv", 100, 100);
    test("v", 0, -100);
    test("vi", 100, -100);
}

#[test]
fn test_range_down_geometric() {
    let eo = get_expected_test_outputs();
    let test = |number, scale, min| {
        eo.match_vec_f(
            &format!("random_range_down_geometric_{}", number),
            &mut range_down_geometric(&EXAMPLE_SEED, scale, min),
        )
    };
    test("i", 0, 2);
    test("ii", 100, 2);
    test("iii", 0, 100);
    test("iv", 100, 100);
    test("v", 0, -100);
    test("vi", 100, -100);
}
