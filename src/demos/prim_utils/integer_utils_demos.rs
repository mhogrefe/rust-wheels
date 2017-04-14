use iterators::iterator_provider::IteratorProvider;
use prim_utils::integer_utils::*;

macro_rules! demo_ceiling_log_2 {
    ($ts: expr, $d: ident, $pt: ident) => {
        pub fn $d(p: &IteratorProvider, limit: usize) {
            for i in p.$pt().take(limit) {
                println!("ceiling_log_2_{}({}) = {}", $ts, i, ceiling_log_2(i));
            }
        }
    }
}

demo_ceiling_log_2!("u8", demo_ceiling_log_2_u8, positive_u8s);
demo_ceiling_log_2!("u16", demo_ceiling_log_2_u16, positive_u16s);
demo_ceiling_log_2!("u32", demo_ceiling_log_2_u32, positive_u32s);
demo_ceiling_log_2!("u64", demo_ceiling_log_2_u64, positive_u64s);
demo_ceiling_log_2!("usize", demo_ceiling_log_2_usize, positive_usizes);

macro_rules! demo_bits {
    ($ts: expr, $d: ident, $nt: ident) => {
        pub fn $d(p: &IteratorProvider, limit: usize) {
            for i in p.$nt().take(limit) {
                println!("bits_{}({}) = {:?}", $ts, i, bits(i));
            }
        }
    }
}

demo_bits!("u8", demo_bits_u8, u8s);
demo_bits!("u16", demo_bits_u16, u16s);
demo_bits!("u32", demo_bits_u32, u32s);
demo_bits!("u64", demo_bits_u64, u64s);
demo_bits!("usize", demo_bits_usize, usizes);
