use iterators::iterator_provider::IteratorProvider;
use prim_utils::integer_utils::*;

macro_rules! demo_ceiling_log_2 {
    ($t: ty, $ts: expr, $d: ident) => {
        pub fn $d(p: &IteratorProvider, limit: usize) {
            for i in p.positive_u::<$t>().take(limit) {
                println!("ceiling_log_2_{}({}) = {}", $ts, i, ceiling_log_2_u(i));
            }
        }
    }
}

demo_ceiling_log_2!(u8, "u8", demo_ceiling_log_2_u8);
demo_ceiling_log_2!(u16, "u16", demo_ceiling_log_2_u16);
demo_ceiling_log_2!(u32, "u32", demo_ceiling_log_2_u32);
demo_ceiling_log_2!(u64, "u64", demo_ceiling_log_2_u64);
demo_ceiling_log_2!(usize, "usize", demo_ceiling_log_2_usize);

macro_rules! demo_bits {
    ($t: ty, $ts: expr, $d: ident) => {
        pub fn $d(p: &IteratorProvider, limit: usize) {
            for i in p.all_u::<$t>().take(limit) {
                println!("bits_{}({}) = {:?}", $ts, i, bits_u(i));
            }
        }
    }
}

demo_bits!(u8, "u8", demo_bits_u8);
demo_bits!(u16, "u16", demo_bits_u16);
demo_bits!(u32, "u32", demo_bits_u32);
demo_bits!(u64, "u64", demo_bits_u64);
demo_bits!(usize, "usize", demo_bits_usize);
