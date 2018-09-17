use malachite_base::misc::{CheckedFrom, CheckedInto};
use malachite_base::num::{One, Parity, PrimitiveFloat, PrimitiveUnsigned, SignificantBits, Zero};
use malachite_nz::integer::Integer;
use std::cmp::Ordering;
use std::ops::{Add, Neg, Shl, Shr, Sub};

#[derive(Debug, Eq, PartialEq)]
pub struct BinaryFraction {
    mantissa: Integer,
    exponent: i32,
}

impl Zero for BinaryFraction {
    const ZERO: BinaryFraction = BinaryFraction {
        mantissa: Integer::ZERO,
        exponent: 0,
    };
}

impl One for BinaryFraction {
    const ONE: BinaryFraction = BinaryFraction {
        mantissa: Integer::ONE,
        exponent: 0,
    };
}

impl Neg for BinaryFraction {
    type Output = BinaryFraction;

    fn neg(self) -> BinaryFraction {
        BinaryFraction {
            mantissa: -self.mantissa,
            exponent: self.exponent,
        }
    }
}

impl<'a> Neg for &'a BinaryFraction {
    type Output = BinaryFraction;

    fn neg(self) -> BinaryFraction {
        BinaryFraction {
            mantissa: -&self.mantissa,
            exponent: self.exponent,
        }
    }
}

impl PartialOrd for BinaryFraction {
    fn partial_cmp(&self, other: &BinaryFraction) -> Option<Ordering> {
        let sign_cmp = self.sign().cmp(&other.sign());
        Some(if sign_cmp != Ordering::Equal {
            sign_cmp
        } else {
            match self.exponent.cmp(&other.exponent) {
                Ordering::Equal => self.mantissa.cmp(&other.mantissa),
                Ordering::Less => self
                    .mantissa
                    .cmp(&(&other.mantissa << (other.exponent - self.exponent))),
                Ordering::Greater => {
                    (&self.mantissa << (self.exponent - other.exponent)).cmp(&other.mantissa)
                }
            }
        })
    }
}

impl Shl<i32> for BinaryFraction {
    type Output = BinaryFraction;

    fn shl(self, rhs: i32) -> BinaryFraction {
        BinaryFraction {
            mantissa: self.mantissa,
            exponent: self.exponent + rhs,
        }
    }
}

impl<'a> Shl<i32> for &'a BinaryFraction {
    type Output = BinaryFraction;

    fn shl(self, rhs: i32) -> BinaryFraction {
        BinaryFraction {
            mantissa: self.mantissa.clone(),
            exponent: self.exponent + rhs,
        }
    }
}

impl Shr<i32> for BinaryFraction {
    type Output = BinaryFraction;

    fn shr(self, rhs: i32) -> BinaryFraction {
        BinaryFraction {
            mantissa: self.mantissa,
            exponent: self.exponent - rhs,
        }
    }
}

impl<'a> Shr<i32> for &'a BinaryFraction {
    type Output = BinaryFraction;

    fn shr(self, rhs: i32) -> BinaryFraction {
        BinaryFraction {
            mantissa: self.mantissa.clone(),
            exponent: self.exponent - rhs,
        }
    }
}

impl Add<BinaryFraction> for BinaryFraction {
    type Output = BinaryFraction;

    fn add(self, rhs: BinaryFraction) -> BinaryFraction {
        if self == BinaryFraction::ZERO {
            rhs
        } else if rhs == BinaryFraction::ZERO {
            self
        } else {
            match self.exponent.cmp(&rhs.exponent) {
                Ordering::Equal => BinaryFraction::new(self.mantissa + rhs.mantissa, self.exponent),
                Ordering::Less => BinaryFraction::new(
                    (rhs.mantissa << (rhs.exponent - self.exponent)) + self.mantissa,
                    self.exponent,
                ),
                Ordering::Greater => BinaryFraction::new(
                    (self.mantissa << (self.exponent - rhs.exponent)) + rhs.mantissa,
                    rhs.exponent,
                ),
            }
        }
    }
}

impl Sub<BinaryFraction> for BinaryFraction {
    type Output = BinaryFraction;

