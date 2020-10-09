#![allow(
    unknown_lints,
    suspicious_arithmetic_impl,
    suspicious_op_assign_impl,
    unstable_name_collisions
)]

extern crate gnuplot;
extern crate itertools;
extern crate malachite_base;
extern crate malachite_nz;
extern crate rand;
extern crate sha3;
extern crate stats;
extern crate time;

pub mod iterators {
    pub mod adaptors;
    pub mod chars;
    pub mod common;
    pub mod dependent_pairs;
    pub mod general;
    pub mod integers;
    pub mod integers_geometric;
    pub mod naturals;
    pub mod options;
    pub mod orderings;
    pub mod primitive_floats;
    pub mod primitive_ints;
    pub mod rounding_modes;
    pub mod strings;
    pub mod tuples;
    pub mod vecs;
}

pub mod prim_utils {
    pub mod primitive_float_utils;
}
