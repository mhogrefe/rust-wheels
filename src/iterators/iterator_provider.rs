#[cfg(feature = "gmp")]
use gmp_to_flint_adaptor_lib::integer::Integer;
#[cfg(feature = "native")]
use num_to_flint_adaptor_lib::integer::Integer;

use itertools::Interleave;
use itertools::Itertools;
use prim_utils::char_utils::*;
use prim_utils::traits::*;
use rand::{IsaacRng, Rand, Rng, SeedableRng};
use rand::distributions::{IndependentSample, Range};
use sha3::{Digest, Sha3_256};
use std::char;
use std::clone::Clone;
use std::cmp::Ordering;
use std::iter::*;
use std::marker::PhantomData;

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


pub struct ExhaustiveFromVector<T> {
    xs: Vec<T>,
    range: RangeIncreasing<usize>,
}

impl<T> ExhaustiveFromVector<T> {
    fn new(xs: Vec<T>) -> ExhaustiveFromVector<T> {
        let max = &xs.len() - 1;
        ExhaustiveFromVector {
            xs: xs,
            range: RangeIncreasing::new(0, max),
        }
    }
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.xs.len();
        (len, Some(len))
    }
}

pub enum Booleans {
    Exhaustive(ExhaustiveFromVector<bool>),
    Random(IsaacRng),
}

impl Booleans {
    pub fn exhaustive() -> Booleans {
        Booleans::Exhaustive(ExhaustiveFromVector::new(vec![false, true]))
    }

    pub fn random(seed: &[u32]) -> Booleans {
        Booleans::Random(SeedableRng::from_seed(seed))
    }
}

impl Iterator for Booleans {
    type Item = bool;

    fn next(&mut self) -> Option<bool> {
        match self {
            &mut Booleans::Exhaustive(ref mut xs) => xs.next(),
            &mut Booleans::Random(ref mut rng) => Some(rng.gen()),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            &Booleans::Exhaustive(_) => (2, Some(2)),
            &Booleans::Random(_) => (0, None),
        }
    }
}

pub struct RangeIncreasing<T: Walkable> {
    i: T,
    b: T,
    done: bool,
}

impl<T: Walkable> RangeIncreasing<T> {
    fn new(a: T, b: T) -> RangeIncreasing<T> {
        if a > b {
            panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
        }
        RangeIncreasing {
            i: a,
            b: b,
            done: false,
        }
    }
}

impl<T: Walkable> Iterator for RangeIncreasing<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.done {
            None
        } else {
            self.done = self.i == self.b;
            let ret = self.i;
            if !self.done {
                self.i.increment();
            }
            Some(ret)
        }
    }
}

pub struct RangeDecreasing<T: Walkable> {
    a: T,
    i: T,
    done: bool,
}

impl<T: Walkable> RangeDecreasing<T> {
    fn new(a: T, b: T) -> RangeDecreasing<T> {
        if a > b {
            panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
        }
        RangeDecreasing {
            a: a,
            i: b,
            done: false,
        }
    }
}

impl<T: Walkable> Iterator for RangeDecreasing<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.done {
            None
        } else {
            self.done = self.i == self.a;
            let ret = self.i;
            if !self.done {
                self.i.decrement();
            }
            Some(ret)
        }
    }
}

pub struct Random<T: Rand> {
    rng: IsaacRng,
    boo: PhantomData<T>,
}

impl<T: Rand> Random<T> {
    fn new(seed: &[u32]) -> Random<T> {
        Random {
            rng: SeedableRng::from_seed(seed),
            boo: PhantomData,
        }
    }
}

impl<T: Rand> Iterator for Random<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        Some(self.rng.gen::<T>())
    }
}

pub enum PositiveU<T: Rand + Walkable> {
    Exhaustive(RangeIncreasing<T>),
    Random(Random<T>),
}

impl<T: PrimUnsignedInt> PositiveU<T> {
    pub fn exhaustive() -> PositiveU<T> {
        PositiveU::Exhaustive(RangeIncreasing::new(T::from_u8(1), T::max_value()))
    }

    pub fn random(seed: &[u32]) -> PositiveU<T> {
        PositiveU::Random(Random::new(&seed))
    }
}

impl<T: PrimUnsignedInt> Iterator for PositiveU<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            &mut PositiveU::Exhaustive(ref mut xs) => xs.next(),
            &mut PositiveU::Random(ref mut xs) => {
                let zero = T::from_u8(0);
                loop {
                    let x = xs.next();
                    if x.is_none() || x.unwrap() != zero {
                        return x;
                    }
                }
            }
        }
    }
}

