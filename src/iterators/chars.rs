use std::char;

use malachite_base::chars::{char_to_contiguous_range, contiguous_range_to_char};
use rand::distributions::{IndependentSample, Range};
use rand::{IsaacRng, Rng, SeedableRng};

pub struct RandomAsciiChars(IsaacRng);

impl Iterator for RandomAsciiChars {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        Some(char::from_u32(u32::from(self.0.gen::<u8>() & 0x7f)).unwrap())
    }
}

pub fn random_ascii_chars(seed: &[u32]) -> RandomAsciiChars {
    RandomAsciiChars(IsaacRng::from_seed(seed))
}

pub struct RandomRangeChar {
    rng: Box<IsaacRng>,
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
        rng: Box::new(IsaacRng::from_seed(seed)),
        range: Range::new(char_to_contiguous_range(a), char_to_contiguous_range(b) + 1),
    }
}

pub fn random_range_up_char(seed: &[u32], a: char) -> RandomRangeChar {
    random_range_char(seed, a, char::MAX)
}

pub fn random_range_down_char(seed: &[u32], a: char) -> RandomRangeChar {
    random_range_char(seed, '\0', a)
}
