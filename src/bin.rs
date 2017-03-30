extern crate rust_wheels_lib;

use rust_wheels_lib::iterators::iterator_provider::IteratorProvider;

fn main() {
    let p = IteratorProvider::example_random();
    for i in p.u8s().take(100) {
        println!("{}", i);
    }
    println!();
    for i in p.altered("alt").u8s().take(100) {
        println!("{}", i);
    }
}
