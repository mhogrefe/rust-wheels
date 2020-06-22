use common::get_expected_test_outputs;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random;

#[test]
fn test_bools() {
    let eo = get_expected_test_outputs();
    eo.match_vec_f("random_bools", &mut random::<bool>(&EXAMPLE_SEED));
}
