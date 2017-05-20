pub struct LogPairIndices(u64);

impl LogPairIndices {
    pub fn new() -> LogPairIndices {
        LogPairIndices(1)
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn indices(&self) -> (usize, usize) {
        let y = self.0.trailing_zeros();
        ((self.0 >> (y + 1)) as usize, y as usize)
    }
}

impl Default for LogPairIndices {
    fn default() -> LogPairIndices {
        LogPairIndices(1)
    }
}

#[derive(Default)]
pub struct SqrtPairIndices {
    pub x: u64,
    pub y: u64,
}

impl SqrtPairIndices {
    pub fn new() -> SqrtPairIndices {
        SqrtPairIndices { x: 0, y: 0 }
    }

    pub fn increment(&mut self) {
        let mut ix = 0;
        let mut iy = 0;
        loop {
            let mask = 1 << iy;
            if self.y & mask != 0 {
                self.y &= !mask;
                iy += 1;
            } else {
                self.y |= mask;
                return;
            }
            for _ in 0..2 {
                let mask = 1 << ix;
                if self.x & mask != 0 {
                    self.x &= !mask;
                    ix += 1;
                } else {
                    self.x |= mask;
                    return;
                }
            }
        }
    }
}

pub struct ZOrderTupleIndices(pub Vec<u64>);

impl ZOrderTupleIndices {
    pub fn new(size: usize) -> ZOrderTupleIndices {
        let mut v = Vec::new();
        v.resize(size, 0);
        ZOrderTupleIndices(v)
    }

    pub fn increment(&mut self) {
        for j in 0..64 {
            let mask = 1 << j;
            for i in (0..self.0.len()).rev() {
                if self.0[i] & mask != 0 {
                    self.0[i] &= !mask;
                } else {
                    self.0[i] |= mask;
                    return;
                }
            }
        }
    }
}
