extern crate rust_wheels_lib;

use rust_wheels_lib::iterators::iterator_provider::IteratorProvider;

fn main() {
    let mut p = IteratorProvider::Exhaustive;
    //let mut p = IteratorProvider::example_random();
    for i in p.u8s().take(100) {
        println!("{}", i);
    }
}
