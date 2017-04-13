#[cfg(feature = "gmp")]
use gmp_to_flint_adaptor_lib::integer::Integer;
#[cfg(feature = "native")]
use num_to_flint_adaptor_lib::integer::Integer;

use itertools::Interleave;
use itertools::Itertools;
use prim_utils::char_utils::*;
use rand::{IsaacRng, Rng, SeedableRng};
use rand::distributions::{IndependentSample, Range};
use sha3::{Digest, Sha3_256};
use std::char;
use std::clone::Clone;
use std::cmp::Ordering;
use std::iter::*;

const SEED_SIZE: usize = 256;

const EXAMPLE_SEED: [u32; SEED_SIZE] =
    [0xc2ba7ec5, 0x8570291c, 0xc01903b4, 0xb3b63b5e, 0x60a15a04, 0x49cb3889, 0x8656014b,
     0x4489160e, 0x4cfc5c82, 0x4b50a3c8, 0x842d828e, 0x9b2d83d5, 0x8f700df6, 0x06182781,
     0x7d9f61f9, 0x0af20cd5, 0x94d77191, 0xc3d93b66, 0x19f8b8f6, 0xba2eefdb, 0x678ad2b8,
     0x3b53214d, 0x8058837a, 0x3e899556, 0x2f039c85, 0x7b0e4c96, 0xd4e1848a, 0x278ff71b,
     0x19f07329, 0x11c6dd13, 0xaf62556f, 0x997b71b0, 0x53a80a7d, 0x3c62630b, 0x98e25284,
     0x855e8f46, 0xc051e439, 0xabeed5b1, 0x6ef28ce9, 0xc5565c11, 0x092e2a2b, 0x9067a520,
     0x9e70aef7, 0x943e7656, 0x18c4d180, 0x507f1efc, 0x4cae8182, 0x8bb2d120, 0x008b968e,
     0xd6e9fbd9, 0x537a5896, 0x7d8aa351, 0xdb1a211e, 0xcdfb287a, 0x70c87163, 0x1f1b5cf4,
     0x75d0a44e, 0x67619075, 0x22743ffc, 0xed61527e, 0xbaa109b9, 0xf6089407, 0xfb30b32b,
     0xe0c15709, 0x4a34890a, 0x7b4f04e4, 0x0144f2e6, 0x217e07b4, 0x0caf93eb, 0xe698376f,
     0xa146167e, 0xf4345d67, 0x224dc81c, 0x2fadfdee, 0xfea4a8ec, 0x76293549, 0x82f8713a,
     0x3afcfe7d, 0xfd9b3b8b, 0x0329303d, 0x8c85be02, 0xb65e899a, 0xa166256b, 0x5146cf71,
     0xfc076118, 0xddded979, 0xb16a0d0e, 0x4b37a80e, 0xfc3d5e1f, 0x44f2cebd, 0xaf18b58d,
     0xaa19a03d, 0x55badec6, 0xefaa7e5f, 0xa2a07a27, 0x32972064, 0x309acda5, 0x26ef3b9a,
     0xe280fa00, 0x0475bb12, 0x5af7a531, 0xece9bed3, 0x802b1c7b, 0x69f6d54b, 0xc4502e07,
     0xbdf7e9c7, 0x0a1bd4ca, 0x01e8163d, 0xd78c8cdf, 0x23e4c122, 0x802f6a97, 0x2364b37e,
     0xcedefe5f, 0x0cc18f37, 0x4b24c75d, 0x9d75f0ed, 0xd519acd2, 0x8ddabaca, 0xac7a26ab,
     0xf2f9d5f0, 0xcd9e65d7, 0xaf56730a, 0xaf0f51e2, 0x2b80a753, 0xc5e709ff, 0x74558483,
     0xff1413f3, 0xe8ac0312, 0x7caf6683, 0x0806dbdf, 0xb914a79c, 0x3c22f9ca, 0x559a5450,
     0x0d44af11, 0x6917e268, 0xc464800c, 0x8dbf36a3, 0xe783a8e0, 0x5d2e69ec, 0x7e606281,
     0x7cbce3d7, 0xf8df4062, 0x63852a67, 0xdd712417, 0x33f43ef5, 0x04668ce5, 0x3aaff9fe,
     0xa1a015a4, 0x2036353f, 0xb62e3085, 0x02c37c33, 0x918a0e96, 0xdcffc070, 0x44f652f8,
     0xf23b9e9e, 0xfe9f2bd7, 0x355857f6, 0x735e0efc, 0x46e29345, 0xea77a3ff, 0x40cbcad9,
     0xbd2e5013, 0x7ab2c23b, 0xfd57b407, 0xc1553de4, 0xfdf95c8b, 0xfbb00eb7, 0x3a1e33e6,
     0x6ab7973e, 0xe7585473, 0xb16ccab0, 0x20efa765, 0x604728a7, 0x7de0b479, 0xf708d51c,
     0x480d51d1, 0xc54388e6, 0x78c0c2a8, 0xb7f2f73e, 0xc184d42f, 0x43c9c988, 0x81d5c1e3,
     0xeb3f8337, 0x5d8bfb01, 0x101a0a28, 0xa9db2a65, 0x2b198777, 0x36b32652, 0x4c23e850,
     0x356b0620, 0x8a9c7afb, 0x9e5c04bb, 0x2ba85921, 0xf4a4931d, 0x50069920, 0xd4548176,
     0x9deb2296, 0xfacb9fce, 0x809b0c63, 0x0f83e2b5, 0xa552296b, 0xebfde28b, 0xb091d265,
     0xe5eeb245, 0xa82118f5, 0xe1a7324e, 0x405df7df, 0x61ed1072, 0x31c2fd1e, 0xfc512b29,
     0xf4147274, 0x4d69b85a, 0x70feddb5, 0xb847511f, 0xc06dbb14, 0x8b49c1bf, 0x7f8b87ac,
     0x26263842, 0x187a71d6, 0x7a518b43, 0x17780e8a, 0x60ba19c1, 0x32a1636f, 0x87e6a806,
     0x8238efe3, 0x6cf2aa46, 0xf04ab8e4, 0x7b087b4e, 0x54f98594, 0x46e2cb12, 0x374a7391,
     0x6f118b9d, 0xc27814ff, 0xb2548865, 0x0029b619, 0x81c244c3, 0x4916def1, 0x53b1fc20,
     0x41a19f4f, 0x025eb0c3, 0x9efa239a, 0x89c07eed, 0xeb99a870, 0x931f129f, 0x075f60e4,
     0x951db99f, 0xcb52a1e1, 0x542600f0, 0xeb82b9a3, 0x5e4fd956, 0xe7f71bab, 0x8f79aeff,
     0xab5b81ae, 0x7aa714f0, 0x8a8260b7, 0x123fc3c9];

