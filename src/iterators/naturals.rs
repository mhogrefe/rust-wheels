use malachite::natural::Natural;

pub struct RangeIncreasingNatural {
    i: Natural,
    b: Natural,
    done: bool,
}

impl Iterator for RangeIncreasingNatural {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        if self.done {
            None
        } else {
            self.done = self.i == self.b;
            let ret = self.i.clone();
            if !self.done {
                self.i += 1
            }
            Some(ret)
        }
    }
}

pub struct RangeDecreasingNatural {
    a: Natural,
    i: Natural,
    done: bool,
}

impl Iterator for RangeDecreasingNatural {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        if self.done {
            None
        } else {
            self.done = self.i == self.a;
            let ret = self.i.clone();
            if !self.done {
                self.i -= 1
            }
            Some(ret)
        }
    }
}

pub struct RangeIncreasingUnboundedNatural {
    i: Natural,
}

impl Iterator for RangeIncreasingUnboundedNatural {
    type Item = Natural;

    fn next(&mut self) -> Option<Natural> {
        let ret = self.i.clone();
        self.i += 1;
        Some(ret)
    }
}

pub fn range_increasing_natural(a: Natural, b: Natural) -> RangeIncreasingNatural {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RangeIncreasingNatural {
        i: a,
        b: b,
        done: false,
    }
}

pub fn range_decreasing_natural(a: Natural, b: Natural) -> RangeDecreasingNatural {
    if a > b {
        panic!("a must be less than or equal to b. a: {}, b: {}", a, b);
    }
    RangeDecreasingNatural {
        a: a,
        i: b,
        done: false,
    }
}

pub fn range_up_increasing_natural(a: Natural) -> RangeIncreasingUnboundedNatural {
    RangeIncreasingUnboundedNatural { i: a }
}

pub fn range_down_increasing_natural(a: Natural) -> RangeIncreasingNatural {
    range_increasing_natural(Natural::from(0), a)
}

pub fn range_down_decreasing_natural(a: Natural) -> RangeDecreasingNatural {
    range_decreasing_natural(Natural::from(0), a)
}

//TODO test
pub fn exhaustive_positive_naturals() -> RangeIncreasingUnboundedNatural {
    range_up_increasing_natural(Natural::from(1))
}

//TODO test
pub fn exhaustive_naturals() -> RangeIncreasingUnboundedNatural {
    range_up_increasing_natural(Natural::from(0))
}
