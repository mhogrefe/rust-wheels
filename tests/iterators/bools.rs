use rust_wheels::iterators::bools::*;

use common::get_expected_test_outputs;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;

#[test]
fn test_bools() {
    let eo = get_expected_test_outputs();
    eo.match_vec("exhaustive_bools", &mut exhaustive_bools());
    eo.match_vec_f("random_bools", &mut random_x::<bool>(&EXAMPLE_SEED[..]));
}
