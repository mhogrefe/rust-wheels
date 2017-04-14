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
demo_ceiling_log_2!("i8", demo_ceiling_log_2_i8, positive_i8s);
demo_ceiling_log_2!("i16", demo_ceiling_log_2_i16, positive_i16s);
demo_ceiling_log_2!("i32", demo_ceiling_log_2_i32, positive_i32s);
demo_ceiling_log_2!("i64", demo_ceiling_log_2_i64, positive_i64s);
demo_ceiling_log_2!("isize", demo_ceiling_log_2_isize, positive_isizes);

macro_rules! demo_bits {
    ($ts: expr, $d: ident, $f: ident, $nt: ident) => {
        pub fn $d(p: &IteratorProvider, limit: usize) {
            for i in p.$nt().take(limit) {
                println!("bits_{}({}) = {:?}", $ts, i, $f(i));
            }
        }
    }
}

demo_bits!("u8", demo_bits_u8, bits_u8, u8s);
demo_bits!("u16", demo_bits_u16, bits_u16, u16s);
demo_bits!("u32", demo_bits_u32, bits_u32, u32s);
demo_bits!("u64", demo_bits_u64, bits_u64, u64s);
demo_bits!("usize", demo_bits_usize, bits_usize, usizes);
demo_bits!("i8", demo_bits_i8, bits_i8, natural_i8s);
demo_bits!("i16", demo_bits_i16, bits_i16, natural_i16s);
demo_bits!("i32", demo_bits_i32, bits_i32, natural_i32s);
demo_bits!("i64", demo_bits_i64, bits_i64, natural_i64s);
demo_bits!("isize", demo_bits_isize, bits_isize, natural_isizes);