pub enum AllU<T: PrimUnsignedInt> {
    Exhaustive(RangeIncreasing<T>),
    Random(Random<T>),
}

impl<T: PrimUnsignedInt> AllU<T> {
    pub fn exhaustive() -> AllU<T> {
        AllU::Exhaustive(RangeIncreasing::new(T::from_u8(0), T::max_value()))
    }

    pub fn random(seed: &[u32]) -> AllU<T> {
        AllU::Random(Random::new(&seed))
    }
}

impl<T: PrimUnsignedInt> Iterator for AllU<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            &mut AllU::Exhaustive(ref mut xs) => xs.next(),
            &mut AllU::Random(ref mut xs) => xs.next(),
        }
    }
}

pub enum RandomRange<T: Rand> {
    Some(bool, IsaacRng, Range<T>),
    All(Random<T>),
}

impl<T: PrimInt> RandomRange<T> {
    pub fn new(a: T, b: T, seed: &[u32]) -> RandomRange<T> {
        if a == T::min_value() && b == T::max_value() {
            RandomRange::All(Random::new(&seed))
        } else if b == T::max_value() {
            RandomRange::Some(true,
                              SeedableRng::from_seed(&seed[..]),
                              Range::new(a - T::from_u8(1), b))
        } else {
            RandomRange::Some(false,
                              SeedableRng::from_seed(&seed[..]),
                              Range::new(a, b + T::from_u8(1)))
        }
    }
}

impl<T: PrimInt> Iterator for RandomRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            &mut RandomRange::Some(shift, ref mut rng, ref range) => {
                Some(if shift {
                         range.ind_sample(rng) + T::from_u8(1)
                     } else {
                         range.ind_sample(rng)
                     })
            }
            &mut RandomRange::All(ref mut xs) => xs.next(),
        }
    }
}

pub enum RangeU<T: Rand + Walkable> {
    Exhaustive(RangeIncreasing<T>),
    Random(RandomRange<T>),
}

impl<T: PrimUnsignedInt> RangeU<T> {
    pub fn exhaustive(a: T, b: T) -> RangeU<T> {
        RangeU::Exhaustive(RangeIncreasing::new(a, b))
    }

    pub fn random(a: T, b: T, seed: &[u32]) -> RangeU<T> {
        RangeU::Random(RandomRange::new(a, b, seed))
    }
}

impl<T: PrimUnsignedInt> Iterator for RangeU<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            &mut RangeU::Exhaustive(ref mut xs) => xs.next(),
            &mut RangeU::Random(ref mut xs) => xs.next(),
        }
    }
}

pub struct RandomFromVector<T> {
    xs: Vec<T>,
    range: RangeU<usize>,
}

impl<T> RandomFromVector<T> {
    pub fn new(xs: Vec<T>, seed: &[u32]) -> RandomFromVector<T> {
        let limit = &xs.len() - 1;
        RandomFromVector {
            xs: xs,
            range: RangeU::random(0, limit, seed),
        }
    }
}

impl<T: Clone> Iterator for RandomFromVector<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.range.next().map(|i| self.xs[i].clone())
    }
}

pub enum FromVector<T> {
    Exhaustive(ExhaustiveFromVector<T>),
    Random(RandomFromVector<T>),
}

impl<T> FromVector<T> {
    pub fn exhaustive(xs: Vec<T>) -> FromVector<T> {
        FromVector::Exhaustive(ExhaustiveFromVector::new(xs))
    }

    pub fn random(xs: Vec<T>, seed: &[u32]) -> FromVector<T> {
        FromVector::Random(RandomFromVector::new(xs, seed))
    }
}

impl<T: Clone> Iterator for FromVector<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            &mut FromVector::Exhaustive(ref mut xs) => xs.next(),
            &mut FromVector::Random(ref mut xs) => xs.next(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        match self {
            &FromVector::Exhaustive(ExhaustiveFromVector { ref xs, range: _ }) => {
                let len = xs.len();
                (len, Some(len))
            }
            &FromVector::Random(_) => (0, None),
        }
    }
}

pub enum PositiveI<T: PrimSignedInt> {
    Exhaustive(RangeIncreasing<T>),
    Random(Random<T>),
}

impl<T: PrimSignedInt> PositiveI<T> {
    pub fn exhaustive() -> PositiveI<T> {
        PositiveI::Exhaustive(RangeIncreasing::new(T::from_u8(1), T::max_value()))
    }

