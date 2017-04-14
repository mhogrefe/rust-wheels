use std::fmt::Display;
use std::ops::*;

pub trait PrimInt
    : BitAnd<Output = Self> + BitOr<Output = Self> + BitXor<Output = Self> + Copy + Display + Eq +
      Ord + ShlAssign<Self> + ShrAssign + Add<Output = Self> + Sub<Output = Self> {
    fn leading_zeros(&self) -> u32;

    fn bit_count() -> u32;

    fn from_u8(i: u8) -> Self;
}
