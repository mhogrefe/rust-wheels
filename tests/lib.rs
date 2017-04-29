#[cfg(feature = "gmp")]
extern crate malachite_gmp;
#[cfg(feature = "native")]
extern crate malachite_native;
extern crate rust_wheels;

pub mod common;

pub mod iterators {
    pub mod iterator_provider_tests;
}

pub mod prim_utils {
    pub mod integer_utils_tests;
}