    pub fn random(seed: &[u32]) -> PositiveI<T> {
        PositiveI::Random(Random::new(&seed))
    }
}

impl<T: PrimSignedInt> Iterator for PositiveI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            &mut PositiveI::Exhaustive(ref mut xs) => xs.next(),
            &mut PositiveI::Random(ref mut xs) => {
                let zero = T::from_u8(0);
                loop {
                    let x = xs.next().map(|x| x & T::max_value());
                    if x.is_none() || x.unwrap() != zero {
                        return x;
                    }
                }
            }
        }
    }
}

pub enum NegativeI<T: PrimSignedInt> {
    Exhaustive(RangeDecreasing<T>),
    Random(Random<T>),
}

impl<T: PrimSignedInt> NegativeI<T> {
    pub fn exhaustive() -> NegativeI<T> {
        NegativeI::Exhaustive(RangeDecreasing::new(T::min_value(), T::from_i8(-1)))
    }

    pub fn random(seed: &[u32]) -> NegativeI<T> {
        NegativeI::Random(Random::new(&seed))
    }
}

impl<T: PrimSignedInt> Iterator for NegativeI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            &mut NegativeI::Exhaustive(ref mut xs) => xs.next(),
            &mut NegativeI::Random(ref mut xs) => xs.next().map(|x| !(x & T::max_value())),
        }
    }
}

pub enum NaturalI<T: PrimSignedInt> {
    Exhaustive(RangeIncreasing<T>),
    Random(Random<T>),
}

impl<T: PrimSignedInt> NaturalI<T> {
    pub fn exhaustive() -> NaturalI<T> {
        NaturalI::Exhaustive(RangeIncreasing::new(T::from_u8(0), T::max_value()))
    }

    pub fn random(seed: &[u32]) -> NaturalI<T> {
        NaturalI::Random(Random::new(&seed))
    }
}

impl<T: PrimSignedInt> Iterator for NaturalI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            &mut NaturalI::Exhaustive(ref mut xs) => xs.next(),
            &mut NaturalI::Random(ref mut xs) => xs.next().map(|x| x & T::max_value()),
        }
    }
}

pub enum NonzeroI<T: PrimSignedInt> {
    Exhaustive(Interleave<PositiveI<T>, NegativeI<T>>),
    Random(Random<T>),
}

impl<T: PrimSignedInt> NonzeroI<T> {
    pub fn exhaustive() -> NonzeroI<T> {
        NonzeroI::Exhaustive(PositiveI::exhaustive().interleave(NegativeI::exhaustive()))
    }

    pub fn random(seed: &[u32]) -> NonzeroI<T> {
        NonzeroI::Random(Random::new(&seed))
    }
}

impl<T: PrimSignedInt> Iterator for NonzeroI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            &mut NonzeroI::Exhaustive(ref mut xs) => xs.next(),
            &mut NonzeroI::Random(ref mut xs) => {
                let zero = T::from_u8(0);
                loop {
                    let x = xs.next();
                    if x.is_none() || x.unwrap() != zero {
                        return x;
                    }
                }
            }
        }
    }
}

pub enum AllI<T: PrimSignedInt> {
    Exhaustive(Chain<Once<T>, NonzeroI<T>>),
    Random(Random<T>),
}

impl<T: PrimSignedInt> AllI<T> {
    pub fn exhaustive() -> AllI<T> {
        AllI::Exhaustive(once(T::from_u8(0)).chain(NonzeroI::exhaustive()))
    }

    pub fn random(seed: &[u32]) -> AllI<T> {
        AllI::Random(Random::new(&seed))
    }
}

impl<T: PrimSignedInt> Iterator for AllI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            &mut AllI::Exhaustive(ref mut xs) => xs.next(),
            &mut AllI::Random(ref mut xs) => xs.next(),
        }
    }
}

pub enum ExhaustiveRangeI<T: PrimSignedInt> {
    AllNonNegative(RangeIncreasing<T>),
    AllNonPositive(RangeDecreasing<T>),
    SomeOfEachSign(Chain<Once<T>, Interleave<RangeIncreasing<T>, RangeDecreasing<T>>>),
}

