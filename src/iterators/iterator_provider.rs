#[cfg(feature = "gmp")]
use malachite_gmp::integer::Integer;
#[cfg(feature = "native")]
use malachite_native::integer::Integer;

use iterators::adaptors::MultiChain;
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

pub const SEED_SIZE: usize = 256;

pub const EXAMPLE_SEED: [u32; SEED_SIZE] =
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
        let byte_1 = hash[j] as u32;
        let byte_2 = hash[j + 1] as u32;
        let byte_3 = hash[j + 2] as u32;
        let byte_4 = hash[j + 3] as u32;
        scrambled_seed[i] = seed[i] ^ (byte_1 << 24 | byte_2 << 16 | byte_3 << 8 | byte_4)
    }
    scrambled_seed
}

pub struct ExhaustiveFromVector<T> {
    xs: Vec<T>,
    i: usize,
}

pub fn exhaustive_from_vector<T>(xs: Vec<T>) -> ExhaustiveFromVector<T> {
    ExhaustiveFromVector { xs: xs, i: 0 }
}

impl<T: Clone> Iterator for ExhaustiveFromVector<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.i == self.xs.len() {
            None
        } else {
            let next = self.xs[self.i].clone();
            self.i += 1;
            Some(next)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.xs.len();
        (len, Some(len))
    }
}

pub struct RandomFromVector<T> {
    xs: Vec<T>,
    range: RandomRange<usize>,
}

pub fn random_from_vector<T>(xs: Vec<T>, seed: &[u32]) -> RandomFromVector<T> {
    if xs.is_empty() {
        panic!("Cannot randomly generate values from an empty Vec.");
    }
    let limit = &xs.len() - 1;
    RandomFromVector {
        xs: xs,
        range: random_range(0, limit, seed),
    }
}

impl<T: Clone> Iterator for RandomFromVector<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.range.next().map(|i| self.xs[i].clone())
    }
}

pub fn exhaustive_bools() -> ExhaustiveFromVector<bool> {
    exhaustive_from_vector(vec![false, true])
}

pub struct RangeIncreasing<T: Walkable> {
    i: T,
    b: T,
    done: bool,
}

pub fn range_increasing_x<T: Walkable>(a: T, b: T) -> RangeIncreasing<T> {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RangeIncreasing {
        i: a,
        b: b,
        done: false,
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

pub fn range_decreasing_x<T: Walkable>(a: T, b: T) -> RangeDecreasing<T> {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RangeDecreasing {
        a: a,
        i: b,
        done: false,
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

pub fn range_up_increasing_x<T: PrimInt>(a: T) -> RangeIncreasing<T> {
    range_increasing_x(a, T::max_value())
}

pub fn range_up_decreasing_x<T: PrimInt>(a: T) -> RangeDecreasing<T> {
    range_decreasing_x(a, T::max_value())
}

pub fn range_down_increasing_x<T: PrimInt>(b: T) -> RangeIncreasing<T> {
    range_increasing_x(T::min_value(), b)
}

pub fn range_down_decreasing_x<T: PrimInt>(b: T) -> RangeDecreasing<T> {
    range_decreasing_x(T::min_value(), b)
}

pub fn x_increasing<T: PrimInt>() -> RangeIncreasing<T> {
    range_increasing_x(T::min_value(), T::max_value())
}

pub fn x_decreasing<T: PrimInt>() -> RangeDecreasing<T> {
    range_decreasing_x(T::min_value(), T::max_value())
}

pub struct Random<T: Rand> {
    rng: IsaacRng,
    boo: PhantomData<T>,
}

pub fn random_x<T: Rand>(seed: &[u32]) -> Random<T> {
    Random {
        rng: SeedableRng::from_seed(seed),
        boo: PhantomData,
    }
}

impl<T: Rand> Iterator for Random<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        Some(self.rng.gen::<T>())
    }
}

pub struct RandomPositiveU<T: Rand>(Random<T>);

pub fn exhaustive_positive_x<T: PrimInt>() -> RangeIncreasing<T> {
    range_increasing_x(T::from_u8(1), T::max_value())
}

pub fn random_positive_u<T: Rand>(seed: &[u32]) -> RandomPositiveU<T> {
    RandomPositiveU(random_x(seed))
}

impl<T: PrimUnsignedInt> Iterator for RandomPositiveU<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let zero = T::from_u8(0);
        loop {
            let x = self.0.next();
            if x.is_none() || x.unwrap() != zero {
                return x;
            }
        }
    }
}