pub enum IteratorProvider {
    Exhaustive,
    Random(String, [u32; SEED_SIZE]),
}

pub fn scramble(seed: &[u32; SEED_SIZE], s: &str) -> [u32; SEED_SIZE] {
    let mut hasher = Sha3_256::new();
    hasher.input(s.as_bytes());
    let hash = hasher.result();
    let mut scrambled_seed = [0; SEED_SIZE];
    for i in 0..SEED_SIZE {
        let j = (i & 0x7) << 2;
        let a = hash[j] as u32;
        let b = hash[j + 1] as u32;
        let c = hash[j + 2] as u32;
        let d = hash[j + 3] as u32;
        scrambled_seed[i] = seed[i] ^ (a << 24 | b << 16 | c << 8 | d)
    }
    scrambled_seed
}

pub enum Booleans {
    Exhaustive(u8),
    Random(IsaacRng),
}

impl Iterator for Booleans {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        match self {
            &mut Booleans::Exhaustive(ref mut i) => {
                match *i {
                    0 => {
                        *i += 1;
                        Some(false)
                    }
                    1 => {
                        *i += 1;
                        Some(true)
                    }
                    _ => None,
                }
            }
            &mut Booleans::Random(ref mut rng) => Some(rng.gen()),
        }
    }
}

macro_rules! integer_range {
    ($t: ty, $ri_s: ident, $rd_s: ident, $rand_s: ident, $max: expr) => {
        pub struct $ri_s {
            i: $t,
            b: $t,
            done: bool,
        }

        impl $ri_s {
            fn new(a: $t, b: $t) -> $ri_s {
                if a > b {
                    panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
                }
                $ri_s {
                    i: a,
                    b: b,
                    done: false,
                }
            }
        }

        impl Iterator for $ri_s {
            type Item = $t;

            fn next(&mut self) -> Option<$t> {
                if self.done {
                    None
                } else {
                    self.done = self.i == self.b;
                    let ret = self.i;
                    if !self.done {
                        self.i += 1
                    }
                    Some(ret)
                }
            }
        }

        pub struct $rd_s {
            a: $t,
            i: $t,
            done: bool,
        }

        impl $rd_s {
            fn new(a: $t, b: $t) -> $rd_s {
                if a > b {
                    panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
                }
                $rd_s {
                    a: a,
                    i: b,
                    done: false,
                }
            }
        }

        impl Iterator for $rd_s {
            type Item = $t;

            fn next(&mut self) -> Option<$t> {
                if self.done {
                    None
                } else {
                    self.done = self.i == self.a;
                    let ret = self.i;
                    if !self.done {
                        self.i -= 1
                    }
                    Some(ret)
                }
            }
        }

        pub struct $rand_s {
            rng: IsaacRng,
        }

        impl $rand_s {
            fn new(seed: &[u32]) -> $rand_s {
                $rand_s { rng: SeedableRng::from_seed(seed) }
            }
        }

        impl Iterator for $rand_s {
            type Item = $t;

            fn next(&mut self) -> Option<$t> {
                Some(self.rng.gen::<$t>())
            }
        }
    }
}

integer_range!(u8,
               RangeIncreasingU8,
               RangeDecreasingU8,
               RandomU8s,
               u8::max_value());
integer_range!(u16,
               RangeIncreasingU16,
               RangeDecreasingU16,
               RandomU16s,
               u16::max_value());
integer_range!(u32,
               RangeIncreasingU32,
               RangeDecreasingU32,
               RandomU32s,
               u32::max_value());
integer_range!(u64,
               RangeIncreasingU64,
               RangeDecreasingU64,
               RandomU64s,
               u64::max_value());
integer_range!(usize,
               RangeIncreasingUsize,
               RangeDecreasingUsize,
               RandomUsizes,
               usize::max_value());
integer_range!(i8,
               RangeIncreasingI8,
               RangeDecreasingI8,
               RandomI8s,
               i8::max_value());
integer_range!(i16,
               RangeIncreasingI16,
               RangeDecreasingI16,
               RandomI16s,
               i16::max_value());
integer_range!(i32,
               RangeIncreasingI32,
               RangeDecreasingI32,
               RandomI32s,
               i32::max_value());
integer_range!(i64,
               RangeIncreasingI64,
               RangeDecreasingI64,
               RandomI64s,
               i64::max_value());
integer_range!(isize,
               RangeIncreasingIsize,
               RangeDecreasingIsize,
               RandomIsizes,
               isize::max_value());

macro_rules! integer_range_u {
    (
        $t: ty,
        $all_s: ident,
        $ri_s: ident,
        $rr_s: ident,
        $rand_s: ident,
        $pos_s: ident,
        $r_s: ident,
        $max: expr
    ) => {
        pub enum $pos_s {
            Exhaustive($ri_s),
            Random($rand_s),
        }

        impl Iterator for $pos_s {
            type Item = $t;

            fn next(&mut self) -> Option<$t> {
                match self {
                    &mut $pos_s::Exhaustive(ref mut it) => it.next(),
                    &mut $pos_s::Random(ref mut it) => {
                        loop {
                            let x = it.next();
                            if x.is_none() || x.unwrap() != 0 {
                                return x;
                            }
                        }
                    },
                }
            }
        }

        pub enum $all_s {
            Exhaustive($ri_s),
            Random($rand_s),
        }

        impl Iterator for $all_s {
            type Item = $t;

            fn next(&mut self) -> Option<$t> {
                match self {
                    &mut $all_s::Exhaustive(ref mut it) => it.next(),
                    &mut $all_s::Random(ref mut it) => it.next(),
                }
            }
        }

        pub enum $rr_s {
            Some(bool, IsaacRng, Range<$t>),
            All($rand_s),
        }

        impl $rr_s {
            fn new(a: $t, b: $t, seed: &[u32]) -> $rr_s {
                if a == 0 && b == $max {
                    $rr_s::All($rand_s::new(&seed))
                } else if b == $max {
                    $rr_s::Some(true, SeedableRng::from_seed(&seed[..]), Range::new(a - 1, b))
                } else {
                    $rr_s::Some(false, SeedableRng::from_seed(&seed[..]), Range::new(a, b + 1))
                }
            }
        }

        impl Iterator for $rr_s {
            type Item = $t;

            fn next(&mut self) -> Option<$t> {
                match self {
                    &mut $rr_s::Some(shift, ref mut rng, ref range) => Some(if shift {
                        range.ind_sample(rng) + 1
                    } else {
                        range.ind_sample(rng)
                    }),
                    &mut $rr_s::All(ref mut it) => it.next(),
                }
            }
        }

        pub enum $r_s {
            Exhaustive($ri_s),
            Random($rr_s),
        }

        impl Iterator for $r_s {
            type Item = $t;

            fn next(&mut self) -> Option<$t> {
                match self {
                    &mut $r_s::Exhaustive(ref mut it) => it.next(),
                    &mut $r_s::Random(ref mut it) => it.next(),
                }
            }
        }
    }
}

