use iterators::general::CachedIterator;

#[derive(Debug, Eq, PartialEq)]
pub struct LogPairIndices(u64);

impl LogPairIndices {
    pub fn new() -> LogPairIndices {
        LogPairIndices(1)
    }

    pub fn from_indices(i: usize, j: usize) -> Option<LogPairIndices> {
        let i = i as u64;
        let j = j as u64;
        if i.leading_zeros() == 0 {
            return None;
        }
        let i = (i << 1) | 1;
        if (i.leading_zeros() as u64) < j {
            None
        } else {
            Some(LogPairIndices(i << j))
        }
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

pub struct LogPairsFromSingle<I: Iterator>
    where I::Item: Clone
{
    xs: CachedIterator<I>,
    i: LogPairIndices,
    stop_checking_size: bool,
    max_indices: Option<LogPairIndices>,
}

impl<I: Iterator> Iterator for LogPairsFromSingle<I>
    where I::Item: Clone
{
    type Item = (I::Item, I::Item);

    fn next(&mut self) -> Option<(I::Item, I::Item)> {
        loop {
            if self.max_indices.as_ref() == Some(&self.i) {
                return None;
            }
            let (i, j) = self.i.indices();
            let ox = self.xs.get(i);
            if ox.is_none() {
                self.i.increment();
                continue;
            }
            let oy = self.xs.get(j);
            if oy.is_none() {
                self.i.increment();
                continue;
            }
            if !self.stop_checking_size {
                if let Some(size) = self.xs.currently_known_size() {
                    self.max_indices = LogPairIndices::from_indices(size, size);
                    self.stop_checking_size = true;
                }
            }
            self.i.increment();
            return Some((ox.unwrap(), oy.unwrap()));
        }
    }
}

//TODO test
pub fn log_pairs_from_single<I: Iterator>(xs: I) -> LogPairsFromSingle<I>
    where I::Item: Clone
{
    LogPairsFromSingle {
        xs: CachedIterator::new(xs),
        i: LogPairIndices::new(),
        stop_checking_size: false,
        max_indices: None,
    }
}
