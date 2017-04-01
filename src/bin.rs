extern crate rust_wheels_lib;

use rust_wheels_lib::iterators::iterator_provider::IteratorProvider;
use rust_wheels_lib::iterators::adaptors::*;

fn main() {
    for (k, v) in get_most_common_values(10,
                                         &mut IteratorProvider::example_random()
                                                  .u8s()
                                                  .take(10000)) {
        println!("{} -> {}", k, v);
    }
}
