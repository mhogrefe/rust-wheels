use iterators::common::scramble;
use iterators::general::{Random, random_x};
use iterators::primitive_ints::{RandomRange, random_range};

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
    PositiveU32sGeometric(random_range(seed, 0, scale + 1))
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
    NaturalU32sGeometric(random_range(seed, 0, scale + 1))
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
