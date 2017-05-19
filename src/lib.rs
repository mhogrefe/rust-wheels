extern crate malachite;
extern crate itertools;
extern crate rand;
extern crate sha3;

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
    pub mod naturals;
    pub mod options;
    pub mod orderings;
    pub mod primitive_ints;
}
pub mod prim_utils {
    pub mod char_utils;
    pub mod integer_utils;
    pub mod traits;
}