    fn sub(self, rhs: BinaryFraction) -> BinaryFraction {
        if self == BinaryFraction::ZERO {
            -rhs
        } else if rhs == BinaryFraction::ZERO {
            self
        } else {
            match self.exponent.cmp(&rhs.exponent) {
                Ordering::Equal => BinaryFraction::new(self.mantissa - rhs.mantissa, self.exponent),
                Ordering::Less => BinaryFraction::new(
                    self.mantissa - (rhs.mantissa << (rhs.exponent - self.exponent)),
                    self.exponent,
                ),
                Ordering::Greater => BinaryFraction::new(
                    (self.mantissa << (self.exponent - rhs.exponent)) - rhs.mantissa,
                    rhs.exponent,
                ),
            }
        }
    }
}

impl BinaryFraction {
    pub fn new(mantissa: Integer, exponent: i32) -> BinaryFraction {
        if let Some(trailing_zeros) = mantissa.trailing_zeros() {
            let trailing_zeros = trailing_zeros as i32;
            BinaryFraction {
                mantissa: mantissa >> trailing_zeros,
                exponent: exponent + trailing_zeros,
            }
        } else {
            BinaryFraction::ZERO
        }
    }

    fn sign(&self) -> Ordering {
        self.mantissa.sign()
    }

    fn into_integer(self) -> Option<Integer> {
        if self.exponent >= 0 {
            Some(self.mantissa << self.exponent)
        } else {
            None
        }
    }

    pub fn from_float<T: PrimitiveFloat>(mut f: T) -> Option<BinaryFraction>
    where
        Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    {
        if f == T::ZERO {
            Some(BinaryFraction::ZERO)
        } else if f == T::ONE {
            Some(BinaryFraction::ONE)
        } else if !f.is_finite() {
            None
        } else {
            let positive = f.is_sign_positive();
            if !positive {
                f.neg_assign();
            }
            let (mut mantissa, offset_exponent) = f.to_adjusted_mantissa_and_exponent();
            let mut exponent = offset_exponent as i32;
            if exponent == 0 {
                exponent = T::MIN_EXPONENT;
            } else {
                mantissa += T::UnsignedOfEqualWidth::ONE << T::MANTISSA_WIDTH.into();
                exponent += T::MIN_EXPONENT - 1;
            }
            let mantissa = mantissa.to_signed_checked().unwrap();
            let signed_mantissa = if positive { mantissa } else { -mantissa };
            Some(BinaryFraction::new(
                Integer::from(signed_mantissa),
                exponent,
            ))
        }
    }

    pub fn largest_finite_float<T: PrimitiveFloat>() -> BinaryFraction
    where
        Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    {
        BinaryFraction::from_float(T::MAX_FINITE).unwrap()
    }

    pub fn to_float<T: PrimitiveFloat>(&self) -> Option<T>
    where
        Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
        T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
    {
        if *self == BinaryFraction::ZERO {
            return Some(T::ZERO);
        }
        if self.mantissa < 0 {
            return (-self).to_float::<T>().map(|f| -f);
        }
        let fp_exponent = u32::checked_from(self.mantissa.significant_bits())
            .unwrap()
            .to_signed_checked()
            .unwrap()
            + self.exponent
            - 1;
        let signed_max_exponent = T::MAX_EXPONENT.to_signed_checked().unwrap();
        if fp_exponent > signed_max_exponent
            || fp_exponent == signed_max_exponent
                && *self > BinaryFraction::largest_finite_float::<T>()
        {
            return None;
        }
        let (adjusted_mantissa, adjusted_exponent) = if fp_exponent < T::MIN_NORMAL_EXPONENT {
            (self >> T::MIN_EXPONENT, 0)
        } else {
            (
                ((self >> fp_exponent) - BinaryFraction::ONE) << T::MANTISSA_WIDTH as i32,
                fp_exponent + T::MAX_EXPONENT as i32,
            )
        };
        adjusted_mantissa.into_integer().map(|i| {
            T::from_adjusted_mantissa_and_exponent(
                (&i).checked_into().unwrap(),
                adjusted_exponent as u32,
            )
        })
    }
}

pub fn from_mantissa_and_exponent<T: PrimitiveFloat>(
    mantissa: T::SignedOfEqualWidth,
    exponent: i32,
) -> Option<T>
where
    Integer: From<T::SignedOfEqualWidth>,
    Integer: From<<T::UnsignedOfEqualWidth as PrimitiveUnsigned>::SignedOfEqualWidth>,
    T::UnsignedOfEqualWidth: for<'a> CheckedFrom<&'a Integer>,
{
    if mantissa == <T::SignedOfEqualWidth as Zero>::ZERO && exponent != 0 || mantissa.is_even() {
        None
    } else {
        BinaryFraction::new(Integer::from(mantissa), exponent).to_float()
    }
}