integer_range_u!(u8,
                 U8s,
                 RangeIncreasingU8,
                 RandomRangeU8,
                 RandomU8s,
                 PositiveU8s,
                 RangeU8,
                 u8::max_value());
integer_range_u!(u16,
                 U16s,
                 RangeIncreasingU16,
                 RandomRangeU16,
                 RandomU16s,
                 PositiveU16s,
                 RangeU16,
                 u16::max_value());
integer_range_u!(u32,
                 U32s,
                 RangeIncreasingU32,
                 RandomRangeU32,
                 RandomU32s,
                 PositiveU32s,
                 RangeU32,
                 u32::max_value());
integer_range_u!(u64,
                 U64s,
                 RangeIncreasingU64,
                 RandomRangeU64,
                 RandomU64s,
                 PositiveU64s,
                 RangeU64,
                 u64::max_value());
integer_range_u!(usize,
                 Usizes,
                 RangeIncreasingUsize,
                 RandomRangeUsize,
                 RandomUsizes,
                 PositiveUsizes,
                 RangeUsize,
                 usize::max_value());

macro_rules! integer_range_i {
    (
        $t: ty,
        $ut: ty,
        $pos_s: ident,
        $neg_s: ident,
        $nat_s: ident,
        $nz_s: ident,
        $all_s: ident,
        $ri_s: ident,
        $rd_s: ident,
        $rand_s: ident,
        $r_s: ident,
        $er_s: ident,
        $rr_s: ident,
        $min: expr,
        $max: expr
) => {
        pub enum $pos_s {
            Exhaustive($ri_s),
            Random($t, $rand_s),
        }

        impl Iterator for $pos_s {
            type Item = $t;

            fn next(&mut self) -> Option<$t> {
                match self {
                    &mut $pos_s::Exhaustive(ref mut it) => it.next(),
                    &mut $pos_s::Random(mask, ref mut it) => {
                        loop {
                            let x = it.next().map(|x| x & mask);
                            if x.is_none() || x.unwrap() != 0 {
                                return x;
                            }
                        }
                    },
                }
            }
        }

        pub enum $neg_s {
            Exhaustive($rd_s),
            Random($t, $rand_s),
        }

        impl Iterator for $neg_s {
            type Item = $t;

            fn next(&mut self) -> Option<$t> {
                match self {
                    &mut $neg_s::Exhaustive(ref mut it) => it.next(),
                    &mut $neg_s::Random(mask, ref mut it) => it.next().map(|x| !(x & mask)),
                }
            }
        }

        pub enum $nat_s {
            Exhaustive($ri_s),
            Random($t, $rand_s),
        }

        impl Iterator for $nat_s {
            type Item = $t;

            fn next(&mut self) -> Option<$t> {
                match self {
                    &mut $nat_s::Exhaustive(ref mut it) => it.next(),
                    &mut $nat_s::Random(mask, ref mut it) => it.next().map(|x| x & mask),
                }
            }
        }

        pub enum $nz_s {
            Exhaustive(Interleave<$pos_s, $neg_s>),
            Random($rand_s),
        }

        impl Iterator for $nz_s {
            type Item = $t;

            fn next(&mut self) -> Option<$t> {
                match self {
                    &mut $nz_s::Exhaustive(ref mut it) => it.next(),
                    &mut $nz_s::Random(ref mut it) => {
                        loop {
                            let x = it.next();
                            if x.is_none() || x.unwrap() != 0 {
                                return x;
                            }
                        }
                    }
                }
            }
        }

        pub enum $all_s {
            Exhaustive(Chain<Once<$t>, $nz_s>),
            Random($rand_s),
        }

        impl Iterator for $all_s {
            type Item = $t;

            fn next(&mut self) -> Option<$t> {
                match self {
                    &mut $all_s::Exhaustive(ref mut it) => it.next(),
                    &mut $all_s::Random(ref mut it) => it.next(),
                }
            }
        }

        pub enum $er_s {
            AllNonNegative($ri_s),
            AllNonPositive($rd_s),
            SomeOfEachSign(Chain<Once<$t>, Interleave<$ri_s, $rd_s>>),
        }

        pub enum $rr_s {
            Some(bool, IsaacRng, Range<$t>),
            All($rand_s),
        }

        impl $rr_s {
            fn new(a: $t, b: $t, seed: &[u32]) -> $rr_s {
                if a == $min && b == $max {
                    $rr_s::All($rand_s::new(&seed))
                } else if b == $max {
                    $rr_s::Some(true, SeedableRng::from_seed(seed), Range::new(a - 1, b))
                } else {
                    $rr_s::Some(false, SeedableRng::from_seed(seed), Range::new(a, b + 1))
                }
            }
        }

        impl Iterator for $rr_s {
            type Item = $t;

            fn next(&mut self) -> Option<$t> {
                match self {
                    &mut $rr_s::Some(shift, ref mut rng, ref range) => Some(if shift {
                        range.ind_sample(rng) + 1
                    } else {
                        range.ind_sample(rng)
                    }),
                    &mut $rr_s::All(ref mut it) => it.next(),
                }
            }
        }

        pub enum $r_s {
            Exhaustive($er_s),
            Random($rr_s),
        }

        impl Iterator for $r_s {
            type Item = $t;

            fn next(&mut self) -> Option<$t> {
                match self {
                    &mut $r_s::Exhaustive(ref mut it) => {
                        match it {
                            &mut $er_s::AllNonNegative(ref mut it) => it.next(),
                            &mut $er_s::AllNonPositive(ref mut it) => it.next(),
                            &mut $er_s::SomeOfEachSign(ref mut it) => it.next(),
                        }
                    },
                    &mut $r_s::Random(ref mut it) => it.next(),
                }
            }
        }
    }
}

integer_range_i!(i8,
                 u8,
                 PositiveI8s,
                 NegativeI8s,
                 NaturalI8s,
                 NonzeroI8s,
                 I8s,
                 RangeIncreasingI8,
                 RangeDecreasingI8,
                 RandomI8s,
                 RangeI8,
                 ExhaustiveRangeI8,
                 RandomRangeI8,
                 i8::min_value(),
                 i8::max_value());
