#![allow(unknown_lints, suspicious_arithmetic_impl, suspicious_op_assign_impl)]

extern crate gnuplot;
extern crate itertools;
extern crate malachite_base;
extern crate malachite_nz;
extern crate rand;
extern crate sha3;
extern crate stats;
extern crate time;

pub mod benchmarks;
pub mod io {
    pub mod readers;
}
pub mod iterators {
    pub mod adaptors;
    pub mod bools;
    pub mod chars;
    pub mod common;
    pub mod dependent_pairs;
    pub mod general;
    pub mod integers;
    pub mod integers_geometric;
    pub mod vecs;
    pub mod naturals;
    pub mod options;
    pub mod orderings;
    pub mod primitive_floats;
    pub mod primitive_ints;
    pub mod rounding_modes;
    pub mod tuples;
}
pub mod prim_utils {
    pub mod primitive_float_utils;
    pub mod primitive_int_utils;
    pub mod string_utils;
}