pub struct RandomPositiveI<T: PrimSignedInt>(Random<T>);

pub fn random_positive_i<T: PrimSignedInt>(seed: &[u32]) -> RandomPositiveI<T> {
    RandomPositiveI(random_x(seed))
}

impl<T: PrimSignedInt> Iterator for RandomPositiveI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let zero = T::from_u8(0);
        loop {
            let x = self.0.next().map(|x| x & T::max_value());
            if x.is_none() || x.unwrap() != zero {
                return x;
            }
        }
    }
}

pub fn exhaustive_u<T: PrimUnsignedInt>() -> RangeIncreasing<T> {
    range_increasing_x(T::from_u8(0), T::max_value())
}

pub enum RandomRange<T: Rand> {
    Some(bool, IsaacRng, Range<T>),
    All(Random<T>),
}

pub fn random_range<T: PrimInt>(a: T, b: T, seed: &[u32]) -> RandomRange<T> {
    if a == T::min_value() && b == T::max_value() {
        RandomRange::All(random_x(seed))
    } else if b == T::max_value() {
        RandomRange::Some(true,
                          SeedableRng::from_seed(seed),
                          Range::new(a - T::from_u8(1), b))
    } else {
        RandomRange::Some(false,
                          SeedableRng::from_seed(seed),
                          Range::new(a, b + T::from_u8(1)))
    }
}

impl<T: PrimInt> Iterator for RandomRange<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        match *self {
            RandomRange::Some(shift, ref mut rng, ref range) => {
                Some(if shift {
                         range.ind_sample(rng) + T::from_u8(1)
                     } else {
                         range.ind_sample(rng)
                     })
            }
            RandomRange::All(ref mut xs) => xs.next(),
        }
    }
}

pub fn random_range_up<T: PrimInt>(a: T, seed: &[u32]) -> RandomRange<T> {
    random_range(a, T::max_value(), seed)
}

pub fn random_range_down<T: PrimInt>(a: T, seed: &[u32]) -> RandomRange<T> {
    random_range(T::min_value(), a, seed)
}

pub fn exhaustive_negative_i<T: PrimSignedInt>() -> RangeDecreasing<T> {
    range_decreasing_x(T::min_value(), T::from_i8(-1))
}

pub struct RandomNegativeI<T: PrimSignedInt>(Random<T>);

pub fn random_negative_i<T: PrimSignedInt>(seed: &[u32]) -> RandomNegativeI<T> {
    RandomNegativeI(random_x(seed))
}

impl<T: PrimSignedInt> Iterator for RandomNegativeI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.next().map(|x| !(x & T::max_value()))
    }
}

pub fn exhaustive_natural_i<T: PrimSignedInt>() -> RangeIncreasing<T> {
    range_increasing_x(T::from_u8(0), T::max_value())
}

pub struct RandomNaturalI<T: PrimSignedInt>(Random<T>);

pub fn random_natural_i<T: PrimSignedInt>(seed: &[u32]) -> RandomNaturalI<T> {
    RandomNaturalI(random_x(seed))
}

impl<T: PrimSignedInt> Iterator for RandomNaturalI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.0.next().map(|x| x & T::max_value())
    }
}

pub fn exhaustive_nonzero_i<T: PrimSignedInt>
    ()
    -> Interleave<RangeIncreasing<T>, RangeDecreasing<T>>
{
    exhaustive_positive_x().interleave(exhaustive_negative_i())
}

pub struct RandomNonzeroI<T: PrimSignedInt>(Random<T>);