impl<T: PrimSignedInt> ExhaustiveRangeI<T> {
    pub fn new(a: T, b: T) -> ExhaustiveRangeI<T> {
        let zero = T::from_u8(0);
        if a >= zero {
            ExhaustiveRangeI::AllNonNegative(RangeIncreasing::new(a, b))
        } else if b <= zero {
            ExhaustiveRangeI::AllNonPositive(RangeDecreasing::new(a, b))
        } else {
            ExhaustiveRangeI::SomeOfEachSign(
                    once(zero).chain(
                            RangeIncreasing::new(T::from_u8(1), b).interleave(
                                    RangeDecreasing::new(a, T::from_i8(-1))
                            )
                    )
            )
        }
    }
}

impl<T: PrimSignedInt> Iterator for ExhaustiveRangeI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            &mut ExhaustiveRangeI::AllNonNegative(ref mut xs) => xs.next(),
            &mut ExhaustiveRangeI::AllNonPositive(ref mut xs) => xs.next(),
            &mut ExhaustiveRangeI::SomeOfEachSign(ref mut xs) => xs.next(),
        }
    }
}

pub enum RangeI<T: PrimSignedInt> {
    Exhaustive(ExhaustiveRangeI<T>),
    Random(RandomRange<T>),
}

impl<T: PrimSignedInt> RangeI<T> {
    pub fn exhaustive(a: T, b: T) -> RangeI<T> {
        RangeI::Exhaustive(ExhaustiveRangeI::new(a, b))
    }

    pub fn random(a: T, b: T, seed: &[u32]) -> RangeI<T> {
        RangeI::Random(RandomRange::new(a, b, seed))
    }
}

impl<T: PrimSignedInt> Iterator for RangeI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match self {
            &mut RangeI::Exhaustive(ref mut xs) => xs.next(),
            &mut RangeI::Random(ref mut xs) => xs.next(),
        }
    }
}

pub struct ExhaustiveChars {
    ranges: Vec<RangeIncreasing<char>>,
    i: usize,
}

