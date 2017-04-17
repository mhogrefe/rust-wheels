use rand::distributions::range::SampleRange;
use rand::Rand;
use std::fmt::Debug;
use std::fmt::Display;
use std::ops::*;

pub trait PrimInt
    : BitAnd<Output=Self> + BitOr<Output=Self> + BitXor<Output=Self> + Copy + Display + Debug + Eq +
    Ord + Shl<u32, Output=Self> + ShlAssign<u32> + Shr<u32, Output=Self> + ShrAssign<u32> +
    Add<Output=Self> + Sub<Output=Self> + Rem<Output=Self> + DivAssign + Not<Output=Self> + Rand +
    SampleRange + Walkable {

    fn name() -> &'static str;

    fn bit_count() -> u32;

    fn min_value() -> Self;

    fn max_value() -> Self;

    fn from_u8(i: u8) -> Self;

    fn leading_zeros(&self) -> u32;
}

pub trait PrimUnsignedInt: PrimInt {}

pub trait PrimSignedInt: PrimInt {
    fn from_i8(i: i8) -> Self;
}

pub trait Walkable: Copy + Display + Eq + Ord {
    fn increment(&mut self);

    fn decrement(&mut self);
}