pub fn random_nonzero_i<T: PrimSignedInt>(seed: &[u32]) -> RandomNonzeroI<T> {
    RandomNonzeroI(random_x(seed))
}

impl<T: PrimSignedInt> Iterator for RandomNonzeroI<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        let zero = T::from_u8(0);
        loop {
            let x = self.0.next();
            if x.is_none() || x.unwrap() != zero {
                return x;
            }
        }
    }
}

pub fn exhaustive_i<T: PrimSignedInt>
    ()
    -> Chain<Once<T>, Interleave<RangeIncreasing<T>, RangeDecreasing<T>>>
{
    once(T::from_u8(0)).chain(exhaustive_nonzero_i())
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
            ExhaustiveRangeI::AllNonNegative(range_increasing_x(a, b))
        } else if b <= zero {
            ExhaustiveRangeI::AllNonPositive(range_decreasing_x(a, b))
        } else {
            ExhaustiveRangeI::SomeOfEachSign(
                    once(zero).chain(
                            range_increasing_x(T::from_u8(1), b).interleave(
                                    range_decreasing_x(a, T::from_i8(-1))
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
        RangeI::Random(random_range(a, b, seed))
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

pub fn exhaustive_chars() -> MultiChain<RangeIncreasing<char>> {
    MultiChain::new(vec![range_increasing_x('a', 'z'),
                         range_increasing_x('A', 'Z'),
                         range_increasing_x('0', '9'),
                         range_increasing_x('!', '/'),
                         range_increasing_x(':', '@'),
                         range_increasing_x('[', '`'),
                         range_increasing_x('{', '~'),
                         range_increasing_x(' ', ' '),
                         range_increasing_x('\0', '\u{1F}'),
                         range_increasing_x('\u{7F}', char::MAX)])
}

pub fn exhaustive_ascii_chars() -> MultiChain<RangeIncreasing<char>> {
    MultiChain::new(vec![range_increasing_x('a', 'z'),
                         range_increasing_x('A', 'Z'),
                         range_increasing_x('0', '9'),
                         range_increasing_x('!', '/'),
                         range_increasing_x(':', '@'),
                         range_increasing_x('[', '`'),
                         range_increasing_x('{', '~'),
                         range_increasing_x(' ', ' '),
                         range_increasing_x('\0', '\u{1F}'),
                         range_increasing_x('\u{7F}', '\u{7F}')])
}

pub struct RandomAsciiChars(IsaacRng);

pub fn random_ascii_chars(seed: &[u32]) -> RandomAsciiChars {
    RandomAsciiChars(SeedableRng::from_seed(seed))
}

impl Iterator for RandomAsciiChars {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        Some(char::from_u32((self.0.gen::<u8>() & 0x7f) as u32).unwrap())
    }
}

pub struct RandomRangeChar {
    rng: IsaacRng,
    range: Range<u32>,
}

pub fn random_range_char(a: char, b: char, seed: &[u32]) -> RandomRangeChar {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RandomRangeChar {
        rng: SeedableRng::from_seed(seed),
        range: Range::new(char_to_contiguous_range(a), char_to_contiguous_range(b) + 1),
    }
}

impl Iterator for RandomRangeChar {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        contiguous_range_to_char(self.range.ind_sample(&mut self.rng))
    }
}

pub fn exhaustive_range_up_char(a: char) -> RangeIncreasing<char> {
    range_increasing_x(a, char::MAX)
}

pub fn exhaustive_range_down_char(a: char) -> RangeIncreasing<char> {
    range_increasing_x('\0', a)
}

pub fn random_range_up_char(a: char, seed: &[u32]) -> RandomRangeChar {
    random_range_char(a, char::MAX, seed)
}

pub fn random_range_down_char(a: char, seed: &[u32]) -> RandomRangeChar {
    random_range_char('\0', a, seed)
}

pub struct RangeIncreasingInteger {
    i: Integer,
    b: Integer,
    done: bool,
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

impl Iterator for ExhaustiveRangeInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        match self {
            &mut ExhaustiveRangeInteger::AllNonNegative(ref mut xs) => xs.next(),
            &mut ExhaustiveRangeInteger::AllNonPositive(ref mut xs) => xs.next(),
            &mut ExhaustiveRangeInteger::SomeOfEachSign(ref mut xs) => xs.next(),
        }
    }
}

pub struct RandomRangeInteger {
    rng: IsaacRng,
    diameter: Integer,
    a: Integer,
}

impl Iterator for RandomRangeInteger {
    type Item = Integer;

    fn next(&mut self) -> Option<Integer> {
        let mut random = self.diameter.clone();
        random.random_below(&mut self.rng);
        random += &self.a;
        Some(random)
    }
}

pub fn range_up_increasing_integer(a: Integer) -> RangeIncreasingUnboundedInteger {
    RangeIncreasingUnboundedInteger { i: a }
}

pub fn range_down_decreasing_integer(a: Integer) -> RangeDecreasingUnboundedInteger {
    RangeDecreasingUnboundedInteger { i: a }
}

pub fn range_increasing_integer(a: Integer, b: Integer) -> RangeIncreasingInteger {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RangeIncreasingInteger {
        i: a,
        b: b,
        done: false,
    }
}

pub fn range_decreasing_integer(a: Integer, b: Integer) -> RangeDecreasingInteger {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RangeDecreasingInteger {
        a: a,
        i: b,
        done: false,
    }
}

pub fn exhaustive_range_integer(a: Integer, b: Integer) -> ExhaustiveRangeInteger {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    if a >= 0 {
        ExhaustiveRangeInteger::AllNonNegative(range_increasing_integer(a, b))
    } else if b <= 0 {
        ExhaustiveRangeInteger::AllNonPositive(range_decreasing_integer(a, b))
    } else {
        ExhaustiveRangeInteger::SomeOfEachSign(
                once(Integer::from(0)).chain(
                    range_increasing_integer(Integer::from(1), b)
                        .interleave(
                            range_decreasing_integer(
                                a,
                                Integer::from(-1)
                            )
                        )
                )
            )
    }
}

pub fn random_range_integer(seed: &[u32], a: Integer, b: Integer) -> RandomRangeInteger {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    let mut diameter = b - &a;
    diameter += 1;
    RandomRangeInteger {
        rng: SeedableRng::from_seed(seed),
        diameter,
        a,
    }
}

pub struct PositiveU32sGeometric(RandomRange<u32>);

impl Iterator for PositiveU32sGeometric {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let mut j = 0;
        loop {
            j += 1;
            if self.0.next().unwrap() == 0 {
                return Some(j);
            }
        }
    }
}