integer_range_i!(i16,
                 u16,
                 PositiveI16s,
                 NegativeI16s,
                 NaturalI16s,
                 NonzeroI16s,
                 I16s,
                 RangeIncreasingI16,
                 RangeDecreasingI16,
                 RandomI16s,
                 RangeI16,
                 ExhaustiveRangeI16,
                 RandomRangeI16,
                 i16::min_value(),
                 i16::max_value());
integer_range_i!(i32,
                 u32,
                 PositiveI32s,
                 NegativeI32s,
                 NaturalI32s,
                 NonzeroI32s,
                 I32s,
                 RangeIncreasingI32,
                 RangeDecreasingI32,
                 RandomI32s,
                 RangeI32,
                 ExhaustiveRangeI32,
                 RandomRangeI32,
                 i32::min_value(),
                 i32::max_value());
integer_range_i!(i64,
                 u64,
                 PositiveI64s,
                 NegativeI64s,
                 NaturalI64s,
                 NonzeroI64s,
                 I64s,
                 RangeIncreasingI64,
                 RangeDecreasingI64,
                 RandomI64s,
                 RangeI64,
                 ExhaustiveRangeI64,
                 RandomRangeI64,
                 i64::min_value(),
                 i64::max_value());
integer_range_i!(isize,
                 usize,
                 PositiveIsizes,
                 NegativeIsizes,
                 NaturalIsizes,
                 NonzeroIsizes,
                 Isizes,
                 RangeIncreasingIsize,
                 RangeDecreasingIsize,
                 RandomIsizes,
                 RangeIsize,
                 ExhaustiveRangeIsize,
                 RandomRangeIsize,
                 isize::min_value(),
                 isize::max_value());

pub struct RangeIncreasingChar {
    i: u32,
    b: u32,
    done: bool,
}

impl RangeIncreasingChar {
    fn new(a: char, b: char) -> RangeIncreasingChar {
        if a > b {
            panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
        }
        RangeIncreasingChar {
            i: char_to_contiguous_range(a),
            b: char_to_contiguous_range(b),
            done: false,
        }
    }
}

impl Iterator for RangeIncreasingChar {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.done {
            None
        } else {
            self.done = self.i == self.b;
            let ret = self.i.clone();
            if !self.done {
                self.i += 1;
            }
            Some(contiguous_range_to_char(ret).unwrap())
        }
    }
}

pub struct RangeDecreasingChar {
    a: u32,
    i: u32,
    done: bool,
}

impl RangeDecreasingChar {
    fn new(a: char, b: char) -> RangeDecreasingChar {
        if a > b {
            panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
        }
        RangeDecreasingChar {
            a: char_to_contiguous_range(a),
            i: char_to_contiguous_range(b),
            done: false,
        }
    }
}

impl Iterator for RangeDecreasingChar {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.done {
            None
        } else {
            self.done = self.i == self.a;
            let ret = self.i.clone();
            if !self.done {
                self.i -= 1;
            }
            Some(contiguous_range_to_char(ret).unwrap())
        }
    }
}

pub struct ExhaustiveChars {
    ranges: Vec<RangeIncreasingChar>,
    i: usize,
}

impl ExhaustiveChars {
    pub fn new(ranges: Vec<RangeIncreasingChar>) -> ExhaustiveChars {
        ExhaustiveChars {
            ranges: ranges,
            i: 0,
        }
    }
}

impl Iterator for ExhaustiveChars {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        if self.i == self.ranges.len() {
            None
        } else {
            if let Some(c) = self.ranges[self.i].next() {
                Some(c)
            } else {
                self.i += 1;
                if self.i == self.ranges.len() {
                    None
                } else {
                    self.ranges[self.i].next()
                }
            }
        }
    }
}

pub enum Chars {
    Exhaustive(ExhaustiveChars),
    Random(IsaacRng),
}

impl Iterator for Chars {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self {
            &mut Chars::Exhaustive(ref mut it) => it.next(),
            &mut Chars::Random(ref mut rng) => Some(rng.gen()),
        }
    }
}

pub enum AsciiChars {
    Exhaustive(ExhaustiveChars),
    Random(IsaacRng),
}

impl Iterator for AsciiChars {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self {
            &mut AsciiChars::Exhaustive(ref mut it) => it.next(),
            &mut AsciiChars::Random(ref mut rng) => {
                Some(char::from_u32((rng.gen::<u8>() & 0x7f) as u32).unwrap())
            }
        }
    }
}

pub struct RangeIncreasingInteger {
    i: Integer,
    b: Integer,
    done: bool,
}

impl RangeIncreasingInteger {
    fn new(a: Integer, b: Integer) -> RangeIncreasingInteger {
        if a > b {
            panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
        }
        RangeIncreasingInteger {
            i: a,
            b: b,
            done: false,
        }
    }
}

impl Iterator for RangeIncreasingInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        if self.done {
            None
        } else {
            self.done = self.i == self.b;
            let ret = self.i.clone();
            if !self.done {
                self.i += 1
            }
            Some(ret)
        }
    }
}

pub struct RangeDecreasingInteger {
    a: Integer,
    i: Integer,
    done: bool,
}

impl RangeDecreasingInteger {
    fn new(a: Integer, b: Integer) -> RangeDecreasingInteger {
        if a > b {
            panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
        }
        RangeDecreasingInteger {
            a: a,
            i: b,
            done: false,
        }
    }
}

impl Iterator for RangeDecreasingInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        if self.done {
            None
        } else {
            self.done = self.i == self.a;
            let ret = self.i.clone();
            if !self.done {
                self.i -= 1
            }
            Some(ret)
        }
    }
}

pub struct RangeIncreasingUnboundedInteger {
    i: Integer,
}

impl RangeIncreasingUnboundedInteger {
    fn new(a: Integer) -> RangeIncreasingUnboundedInteger {
        RangeIncreasingUnboundedInteger { i: a }
    }
}

impl Iterator for RangeIncreasingUnboundedInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        let ret = self.i.clone();
        self.i += 1;
        Some(ret)
    }
}

pub struct RangeDecreasingUnboundedInteger {
    i: Integer,
}

impl RangeDecreasingUnboundedInteger {
    fn new(a: Integer) -> RangeDecreasingUnboundedInteger {
        RangeDecreasingUnboundedInteger { i: a }
    }
}

impl Iterator for RangeDecreasingUnboundedInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        let ret = self.i.clone();
        self.i -= 1;
        Some(ret)
    }
}

