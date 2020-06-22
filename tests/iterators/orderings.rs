use common::get_expected_test_outputs;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::orderings::*;

#[test]
fn test_orderings() {
    let eo = get_expected_test_outputs();
    eo.match_vec_f_debug("random_orderings", &mut random_orderings(&EXAMPLE_SEED));
}