pub fn positive_u32s_geometric(seed: &[u32], scale: u32) -> PositiveU32sGeometric {
    PositiveU32sGeometric(random_range(0, scale + 1, seed))
}

pub struct NaturalU32sGeometric(RandomRange<u32>);

impl Iterator for NaturalU32sGeometric {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let mut j = 0;
        loop {
            if self.0.next().unwrap() == 0 {
                return Some(j);
            }
            j += 1;
        }
    }
}

pub fn natural_u32s_geometric(seed: &[u32], scale: u32) -> NaturalU32sGeometric {
    NaturalU32sGeometric(random_range(0, scale + 1, seed))
}

pub struct NegativeI32sGeometric(PositiveU32sGeometric);

impl Iterator for NegativeI32sGeometric {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.0.next().map(|i| -(i as i32))
    }
}

pub fn negative_i32s_geometric(seed: &[u32], scale: u32) -> NegativeI32sGeometric {
    NegativeI32sGeometric(positive_u32s_geometric(seed, scale))
}

pub enum NonzeroI32sGeometric {
    Exhaustive(Interleave<RangeIncreasing<i32>, RangeDecreasing<i32>>),
    Random(PositiveU32sGeometric, Random<bool>),
}

impl NonzeroI32sGeometric {
    pub fn exhaustive() -> NonzeroI32sGeometric {
        NonzeroI32sGeometric::Exhaustive(exhaustive_nonzero_i())
    }
}