pub enum ExhaustiveRangeInteger {
    AllNonNegative(RangeIncreasingInteger),
    AllNonPositive(RangeDecreasingInteger),
    SomeOfEachSign(Chain<Once<Integer>,
                         Interleave<RangeIncreasingInteger, RangeDecreasingInteger>>),
}

pub enum RangeInteger {
    Exhaustive(ExhaustiveRangeInteger),
    Random(IsaacRng, Integer, Integer),
}

impl Iterator for RangeInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        match self {
            &mut RangeInteger::Exhaustive(ref mut it) => {
                match it {
                    &mut ExhaustiveRangeInteger::AllNonNegative(ref mut it) => it.next(),
                    &mut ExhaustiveRangeInteger::AllNonPositive(ref mut it) => it.next(),
                    &mut ExhaustiveRangeInteger::SomeOfEachSign(ref mut it) => it.next(),
                }
            }
            &mut RangeInteger::Random(ref mut rng, ref diameter, ref lower) => {
                let mut random = diameter.clone();
                random.random_below(rng);
                random += lower;
                Some(random)
            }
        }
    }
}

macro_rules! integer_range_impl {
    (
        $t: ty,
        $rui_f: ident,
        $rud_f: ident,
        $rdi_f: ident,
        $rdd_f: ident,
        $ri_f: ident,
        $rd_f: ident,
        $i_f: ident,
        $d_f: ident,
        $pos_f: ident,
        $nat_f: ident,
        $ri_s: ident,
        $rd_s: ident,
        $min: expr,
        $max: expr
    ) => {
        pub fn $rui_f(&self, a: $t) -> $ri_s {
            $ri_s::new(a, $max)
        }

        pub fn $rud_f(&self, a: $t) -> $rd_s {
            $rd_s::new(a, $max)
        }

        pub fn $rdi_f(&self, b: $t) -> $ri_s {
            $ri_s::new($min, b)
        }

        pub fn $rdd_f(&self, b: $t) -> $rd_s {
            $rd_s::new($min, b)
        }

        pub fn $ri_f(&self, a: $t, b: $t) -> $ri_s {
            $ri_s::new(a, b)
        }

        pub fn $rd_f(&self, a: $t, b: $t) -> $rd_s {
            $rd_s::new(a, b)
        }

        pub fn $i_f(&self) -> $ri_s {
            $ri_s::new($min, $max)
        }

        pub fn $d_f(&self) -> $rd_s {
            $rd_s::new($min, $max)
        }
    }
}

macro_rules! integer_range_impl_u {
    (
        $t: ty,
        $pos_f: ident,
        $all_f: ident,
        $ru_f: ident,
        $rd_f: ident,
        $r_f: ident,
        $ri_s: ident,
        $all_s: ident,
        $rand_s: ident,
        $pos_s: ident,
        $r_s: ident,
        $rr_s: ident,
        $max: expr
    ) => {
        pub fn $pos_f(&self) -> $pos_s {
            match self {
                &IteratorProvider::Exhaustive => $pos_s::Exhaustive($ri_s::new(1, $max)),
                &IteratorProvider::Random(_, seed) => $pos_s::Random($rand_s::new(&seed)),
            }
        }

        pub fn $all_f(&self) -> $all_s {
            match self {
                &IteratorProvider::Exhaustive => $all_s::Exhaustive($ri_s::new(0, $max)),
                &IteratorProvider::Random(_, seed) => $all_s::Random($rand_s::new(&seed)),
            }
        }

        pub fn $ru_f(&self, a: $t) -> $r_s {
            self.$r_f(a, $max)
        }

        pub fn $rd_f(&self, a: $t) -> $r_s {
            self.$r_f(0, a)
        }

        pub fn $r_f(&self, a: $t, b: $t) -> $r_s {
            match self {
                &IteratorProvider::Exhaustive => $r_s::Exhaustive($ri_s::new(a, b)),
                &IteratorProvider::Random(_, seed) => $r_s::Random($rr_s::new(a, b, &seed[..])),
            }
        }
    }
}

macro_rules! integer_range_impl_i {
    (
        $t: ty,
        $ut: ty,
        $pos_f: ident,
        $neg_f: ident,
        $nat_f: ident,
        $nz_f: ident,
        $all_f: ident,
        $ru_f: ident,
        $rd_f: ident,
        $r_f: ident,
        $ri_s: ident,
        $rd_s: ident,
        $pos_s: ident,
        $neg_s: ident,
        $nat_s: ident,
        $nz_s: ident,
        $rand_s: ident,
        $all_s: ident,
        $r_s: ident,
        $er_s: ident,
        $rr_s: ident,
        $min: expr,
        $max: expr
    ) => {
        pub fn $pos_f(&self) -> $pos_s {
            match self {
                &IteratorProvider::Exhaustive => $pos_s::Exhaustive($ri_s::new(1, $max)),
                &IteratorProvider::Random(_, seed) =>
                        $pos_s::Random($max, $rand_s::new(&seed)),
            }
        }

        pub fn $neg_f(&self) -> $neg_s {
            match self {
                &IteratorProvider::Exhaustive => $neg_s::Exhaustive($rd_s::new($min, -1)),
                &IteratorProvider::Random(_, seed) =>
                        $neg_s::Random($max, $rand_s::new(&seed)),
            }
        }

        pub fn $nat_f(&self) -> $nat_s {
            match self {
                &IteratorProvider::Exhaustive => $nat_s::Exhaustive($ri_s::new(0, $max)),
                &IteratorProvider::Random(_, seed) =>
                        $nat_s::Random($max, $rand_s::new(&seed)),
            }
        }

        pub fn $nz_f(&self) -> $nz_s {
            match self {
                &IteratorProvider::Exhaustive =>
                        $nz_s::Exhaustive(self.$pos_f().interleave(self.$neg_f())),
                &IteratorProvider::Random(_, seed) => $nz_s::Random($rand_s::new(&seed)),
            }
        }

        pub fn $all_f(&self) -> $all_s {
            match self {
                &IteratorProvider::Exhaustive => $all_s::Exhaustive(once(0).chain(self.$nz_f())),
                &IteratorProvider::Random(_, seed) => $all_s::Random($rand_s::new(&seed)),
            }
        }

        pub fn $ru_f(&self, a: $t) -> $r_s {
            self.$r_f(a, $max)
        }

        pub fn $rd_f(&self, a: $t) -> $r_s {
            self.$r_f($min, a)
        }

        pub fn $r_f(&self, a: $t, b: $t) -> $r_s {
            if a > b {
                panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
            }
            match self {
                &IteratorProvider::Exhaustive => {
                    $r_s::Exhaustive(if a >= 0 {
                        $er_s::AllNonNegative($ri_s::new(a, b))
                    } else if b <= 0 {
                        $er_s::AllNonPositive($rd_s::new(a, b))
                    } else {
                        $er_s::SomeOfEachSign(
                                once(0).chain($ri_s::new(1, b).interleave($rd_s::new(a, -1)))
                        )
                    })
                },
                &IteratorProvider::Random(_, seed) => $r_s::Random($rr_s::new(a, b, &seed[..]))
            }
        }
    }
}

