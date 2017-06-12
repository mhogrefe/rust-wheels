extern crate gnuplot;
extern crate itertools;
extern crate malachite;
extern crate rand;
extern crate sha3;
extern crate stats;
extern crate time;

pub mod benchmarks;
pub mod demos {
    pub mod prim_utils {
        pub mod integer_utils_demos;
    }
}
pub mod io {
    pub mod readers;
}
pub mod iterators {
    pub mod adaptors;
    pub mod bools;
    pub mod chars;
    pub mod common;
    pub mod general;
    pub mod integers;
    pub mod integers_geometric;
    pub mod lists;
    pub mod naturals;
    pub mod options;
    pub mod orderings;
    pub mod primitive_ints;
    pub mod tuples;
}
pub mod prim_utils {
    pub mod char_utils;
    pub mod integer_utils;
    pub mod traits;
}
