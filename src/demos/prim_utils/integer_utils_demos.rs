use iterators::primitive_ints::{exhaustive_positive_x, exhaustive_u};
use prim_utils::integer_utils::*;
use prim_utils::traits::PrimUnsignedInt;

pub fn demo_ceiling_log_2_u<T: PrimUnsignedInt>(limit: usize) {
    for i in exhaustive_positive_x::<T>().take(limit) {
        println!(
            "ceiling_log_2_{}({}) = {}",
            T::name(),
            i,
            ceiling_log_2_u(i)
        );
    }
}

pub fn demo_bits_u<T: PrimUnsignedInt>(limit: usize) {
    for i in exhaustive_u::<T>().take(limit) {
        println!("bits_{}({}) = {:?}", T::name(), i, bits_u(i));
    }
}
