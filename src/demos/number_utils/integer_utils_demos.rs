use iterators::iterator_provider::IteratorProvider;
use number_utils::integer_utils::*;

pub fn demo_ceiling_log_2_u8(p: &IteratorProvider, limit: usize) {
    for i in p.u8s().filter(|&j| j != 0).take(limit) {
        println!("ceiling_log_2_u8({}) = {}", i, ceiling_log_2_u8(i));
    }
}
