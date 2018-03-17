use malachite_base::misc::Walkable;
use rand::distributions::range::SampleRange;
use rand::Rand;
use std::fmt::Debug;
use std::fmt::Display;
use std::hash::Hash;
use std::ops::*;

pub trait PrimInt
    : BitAnd<Output = Self>
    + BitOr<Output = Self>
    + BitXor<Output = Self>
    + Copy
    + Display
    + Debug
    + Eq
    + Hash
    + Ord
    + Shl<u32, Output = Self>
    + ShlAssign<u32>
    + Shr<u32, Output = Self>
    + ShrAssign<u32>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Rem<Output = Self>
    + DivAssign
    + Not<Output = Self>
    + Rand
    + SampleRange
    + Walkable {
    fn name() -> &'static str;

    fn min_value() -> Self;

    fn max_value() -> Self;

    fn from_u8(i: u8) -> Self;
}

pub trait PrimUnsignedInt: PrimInt {}

pub trait PrimSignedInt: PrimInt {
    fn from_i8(i: i8) -> Self;
}
