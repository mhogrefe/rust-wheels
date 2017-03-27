extern crate rand;

use self::rand::{IsaacRng, Rng, SeedableRng};

const SEED_SIZE: usize = 4;

pub enum IteratorProvider {
    Exhaustive,
    Random([u32; SEED_SIZE]),
}

macro_rules! integer_range {
    ($t: ty, $ri: ident, $rd: ident, $r: ident, $max: expr) => {
        pub struct $ri {
            i: $t,
            b: $t,
            done: bool,
        }

        impl $ri {
            fn new(a: $t, b: $t) -> $ri {
                $ri {
                    i: a,
                    b: b,
                    done: false,
                }
            }
        }

        impl Iterator for $ri {
            type Item = $t;

            fn next(&mut self) -> Option<Self::Item> {
                if self.done {
                    None
                } else {
                    self.done = self.i == self.b;
                    let ret = self.i;
                    if !self.done {
                        self.i += 1
                    }
                    Some(ret)
                }
            }
        }

        pub struct $rd {
            a: $t,
            i: $t,
            done: bool,
        }

        impl $rd {
            fn new(a: $t, b: $t) -> $rd {
                $rd {
                    a: a,
                    i: b,
                    done: false,
                }
            }
        }

        impl Iterator for $rd {
            type Item = $t;

            fn next(&mut self) -> Option<Self::Item> {
                if self.done {
                    None
                } else {
                    self.done = self.i == self.a;
                    let ret = self.i;
                    if !self.done {
                        self.i -= 1
                    }
                    Some(ret)
                }
            }
        }

        pub struct $r {
            rng: IsaacRng,
        }

        impl $r {
            fn new(seed: &[u32]) -> $r {
                $r { rng: SeedableRng::from_seed(seed) }
            }
        }

        impl Iterator for $r {
            type Item = $t;

            fn next(&mut self) -> Option<Self::Item> {
                Some(self.rng.gen::<$t>())
            }
        }
    }
}

integer_range!(u8,
               RangeIncreasingU8,
               RangeDecreasingU8,
               RandomU8s,
               u8::max_value());
integer_range!(u16,
               RangeIncreasingU16,
               RangeDecreasingU16,
               RandomU16s,
               u16::max_value());
integer_range!(u32,
               RangeIncreasingU32,
               RangeDecreasingU32,
               RandomU32s,
               u32::max_value());
integer_range!(u64,
               RangeIncreasingU64,
               RangeDecreasingU64,
               RandomU64s,
               u64::max_value());
integer_range!(usize,
               RangeIncreasingUsize,
               RangeDecreasingUsize,
               RandomUsizes,
               usize::max_value());
integer_range!(i8,
               RangeIncreasingI8,
               RangeDecreasingI8,
               RandomI8s,
               i8::max_value());
integer_range!(i16,
               RangeIncreasingI16,
               RangeDecreasingI16,
               RandomI16s,
               i16::max_value());
integer_range!(i32,
               RangeIncreasingI32,
               RangeDecreasingI32,
               RandomI32s,
               i32::max_value());
integer_range!(i64,
               RangeIncreasingI64,
               RangeDecreasingI64,
               RandomI64s,
               i64::max_value());
integer_range!(isize,
               RangeIncreasingIsize,
               RangeDecreasingIsize,
               RandomIsizes,
               isize::max_value());

macro_rules! integer_range_impl {
    (
        $t: ty,
        $rui_f: ident,
        $rud_f: ident,
        $rdi_f: ident,
        $rdd_f: ident,
        $ri_f: ident,
        $rd_f: ident,
        $i: ident,
        $d: ident,
        $ri_s: ident,
        $rd_s: ident,
        $min: expr,
        $max: expr
    ) => {
        pub fn $rui_f(&mut self, a: $t) -> $ri_s {
            $ri_s::new(a, $max)
        }

        pub fn $rud_f(&mut self, a: $t) -> $rd_s {
            $rd_s::new(a, $max)
        }

        pub fn $rdi_f(&mut self, b: $t) -> $ri_s {
            $ri_s::new($min, b)
        }

        pub fn $rdd_f(&mut self, b: $t) -> $rd_s {
            $rd_s::new($min, b)
        }

        pub fn $ri_f(&mut self, a: $t, b: $t) -> $ri_s {
            if a > b {
                panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
            }
            $ri_s::new(a, b)
        }

        pub fn $rd_f(&mut self, a: $t, b: $t) -> $rd_s {
            if a > b {
                panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
            }
            $rd_s::new(a, b)
        }

        pub fn $i(&mut self) -> $ri_s {
            $ri_s::new($min, $max)
        }

        pub fn $d(&mut self) -> $rd_s {
            $rd_s::new($min, $max)
        }
    }
}

pub enum U8s {
    Random(RandomU8s),
}

