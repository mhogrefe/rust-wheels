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

macro_rules! integer_range_impl {
    (
        $t: ty,
        $rui_f: ident,
        $rud_f: ident,
        $rdi_f: ident,
        $rdd_f: ident,
        $ri_f: ident,
        $rd_f: ident,
        $ri_s: ident,
        $rd_s: ident,
        $max: expr
    ) => {
        pub fn $rui_f(&mut self, a: $t) -> $ri_s {
            $ri_s::new(a, $max)
        }

        pub fn $rud_f(&mut self, a: $t) -> $rd_s {
            $rd_s::new(a, $max)
        }

        pub fn $rdi_f(&mut self, b: $t) -> $ri_s {
            $ri_s::new(0, b)
        }

        pub fn $rdd_f(&mut self, b: $t) -> $rd_s {
            $rd_s::new(0, b)
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
                        RangeIncreasingU8,
                        RangeDecreasingU8,
                        u8::max_value());

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
