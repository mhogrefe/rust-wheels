use rust_wheels::iterators::general::*;

use common::get_expected_test_outputs;
use rust_wheels::io::readers::parse_vec;
use rust_wheels::iterators::common::EXAMPLE_SEED;

#[test]
fn test_random_from_vector() {
    let eo = get_expected_test_outputs();
    let test = |number, xs| {
        let xs: Vec<u32> = parse_vec(xs).unwrap();
        eo.match_vec_f(
            &format!("random_from_vector_{}", number),
            &mut random_from_vector(&EXAMPLE_SEED[..], xs),
        );
    };
    test("i", "[5]");
    test("ii", "[1, 2, 3]");
    test("iii", "[3, 1, 4, 1]");
}

#[test]
#[should_panic(expected = "Cannot randomly generate values from an empty Vec.")]
fn random_from_vector_fail() {
    random_from_vector(&EXAMPLE_SEED[..], Vec::<u32>::new());
}
