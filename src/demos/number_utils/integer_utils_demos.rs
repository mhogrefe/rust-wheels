use iterators::iterator_provider::IteratorProvider;
use number_utils::integer_utils::*;

pub fn demo_is_power_of_two_u8(p: &IteratorProvider, limit: usize) {
    for i in p.u8s().filter(|&j| j != 0).take(limit) {
        if is_power_of_two_u8(i) {
            println!("{} is a power of two", i);
        } else {
            println!("{} is not a power of two", i);
        }
    }
}
