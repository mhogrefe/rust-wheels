#[cfg(feature = "gmp")]
extern crate gmp_to_flint_adaptor_lib;
#[cfg(feature = "native")]
extern crate num_to_flint_adaptor_lib;
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
    pub mod iterator_provider;
}

pub mod prim_utils {
    pub mod char_utils;
    pub mod integer_utils;
}
