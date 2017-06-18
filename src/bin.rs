extern crate rust_wheels;

use rust_wheels::demos::prim_utils::integer_utils_demos::*;
use rust_wheels::iterators::vecs::exhaustive_vecs;
use std::env;

fn main() {
    let xs: Vec<u32> = vec![1, 2, 3];
    for v in exhaustive_vecs(xs.iter()).take(10000) {
        println!("{:?}", v);
    }
    if 1 == 1 {
        return;
    }
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 && args.len() != 4 {
        panic!("Usage: [exhaustive|random] [limit] [demo name]");
    }
    let limit = if args.len() == 4 {
        args[2].parse().unwrap()
    } else {
        usize::max_value()
    };
    match args[args.len() - 1].as_ref() {
        "ceiling_log_2_u8" => demo_ceiling_log_2_u::<u8>(limit),
        "ceiling_log_2_u16" => demo_ceiling_log_2_u::<u16>(limit),
        "ceiling_log_2_u32" => demo_ceiling_log_2_u::<u32>(limit),
        "ceiling_log_2_u64" => demo_ceiling_log_2_u::<u64>(limit),
        "ceiling_log_2_usize" => demo_ceiling_log_2_u::<usize>(limit),
        "bits_u8" => demo_bits_u::<u8>(limit),
        "bits_u16" => demo_bits_u::<u16>(limit),
        "bits_u32" => demo_bits_u::<u32>(limit),
        "bits_u64" => demo_bits_u::<u64>(limit),
        "bits_usize" => demo_bits_u::<usize>(limit),

        _ => panic!("Invalid demo name: {}", &args[args.len() - 1]),
    }
}
