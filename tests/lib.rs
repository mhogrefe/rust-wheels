extern crate malachite;
extern crate malachite_base;
extern crate rust_wheels;

pub mod common;

pub mod iterators {
    pub mod bools;
    pub mod chars;
    pub mod general;
    pub mod integers;
    pub mod integers_geometric;
    pub mod naturals;
    pub mod orderings;
    pub mod primitive_ints;
    pub mod rounding_modes;
}

pub mod prim_utils {
    pub mod integer_utils;
}