impl ExhaustiveChars {
    pub fn new(ranges: Vec<RangeIncreasing<char>>) -> ExhaustiveChars {
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

pub enum RangeChar {
    Exhaustive(RangeIncreasing<char>),
    Random(IsaacRng, Range<u32>),
}

impl Iterator for RangeChar {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        match self {
            &mut RangeChar::Exhaustive(ref mut it) => it.next(),
            &mut RangeChar::Random(ref mut rng, ref range) => {
                contiguous_range_to_char(range.ind_sample(rng))
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

pub enum PositiveU32sGeometric {
    Exhaustive(RangeIncreasing<u32>),
    Random(RandomRange<u32>),
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
    Exhaustive(RangeIncreasing<u32>),
    Random(RandomRange<u32>),
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
            &IteratorProvider::Exhaustive => Booleans::exhaustive(),
            &IteratorProvider::Random(_, seed) => Booleans::random(&seed[..]),
        }
    }

    pub fn range_up_increasing_x<T: PrimInt>(&self, a: T) -> RangeIncreasing<T> {
        RangeIncreasing::new(a, T::max_value())
    }

    pub fn range_up_decreasing_x<T: PrimInt>(&self, a: T) -> RangeDecreasing<T> {
        RangeDecreasing::new(a, T::max_value())
    }

    pub fn range_down_increasing_x<T: PrimInt>(&self, b: T) -> RangeIncreasing<T> {
        RangeIncreasing::new(T::min_value(), b)
    }

    pub fn range_down_decreasing_x<T: PrimInt>(&self, b: T) -> RangeDecreasing<T> {
        RangeDecreasing::new(T::min_value(), b)
    }

    pub fn range_increasing_x<T: PrimInt>(&self, a: T, b: T) -> RangeIncreasing<T> {
        RangeIncreasing::new(a, b)
    }

    pub fn range_decreasing_x<T: PrimInt>(&self, a: T, b: T) -> RangeDecreasing<T> {
        RangeDecreasing::new(a, b)
    }

    pub fn x_increasing<T: PrimInt>(&self) -> RangeIncreasing<T> {
        RangeIncreasing::new(T::min_value(), T::max_value())
    }

    pub fn x_decreasing<T: PrimInt>(&self) -> RangeDecreasing<T> {
        RangeDecreasing::new(T::min_value(), T::max_value())
    }

    pub fn positive_u<T: PrimUnsignedInt>(&self) -> PositiveU<T> {
        match self {
            &IteratorProvider::Exhaustive => PositiveU::exhaustive(),
            &IteratorProvider::Random(_, seed) => PositiveU::random(&seed[..]),
        }
    }

    pub fn all_u<T: PrimUnsignedInt>(&self) -> AllU<T> {
        match self {
            &IteratorProvider::Exhaustive => AllU::exhaustive(),
            &IteratorProvider::Random(_, seed) => AllU::random(&seed[..]),
        }
    }

    pub fn range_up_u<T: PrimUnsignedInt>(&self, a: T) -> RangeU<T> {
        self.range_u(a, T::max_value())
    }

    pub fn range_down_u<T: PrimUnsignedInt>(&self, a: T) -> RangeU<T> {
        self.range_u(T::from_u8(0), a)
    }

    pub fn range_u<T: PrimUnsignedInt>(&self, a: T, b: T) -> RangeU<T> {
        if a > b {
            panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
        }
        match self {
            &IteratorProvider::Exhaustive => RangeU::exhaustive(a, b),
            &IteratorProvider::Random(_, seed) => RangeU::random(a, b, &seed[..]),
        }
    }

    pub fn positive_i<T: PrimSignedInt>(&self) -> PositiveI<T> {
        match self {
            &IteratorProvider::Exhaustive => PositiveI::exhaustive(),
            &IteratorProvider::Random(_, seed) => PositiveI::random(&seed[..]),
        }
    }

    pub fn negative_i<T: PrimSignedInt>(&self) -> NegativeI<T> {
        match self {
            &IteratorProvider::Exhaustive => NegativeI::exhaustive(),
            &IteratorProvider::Random(_, seed) => NegativeI::random(&seed[..]),
        }
    }

    pub fn natural_i<T: PrimSignedInt>(&self) -> NaturalI<T> {
        match self {
            &IteratorProvider::Exhaustive => NaturalI::exhaustive(),
            &IteratorProvider::Random(_, seed) => NaturalI::random(&seed[..]),
        }
    }

    pub fn nonzero_i<T: PrimSignedInt>(&self) -> NonzeroI<T> {
        match self {
            &IteratorProvider::Exhaustive => NonzeroI::exhaustive(),
            &IteratorProvider::Random(_, seed) => NonzeroI::random(&seed[..]),
        }
    }

    pub fn all_i<T: PrimSignedInt>(&self) -> AllI<T> {
        match self {
            &IteratorProvider::Exhaustive => AllI::exhaustive(),
            &IteratorProvider::Random(_, seed) => AllI::random(&seed[..]),
        }
    }

    pub fn range_up_i<T: PrimSignedInt>(&self, a: T) -> RangeI<T> {
        self.range_i(a, T::max_value())
    }

    pub fn range_down_i<T: PrimSignedInt>(&self, a: T) -> RangeI<T> {
        self.range_i(T::min_value(), a)
    }

    pub fn range_i<T: PrimSignedInt>(&self, a: T, b: T) -> RangeI<T> {
        if a > b {
            panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
        }
        match self {
            &IteratorProvider::Exhaustive => RangeI::exhaustive(a, b),
            &IteratorProvider::Random(_, seed) => RangeI::random(a, b, &seed[..]),
        }
    }

    pub fn chars_increasing(&self) -> RangeIncreasing<char> {
        RangeIncreasing::new('\0', char::MAX)
    }

    pub fn chars_decreasing(&self) -> RangeDecreasing<char> {
        RangeDecreasing::new('\0', char::MAX)
    }

    pub fn ascii_chars_increasing(&self) -> RangeIncreasing<char> {
        RangeIncreasing::new('\0', char::from_u32(127).unwrap())
    }

    pub fn ascii_chars_decreasing(&self) -> RangeDecreasing<char> {
        RangeDecreasing::new('\0', char::from_u32(127).unwrap())
    }

    pub fn chars(&self) -> Chars {
        match self {
            &IteratorProvider::Exhaustive => {
                Chars::Exhaustive(ExhaustiveChars::new(vec![RangeIncreasing::new('a', 'z'),
                                                            RangeIncreasing::new('A', 'Z'),
                                                            RangeIncreasing::new('0', '9'),
                                                            RangeIncreasing::new('!', '/'),
                                                            RangeIncreasing::new(':', '@'),
                                                            RangeIncreasing::new('[', '`'),
                                                            RangeIncreasing::new('{', '~'),
                                                            RangeIncreasing::new(' ', ' '),
                                                            RangeIncreasing::new('\0', '\u{1F}'),
                                                            RangeIncreasing::new('\u{7F}',
                                                                                 char::MAX)]))
            }
            &IteratorProvider::Random(_, seed) => Chars::Random(SeedableRng::from_seed(&seed[..])),
        }
    }

    pub fn ascii_chars(&self) -> AsciiChars {
        match self {
            &IteratorProvider::Exhaustive => {
                AsciiChars::Exhaustive(ExhaustiveChars::new(vec![RangeIncreasing::new('a', 'z'),
                                                                 RangeIncreasing::new('A', 'Z'),
                                                                 RangeIncreasing::new('0', '9'),
                                                                 RangeIncreasing::new('!', '/'),
                                                                 RangeIncreasing::new(':', '@'),
                                                                 RangeIncreasing::new('[', '`'),
                                                                 RangeIncreasing::new('{', '~'),
                                                                 RangeIncreasing::new(' ', ' '),
                                                                 RangeIncreasing::new('\0',
                                                                                      '\u{1F}'),
                                                                 RangeIncreasing::new('\u{7F}',
                                                                                      '\u{7F}')]))
            }
            &IteratorProvider::Random(_, seed) => {
                AsciiChars::Random(SeedableRng::from_seed(&seed[..]))
            }
        }
    }

    pub fn range_up_char(&self, a: char) -> RangeChar {
        self.range_char(a, char::MAX)
    }

    pub fn range_down_char(&self, a: char) -> RangeChar {
        self.range_char('\0', a)
    }

    pub fn range_char(&self, a: char, b: char) -> RangeChar {
        if a > b {
            panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
        }
        match self {
            &IteratorProvider::Exhaustive => RangeChar::Exhaustive(RangeIncreasing::new(a, b)),
            &IteratorProvider::Random(_, seed) => {
                RangeChar::Random(SeedableRng::from_seed(&seed[..]),
                                  Range::new(char_to_contiguous_range(a),
                                             char_to_contiguous_range(b) + 1))
            }
        }
    }

    pub fn range_up_increasing_char(&self, a: char) -> RangeIncreasing<char> {
        RangeIncreasing::new(a, char::MAX)
    }

    pub fn range_up_decreasing_char(&self, a: char) -> RangeDecreasing<char> {
        RangeDecreasing::new(a, char::MAX)
    }

    pub fn range_down_increasing_char(&self, a: char) -> RangeIncreasing<char> {
        RangeIncreasing::new('\0', a)
    }

    pub fn range_down_decreasing_char(&self, a: char) -> RangeDecreasing<char> {
        RangeDecreasing::new('\0', a)
    }

    pub fn range_increasing_char(&self, a: char, b: char) -> RangeIncreasing<char> {
        RangeIncreasing::new(a, b)
    }

    pub fn range_decreasing_char(&self, a: char, b: char) -> RangeDecreasing<char> {
        RangeDecreasing::new(a, b)
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
        ExhaustiveFromVector::new(xs)
    }

    pub fn generate_from_vector<T>(&self, xs: Vec<T>) -> FromVector<T> {
        if xs.is_empty() {
            if let IteratorProvider::Random(_, _) = *self {
                panic!("Cannot randomly generate values from an empty Vec.");
            }
        }
        match self {
            &IteratorProvider::Exhaustive => FromVector::exhaustive(xs),
            &IteratorProvider::Random(_, seed) => FromVector::random(xs, &seed[..]),
        }
    }

    pub fn orderings_increasing(&self) -> ExhaustiveFromVector<Ordering> {
        ExhaustiveFromVector::new(vec![Ordering::Less, Ordering::Equal, Ordering::Greater])
    }

    pub fn orderings(&self) -> FromVector<Ordering> {
        self.generate_from_vector(vec![Ordering::Equal, Ordering::Less, Ordering::Greater])
    }

    pub fn positive_u32s_geometric(&self, scale: u32) -> PositiveU32sGeometric {
        match self {
            &IteratorProvider::Exhaustive => {
                PositiveU32sGeometric::Exhaustive(RangeIncreasing::new(1, u32::max_value()))
            }
            &IteratorProvider::Random(_, seed) => {
                PositiveU32sGeometric::Random(RandomRange::new(0, scale + 1, &seed))
            }
        }
    }

    pub fn natural_u32s_geometric(&self, scale: u32) -> NaturalU32sGeometric {
        match self {
            &IteratorProvider::Exhaustive => {
                NaturalU32sGeometric::Exhaustive(RangeIncreasing::new(0, u32::max_value()))
            }
            &IteratorProvider::Random(_, seed) => {
                NaturalU32sGeometric::Random(RandomRange::new(0, scale + 1, &seed))
            }
        }
    }
}
