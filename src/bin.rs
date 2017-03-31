extern crate rust_wheels_lib;

use rust_wheels_lib::demos::number_utils::integer_utils_demos::*;
use rust_wheels_lib::iterators::iterator_provider::IteratorProvider;

fn main() {
    demo_is_power_of_two_u8(&IteratorProvider::example_random(), 10000);
    //demo_is_power_of_two_u8(&IteratorProvider::Exhaustive, 10000);
}