pub struct ExhaustiveFromVector<T> {
    xs: Vec<T>,
    range: RangeIncreasingUsize,
}

impl<T: Clone> Iterator for ExhaustiveFromVector<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.xs.is_empty() {
            None
        } else {
            self.range.next().map(|i| self.xs[i].clone())
        }
    }
}

pub struct FromVector<T> {
    xs: Vec<T>,
    range: RangeUsize,
}

impl<T: Clone> Iterator for FromVector<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.xs.is_empty() {
            None
        } else {
            self.range.next().map(|i| self.xs[i].clone())
        }
    }
}

pub enum PositiveU32sGeometric {
    Exhaustive(RangeIncreasingU32),
    Random(RandomRangeU32),
}

impl Iterator for PositiveU32sGeometric {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        match self {
            &mut PositiveU32sGeometric::Exhaustive(ref mut it) => it.next(),
            &mut PositiveU32sGeometric::Random(ref mut it) => {
                let mut j = 0;
                loop {
                    j += 1;
                    if it.next().unwrap() == 0 {
                        break;
                    }
                }
                Some(j)
            }
        }
    }
}

pub enum NaturalU32sGeometric {
    Exhaustive(RangeIncreasingU32),
    Random(RandomRangeU32),
}

impl Iterator for NaturalU32sGeometric {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        match self {
            &mut NaturalU32sGeometric::Exhaustive(ref mut it) => it.next(),
            &mut NaturalU32sGeometric::Random(ref mut it) => {
                let mut j = 0;
                loop {
                    if it.next().unwrap() == 0 {
                        break;
                    }
                    j += 1;
                }
                Some(j)
            }
        }
    }
}

impl IteratorProvider {
    pub fn example_random() -> IteratorProvider {
        let key = "example";
        IteratorProvider::Random(key.to_string(), scramble(&EXAMPLE_SEED, key))
    }

    pub fn altered(&self, key_addition: &str) -> IteratorProvider {
        match self {
            &IteratorProvider::Random(ref key, ref seed) => {
                let new_key = format!("{}-{}", key, key_addition);
                let scrambled_seed = scramble(&seed, &new_key);
                IteratorProvider::Random(new_key, scrambled_seed)
            }
            &IteratorProvider::Exhaustive => IteratorProvider::Exhaustive,
        }
    }

    pub fn bools(&self) -> Booleans {
        match self {
            &IteratorProvider::Exhaustive => Booleans::Exhaustive(0),
            &IteratorProvider::Random(_, seed) => {
                Booleans::Random(SeedableRng::from_seed(&seed[..]))
            }
        }
    }

    integer_range_impl!(u8,
                        range_up_increasing_u8,
                        range_up_decreasing_u8,
                        range_down_increasing_u8,
                        range_down_decreasing_u8,
                        range_increasing_u8,
                        range_decreasing_u8,
                        u8s_increasing,
                        u8s_decreasing,
                        positive_u8s,
                        natural_u8s,
                        RangeIncreasingU8,
                        RangeDecreasingU8,
                        0,
                        u8::max_value());
    integer_range_impl!(u16,
                        range_up_increasing_u16,
                        range_up_decreasing_u16,
                        range_down_increasing_u16,
                        range_down_decreasing_u16,
                        range_increasing_u16,
                        range_decreasing_u16,
                        u16s_increasing,
                        u16s_decreasing,
                        positive_u16s,
                        natural_u16s,
                        RangeIncreasingU16,
                        RangeDecreasingU16,
                        0,
                        u16::max_value());
    integer_range_impl!(u32,
                        range_up_increasing_u32,
                        range_up_decreasing_u32,
                        range_down_increasing_u32,
                        range_down_decreasing_u32,
                        range_increasing_u32,
                        range_decreasing_u32,
                        u32s_increasing,
                        u32s_decreasing,
                        positive_u32s,
                        natural_u32s,
                        RangeIncreasingU32,
                        RangeDecreasingU32,
                        0,
                        u32::max_value());
    integer_range_impl!(u64,
                        range_up_increasing_u64,
                        range_up_decreasing_u64,
                        range_down_increasing_u64,
                        range_down_decreasing_u64,
                        range_increasing_u64,
                        range_decreasing_u64,
                        u64s_increasing,
                        u64s_decreasing,
                        positive_u64s,
                        natural_u64s,
                        RangeIncreasingU64,
                        RangeDecreasingU64,
                        0,
                        u64::max_value());
    integer_range_impl!(usize,
                        range_up_increasing_usize,
                        range_up_decreasing_usize,
                        range_down_increasing_usize,
                        range_down_decreasing_usize,
                        range_increasing_usize,
                        range_decreasing_usize,
                        usizes_increasing,
                        usizes_decreasing,
                        positive_usizes,
                        natural_usizes,
                        RangeIncreasingUsize,
                        RangeDecreasingUsize,
                        0,
                        usize::max_value());
    integer_range_impl!(i8,
                        range_up_increasing_i8,
                        range_up_decreasing_i8,
                        range_down_increasing_i8,
                        range_down_decreasing_i8,
                        range_increasing_i8,
                        range_decreasing_i8,
                        i8s_increasing,
                        i8s_decreasing,
                        positive_i8s,
                        natural_i8s,
                        RangeIncreasingI8,
                        RangeDecreasingI8,
                        i8::min_value(),
                        i8::max_value());
    integer_range_impl!(i16,
                        range_up_increasing_i16,
                        range_up_decreasing_i16,
                        range_down_increasing_i16,
                        range_down_decreasing_i16,
                        range_increasing_i16,
                        range_decreasing_i16,
                        i16s_increasing,
                        i16s_decreasing,
                        positive_i16s,
                        natural_i16s,
                        RangeIncreasingI16,
                        RangeDecreasingI16,
                        i16::min_value(),
                        i16::max_value());
    integer_range_impl!(i32,
                        range_up_increasing_i32,
                        range_up_decreasing_i32,
                        range_down_increasing_i32,
                        range_down_decreasing_i32,
                        range_increasing_i32,
                        range_decreasing_i32,
                        i32s_increasing,
                        i32s_decreasing,
                        positive_i32s,
                        natural_i32s,
                        RangeIncreasingI32,
                        RangeDecreasingI32,
                        i32::min_value(),
                        i32::max_value());
    integer_range_impl!(i64,
                        range_up_increasing_i64,
                        range_up_decreasing_i64,
                        range_down_increasing_i64,
                        range_down_decreasing_i64,
                        range_increasing_i64,
                        range_decreasing_i64,
                        i64s_increasing,
                        i64s_decreasing,
                        positive_i64s,
                        natural_i64s,
                        RangeIncreasingI64,
                        RangeDecreasingI64,
                        i64::min_value(),
                        i64::max_value());
    integer_range_impl!(isize,
                        range_up_increasing_isize,
                        range_up_decreasing_isize,
                        range_down_increasing_isize,
                        range_down_decreasing_isize,
                        range_increasing_isize,
                        range_decreasing_isize,
                        isizes_increasing,
                        isizes_decreasing,
                        positive_isizes,
                        natural_isizes,
                        RangeIncreasingIsize,
                        RangeDecreasingIsize,
                        isize::min_value(),
                        isize::max_value());

