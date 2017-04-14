use rand::distributions::range::SampleRange;
use rand::Rand;
use std::fmt::Display;
use std::ops::*;

pub trait PrimInt
    : BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self> + Copy + Display + Eq +
      Ord + ShlAssign<Self> + ShrAssign + Add<Output = Self> + Sub<Output = Self> + Rand +
      SampleRange + Walkable {

    fn bit_count() -> u32;

    fn min_value() -> Self;

    fn max_value() -> Self;

    fn from_u8(i: u8) -> Self;

    fn leading_zeros(&self) -> u32;
}

pub trait PrimUnsignedInt: PrimInt {}

pub trait Walkable: Copy + Display + Eq + Ord {
    fn increment(&mut self);

    fn decrement(&mut self);
}
