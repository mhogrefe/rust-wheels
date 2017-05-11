use iterators::adaptors::MultiChain;
use iterators::general::{RangeDecreasing, range_decreasing_x, RangeIncreasing, range_increasing_x};
use prim_utils::char_utils::{char_to_contiguous_range, contiguous_range_to_char};
use rand::distributions::{IndependentSample, Range};
use rand::{IsaacRng, Rng, SeedableRng};
use std::char;

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

pub fn exhaustive_range_up_char(a: char) -> RangeIncreasing<char> {
    range_increasing_x(a, char::MAX)
}

pub fn exhaustive_range_down_char(a: char) -> RangeIncreasing<char> {
    range_increasing_x('\0', a)
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

pub struct RandomAsciiChars(IsaacRng);

impl Iterator for RandomAsciiChars {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        Some(char::from_u32((self.0.gen::<u8>() & 0x7f) as u32).unwrap())
    }
}

pub fn random_ascii_chars(seed: &[u32]) -> RandomAsciiChars {
    RandomAsciiChars(SeedableRng::from_seed(seed))
}

pub struct RandomRangeChar {
    rng: IsaacRng,
    range: Range<u32>,
}

impl Iterator for RandomRangeChar {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        contiguous_range_to_char(self.range.ind_sample(&mut self.rng))
    }
}

pub fn random_range_char(seed: &[u32], a: char, b: char) -> RandomRangeChar {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RandomRangeChar {
        rng: SeedableRng::from_seed(seed),
        range: Range::new(char_to_contiguous_range(a), char_to_contiguous_range(b) + 1),
    }
}

pub fn random_range_up_char(seed: &[u32], a: char) -> RandomRangeChar {
    random_range_char(seed, a, char::MAX)
}

pub fn random_range_down_char(seed: &[u32], a: char) -> RandomRangeChar {
    random_range_char(seed, '\0', a)
}