    integer_range_impl_u!(u8,
                          positive_u8s,
                          u8s,
                          range_up_u8,
                          range_down_u8,
                          range_u8,
                          RangeIncreasingU8,
                          U8s,
                          RandomU8s,
                          PositiveU8s,
                          RangeU8,
                          RandomRangeU8,
                          u8::max_value());
    integer_range_impl_u!(u16,
                          positive_u16s,
                          u16s,
                          range_up_u16,
                          range_down_u16,
                          range_u16,
                          RangeIncreasingU16,
                          U16s,
                          RandomU16s,
                          PositiveU16s,
                          RangeU16,
                          RandomRangeU16,
                          u16::max_value());
    integer_range_impl_u!(u32,
                          positive_u32s,
                          u32s,
                          range_up_u32,
                          range_down_u32,
                          range_u32,
                          RangeIncreasingU32,
                          U32s,
                          RandomU32s,
                          PositiveU32s,
                          RangeU32,
                          RandomRangeU32,
                          u32::max_value());
    integer_range_impl_u!(u64,
                          positive_u64s,
                          u64s,
                          range_up_u64,
                          range_down_u64,
                          range_u64,
                          RangeIncreasingU64,
                          U64s,
                          RandomU64s,
                          PositiveU64s,
                          RangeU64,
                          RandomRangeU64,
                          u64::max_value());
    integer_range_impl_u!(usize,
                          positive_usizes,
                          usizes,
                          range_up_usize,
                          range_down_usize,
                          range_usize,
                          RangeIncreasingUsize,
                          Usizes,
                          RandomUsizes,
                          PositiveUsizes,
                          RangeUsize,
                          RandomRangeUsize,
                          usize::max_value());

    integer_range_impl_i!(i8,
                          u8,
                          positive_i8s,
                          negative_i8s,
                          natural_i8s,
                          nonzero_i8s,
                          i8s,
                          range_up_i8,
                          range_down_i8,
                          range_i8,
                          RangeIncreasingI8,
                          RangeDecreasingI8,
                          PositiveI8s,
                          NegativeI8s,
                          NaturalI8s,
                          NonzeroI8s,
                          RandomI8s,
                          I8s,
                          RangeI8,
                          ExhaustiveRangeI8,
                          RandomRangeI8,
                          i8::min_value(),
                          i8::max_value());
    integer_range_impl_i!(i16,
                          u16,
                          positive_i16s,
                          negative_i16s,
                          natural_i16s,
                          nonzero_i16s,
                          i16s,
                          range_up_i16,
                          range_down_i16,
                          range_i16,
                          RangeIncreasingI16,
                          RangeDecreasingI16,
                          PositiveI16s,
                          NegativeI16s,
                          NaturalI16s,
                          NonzeroI16s,
                          RandomI16s,
                          I16s,
                          RangeI16,
                          ExhaustiveRangeI16,
                          RandomRangeI16,
                          i16::min_value(),
                          i16::max_value());
    integer_range_impl_i!(i32,
                          u32,
                          positive_i32s,
                          negative_i32s,
                          natural_i32s,
                          nonzero_i32s,
                          i32s,
                          range_up_i32,
                          range_down_i32,
                          range_i32,
                          RangeIncreasingI32,
                          RangeDecreasingI32,
                          PositiveI32s,
                          NegativeI32s,
                          NaturalI32s,
                          NonzeroI32s,
                          RandomI32s,
                          I32s,
                          RangeI32,
                          ExhaustiveRangeI32,
                          RandomRangeI32,
                          i32::min_value(),
                          i32::max_value());
    integer_range_impl_i!(i64,
                          u64,
                          positive_i64s,
                          negative_i64s,
                          natural_i64s,
                          nonzero_i64s,
                          i64s,
                          range_up_i64,
                          range_down_i64,
                          range_i64,
                          RangeIncreasingI64,
                          RangeDecreasingI64,
                          PositiveI64s,
                          NegativeI64s,
                          NaturalI64s,
                          NonzeroI64s,
                          RandomI64s,
                          I64s,
                          RangeI64,
                          ExhaustiveRangeI64,
                          RandomRangeI64,
                          i64::min_value(),
                          i64::max_value());
    integer_range_impl_i!(isize,
                          usize,
                          positive_isizes,
                          negative_isizes,
                          natural_isizes,
                          nonzero_isizes,
                          isizes,
                          range_up_isize,
                          range_down_isize,
                          range_isize,
                          RangeIncreasingIsize,
                          RangeDecreasingIsize,
                          PositiveIsizes,
                          NegativeIsizes,
                          NaturalIsizes,
                          NonzeroIsizes,
                          RandomIsizes,
                          Isizes,
                          RangeIsize,
                          ExhaustiveRangeIsize,
                          RandomRangeIsize,
                          isize::min_value(),
                          isize::max_value());

    pub fn chars_increasing(&self) -> RangeIncreasingChar {
        RangeIncreasingChar::new('\0', char::MAX)
    }

    pub fn chars_decreasing(&self) -> RangeDecreasingChar {
        RangeDecreasingChar::new('\0', char::MAX)
    }

    pub fn ascii_chars_increasing(&self) -> RangeIncreasingChar {
        RangeIncreasingChar::new('\0', char::from_u32(127).unwrap())
    }

    pub fn ascii_chars_decreasing(&self) -> RangeDecreasingChar {
        RangeDecreasingChar::new('\0', char::from_u32(127).unwrap())
    }

