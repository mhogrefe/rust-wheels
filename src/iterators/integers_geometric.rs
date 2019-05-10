use malachite_base::conversion::WrappingFrom;
use rand::{IsaacRng, Rng, SeedableRng};

use iterators::common::scramble;
use iterators::general::{random, Random};

pub struct PositiveU32sGeometric {
    rng: Box<IsaacRng>,
    weight: u32,
}

impl Iterator for PositiveU32sGeometric {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let mut j = 1;
        while !self.rng.gen_weighted_bool(self.weight) {
            j += 1;
        }
        Some(j)
    }
}

pub fn positive_u32s_geometric(seed: &[u32], scale: u32) -> PositiveU32sGeometric {
    PositiveU32sGeometric {
        rng: Box::new(IsaacRng::from_seed(seed)),
        weight: scale + 2,
    }
}

pub struct U32sGeometric {
    rng: Box<IsaacRng>,
    weight: u32,
}

impl Iterator for U32sGeometric {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let mut j = 0;
        while !self.rng.gen_weighted_bool(self.weight) {
            j += 1;
        }
        Some(j)
    }
}

pub fn u32s_geometric(seed: &[u32], scale: u32) -> U32sGeometric {
    U32sGeometric {
        rng: Box::new(IsaacRng::from_seed(seed)),
        weight: scale + 2,
    }
}

pub struct NegativeI32sGeometric(PositiveU32sGeometric);

impl Iterator for NegativeI32sGeometric {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.0.next().map(|i| -i32::wrapping_from(i))
    }
}

pub fn negative_i32s_geometric(seed: &[u32], scale: u32) -> NegativeI32sGeometric {
    NegativeI32sGeometric(positive_u32s_geometric(seed, scale))
}

pub struct NonzeroI32sGeometric {
    sign_gen: Random<bool>,
    abs_gen: PositiveU32sGeometric,
}

impl Iterator for NonzeroI32sGeometric {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.sign_gen.next().unwrap() {
            self.abs_gen.next().map(i32::wrapping_from)
        } else {
            self.abs_gen.next().map(|i| -i32::wrapping_from(i))
        }
    }
}

pub fn nonzero_i32s_geometric(seed: &[u32], scale: u32) -> NonzeroI32sGeometric {
    NonzeroI32sGeometric {
        sign_gen: random(&scramble(seed, "sign")),
        abs_gen: positive_u32s_geometric(&scramble(seed, "abs"), scale),
    }
}

pub struct I32sGeometric {
    signs: Random<bool>,
    abs: U32sGeometric,
}

impl Iterator for I32sGeometric {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.signs.next().unwrap() {
            self.abs.next().map(i32::wrapping_from)
        } else {
            self.abs.next().map(|i| -i32::wrapping_from(i))
        }
    }
}

pub fn i32s_geometric(seed: &[u32], scale: u32) -> I32sGeometric {
    I32sGeometric {
        signs: random(&scramble(seed, "signs")),
        abs: u32s_geometric(&scramble(seed, "abs"), scale),
    }
}

pub struct RangeUpGeometricU32 {
    naturals: U32sGeometric,
    min: u32,
}

impl Iterator for RangeUpGeometricU32 {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        self.naturals.next().map(|u| u + self.min)
    }
}

pub fn range_up_geometric_u32(seed: &[u32], scale: u32, min: u32) -> RangeUpGeometricU32 {
    RangeUpGeometricU32 {
        naturals: u32s_geometric(seed, scale),
        min,
    }
}

pub struct RangeUpGeometricI32 {
    naturals: U32sGeometric,
    min: i32,
}

impl Iterator for RangeUpGeometricI32 {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.naturals
            .next()
            .map(|u| i32::wrapping_from(u) + self.min)
    }
}

pub fn range_up_geometric_i32(seed: &[u32], scale: u32, min: i32) -> RangeUpGeometricI32 {
    RangeUpGeometricI32 {
        naturals: u32s_geometric(seed, scale),
        min,
    }
}

pub struct RangeDownGeometric {
    naturals: U32sGeometric,
    max: i32,
}

impl Iterator for RangeDownGeometric {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.naturals
            .next()
            .map(|u| self.max - i32::wrapping_from(u))
    }
}

pub fn range_down_geometric(seed: &[u32], scale: u32, max: i32) -> RangeDownGeometric {
    RangeDownGeometric {
        naturals: u32s_geometric(seed, scale),
        max,
    }
}
