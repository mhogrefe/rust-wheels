use rust_wheels::iterators::rounding_modes::*;

use common::get_expected_test_outputs;
use rust_wheels::iterators::common::EXAMPLE_SEED;

#[test]
fn test_rounding_modes() {
    let eo = get_expected_test_outputs();
    eo.match_vec_debug(
        "exhaustive_rounding_modes",
        &mut exhaustive_rounding_modes(),
    );
    eo.match_vec_f_debug(
        "random_rounding_modes",
        &mut random_rounding_modes(&EXAMPLE_SEED[..]),
    );
}