impl Iterator for U8s {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            &mut U8s::Random(ref mut it) => it.next(),
        }
    }
}

impl IteratorProvider {
    pub fn example_random() -> IteratorProvider {
        IteratorProvider::Random([0xc2ba7ec5, 0x8570291c, 0xc01903b4, 0xb3b63b5e])
    }

    integer_range_impl!(u8,
                        range_up_increasing_u8,
                        range_up_decreasing_u8,
                        range_down_increasing_u8,
                        range_down_decreasing_u8,
                        range_increasing_u8,
                        range_decreasing_u8,
                        u8s_increasing,
                        u8s_decreasing,
                        RangeIncreasingU8,
                        RangeDecreasingU8,
                        0,
                        u8::max_value());
    integer_range_impl!(u16,
                        range_up_increasing_u16,
                        range_up_decreasing_u16,
                        range_down_increasing_u16,
                        range_down_decreasing_u16,
                        range_increasing_u16,
                        range_decreasing_u16,
                        u16s_increasing,
                        u16s_decreasing,
                        RangeIncreasingU16,
                        RangeDecreasingU16,
                        0,
                        u16::max_value());
    integer_range_impl!(u32,
                        range_up_increasing_u32,
                        range_up_decreasing_u32,
                        range_down_increasing_u32,
                        range_down_decreasing_u32,
                        range_increasing_u32,
                        range_decreasing_u32,
                        u32s_increasing,
                        u32s_decreasing,
                        RangeIncreasingU32,
                        RangeDecreasingU32,
                        0,
                        u32::max_value());
    integer_range_impl!(u64,
                        range_up_increasing_u64,
                        range_up_decreasing_u64,
                        range_down_increasing_u64,
                        range_down_decreasing_u64,
                        range_increasing_u64,
                        range_decreasing_u64,
                        u64s_increasing,
                        u64s_decreasing,
                        RangeIncreasingU64,
                        RangeDecreasingU64,
                        0,
                        u64::max_value());
    integer_range_impl!(usize,
                        range_up_increasing_usize,
                        range_up_decreasing_usize,
                        range_down_increasing_usize,
                        range_down_decreasing_usize,
                        range_increasing_usize,
                        range_decreasing_usize,
                        usizes_increasing,
                        usizes_decreasing,
                        RangeIncreasingUsize,
                        RangeDecreasingUsize,
                        0,
                        usize::max_value());
    integer_range_impl!(i8,
                        range_up_increasing_i8,
                        range_up_decreasing_i8,
                        range_down_increasing_i8,
                        range_down_decreasing_i8,
                        range_increasing_i8,
                        range_decreasing_i8,
                        i8s_increasing,
                        i8s_decreasing,
                        RangeIncreasingI8,
                        RangeDecreasingI8,
                        i8::min_value(),
                        i8::max_value());
    integer_range_impl!(i16,
                        range_up_increasing_i16,
                        range_up_decreasing_i16,
                        range_down_increasing_i16,
                        range_down_decreasing_i16,
                        range_increasing_i16,
                        range_decreasing_i16,
                        i16s_increasing,
                        i16s_decreasing,
                        RangeIncreasingI16,
                        RangeDecreasingI16,
                        i16::min_value(),
                        i16::max_value());
    integer_range_impl!(i32,
                        range_up_increasing_i32,
                        range_up_decreasing_i32,
                        range_down_increasing_i32,
                        range_down_decreasing_i32,
                        range_increasing_i32,
                        range_decreasing_i32,
                        i32s_increasing,
                        i32s_decreasing,
                        RangeIncreasingI32,
                        RangeDecreasingI32,
                        i32::min_value(),
                        i32::max_value());
    integer_range_impl!(i64,
                        range_up_increasing_i64,
                        range_up_decreasing_i64,
                        range_down_increasing_i64,
                        range_down_decreasing_i64,
                        range_increasing_i64,
                        range_decreasing_i64,
                        i64s_increasing,
                        i64s_decreasing,
                        RangeIncreasingI64,
                        RangeDecreasingI64,
                        i64::min_value(),
                        i64::max_value());
    integer_range_impl!(isize,
                        range_up_increasing_isize,
                        range_up_decreasing_isize,
                        range_down_increasing_isize,
                        range_down_decreasing_isize,
                        range_increasing_isize,
                        range_decreasing_isize,
                        isizes_increasing,
                        isizes_decreasing,
                        RangeIncreasingIsize,
                        RangeDecreasingIsize,
                        isize::min_value(),
                        isize::max_value());

    pub fn u8s(&mut self) -> U8s {
        match self {
            &mut IteratorProvider::Exhaustive => {
                let dummy: [u32; 4] = [0, 0, 0, 0];
                U8s::Random(RandomU8s::new(&dummy))
            }
            &mut IteratorProvider::Random(seed) => U8s::Random(RandomU8s::new(&seed)),
        }
    }
}
