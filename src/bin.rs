extern crate rust_wheels;

use rust_wheels::demos::prim_utils::integer_utils_demos::*;
use rust_wheels::iterators::iterator_provider::IteratorProvider;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 && args.len() != 4 {
        panic!("Usage: [exhaustive|random] [limit] [demo name]");
    }
    let mut exhaustive = true;
    match args[1].as_ref() {
        "exhaustive" => {}
        "random" => exhaustive = false,
        _ => panic!("Invalid first argument: {}", args[1]),
    }
    let p = if exhaustive {
        IteratorProvider::Exhaustive
    } else {
        IteratorProvider::example_random()
    };
    let limit = if args.len() == 4 {
        args[2].parse().unwrap()
    } else {
        usize::max_value()
    };
    match args[args.len() - 1].as_ref() {
        "ceiling_log_2_u8" => demo_ceiling_log_2_u::<u8>(&p, limit),
        "ceiling_log_2_u16" => demo_ceiling_log_2_u::<u16>(&p, limit),
        "ceiling_log_2_u32" => demo_ceiling_log_2_u::<u32>(&p, limit),
        "ceiling_log_2_u64" => demo_ceiling_log_2_u::<u64>(&p, limit),
        "ceiling_log_2_usize" => demo_ceiling_log_2_u::<usize>(&p, limit),
        "bits_u8" => demo_bits_u::<u8>(&p, limit),
        "bits_u16" => demo_bits_u::<u16>(&p, limit),
        "bits_u32" => demo_bits_u::<u32>(&p, limit),
        "bits_u64" => demo_bits_u::<u64>(&p, limit),
        "bits_usize" => demo_bits_u::<usize>(&p, limit),

        _ => panic!("Invalid demo name: {}", &args[args.len() - 1]),
    }
}
