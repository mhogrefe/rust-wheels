use iterators::iterator_provider::IteratorProvider;
use prim_utils::integer_utils::*;
use prim_utils::traits::PrimUnsignedInt;

pub fn demo_ceiling_log_2_u<T: PrimUnsignedInt>(p: &IteratorProvider, limit: usize) {
    for i in p.positive_u::<T>().take(limit) {
        println!("ceiling_log_2_{}({}) = {}",
                 T::name(),
                 i,
                 ceiling_log_2_u(i));
    }
}

pub fn demo_bits_u<T: PrimUnsignedInt>(p: &IteratorProvider, limit: usize) {
    for i in p.all_u::<T>().take(limit) {
        println!("bits_{}({}) = {:?}", T::name(), i, bits_u(i));
    }
}