    pub fn chars(&self) -> Chars {
        match self {
            &IteratorProvider::Exhaustive => {
                Chars::Exhaustive(ExhaustiveChars::new(vec![RangeIncreasingChar::new('a', 'z'),
                                                            RangeIncreasingChar::new('A', 'Z'),
                                                            RangeIncreasingChar::new('0', '9'),
                                                            RangeIncreasingChar::new('!', '/'),
                                                            RangeIncreasingChar::new(':', '@'),
                                                            RangeIncreasingChar::new('[', '`'),
                                                            RangeIncreasingChar::new('{', '~'),
                                                            RangeIncreasingChar::new(' ', ' '),
                                                            RangeIncreasingChar::new('\0',
                                                                                     '\u{1F}'),
                                                            RangeIncreasingChar::new('\u{7F}',
                                                                                     char::MAX)]))
            }
            &IteratorProvider::Random(_, seed) => Chars::Random(SeedableRng::from_seed(&seed[..])),
        }
    }

    pub fn ascii_chars(&self) -> AsciiChars {
        match self {
            &IteratorProvider::Exhaustive => {
                AsciiChars::Exhaustive(ExhaustiveChars::new(vec!(
                                              RangeIncreasingChar::new('a', 'z'),
                                              RangeIncreasingChar::new('A', 'Z'),
                                              RangeIncreasingChar::new('0', '9'),
                                              RangeIncreasingChar::new('!', '/'),
                                              RangeIncreasingChar::new(':', '@'),
                                              RangeIncreasingChar::new('[', '`'),
                                              RangeIncreasingChar::new('{', '~'),
                                              RangeIncreasingChar::new(' ', ' '),
                                              RangeIncreasingChar::new('\0', '\u{1F}'),
                                              RangeIncreasingChar::new('\u{7F}', '\u{7F}'))))
            }
            &IteratorProvider::Random(_, seed) => {
                AsciiChars::Random(SeedableRng::from_seed(&seed[..]))
            }
        }
    }

    pub fn range_up_increasing_char(&self, a: char) -> RangeIncreasingChar {
        RangeIncreasingChar::new(a, char::MAX)
    }

    pub fn range_up_decreasing_char(&self, a: char) -> RangeDecreasingChar {
        RangeDecreasingChar::new(a, char::MAX)
    }

    pub fn range_down_increasing_char(&self, a: char) -> RangeIncreasingChar {
        RangeIncreasingChar::new('\0', a)
    }

    pub fn range_down_decreasing_char(&self, a: char) -> RangeDecreasingChar {
        RangeDecreasingChar::new('\0', a)
    }

    pub fn range_increasing_char(&self, a: char, b: char) -> RangeIncreasingChar {
        RangeIncreasingChar::new(a, b)
    }

    pub fn range_decreasing_char(&self, a: char, b: char) -> RangeDecreasingChar {
        RangeDecreasingChar::new(a, b)
    }

    pub fn range_up_increasing_integer(&self, a: Integer) -> RangeIncreasingUnboundedInteger {
        RangeIncreasingUnboundedInteger::new(a)
    }

    pub fn range_down_decreasing_integer(&self, b: Integer) -> RangeDecreasingUnboundedInteger {
        RangeDecreasingUnboundedInteger::new(b)
    }

    pub fn range_increasing_integer(&self, a: Integer, b: Integer) -> RangeIncreasingInteger {
        RangeIncreasingInteger::new(a, b)
    }

    pub fn range_decreasing_integer(&self, a: Integer, b: Integer) -> RangeDecreasingInteger {
        RangeDecreasingInteger::new(a, b)
    }

    pub fn range_integer(&self, a: Integer, b: Integer) -> RangeInteger {
        if a > b {
            panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
        }
        match self {
            &IteratorProvider::Exhaustive => {
                let xs = if a >= 0 {
                    ExhaustiveRangeInteger::AllNonNegative(RangeIncreasingInteger::new(a, b))
                } else if b <= 0 {
                    ExhaustiveRangeInteger::AllNonPositive(RangeDecreasingInteger::new(a, b))
                } else {
                    ExhaustiveRangeInteger::SomeOfEachSign(
                            once(Integer::from(0)).chain(
                                    RangeIncreasingInteger::new(Integer::from(1), b)
                                            .interleave(
                                                    RangeDecreasingInteger::new(
                                                            a,
                                                            Integer::from(-1)
                                                    )
                                            )
                            )
                    )
                };
                RangeInteger::Exhaustive(xs)
            }
            &IteratorProvider::Random(_, seed) => {
                let mut diameter = b - &a;
                diameter += 1;
                RangeInteger::Random(SeedableRng::from_seed(&seed[..]), diameter, a)
            }
        }
    }

    pub fn exhaustive_generate_from_vector<T>(&self, xs: Vec<T>) -> ExhaustiveFromVector<T> {
        let max = &xs.len() - 1;
        ExhaustiveFromVector {
            xs: xs,
            range: RangeIncreasingUsize::new(0, max),
        }
    }

    pub fn generate_from_vector<T>(&self, xs: Vec<T>) -> FromVector<T> {
        if xs.is_empty() {
            if let IteratorProvider::Random(_, _) = *self {
                panic!("Cannot randomly generate values from an empty Vec.");
            }
        }
        let max = &xs.len() - 1;
        FromVector {
            xs: xs,
            range: self.range_usize(0, max),
        }
    }

    pub fn orderings_increasing(&self) -> ExhaustiveFromVector<Ordering> {
        self.exhaustive_generate_from_vector(vec![Ordering::Less,
                                                  Ordering::Equal,
                                                  Ordering::Greater])
    }

    pub fn orderings(&self) -> FromVector<Ordering> {
        self.generate_from_vector(vec![Ordering::Equal, Ordering::Less, Ordering::Greater])
    }

    pub fn positive_u32s_geometric(&self, scale: u32) -> PositiveU32sGeometric {
        match self {
            &IteratorProvider::Exhaustive => {
                PositiveU32sGeometric::Exhaustive(RangeIncreasingU32::new(1, u32::max_value()))
            }
            &IteratorProvider::Random(_, seed) => {
                PositiveU32sGeometric::Random(RandomRangeU32::new(0, scale + 1, &seed))
            }
        }
    }

    pub fn natural_u32s_geometric(&self, scale: u32) -> NaturalU32sGeometric {
        match self {
            &IteratorProvider::Exhaustive => {
                NaturalU32sGeometric::Exhaustive(RangeIncreasingU32::new(0, u32::max_value()))
            }
            &IteratorProvider::Random(_, seed) => {
                NaturalU32sGeometric::Random(RandomRangeU32::new(0, scale + 1, &seed))
            }
        }
    }
}