impl Iterator for NonzeroI32sGeometric {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        match self {
            &mut NonzeroI32sGeometric::Exhaustive(ref mut xs) => xs.next(),
            &mut NonzeroI32sGeometric::Random(ref mut us, ref mut bs) => {
                if bs.next().unwrap() {
                    us.next().map(|i| i as i32)
                } else {
                    us.next().map(|i| -(i as i32))
                }
            }
        }
    }
}

pub enum I32sGeometric {
    Exhaustive(Chain<Once<i32>, Interleave<RangeIncreasing<i32>, RangeDecreasing<i32>>>),
    Random(NaturalU32sGeometric, Random<bool>),
}

impl I32sGeometric {
    pub fn exhaustive() -> I32sGeometric {
        I32sGeometric::Exhaustive(exhaustive_i())
    }
}

impl Iterator for I32sGeometric {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        match self {
            &mut I32sGeometric::Exhaustive(ref mut xs) => xs.next(),
            &mut I32sGeometric::Random(ref mut us, ref mut bs) => {
                if bs.next().unwrap() {
                    us.next().map(|i| i as i32)
                } else {
                    us.next().map(|i| -(i as i32))
                }
            }
        }
    }
}

pub fn orderings_increasing() -> ExhaustiveFromVector<Ordering> {
    exhaustive_from_vector(vec![Ordering::Less, Ordering::Equal, Ordering::Greater])
}

pub fn exhaustive_orderings() -> ExhaustiveFromVector<Ordering> {
    exhaustive_from_vector(vec![Ordering::Equal, Ordering::Less, Ordering::Greater])
}

pub fn random_orderings(seed: &[u32]) -> RandomFromVector<Ordering> {
    random_from_vector(vec![Ordering::Equal, Ordering::Less, Ordering::Greater],
                       seed)
}

pub fn chars_increasing() -> RangeIncreasing<char> {
    range_increasing_x('\0', char::MAX)
}

pub fn chars_decreasing() -> RangeDecreasing<char> {
    range_decreasing_x('\0', char::MAX)
}

pub fn ascii_chars_increasing() -> RangeIncreasing<char> {
    range_increasing_x('\0', char::from_u32(127).unwrap())
}

pub fn ascii_chars_decreasing() -> RangeDecreasing<char> {
    range_decreasing_x('\0', char::from_u32(127).unwrap())
}

pub fn range_up_increasing_char(a: char) -> RangeIncreasing<char> {
    range_increasing_x(a, char::MAX)
}

pub fn range_up_decreasing_char(a: char) -> RangeDecreasing<char> {
    range_decreasing_x(a, char::MAX)
}

pub fn range_down_increasing_char(a: char) -> RangeIncreasing<char> {
    range_increasing_x('\0', a)
}

pub fn range_down_decreasing_char(a: char) -> RangeDecreasing<char> {
    range_decreasing_x('\0', a)
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
    /*
    pub fn nonzero_i32s_geometric(&self, scale: u32) -> NonzeroI32sGeometric {
        match self {
            &IteratorProvider::Exhaustive => NonzeroI32sGeometric::exhaustive(),
            &IteratorProvider::Random(ref key, ref seed) => {
                NonzeroI32sGeometric::Random(IteratorProvider::Random(key.clone(), *seed)
                                                 .altered("abs")
                                                 .positive_u32s_geometric(scale),
                                             random_bools(&scramble(seed, "sign")[..]))
            }
        }
    }

    pub fn i32s_geometric(&self, scale: u32) -> I32sGeometric {
        match self {
            &IteratorProvider::Exhaustive => I32sGeometric::exhaustive(),
            &IteratorProvider::Random(ref key, ref seed) => {
                I32sGeometric::Random(IteratorProvider::Random(key.clone(), *seed)
                                          .altered("abs")
                                          .natural_u32s_geometric(scale),
                                      random_bools(&scramble(seed, "sign")[..]))
            }
        }
    }*/
}
