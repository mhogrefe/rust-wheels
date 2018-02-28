use rust_wheels::iterators::orderings::*;

use common::get_expected_test_outputs;
use rust_wheels::iterators::common::EXAMPLE_SEED;

#[test]
fn test_orderings_increasing() {
    let eo = get_expected_test_outputs();
    eo.match_vec_debug(
        "exhaustive_orderings_increasing",
        &mut orderings_increasing(),
    );
}

#[test]
fn test_orderings() {
    let eo = get_expected_test_outputs();
    eo.match_vec_debug("exhaustive_orderings", &mut exhaustive_orderings());
    eo.match_vec_f_debug("random_orderings", &mut random_orderings(&EXAMPLE_SEED));
}
