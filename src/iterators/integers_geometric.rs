use iterators::common::scramble;
use iterators::general::{Random, random_x};
use rand::{IsaacRng, Rng, SeedableRng};

pub struct PositiveU32sGeometric {
    rng: IsaacRng,
    weight: u32,
}

impl Iterator for PositiveU32sGeometric {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let mut j = 0;
        loop {
            j += 1;
            if self.rng.gen_weighted_bool(self.weight) {
                return Some(j);
            }
        }
    }
}

pub fn positive_u32s_geometric(seed: &[u32], scale: u32) -> PositiveU32sGeometric {
    PositiveU32sGeometric {
        rng: IsaacRng::from_seed(seed),
        weight: scale + 2,
    }
}

pub struct NaturalU32sGeometric {
    rng: IsaacRng,
    weight: u32,
}

impl Iterator for NaturalU32sGeometric {
    type Item = u32;

    fn next(&mut self) -> Option<u32> {
        let mut j = 0;
        loop {
            if self.rng.gen_weighted_bool(self.weight) {
                return Some(j);
            }
            j += 1;
        }
    }
}

pub fn natural_u32s_geometric(seed: &[u32], scale: u32) -> NaturalU32sGeometric {
    NaturalU32sGeometric {
        rng: IsaacRng::from_seed(seed),
        weight: scale + 2,
    }
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

pub struct NonzeroI32sGeometric {
    sign_gen: Random<bool>,
    abs_gen: PositiveU32sGeometric,
}

impl Iterator for NonzeroI32sGeometric {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.sign_gen.next().unwrap() {
            self.abs_gen.next().map(|i| i as i32)
        } else {
            self.abs_gen.next().map(|i| -(i as i32))
        }
    }
}

pub fn nonzero_i32s_geometric(seed: &[u32], scale: u32) -> NonzeroI32sGeometric {
    NonzeroI32sGeometric {
        sign_gen: random_x(&scramble(seed, "sign")),
        abs_gen: positive_u32s_geometric(&scramble(seed, "abs"), scale),
    }
}

pub struct I32sGeometric {
    sign_gen: Random<bool>,
    abs_gen: NaturalU32sGeometric,
}

impl Iterator for I32sGeometric {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        if self.sign_gen.next().unwrap() {
            self.abs_gen.next().map(|i| i as i32)
        } else {
            self.abs_gen.next().map(|i| -(i as i32))
        }
    }
}

pub fn i32s_geometric(seed: &[u32], scale: u32) -> I32sGeometric {
    I32sGeometric {
        sign_gen: random_x(&scramble(seed, "sign")),
        abs_gen: natural_u32s_geometric(&scramble(seed, "abs"), scale),
    }
}

pub struct RangeUpGeometricU32 {
    naturals: NaturalU32sGeometric,
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
        naturals: natural_u32s_geometric(seed, scale),
        min: min,
    }
}

pub struct RangeUpGeometricI32 {
    naturals: NaturalU32sGeometric,
    min: i32,
}

impl Iterator for RangeUpGeometricI32 {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.naturals.next().map(|u| u as i32 + self.min)
    }
}

pub fn range_up_geometric_i32(seed: &[u32], scale: u32, min: i32) -> RangeUpGeometricI32 {
    RangeUpGeometricI32 {
        naturals: natural_u32s_geometric(seed, scale),
        min: min,
    }
}

pub struct RangeDownGeometric {
    naturals: NaturalU32sGeometric,
    max: i32,
}

impl Iterator for RangeDownGeometric {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.naturals.next().map(|u| self.max - u as i32)
    }
}

pub fn range_down_geometric(seed: &[u32], scale: u32, max: i32) -> RangeDownGeometric {
    RangeDownGeometric {
        naturals: natural_u32s_geometric(seed, scale),
        max: max,
    }
}
